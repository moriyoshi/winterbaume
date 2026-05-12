use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, default_account_id,
};

use crate::state::{MediaStoreError, MediaStoreState};
use crate::types::Tag;
use crate::views::MediaStoreStateView;
use crate::wire;

pub struct MediaStoreService {
    pub(crate) state: Arc<BackendState<MediaStoreState>>,
    pub(crate) notifier: StateChangeNotifier<MediaStoreStateView>,
}

impl MediaStoreService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for MediaStoreService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for MediaStoreService {
    fn service_name(&self) -> &str {
        "mediastore"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://mediastore\..*\.amazonaws\.com",
            r"https?://mediastore\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl MediaStoreService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();
        let state = self.state.get(account_id, &region);

        // MediaStore uses awsJson1.1 protocol with X-Amz-Target header
        let target = request
            .headers
            .get("x-amz-target")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");

        // Extract operation name from target like "MediaStore_20170901.CreateContainer"
        let operation = target.split('.').next_back().unwrap_or("");

        let body: Value = if request.body.is_empty() {
            json!({})
        } else {
            match serde_json::from_slice(&request.body) {
                Ok(v) => v,
                Err(_) => return json_error(400, "SerializationException", "Invalid JSON"),
            }
        };

        match operation {
            "CreateContainer" => {
                let name = body
                    .get("ContainerName")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let tags = parse_tags(body.get("Tags"));

                let mut state = state.write().await;
                match state.create_container(name, tags) {
                    Ok(container) => {
                        let output = wire::CreateContainerOutput {
                            container: Some(container_to_wire(container)),
                        };
                        wire::serialize_create_container_response(&output)
                    }
                    Err(e) => mediastore_error_response(&e),
                }
            }
            "DescribeContainer" => {
                let name = body
                    .get("ContainerName")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let mut state = state.write().await;
                match state.describe_container(name) {
                    Ok(container) => {
                        let output = wire::DescribeContainerOutput {
                            container: Some(container_to_wire(container)),
                        };
                        wire::serialize_describe_container_response(&output)
                    }
                    Err(e) => mediastore_error_response(&e),
                }
            }
            "DeleteContainer" => {
                let name = body
                    .get("ContainerName")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let mut state = state.write().await;
                match state.delete_container(name) {
                    Ok(()) => {
                        wire::serialize_delete_container_response(&wire::DeleteContainerOutput {})
                    }
                    Err(e) => mediastore_error_response(&e),
                }
            }
            "ListContainers" => {
                let state = state.read().await;
                let containers = state.list_containers();
                let output = wire::ListContainersOutput {
                    containers: Some(containers.iter().map(|c| container_to_wire(c)).collect()),
                    next_token: None,
                };
                wire::serialize_list_containers_response(&output)
            }
            "ListTagsForResource" => {
                // FIX(terraform-e2e): terraform sends the full container ARN
                // (`arn:aws:mediastore:<region>:<account>:container/<name>`); the state
                // map is keyed by container name, so split on `/` to extract the trailing
                // path component before lookup.
                let resource = body.get("Resource").and_then(|v| v.as_str()).unwrap_or("");
                let name = resource.rsplit('/').next().unwrap_or(resource);
                let state = state.read().await;
                match state.list_tags_for_resource(name) {
                    Ok(Some(tags)) => {
                        let output = wire::ListTagsForResourceOutput {
                            tags: Some(tags.iter().map(tag_to_wire).collect()),
                        };
                        wire::serialize_list_tags_for_resource_response(&output)
                    }
                    Ok(None) => {
                        let output = wire::ListTagsForResourceOutput { tags: None };
                        wire::serialize_list_tags_for_resource_response(&output)
                    }
                    Err(e) => mediastore_error_response(&e),
                }
            }
            "PutLifecyclePolicy" => {
                let container_name = body
                    .get("ContainerName")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let lifecycle_policy = body
                    .get("LifecyclePolicy")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let mut state = state.write().await;
                match state.put_lifecycle_policy(container_name, lifecycle_policy) {
                    Ok(()) => wire::serialize_put_lifecycle_policy_response(
                        &wire::PutLifecyclePolicyOutput {},
                    ),
                    Err(e) => mediastore_error_response(&e),
                }
            }
            "GetLifecyclePolicy" => {
                let container_name = body
                    .get("ContainerName")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let state = state.read().await;
                match state.get_lifecycle_policy(container_name) {
                    Ok(policy) => {
                        let output = wire::GetLifecyclePolicyOutput {
                            lifecycle_policy: Some(policy.to_string()),
                        };
                        wire::serialize_get_lifecycle_policy_response(&output)
                    }
                    Err(e) => mediastore_error_response(&e),
                }
            }
            "PutContainerPolicy" => {
                let container_name = body
                    .get("ContainerName")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let policy = body.get("Policy").and_then(|v| v.as_str()).unwrap_or("");
                let mut state = state.write().await;
                match state.put_container_policy(container_name, policy) {
                    Ok(()) => wire::serialize_put_container_policy_response(
                        &wire::PutContainerPolicyOutput {},
                    ),
                    Err(e) => mediastore_error_response(&e),
                }
            }
            "GetContainerPolicy" => {
                let container_name = body
                    .get("ContainerName")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let state = state.read().await;
                match state.get_container_policy(container_name) {
                    Ok(policy) => {
                        let output = wire::GetContainerPolicyOutput {
                            policy: Some(policy.to_string()),
                        };
                        wire::serialize_get_container_policy_response(&output)
                    }
                    Err(e) => mediastore_error_response(&e),
                }
            }
            "PutMetricPolicy" => {
                let container_name = body
                    .get("ContainerName")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let metric_policy = body.get("MetricPolicy").cloned().unwrap_or(json!(null));
                let mut state = state.write().await;
                match state.put_metric_policy(container_name, metric_policy) {
                    Ok(()) => {
                        wire::serialize_put_metric_policy_response(&wire::PutMetricPolicyOutput {})
                    }
                    Err(e) => mediastore_error_response(&e),
                }
            }
            "GetMetricPolicy" => {
                let container_name = body
                    .get("ContainerName")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let state = state.read().await;
                match state.get_metric_policy(container_name) {
                    Ok(policy) => {
                        let output = wire::GetMetricPolicyOutput {
                            metric_policy: Some(metric_policy_from_value(policy)),
                        };
                        wire::serialize_get_metric_policy_response(&output)
                    }
                    Err(e) => mediastore_error_response(&e),
                }
            }
            _ => json_error(
                400,
                "UnknownOperationException",
                &format!("Unknown operation: {operation}"),
            ),
        }
    }
}

