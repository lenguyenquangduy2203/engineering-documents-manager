mod doc_resolver;
mod crud;
mod layouts;
mod publication;

use std::{sync::Arc, result::Result};

use anyhow::{Ok, anyhow};
use sqlx::{Executor, Pool, Row, Sqlite, sqlite::SqliteRow};

use crate::core::documents::models::{ doc::Document, doc_types::DocTypes, doc_status::DocStatus};

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
