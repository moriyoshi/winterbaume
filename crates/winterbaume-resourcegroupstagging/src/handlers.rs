use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
    StatefulService,
};

use crate::state::TaggingState;
use crate::views::TaggingStateView;
use crate::wire;

pub struct TaggingService {
    pub(crate) state: Arc<BackendState<TaggingState>>,
    pub(crate) notifier: StateChangeNotifier<TaggingStateView>,
}

impl TaggingService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for TaggingService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for TaggingService {
    fn service_name(&self) -> &str {
        "tagging"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://tagging\..*\.amazonaws\.com",
            r"https?://tagging\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl TaggingService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;

        let action = request
            .headers
            .get("x-amz-target")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.split('.').next_back())
            .map(|s| s.to_string());

        let action = match action {
            Some(a) => a,
            None => {
                return json_error_response(400, "MissingAction", "Missing X-Amz-Target header");
            }
        };

        let body: Value = match serde_json::from_slice(&request.body) {
            Ok(v) => v,
            Err(_) => {
                return json_error_response(400, "SerializationException", "Invalid JSON body");
            }
        };

        let state = self.state.get(account_id, &region);

        let response = match action.as_str() {
            "TagResources" => self.handle_tag_resources(&state, &body).await,
            "UntagResources" => self.handle_untag_resources(&state, &body).await,
            "GetResources" => self.handle_get_resources(&state, &body).await,
            "GetTagKeys" => self.handle_get_tag_keys(&state).await,
            "GetTagValues" => self.handle_get_tag_values(&state, &body).await,
            // --- Unimplemented operations ---
            "DescribeReportCreation" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeReportCreation is not yet implemented in winterbaume-tagging",
            ),
            "GetComplianceSummary" => json_error_response(
                501,
                "NotImplementedError",
                "GetComplianceSummary is not yet implemented in winterbaume-tagging",
            ),
            "ListRequiredTags" => json_error_response(
                501,
                "NotImplementedError",
                "ListRequiredTags is not yet implemented in winterbaume-tagging",
            ),
            "StartReportCreation" => json_error_response(
                501,
                "NotImplementedError",
                "StartReportCreation is not yet implemented in winterbaume-tagging",
            ),
            _ => json_error_response(400, "InvalidAction", &format!("Unknown operation {action}")),
        };
        if response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_tag_resources(
        &self,
        state: &Arc<tokio::sync::RwLock<TaggingState>>,
        body: &Value,
    ) -> MockResponse {
        let arns: Vec<String> = body
            .get("ResourceARNList")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();

        if arns.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "ResourceARNList is required and must not be empty",
            );
        }

        let tags: HashMap<String, String> = body
            .get("Tags")
            .and_then(|v| v.as_object())
            .map(|obj| {
                obj.iter()
                    .filter_map(|(k, v)| v.as_str().map(|val| (k.clone(), val.to_string())))
                    .collect()
            })
            .unwrap_or_default();

        if tags.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "Tags is required and must not be empty",
            );
        }

        let mut state = state.write().await;
        let failures = state.tag_resources(&arns, &tags);

        let failed_map = failures_to_map(failures);

        let output = wire::TagResourcesOutput {
            failed_resources_map: Some(failed_map),
        };
        wire::serialize_tag_resources_response(&output)
    }

    async fn handle_untag_resources(
        &self,
        state: &Arc<tokio::sync::RwLock<TaggingState>>,
        body: &Value,
    ) -> MockResponse {
        let arns: Vec<String> = body
            .get("ResourceARNList")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();

        if arns.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "ResourceARNList is required and must not be empty",
            );
        }

        let tag_keys: Vec<String> = body
            .get("TagKeys")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();

        if tag_keys.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "TagKeys is required and must not be empty",
            );
        }

        let mut state = state.write().await;
        let failures = state.untag_resources(&arns, &tag_keys);

        let failed_map = failures_to_map(failures);

        let output = wire::UntagResourcesOutput {
            failed_resources_map: Some(failed_map),
        };
        wire::serialize_untag_resources_response(&output)
    }

    async fn handle_get_resources(
        &self,
        state: &Arc<tokio::sync::RwLock<TaggingState>>,
        body: &Value,
    ) -> MockResponse {
        let tag_filters: Vec<(String, Vec<String>)> = body
            .get("TagFilters")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|f| {
                        let key = f.get("Key")?.as_str()?.to_string();
                        let values: Vec<String> = f
                            .get("Values")
                            .and_then(|v| v.as_array())
                            .map(|vals| {
                                vals.iter()
                                    .filter_map(|v| v.as_str().map(String::from))
                                    .collect()
                            })
                            .unwrap_or_default();
                        Some((key, values))
                    })
                    .collect()
            })
            .unwrap_or_default();

        let resource_type_filters: Vec<String> = body
            .get("ResourceTypeFilters")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();

        let arn_filters: Vec<String> = body
            .get("ResourceARNList")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();

        let state = state.read().await;
        let mut resources = state.get_resources(&tag_filters, &resource_type_filters);
        if !arn_filters.is_empty() {
            resources.retain(|r| arn_filters.contains(&r.arn));
        }

        let mapping_list: Vec<wire::ResourceTagMapping> = resources
            .iter()
            .map(|r| {
                let tags: Vec<wire::Tag> = r
                    .tags
                    .iter()
                    .map(|(k, v)| wire::Tag {
                        key: Some(k.clone()),
                        value: Some(v.clone()),
                    })
                    .collect();
                wire::ResourceTagMapping {
                    resource_a_r_n: Some(r.arn.clone()),
                    tags: Some(tags),
                    compliance_details: None,
                }
            })
            .collect();

        let output = wire::GetResourcesOutput {
            pagination_token: Some(String::new()),
            resource_tag_mapping_list: Some(mapping_list),
        };
        wire::serialize_get_resources_response(&output)
    }

    async fn handle_get_tag_keys(
        &self,
        state: &Arc<tokio::sync::RwLock<TaggingState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let keys = state.get_tag_keys();

        let output = wire::GetTagKeysOutput {
            pagination_token: Some(String::new()),
            tag_keys: Some(keys),
        };
        wire::serialize_get_tag_keys_response(&output)
    }

    async fn handle_get_tag_values(
        &self,
        state: &Arc<tokio::sync::RwLock<TaggingState>>,
        body: &Value,
    ) -> MockResponse {
        let key = match body.get("Key").and_then(|v| v.as_str()) {
            Some(k) => k,
            None => {
                return json_error_response(400, "InvalidParameterException", "Key is required");
            }
        };

        let state = state.read().await;
        let values = state.get_tag_values(key);

        let output = wire::GetTagValuesOutput {
            pagination_token: Some(String::new()),
            tag_values: Some(values),
        };
        wire::serialize_get_tag_values_response(&output)
    }
}

/// Convert state-level TaggingError failures into wire FailureInfo map.
fn failures_to_map(
    failures: HashMap<String, crate::state::TaggingError>,
) -> HashMap<String, wire::FailureInfo> {
    use crate::state::TaggingError;
    failures
        .into_iter()
        .map(|(arn, err)| {
            let (status_code, error_code) = match &err {
                TaggingError::EmptyArn => (400, "InvalidParameterException"),
            };
            (
                arn,
                wire::FailureInfo {
                    status_code: Some(status_code),
                    error_code: Some(error_code.to_string()),
                    error_message: Some(err.to_string()),
                },
            )
        })
        .collect()
}

fn json_error_response(status: u16, error_type: &str, message: &str) -> MockResponse {
    MockResponse::json(
        status,
        json!({"__type": error_type, "message": message}).to_string(),
    )
}
