use crate::core::documents::models::{api::ApiDoc, sdd::SystemDesignDoc};

/* #region Tiny Value Object */
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/* #endregion */
/* #region Serde DTO */
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "UPPERCASE")]
/* #endregion */
/* #region Strum Enum */
#[derive(strum_macros::Display, strum_macros::EnumString, strum_macros::EnumIter)]
#[strum(serialize_all = "UPPERCASE")]
/* #endregion */
/* #region Sqlx Data Type */
#[derive(sqlx::Type)]
#[sqlx(type_name = "text")]
#[sqlx(rename_all = "UPPERCASE")]
/* #endregion */
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
