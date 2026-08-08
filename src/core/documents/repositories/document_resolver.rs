use async_trait::async_trait;

use crate::core::documents::models::doc::Document;

#[async_trait]
pub trait DocumentsResolver: Send + Sync {
    async fn find_doc_by_id(&self, doc_id: u32) -> anyhow::Result<Option<Document>>;
}
