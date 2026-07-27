use std::fmt::Display;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Deserialize, Serialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum DocStatus {
    Draft,
    Publishing,
    Published,
    Failed,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DocStatusError {
    #[error("Document is already in the process of publishing")]
    AlreadyPublishing,

    #[error("Published documents cannot re-enter publishing")]
    AlreadyPublished,

    #[error("Cannot directly finalize a failed document without restarting publish")]
    InvalidFinalizeFromFailed,

    #[error("Cannot transition to Failed state from state '{0}'")]
    InvalidFailureTransition(DocStatus),

    #[error("Cannot modify layout while document is in state '{0}'")]
    LayoutModificationNotAllowed(DocStatus),
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

impl From<&DocStatus> for String {
    fn from(value: &DocStatus) -> Self {
        match value {
            DocStatus::Draft => "DRAFT".into(),
            DocStatus::Publishing => "PUBLISHING".into(),
            DocStatus::Published => "PUBLISHED".into(),
            DocStatus::Failed => "FAILED".into(),
        }
    }
}

impl Display for DocStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content: String = self.into();
        write!(f, "{}", content)
    }
}
