use crate::core::documents::models::{doc::Document, doc_types::DocTypes};

/* #region Serde Request */
#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
/* #endregion */
pub struct CreateDocRequest {
    pub doc_type: DocTypes,
    pub title: String,
}

impl From<CreateDocRequest> for Document {
    fn from(req: CreateDocRequest) -> Self {
        Self::new(req.doc_type, &req.title)
    }
}

/* #region Serde Response */
#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
/* #endregion */
pub struct CreatedResponse {
    pub id: u32,
}
