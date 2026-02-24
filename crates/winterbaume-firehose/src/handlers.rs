use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
    StatefulService,
};

use crate::state::{FirehoseError, FirehoseState};
use crate::views::FirehoseStateView;
use crate::wire;

pub struct FirehoseService {
    pub(crate) state: Arc<BackendState<FirehoseState>>,
    pub(crate) notifier: StateChangeNotifier<FirehoseStateView>,
}

impl FirehoseService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for FirehoseService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for FirehoseService {
    fn service_name(&self) -> &str {
        "firehose"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://firehose\..*\.amazonaws\.com",
            r"https?://firehose\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl FirehoseService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;

        // Extract action from X-Amz-Target header
        // Format: "Firehose_20150804.CreateDeliveryStream"
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

        if serde_json::from_slice::<Value>(&request.body).is_err() {
            return json_error_response(400, "SerializationException", "Invalid JSON body");
        }
        let body_bytes: &[u8] = &request.body;

        let state = self.state.get(account_id, &region);

        let response = match action.as_str() {
            "CreateDeliveryStream" => {
                self.handle_create_delivery_stream(&state, body_bytes, account_id, &region)
                    .await
            }
            "DeleteDeliveryStream" => self.handle_delete_delivery_stream(&state, body_bytes).await,
            "DescribeDeliveryStream" => {
                self.handle_describe_delivery_stream(&state, body_bytes)
                    .await
            }
            "ListDeliveryStreams" => self.handle_list_delivery_streams(&state).await,
            "PutRecord" => self.handle_put_record(&state, body_bytes).await,
            "PutRecordBatch" => self.handle_put_record_batch(&state, body_bytes).await,
            "ListTagsForDeliveryStream" => {
                self.handle_list_tags_for_delivery_stream(&state, body_bytes)
                    .await
            }
            "TagDeliveryStream" => self.handle_tag_delivery_stream(&state, body_bytes).await,
            "UntagDeliveryStream" => self.handle_untag_delivery_stream(&state, body_bytes).await,
            "StartDeliveryStreamEncryption" => {
                self.handle_start_delivery_stream_encryption(&state, body_bytes)
                    .await
            }
            "StopDeliveryStreamEncryption" => {
                self.handle_stop_delivery_stream_encryption(&state, body_bytes)
                    .await
            }
            "UpdateDestination" => self.handle_update_destination(&state, body_bytes).await,
            _ => json_error_response(
                400,
                "InvalidAction",
                &format!("Could not find operation {action} for Firehose"),
            ),
        };

        if response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_create_delivery_stream(
        &self,
        state: &Arc<tokio::sync::RwLock<FirehoseState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_delivery_stream_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.delivery_stream_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'DeliveryStreamName'");
        }
        let name = input.delivery_stream_name.as_str();
        let destination_type = input.delivery_stream_type.as_deref();

        let mut state = state.write().await;
        match state.create_delivery_stream(name, destination_type, account_id, region) {
            Ok(stream) => {
                wire::serialize_create_delivery_stream_response(&wire::CreateDeliveryStreamOutput {
                    delivery_stream_a_r_n: Some(stream.arn.clone()),
                })
            }
            Err(e) => firehose_error_response(&e),
        }
    }

    async fn handle_delete_delivery_stream(
        &self,
        state: &Arc<tokio::sync::RwLock<FirehoseState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_delivery_stream_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.delivery_stream_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'DeliveryStreamName'");
        }
        let name = input.delivery_stream_name.as_str();

