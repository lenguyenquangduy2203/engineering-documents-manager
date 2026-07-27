use std::sync::Arc;

use axum::{Json, extract::{Path, State}, http::StatusCode, response::IntoResponse};

use crate::{core::documents::{models::doc::DocumentMetadataForUpdate, repositories::{DocumentLifecycleManager, DocumentUpdateError}}, web::routes::documents::payloads::update_doc::UpdateDocRequest};

pub async fn handler(
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