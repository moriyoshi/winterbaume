use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, default_account_id,
};

use crate::state::{KinesisError, KinesisState};
use crate::views::KinesisStateView;
use crate::wire;

pub struct KinesisService {
    pub(crate) state: Arc<BackendState<KinesisState>>,
    pub(crate) notifier: StateChangeNotifier<KinesisStateView>,
}

impl KinesisService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for KinesisService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for KinesisService {
    fn service_name(&self) -> &str {
        "kinesis"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://kinesis\..*\.amazonaws\.com",
            r"https?://kinesis\.amazonaws\.com",
            r"https?://.*\.control-kinesis\..*\.amazonaws\.com",
            r"https?://control-kinesis\..*\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl KinesisService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = extract_kinesis_region(&request.uri);
        let account_id = default_account_id();

        // Extract action from X-Amz-Target header
        // Format: "Kinesis_20131202.CreateStream"
        let action = request
            .headers
            .get("x-amz-target")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.rsplit('.').next())
            .map(|s| s.to_string());

        let action = match action {
            Some(a) => a,
            None => {
                return json_error_response(400, "MissingAction", "Missing X-Amz-Target header");
            }
        };

        // Validate the body is well-formed JSON up-front; the typed deserialisers in
        // `wire` re-parse the bytes per operation.
        if serde_json::from_slice::<Value>(&request.body).is_err() {
            return json_error_response(400, "SerializationException", "Invalid JSON body");
        }
        let body_bytes: &[u8] = &request.body;

        let state = self.state.get(account_id, &region);

        let response = match action.as_str() {
            "CreateStream" => {
                self.handle_create_stream(&state, body_bytes, account_id, &region)
                    .await
            }
            "DeleteStream" => {
                self.handle_delete_stream(&state, body_bytes, account_id)
                    .await
            }
            "DescribeStream" => {
                self.handle_describe_stream(&state, body_bytes, account_id)
                    .await
            }
            "DescribeStreamSummary" => {
                self.handle_describe_stream_summary(&state, body_bytes, account_id)
                    .await
            }
            "DescribeLimits" => self.handle_describe_limits(&state).await,
            "ListStreams" => self.handle_list_streams(&state).await,
            "ListShards" => {
                self.handle_list_shards(&state, body_bytes, account_id)
                    .await
            }
            "PutRecord" => self.handle_put_record(&state, body_bytes, account_id).await,
            "PutRecords" => {
                self.handle_put_records(&state, body_bytes, account_id)
                    .await
            }
            "GetRecords" => {
                self.handle_get_records(&state, body_bytes, account_id)
                    .await
            }
            "GetShardIterator" => {
                self.handle_get_shard_iterator(&state, body_bytes, account_id)
                    .await
            }
            "AddTagsToStream" => {
                self.handle_add_tags_to_stream(&state, body_bytes, account_id)
                    .await
            }
            "RemoveTagsFromStream" => {
                self.handle_remove_tags_from_stream(&state, body_bytes, account_id)
                    .await
            }
            "ListTagsForStream" => {
                self.handle_list_tags_for_stream(&state, body_bytes, account_id)
                    .await
            }
            "RegisterStreamConsumer" => {
                self.handle_register_stream_consumer(&state, body_bytes)
                    .await
            }
            "DeregisterStreamConsumer" => {
                self.handle_deregister_stream_consumer(&state, body_bytes, account_id)
                    .await
            }
            "DescribeStreamConsumer" => {
                self.handle_describe_stream_consumer(&state, body_bytes, account_id)
                    .await
            }
            "ListStreamConsumers" => self.handle_list_stream_consumers(&state, body_bytes).await,
            "IncreaseStreamRetentionPeriod" => {
                self.handle_increase_stream_retention_period(&state, body_bytes, account_id)
                    .await
            }
            "DecreaseStreamRetentionPeriod" => {
                self.handle_decrease_stream_retention_period(&state, body_bytes, account_id)
                    .await
            }
            "StartStreamEncryption" => {
                self.handle_start_stream_encryption(&state, body_bytes, account_id)
                    .await
            }
            "StopStreamEncryption" => {
                self.handle_stop_stream_encryption(&state, body_bytes, account_id)
                    .await
            }
            "EnableEnhancedMonitoring" => {
                self.handle_enable_enhanced_monitoring(&state, body_bytes, account_id)
                    .await
            }
            "DisableEnhancedMonitoring" => {
                self.handle_disable_enhanced_monitoring(&state, body_bytes, account_id)
                    .await
            }
            "PutResourcePolicy" => {
                self.handle_put_resource_policy(&state, body_bytes, account_id)
                    .await
            }
            "GetResourcePolicy" => {
                self.handle_get_resource_policy(&state, body_bytes, account_id)
                    .await
            }
            "DeleteResourcePolicy" => {
                self.handle_delete_resource_policy(&state, body_bytes, account_id)
                    .await
            }
            "UpdateStreamMode" => self.handle_update_stream_mode(&state, body_bytes).await,
            "UpdateShardCount" => {
                self.handle_update_shard_count(&state, body_bytes, account_id)
                    .await
            }
            "MergeShards" => {
                self.handle_merge_shards(&state, body_bytes, account_id)
                    .await
            }
            "SplitShard" => {
                self.handle_split_shard(&state, body_bytes, account_id)
                    .await
            }
            "DescribeAccountSettings" => self.handle_describe_account_settings(&state).await,
            "UpdateAccountSettings" => {
                self.handle_update_account_settings(&state, body_bytes)
                    .await
            }
            "TagResource" => self.handle_tag_resource(&state, body_bytes).await,
            "UntagResource" => self.handle_untag_resource(&state, body_bytes).await,
            "ListTagsForResource" => self.handle_list_tags_for_resource(&state, body_bytes).await,
            "UpdateMaxRecordSize" => {
                self.handle_update_max_record_size(&state, body_bytes, account_id)
                    .await
            }
            "UpdateStreamWarmThroughput" => {
                self.handle_update_stream_warm_throughput(&state, body_bytes, account_id)
                    .await
            }
            "SubscribeToShard" => json_error_response(
                400,
                "InvalidOperation",
                "SubscribeToShard requires HTTP/2 and is not supported by this mock",
            ),
            _ => json_error_response(
                400,
                "InvalidAction",
                &format!("Could not find operation {action} for Kinesis"),
            ),
        };
        use winterbaume_core::StatefulService;
        if response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_create_stream(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_stream_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.stream_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'StreamName'");
        }
        let shard_count = input.shard_count;
        let stream_mode = input
            .stream_mode_details
            .as_ref()
            .map(|d| d.stream_mode.as_str());

