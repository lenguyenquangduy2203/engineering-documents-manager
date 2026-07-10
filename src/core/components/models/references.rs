use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct ApiEndpoint {
    pub endpoint: String,
    pub method: String,
    pub request_body_example: String,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(tag = "type", content = "data")]
pub enum ReferenceSubType {
    ApiEndpoint(ApiEndpoint),
}

impl ReferenceSubType {
    pub fn get_type(&self) -> String {
        match self {
            ReferenceSubType::ApiEndpoint(_) => "ApiEndpoint".into(),
        }
    }
}
