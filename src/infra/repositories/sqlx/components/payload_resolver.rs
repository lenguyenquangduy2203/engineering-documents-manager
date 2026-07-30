use anyhow::Context;
use async_trait::async_trait;

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
        
        sqlx::query_as!(
            IntermediateComponentPayloadRef,
            r#"
            SELECT 
                c.id AS "component_id!: u32", 
                v.id AS "version_id!: u32",
                v.data AS "payload_str!"
            FROM components c
            LEFT JOIN component_versions v ON c.id = v.component_id
            WHERE v.id IN (SELECT value FROM json_each(?))
            "#,
            json_ids,
        ).fetch_all(&*self.dbc).await
        .with_context(|| format!("Failed to fetch component payloads for version_ids: {version_ids:?}"))?
        .into_iter().map(ComponentPayloadRef::try_from).collect()
    }
}

struct IntermediateComponentPayloadRef {
    pub component_id: u32,
    pub version_id: u32,
    pub payload_str: String,
}

impl TryFrom<IntermediateComponentPayloadRef> for ComponentPayloadRef {
    type Error = anyhow::Error;

    fn try_from(raw: IntermediateComponentPayloadRef) -> Result<Self, Self::Error> {
        let payload: ComponentPayload = serde_json::from_str(&raw.payload_str)
            .with_context(|| format!("Failed to parse ComponentPayload JSON for component_id={}", raw.component_id))?;

        Ok(Self {
            component_id: raw.component_id,
            version_id: raw.version_id,
            payload,
        })
    }
}
