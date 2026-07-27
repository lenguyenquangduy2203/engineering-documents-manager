use thiserror::Error;

#[derive(Debug, Error)]
pub enum DocumentMetadataError {
    #[error("Invalid metadata: {message}")]
    InvalidMetadata { message: String },
}
