use std::collections::HashMap;

use chrono::Utc;
use thiserror::Error;
use uuid::Uuid;

use crate::types::{ManagedResource, OperationStatus, OperationType, ResourceRequest};

/// Domain-specific error enum for Cloud Control API.
/// Contains no HTTP status codes or AWS error type strings.
#[derive(Debug, Error)]
pub enum CloudControlError {
    #[error("Resource of type {type_name} with identifier {identifier} already exists.")]
    AlreadyExists {
        type_name: String,
        identifier: String,
    },
    #[error("Resource of type {type_name} with identifier {identifier} not found.")]
    ResourceNotFound {
        type_name: String,
        identifier: String,
    },
    #[error("A resource operation with the specified request token {token} was not found.")]
    RequestTokenNotFound { token: String },
    #[error("The specified extension {type_name} does not exist in the CloudFormation registry.")]
    TypeNotFound { type_name: String },
    #[error("{message}")]
    InvalidRequest { message: String },
    #[error(
        "The resource operation request with token {token} cannot be cancelled because its status is {status}."
    )]
    NotCancellable { token: String, status: String },
}

#[derive(Debug, Default)]
pub struct CloudControlState {
    /// Resources keyed by (type_name, identifier).
    pub resources: HashMap<(String, String), ManagedResource>,
    /// Operation requests keyed by request_token.
    pub requests: HashMap<String, ResourceRequest>,
}

impl CloudControlState {
    /// Create a resource. Operations complete synchronously in the mock.
    pub fn create_resource(
        &mut self,
        type_name: &str,
        desired_state: &str,
    ) -> Result<ResourceRequest, CloudControlError> {
        // Parse the desired state to extract an identifier, or generate one.
        let identifier = extract_identifier_from_model(desired_state)
            .unwrap_or_else(|| Uuid::new_v4().to_string());

        let key = (type_name.to_string(), identifier.clone());
        if self.resources.contains_key(&key) {
            return Err(CloudControlError::AlreadyExists {
                type_name: type_name.to_string(),
                identifier,
            });
        }

        let resource = ManagedResource {
            type_name: type_name.to_string(),
            identifier: identifier.clone(),
            resource_model: desired_state.to_string(),
        };
        self.resources.insert(key, resource);

        let request_token = Uuid::new_v4().to_string();
        let request = ResourceRequest {
            request_token: request_token.clone(),
            type_name: type_name.to_string(),
            identifier: identifier.clone(),
            operation: OperationType::Create,
            operation_status: OperationStatus::Success,
            event_time: Utc::now(),
            resource_model: Some(desired_state.to_string()),
            status_message: None,
            error_code: None,
        };
        self.requests.insert(request_token, request.clone());

        Ok(request)
    }

    /// Delete a resource by type name and identifier.
    pub fn delete_resource(
        &mut self,
        type_name: &str,
        identifier: &str,
    ) -> Result<ResourceRequest, CloudControlError> {
        let key = (type_name.to_string(), identifier.to_string());
        if self.resources.remove(&key).is_none() {
            return Err(CloudControlError::ResourceNotFound {
                type_name: type_name.to_string(),
                identifier: identifier.to_string(),
            });
        }

        let request_token = Uuid::new_v4().to_string();
        let request = ResourceRequest {
            request_token: request_token.clone(),
            type_name: type_name.to_string(),
            identifier: identifier.to_string(),
            operation: OperationType::Delete,
            operation_status: OperationStatus::Success,
            event_time: Utc::now(),
            resource_model: None,
            status_message: None,
            error_code: None,
        };
        self.requests.insert(request_token, request.clone());

        Ok(request)
    }

    /// Update a resource by applying a JSON patch document.
    pub fn update_resource(
        &mut self,
        type_name: &str,
        identifier: &str,
        patch_document: &str,
    ) -> Result<ResourceRequest, CloudControlError> {
        let key = (type_name.to_string(), identifier.to_string());
        let resource =
            self.resources
                .get_mut(&key)
                .ok_or_else(|| CloudControlError::ResourceNotFound {
                    type_name: type_name.to_string(),
                    identifier: identifier.to_string(),
                })?;

        // Apply the patch: parse existing model and patch operations.
        let mut model: serde_json::Value = serde_json::from_str(&resource.resource_model)
            .unwrap_or(serde_json::Value::Object(serde_json::Map::new()));
        let patches: Vec<serde_json::Value> =
            serde_json::from_str(patch_document).unwrap_or_default();
        for patch in &patches {
            apply_json_patch(&mut model, patch);
        }
        let updated_model = serde_json::to_string(&model).unwrap_or_default();
        resource.resource_model = updated_model.clone();

        let request_token = Uuid::new_v4().to_string();
        let request = ResourceRequest {
            request_token: request_token.clone(),
            type_name: type_name.to_string(),
            identifier: identifier.to_string(),
            operation: OperationType::Update,
            operation_status: OperationStatus::Success,
            event_time: Utc::now(),
            resource_model: Some(updated_model),
            status_message: None,
            error_code: None,
        };
        self.requests.insert(request_token, request.clone());

        Ok(request)
    }

