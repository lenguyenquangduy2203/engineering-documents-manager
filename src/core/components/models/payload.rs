use serde::{Deserialize, Serialize};

use crate::core::components::models::design_specs::DesignSpecSubType;

#[derive(Deserialize, Serialize, Clone)]
#[serde(tag = "group", content = "data")]
pub enum ComponentPayload {
    DesignSpec(DesignSpecSubType),
}

impl ComponentPayload {
    pub fn get_group(&self) -> String {
        match self {
            ComponentPayload::DesignSpec(_) => "DesignSpec".into(),
        }
    }

    pub fn get_identifier(&self) -> String {
        match self {
            ComponentPayload::DesignSpec(design_spec_sub_type) => {
                format!("{}:{}", self.get_group(), design_spec_sub_type.get_type())
            }
        }
    }
}
