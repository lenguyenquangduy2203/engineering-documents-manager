use serde::{Deserialize, Serialize};
use sqlx::prelude::Type;
use strum_macros::{Display, EnumString};

use crate::core::documents::models::{api::ApiDoc, sdd::SystemDesignDoc};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, EnumString, Display, Type)]
#[serde(rename_all = "UPPERCASE")]
#[strum(serialize_all = "UPPERCASE")]
#[sqlx(type_name = "TEXT", rename_all = "UPPERCASE")]
pub enum DocTypes {
    Sdd,
    Api,
}

impl DocTypes {
    pub fn is_allowed(&self, component_type: &str) -> bool {
        match self {
            DocTypes::Sdd => SystemDesignDoc::is_allowed(component_type),
            DocTypes::Api => ApiDoc::is_allowed(component_type),
        }
    }
}

impl From<&DocTypes> for String {
    fn from(value: &DocTypes) -> Self {
        match value {
            DocTypes::Sdd => "SDD".into(),
            DocTypes::Api => "API".into(),
        }
    }
}