        let mut state = state.write().await;
        match state.delete_delivery_stream(name) {
            Ok(()) => wire::serialize_delete_delivery_stream_response(
                &wire::DeleteDeliveryStreamOutput {},
            ),
            Err(e) => firehose_error_response(&e),
        }
    }

    async fn handle_describe_delivery_stream(
        &self,
        state: &Arc<tokio::sync::RwLock<FirehoseState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_delivery_stream_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.delivery_stream_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'DeliveryStreamName'");
        }
        let name = input.delivery_stream_name.as_str();

        let state = state.read().await;
        match state.describe_delivery_stream(name) {
            Ok(stream) => wire::serialize_describe_delivery_stream_response(
                &wire::DescribeDeliveryStreamOutput {
                    delivery_stream_description: Some(wire::DeliveryStreamDescription {
                        delivery_stream_name: Some(stream.name.clone()),
                        delivery_stream_a_r_n: Some(stream.arn.clone()),
                        delivery_stream_status: Some(stream.status.clone()),
                        delivery_stream_type: Some(stream.destination_type.clone()),
                        create_timestamp: Some(stream.created_timestamp.timestamp() as f64),
                        has_more_destinations: Some(false),
                        destinations: Some(vec![wire::DestinationDescription {
                            destination_id: Some(stream.destination_id.clone()),
                            ..Default::default()
                        }]),
                        version_id: Some(stream.version_id.clone()),
                        ..Default::default()
                    }),
                },
            ),
            Err(e) => firehose_error_response(&e),
        }
    }

    async fn handle_list_delivery_streams(
        &self,
        state: &Arc<tokio::sync::RwLock<FirehoseState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let streams = state.list_delivery_streams();

        wire::serialize_list_delivery_streams_response(&wire::ListDeliveryStreamsOutput {
            delivery_stream_names: Some(streams.into_iter().map(|s| s.to_string()).collect()),
            has_more_delivery_streams: Some(false),
        })
    }

    async fn handle_put_record(
        &self,
        state: &Arc<tokio::sync::RwLock<FirehoseState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_put_record_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.delivery_stream_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'DeliveryStreamName'");
        }
        let name = input.delivery_stream_name.as_str();

        let mut state = state.write().await;
        match state.put_record(name) {
            Ok(record_id) => wire::serialize_put_record_response(&wire::PutRecordOutput {
                record_id: Some(record_id),
                encrypted: Some(false),
            }),
            Err(e) => firehose_error_response(&e),
        }
    }

    async fn handle_put_record_batch(
        &self,
        state: &Arc<tokio::sync::RwLock<FirehoseState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_put_record_batch_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.delivery_stream_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'DeliveryStreamName'");
        }
        let name = input.delivery_stream_name.as_str();
        let record_count = input.records.len();

        let mut state = state.write().await;
        match state.put_record_batch(name, record_count) {
            Ok(record_ids) => {
                let request_responses: Vec<wire::PutRecordBatchResponseEntry> = record_ids
                    .iter()
                    .map(|id| wire::PutRecordBatchResponseEntry {
                        record_id: Some(id.clone()),
                        ..Default::default()
                    })
                    .collect();

                wire::serialize_put_record_batch_response(&wire::PutRecordBatchOutput {
                    failed_put_count: Some(0),
                    request_responses: Some(request_responses),
                    ..Default::default()
                })
            }
            Err(e) => firehose_error_response(&e),
        }
    }

    async fn handle_list_tags_for_delivery_stream(
        &self,
        state: &Arc<tokio::sync::RwLock<FirehoseState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_delivery_stream_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.delivery_stream_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'DeliveryStreamName'");
        }
        let name = input.delivery_stream_name.as_str();

        let state = state.read().await;
        match state.list_tags_for_delivery_stream(name) {
            Ok(tags) => {
                let tag_list: Vec<wire::Tag> = tags
                    .iter()
                    .map(|(k, v)| wire::Tag {
                        key: k.clone(),
                        value: Some(v.clone()),
                    })
                    .collect();
                wire::serialize_list_tags_for_delivery_stream_response(
                    &wire::ListTagsForDeliveryStreamOutput {
                        tags: Some(tag_list),
                        has_more_tags: Some(false),
                    },
                )
            }
            Err(e) => firehose_error_response(&e),
        }
    }

    async fn handle_tag_delivery_stream(
        &self,
        state: &Arc<tokio::sync::RwLock<FirehoseState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_tag_delivery_stream_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.delivery_stream_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'DeliveryStreamName'");
        }
        let name = input.delivery_stream_name.as_str();

        let tags_value: Vec<Value> = input
            .tags
            .iter()
            .map(|t| serde_json::to_value(t).unwrap_or(Value::Null))
            .collect();

        let mut state = state.write().await;
        match state.tag_delivery_stream(name, &tags_value) {
            Ok(()) => {
                wire::serialize_tag_delivery_stream_response(&wire::TagDeliveryStreamOutput {})
            }
            Err(e) => firehose_error_response(&e),
        }
    }

    async fn handle_untag_delivery_stream(
        &self,
        state: &Arc<tokio::sync::RwLock<FirehoseState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_untag_delivery_stream_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.delivery_stream_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'DeliveryStreamName'");
        }
        let name = input.delivery_stream_name.as_str();
        let tag_keys: Vec<&str> = input.tag_keys.iter().map(|s| s.as_str()).collect();

        let mut state = state.write().await;
        match state.untag_delivery_stream(name, &tag_keys) {
            Ok(()) => {
                wire::serialize_untag_delivery_stream_response(&wire::UntagDeliveryStreamOutput {})
            }
            Err(e) => firehose_error_response(&e),
        }
    }

    async fn handle_start_delivery_stream_encryption(
        &self,
        state: &Arc<tokio::sync::RwLock<FirehoseState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_start_delivery_stream_encryption_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.delivery_stream_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'DeliveryStreamName'");
        }
        let name = input.delivery_stream_name.as_str();

        let mut state = state.write().await;
        match state.start_delivery_stream_encryption(name) {
            Ok(()) => wire::serialize_start_delivery_stream_encryption_response(
                &wire::StartDeliveryStreamEncryptionOutput {},
            ),
            Err(e) => firehose_error_response(&e),
        }
    }

    async fn handle_stop_delivery_stream_encryption(
        &self,
        state: &Arc<tokio::sync::RwLock<FirehoseState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_stop_delivery_stream_encryption_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.delivery_stream_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'DeliveryStreamName'");
        }
        let name = input.delivery_stream_name.as_str();

        let mut state = state.write().await;
        match state.stop_delivery_stream_encryption(name) {
            Ok(()) => wire::serialize_stop_delivery_stream_encryption_response(
                &wire::StopDeliveryStreamEncryptionOutput {},
            ),
            Err(e) => firehose_error_response(&e),
        }
    }

    async fn handle_update_destination(
        &self,
        state: &Arc<tokio::sync::RwLock<FirehoseState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_destination_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.delivery_stream_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'DeliveryStreamName'");
        }
        if input.current_delivery_stream_version_id.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing 'CurrentDeliveryStreamVersionId'",
            );
        }
        let name = input.delivery_stream_name.as_str();
        let current_version_id = input.current_delivery_stream_version_id.as_str();

        let mut state = state.write().await;
        match state.update_destination(name, current_version_id) {
            Ok(()) => {
                wire::serialize_update_destination_response(&wire::UpdateDestinationOutput {})
            }
            Err(e) => firehose_error_response(&e),
        }
    }
}

fn firehose_error_response(err: &FirehoseError) -> MockResponse {
    let (status, code) = match err {
        FirehoseError::ResourceInUse { .. } => (400, "ResourceInUseException"),
        FirehoseError::ResourceNotFound { .. } => (400, "ResourceNotFoundException"),
        FirehoseError::ConcurrentModification => (400, "ConcurrentModificationException"),
    };
    let body = json!({
        "__type": code,
        "message": err.to_string(),
    });
    MockResponse::json(status, body.to_string())
}

fn json_error_response(status: u16, code: &str, message: &str) -> MockResponse {
    let body = json!({
        "__type": code,
        "message": message,
    });
    MockResponse::json(status, body.to_string())
}
