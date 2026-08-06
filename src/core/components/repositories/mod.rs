pub mod components_repository;

use async_trait::async_trait;

use crate::core::components::models::payload::ComponentPayload;

pub struct ComponentRef {
    pub id: u32,
    pub version_id: u32,
    pub component_type: String,
}

#[async_trait]
pub trait ComponentTypeResolver: Send + Sync {
    async fn find_all_components_with_type_by_version_ids(
        &self,
        version_ids: &[u32]
    ) -> anyhow::Result<Vec<ComponentRef>>;
}

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