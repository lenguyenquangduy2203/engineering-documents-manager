use async_trait::async_trait;

use crate::core::components::models::{payload::ComponentPayload, wrapper::Component};

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

#[async_trait]
pub trait ComponentsRepository: Send + Sync {
    async fn create_new(&self, component: &Component<ComponentPayload>) -> anyhow::Result<u32>;
    async fn find_all_latest_version(
        &self, 
        filter: ComponentFilterQuery
    ) -> anyhow::Result<Vec<Component<ComponentPayload>>>;
}
