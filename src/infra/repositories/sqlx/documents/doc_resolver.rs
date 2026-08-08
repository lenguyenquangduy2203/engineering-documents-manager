use async_trait::async_trait;

use crate::core::documents::{models::doc::Document, repositories::document_resolver::DocumentsResolver};

use super::SqliteDocumentsRepository;

#[async_trait]
impl DocumentsResolver for SqliteDocumentsRepository {
    async fn find_doc_by_id(&self, doc_id: u32) -> anyhow::Result<Option<Document>> {
        Self::fetch_opt_document_row(doc_id, &self.dbc).await
    }
}