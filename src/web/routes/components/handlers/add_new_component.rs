use std::sync::Arc;

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Json, extract::State};

use crate::core::components::models::payload::ComponentPayload;
use crate::core::components::repositories::components_repository::ComponentsRepository;
use crate::web::routes::components::payloads::create_component::{CreateComponentRequest, CreatedResponse};

pub async fn handler(
    State(ctx): State<Arc<dyn ComponentsRepository>>, 
    Json(request): Json<CreateComponentRequest<ComponentPayload>>
) -> impl IntoResponse {
    match ctx.create_new(&request.into()).await {
        Ok(generated_id) => {
            (
                StatusCode::CREATED, 
                Json(CreatedResponse { id: generated_id })
            ).into_response()
        }
        Err(err) => {
            tracing::warn!("Database component insert execution failed: {:?}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR, 
                "Failed to register component changes to the workspace history ledger."
            ).into_response()
        }
    }
}
