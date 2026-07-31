use anyhow::{anyhow, Ok};
use sqlx::prelude::FromRow;

use crate::core::documents::models::{doc::Document, doc_status::DocStatus, doc_types::DocTypes};

#[derive(FromRow)]
pub struct DocRow {
    pub id: u32,
    pub doc_type: String,
    pub title: String,
    pub status: String,
    pub layout_version_ids: Option<String>,
}

impl DocRow {
    fn get_layout_version_ids_as_vec(&self) -> anyhow::Result<Vec<u32>> {
        match &self.layout_version_ids {
            Some(s) if !s.is_empty() => s
                .split(',')
                .map(|val| val.trim().parse::<u32>().map_err(|e| anyhow!(e)))
                .collect::<Result<Vec<u32>, _>>(),
            _ => Ok(Vec::new()),
        }
    }
}

impl TryFrom<&str> for DocStatus {
    type Error = anyhow::Error;

    fn try_from(status: &str) -> Result<Self, Self::Error> {
        serde_json::from_value(serde_json::Value::String(status.to_string()))
            .map_err(|_| anyhow!("Unknown document status"))
    }
}

impl TryFrom<DocRow> for Document {
    type Error = anyhow::Error;

    fn try_from(row: DocRow) -> Result<Self, Self::Error> {
        Ok(Document {
            id: Some(row.id),
            doc_type: DocTypes::try_from(row.doc_type.as_str())?,
            title: row.title.clone(),
            status: DocStatus::try_from(row.status.as_str())?,
            layout_version_ids: row.get_layout_version_ids_as_vec()?,
        })
    }
}
