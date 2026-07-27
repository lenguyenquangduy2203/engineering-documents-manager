use thiserror::Error;

use crate::core::documents::errors::doc_layout::DocumentLayoutError;

#[derive(Debug, Error)]
pub enum DocumentLayoutServiceError {
    #[error("Document with ID {0} was not found")]
    DocumentNotFound(u32),

    #[error(transparent)]
    Domain(#[from] DocumentLayoutError),

    #[error("Infrastructure failure: {0}")]
    Internal(#[from] anyhow::Error),
}
