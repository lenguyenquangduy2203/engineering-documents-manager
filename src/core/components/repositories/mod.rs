pub mod components_repository;
pub mod component_type_resolver;

use async_trait::async_trait;

use crate::core::components::models::payload::ComponentPayload;

pub struct ComponentPayloadRef {
    pub component_id: u32,
    pub version_id: u32,
    pub payload: ComponentPayload,
}

#[async_trait]
pub trait ComponentPayloadResolver: Send + Sync {
    async fn find_all_components_with_payload_by_version_ids(
        &self,
        doc_id: u32,
        version_ids: &[u32]
    ) -> anyhow::Result<Vec<ComponentPayloadRef>>;
}