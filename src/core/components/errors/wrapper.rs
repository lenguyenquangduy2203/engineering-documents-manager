use thiserror::Error;

#[derive(Debug, Error)]
pub enum ComponentError {
    #[error("Incompatible component type: expected {expected}, received {received}")]
    IncompatibleType { expected: String, received: String },
}
