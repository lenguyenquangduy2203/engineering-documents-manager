use std::{sync::Arc};

use axum::{Json, Router, extract::{Path, Query, State}, http::StatusCode, response::IntoResponse, routing::{delete, get, patch, post, put}};
use serde::{Deserialize, Serialize};

use crate::{core::{applications::services::{publish_document::{DocumentPublishingService, PublishDocumentError}, update_document_layouts::{DocumentLayoutService, DocumentLayoutServiceError}}, components::repositories::{ComponentPayloadResolver, ComponentTypeResolver}, documents::{models::{doc::{Document, DocumentLayoutError, DocumentMetadataForUpdate}, doc_types::DocTypes}, repositories::{DocumentFilterQuery, DocumentLayoutsModifier, DocumentLifecycleManager, DocumentPublisher, DocumentUpdateError, DocumentsResolver}}}, infra::rendering::services::DocumentExportService, web::routes::context::Context};

pub fn build() -> Router<Context> {
    Router::new()
        .route("/documents", post(add_new_document))
        .route("/documents", get(get_all_documents_with_layouts_order))
        .route("/documents/{id}", get(get_document_with_layouts_by_id))
        .route("/documents/{id}", patch(partially_update_document_by_id))
        .route("/documents/{id}", delete(remove_document_by_id))
        .route("/documents/{id}/layouts", put(update_document_layouts_by_id))
        .route("/documents/{id}/publish", post(publish_document_by_id))
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CreateDocRequest {
    pub doc_type: DocTypes,
    pub title: String,
}

impl From<CreateDocRequest> for Document {
    fn from(req: CreateDocRequest) -> Self {
        Self::new(req.doc_type, &req.title)
    }
}

#[derive(Serialize)]
struct CreatedResponse {
    id: u32,
}

async fn add_new_document(
    State(document_lifecycle_manager): State<Arc<dyn DocumentLifecycleManager>>,
    Json(req): Json<CreateDocRequest>
) -> impl IntoResponse {
    match document_lifecycle_manager.create_new(&Document::from(req)).await {
        Ok(generated_id) => (
                StatusCode::CREATED, 
                Json(CreatedResponse { id: generated_id })
            ).into_response(),
        Err(err) => {
            tracing::warn!("Database document insert execution failed: {:?}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR, 
                "Failed to register document changes to the workspace history ledger."
            ).into_response()
        },
    }
}

async fn get_all_documents_with_layouts_order(
    State(document_lifecycle_manager): State<Arc<dyn DocumentLifecycleManager>>,
    Query(filter): Query<DocumentFilterQuery>
) -> impl IntoResponse {
    match document_lifecycle_manager.find_all_docs(filter).await {
        Ok(documents) => (StatusCode::OK, Json(documents)).into_response(),
        Err(err) => {
            tracing::warn!("Failed to filter document workspace: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to read documents").into_response()
        },
    }
}

async fn get_document_with_layouts_by_id(
    State(document_lifecycle_manager): State<Arc<dyn DocumentLifecycleManager>>,
    Path(id): Path<u32>
) -> impl IntoResponse {
    match document_lifecycle_manager.find_doc_by_id(id).await {
        Ok(Some(document)) => (StatusCode::OK, Json(document)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Found no document with expected id").into_response(),
        Err(err) => {
            tracing::warn!("Failed to find document with id={:?}: {:?}", id, err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to find document").into_response()
        },
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct UpdateDocRequest {
    pub title: Option<String>,
}

async fn partially_update_document_by_id(
    State(document_lifecycle_manager): State<Arc<dyn DocumentLifecycleManager>>,
    Path(id): Path<u32>,
    Json(request): Json<UpdateDocRequest>
) -> impl IntoResponse {
    let incoming_document = DocumentMetadataForUpdate { 
        id, 
        title: request.title,
    };

    match document_lifecycle_manager.update_doc(incoming_document).await {
        Ok(Some(doc)) => (StatusCode::OK, Json(doc)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Found no document with expected id for update").into_response(),
        Err(DocumentUpdateError::Domain(err)) => (
            StatusCode::BAD_REQUEST,
            err.to_string()
        ).into_response(),
        Err(err) => {
            tracing::warn!("Failed to update document with id={:?}: {:?}", id, err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to update document").into_response()
        },
    }
}

async fn remove_document_by_id(
    State(document_lifecycle_manager): State<Arc<dyn DocumentLifecycleManager>>,
    Path(id): Path<u32>
) -> impl IntoResponse {
    match document_lifecycle_manager.remove_doc_with_all_layouts_by_id(id).await {
        Ok(true) => StatusCode::NO_CONTENT.into_response(),
        Ok(false) => (
            StatusCode::NOT_FOUND, 
            format!("Document with ID {id} does not exist")
        ).into_response(),
        Err(err) => {
            tracing::error!(error = %err, id = id, "Failed to delete document");
            (
                StatusCode::INTERNAL_SERVER_ERROR, 
                "Internal server error"
            ).into_response()
        }
    }
}

async fn update_document_layouts_by_id(
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

async fn publish_document_by_id(
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