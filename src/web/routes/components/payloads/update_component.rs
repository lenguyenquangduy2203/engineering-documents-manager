/* #region Serde Request */
#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
/* #endregion */
pub struct UpdateComponentRequest<T> {
    pub title: String,
    pub payload: T,
}
