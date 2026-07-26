use async_trait::async_trait;

use crate::core::components::models::{payload::ComponentPayload, wrapper::{Component, ComponentError}};

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Default)]
pub struct ComponentFilterQuery {
    // Basic text search against component titles
    pub title: Option<String>,
    
    // Group filter (e.g., "DesignSpec")
    pub group: Option<String>,
    
    // Subtype filter (e.g., "DecisionRecord")
    pub subtype: Option<String>,
    
    // Pagination controls
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, thiserror::Error)]
pub enum UpdateComponentError {
    #[error(transparent)]
    Domain(#[from] ComponentError),

    #[error("Database failure: {0}")]
    Database(#[from] sqlx::Error),

    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum RemoveComponentError {
    #[error("Database query failed: {0}")]
    Database(#[from] sqlx::Error),

    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

#[async_trait]
pub trait ComponentsRepository: Send + Sync {
    async fn create_new(&self, component: &Component<ComponentPayload>) -> anyhow::Result<u32>;
    async fn find_all_latest_version(
        &self, 
        filter: ComponentFilterQuery
    ) -> anyhow::Result<Vec<Component<ComponentPayload>>>;
    async fn find_latest_version_by_id(&self, component_id: u32) -> anyhow::Result<Option<Component<ComponentPayload>>>;
    async fn update_component(
        &self, 
        component_id: u32,
        incoming_component: Component<ComponentPayload>
    ) -> std::result::Result<Option<Component<ComponentPayload>>, UpdateComponentError>;
    async fn remove_component_with_all_versions_by_id(
        &self, 
        component_id: u32
    ) -> std::result::Result<bool, RemoveComponentError>;
}


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
        version_ids: &[u32]
    ) -> anyhow::Result<Vec<ComponentPayloadRef>>;
}