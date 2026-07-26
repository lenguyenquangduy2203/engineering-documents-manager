use thiserror::Error;

use crate::core::{components::repositories::ComponentTypeResolver, documents::{models::doc::{ComponentSummary, DocumentLayoutError}, repositories::{DocumentLayoutsModifier, DocumentsResolver}}};

type DocumentLayoutServiceDepsTuple<'a> = (
    &'a dyn DocumentsResolver,
    &'a dyn ComponentTypeResolver,
    &'a dyn DocumentLayoutsModifier,
);

#[derive(Debug, Error)]
pub enum DocumentLayoutServiceError {
    #[error("Document with ID {0} was not found")]
    DocumentNotFound(u32),

    #[error(transparent)]
    Domain(#[from] DocumentLayoutError),

    #[error("Infrastructure failure: {0}")]
    Internal(#[from] anyhow::Error),
}

pub struct DocumentLayoutService;

impl DocumentLayoutService {
    pub async fn update_layouts(
        (documents_resolver, component_type_resolver, document_layouts_modifier): DocumentLayoutServiceDepsTuple<'_>,
        doc_id: u32,
        version_ids: &[u32],
    ) -> Result<(), DocumentLayoutServiceError> {
        let mut doc = documents_resolver
            .find_doc_by_id(doc_id).await?
            .ok_or(DocumentLayoutServiceError::DocumentNotFound(doc_id))?;

        let component_refs = component_type_resolver
            .find_all_components_with_type_by_version_ids(version_ids).await?;

        // Map repository structs to domain value objects
        let summaries: Vec<ComponentSummary> = component_refs
            .iter()
            .map(|c| ComponentSummary {
                root_id: c.id,
                component_type: &c.component_type,
            })
            .collect();

        // All business logic runs inside the aggregate!
        doc.update_layout(version_ids.to_vec(), &summaries)?;
        document_layouts_modifier.replace_layouts(doc_id, &doc.layout_version_ids).await?;

        Ok(())
    }
}