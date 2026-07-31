mod doc_resolver;
mod crud;
mod layouts;
mod publication;
mod rows;

use std::sync::Arc;

use anyhow::Ok;
use sqlx::{Executor, Pool, Sqlite};

use crate::core::documents::models::doc::Document;
use crate::infra::repositories::sqlx::documents::rows::doc::DocRow;

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
    ) -> anyhow::Result<Option<Document>> {
        Ok(sqlx::query_as!(
            DocRow,
            r#"
            SELECT 
                d.id AS "id!: u32", 
                d.type AS "doc_type", 
                d.title AS "title", 
                d.status AS "status",
                GROUP_CONCAT(l.component_version_id ORDER BY l.position ASC) AS "layout_version_ids?: String"
            FROM documents d
            LEFT JOIN document_layouts l ON d.id = l.document_id
            WHERE d.id = ?
            GROUP BY d.id
            "#,
            doc_id,
        )
        .fetch_optional(executor).await?
        .map(DocRow::try_into).transpose()?)
    }
}
