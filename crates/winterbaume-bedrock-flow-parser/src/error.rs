use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("missing required field `{field}` in {context}")]
    MissingField { field: String, context: String },

    #[error("expected {expected} for field `{field}`, got {actual}")]
    TypeError {
        field: String,
        expected: String,
        actual: String,
    },

    #[error("unknown node type: {0}")]
    UnknownNodeType(String),

    #[error("unknown data type: {0}")]
    UnknownDataType(String),

    #[error("unknown connection type: {0}")]
    UnknownConnectionType(String),
}
