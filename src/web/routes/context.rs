use std::sync::Arc;

use axum_macros::FromRef;

use crate::{
    core::{
        components::repositories::{
            ComponentPayloadResolver, ComponentTypeResolver, ComponentsRepository,
        },
        documents::repositories::{
            DocumentLayoutsModifier, DocumentLifecycleManager, DocumentPublisher, DocumentsResolver,
        },
    },
    infra::rendering::services::DocumentExportService,
};

#[derive(Clone, FromRef)]
pub struct Context {
    component_repository: Arc<dyn ComponentsRepository>,
    component_type_resolver: Arc<dyn ComponentTypeResolver>,
    component_payload_resolver: Arc<dyn ComponentPayloadResolver>,

    document_lifecycle_manager: Arc<dyn DocumentLifecycleManager>,
    document_layouts_modifier: Arc<dyn DocumentLayoutsModifier>,
    documents_resolver: Arc<dyn DocumentsResolver>,
    document_publisher: Arc<dyn DocumentPublisher>,

    document_export_service: Arc<dyn DocumentExportService>,
}

impl Context {
    pub fn new(
        component_repository: Arc<dyn ComponentsRepository>,
        component_type_resolver: Arc<dyn ComponentTypeResolver>,
        component_payload_resolver: Arc<dyn ComponentPayloadResolver>,

        document_lifecycle_manager: Arc<dyn DocumentLifecycleManager>,
        document_layouts_modifier: Arc<dyn DocumentLayoutsModifier>,
        documents_resolver: Arc<dyn DocumentsResolver>,
        document_publisher: Arc<dyn DocumentPublisher>,

        document_export_service: Arc<dyn DocumentExportService>,
    ) -> Self {
        Self {
            component_repository,
            component_type_resolver,
            component_payload_resolver,

            document_lifecycle_manager,
            document_layouts_modifier,
            documents_resolver,
            document_publisher,

            document_export_service,
        }
    }
}
