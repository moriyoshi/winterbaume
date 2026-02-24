use std::collections::HashMap;

use crate::wire::Block;

/// Domain errors produced by Textract request validation.
#[derive(Debug, thiserror::Error)]
pub enum TextractError {
    #[error("Missing X-Amz-Target header")]
    MissingAction,
    #[error("Invalid JSON body")]
    SerializationException,
    #[error("Request has invalid parameters")]
    InvalidParameter,
    #[error("Unknown operation: {action}")]
    UnknownOperation { action: String },
}

/// A stored async Textract job.
#[derive(Debug, Clone, Default)]
pub struct TextractJob {
    /// The action type (e.g. "DocumentAnalysis", "DocumentTextDetection").
    pub job_type: String,
    /// The blocks associated with this job.
    pub blocks: Vec<Block>,
}

/// Textract service state.
///
/// Textract operations are largely stateless in this mock: all async jobs
/// (StartDocumentAnalysis, StartDocumentTextDetection) immediately report
/// SUCCEEDED on the first poll.  The job map stores the mock blocks so that
/// GetDocument* can return non-empty results.
#[derive(Debug, Default)]
pub struct TextractState {
    /// Async jobs keyed by job ID.
    pub jobs: HashMap<String, TextractJob>,
}
