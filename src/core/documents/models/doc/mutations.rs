use crate::core::documents::errors::{
    doc_layout::DocumentLayoutError, doc_metadata::DocumentMetadataError,
    doc_status::DocStatusError,
};

use super::{ComponentSummary, Document, DocumentMetadataForUpdate};

impl Document {
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
}
