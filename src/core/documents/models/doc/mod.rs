mod lifecycle;
mod mutations;

use crate::core::documents::models::{
    doc_status::DocStatus, doc_types::DocTypes, layout_version_ids::LayoutVersionIds,
};

/* #region Domain Entity */
#[derive(Debug, Clone)]
/* #endregion */
/* #region Serde DTO */
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
/* #endregion */
/* #region Sqlx Record */
#[derive(sqlx::FromRow)]
/* #endregion */
pub struct Document {
    pub id: Option<u32>,
    pub doc_type: DocTypes,
    pub title: String,
    pub status: DocStatus,
    pub layout_version_ids: LayoutVersionIds,
}

impl Document {
    pub fn new(doc_type: DocTypes, title: &str) -> Self {
        Self {
            id: None,
            doc_type,
            title: title.into(),
            status: DocStatus::Draft,
            layout_version_ids: LayoutVersionIds::default(),
        }
    }
}

/* #region Config Object */
#[derive(Debug, Clone, Default)]
/* #endregion */
/* #region Serde Request */
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
/* #endregion */
pub struct DocumentMetadataForUpdate {
    pub id: u32,
    pub title: Option<String>,
}

/* #region Value Object */
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/* #endregion */
pub struct ComponentSummary<'a> {
    pub root_id: u32,
    pub component_type: &'a str,
}
