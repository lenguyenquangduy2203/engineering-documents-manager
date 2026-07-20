use anyhow::anyhow;

use crate::core::{components::repositories::ComponentTypeResolver, documents::repositories::{DocumentLayoutsModifier, DocumentsResolver}};


pub struct DocumentLayoutService;

impl DocumentLayoutService {
    pub async fn update_layouts(
        documents_resolver: &dyn DocumentsResolver,
        component_type_resolver: &dyn ComponentTypeResolver,
        document_layouts_modifier: &dyn DocumentLayoutsModifier,
        doc_id: u32,
        version_ids: &[u32],
    ) -> anyhow::Result<()> {
        let doc = documents_resolver.find_doc_by_id(doc_id).await?
            .ok_or_else(|| anyhow!("Document not found"))?;
        let component_refs = component_type_resolver.find_all_components_with_type_by_version_ids(version_ids).await?;

        if component_refs.len() != version_ids.len() {
            return Err(anyhow!("Incompatible number of component"));
        }

        // Assume number of components used in each document is small to medium (1, 100)
        // Choosing in-place sort for small memory allocation with small to medium additional steps
        let mut component_ids: Vec<u32> = component_refs.iter()
            .map(|c| c.id)
            .collect();
        let original_len = component_ids.len();

        component_ids.sort_unstable();
        component_ids.dedup();
        
        if component_ids.len() != original_len {
            return Err(anyhow!("Layout conflict: Cannot add multiple versions of the same root component to a single layout"));
        }

        let all_allowed = component_refs.iter()
            .map(|c| &c.component_type)
            .all(|t| doc.doc_type.is_allowed(t));
        
        if !all_allowed {
            return Err(anyhow!("Document type mismatch: One or more component layouts are barred from this document type"));
        }

        document_layouts_modifier.replace_layouts(doc_id, version_ids).await?;

        Ok(())
    }
}