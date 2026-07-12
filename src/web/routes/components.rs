use axum::{Json, Router, extract::{Path, Query, State}, http::StatusCode, response::IntoResponse, routing::{get, post}};
use serde::{Deserialize, Serialize};
use crate::{core::components::{models::{payload::ComponentPayload, values::version::Version, wrapper::Component}, repositories::{ComponentFilterQuery, ComponentsRepository}}, web::routes::context::Context};

pub fn build() -> Router<Context> {
    Router::new()
        .route("/components", post(add_new_component))
        .route("/components", get(get_all_latest_components))
        .route("/components/{id}", get(get_latest_component))
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CreateComponentRequest<T> {
    pub title: String,
    pub payload: T,
}

impl<T> From<CreateComponentRequest<T>> for Component<T> {
    fn from(req: CreateComponentRequest<T>) -> Self {
        Component { 
            id: None, 
            version: Version::default(), 
            title: req.title, 
            payload: req.payload 
        }
    }
}

#[derive(Serialize)]
struct CreatedResponse {
    id: u32,
}

async fn add_new_component(
    State(ctx): State<Context>, 
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

async fn get_all_latest_components(
    State(ctx): State<Context>,
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

async fn get_latest_component(
    State(ctx): State<Context>,
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