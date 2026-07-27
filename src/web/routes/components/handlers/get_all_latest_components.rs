use std::sync::Arc;

use axum::{Json, extract::{Query, State}, http::StatusCode, response::IntoResponse};

use crate::core::components::repositories::{ComponentFilterQuery, ComponentsRepository};

pub async fn handler(
    State(ctx): State<Arc<dyn ComponentsRepository>>,
    Query(filter): Query<ComponentFilterQuery>,
) -> impl IntoResponse {
    match ctx.find_all_latest_version(filter).await {
        Ok(components) => (StatusCode::OK, Json(components)).into_response(),
        Err(err) => {
            tracing::warn!("Failed to filter component workspace: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to read components").into_response()
        }
    }
}