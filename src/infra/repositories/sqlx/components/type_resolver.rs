use async_trait::async_trait;
use sqlx::{Row, sqlite::SqliteRow};

use crate::core::components::repositories::{ComponentRef, ComponentTypeResolver};

use super::SqliteComponentRepository;

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
