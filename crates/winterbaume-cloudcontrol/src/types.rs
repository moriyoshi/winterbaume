use chrono::{DateTime, Utc};

/// Represents a resource managed by Cloud Control API.
#[derive(Debug, Clone)]
pub struct ManagedResource {
    /// The CloudFormation type name, e.g. `AWS::S3::Bucket`.
    pub type_name: String,
    /// The primary identifier for the resource.
    pub identifier: String,
    /// JSON string representing the resource model (properties).
    pub resource_model: String,
}

/// Represents a resource operation request.
#[derive(Debug, Clone)]
pub struct ResourceRequest {
    /// Unique token identifying this request.
    pub request_token: String,
    /// The type name of the resource.
    pub type_name: String,
    /// The primary identifier for the resource (may be empty for CREATE before completion).
    pub identifier: String,
    /// The operation type.
    pub operation: OperationType,
    /// Current status of the operation.
    pub operation_status: OperationStatus,
    /// When the request was initiated.
    pub event_time: DateTime<Utc>,
    /// JSON string representing the resource model after the operation.
    pub resource_model: Option<String>,
    /// Status message for the operation.
    pub status_message: Option<String>,
    /// Error code if the operation failed.
    pub error_code: Option<String>,
}

/// The type of resource operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OperationType {
    Create,
    Delete,
    Update,
}

impl OperationType {
    pub fn as_str(&self) -> &'static str {
        match self {
            OperationType::Create => "CREATE",
            OperationType::Delete => "DELETE",
            OperationType::Update => "UPDATE",
        }
    }
}

/// Status of a resource operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OperationStatus {
    Pending,
    InProgress,
    Success,
    Failed,
    CancelInProgress,
    CancelComplete,
}

impl OperationStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            OperationStatus::Pending => "PENDING",
            OperationStatus::InProgress => "IN_PROGRESS",
            OperationStatus::Success => "SUCCESS",
            OperationStatus::Failed => "FAILED",
            OperationStatus::CancelInProgress => "CANCEL_IN_PROGRESS",
            OperationStatus::CancelComplete => "CANCEL_COMPLETE",
        }
    }
}
