use async_trait::async_trait;
use sqlx::{QueryBuilder, Sqlite, types::Json};

use crate::core::components::{
    models::{payload::ComponentPayload, wrapper::Component}, 
    queries::component::ComponentQuery, 
    repositories::{
        ComponentFilterQuery, ComponentsRepository, 
        RemoveComponentError, UpdateComponentError
        }
};

use super::SqliteComponentRepository;

#[async_trait]
impl ComponentsRepository for SqliteComponentRepository {
    async fn create_new(&self, component: &Component<ComponentPayload>) -> anyhow::Result<u32> {
        let mut tx = self.dbc.begin().await?;
        let identifier = component.payload.get_identifier();
        let payload = serde_json::to_value(&component.payload)?;
        let result = sqlx::query!(
            r#"
            INSERT INTO components (type, current_title, latest_version_number) 
            VALUES (?, ?, ?)
            "#, 
            identifier, 
            component.title,
            component.version.0,
        )
        .execute(&mut *tx)
        .await?;

        let generated_id = result.last_insert_rowid() as u32;
        Self::insert_new_component_version(
            generated_id, 
            component.version, 
            &payload, 
            &mut tx
        )
        .await?;

        tx.commit().await?;
        
        Ok(generated_id)
    }

    async fn find_all_latest_version(
        &self, 
        filter: ComponentFilterQuery
    ) -> anyhow::Result<Vec<Component<ComponentPayload>>> {
        let mut qb: QueryBuilder<Sqlite> = QueryBuilder::new(
            r#"
            SELECT 
                c.id AS "id", 
                c.latest_version_number AS "version", 
                c.current_title AS "title", 
                v.data as "payload"
            FROM components c
            JOIN component_versions v 
                ON c.id = v.component_id 
                AND c.latest_version_number = v.version_number
            WHERE 1=1
            "#
        );
        let specs = ComponentQuery::new(filter);
        specs.apply(&mut qb);
        let query = qb.build_query_as::<Component<Json<ComponentPayload>>>();
        let rows = query.fetch_all(&self.dbc).await?;

        Ok(rows.into_iter()
            .map(Component::into)
            .collect())
    }

    async fn find_latest_version_by_id(&self, component_id: u32) -> anyhow::Result<Option<Component<ComponentPayload>>> {
        Self::fetch_opt_component_payload(component_id, &self.dbc).await
    }

    async fn update_component(
        &self, 
        component_id: u32,
        incoming_component: Component<ComponentPayload>
    ) -> std::result::Result<Option<Component<ComponentPayload>>, UpdateComponentError> {
        let mut tx = self.dbc.begin_with("BEGIN IMMEDIATE").await?;
        let current_component = match Self::fetch_opt_component_payload(component_id, &mut *tx).await? {
            Some(r) => r,
            None => return Result::Ok(None),
        };

        let updated_component = current_component.apply_changes(incoming_component)?;
        let updated_payload_value = serde_json::to_value(&updated_component.payload)
            .map_err(|e| anyhow::anyhow!("Serialization failed: {e}"))?;

        sqlx::query!(
            r#"
            UPDATE components
            SET latest_version_number = ?, current_title = ?
            WHERE id = ?
            "#,
            updated_component.version.0,
            updated_component.title,
            updated_component.id,
        )
        .execute(&mut *tx)
        .await?;
        
        Self::insert_new_component_version(
            component_id, 
            updated_component.version, 
            &updated_payload_value, 
            &mut tx
        )
        .await?;

        tx.commit().await?;

        Result::Ok(Some(updated_component))
    }

    async fn remove_component_with_all_versions_by_id(
        &self, 
        component_id: u32
    ) -> std::result::Result<bool, RemoveComponentError> {
        let mut tx = self.dbc.begin().await?;
        sqlx::query!(
            r#"
            DELETE FROM component_versions
            WHERE component_id = ?
            "#,
            component_id
        )
        .execute(&mut *tx)
        .await?;

        let res = sqlx::query!(
            r#"
            DELETE FROM components
            WHERE id = ?
            "#,
            component_id
        )
        .execute(&mut *tx)
        .await?;

        if res.rows_affected() == 0 {
            return Result::Ok(false);
        }

        tx.commit().await?;

        Result::Ok(true)
    }
}
