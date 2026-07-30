mod crud;
mod type_resolver;
mod payload_resolver;

use std::sync::Arc;

use anyhow::Ok;
use serde_json::Value;
use sqlx::{Executor, Pool, Sqlite, Transaction, sqlite::SqliteRow};

use crate::core::components::models::values::version::Version;

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
