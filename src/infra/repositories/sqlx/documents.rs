use std::sync::Arc;

use anyhow::anyhow;
use async_trait::async_trait;
use sqlx::{Executor, Pool, QueryBuilder, Row, Sqlite, sqlite::SqliteRow};

use crate::core::documents::{models::{ doc::{DocStatus, Document, DocumentMetadataForUpdate}, doc_types::DocTypes}, queries::document::DocumentQuery, repositories::{DocumentFilterQuery, DocumentLayoutsModifier, DocumentLifecycleManager, DocumentsResolver}};

impl TryFrom<&str> for DocStatus {
    type Error = anyhow::Error;

    fn try_from(status: &str) -> Result<Self, Self::Error> {
        serde_json::from_value(
            serde_json::Value::String(status.to_string())
        ).map_err(|_| anyhow!("Unknown document status"))
    }
}

impl TryFrom<SqliteRow> for Document {
    type Error = anyhow::Error;

    fn try_from(row: SqliteRow) -> Result<Self, Self::Error> {
        let layout_ids_str: Option<String> = row.get("layout_version_ids");
        let layout_version_ids = match layout_ids_str {
            Some(s) if !s.is_empty() => s
                .split(',')
                .map(|val| val.trim().parse::<u32>().map_err(|e| anyhow!(e)))
                .collect::<Result<Vec<u32>, _>>()?,
            _ => Vec::new(),
        };

        Ok(Self { 
            id: Some(row.get("id")), 
            doc_type: DocTypes::try_from(row.get::<&str, _>("type"))?, 
            title: row.get("title"), 
            status: DocStatus::try_from(row.get::<&str, _>("status"))?, 
            layout_version_ids
        })
    }
}

pub struct SqliteDocumentsRepository {
    dbc: Arc<Pool<Sqlite>>,
}

impl SqliteDocumentsRepository {
    pub fn new(dbc: Arc<Pool<Sqlite>>) -> Self {
        Self { dbc }
    }

    async fn fetch_opt_document_row<'c, E: Executor<'c, Database = Sqlite>>(
        doc_id: u32,
        executor: E
    ) -> anyhow::Result<Option<SqliteRow>> {
        Ok(sqlx::query(
            r#"
            SELECT 
                d.id, 
                d.type, 
                d.title, 
                d.status,
                GROUP_CONCAT(l.component_version_id ORDER BY l.position ASC) AS layout_version_ids
            FROM documents d
            LEFT JOIN document_layouts l ON d.id = l.document_id
            WHERE d.id = $1
            GROUP BY d.id
            "#
        )
        .bind(doc_id)
        .fetch_optional(executor)
        .await?)
    }
}

#[async_trait]
impl DocumentsResolver for SqliteDocumentsRepository {
    async fn find_doc_by_id(&self, doc_id: u32) -> anyhow::Result<Option<Document>> {
        match Self::fetch_opt_document_row(doc_id, &*self.dbc).await? {
            Some(row) => Ok(Some(Document::try_from(row)?)),
            None => Ok(None),
        }
    }
}

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

    async fn update_doc(&self, incoming_document: DocumentMetadataForUpdate) -> anyhow::Result<Option<Document>> {
        let doc_id = incoming_document.id;
        let mut tx = self.dbc.begin_with("BEGIN IMMEDIATE").await?;
        let row = match Self::fetch_opt_document_row(doc_id, &mut *tx).await? {
            Some(r) => r,
            None => return Ok(None),
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

        Ok(Some(to_be_updated_doc))
    }

    async fn remove_doc_with_all_layouts_by_id(&self, doc_id: u32) -> anyhow::Result<()> {
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
            return Err(anyhow!("Document with ID {} does not exist", doc_id));
        }

        Ok(())
    }
}

#[async_trait]
impl DocumentLayoutsModifier for SqliteDocumentsRepository {
    async fn replace_layouts(&self, doc_id: u32, version_ids: &[u32]) -> anyhow::Result<()> {
        let json_ids = serde_json::to_string(version_ids)?;
        let mut tx = self.dbc.begin().await?;

        sqlx::query!(
            r#"
            DELETE FROM document_layouts
            WHERE document_id = ?
                AND component_version_id IN (
                    SELECT value FROM json_each(?)
                )
            "#,
            doc_id,
            json_ids,
        )
        .execute(&mut *tx)
        .await?;

        if version_ids.is_empty() {
            tx.commit().await?;

            return Ok(());
        }

        let values: Vec<(u32, u32, usize)> = version_ids.iter()
            .enumerate()
            .map(|(index, id)| (doc_id, *id, index))
            .collect();
        let json_values = serde_json::to_string(&values)?;

        sqlx::query!(
            r#"
            INSERT INTO document_layouts (document_id, component_version_id, position)
            SELECT
                json_extract(value, '$[0]'),
                json_extract(value, '$[1]'),
                json_extract(value, '$[2]')
            FROM json_each(?)
            "#,
            json_values
        )
        .execute(&mut *tx)
        .await?;
            
        tx.commit().await?;

        Ok(())
    }
}