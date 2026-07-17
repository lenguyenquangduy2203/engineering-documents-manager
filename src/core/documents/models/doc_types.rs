use serde::{Deserialize, Serialize};

use crate::core::documents::models::{api::ApiDoc, sdd::SystemDesignDoc};

#[derive(Deserialize, Serialize, Clone)]
#[serde(tag = "type", content = "data")]
pub enum DocTypes {
    SDD(SystemDesignDoc),
    API(ApiDoc),
}

impl DocTypes {
    pub fn is_allowed(&self, component_type: &str) -> bool {
        match self {
            DocTypes::SDD(system_design_doc) => system_design_doc.is_allowed(component_type),
            DocTypes::API(api_doc) => api_doc.is_allowed(component_type),
        }
    }
}
