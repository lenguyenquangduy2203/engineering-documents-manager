pub mod document_resolver;
pub mod document_lifecycle_manager;
pub mod document_layouts_modifier;

use async_trait::async_trait;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Default)]
pub struct DocumentPublishParams {
    pub id: u32,
    pub status: String,
    pub published_at: Option<DateTime<Utc>>,
}

#[async_trait]
pub trait DocumentPublisher: Send + Sync {
    async fn update_doc_publication(&self, params: DocumentPublishParams) -> anyhow::Result<()>;
}