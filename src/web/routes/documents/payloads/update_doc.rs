/* #region Serde Request */
#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
/* #endregion */
pub struct UpdateDocRequest {
    pub title: Option<String>,
}
