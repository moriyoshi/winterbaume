use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("missing required field `{field}` in {context}")]
    MissingField { field: String, context: String },

    #[error("unknown statement type: {0}")]
    UnknownStatement(String),

    #[error("statement object has no recognised statement key")]
    EmptyStatement,

    #[error("expected {expected} for field `{field}`, got {actual}")]
    TypeError {
        field: String,
        expected: String,
        actual: String,
    },

    #[error("JSON deserialization error: {0}")]
    Json(#[from] serde_json::Error),
}
