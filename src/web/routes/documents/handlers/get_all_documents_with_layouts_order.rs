use std::sync::Arc;

use axum::{Json, extract::{Query, State}, http::StatusCode, response::IntoResponse};

use crate::core::documents::repositories::{DocumentFilterQuery, DocumentLifecycleManager};

pub async fn handler(
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