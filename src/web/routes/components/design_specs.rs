use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};

use crate::{core::components::{models::Component, repositories::ComponentsRepository}, web::routes::context::Context};

#[derive(Serialize, Deserialize, Clone)]
pub struct DecisionRecord {
    pub decision: String,
    pub rationale: String,
    pub alternatives_considered: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "data")]
pub enum DesignSpecSubType {
    DecisionRecord(DecisionRecord),
}

pub type DesignSpecComponent = Component<DesignSpecSubType>;

#[derive(Serialize)]
pub struct CreatedResponse {
    id: u32,
}

pub async fn add_new_design_spec(
    State(ctx): State<Context>, 
    Json(design_spec): Json<DesignSpecComponent>
) -> impl IntoResponse {
    match ctx.save(&design_spec).await {
        Ok(generated_id) => {
            (
                StatusCode::CREATED, 
                Json(CreatedResponse { id: generated_id })
            ).into_response()
        }
        Err(err) => {
            tracing::warn!("Database layout update execution failed: {:?}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR, 
                "Failed to register component changes to the workspace history ledger."
            ).into_response()
        }
    }
}
