use axum::{Json, Router, extract::{Query, State}, http::StatusCode, response::IntoResponse, routing::{get, post}};
use serde::Serialize;
use crate::{core::components::{models::{payload::ComponentPayload, wrapper::Component}, repositories::{ComponentFilterQuery, ComponentsRepository}}, web::routes::context::Context};

pub fn build() -> Router<Context> {
    Router::new()
        .route("/components", post(add_new_component))
        .route("/components", get(get_all_latest_components))
}

#[derive(Serialize)]
struct CreatedResponse {
    id: u32,
}

async fn add_new_component(
    State(ctx): State<Context>, 
    Json(component): Json<Component<ComponentPayload>>
) -> impl IntoResponse {
    match ctx.create_new(&component).await {
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

async fn get_all_latest_components(
    State(ctx): State<Context>,
    Query(filter): Query<ComponentFilterQuery>,
) -> impl IntoResponse {
    match ctx.find_all_latest_version(filter).await {
        Ok(components) => (StatusCode::OK, Json(components)).into_response(),
        Err(err) => {
            eprintln!("Failed to filter component workspace: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to read components").into_response()
        }
    }
}