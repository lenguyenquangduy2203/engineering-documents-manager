use async_trait::async_trait;
use sqlx::{Row, sqlite::SqliteRow};

use crate::core::components::{models::payload::ComponentPayload, repositories::{ComponentPayloadRef, ComponentPayloadResolver}};

use super::SqliteComponentRepository;

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
