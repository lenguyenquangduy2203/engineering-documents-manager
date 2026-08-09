use async_trait::async_trait;
use anyhow::anyhow;

use crate::core::documents::repositories::document_publisher::{DocumentPublishParams, DocumentPublisher};

use super::SqliteDocumentsRepository;

#[async_trait]
impl DocumentPublisher for SqliteDocumentsRepository {
    async fn update_doc_publication(&self, params: DocumentPublishParams) -> anyhow::Result<()> {
        let doc_id = params.id;
        let mut tx = self.dbc.begin_with("BEGIN IMMEDIATE").await?;
        Self::fetch_opt_document_row(doc_id, &mut *tx).await?
            .ok_or_else(|| anyhow!("Document {} is not found", doc_id))?;

        let formatted_published_at = match params.published_at {
            Some(published_at) => Some(published_at.format("%Y-%m-%d %H:%M:%S").to_string()),
            None => None,
        };

        sqlx::query!(
            r#"
            UPDATE documents
            SET
                status = ?,
                is_completed = ?,
                published_at = ?
            WHERE id = ?
            "#,
            params.status,
            1,
            formatted_published_at,
            doc_id,
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(())
    }
}
