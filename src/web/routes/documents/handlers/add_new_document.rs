use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use crate::{
    core::documents::{models::doc::Document, repositories::DocumentLifecycleManager}, 
    web::routes::documents::payloads::create_doc::{CreateDocRequest, CreatedResponse}
};

pub async fn handler(
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
