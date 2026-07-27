use thiserror::Error;

#[derive(Debug, Error)]
pub enum DocumentPublishingError {
    #[error("Cannot publish an empty document layout")]
    EmptyLayout,

    #[error(transparent)]
    Status(#[from] super::doc_status::DocStatusError),
}
