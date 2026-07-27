use anyhow::anyhow;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::core::documents::models::{
    doc_status::{DocStatus, DocStatusError},
    doc_types::DocTypes,
};

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
