use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use thiserror::Error;

use crate::core::documents::{
    models::{
        doc::{Document, DocumentMetadataForUpdate}, 
        doc_types::DocTypes, 
        doc_status::DocStatus
    }, 
    errors::doc_metadata::DocumentMetadataError
};

#[async_trait]
pub trait DocumentsResolver: Send + Sync {
    async fn find_doc_by_id(&self, doc_id: u32) -> anyhow::Result<Option<Document>>;
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct DocumentFilterQuery {
    pub title: Option<String>,
    pub doc_type: Option<DocTypes>,
    pub status: Option<DocStatus>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Error)]
pub enum DocumentUpdateError {
    #[error(transparent)]
    Domain(#[from] DocumentMetadataError),

    #[error("Database failure: {0}")]
    Database(#[from] sqlx::Error),

    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

#[async_trait]
pub trait DocumentLifecycleManager: DocumentsResolver {
    async fn create_new(&self, document: &Document) -> anyhow::Result<u32>;
    async fn find_all_docs(&self, filter: DocumentFilterQuery) -> anyhow::Result<Vec<Document>>;

    async fn update_doc(&self, incoming_document: DocumentMetadataForUpdate) -> std::result::Result<Option<Document>, DocumentUpdateError>;
    async fn remove_doc_with_all_layouts_by_id(&self, doc_id: u32) -> anyhow::Result<bool>;
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