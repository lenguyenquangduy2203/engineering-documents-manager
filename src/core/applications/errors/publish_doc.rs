use thiserror::Error;

use crate::core::documents::errors::doc_publication::DocumentPublishingError;

#[derive(Debug, Error)]
pub enum PublishDocumentError {
    #[error("Document with ID {0} was not found")]
    NotFound(u32),

    #[error(transparent)]
    Domain(#[from] DocumentPublishingError),

    #[error("Internal error: {0}")]
    Internal(#[from] anyhow::Error),
}
