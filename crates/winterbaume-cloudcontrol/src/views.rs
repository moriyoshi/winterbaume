//! Serde-compatible view types for Cloud Control API state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::CloudControlService;
use crate::state::CloudControlState;
use crate::types::{ManagedResource, OperationStatus, OperationType, ResourceRequest};

/// Serializable view of the entire Cloud Control API state for one account/region.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CloudControlStateView {
    /// Resources keyed by "{type_name}|{identifier}".
    #[serde(default)]
    pub resources: HashMap<String, ResourceView>,
    /// Operation requests keyed by request_token.
    #[serde(default)]
    pub requests: HashMap<String, RequestView>,
}

/// Serializable view of a managed resource.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceView {
    pub type_name: String,
    pub identifier: String,
    pub resource_model: String,
}

/// Serializable view of a resource operation request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestView {
    pub request_token: String,
    pub type_name: String,
    pub identifier: String,
    pub operation: String,
    pub operation_status: String,
    pub event_time: String,
    #[serde(default)]
    pub resource_model: Option<String>,
    #[serde(default)]
    pub status_message: Option<String>,
    #[serde(default)]
    pub error_code: Option<String>,
}

impl From<&CloudControlState> for CloudControlStateView {
    fn from(state: &CloudControlState) -> Self {
        Self {
            resources: state
                .resources
                .iter()
                .map(|((type_name, identifier), r)| {
                    let key = format!("{}|{}", type_name, identifier);
                    (
                        key,
                        ResourceView {
                            type_name: r.type_name.clone(),
                            identifier: r.identifier.clone(),
                            resource_model: r.resource_model.clone(),
                        },
                    )
                })
                .collect(),
            requests: state
                .requests
                .iter()
                .map(|(token, r)| {
                    (
                        token.clone(),
                        RequestView {
                            request_token: r.request_token.clone(),
                            type_name: r.type_name.clone(),
                            identifier: r.identifier.clone(),
                            operation: r.operation.as_str().to_string(),
                            operation_status: r.operation_status.as_str().to_string(),
                            event_time: r.event_time.to_rfc3339(),
                            resource_model: r.resource_model.clone(),
                            status_message: r.status_message.clone(),
                            error_code: r.error_code.clone(),
                        },
                    )
                })
                .collect(),
        }
    }
}

impl From<CloudControlStateView> for CloudControlState {
    fn from(view: CloudControlStateView) -> Self {
        let resources = view
            .resources
            .into_values()
            .map(|rv| {
                let key = (rv.type_name.clone(), rv.identifier.clone());
                let resource = ManagedResource {
                    type_name: rv.type_name,
                    identifier: rv.identifier,
                    resource_model: rv.resource_model,
                };
                (key, resource)
            })
            .collect();

        let requests = view
            .requests
            .into_values()
            .map(|rv| {
                let operation = match rv.operation.as_str() {
                    "DELETE" => OperationType::Delete,
                    "UPDATE" => OperationType::Update,
                    _ => OperationType::Create,
                };
                let operation_status = match rv.operation_status.as_str() {
                    "PENDING" => OperationStatus::Pending,
                    "IN_PROGRESS" => OperationStatus::InProgress,
                    "FAILED" => OperationStatus::Failed,
                    "CANCEL_IN_PROGRESS" => OperationStatus::CancelInProgress,
                    "CANCEL_COMPLETE" => OperationStatus::CancelComplete,
                    _ => OperationStatus::Success,
                };
                let event_time = chrono::DateTime::parse_from_rfc3339(&rv.event_time)
                    .map(|dt| dt.with_timezone(&chrono::Utc))
                    .unwrap_or_else(|_| chrono::Utc::now());

                let request = ResourceRequest {
                    request_token: rv.request_token.clone(),
                    type_name: rv.type_name,
                    identifier: rv.identifier,
                    operation,
                    operation_status,
                    event_time,
                    resource_model: rv.resource_model,
                    status_message: rv.status_message,
                    error_code: rv.error_code,
                };
                (rv.request_token, request)
            })
            .collect();

        CloudControlState {
            resources,
            requests,
        }
    }
}

impl StatefulService for CloudControlService {
    type StateView = CloudControlStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        CloudControlStateView::from(&*guard)
    }

    async fn restore(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            *guard = CloudControlState::from(view);
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    async fn merge(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            for rv in view.resources.into_values() {
                let key = (rv.type_name.clone(), rv.identifier.clone());
                guard.resources.insert(
                    key,
                    ManagedResource {
                        type_name: rv.type_name,
                        identifier: rv.identifier,
                        resource_model: rv.resource_model,
                    },
                );
            }
            // Merge requests additively
            for rv in view.requests.into_values() {
                let operation = match rv.operation.as_str() {
                    "DELETE" => OperationType::Delete,
                    "UPDATE" => OperationType::Update,
                    _ => OperationType::Create,
                };
                let operation_status = match rv.operation_status.as_str() {
                    "PENDING" => OperationStatus::Pending,
                    "IN_PROGRESS" => OperationStatus::InProgress,
                    "FAILED" => OperationStatus::Failed,
                    "CANCEL_IN_PROGRESS" => OperationStatus::CancelInProgress,
                    "CANCEL_COMPLETE" => OperationStatus::CancelComplete,
                    _ => OperationStatus::Success,
                };
                let event_time = chrono::DateTime::parse_from_rfc3339(&rv.event_time)
                    .map(|dt| dt.with_timezone(&chrono::Utc))
                    .unwrap_or_else(|_| chrono::Utc::now());

                guard.requests.insert(
                    rv.request_token.clone(),
                    ResourceRequest {
                        request_token: rv.request_token,
                        type_name: rv.type_name,
                        identifier: rv.identifier,
                        operation,
                        operation_status,
                        event_time,
                        resource_model: rv.resource_model,
                        status_message: rv.status_message,
                        error_code: rv.error_code,
                    },
                );
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
