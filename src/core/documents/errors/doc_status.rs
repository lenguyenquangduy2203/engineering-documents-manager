use thiserror::Error;

use crate::core::documents::models::doc_status::DocStatus;

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
