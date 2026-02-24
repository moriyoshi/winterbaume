use thiserror::Error;

/// Errors that can occur when working with Terraform state files.
#[derive(Debug, Error)]
pub enum TfStateError {
    /// JSON serialization/deserialization error.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// I/O error.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// The state file has an unsupported version (only v4 is supported).
    #[error("unsupported state version: {version} (only version 4 is supported)")]
    UnsupportedVersion {
        /// The version number found in the state file.
        version: u32,
    },

    /// The requested resource was not found.
    #[error("resource not found: {resource_type}.{name}")]
    ResourceNotFound {
        /// The resource type (e.g. "aws_s3_bucket").
        resource_type: String,
        /// The resource name.
        name: String,
    },

    /// A resource with the same type and name already exists.
    #[error("duplicate resource: {resource_type}.{name}")]
    DuplicateResource {
        /// The resource type.
        resource_type: String,
        /// The resource name.
        name: String,
    },
}
