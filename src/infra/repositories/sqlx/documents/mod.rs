mod doc_resolver;
mod crud;
mod layouts;
mod publication;

use anyhow::Ok;
use sqlx::error::BoxDynError;
use sqlx::{Database, Decode, Executor, Sqlite, SqlitePool, Type};

use crate::core::documents::models::layout_version_ids::LayoutVersionIds;
use crate::core::documents::models::{doc::Document, doc_types::DocTypes, doc_status::DocStatus};

pub struct SqliteDocumentsRepository {
    dbc: SqlitePool,
}

impl SqliteDocumentsRepository {
    pub fn new(dbc: SqlitePool) -> Self {
        Self { dbc }
    }

    async fn fetch_opt_document_row<'c, E: Executor<'c, Database = Sqlite>>(
        doc_id: u32,
        executor: E
    ) -> anyhow::Result<Option<Document>> {
        Ok(sqlx::query_as!(
            Document,
            r#"
            SELECT 
                d.id AS "id!: u32", 
                d.type AS "doc_type: DocTypes", 
                d.title AS "title", 
                d.status AS "status: DocStatus",
                GROUP_CONCAT(l.component_version_id ORDER BY l.position ASC) AS "layout_version_ids?: String"
            FROM documents d
            LEFT JOIN document_layouts l ON d.id = l.document_id
            WHERE d.id = ?
            GROUP BY d.id
            "#,
            doc_id,
        )
        .fetch_optional(executor).await?)
    }
}

// 1. Tell SQLx this domain type maps to a text column
impl<DB: Database> Type<DB> for LayoutVersionIds
where
    String: Type<DB>,
{
    fn type_info() -> DB::TypeInfo {
        <String as Type<DB>>::type_info()
    }

    fn compatible(ty: &DB::TypeInfo) -> bool {
        <String as Type<DB>>::compatible(ty)
    }
}

// 2. Instruct SQLx how to decode the DB String directly into LayoutVersionIds
impl<'r, DB: Database> Decode<'r, DB> for LayoutVersionIds
where
    Option<String>: Decode<'r, DB>,
{
    fn decode(value: DB::ValueRef<'r>) -> Result<Self, BoxDynError> {
        let raw_str = <Option<String> as Decode<DB>>::decode(value)?;

        let ids = match raw_str {
            Some(s) if !s.trim().is_empty() => s
                .split(',')
                .map(|val| {
                    val.trim()
                        .parse::<u32>()
                        .map_err(|e| -> BoxDynError { 
                            format!("Failed to parse layout version id '{val}': {e}").into() 
                        })
                })
                .collect::<Result<Vec<u32>, BoxDynError>>()?,
            _ => Vec::new(),
        };

        std::result::Result::Ok(LayoutVersionIds(ids))
    }
}