        let mut state = state.write().await;
        match state.create_stream(
            &input.stream_name,
            shard_count,
            stream_mode,
            account_id,
            region,
        ) {
            Ok(_) => wire::serialize_create_stream_response(),
            Err(e) => kinesis_error_response(&e),
        }
    }

    async fn handle_delete_stream(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisState>>,
        body: &[u8],
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_stream_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let stream_name = input.stream_name.as_deref();
        let stream_arn = input.stream_a_r_n.as_deref();

        let mut state = state.write().await;
        let result = if let Some(name) = stream_name {
            state.delete_stream(name, account_id)
        } else if let Some(arn) = stream_arn {
            state.delete_stream_by_arn(arn, account_id)
        } else {
            return json_error_response(
                400,
                "ValidationException",
                "Missing 'StreamName' or 'StreamARN'",
            );
        };

        match result {
            Ok(()) => wire::serialize_delete_stream_response(),
            Err(e) => kinesis_error_response(&e),
        }
    }

    async fn handle_describe_stream(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisState>>,
        body: &[u8],
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_stream_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let stream_name = input.stream_name.as_deref();
        let stream_arn = input.stream_a_r_n.as_deref();
        let limit = input.limit.map(|v| v as usize);

        let state = state.read().await;
        let stream = if let Some(name) = stream_name {
            state.describe_stream(name, account_id)
        } else if let Some(arn) = stream_arn {
            state.describe_stream_by_arn(arn, account_id)
        } else {
            return json_error_response(
                400,
                "ValidationException",
                "Missing 'StreamName' or 'StreamARN'",
            );
        };

        match stream {
            Ok(stream) => {
                let all_shards: Vec<wire::Shard> = stream
                    .shards
                    .iter()
                    .map(|s| wire::Shard {
                        shard_id: Some(s.shard_id.clone()),
                        hash_key_range: Some(wire::HashKeyRange {
                            starting_hash_key: Some(s.starting_hash_key.clone()),
                            ending_hash_key: Some(s.ending_hash_key.clone()),
                        }),
                        sequence_number_range: Some(wire::SequenceNumberRange {
                            starting_sequence_number: Some("0".to_string()),
                            ending_sequence_number: if s.closed {
                                Some("0".to_string())
                            } else {
                                None
                            },
                        }),
                        parent_shard_id: s.parent_shard_id.clone(),
                        adjacent_parent_shard_id: s.adjacent_parent_shard_id.clone(),
                    })
                    .collect();

                let (shards, has_more) = if let Some(limit) = limit {
                    let truncated = all_shards.len() > limit;
                    (all_shards.into_iter().take(limit).collect(), truncated)
                } else {
                    (all_shards, false)
                };

                wire::serialize_describe_stream_response(&wire::DescribeStreamOutput {
                    stream_description: Some(wire::StreamDescription {
                        stream_name: Some(stream.name.clone()),
                        stream_a_r_n: Some(stream.arn.clone()),
                        stream_status: Some(stream.status.clone()),
                        shards: Some(shards),
                        has_more_shards: Some(has_more),
                        retention_period_hours: Some(stream.retention_period_hours as i32),
                        stream_creation_timestamp: Some(stream.created_timestamp.timestamp() as f64),
                        enhanced_monitoring: Some(vec![wire::EnhancedMetrics {
                            shard_level_metrics: Some(stream.enhanced_monitoring.clone()),
                        }]),
                        // FIX(terraform-e2e): always return encryption_type explicitly
                        // (even "NONE") so the terraform provider doesn't see a diff
                        // between state and refresh and try to call StopStreamEncryption.
                        encryption_type: Some(stream.encryption_type.clone()),
                        key_id: stream.key_id.clone(),
                        stream_mode_details: Some(wire::StreamModeDetails {
                            stream_mode: stream.stream_mode.clone(),
                        }),
                        ..Default::default()
                    }),
                })
            }
            Err(e) => kinesis_error_response(&e),
        }
    }

    async fn handle_list_streams(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let streams = state.list_streams();

        let mut stream_names: Vec<String> = streams.iter().map(|s| s.name.clone()).collect();
        stream_names.sort();

        let stream_summaries: Vec<wire::StreamSummary> = streams
            .iter()
            .map(|s| wire::StreamSummary {
                stream_name: Some(s.name.clone()),
                stream_a_r_n: Some(s.arn.clone()),
                stream_status: Some(s.status.clone()),
                stream_creation_timestamp: Some(s.created_timestamp.timestamp() as f64),
                stream_mode_details: Some(wire::StreamModeDetails {
                    stream_mode: s.stream_mode.clone(),
                }),
            })
            .collect();

        let has_more = stream_names.len() > 10;
        let truncated_names: Vec<String> = stream_names.iter().take(10).cloned().collect();

        wire::serialize_list_streams_response(&wire::ListStreamsOutput {
            has_more_streams: Some(has_more),
            stream_names: Some(truncated_names),
            next_token: None,
            stream_summaries: Some(stream_summaries),
        })
    }

    async fn handle_put_record(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisState>>,
        body: &[u8],
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_put_record_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let stream_name = input.stream_name.as_deref();
        let stream_arn = input.stream_a_r_n.as_deref();
        if input.partition_key.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'PartitionKey'");
        }
        let partition_key = input.partition_key.as_str();

        let mut state = state.write().await;
        let result = if let Some(name) = stream_name {
            state.put_record(name, partition_key, account_id)
        } else if let Some(arn) = stream_arn {
            state.put_record_by_arn(arn, partition_key, account_id)
        } else {
            return json_error_response(
                400,
                "ValidationException",
                "Missing 'StreamName' or 'StreamARN'",
            );
        };

        match result {
            Ok((sequence_number, shard_id)) => {
                wire::serialize_put_record_response(&wire::PutRecordOutput {
                    sequence_number: Some(sequence_number),
                    shard_id: Some(shard_id),
                    encryption_type: None,
                })
            }
            Err(e) => kinesis_error_response(&e),
        }
    }

    async fn handle_put_records(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisState>>,
        body: &[u8],
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_put_records_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let stream_name = input.stream_name.as_deref();
        let stream_arn = input.stream_a_r_n.as_deref();

        if input.records.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Records'");
        }
        let records: Vec<String> = input
            .records
            .iter()
            .filter(|r| !r.partition_key.is_empty())
            .map(|r| r.partition_key.clone())
            .collect();

        let mut state = state.write().await;

        // Resolve the stream name from ARN if needed
        let resolved_name = if let Some(n) = stream_name {
            n.to_string()
        } else if let Some(arn) = stream_arn {
            match state.describe_stream_by_arn(arn, account_id) {
                Ok(s) => s.name.clone(),
                Err(e) => return kinesis_error_response(&e),
            }
        } else {
            return json_error_response(
                400,
                "ValidationException",
                "Missing 'StreamName' or 'StreamARN'",
            );
        };

        match state.put_records(&resolved_name, records, account_id) {
            Ok(results) => {
                let entries: Vec<wire::PutRecordsResultEntry> = results
                    .iter()
                    .map(|(seq, shard)| wire::PutRecordsResultEntry {
                        sequence_number: Some(seq.clone()),
                        shard_id: Some(shard.clone()),
                        ..Default::default()
                    })
                    .collect();

                wire::serialize_put_records_response(&wire::PutRecordsOutput {
                    failed_record_count: Some(0),
                    records: Some(entries),
                    encryption_type: None,
                })
            }
            Err(e) => kinesis_error_response(&e),
        }
    }

    async fn handle_get_records(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisState>>,
        body: &[u8],
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_get_records_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.shard_iterator.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ShardIterator'");
        }
        let shard_iterator = input.shard_iterator.as_str();

        let limit = input.limit.map(|v| v as usize);

        // Parse shard iterator: "iter:{stream_name}:{shard_id}" or legacy "stream:{name}"
        let (stream_name, shard_id) = if let Some(rest) = shard_iterator.strip_prefix("iter:") {
            match rest.split_once(':') {
                Some((name, shard)) => (name, Some(shard)),
                None => (rest, None),
            }
        } else if let Some(name) = shard_iterator.strip_prefix("stream:") {
            (name, None)
        } else {
            (shard_iterator, None)
        };

        let state = state.read().await;
        match state.get_records(stream_name, shard_id, limit, account_id) {
            Ok(records) => {
                let entries: Vec<wire::Record> = records
                    .iter()
                    .map(|r| wire::Record {
                        sequence_number: Some(r.sequence_number.clone()),
                        data: Some(String::new()),
                        partition_key: Some(r.partition_key.clone()),
                        approximate_arrival_timestamp: Some(r.timestamp.timestamp() as f64),
                        ..Default::default()
                    })
                    .collect();

                wire::serialize_get_records_response(&wire::GetRecordsOutput {
                    records: Some(entries),
                    millis_behind_latest: Some(0),
                    next_shard_iterator: Some(shard_iterator.to_string()),
                    child_shards: None,
                })
            }
            Err(e) => kinesis_error_response(&e),
        }
    }

    async fn handle_get_shard_iterator(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisState>>,
        body: &[u8],
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_get_shard_iterator_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let stream_name = input.stream_name.as_deref();
        let stream_arn = input.stream_a_r_n.as_deref();
        if input.shard_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ShardId'");
        }
        let shard_id = input.shard_id.as_str();

        let state = state.read().await;

        // Resolve stream name
        let resolved_name = if let Some(name) = stream_name {
            name.to_string()
        } else if let Some(arn) = stream_arn {
            match state.describe_stream_by_arn(arn, account_id) {
                Ok(s) => s.name.clone(),
                Err(e) => return kinesis_error_response(&e),
            }
        } else {
            return json_error_response(
                400,
                "ValidationException",
                "Missing 'StreamName' or 'StreamARN'",
            );
        };

        match state.get_shard_iterator(&resolved_name, shard_id, account_id) {
            Ok(iterator) => {
                wire::serialize_get_shard_iterator_response(&wire::GetShardIteratorOutput {
                    shard_iterator: Some(iterator),
                })
            }
            Err(e) => kinesis_error_response(&e),
        }
    }

    async fn handle_add_tags_to_stream(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisState>>,
        body: &[u8],
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_add_tags_to_stream_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let stream_name = input.stream_name.as_deref();
        let stream_arn = input.stream_a_r_n.as_deref();
        if input.tags.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Tags'");
        }
        let tags = input.tags;

        let mut state = state.write().await;
        match state.add_tags_to_stream(stream_name, stream_arn, tags, account_id) {
            Ok(()) => wire::serialize_add_tags_to_stream_response(),
            Err(e) => kinesis_error_response(&e),
        }
    }

    async fn handle_remove_tags_from_stream(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisState>>,
        body: &[u8],
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_remove_tags_from_stream_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let stream_name = input.stream_name.as_deref();
        let stream_arn = input.stream_a_r_n.as_deref();
        if input.tag_keys.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'TagKeys'");
        }

        let mut state = state.write().await;
        match state.remove_tags_from_stream(stream_name, stream_arn, &input.tag_keys, account_id) {
            Ok(()) => wire::serialize_remove_tags_from_stream_response(),
            Err(e) => kinesis_error_response(&e),
        }
    }

    async fn handle_list_tags_for_stream(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisState>>,
        body: &[u8],
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_stream_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let stream_name = input.stream_name.as_deref();
        let stream_arn = input.stream_a_r_n.as_deref();

        let state = state.read().await;
        match state.list_tags_for_stream(stream_name, stream_arn, account_id) {
            Ok(tags) => {
                let tag_list: Vec<wire::Tag> = tags
                    .iter()
                    .map(|(k, v)| wire::Tag {
                        key: Some(k.clone()),
                        value: Some(v.clone()),
                    })
                    .collect();
                wire::serialize_list_tags_for_stream_response(&wire::ListTagsForStreamOutput {
                    has_more_tags: Some(false),
                    tags: Some(tag_list),
                })
            }
            Err(e) => kinesis_error_response(&e),
        }
    }

    async fn handle_register_stream_consumer(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_register_stream_consumer_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.stream_a_r_n.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'StreamARN'");
        }
        if input.consumer_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ConsumerName'");
        }

        let mut state = state.write().await;
        match state.register_stream_consumer(&input.stream_a_r_n, &input.consumer_name) {
            Ok(consumer) => wire::serialize_register_stream_consumer_response(
                &wire::RegisterStreamConsumerOutput {
                    consumer: Some(wire::Consumer {
                        consumer_name: Some(consumer.consumer_name.clone()),
                        consumer_a_r_n: Some(consumer.consumer_arn.clone()),
                        consumer_status: Some(consumer.consumer_status.clone()),
                        consumer_creation_timestamp: Some(
                            consumer.creation_timestamp.timestamp() as f64
                        ),
                    }),
                },
            ),
            Err(e) => kinesis_error_response(&e),
        }
    }

    async fn handle_deregister_stream_consumer(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisState>>,
        body: &[u8],
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_deregister_stream_consumer_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let stream_arn = input.stream_a_r_n.as_deref().unwrap_or("");
        let consumer_name = input.consumer_name.as_deref();
        let consumer_arn = input.consumer_a_r_n.as_deref();

        let mut state = state.write().await;
        match state.deregister_stream_consumer(stream_arn, consumer_name, consumer_arn, account_id)
        {
            Ok(()) => wire::serialize_deregister_stream_consumer_response(),
            Err(e) => kinesis_error_response(&e),
        }
    }

    async fn handle_describe_stream_consumer(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisState>>,
        body: &[u8],
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_stream_consumer_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let stream_arn = input.stream_a_r_n.as_deref().unwrap_or("");
        let consumer_name = input.consumer_name.as_deref();
        let consumer_arn = input.consumer_a_r_n.as_deref();

        let state = state.read().await;
        match state.describe_stream_consumer(stream_arn, consumer_name, consumer_arn, account_id) {
            Ok((consumer, actual_stream_arn)) => wire::serialize_describe_stream_consumer_response(
                &wire::DescribeStreamConsumerOutput {
                    consumer_description: Some(wire::ConsumerDescription {
                        consumer_name: Some(consumer.consumer_name.clone()),
                        consumer_a_r_n: Some(consumer.consumer_arn.clone()),
                        consumer_status: Some(consumer.consumer_status.clone()),
                        consumer_creation_timestamp: Some(
                            consumer.creation_timestamp.timestamp() as f64
                        ),
                        stream_a_r_n: Some(actual_stream_arn.to_string()),
                    }),
                },
            ),
            Err(e) => kinesis_error_response(&e),
        }
    }

    async fn handle_list_stream_consumers(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_stream_consumers_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.stream_a_r_n.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'StreamARN'");
        }

        let state = state.read().await;
        match state.list_stream_consumers(&input.stream_a_r_n) {
            Ok(consumers) => {
                let consumer_list: Vec<wire::Consumer> = consumers
                    .iter()
                    .map(|c| wire::Consumer {
                        consumer_name: Some(c.consumer_name.clone()),
                        consumer_a_r_n: Some(c.consumer_arn.clone()),
                        consumer_status: Some(c.consumer_status.clone()),
                        consumer_creation_timestamp: Some(c.creation_timestamp.timestamp() as f64),
                    })
                    .collect();
                wire::serialize_list_stream_consumers_response(&wire::ListStreamConsumersOutput {
                    consumers: Some(consumer_list),
                    next_token: None,
                })
            }
            Err(e) => kinesis_error_response(&e),
        }
    }

    async fn handle_increase_stream_retention_period(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisState>>,
        body: &[u8],
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_increase_stream_retention_period_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let stream_name = input.stream_name.as_deref();
        let stream_arn = input.stream_a_r_n.as_deref();
        let hours = input.retention_period_hours as u32;

        let mut state = state.write().await;
        match state.increase_retention_period(stream_name, stream_arn, hours, account_id) {
            Ok(()) => wire::serialize_increase_stream_retention_period_response(),
            Err(e) => kinesis_error_response(&e),
        }
    }

    async fn handle_decrease_stream_retention_period(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisState>>,
        body: &[u8],
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_decrease_stream_retention_period_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let stream_name = input.stream_name.as_deref();
        let stream_arn = input.stream_a_r_n.as_deref();
        let hours = input.retention_period_hours as u32;

        let mut state = state.write().await;
        match state.decrease_retention_period(stream_name, stream_arn, hours, account_id) {
            Ok(()) => wire::serialize_decrease_stream_retention_period_response(),
            Err(e) => kinesis_error_response(&e),
        }
    }

    async fn handle_start_stream_encryption(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisState>>,
        body: &[u8],
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_start_stream_encryption_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let stream_name = input.stream_name.as_deref();
        let stream_arn = input.stream_a_r_n.as_deref();
        if input.encryption_type.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'EncryptionType'");
        }
        if input.key_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'KeyId'");
        }

        let mut state = state.write().await;
        match state.start_stream_encryption(
            stream_name,
            stream_arn,
            &input.encryption_type,
            &input.key_id,
            account_id,
        ) {
            Ok(()) => wire::serialize_start_stream_encryption_response(),
            Err(e) => kinesis_error_response(&e),
        }
    }

    async fn handle_stop_stream_encryption(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisState>>,
        body: &[u8],
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_stop_stream_encryption_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let stream_name = input.stream_name.as_deref();
        let stream_arn = input.stream_a_r_n.as_deref();

        let mut state = state.write().await;
        match state.stop_stream_encryption(stream_name, stream_arn, account_id) {
            Ok(()) => wire::serialize_stop_stream_encryption_response(),
            Err(e) => kinesis_error_response(&e),
        }
    }

    async fn handle_enable_enhanced_monitoring(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisState>>,
        body: &[u8],
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_enable_enhanced_monitoring_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let stream_name = input.stream_name.as_deref();
        let stream_arn = input.stream_a_r_n.as_deref();
        let metrics = input.shard_level_metrics;

        let mut state = state.write().await;
        match state.enable_enhanced_monitoring(stream_name, stream_arn, metrics, account_id) {
            Ok((current_before, desired_after, sname, sarn)) => {
                wire::serialize_enable_enhanced_monitoring_response(
                    &wire::EnhancedMonitoringOutput {
                        current_shard_level_metrics: Some(current_before),
                        desired_shard_level_metrics: Some(desired_after),
                        stream_name: Some(sname),
                        stream_a_r_n: Some(sarn),
                    },
                )
            }
            Err(e) => kinesis_error_response(&e),
        }
    }

    async fn handle_disable_enhanced_monitoring(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisState>>,
        body: &[u8],
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_disable_enhanced_monitoring_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let stream_name = input.stream_name.as_deref();
        let stream_arn = input.stream_a_r_n.as_deref();
        let metrics = input.shard_level_metrics;

        let mut state = state.write().await;
        match state.disable_enhanced_monitoring(stream_name, stream_arn, metrics, account_id) {
            Ok((current_before, desired_after, sname, sarn)) => {
                wire::serialize_disable_enhanced_monitoring_response(
                    &wire::EnhancedMonitoringOutput {
                        current_shard_level_metrics: Some(current_before),
                        desired_shard_level_metrics: Some(desired_after),
                        stream_name: Some(sname),
                        stream_a_r_n: Some(sarn),
                    },
                )
            }
            Err(e) => kinesis_error_response(&e),
        }
    }

    async fn handle_put_resource_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisState>>,
        body: &[u8],
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_put_resource_policy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_a_r_n.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ResourceARN'");
        }
        if input.policy.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Policy'");
        }

        let mut state = state.write().await;
        match state.put_resource_policy(&input.resource_a_r_n, &input.policy, account_id) {
            Ok(()) => wire::serialize_put_resource_policy_response(),
            Err(e) => kinesis_error_response(&e),
        }
    }

    async fn handle_get_resource_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisState>>,
        body: &[u8],
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_get_resource_policy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_a_r_n.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ResourceARN'");
        }

        let state = state.read().await;
        match state.get_resource_policy(&input.resource_a_r_n, account_id) {
            Ok(policy) => {
                wire::serialize_get_resource_policy_response(&wire::GetResourcePolicyOutput {
                    policy: Some(policy.unwrap_or("{}").to_string()),
                })
            }
            Err(e) => kinesis_error_response(&e),
        }
    }

    async fn handle_delete_resource_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisState>>,
        body: &[u8],
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_resource_policy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_a_r_n.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ResourceARN'");
        }

        let mut state = state.write().await;
        match state.delete_resource_policy(&input.resource_a_r_n, account_id) {
            Ok(()) => wire::serialize_delete_resource_policy_response(),
            Err(e) => kinesis_error_response(&e),
        }
    }

    async fn handle_update_stream_mode(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_stream_mode_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.stream_a_r_n.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'StreamARN'");
        }
        let stream_mode = if input.stream_mode_details.stream_mode.is_empty() {
            "PROVISIONED"
        } else {
            input.stream_mode_details.stream_mode.as_str()
        };

        let mut state = state.write().await;
        match state.update_stream_mode(&input.stream_a_r_n, stream_mode) {
            Ok(()) => wire::serialize_update_stream_mode_response(),
            Err(e) => kinesis_error_response(&e),
        }
    }

    async fn handle_update_shard_count(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisState>>,
        body: &[u8],
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_update_shard_count_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let stream_name = input.stream_name.as_deref();
        let stream_arn = input.stream_a_r_n.as_deref();
        let target_shard_count = input.target_shard_count;

        let mut state = state.write().await;
        match state.update_shard_count(stream_name, stream_arn, target_shard_count, account_id) {
            Ok((current, target, sname)) => {
                wire::serialize_update_shard_count_response(&wire::UpdateShardCountOutput {
                    current_shard_count: Some(current),
                    target_shard_count: Some(target),
                    stream_name: Some(sname),
                    ..Default::default()
                })
            }
            Err(e) => kinesis_error_response(&e),
        }
    }

    async fn handle_merge_shards(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisState>>,
        body: &[u8],
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_merge_shards_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let stream_name = input.stream_name.as_deref();
        let stream_arn = input.stream_a_r_n.as_deref();
        if input.shard_to_merge.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ShardToMerge'");
        }
        if input.adjacent_shard_to_merge.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing 'AdjacentShardToMerge'",
            );
        }

        let mut state = state.write().await;
        match state.merge_shards(
            stream_name,
            stream_arn,
            &input.shard_to_merge,
            &input.adjacent_shard_to_merge,
            account_id,
        ) {
            Ok(()) => wire::serialize_merge_shards_response(),
            Err(e) => kinesis_error_response(&e),
        }
    }

    async fn handle_split_shard(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisState>>,
        body: &[u8],
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_split_shard_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let stream_name = input.stream_name.as_deref();
        let stream_arn = input.stream_a_r_n.as_deref();
        if input.shard_to_split.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ShardToSplit'");
        }
        if input.new_starting_hash_key.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'NewStartingHashKey'");
        }

        let mut state = state.write().await;
        match state.split_shard(
            stream_name,
            stream_arn,
            &input.shard_to_split,
            &input.new_starting_hash_key,
            account_id,
        ) {
            Ok(()) => wire::serialize_split_shard_response(),
            Err(e) => kinesis_error_response(&e),
        }
    }

    async fn handle_describe_stream_summary(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisState>>,
        body: &[u8],
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_stream_summary_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let stream_name = input.stream_name.as_deref();
        let stream_arn = input.stream_a_r_n.as_deref();

        let state = state.read().await;
        let result = if let Some(name) = stream_name {
            state.describe_stream(name, account_id)
        } else if let Some(arn) = stream_arn {
            state.describe_stream_by_arn(arn, account_id)
        } else {
            return json_error_response(
                400,
                "ValidationException",
                "Missing 'StreamName' or 'StreamARN'",
            );
        };

        match result {
            Ok(stream) => {
                let open_count = state.open_shard_count(stream);
                wire::serialize_describe_stream_summary_response(
                    &wire::DescribeStreamSummaryOutput {
                        stream_description_summary: Some(wire::StreamDescriptionSummary {
                            stream_name: Some(stream.name.clone()),
                            stream_a_r_n: Some(stream.arn.clone()),
                            stream_status: Some(stream.status.clone()),
                            retention_period_hours: Some(stream.retention_period_hours as i32),
                            stream_creation_timestamp: Some(
                                stream.created_timestamp.timestamp() as f64
                            ),
                            enhanced_monitoring: Some(vec![wire::EnhancedMetrics {
                                shard_level_metrics: Some(stream.enhanced_monitoring.clone()),
                            }]),
                            open_shard_count: Some(open_count),
                            consumer_count: Some(stream.consumers.len() as i32),
                            encryption_type: if stream.encryption_type == "NONE" {
                                None
                            } else {
                                Some(stream.encryption_type.clone())
                            },
                            key_id: stream.key_id.clone(),
                            stream_mode_details: Some(wire::StreamModeDetails {
                                stream_mode: stream.stream_mode.clone(),
                            }),
                            ..Default::default()
                        }),
                    },
                )
            }
            Err(e) => kinesis_error_response(&e),
        }
    }

    async fn handle_describe_limits(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let total_open_shards: i32 = state
            .streams
            .values()
            .map(|s| s.shards.iter().filter(|sh| !sh.closed).count() as i32)
            .sum();
        let on_demand_count = state
            .streams
            .values()
            .filter(|s| s.stream_mode == "ON_DEMAND")
            .count() as i32;

        wire::serialize_describe_limits_response(&wire::DescribeLimitsOutput {
            open_shard_count: Some(total_open_shards),
            shard_limit: Some(6000),
            on_demand_stream_count: Some(on_demand_count),
            on_demand_stream_count_limit: Some(50),
        })
    }

    async fn handle_list_shards(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisState>>,
        body: &[u8],
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_list_shards_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let stream_name = input.stream_name.as_deref();
        let stream_arn = input.stream_a_r_n.as_deref();
        let max_results = input.max_results.map(|v| v as usize);
        let next_token = input.next_token.as_deref();

        let state = state.read().await;
        match state.list_shards(stream_name, stream_arn, account_id) {
            Ok(all_shards) => {
                // Only return active (non-closed) shards for list_shards
                let active_shards: Vec<&crate::types::ShardData> =
                    all_shards.iter().filter(|s| !s.closed).collect();

                let start_idx = if let Some(token) = next_token {
                    token.parse::<usize>().unwrap_or(0)
                } else {
                    0
                };

                let end_idx = if let Some(max) = max_results {
                    (start_idx + max).min(active_shards.len())
                } else {
                    active_shards.len()
                };

                let page: Vec<wire::Shard> = active_shards[start_idx..end_idx]
                    .iter()
                    .map(|s| wire::Shard {
                        shard_id: Some(s.shard_id.clone()),
                        hash_key_range: Some(wire::HashKeyRange {
                            starting_hash_key: Some(s.starting_hash_key.clone()),
                            ending_hash_key: Some(s.ending_hash_key.clone()),
                        }),
                        sequence_number_range: Some(wire::SequenceNumberRange {
                            starting_sequence_number: Some("0".to_string()),
                            ending_sequence_number: None,
                        }),
                        parent_shard_id: s.parent_shard_id.clone(),
                        adjacent_parent_shard_id: s.adjacent_parent_shard_id.clone(),
                    })
                    .collect();

                let next = if end_idx < active_shards.len() {
                    Some(end_idx.to_string())
                } else {
                    None
                };

                wire::serialize_list_shards_response(&wire::ListShardsOutput {
                    shards: Some(page),
                    next_token: next,
                })
            }
            Err(e) => kinesis_error_response(&e),
        }
    }

    async fn handle_describe_account_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisState>>,
    ) -> MockResponse {
        let _state = state.read().await;
        wire::serialize_describe_account_settings_response(&wire::DescribeAccountSettingsOutput {
            minimum_throughput_billing_commitment: None,
        })
    }

    async fn handle_update_account_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_account_settings_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let status = input.minimum_throughput_billing_commitment.status;
        let commitment_status = if status.is_empty() {
            None
        } else {
            Some(status.as_str())
        };
        let mut state = state.write().await;
        state.update_account_settings(commitment_status);
        wire::serialize_update_account_settings_response(&wire::UpdateAccountSettingsOutput {
            minimum_throughput_billing_commitment: None,
        })
    }

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_a_r_n.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ResourceARN'");
        }
        let tags = input.tags;

        let mut state = state.write().await;
        match state.tag_resource(&input.resource_a_r_n, tags) {
            Ok(()) => wire::serialize_tag_resource_response(),
            Err(e) => kinesis_error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_a_r_n.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ResourceARN'");
        }

        let mut state = state.write().await;
        match state.untag_resource(&input.resource_a_r_n, &input.tag_keys) {
            Ok(()) => wire::serialize_untag_resource_response(),
            Err(e) => kinesis_error_response(&e),
        }
    }

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_a_r_n.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ResourceARN'");
        }

        let state = state.read().await;
        match state.list_tags_for_resource(&input.resource_a_r_n) {
            Ok(tags) => {
                let tag_list: Vec<wire::Tag> = tags
                    .into_iter()
                    .map(|(k, v)| wire::Tag {
                        key: Some(k),
                        value: Some(v),
                    })
                    .collect();
                wire::serialize_list_tags_for_resource_response(&wire::ListTagsForResourceOutput {
                    tags: Some(tag_list),
                })
            }
            Err(e) => kinesis_error_response(&e),
        }
    }

    async fn handle_update_max_record_size(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisState>>,
        body: &[u8],
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_update_max_record_size_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        // The Smithy model only declares StreamARN/StreamId on this input shape, so
        // StreamName is always None here.
        let stream_arn = input.stream_a_r_n.as_deref();
        let stream_name: Option<&str> = None;
        let max_record_size = input.max_record_size_in_ki_b;

        let mut state = state.write().await;
        match state.update_max_record_size(stream_name, stream_arn, max_record_size, account_id) {
            Ok(()) => wire::serialize_update_max_record_size_response(),
            Err(e) => kinesis_error_response(&e),
        }
    }

    async fn handle_update_stream_warm_throughput(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisState>>,
        body: &[u8],
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_update_stream_warm_throughput_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let stream_name = input.stream_name.as_deref();
        let stream_arn = input.stream_a_r_n.as_deref();

        let mut state = state.write().await;
        match state.update_stream_warm_throughput(stream_name, stream_arn, account_id) {
            Ok((sname, sarn)) => wire::serialize_update_stream_warm_throughput_response(
                &wire::UpdateStreamWarmThroughputOutput {
                    stream_name: Some(sname),
                    stream_a_r_n: Some(sarn),
                    warm_throughput: None,
                },
            ),
            Err(e) => kinesis_error_response(&e),
        }
    }
}

