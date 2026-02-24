//! Error types for Terraform state conversion.

/// Errors that can occur during Terraform resource conversion.
#[derive(Debug, thiserror::Error)]
pub enum ConversionError {
    /// A required attribute is missing from the Terraform resource.
    #[error("missing required attribute '{attribute}' on resource type '{resource_type}'")]
    MissingAttribute {
        /// The Terraform resource type (e.g. "aws_s3_bucket").
        resource_type: String,
        /// The attribute key that was expected.
        attribute: String,
    },

    /// An attribute has an invalid value.
    #[error("invalid attribute '{attribute}' on '{resource_type}': {detail}")]
    InvalidAttribute {
        /// The Terraform resource type.
        resource_type: String,
        /// The attribute key.
        attribute: String,
        /// Description of why the value is invalid.
        detail: String,
    },

    /// The resource type is not supported by any registered converter.
    #[error("unsupported resource type: {resource_type}")]
    UnsupportedResourceType {
        /// The unsupported Terraform resource type.
        resource_type: String,
    },

    /// An error occurred in the underlying service state view operation.
    #[error("state view error: {0}")]
    StateView(#[from] winterbaume_core::StateViewError),
}
