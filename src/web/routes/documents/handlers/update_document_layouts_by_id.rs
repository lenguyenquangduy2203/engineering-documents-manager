use std::sync::Arc;

use axum::{Json, extract::{Path, State}, http::StatusCode, response::IntoResponse};

use crate::core::{
    applications::{
        services::update_document_layouts::DocumentLayoutService, 
        errors::update_layouts::DocumentLayoutServiceError
    }, 
    components::repositories::ComponentTypeResolver, 
    documents::{
        errors::doc_layout::DocumentLayoutError, 
        repositories::{DocumentLayoutsModifier, DocumentsResolver}
    }
};

pub async fn handler(
    State(documents_resolver): State<Arc<dyn DocumentsResolver>>,
    State(component_type_resolver): State<Arc<dyn ComponentTypeResolver>>,
    State(document_layouts_modifier): State<Arc<dyn DocumentLayoutsModifier>>,
    Path(id): Path<u32>,
    Json(version_ids): Json<Vec<u32>>
) -> impl IntoResponse {
    let deps = (
        documents_resolver.as_ref(), 
        component_type_resolver.as_ref(), 
        document_layouts_modifier.as_ref()
    );

    match DocumentLayoutService::update_layouts(deps, id, &version_ids).await {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        
        // 404 Not Found
        Err(DocumentLayoutServiceError::DocumentNotFound(id)) => (
            StatusCode::NOT_FOUND,
            format!("Document {id} not found"),
        ).into_response(),

        // 400 Bad Request
        Err(DocumentLayoutServiceError::Domain(DocumentLayoutError::IncompatibleComponentCount { expected, found })) => (
            StatusCode::BAD_REQUEST,
            format!("Component count mismatch: expected {expected}, found {found}"),
        ).into_response(),

        // 409 Conflict
        Err(DocumentLayoutServiceError::Domain(DocumentLayoutError::DuplicateRootComponents)) => (
            StatusCode::CONFLICT,
            "Cannot add multiple versions of the same root component",
        ).into_response(),

        // 403 Forbidden
        Err(DocumentLayoutServiceError::Domain(DocumentLayoutError::TypeMismatch)) => (
            StatusCode::FORBIDDEN,
            "One or more components are not allowed in this document type",
        ).into_response(),

        // 422 Unprocessable Entity (State machine error)
        Err(DocumentLayoutServiceError::Domain(DocumentLayoutError::Status(err))) => (
            StatusCode::UNPROCESSABLE_ENTITY,
            err.to_string(),
        ).into_response(),

        // 500 Internal Server Error (Database/System crashes)
        Err(DocumentLayoutServiceError::Internal(err)) => {
            tracing::error!(error = %err, "Failed to update document layout");
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
        }
    }
}