fn kinesis_error_response(err: &KinesisError) -> MockResponse {
    let (status, error_type) = match err {
        KinesisError::StreamAlreadyExists { .. } => (400, "ResourceInUseException"),
        KinesisError::ConsumerAlreadyExists { .. } => (400, "ResourceInUseException"),
        KinesisError::StreamNotFound { .. } => (400, "ResourceNotFoundException"),
        KinesisError::StreamArnNotFound { .. } => (400, "ResourceNotFoundException"),
        KinesisError::StreamArnDoesNotExist { .. } => (400, "ResourceNotFoundException"),
        KinesisError::ConsumerNotFound { .. } => (400, "ResourceNotFoundException"),
        KinesisError::ShardNotFound => (400, "ResourceNotFoundException"),
        KinesisError::ShardDoesNotExist { .. } => (400, "ResourceNotFoundException"),
        KinesisError::ShardInStreamNotFound { .. } => (400, "ResourceNotFoundException"),
        KinesisError::ResourceStreamNotFound { .. } => (400, "ResourceNotFoundException"),
        KinesisError::ResourceNotFound { .. } => (400, "ResourceNotFoundException"),
        KinesisError::NoResourcePolicy { .. } => (400, "ResourceNotFoundException"),
        KinesisError::StreamArnModeNotFound { .. } => (400, "ResourceNotFoundException"),
        KinesisError::MissingStreamIdentifier => (400, "ValidationException"),
        KinesisError::StreamOnDemand { .. } => (400, "ValidationException"),
        KinesisError::InvalidShardIdPattern { .. } => (400, "ValidationException"),
        KinesisError::InvalidHashKeyPattern { .. } => (400, "ValidationException"),
        KinesisError::RetentionPeriodTooShort { .. } => (400, "InvalidArgumentException"),
        KinesisError::RetentionPeriodTooLong { .. } => (400, "InvalidArgumentException"),
        KinesisError::RetentionCannotShorten { .. } => (400, "InvalidArgumentException"),
        KinesisError::RetentionCannotLengthen { .. } => (400, "InvalidArgumentException"),
        KinesisError::ShardsNotAdjacent { .. } => (400, "InvalidArgumentException"),
        KinesisError::ShardAlreadyClosed { .. } => (400, "InvalidArgumentException"),
        KinesisError::InvalidHashKeyForSplit { .. } => (400, "InvalidArgumentException"),
    };
    let body = json!({
        "__type": error_type,
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

/// Extract region from Kinesis URI, handling both standard and control-plane endpoints.
fn extract_kinesis_region(uri: &str) -> String {
    if let Some(start) = uri.find("://") {
        let rest = &uri[start + 3..];
        let host = rest.split('/').next().unwrap_or(rest);
        let host = host.split(':').next().unwrap_or(host);
        let parts: Vec<&str> = host.split('.').collect();
        if parts.len() >= 5 && parts[1] == "control-kinesis" {
            return parts[2].to_string();
        }
        if parts.len() >= 4 && parts[0] == "control-kinesis" {
            return parts[1].to_string();
        }
    }
    winterbaume_core::auth::extract_region_from_uri(uri)
}
