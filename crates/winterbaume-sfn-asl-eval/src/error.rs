use thiserror::Error;

/// Internal error type for validation failures that prevent the engine
/// from producing a meaningful `ValidationResult`.
#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("JSON parse error: {0}")]
    JsonParse(#[from] serde_json::Error),
}
