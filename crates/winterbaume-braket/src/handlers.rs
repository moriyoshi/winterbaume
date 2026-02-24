use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
    StatefulService,
    protocol::common::{extract_path, extract_query_string, percent_decode},
};

use crate::state::{BraketError, BraketState};
use crate::views::BraketStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct BraketService {
    pub(crate) state: Arc<BackendState<BraketState>>,
    pub(crate) notifier: StateChangeNotifier<BraketStateView>,
}

impl BraketService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for BraketService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for BraketService {
    fn service_name(&self) -> &str {
        "braket"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://braket\..*\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

type SharedState = Arc<tokio::sync::RwLock<BraketState>>;

impl BraketService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;
        let state = self.state.get(account_id, &region);
        {
            let mut guard = state.write().await;
            guard.ensure_seeded();
        }

        let path = extract_path(&request.uri);
        let raw_segments: Vec<String> = path
            .trim_start_matches('/')
            .split('/')
            .map(percent_decode)
            .collect();
        let segments: Vec<&str> = raw_segments.iter().map(|s| s.as_str()).collect();
        let query = extract_query_string(&request.uri);
        let method = request.method.as_str().to_string();
        let body: Value = if request.body.is_empty() {
            json!({})
        } else {
            serde_json::from_slice(&request.body).unwrap_or(json!({}))
        };

        let (response, mutating) = match (method.as_str(), segments.as_slice()) {
            ("POST", ["job"]) => (self.handle_create_job(&state, &body, &region).await, true),
            ("GET", ["job", arn]) => (self.handle_get_job(&state, arn).await, false),
            ("PUT", ["job", arn, "cancel"]) => (self.handle_cancel_job(&state, arn).await, true),
            ("POST", ["jobs"]) => (self.handle_search_jobs(&state).await, false),
            ("POST", ["quantum-task"]) => {
                (self.handle_create_task(&state, &body, &region).await, true)
            }
            ("GET", ["quantum-task", arn]) => (self.handle_get_task(&state, arn).await, false),
            ("PUT", ["quantum-task", arn, "cancel"]) => {
                (self.handle_cancel_task(&state, arn).await, true)
            }
            ("POST", ["quantum-tasks"]) => (self.handle_search_tasks(&state).await, false),
            ("GET", ["device", arn]) => (self.handle_get_device(&state, arn).await, false),
            ("POST", ["devices"]) => (self.handle_search_devices(&state).await, false),
            ("POST", ["spending-limit"]) => (
                self.handle_create_spending_limit(&state, &body, &region)
                    .await,
                true,
            ),
            ("DELETE", ["spending-limit", arn, "delete"]) => {
                (self.handle_delete_spending_limit(&state, arn).await, true)
            }
            ("PATCH", ["spending-limit", arn, "update"]) => (
                self.handle_update_spending_limit(&state, arn, &body).await,
                true,
            ),
            ("POST", ["spending-limits"]) => {
                (self.handle_search_spending_limits(&state).await, false)
            }
            ("GET", ["tags", rest @ ..]) => {
                let arn = percent_decode(&rest.join("/"));
                (self.handle_list_tags(&state, &arn).await, false)
            }
            ("POST", ["tags", rest @ ..]) => {
                let arn = percent_decode(&rest.join("/"));
                (self.handle_tag_resource(&state, &arn, &body).await, true)
            }
            ("DELETE", ["tags", rest @ ..]) => {
                let arn = percent_decode(&rest.join("/"));
                let keys: Vec<String> = url_query_get_all(query, "tagKeys")
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect();
                (self.handle_untag_resource(&state, &arn, &keys).await, true)
            }
            _ => (
                rest_json_error(404, "ResourceNotFoundException", "No route matches"),
                false,
            ),
        };

        if response.status / 100 == 2 && mutating {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_create_job(
        &self,
        state: &SharedState,
        body: &Value,
        region: &str,
    ) -> MockResponse {
        let job_name = match body.get("jobName").and_then(|v| v.as_str()) {
            Some(s) if !s.is_empty() => s.to_string(),
            _ => return rest_json_error(400, "ValidationException", "jobName is required"),
        };
        let arn = format!("arn:aws:braket:{region}:123456789012:job/{job_name}");
        let now = chrono::Utc::now().to_rfc3339();
        let mut record = body.clone();
        if let Value::Object(ref mut map) = record {
            map.insert("jobArn".into(), json!(arn));
            map.insert("status".into(), json!("QUEUED"));
            map.insert("createdAt".into(), json!(now));
        }
        let tag_map = body
            .get("tags")
            .and_then(|v| v.as_object())
            .map(|m| {
                m.iter()
                    .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                    .collect::<HashMap<_, _>>()
            })
            .unwrap_or_default();
        let mut state = state.write().await;
        state.jobs.insert(arn.clone(), record);
        if !tag_map.is_empty() {
            state.tag_resource(&arn, tag_map);
        }
        wire::serialize_create_job_response(&wire::CreateJobResponse { job_arn: Some(arn) })
    }

    async fn handle_get_job(&self, state: &SharedState, arn: &str) -> MockResponse {
        let state = state.read().await;
        match state.jobs.get(arn) {
            Some(v) => {
                let result: wire::GetJobResponse =
                    serde_json::from_value(v.clone()).unwrap_or_default();
                wire::serialize_get_job_response(&result)
            }
            None => err_response(&BraketError::NotFound {
                arn: arn.to_string(),
            }),
        }
    }

    async fn handle_cancel_job(&self, state: &SharedState, arn: &str) -> MockResponse {
        let mut state = state.write().await;
        match state.jobs.get_mut(arn) {
            Some(v) => {
                if let Value::Object(map) = v {
                    map.insert("status".into(), json!("CANCELLING"));
                }
                wire::serialize_cancel_job_response(&wire::CancelJobResponse {
                    job_arn: Some(arn.to_string()),
                    cancellation_status: Some("CANCELLING".to_string()),
                })
            }
            None => err_response(&BraketError::NotFound {
                arn: arn.to_string(),
            }),
        }
    }

    async fn handle_search_jobs(&self, state: &SharedState) -> MockResponse {
        let state = state.read().await;
        let jobs: Vec<wire::JobSummary> = state
            .list_jobs()
            .into_iter()
            .map(|j| wire::JobSummary {
                job_arn: j.get("jobArn").and_then(|v| v.as_str()).map(str::to_string),
                job_name: j
                    .get("jobName")
                    .and_then(|v| v.as_str())
                    .map(str::to_string),
                device: j
                    .get("deviceConfig")
                    .and_then(|d| d.get("device"))
                    .and_then(|v| v.as_str())
                    .map(str::to_string),
                status: j.get("status").and_then(|v| v.as_str()).map(str::to_string),
                created_at: j
                    .get("createdAt")
                    .and_then(|v| v.as_str())
                    .map(str::to_string),
                ..Default::default()
            })
            .collect();
        wire::serialize_search_jobs_response(&wire::SearchJobsResponse {
            jobs: Some(jobs),
            next_token: None,
        })
    }

    async fn handle_create_task(
        &self,
        state: &SharedState,
        body: &Value,
        region: &str,
    ) -> MockResponse {
        let device_arn = match body.get("deviceArn").and_then(|v| v.as_str()) {
            Some(s) if !s.is_empty() => s.to_string(),
            _ => return rest_json_error(400, "ValidationException", "deviceArn is required"),
        };
        let id = uuid::Uuid::new_v4().simple().to_string();
        let arn = format!("arn:aws:braket:{region}:123456789012:quantum-task/{id}");
        let now = chrono::Utc::now().to_rfc3339();
        let mut record = body.clone();
        if let Value::Object(ref mut map) = record {
            map.insert("quantumTaskArn".into(), json!(arn));
            map.insert("deviceArn".into(), json!(device_arn));
            map.insert("status".into(), json!("CREATED"));
            map.insert("createdAt".into(), json!(now));
        }
        let mut state = state.write().await;
        state.tasks.insert(arn.clone(), record);
        wire::serialize_create_quantum_task_response(&wire::CreateQuantumTaskResponse {
            quantum_task_arn: Some(arn),
        })
    }

    async fn handle_get_task(&self, state: &SharedState, arn: &str) -> MockResponse {
        let state = state.read().await;
        match state.tasks.get(arn) {
            Some(v) => {
                let result: wire::GetQuantumTaskResponse =
                    serde_json::from_value(v.clone()).unwrap_or_default();
                wire::serialize_get_quantum_task_response(&result)
            }
            None => err_response(&BraketError::NotFound {
                arn: arn.to_string(),
            }),
        }
    }

    async fn handle_cancel_task(&self, state: &SharedState, arn: &str) -> MockResponse {
        let mut state = state.write().await;
        match state.tasks.get_mut(arn) {
            Some(v) => {
                if let Value::Object(map) = v {
                    map.insert("status".into(), json!("CANCELLING"));
                }
                wire::serialize_cancel_quantum_task_response(&wire::CancelQuantumTaskResponse {
                    quantum_task_arn: Some(arn.to_string()),
                    cancellation_status: Some("CANCELLING".to_string()),
                })
            }
            None => err_response(&BraketError::NotFound {
                arn: arn.to_string(),
            }),
        }
    }

    async fn handle_search_tasks(&self, state: &SharedState) -> MockResponse {
        let state = state.read().await;
        let tasks: Vec<wire::QuantumTaskSummary> = state
            .list_tasks()
            .into_iter()
            .map(|t| wire::QuantumTaskSummary {
                quantum_task_arn: t
                    .get("quantumTaskArn")
                    .and_then(|v| v.as_str())
                    .map(str::to_string),
                device_arn: t
                    .get("deviceArn")
                    .and_then(|v| v.as_str())
                    .map(str::to_string),
                status: t.get("status").and_then(|v| v.as_str()).map(str::to_string),
                shots: t.get("shots").and_then(|v| v.as_i64()),
                output_s3_bucket: t
                    .get("outputS3Bucket")
                    .and_then(|v| v.as_str())
                    .map(str::to_string),
                output_s3_directory: t
                    .get("outputS3Directory")
                    .and_then(|v| v.as_str())
                    .map(str::to_string),
                created_at: t
                    .get("createdAt")
                    .and_then(|v| v.as_str())
                    .map(str::to_string),
                ..Default::default()
            })
            .collect();
        wire::serialize_search_quantum_tasks_response(&wire::SearchQuantumTasksResponse {
            quantum_tasks: Some(tasks),
            next_token: None,
        })
    }

    async fn handle_get_device(&self, state: &SharedState, arn: &str) -> MockResponse {
        let state = state.read().await;
        match state.devices.get(arn) {
            Some(v) => {
                let result: wire::GetDeviceResponse =
                    serde_json::from_value(v.clone()).unwrap_or_default();
                wire::serialize_get_device_response(&result)
            }
            None => err_response(&BraketError::NotFound {
                arn: arn.to_string(),
            }),
        }
    }

    async fn handle_search_devices(&self, state: &SharedState) -> MockResponse {
        let state = state.read().await;
        let devices: Vec<wire::DeviceSummary> = state
            .list_devices()
            .into_iter()
            .map(|d| wire::DeviceSummary {
                device_arn: d
                    .get("deviceArn")
                    .and_then(|v| v.as_str())
                    .map(str::to_string),
                device_name: d
                    .get("deviceName")
                    .and_then(|v| v.as_str())
                    .map(str::to_string),
                device_type: d
                    .get("deviceType")
                    .and_then(|v| v.as_str())
                    .map(str::to_string),
                device_status: d
                    .get("deviceStatus")
                    .and_then(|v| v.as_str())
                    .map(str::to_string),
                provider_name: d
                    .get("providerName")
                    .and_then(|v| v.as_str())
                    .map(str::to_string),
            })
            .collect();
        wire::serialize_search_devices_response(&wire::SearchDevicesResponse {
            devices: Some(devices),
            next_token: None,
        })
    }

    async fn handle_create_spending_limit(
        &self,
        state: &SharedState,
        body: &Value,
        region: &str,
    ) -> MockResponse {
        let device_arn = match body.get("deviceArn").and_then(|v| v.as_str()) {
            Some(s) if !s.is_empty() => s.to_string(),
            _ => return rest_json_error(400, "ValidationException", "deviceArn is required"),
        };
        let id = uuid::Uuid::new_v4().simple().to_string();
        let arn = format!("arn:aws:braket:{region}:123456789012:spending-limit/{id}");
        let now = chrono::Utc::now().to_rfc3339();
        let mut record = body.clone();
        if let Value::Object(ref mut map) = record {
            map.insert("spendingLimitArn".into(), json!(arn));
            map.insert("deviceArn".into(), json!(device_arn));
            map.insert("status".into(), json!("ACTIVE"));
            map.insert("createdAt".into(), json!(now));
        }
        let mut state = state.write().await;
        state.spending_limits.insert(arn.clone(), record);
        wire::serialize_create_spending_limit_response(&wire::CreateSpendingLimitResponse {
            spending_limit_arn: Some(arn),
        })
    }

    async fn handle_delete_spending_limit(&self, state: &SharedState, arn: &str) -> MockResponse {
        let mut state = state.write().await;
        if state.spending_limits.remove(arn).is_some() {
            wire::serialize_delete_spending_limit_response(&wire::DeleteSpendingLimitResponse {})
        } else {
            err_response(&BraketError::NotFound {
                arn: arn.to_string(),
            })
        }
    }

    async fn handle_update_spending_limit(
        &self,
        state: &SharedState,
        arn: &str,
        body: &Value,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.spending_limits.get_mut(arn) {
            Some(v) => {
                if let (Value::Object(target), Value::Object(src)) = (v, body) {
                    for (k, val) in src {
                        target.insert(k.clone(), val.clone());
                    }
                }
                wire::serialize_update_spending_limit_response(
                    &wire::UpdateSpendingLimitResponse {},
                )
            }
            None => err_response(&BraketError::NotFound {
                arn: arn.to_string(),
            }),
        }
    }

    async fn handle_search_spending_limits(&self, state: &SharedState) -> MockResponse {
        let state = state.read().await;
        let limits: Vec<wire::SpendingLimitSummary> = state
            .list_spending_limits()
            .into_iter()
            .map(|l| wire::SpendingLimitSummary {
                spending_limit_arn: l
                    .get("spendingLimitArn")
                    .and_then(|v| v.as_str())
                    .map(str::to_string),
                device_arn: l
                    .get("deviceArn")
                    .and_then(|v| v.as_str())
                    .map(str::to_string),
                spending_limit: l
                    .get("limitAmount")
                    .and_then(|v| v.as_str())
                    .map(str::to_string),
                created_at: l
                    .get("createdAt")
                    .and_then(|v| v.as_str())
                    .map(str::to_string),
                ..Default::default()
            })
            .collect();
        wire::serialize_search_spending_limits_response(&wire::SearchSpendingLimitsResponse {
            spending_limits: Some(limits),
            next_token: None,
        })
    }

    async fn handle_list_tags(&self, state: &SharedState, arn: &str) -> MockResponse {
        let state = state.read().await;
        let tags = state.list_tags(arn);
        wire::serialize_list_tags_for_resource_response(&wire::ListTagsForResourceResponse {
            tags: Some(tags),
        })
    }

    async fn handle_tag_resource(
        &self,
        state: &SharedState,
        arn: &str,
        body: &Value,
    ) -> MockResponse {
        let tag_map: HashMap<String, String> = body
            .get("tags")
            .and_then(|v| v.as_object())
            .map(|m| {
                m.iter()
                    .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                    .collect()
            })
            .unwrap_or_default();
        let mut state = state.write().await;
        state.tag_resource(arn, tag_map);
        wire::serialize_tag_resource_response(&wire::TagResourceResponse {})
    }

    async fn handle_untag_resource(
        &self,
        state: &SharedState,
        arn: &str,
        keys: &[String],
    ) -> MockResponse {
        let mut state = state.write().await;
        state.untag_resource(arn, keys);
        wire::serialize_untag_resource_response(&wire::UntagResourceResponse {})
    }
}

fn rest_json_error(status: u16, error_type: &str, message: &str) -> MockResponse {
    let body = json!({"__type": error_type, "message": message});
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}

fn err_response(err: &BraketError) -> MockResponse {
    let (status, error_type) = match err {
        BraketError::NotFound { .. } => (404, "ResourceNotFoundException"),
        BraketError::Validation { .. } => (400, "ValidationException"),
    };
    rest_json_error(status, error_type, &err.to_string())
}

fn url_query_get_all<'a>(query: &'a str, key: &str) -> Vec<&'a str> {
    query
        .split('&')
        .filter_map(|kv| {
            let (k, v) = kv.split_once('=').unwrap_or((kv, ""));
            if k == key { Some(v) } else { None }
        })
        .collect()
}
