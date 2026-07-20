use std::sync::Arc;

use axum_macros::FromRef;

use crate::core::{
    components::repositories::{ComponentTypeResolver, ComponentsRepository},
    documents::repositories::{
        DocumentLayoutsModifier, DocumentLifecycleManager, DocumentsResolver,
    },
};

#[derive(Clone, FromRef)]
pub struct Context {
    component_repository: Arc<dyn ComponentsRepository>,
    component_type_resolver: Arc<dyn ComponentTypeResolver>,
    document_lifecycle_manager: Arc<dyn DocumentLifecycleManager>,
    document_layouts_modifier: Arc<dyn DocumentLayoutsModifier>,
    documents_resolver: Arc<dyn DocumentsResolver>,
}

impl Context {
    pub fn new(
        component_repository: Arc<dyn ComponentsRepository>,
        component_type_resolver: Arc<dyn ComponentTypeResolver>,
        document_lifecycle_manager: Arc<dyn DocumentLifecycleManager>,
        document_layouts_modifier: Arc<dyn DocumentLayoutsModifier>,
        documents_resolver: Arc<dyn DocumentsResolver>,
    ) -> Self {
        Self {
            component_repository,
            component_type_resolver,
            document_lifecycle_manager,
            document_layouts_modifier,
            documents_resolver,
        }
    }
}
