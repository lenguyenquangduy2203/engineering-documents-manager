use std::sync::Arc;

use axum::{Json, extract::{Path, State}, http::StatusCode, response::IntoResponse};

use crate::core::components::repositories::ComponentsRepository;

pub async fn handler(
    State(ctx): State<Arc<dyn ComponentsRepository>>,
    Path(id): Path<u32>,
) -> impl IntoResponse {
    match ctx.find_latest_version_by_id(id).await {
        Ok(opt) => match opt {
            Some(component) => (StatusCode::OK, Json(component)).into_response(),
            None => (StatusCode::NOT_FOUND, "Found no component with expected id").into_response(),
        },
        Err(err) => {
            tracing::warn!("Failed to find component with id={:?}: {:?}", id, err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to find component").into_response()
        },
    }
}