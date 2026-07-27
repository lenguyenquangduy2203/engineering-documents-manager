use std::sync::Arc;

use axum::{
    Json, 
    extract::{Path, State}, 
    http::StatusCode, 
    response::IntoResponse
};

use crate::{
    core::components::{
        models::{payload::ComponentPayload, values::version::Version, wrapper::Component}, 
        repositories::{ComponentsRepository, UpdateComponentError}
    }, 
    web::routes::components::payloads::update_component::UpdateComponentRequest
};

pub async fn handler(
    State(ctx): State<Arc<dyn ComponentsRepository>>,
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
