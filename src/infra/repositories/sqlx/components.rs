use std::sync::Arc;

use anyhow::anyhow;
use async_trait::async_trait;
use sqlx::{Pool, QueryBuilder, Sqlite};

use crate::core::components::{models::{payload::ComponentPayload, values::version::Version, wrapper::Component}, queries::component::ComponentQuery, repositories::{ComponentFilterQuery, ComponentsRepository}};

pub struct SqliteComponentRepository {
    dbc: Arc<Pool<Sqlite>>,
}

impl SqliteComponentRepository {
    pub fn new(dbc: Arc<Pool<Sqlite>>) -> Self {
        Self { dbc }
    }
}

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
        sqlx::query!(
            r#"
            INSERT INTO component_versions (component_id, version_number, data)
            VALUES (?, ?, ?)
            "#,
            generated_id,
            component.version.0,
            payload
        )
        .execute(&mut *tx)
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
            SELECT c.id, c.latest_version_number, c.type as component_type, c.current_title, v.data as payload_json
            FROM components c
            JOIN component_versions v 
                ON c.id = v.component_id 
                AND c.latest_version_number = v.version_number
            WHERE 1=1
            "#
        );
        let specs = ComponentQuery::new(filter);
        specs.apply(&mut qb);
        let query = qb.build();
        let rows = query.fetch_all(&*self.dbc).await?;
        let mut components = Vec::new();
        for row in rows {
            use sqlx::Row;
            let payload_str: String = row.get("payload_json");
            let payload: ComponentPayload = serde_json::from_str(&payload_str)?;

            components.push(Component {
                id: Some(row.get::<u32, _>("id")),
                version: Version(row.get::<u32, _>("latest_version_number")),
                title: row.get("current_title"),
                payload,
            });
        }

        Ok(components)
    }

    async fn find_latest_version_by_id(&self, component_id: u32) -> anyhow::Result<Option<Component<ComponentPayload>>> {
        let row = sqlx::query!(
            r#"
            SELECT c.id, c.latest_version_number, c.type as component_type, c.current_title, v.data as payload_json
            FROM components c
            JOIN component_versions v 
                ON c.id = v.component_id 
                AND c.latest_version_number = v.version_number
            WHERE c.id = ?
            "#,
            component_id,
        ).fetch_optional(&*self.dbc).await?;
        
        match row {
            Some(record) => {
                let payload: ComponentPayload = serde_json::from_str(&record.payload_json)?;

                Ok(Some(Component { 
                    id: Some(record.id as u32), 
                    version: Version(record.latest_version_number as u32), 
                    title: record.current_title, 
                    payload 
                }))
            },
            None => Ok(None),
        }
    }

    async fn update_component(
        &self, 
        incoming_component: Component<ComponentPayload>
    ) -> anyhow::Result<Option<Component<ComponentPayload>>> {
        let component_id = incoming_component.id.ok_or_else(|| {
            anyhow!("No id is specified for update")
        })?;

        let mut tx = self.dbc.begin_with("BEGIN IMMEDIATE").await?;
        let row = sqlx::query!(
            r#"
            SELECT c.id, c.latest_version_number, c.type as component_type, c.current_title, v.data as payload_json
            FROM components c
            JOIN component_versions v 
                ON c.id = v.component_id 
                AND c.latest_version_number = v.version_number
            WHERE c.id = ?
            "#,
            component_id,
        )
        .fetch_optional(&mut *tx)
        .await?;

        let record = match row {
            Some(r) => r,
            None => {
                tx.commit().await?; // Cleanly close transaction if nothing to do
                return Ok(None);
            }
        };

        let current_payload: ComponentPayload = serde_json::from_str(&record.payload_json)?;
        let current_component = Component {
            id: Some(record.id as u32),
            version: Version(record.latest_version_number as u32),
            title: record.current_title,
            payload: current_payload,
        };

        let updated_component = current_component.apply_changes(incoming_component)?;
        let updated_payload_value = serde_json::to_value(&updated_component.payload)?;

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
        
        sqlx::query!(
            r#"
            INSERT INTO component_versions (component_id, version_number, data)
            VALUES (?, ?, ?)
            "#,
            updated_component.id,
            updated_component.version.0,
            updated_payload_value
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        return Ok(Some(updated_component));
    }
}
