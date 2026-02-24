use chrono::{DateTime, Utc};

/// Represents a SageMaker endpoint for mock purposes.
#[derive(Debug, Clone)]
pub struct MockEndpoint {
    pub endpoint_name: String,
    pub created_at: DateTime<Utc>,
}

/// A recorded invocation for inspection in tests.
#[derive(Debug, Clone)]
pub struct InvocationRecord {
    pub endpoint_name: String,
    pub content_type: Option<String>,
    pub accept: Option<String>,
    pub custom_attributes: Option<String>,
    pub target_model: Option<String>,
    pub target_variant: Option<String>,
    pub target_container_hostname: Option<String>,
    pub inference_id: Option<String>,
    pub inference_component_name: Option<String>,
    pub timestamp: DateTime<Utc>,
}

/// A recorded async invocation.
#[derive(Debug, Clone)]
pub struct AsyncInvocationRecord {
    pub endpoint_name: String,
    pub content_type: Option<String>,
    pub accept: Option<String>,
    pub custom_attributes: Option<String>,
    pub inference_id: Option<String>,
    pub input_location: Option<String>,
    pub output_location: String,
    pub failure_location: Option<String>,
    pub timestamp: DateTime<Utc>,
}