fn container_to_wire(container: &crate::types::Container) -> wire::Container {
    let creation_time: f64 = container.creation_time.parse().unwrap_or(0.0);
    wire::Container {
        a_r_n: Some(container.arn.clone()),
        access_logging_enabled: None,
        creation_time: Some(creation_time),
        endpoint: Some(container.endpoint.clone()),
        name: Some(container.name.clone()),
        status: Some(container.status.clone()),
    }
}

fn tag_to_wire(tag: &Tag) -> wire::Tag {
    wire::Tag {
        key: tag.key.clone(),
        value: tag.value.clone(),
    }
}

fn metric_policy_from_value(value: &Value) -> wire::MetricPolicy {
    let container_level_metrics = value
        .get("ContainerLevelMetrics")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let metric_policy_rules = value.get("MetricPolicyRules").and_then(|v| {
        v.as_array().map(|arr| {
            arr.iter()
                .map(|r| wire::MetricPolicyRule {
                    object_group: r
                        .get("ObjectGroup")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                    object_group_name: r
                        .get("ObjectGroupName")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                })
                .collect()
        })
    });
    wire::MetricPolicy {
        container_level_metrics,
        metric_policy_rules,
    }
}

fn parse_tags(tags_value: Option<&Value>) -> Option<Vec<Tag>> {
    tags_value.and_then(|v| {
        v.as_array().map(|arr| {
            arr.iter()
                .map(|t| Tag {
                    key: t
                        .get("Key")
                        .and_then(|k| k.as_str())
                        .unwrap_or("")
                        .to_string(),
                    value: t
                        .get("Value")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string()),
                })
                .collect()
        })
    })
}

fn mediastore_error_response(err: &MediaStoreError) -> MockResponse {
    match err {
        MediaStoreError::ContainerInUse { .. } => {
            json_error(400, "ContainerInUseException", &err.to_string())
        }
        MediaStoreError::ResourceNotFound => {
            json_error(400, "ResourceNotFoundException", &err.to_string())
        }
        MediaStoreError::ContainerNotFound => {
            json_error(400, "ContainerNotFoundException", &err.to_string())
        }
        MediaStoreError::PolicyNotFound => {
            json_error(400, "PolicyNotFoundException", &err.to_string())
        }
    }
}

fn json_error(status: u16, code: &str, message: &str) -> MockResponse {
    let body = json!({
        "__type": code,
        "message": message,
    });
    MockResponse::json(status, body.to_string())
}
