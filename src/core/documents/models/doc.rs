use std::fmt::Display;

use anyhow::anyhow;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::core::documents::models::doc_types::DocTypes;

#[derive(Deserialize, Serialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum DocStatus {
    Draft,
    Publishing,
    Published,
    Failed,
}

impl DocStatus {
    /// Validates and returns the next status when initiating a publish operation
    pub fn transition_to_publishing(self) -> anyhow::Result<Self> {
        match self {
            DocStatus::Draft | DocStatus::Failed => Ok(DocStatus::Publishing),
            DocStatus::Publishing => {
                Err(anyhow!("Document is already in the process of publishing."))
            }
            DocStatus::Published => Err(anyhow!("Published documents cannot re-enter publishing.")),
        }
    }

    /// Validates and returns the next status when finalizing a publish operation
    pub fn transition_to_published(self) -> anyhow::Result<Self> {
        match self {
            DocStatus::Publishing | DocStatus::Draft => Ok(DocStatus::Published),
            DocStatus::Published => Err(anyhow!("Document is already published.")),
            DocStatus::Failed => Err(anyhow!(
                "Cannot directly finalize a failed document without restarting publish."
            )),
        }
    }

    /// Validates and returns the next status when a failure occurs
    pub fn transition_to_failed(self) -> anyhow::Result<Self> {
        match self {
            DocStatus::Publishing => Ok(DocStatus::Failed),
            DocStatus::Draft | DocStatus::Published | DocStatus::Failed => {
                Err(anyhow!("Cannot transition to Failed from state {:?}", self))
            }
        }
    }

    pub fn can_modify_layout(self) -> bool {
        matches!(self, DocStatus::Draft | DocStatus::Failed)
    }
}

impl From<&DocStatus> for String {
    fn from(value: &DocStatus) -> Self {
        match value {
            DocStatus::Draft => "DRAFT".into(),
            DocStatus::Publishing => "PUBLISHING".into(),
            DocStatus::Published => "PUBLISHED".into(),
            DocStatus::Failed => "FAILED".into(),
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
        if !self.status.can_modify_layout() {
            return Err(anyhow!(
                "Cannot alter layout while document is in '{}' state.",
                self.status
            ));
        }

        self.layout_version_ids = new_version_ids;

        Ok(())
    }

    pub fn marked_for_publishing(&mut self) -> anyhow::Result<()> {
        if self.layout_version_ids.is_empty() {
            return Err(anyhow!("Cannot publish an empty document layout."));
        }

        self.status = self.status.transition_to_publishing()?;

        Ok(())
    }

    pub fn finalize_publication(&mut self) -> anyhow::Result<DateTime<Utc>> {
        if self.layout_version_ids.is_empty() {
            return Err(anyhow!("Cannot publish an empty document layout."));
        }

        self.status = self.status.transition_to_published()?;

        Ok(Utc::now())
    }

    pub fn mark_failed(&mut self) -> anyhow::Result<()> {
        self.status = self.status.transition_to_failed()?;

        Ok(())
    }
}
