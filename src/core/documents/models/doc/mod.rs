mod lifecycle;
mod mutations;

use serde::{Deserialize, Serialize};

use crate::core::documents::models::{doc_status::DocStatus, doc_types::DocTypes};

#[derive(Deserialize, Serialize, Clone)]
pub struct Document {
    pub id: Option<u32>,
    pub doc_type: DocTypes,
    pub title: String,
    pub status: DocStatus,
    pub layout_version_ids: Vec<u32>,
}

impl Document {
    pub fn new(doc_type: DocTypes, title: &str) -> Self {
        Self {
            id: None,
            doc_type,
            title: title.into(),
            status: DocStatus::Draft,
            layout_version_ids: Vec::new(),
        }
    }
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct DocumentMetadataForUpdate {
    pub id: u32,
    pub title: Option<String>,
}

pub struct ComponentSummary<'a> {
    pub root_id: u32,
    pub component_type: &'a str,
}
