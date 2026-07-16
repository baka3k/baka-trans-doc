use serde::Serialize;
use thiserror::Error;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Error, Serialize)]
#[serde(tag = "code", content = "message", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AppError {
    #[error("Unsupported document format: {0}")]
    UnsupportedFormat(String),
    #[error("The document is invalid or damaged: {0}")]
    InvalidDocument(String),
    #[error("The document package exceeds a safe resource limit: {0}")]
    ResourceLimit(String),
    #[error("The output path is not safe: {0}")]
    UnsafeOutput(String),
    #[error("The output file already exists: {0}")]
    OutputExists(String),
    #[error("Ollama is unavailable: {0}")]
    OllamaUnavailable(String),
    #[error("The selected model is unavailable: {0}")]
    ModelUnavailable(String),
    #[error("The translation response is invalid: {0}")]
    InvalidTranslation(String),
    #[error("The translation was cancelled")]
    Cancelled,
    #[error("Job not found: {0}")]
    JobNotFound(String),
    #[error("Checkpoint is incompatible: {0}")]
    CheckpointMismatch(String),
    #[error("I/O error: {0}")]
    Io(String),
    #[error("Internal error: {0}")]
    Internal(String),
}

impl From<std::io::Error> for AppError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value.to_string())
    }
}

impl From<zip::result::ZipError> for AppError {
    fn from(value: zip::result::ZipError) -> Self {
        Self::InvalidDocument(value.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(value: serde_json::Error) -> Self {
        Self::Internal(value.to_string())
    }
}
