pub mod document_lifecycle_manager;

use async_trait::async_trait;
use chrono::{DateTime, Utc};

use crate::core::documents::models::doc::Document;

#[async_trait]
pub trait DocumentsResolver: Send + Sync {
    async fn find_doc_by_id(&self, doc_id: u32) -> anyhow::Result<Option<Document>>;
}

#[async_trait]
pub trait DocumentLayoutsModifier: Send + Sync {
    async fn replace_layouts(&self, doc_id: u32, version_ids: &[u32]) -> anyhow::Result<()>;
}

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