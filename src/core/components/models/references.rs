use serde::{Deserialize, Serialize};

use crate::core::components::models::values::{http_method::HttpMethod, paths::ApiPath};

#[derive(Deserialize, Serialize, Clone)]
pub struct ApiEndpoint {
    pub endpoint: ApiPath,
    pub method: HttpMethod,
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
