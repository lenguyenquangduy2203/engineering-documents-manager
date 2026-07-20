use std::fmt::Display;

use anyhow::anyhow;
use serde::{Deserialize, Serialize};

use crate::core::documents::models::doc_types::DocTypes;

#[derive(Deserialize, Serialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum DocStatus {
    Draft,
    Published,
}

impl From<&DocStatus> for String {
    fn from(value: &DocStatus) -> Self {
        match value {
            DocStatus::Draft => "DRAFT".into(),
            DocStatus::Published => "PUBLISHED".into(),
        }
    }
}

impl Display for DocStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content: String = self.into();
        write!(f, "{}", content)
    }
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct DocumentMetadataForUpdate {
    pub id: u32,
    pub title: Option<String>,
}

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

    pub fn apply_metadata_changes(
        &mut self,
        incoming_document: DocumentMetadataForUpdate,
    ) -> anyhow::Result<()> {
        if let Some(new_title) = incoming_document.title {
            let trimmed = new_title.trim();
            if trimmed.is_empty() {
                return Err(anyhow!("Document title cannot be empty."));
            }

            self.title = trimmed.to_string();
        }

        Ok(())
    }

    pub fn update_layout(&mut self, new_version_ids: Vec<u32>) -> anyhow::Result<()> {
        if self.status == DocStatus::Published {
            return Err(anyhow!(
                "Cannot alter the structural layout of a published document."
            ));
        }

        self.layout_version_ids = new_version_ids;

        Ok(())
    }

    pub fn finalize_publication(&mut self) -> anyhow::Result<()> {
        match self.status {
            DocStatus::Draft => {
                if self.layout_version_ids.is_empty() {
                    return Err(anyhow!("Cannot publish an empty document."));
                }

                self.status = DocStatus::Published;

                Ok(())
            }
            DocStatus::Published => Err(anyhow!("Document has already been published")),
        }
    }
}
