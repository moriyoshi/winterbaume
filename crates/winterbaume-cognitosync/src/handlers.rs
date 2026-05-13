use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, StatefulService,
    default_account_id,
    protocol::common::{extract_path, percent_decode},
};

use crate::state::{CognitoSyncError, CognitoSyncState, DatasetRecord, DeviceRecord, RecordEntry};
use crate::views::CognitoSyncStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct CognitoSyncService {
    pub(crate) state: Arc<BackendState<CognitoSyncState>>,
    pub(crate) notifier: StateChangeNotifier<CognitoSyncStateView>,
}

impl CognitoSyncService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for CognitoSyncService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for CognitoSyncService {
    fn service_name(&self) -> &str {
        "cognito-sync"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://cognito-sync\..*\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

type SharedState = Arc<tokio::sync::RwLock<CognitoSyncState>>;

impl CognitoSyncService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let raw_segments: Vec<String> = path
            .trim_start_matches('/')
            .split('/')
            .filter(|s| !s.is_empty())
            .map(percent_decode)
            .collect();
        let segments: Vec<&str> = raw_segments.iter().map(|s| s.as_str()).collect();
        let method = request.method.as_str().to_string();
        let body: Value = if request.body.is_empty() {
            json!({})
        } else {
            serde_json::from_slice(&request.body).unwrap_or(json!({}))
        };

        let (response, mutating) = match (method.as_str(), segments.as_slice()) {
            ("GET", ["identitypools"]) => (self.handle_list_pool_usage(&state).await, false),
            ("GET", ["identitypools", pool_id]) => (
                self.handle_describe_pool_usage(&state, pool_id).await,
                false,
            ),
            ("POST", ["identitypools", pool_id, "bulkpublish"]) => {
                (self.handle_bulk_publish(&state, pool_id).await, true)
            }
            ("POST", ["identitypools", pool_id, "getBulkPublishDetails"]) => (
                self.handle_get_bulk_publish_details(&state, pool_id).await,
                false,
            ),
            ("GET", ["identitypools", pool_id, "events"]) => {
                (self.handle_get_cognito_events(&state, pool_id).await, false)
            }
            ("POST", ["identitypools", pool_id, "events"]) => (
                self.handle_set_cognito_events(&state, pool_id, &body).await,
                true,
            ),
            ("GET", ["identitypools", pool_id, "configuration"]) => (
                self.handle_get_pool_configuration(&state, pool_id).await,
                false,
            ),
            ("POST", ["identitypools", pool_id, "configuration"]) => (
                self.handle_set_pool_configuration(&state, pool_id, &body)
                    .await,
                true,
            ),
            ("GET", ["identitypools", pool_id, "identities", identity_id]) => (
                self.handle_describe_identity_usage(&state, pool_id, identity_id)
                    .await,
                false,
            ),
            ("POST", ["identitypools", pool_id, "identity", identity_id, "device"]) => (
                self.handle_register_device(&state, pool_id, identity_id, &body)
                    .await,
                true,
            ),
            (
                "GET",
                [
                    "identitypools",
                    pool_id,
                    "identities",
                    identity_id,
                    "datasets",
                ],
            ) => (
                self.handle_list_datasets(&state, pool_id, identity_id)
                    .await,
                false,
            ),
            (
                "GET",
                [
                    "identitypools",
                    pool_id,
                    "identities",
                    identity_id,
                    "datasets",
                    dataset_name,
                ],
            ) => (
                self.handle_describe_dataset(&state, pool_id, identity_id, dataset_name)
                    .await,
                false,
            ),
            (
                "DELETE",
                [
                    "identitypools",
                    pool_id,
                    "identities",
                    identity_id,
                    "datasets",
                    dataset_name,
                ],
            ) => (
                self.handle_delete_dataset(&state, pool_id, identity_id, dataset_name)
                    .await,
                true,
            ),
            (
                "POST",
                [
                    "identitypools",
                    pool_id,
                    "identities",
                    identity_id,
                    "datasets",
                    dataset_name,
                ],
            ) => (
                self.handle_update_records(&state, pool_id, identity_id, dataset_name, &body)
                    .await,
                true,
            ),
            (
                "GET",
                [
                    "identitypools",
                    pool_id,
                    "identities",
                    identity_id,
                    "datasets",
                    dataset_name,
                    "records",
                ],
            ) => (
                self.handle_list_records(&state, pool_id, identity_id, dataset_name)
                    .await,
                false,
            ),
            (
                "POST",
                [
                    "identitypools",
                    pool_id,
                    "identities",
                    identity_id,
                    "datasets",
                    dataset_name,
                    "subscriptions",
                    device_id,
                ],
            ) => (
                self.handle_subscribe(&state, pool_id, identity_id, dataset_name, device_id)
                    .await,
                true,
            ),
            (
                "DELETE",
                [
                    "identitypools",
                    pool_id,
                    "identities",
                    identity_id,
                    "datasets",
                    dataset_name,
                    "subscriptions",
                    device_id,
                ],
            ) => (
                self.handle_unsubscribe(&state, pool_id, identity_id, dataset_name, device_id)
                    .await,
                true,
            ),
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

    async fn handle_list_pool_usage(&self, state: &SharedState) -> MockResponse {
        let state = state.read().await;
        let mut usages: Vec<wire::IdentityPoolUsage> = state
            .pools
            .iter()
            .map(|(id, p)| wire::IdentityPoolUsage {
                identity_pool_id: Some(id.clone()),
                data_storage: Some(data_storage(p)),
                last_modified_date: Some(p.last_modified),
                sync_sessions_count: None,
            })
            .collect();
        usages.sort_by(|a, b| a.identity_pool_id.cmp(&b.identity_pool_id));
        let count = usages.len() as i32;
        wire::serialize_list_identity_pool_usage_response(&wire::ListIdentityPoolUsageResponse {
            count: Some(count),
            identity_pool_usages: Some(usages),
            max_results: None,
            next_token: None,
        })
    }

    async fn handle_describe_pool_usage(&self, state: &SharedState, pool_id: &str) -> MockResponse {
        let mut state = state.write().await;
        let pool = state.pool_mut(pool_id);
        wire::serialize_describe_identity_pool_usage_response(
            &wire::DescribeIdentityPoolUsageResponse {
                identity_pool_usage: Some(wire::IdentityPoolUsage {
                    identity_pool_id: Some(pool_id.to_string()),
                    data_storage: Some(data_storage(pool)),
                    last_modified_date: Some(pool.last_modified),
                    sync_sessions_count: None,
                }),
            },
        )
    }

    async fn handle_bulk_publish(&self, state: &SharedState, pool_id: &str) -> MockResponse {
        let now = now_secs();
        let mut state = state.write().await;
        let pool = state.pool_mut(pool_id);
        pool.bulk_publish = json!({
            "BulkPublishStatus": "SUCCEEDED",
            "BulkPublishStartTime": now,
            "BulkPublishCompleteTime": now,
        });
        wire::serialize_bulk_publish_response(&wire::BulkPublishResponse {
            identity_pool_id: Some(pool_id.to_string()),
        })
    }

    async fn handle_get_bulk_publish_details(
        &self,
        state: &SharedState,
        pool_id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        let pool = state.pool_mut(pool_id);
        let details = pool.bulk_publish.clone();
        let status = details
            .get("BulkPublishStatus")
            .and_then(|v| v.as_str())
            .unwrap_or("NOT_STARTED")
            .to_string();
        let start = details.get("BulkPublishStartTime").and_then(|v| v.as_f64());
        let complete = details
            .get("BulkPublishCompleteTime")
            .and_then(|v| v.as_f64());
        wire::serialize_get_bulk_publish_details_response(&wire::GetBulkPublishDetailsResponse {
            identity_pool_id: Some(pool_id.to_string()),
            bulk_publish_status: Some(status),
            bulk_publish_start_time: start,
            bulk_publish_complete_time: complete,
            failure_message: None,
        })
    }

    async fn handle_get_cognito_events(&self, state: &SharedState, pool_id: &str) -> MockResponse {
        let mut state = state.write().await;
        let pool = state.pool_mut(pool_id);
        wire::serialize_get_cognito_events_response(&wire::GetCognitoEventsResponse {
            events: Some(pool.events.clone()),
        })
    }

    async fn handle_set_cognito_events(
        &self,
        state: &SharedState,
        pool_id: &str,
        body: &Value,
    ) -> MockResponse {
        let mut state = state.write().await;
        let pool = state.pool_mut(pool_id);
        if let Some(events) = body.get("Events").and_then(|v| v.as_object()) {
            for (k, v) in events {
                if let Some(s) = v.as_str() {
                    if s.is_empty() {
                        pool.events.remove(k);
                    } else {
                        pool.events.insert(k.clone(), s.to_string());
                    }
                }
            }
        }
        wire::serialize_set_cognito_events_response()
    }

    async fn handle_get_pool_configuration(
        &self,
        state: &SharedState,
        pool_id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        let pool = state.pool_mut(pool_id);
        let config: wire::SetIdentityPoolConfigurationRequest =
            serde_json::from_value(pool.configuration.clone()).unwrap_or_default();
        wire::serialize_get_identity_pool_configuration_response(
            &wire::GetIdentityPoolConfigurationResponse {
                identity_pool_id: Some(pool_id.to_string()),
                push_sync: config.push_sync,
                cognito_streams: config.cognito_streams,
            },
        )
    }

    async fn handle_set_pool_configuration(
        &self,
        state: &SharedState,
        pool_id: &str,
        body: &Value,
    ) -> MockResponse {
        let config: wire::SetIdentityPoolConfigurationRequest =
            serde_json::from_value(body.clone()).unwrap_or_default();
        let mut state = state.write().await;
        state.pool_mut(pool_id).configuration = body.clone();
        wire::serialize_set_identity_pool_configuration_response(
            &wire::SetIdentityPoolConfigurationResponse {
                identity_pool_id: Some(pool_id.to_string()),
                push_sync: config.push_sync,
                cognito_streams: config.cognito_streams,
            },
        )
    }

    async fn handle_describe_identity_usage(
        &self,
        state: &SharedState,
        pool_id: &str,
        identity_id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        let identity = state.identity_mut(pool_id, identity_id);
        let dataset_count = identity.datasets.len() as i32;
        let data_storage: i64 = identity.datasets.values().map(|d| d.data_storage).sum();
        let last_modified: f64 = identity
            .datasets
            .values()
            .map(|d| d.last_modified_date)
            .fold(0.0_f64, f64::max);
        wire::serialize_describe_identity_usage_response(&wire::DescribeIdentityUsageResponse {
            identity_usage: Some(wire::IdentityUsage {
                data_storage: Some(data_storage),
                dataset_count: Some(dataset_count),
                identity_id: Some(identity_id.to_string()),
                identity_pool_id: Some(pool_id.to_string()),
                last_modified_date: Some(last_modified),
            }),
        })
    }

    async fn handle_register_device(
        &self,
        state: &SharedState,
        pool_id: &str,
        identity_id: &str,
        body: &Value,
    ) -> MockResponse {
        let platform = body
            .get("Platform")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string();
        let token = body
            .get("Token")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string();
        if platform.is_empty() || token.is_empty() {
            return rest_json_error(
                400,
                "InvalidParameterException",
                "Platform and Token are required",
            );
        }
        let device_id = uuid::Uuid::new_v4().to_string();
        let mut state = state.write().await;
        let identity = state.identity_mut(pool_id, identity_id);
        identity.devices.insert(
            device_id.clone(),
            DeviceRecord {
                device_id: device_id.clone(),
                platform,
                token,
            },
        );
        wire::serialize_register_device_response(&wire::RegisterDeviceResponse {
            device_id: Some(device_id),
        })
    }

    async fn handle_list_datasets(
        &self,
        state: &SharedState,
        pool_id: &str,
        identity_id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        let identity = state.identity_mut(pool_id, identity_id);
        let mut datasets: Vec<wire::Dataset> = identity
            .datasets
            .iter()
            .map(|(name, d)| dataset_to_wire(name, identity_id, d))
            .collect();
        datasets.sort_by(|a, b| a.dataset_name.cmp(&b.dataset_name));
        let count = datasets.len() as i32;
        wire::serialize_list_datasets_response(&wire::ListDatasetsResponse {
            count: Some(count),
            datasets: Some(datasets),
            next_token: None,
        })
    }

    async fn handle_describe_dataset(
        &self,
        state: &SharedState,
        pool_id: &str,
        identity_id: &str,
        dataset_name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        let dataset = state
            .pools
            .get(pool_id)
            .and_then(|p| p.identities.get(identity_id))
            .and_then(|i| i.datasets.get(dataset_name));
        match dataset {
            Some(d) => wire::serialize_describe_dataset_response(&wire::DescribeDatasetResponse {
                dataset: Some(dataset_to_wire(dataset_name, identity_id, d)),
            }),
            None => err_response(&CognitoSyncError::NotFound(format!(
                "dataset {dataset_name}"
            ))),
        }
    }

    async fn handle_delete_dataset(
        &self,
        state: &SharedState,
        pool_id: &str,
        identity_id: &str,
        dataset_name: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        let identity = state.identity_mut(pool_id, identity_id);
        match identity.datasets.remove(dataset_name) {
            Some(d) => wire::serialize_delete_dataset_response(&wire::DeleteDatasetResponse {
                dataset: Some(dataset_to_wire(dataset_name, identity_id, &d)),
            }),
            None => err_response(&CognitoSyncError::NotFound(format!(
                "dataset {dataset_name}"
            ))),
        }
    }

    async fn handle_list_records(
        &self,
        state: &SharedState,
        pool_id: &str,
        identity_id: &str,
        dataset_name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        let dataset_opt = state
            .pools
            .get(pool_id)
            .and_then(|p| p.identities.get(identity_id))
            .and_then(|i| i.datasets.get(dataset_name));
        let (exists, sync_count, records) = match dataset_opt {
            Some(d) => {
                let mut entries: Vec<wire::Record> =
                    d.records.values().map(record_entry_to_wire).collect();
                entries.sort_by(|a, b| a.key.cmp(&b.key));
                (true, d.sync_count, entries)
            }
            None => (false, 0, vec![]),
        };
        let count = records.len() as i32;
        wire::serialize_list_records_response(&wire::ListRecordsResponse {
            count: Some(count),
            dataset_deleted_after_requested_sync_count: Some(false),
            dataset_exists: Some(exists),
            dataset_sync_count: Some(sync_count),
            last_modified_by: None,
            merged_dataset_names: None,
            next_token: None,
            records: Some(records),
            sync_session_token: Some(format!("session-{}", uuid::Uuid::new_v4().simple())),
        })
    }

    async fn handle_update_records(
        &self,
        state: &SharedState,
        pool_id: &str,
        identity_id: &str,
        dataset_name: &str,
        body: &Value,
    ) -> MockResponse {
        let now = now_secs();
        let mut state = state.write().await;
        let identity = state.identity_mut(pool_id, identity_id);
        let dataset = identity
            .datasets
            .entry(dataset_name.to_string())
            .or_insert_with(|| DatasetRecord {
                creation_date: now,
                last_modified_date: now,
                ..Default::default()
            });
        let mut updated: Vec<wire::Record> = vec![];
        if let Some(patches) = body.get("RecordPatches").and_then(|v| v.as_array()) {
            for patch in patches {
                let key = patch
                    .get("Key")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string();
                if key.is_empty() {
                    continue;
                }
                let op = patch
                    .get("Op")
                    .and_then(|v| v.as_str())
                    .unwrap_or("replace");
                if op == "remove" {
                    dataset.records.remove(&key);
                    continue;
                }
                let value = patch
                    .get("Value")
                    .and_then(|v| v.as_str())
                    .map(String::from);
                let device_last_modified =
                    patch.get("DeviceLastModifiedDate").and_then(|v| v.as_f64());
                dataset.sync_count += 1;
                let entry = RecordEntry {
                    key: key.clone(),
                    value,
                    sync_count: dataset.sync_count,
                    last_modified_date: now,
                    last_modified_by: "mock".to_string(),
                    device_last_modified_date: device_last_modified,
                };
                updated.push(record_entry_to_wire(&entry));
                dataset.records.insert(key, entry);
            }
        }
        dataset.last_modified_date = now;
        dataset.data_storage = dataset
            .records
            .values()
            .map(|r| r.value.as_ref().map(|v| v.len() as i64).unwrap_or(0))
            .sum();
        wire::serialize_update_records_response(&wire::UpdateRecordsResponse {
            records: Some(updated),
        })
    }

    async fn handle_subscribe(
        &self,
        state: &SharedState,
        pool_id: &str,
        identity_id: &str,
        dataset_name: &str,
        device_id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        let identity = state.identity_mut(pool_id, identity_id);
        let dataset = identity
            .datasets
            .entry(dataset_name.to_string())
            .or_default();
        if !dataset.subscriptions.iter().any(|d| d == device_id) {
            dataset.subscriptions.push(device_id.to_string());
        }
        wire::serialize_subscribe_to_dataset_response(&wire::SubscribeToDatasetResponse {})
    }

    async fn handle_unsubscribe(
        &self,
        state: &SharedState,
        pool_id: &str,
        identity_id: &str,
        dataset_name: &str,
        device_id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        let identity = state.identity_mut(pool_id, identity_id);
        if let Some(dataset) = identity.datasets.get_mut(dataset_name) {
            dataset.subscriptions.retain(|d| d != device_id);
        }
        wire::serialize_unsubscribe_from_dataset_response(&wire::UnsubscribeFromDatasetResponse {})
    }
}

fn dataset_to_wire(name: &str, identity_id: &str, d: &DatasetRecord) -> wire::Dataset {
    wire::Dataset {
        dataset_name: Some(name.to_string()),
        identity_id: Some(identity_id.to_string()),
        creation_date: Some(d.creation_date),
        last_modified_date: Some(d.last_modified_date),
        last_modified_by: Some(d.last_modified_by.clone()),
        data_storage: Some(d.data_storage),
        num_records: Some(d.records.len() as i64),
    }
}

fn record_entry_to_wire(r: &RecordEntry) -> wire::Record {
    wire::Record {
        key: Some(r.key.clone()),
        value: r.value.clone(),
        sync_count: Some(r.sync_count),
        last_modified_date: Some(r.last_modified_date),
        last_modified_by: Some(r.last_modified_by.clone()),
        device_last_modified_date: r.device_last_modified_date,
    }
}

fn data_storage(pool: &crate::state::IdentityPoolRecord) -> i64 {
    pool.identities
        .values()
        .flat_map(|i| i.datasets.values())
        .map(|d| d.data_storage)
        .sum()
}

fn now_secs() -> f64 {
    chrono::Utc::now().timestamp() as f64
}

fn rest_json_error(status: u16, error_type: &str, message: &str) -> MockResponse {
    let body = json!({"__type": error_type, "message": message});
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}

fn err_response(err: &CognitoSyncError) -> MockResponse {
    let (status, error_type) = match err {
        CognitoSyncError::NotFound(_) => (404, "ResourceNotFoundException"),
        CognitoSyncError::Validation(_) => (400, "InvalidParameterException"),
    };
    rest_json_error(status, error_type, &err.to_string())
}
