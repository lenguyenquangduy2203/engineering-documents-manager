use std::sync::Arc;

use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse};

use crate::core::documents::repositories::document_lifecycle_manager::DocumentLifecycleManager;

pub async fn handler(
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