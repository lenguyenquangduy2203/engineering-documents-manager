mod crud;
mod type_resolver;
mod payload_resolver;
mod rows;

use std::sync::Arc;

use anyhow::Ok;
use serde_json::Value;
use sqlx::{Executor, Pool, Sqlite, Transaction, types::Json};

use crate::core::components::models::{payload::ComponentPayload, values::version::Version, wrapper::Component};

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

    async fn fetch_opt_component_payload<'c, E: Executor<'c, Database = Sqlite>>(
        component_id: u32, 
        executor: E
    ) -> anyhow::Result<Option<Component<ComponentPayload>>> {
        Ok(sqlx::query_as!(
            Component::<Json<ComponentPayload>>,
            r#"
            SELECT 
                c.id AS "id!: u32", 
                c.latest_version_number AS "version: u32", 
                c.current_title AS "title", 
                v.data as "payload!: Json<ComponentPayload>"
            FROM components c
            JOIN component_versions v 
                ON c.id = v.component_id 
                AND c.latest_version_number = v.version_number
            WHERE c.id = ?
            "#,
            component_id,
        )
        .fetch_optional(executor).await?
        .map(Component::into))
    }
}
