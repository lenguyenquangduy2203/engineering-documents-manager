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
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Head,
    Options,
}
