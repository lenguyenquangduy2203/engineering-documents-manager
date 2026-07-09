use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};

use crate::{core::components::repositories::ComponentsRepository, web::routes::context::Context};

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

#[derive(Deserialize)]
pub struct DesignSpecComponent {
    pub title: String,
    pub payload: DesignSpecSubType,
}

#[derive(Serialize)]
pub struct CreatedResponse {
    id: u32,
}

pub async fn add_new_design_spec(
    State(ctx): State<Context>, 
    Json(design_spec): Json<DesignSpecComponent>
) -> impl IntoResponse {
    let subtype_str = match &design_spec.payload {
        DesignSpecSubType::DecisionRecord(_) => "DecisionRecord",
    };
    let component_type = format!("DesignSpec:{}", subtype_str);
    let json_payload = match serde_json::to_value(&design_spec.payload) {
        Ok(val) => val,
        Err(e) => return (StatusCode::BAD_REQUEST, format!("Invalid payload layout: {e}")).into_response(),
    };

    match ctx.save(&component_type, &design_spec.title, json_payload).await {
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
