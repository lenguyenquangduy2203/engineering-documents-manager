use std::{sync::Arc, result::Result};

use anyhow::Ok;
use async_trait::async_trait;
use serde_json::Value;
use sqlx::{Executor, Pool, QueryBuilder, Row, Sqlite, Transaction, sqlite::SqliteRow};

use crate::core::components::{models::{payload::ComponentPayload, values::version::Version, wrapper::Component}, queries::component::ComponentQuery, repositories::{ComponentFilterQuery, ComponentPayloadRef, ComponentPayloadResolver, ComponentRef, ComponentTypeResolver, ComponentsRepository, RemoveComponentError, UpdateComponentError}};

impl TryFrom<SqliteRow> for Component<ComponentPayload> {
    type Error = anyhow::Error;

    fn try_from(row: SqliteRow) -> Result<Self, Self::Error> {
        let payload_str: String = row.get("payload_json");
        let payload: ComponentPayload = serde_json::from_str(&payload_str)?;

        Ok(Self {
            id: Some(row.get::<u32, _>("id")),
            version: Version(row.get::<u32, _>("latest_version_number")),
            title: row.get("current_title"),
            payload,
        })
    }
}

pub struct SqliteComponentRepository {
    dbc: Arc<Pool<Sqlite>>,
}

impl SqliteComponentRepository {
    pub fn new(dbc: Arc<Pool<Sqlite>>) -> Self {
        Self { dbc }
    }

    async fn insert_new_component_version(
        component_id: u32, 
        version: Version, 
        payload: &Value, 
        tx: &mut Transaction<'_, Sqlite>
    ) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO component_versions (component_id, version_number, data)
            VALUES (?, ?, ?)
            "#,
            component_id,
            version.0,
            payload
        )
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    async fn fetch_opt_component_row<'c, E: Executor<'c, Database = Sqlite>>(
        component_id: u32, 
        executor: E
    ) -> anyhow::Result<Option<SqliteRow>> {
        Ok(sqlx::query(
            r#"
            SELECT c.id, c.latest_version_number, c.type as component_type, c.current_title, v.data as payload_json
            FROM components c
            JOIN component_versions v 
                ON c.id = v.component_id 
                AND c.latest_version_number = v.version_number
            WHERE c.id = $1
            "#
        )
        .bind(component_id)
        .fetch_optional(executor).await?)
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

        rows.into_iter()
            .map(Component::try_from)
            .collect()
    }

    async fn find_latest_version_by_id(&self, component_id: u32) -> anyhow::Result<Option<Component<ComponentPayload>>> {
        match Self::fetch_opt_component_row(component_id, &*self.dbc).await? {
            Some(row) => Ok(Some(Component::try_from(row)?)),
            None => Ok(None),
        }
    }

    async fn update_component(
        &self, 
        component_id: u32,
        incoming_component: Component<ComponentPayload>
    ) -> std::result::Result<Option<Component<ComponentPayload>>, UpdateComponentError> {
        let mut tx = self.dbc.begin_with("BEGIN IMMEDIATE").await?;
        let row = match Self::fetch_opt_component_row(component_id, &mut *tx).await? {
            Some(r) => r,
            None => return Result::Ok(None),
        };

        let current_component = Component::try_from(row)?;
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

impl TryFrom<SqliteRow> for ComponentRef {
    type Error = anyhow::Error;

    fn try_from(row: SqliteRow) -> Result<Self, Self::Error> {
        Ok(Self { 
            id: row.get("component_id"), 
            version_id: row.get("version_id"), 
            component_type: row.get("component_type") 
        })
    }
}

#[async_trait]
impl ComponentTypeResolver for SqliteComponentRepository {
    async fn find_all_components_with_type_by_version_ids(
        &self,
        version_ids: &[u32]
    ) -> anyhow::Result<Vec<ComponentRef>> {
        if version_ids.is_empty() {
            return Ok(Vec::new());
        }

        let json_ids = serde_json::to_string(version_ids)?;
        let rows = sqlx::query(
            r#"
            SELECT 
                c.id AS component_id, 
                v.id AS version_id,
                c.type AS component_type
            FROM components c
            LEFT JOIN component_versions v ON c.id = v.component_id
            WHERE v.id IN (SELECT value FROM json_each($1))
            "#
        )
        .bind(json_ids)
        .fetch_all(&*self.dbc)
        .await?;

        rows.into_iter()
            .map(ComponentRef::try_from)
            .collect()
    }
}

impl TryFrom<SqliteRow> for ComponentPayloadRef {
    type Error = anyhow::Error;

    fn try_from(row: SqliteRow) -> Result<Self, Self::Error> {
        let payload_str: String = row.get("payload");
        let payload: ComponentPayload = serde_json::from_str(&payload_str)?;

        Ok(Self { 
            component_id: row.get("component_id"), 
            version_id: row.get("version_id"), 
            payload 
        })
    }
}

#[async_trait]
impl ComponentPayloadResolver for SqliteComponentRepository {
    async fn find_all_components_with_payload_by_version_ids(
        &self,
        version_ids: &[u32]
    ) -> anyhow::Result<Vec<ComponentPayloadRef>> {
        if version_ids.is_empty() {
            return Ok(Vec::new());
        }

        let json_ids = serde_json::to_string(version_ids)?;
        let rows = sqlx::query(
            r#"
            SELECT 
                c.id AS component_id, 
                v.id AS version_id,
                v.data AS payload
            FROM components c
            LEFT JOIN component_versions v ON c.id = v.component_id
            WHERE v.id IN (SELECT value FROM json_each($1))
            "#
        )
        .bind(json_ids)
        .fetch_all(&*self.dbc)
        .await?;

        rows.into_iter()
            .map(ComponentPayloadRef::try_from)
            .collect()
    }
}