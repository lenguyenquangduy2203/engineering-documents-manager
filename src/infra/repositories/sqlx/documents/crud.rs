use async_trait::async_trait;
use sqlx::{QueryBuilder, Sqlite};

use crate::core::documents::{models::doc::{Document, DocumentMetadataForUpdate}, queries::document::DocumentQuery, repositories::{DocumentFilterQuery, DocumentLifecycleManager, DocumentUpdateError}};

use super::SqliteDocumentsRepository;

#[async_trait]
impl DocumentLifecycleManager for SqliteDocumentsRepository {
    async fn create_new(&self, document: &Document) -> anyhow::Result<u32> {
        let res = sqlx::query!(
            r#"
            INSERT INTO documents (type, title, status)
            VALUES (?, ?, ?)
            "#,
            &document.doc_type.to_string(),
            document.title,
            &document.status.to_string(),
        )
        .execute(&*self.dbc)
        .await?;

        Ok(res.last_insert_rowid() as u32)
    }

    async fn find_all_docs(&self, filter: DocumentFilterQuery) -> anyhow::Result<Vec<Document>> {
        let mut qb: QueryBuilder<Sqlite> = QueryBuilder::new(
            r#"
            SELECT
                d.id, 
                d.type, 
                d.title, 
                d.status,
                GROUP_CONCAT(l.component_version_id ORDER BY l.position ASC) AS layout_version_ids
            FROM documents d
            LEFT JOIN document_layouts l
                ON d.id = l.document_id
            WHERE 1=1
            "#
        );
        let specs = DocumentQuery::new(filter);
        specs.apply(&mut qb);
        let query = qb.build();
        let rows = query.fetch_all(&*self.dbc).await?;

        rows.into_iter()
            .map(Document::try_from)
            .collect()
    }

    async fn update_doc(
        &self, 
        incoming_document: DocumentMetadataForUpdate
    ) -> std::result::Result<Option<Document>, DocumentUpdateError> {
        let doc_id = incoming_document.id;
        let mut tx = self.dbc.begin_with("BEGIN IMMEDIATE").await?;
        let row = match Self::fetch_opt_document_row(doc_id, &mut *tx).await? {
            Some(r) => r,
            None => return Result::Ok(None),
        };

        let mut to_be_updated_doc = Document::try_from(row)?;
        to_be_updated_doc.apply_metadata_changes(incoming_document)?;

        sqlx::query!(
            r#"
            UPDATE documents
            SET
                title = ?
            WHERE id = ?
            "#,
            to_be_updated_doc.title,
            doc_id,
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Result::Ok(Some(to_be_updated_doc))
    }

    async fn remove_doc_with_all_layouts_by_id(&self, doc_id: u32) -> anyhow::Result<bool> {
        let res = sqlx::query!(
            r#"
            DELETE FROM documents
            WHERE id = ?
            "#,
            doc_id
        )
        .execute(&*self.dbc)
        .await?;

        if res.rows_affected() == 0 {
            return Ok(false);
        }

        Ok(true)
    }
}
