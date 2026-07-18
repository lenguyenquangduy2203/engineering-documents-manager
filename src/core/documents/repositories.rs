use async_trait::async_trait;
use serde::Deserialize;

use crate::core::documents::models::doc::{DocStatus, Document};

#[async_trait]
pub trait DocumentsResolver: Send + Sync {
    async fn find_doc_by_id(&self, doc_id: u32) -> anyhow::Result<Option<Document>>;
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct DocumentFilterQuery {
    pub title: Option<String>,
    pub doc_type: Option<String>,
    pub status: Option<DocStatus>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[async_trait]
pub trait DocumentLifecycleManager: DocumentsResolver {
    async fn create_new(&self, document: &Document) -> anyhow::Result<u32>;
    async fn find_all_docs(&self, filter: DocumentFilterQuery) -> anyhow::Result<Vec<Document>>;

    async fn update_doc(&self, incoming_document: Document) -> anyhow::Result<Option<Document>>;
    async fn remove_doc_with_all_layouts_by_id(&self, doc_id: u32) -> anyhow::Result<()>;
}

#[async_trait]
pub trait DocumentLayoutsModifier: Send + Sync {
    async fn replace_layouts(&self, doc_id: u32, version_ids: &[u32]) -> anyhow::Result<()>;
}
