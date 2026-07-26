use std::fmt::Display;

use anyhow::anyhow;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::core::documents::models::doc_types::DocTypes;

#[derive(Deserialize, Serialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum DocStatus {
    Draft,
    Publishing,
    Published,
    Failed,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DocStatusError {
    #[error("Document is already in the process of publishing")]
    AlreadyPublishing,

    #[error("Published documents cannot re-enter publishing")]
    AlreadyPublished,

    #[error("Cannot directly finalize a failed document without restarting publish")]
    InvalidFinalizeFromFailed,

    #[error("Cannot transition to Failed state from state '{0}'")]
    InvalidFailureTransition(DocStatus),

    #[error("Cannot modify layout while document is in state '{0}'")]
    LayoutModificationNotAllowed(DocStatus),
}

impl DocStatus {
    pub fn transition_to_publishing(self) -> Result<Self, DocStatusError> {
        match self {
            DocStatus::Draft | DocStatus::Failed => Ok(DocStatus::Publishing),
            DocStatus::Publishing => Err(DocStatusError::AlreadyPublishing),
            DocStatus::Published => Err(DocStatusError::AlreadyPublished),
        }
    }

    pub fn transition_to_published(self) -> Result<Self, DocStatusError> {
        match self {
            DocStatus::Publishing | DocStatus::Draft => Ok(DocStatus::Published),
            DocStatus::Published => Err(DocStatusError::AlreadyPublished),
            DocStatus::Failed => Err(DocStatusError::InvalidFinalizeFromFailed),
        }
    }

    pub fn transition_to_failed(self) -> Result<Self, DocStatusError> {
        match self {
            DocStatus::Publishing => Ok(DocStatus::Failed),
            other => Err(DocStatusError::InvalidFailureTransition(other)),
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

pub struct ComponentSummary<'a> {
    pub root_id: u32,
    pub component_type: &'a str,
}

#[derive(Debug, Error)]
pub enum DocumentMetadataError {
    #[error("Invalid metadata: {message}")]
    InvalidMetadata { message: String },
}

#[derive(Debug, Error)]
pub enum DocumentLayoutError {
    #[error(transparent)]
    Status(#[from] DocStatusError),

    #[error("Incompatible number of components requested: expected {expected}, found {found}")]
    IncompatibleComponentCount { expected: usize, found: usize },

    #[error("Layout conflict: Cannot add multiple versions of the same root component to a single layout")]
    DuplicateRootComponents,

    #[error(
        "Document type mismatch: One or more component layouts are barred from this document type"
    )]
    TypeMismatch,
}

#[derive(Debug, Error)]
pub enum DocumentPublishingError {
    #[error("Cannot publish an empty document layout")]
    EmptyLayout,

    #[error(transparent)]
    Status(#[from] DocStatusError),
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
    ) -> std::result::Result<(), DocumentMetadataError> {
        if let Some(new_title) = incoming_document.title {
            let trimmed = new_title.trim();
            if trimmed.is_empty() {
                return std::result::Result::Err(DocumentMetadataError::InvalidMetadata {
                    message: "Document title cannot be empty.".into(),
                });
            }

            self.title = trimmed.to_string();
        }

        Ok(())
    }

    pub fn update_layout(
        &mut self,
        version_ids: Vec<u32>,
        components: &[ComponentSummary<'_>],
    ) -> Result<(), DocumentLayoutError> {
        // 1. Check layout status permission
        if !self.status.can_modify_layout() {
            return Err(DocStatusError::LayoutModificationNotAllowed(self.status).into());
        }

        // 2. Validate all requested version IDs exist
        if components.len() != version_ids.len() {
            return Err(DocumentLayoutError::IncompatibleComponentCount {
                expected: version_ids.len(),
                found: components.len(),
            });
        }

        // 3. Validate duplicate root components
        let mut root_ids: Vec<u32> = components.iter().map(|c| c.root_id).collect();
        let original_len = root_ids.len();

        root_ids.sort_unstable();
        root_ids.dedup();

        if root_ids.len() != original_len {
            return Err(DocumentLayoutError::DuplicateRootComponents);
        }

        // 4. Validate component type permissions against document type
        let all_allowed = components
            .iter()
            .all(|c| self.doc_type.is_allowed(c.component_type));

        if !all_allowed {
            return Err(DocumentLayoutError::TypeMismatch);
        }

        // 5. Apply state change
        self.layout_version_ids = version_ids;

        Ok(())
    }

    pub fn marked_for_publishing(&mut self) -> Result<(), DocumentPublishingError> {
        if self.layout_version_ids.is_empty() {
            return Err(DocumentPublishingError::EmptyLayout);
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
