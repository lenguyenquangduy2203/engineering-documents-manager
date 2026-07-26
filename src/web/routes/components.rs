use std::sync::Arc;

use axum::{Json, Router, extract::{Path, Query, State}, http::StatusCode, response::IntoResponse, routing::{delete, get, post, put}};
use serde::{Deserialize, Serialize};
use crate::{core::components::{models::{payload::ComponentPayload, values::version::Version, wrapper::Component}, repositories::{ComponentFilterQuery, ComponentsRepository, UpdateComponentError}}, web::routes::context::Context};

pub fn build() -> Router<Context> {
    Router::new()
        .route("/components", post(add_new_component))
        .route("/components", get(get_all_latest_components))
        .route("/components/{id}", get(get_latest_component))
        .route("/components/{id}", put(update_component_by_id))
        .route("/components/{id}", delete(remove_component_by_id))
}

type ComponentCtx = Arc<dyn ComponentsRepository>;

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
    State(ctx): State<ComponentCtx>, 
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
    State(ctx): State<ComponentCtx>,
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
    State(ctx): State<ComponentCtx>,
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

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct UpdateComponentRequest<T> {
    pub title: String,
    pub payload: T,
}

async fn update_component_by_id(
    State(ctx): State<ComponentCtx>,
    Path(id): Path<u32>,
    Json(request): Json<UpdateComponentRequest<ComponentPayload>>
) -> impl IntoResponse {
    let incoming_component = Component {
        id: None,
        title: request.title,
        version: Version::default(),
        payload: request.payload,
    };

    match ctx.update_component(id, incoming_component).await {
        Ok(Some(component)) => (StatusCode::OK, Json(component)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Component not found").into_response(),
        
        // 403 Forbidden for business domain rules
        Err(UpdateComponentError::Domain(err)) => (
            StatusCode::FORBIDDEN, 
            err.to_string()
        ).into_response(),
        
        // 500 for any DB or internal system failure
        Err(err) => {
            tracing::error!(error = %err, id = id, "Failed to update component");
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
        }
    }
}

async fn remove_component_by_id(
    State(ctx): State<ComponentCtx>,
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