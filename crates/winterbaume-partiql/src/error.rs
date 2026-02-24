use thiserror::Error;

#[derive(Debug, Error)]
pub enum PartiqlError {
    #[error("parse error: {0}")]
    Parse(String),

    #[error("unsupported statement: {0}")]
    Unsupported(String),

    #[error("parameter binding error: expected {expected} parameters, got {got}")]
    ParameterCount { expected: usize, got: usize },

    #[error("invalid value: {0}")]
    InvalidValue(String),

    #[error("missing required clause: {0}")]
    MissingClause(String),

    #[error("table name could not be extracted from statement")]
    MissingTableName,
}
