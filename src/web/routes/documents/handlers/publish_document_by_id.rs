use std::sync::Arc;

use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse};

use crate::{
    core::{
        applications::{
            services::publish_document::DocumentPublishingService, 
            errors::publish_doc::PublishDocumentError
        }, 
        components::repositories::ComponentPayloadResolver, 
        documents::repositories::{DocumentPublisher, DocumentsResolver}
    }, 
    infra::rendering::services::DocumentExportService
};

pub async fn handler(
    State(documents_resolver): State<Arc<dyn DocumentsResolver>>,
    State(component_payload_resolver): State<Arc<dyn ComponentPayloadResolver>>,
    State(document_publisher): State<Arc<dyn DocumentPublisher>>,
    State(document_export_service): State<Arc<dyn DocumentExportService>>,
    Path(id): Path<u32>
) -> impl IntoResponse {
    let deps = (
        documents_resolver.clone(),
        component_payload_resolver.clone(),
        document_publisher.clone(),
        document_export_service.clone(),
    );

    match DocumentPublishingService::publish_document(deps, id).await {
        Ok(()) => (StatusCode::ACCEPTED).into_response(),
        
        Err(PublishDocumentError::NotFound(id)) => (
            StatusCode::NOT_FOUND,
            format!("Document {id} not found"),
        ).into_response(),

        // Handles BOTH EmptyLayout AND DocStatusError (AlreadyPublishing, AlreadyPublished, etc.)
        Err(PublishDocumentError::Domain(err)) => (
            StatusCode::UNPROCESSABLE_ENTITY,
            err.to_string(), // Human-readable error message: e.g. "Cannot publish an empty document layout"
        ).into_response(),

        Err(PublishDocumentError::Internal(err)) => {
            tracing::error!(error = %err, doc_id = id, "Publish failed");
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
        }
    }
}