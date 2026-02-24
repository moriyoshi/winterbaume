//! Serde-compatible view types for SageMakerRuntime state snapshots.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::SageMakerRuntimeService;
use crate::state::SageMakerRuntimeState;
use crate::types::{AsyncInvocationRecord, InvocationRecord, MockEndpoint};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SageMakerRuntimeStateView {
    #[serde(default)]
    pub endpoints: HashMap<String, MockEndpointView>,
    #[serde(default)]
    pub invocations: Vec<InvocationRecordView>,
    #[serde(default)]
    pub async_invocations: Vec<AsyncInvocationRecordView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MockEndpointView {
    pub endpoint_name: String,
    pub created_at: DateTime<Utc>,
}

/// Invocation body is excluded from snapshots.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvocationRecordView {
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

/// Invocation body is excluded from snapshots.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AsyncInvocationRecordView {
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

impl From<&SageMakerRuntimeState> for SageMakerRuntimeStateView {
    fn from(state: &SageMakerRuntimeState) -> Self {
        SageMakerRuntimeStateView {
            endpoints: state
                .endpoints
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        MockEndpointView {
                            endpoint_name: v.endpoint_name.clone(),
                            created_at: v.created_at,
                        },
                    )
                })
                .collect(),
            invocations: state
                .invocations
                .iter()
                .map(|r| InvocationRecordView {
                    endpoint_name: r.endpoint_name.clone(),
                    content_type: r.content_type.clone(),
                    accept: r.accept.clone(),
                    custom_attributes: r.custom_attributes.clone(),
                    target_model: r.target_model.clone(),
                    target_variant: r.target_variant.clone(),
                    target_container_hostname: r.target_container_hostname.clone(),
                    inference_id: r.inference_id.clone(),
                    inference_component_name: r.inference_component_name.clone(),
                    timestamp: r.timestamp,
                })
                .collect(),
            async_invocations: state
                .async_invocations
                .iter()
                .map(|r| AsyncInvocationRecordView {
                    endpoint_name: r.endpoint_name.clone(),
                    content_type: r.content_type.clone(),
                    accept: r.accept.clone(),
                    custom_attributes: r.custom_attributes.clone(),
                    inference_id: r.inference_id.clone(),
                    input_location: r.input_location.clone(),
                    output_location: r.output_location.clone(),
                    failure_location: r.failure_location.clone(),
                    timestamp: r.timestamp,
                })
                .collect(),
        }
    }
}

impl From<SageMakerRuntimeStateView> for SageMakerRuntimeState {
    fn from(view: SageMakerRuntimeStateView) -> Self {
        let async_invocation_counter = view.async_invocations.len() as u64;
        SageMakerRuntimeState {
            endpoints: view
                .endpoints
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        MockEndpoint {
                            endpoint_name: v.endpoint_name,
                            created_at: v.created_at,
                        },
                    )
                })
                .collect(),
            invocations: view
                .invocations
                .into_iter()
                .map(|r| InvocationRecord {
                    endpoint_name: r.endpoint_name,
                    content_type: r.content_type,
                    accept: r.accept,
                    custom_attributes: r.custom_attributes,
                    target_model: r.target_model,
                    target_variant: r.target_variant,
                    target_container_hostname: r.target_container_hostname,
                    inference_id: r.inference_id,
                    inference_component_name: r.inference_component_name,
                    timestamp: r.timestamp,
                })
                .collect(),
            async_invocations: view
                .async_invocations
                .into_iter()
                .map(|r| AsyncInvocationRecord {
                    endpoint_name: r.endpoint_name,
                    content_type: r.content_type,
                    accept: r.accept,
                    custom_attributes: r.custom_attributes,
                    inference_id: r.inference_id,
                    input_location: r.input_location,
                    output_location: r.output_location,
                    failure_location: r.failure_location,
                    timestamp: r.timestamp,
                })
                .collect(),
            async_invocation_counter,
        }
    }
}

impl StatefulService for SageMakerRuntimeService {
    type StateView = SageMakerRuntimeStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        SageMakerRuntimeStateView::from(&*guard)
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
            *guard = SageMakerRuntimeState::from(view);
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
            for (k, v) in view.endpoints {
                guard.endpoints.insert(
                    k,
                    MockEndpoint {
                        endpoint_name: v.endpoint_name,
                        created_at: v.created_at,
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
