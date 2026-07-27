use thiserror::Error;

#[derive(Debug, Error)]
pub enum DocumentLayoutError {
    #[error(transparent)]
    Status(#[from] super::doc_status::DocStatusError),

    #[error("Incompatible number of components requested: expected {expected}, found {found}")]
    IncompatibleComponentCount { expected: usize, found: usize },

    #[error("Layout conflict: Cannot add multiple versions of the same root component to a single layout")]
    DuplicateRootComponents,

    #[error(
        "Document type mismatch: One or more component layouts are barred from this document type"
    )]
    TypeMismatch,
}
