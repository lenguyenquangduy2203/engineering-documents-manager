use serde::{Deserialize, Serialize};
use sqlx::prelude::Type;
use strum_macros::{Display, EnumString};

use crate::core::documents::errors::doc_status::DocStatusError;

#[derive(Deserialize, Serialize, Clone, Copy, Debug, PartialEq, Eq, EnumString, Display, Type)]
#[serde(rename_all = "UPPERCASE")]
#[strum(serialize_all = "UPPERCASE")]
#[sqlx(type_name = "TEXT", rename_all = "UPPERCASE")]
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