    /// Get a resource by type name and identifier.
    pub fn get_resource(
        &self,
        type_name: &str,
        identifier: &str,
    ) -> Result<&ManagedResource, CloudControlError> {
        let key = (type_name.to_string(), identifier.to_string());
        self.resources
            .get(&key)
            .ok_or_else(|| CloudControlError::ResourceNotFound {
                type_name: type_name.to_string(),
                identifier: identifier.to_string(),
            })
    }

    /// List resources of a given type.
    pub fn list_resources(&self, type_name: &str) -> Vec<&ManagedResource> {
        self.resources
            .values()
            .filter(|r| r.type_name == type_name)
            .collect()
    }

    /// Get the status of a resource operation request.
    pub fn get_resource_request_status(
        &self,
        request_token: &str,
    ) -> Result<&ResourceRequest, CloudControlError> {
        self.requests
            .get(request_token)
            .ok_or_else(|| CloudControlError::RequestTokenNotFound {
                token: request_token.to_string(),
            })
    }

    /// List all resource operation requests, optionally filtered.
    pub fn list_resource_requests(
        &self,
        operation_filter: Option<&[&str]>,
        status_filter: Option<&[&str]>,
    ) -> Vec<&ResourceRequest> {
        self.requests
            .values()
            .filter(|r| {
                if let Some(ops) = operation_filter {
                    if !ops.contains(&r.operation.as_str()) {
                        return false;
                    }
                }
                if let Some(statuses) = status_filter {
                    if !statuses.contains(&r.operation_status.as_str()) {
                        return false;
                    }
                }
                true
            })
            .collect()
    }

    /// Cancel a resource operation request.
    pub fn cancel_resource_request(
        &mut self,
        request_token: &str,
    ) -> Result<ResourceRequest, CloudControlError> {
        let request = self.requests.get_mut(request_token).ok_or_else(|| {
            CloudControlError::RequestTokenNotFound {
                token: request_token.to_string(),
            }
        })?;

        // In our mock, operations complete synchronously, so they are always SUCCESS.
        // Only PENDING or IN_PROGRESS can be cancelled.
        match request.operation_status {
            OperationStatus::Pending | OperationStatus::InProgress => {
                request.operation_status = OperationStatus::CancelComplete;
                Ok(request.clone())
            }
            _ => Err(CloudControlError::NotCancellable {
                token: request_token.to_string(),
                status: request.operation_status.as_str().to_string(),
            }),
        }
    }
}

/// Extract an identifier from a JSON resource model string.
/// Looks for common identifier fields like "Id", "Name", "Arn".
fn extract_identifier_from_model(model_json: &str) -> Option<String> {
    let parsed: serde_json::Value = serde_json::from_str(model_json).ok()?;
    let obj = parsed.as_object()?;

    // Check common identifier field names
    for field in &[
        "Id",
        "Identifier",
        "Name",
        "Arn",
        "BucketName",
        "FunctionName",
        "TableName",
    ] {
        if let Some(val) = obj.get(*field) {
            if let Some(s) = val.as_str() {
                return Some(s.to_string());
            }
        }
    }
    None
}

/// Apply a single RFC 6902 JSON Patch operation to a JSON value.
fn apply_json_patch(target: &mut serde_json::Value, patch: &serde_json::Value) {
    let op = patch.get("op").and_then(|v| v.as_str()).unwrap_or("");
    let path = patch.get("path").and_then(|v| v.as_str()).unwrap_or("");

    // Simple implementation: only handle top-level paths like "/PropertyName"
    let key = path.trim_start_matches('/');
    if key.is_empty() {
        return;
    }

    match op {
        "add" | "replace" => {
            if let Some(value) = patch.get("value") {
                if let Some(obj) = target.as_object_mut() {
                    obj.insert(key.to_string(), value.clone());
                }
            }
        }
        "remove" => {
            if let Some(obj) = target.as_object_mut() {
                obj.remove(key);
            }
        }
        _ => {} // ignore unsupported operations
    }
}
