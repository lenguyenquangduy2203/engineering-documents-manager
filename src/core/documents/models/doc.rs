use std::path::PathBuf;

use anyhow::anyhow;
use serde::{Deserialize, Serialize};

use crate::core::documents::models::doc_types::DocTypes;

#[derive(Deserialize, Serialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum DocStatus {
    Draft,
    Published,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Document {
    pub id: Option<u32>,
    pub doc_type: DocTypes,
    pub title: String,
    pub status: DocStatus,
    pub layout_version_ids: Vec<u32>,
    pub artifact_path: Option<PathBuf>,
}

impl Document {
    pub fn new(doc_type: DocTypes, title: &str) -> Self {
        Self {
            id: None,
            doc_type,
            title: title.into(),
            status: DocStatus::Draft,
            layout_version_ids: Vec::new(),
            artifact_path: None,
        }
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

    pub fn finalize_publication(&mut self, file_path: PathBuf) -> anyhow::Result<()> {
        match self.status {
            DocStatus::Draft => {
                if self.layout_version_ids.is_empty() {
                    return Err(anyhow!("Cannot publish an empty document."));
                }

                self.status = DocStatus::Published;
                self.artifact_path = Some(file_path);

                Ok(())
            }
            DocStatus::Published => Err(anyhow!("Document has already been published")),
        }
    }
}
