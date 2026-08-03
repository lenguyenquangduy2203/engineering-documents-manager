use crate::core::documents::errors::doc_status::DocStatusError;

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
pub enum DocStatus {
    Draft,
    Publishing,
    Published,
    Failed,
}

impl DocStatus {
    pub fn transition_to_publishing(self) -> Result<Self, DocStatusError> {
        match self {
            DocStatus::Draft | DocStatus::Failed => Ok(DocStatus::Publishing),
            DocStatus::Publishing => Err(DocStatusError::AlreadyPublishing),
            DocStatus::Published => Err(DocStatusError::AlreadyPublished),
        }
    }

    pub fn transition_to_published(self) -> Result<Self, DocStatusError> {
        match self {
            DocStatus::Publishing | DocStatus::Draft => Ok(DocStatus::Published),
            DocStatus::Published => Err(DocStatusError::AlreadyPublished),
            DocStatus::Failed => Err(DocStatusError::InvalidFinalizeFromFailed),
        }
    }

    pub fn transition_to_failed(self) -> Result<Self, DocStatusError> {
        match self {
            DocStatus::Publishing => Ok(DocStatus::Failed),
            other => Err(DocStatusError::InvalidFailureTransition(other)),
        }
    }

    pub fn can_modify_layout(self) -> bool {
        matches!(self, DocStatus::Draft | DocStatus::Failed)
    }
}
