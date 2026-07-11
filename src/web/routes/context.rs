use std::sync::Arc;

use async_trait::async_trait;
use sqlx::{Pool, QueryBuilder, Sqlite};

use crate::core::components::{models::{payload::ComponentPayload, wrapper::Component}, queries::component::ComponentQuery, repositories::{ComponentFilterQuery, ComponentsRepository}};

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
    async fn create_new(&self, component: &Component<ComponentPayload>) -> anyhow::Result<u32> {
        let mut tx = self.dbc.begin().await?;
        let identifier = component.payload.get_identifier();
        let payload = serde_json::to_value(&component.payload)?;
        let result = sqlx::query!(
            r#"
            INSERT INTO components (type, current_title) 
            VALUES (?, ?)
            "#, 
            identifier, 
            component.title
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

    async fn find_all_latest_version(
        &self, 
        filter: ComponentFilterQuery
    ) -> anyhow::Result<Vec<Component<ComponentPayload>>> {
        let mut qb: QueryBuilder<Sqlite> = QueryBuilder::new(
            r#"
            SELECT c.id, c.type as component_type, c.current_title, v.data as payload_json
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
                id: Some(row.get::<i32, _>("id") as u32),
                title: row.get("current_title"),
                payload,
            });
        }

        Ok(components)
    }
}
