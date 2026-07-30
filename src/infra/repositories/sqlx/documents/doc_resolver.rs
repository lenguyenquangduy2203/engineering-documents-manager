use async_trait::async_trait;

use crate::core::documents::{models::doc::Document, repositories::DocumentsResolver};

use super::SqliteDocumentsRepository;

#[async_trait]
impl DocumentsResolver for SqliteDocumentsRepository {
    async fn find_doc_by_id(&self, doc_id: u32) -> anyhow::Result<Option<Document>> {
        match Self::fetch_opt_document_row(doc_id, &*self.dbc).await? {
            Some(row) => Ok(Some(Document::try_from(row)?)),
            None => Ok(None),
        }
    }
}