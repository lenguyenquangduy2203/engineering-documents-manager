use std::sync::Arc;

use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse};

use crate::core::components::repositories::ComponentsRepository;

pub async fn handler(
    State(ctx): State<Arc<dyn ComponentsRepository>>,
    Path(id): Path<u32>
) -> impl IntoResponse {
    match ctx.remove_component_with_all_versions_by_id(id).await {
        Ok(true) => StatusCode::NO_CONTENT.into_response(),
        Ok(false) => (
            StatusCode::NOT_FOUND, 
            format!("Component with ID {id} does not exist")
        ).into_response(),
        Err(err) => {
            tracing::error!(error = %err, id = id, "Failed to delete component");
            (
                StatusCode::INTERNAL_SERVER_ERROR, 
                "Internal server error"
            ).into_response()
        }
    }
}