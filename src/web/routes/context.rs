use std::sync::Arc;

use async_trait::async_trait;
use serde_json::Value;
use sqlx::{Pool, Sqlite};

use crate::core::components::repositories::ComponentsRepository;

#[derive(Clone)]
pub struct Context {
    dbc: Arc<Pool<Sqlite>>,
}

impl Context {
    pub fn new(dbc: Arc<Pool<Sqlite>>) -> Self {
        Self { dbc }
    }
}

#[async_trait]
impl ComponentsRepository for Context {
    async fn save(&self, component_type: &str, title: &str, payload: Value) -> anyhow::Result<u32> {
        let mut tx = self.dbc.begin().await?;
        let result = sqlx::query!(
            r#"
            INSERT INTO components (type, current_title) 
            VALUES (?, ?)
            "#, 
            component_type, 
            title
        )
        .execute(&mut *tx)
        .await?;

        let generated_id = result.last_insert_rowid() as u32;
        let initial_version = 1;
        sqlx::query!(
            r#"
            INSERT INTO component_versions (component_id, version_number, data)
            VALUES (?, ?, ?)
            "#,
            generated_id,
            initial_version,
            payload
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        
        Ok(generated_id)
    }
}
