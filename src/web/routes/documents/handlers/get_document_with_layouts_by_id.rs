use std::sync::Arc;

use axum::{Json, extract::{Path, State}, http::StatusCode, response::IntoResponse};

use crate::core::documents::repositories::DocumentLifecycleManager;

pub async fn handler(
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