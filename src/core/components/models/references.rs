use crate::core::components::models::values::{http_method::HttpMethod, paths::ApiPath};

/* #region Domain Entity */
#[derive(Debug, Clone)]
/* #endregion */
/* #region Serde DTO */
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
/* #endregion */
pub struct ApiEndpoint {
    pub endpoint: ApiPath,
    pub method: HttpMethod,
    pub request_body_example: String,
}

/* #region Domain Entity */
#[derive(Debug, Clone)]
/* #endregion */
/* #region Serde DTO */
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(tag = "type", content = "data")]
/* #endregion */
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
