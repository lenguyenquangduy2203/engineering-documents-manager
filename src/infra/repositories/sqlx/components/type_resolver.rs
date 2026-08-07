use anyhow::Context;
use async_trait::async_trait;

use crate::core::components::repositories::component_type_resolver::{ComponentRef, ComponentTypeResolver};

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

        let json_ids = serde_json::to_string(version_ids)
            .with_context(|| format!("Failed to serialize version_ids to JSON array: {version_ids:?}"))?;

        sqlx::query_as!(
            ComponentRef,
            r#"
            SELECT 
                c.id AS "id!: u32", 
                v.id AS "version_id!: u32",
                c.type AS "component_type"
            FROM components c
            LEFT JOIN component_versions v ON c.id = v.component_id
            WHERE v.id IN (SELECT value FROM json_each(?))
            "#,
            json_ids,
        )
        .fetch_all(&self.dbc)
        .await
        .with_context(|| {
            format!(
                "Failed to fetch components for {} version ID(s) [sample: {:?}]",
                version_ids.len(),
                version_ids.iter().take(5).collect::<Vec<_>>()
            )
        })
    }
}
