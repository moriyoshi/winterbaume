use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
};
use winterbaume_dynamodb::{DynamoDbBackend, DynamoDbError, StreamChangeRecord};

use crate::state::{DynamoDbStreamsError, DynamoDbStreamsState, ShardIteratorState};
use crate::views::DynamoDbStreamsStateView;
use crate::wire;

/// DynamoDB Streams service handler that processes awsJson1.0 protocol requests.
///
/// To wire up real stream data, construct with [`DynamoDbStreamsService::with_dynamodb_backend`]
/// passing the same [`DynamoDbBackend`] instance used by [`winterbaume_dynamodb::DynamoDbService`].
/// When constructed with [`DynamoDbStreamsService::new`] the service has no DynamoDB backend and
/// all stream APIs return empty results (ListStreams) or ResourceNotFoundException
/// (DescribeStream).
pub struct DynamoDbStreamsService {
    pub(crate) state: Arc<BackendState<DynamoDbStreamsState>>,
    pub(crate) notifier: StateChangeNotifier<DynamoDbStreamsStateView>,
    /// Shared DynamoDB backend used to derive stream existence from table state.
    pub(crate) dynamodb_backend: Option<Arc<dyn DynamoDbBackend>>,
}

impl DynamoDbStreamsService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
            dynamodb_backend: None,
        }
    }

    /// Create a Streams service backed by the given DynamoDB backend.
    ///
    /// Pass the same backend instance that was given to
    /// [`winterbaume_dynamodb::DynamoDbService::with_backend`] so that
    /// streams reflect actual table state.
    pub fn with_dynamodb_backend(backend: Arc<dyn DynamoDbBackend>) -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
            dynamodb_backend: Some(backend),
        }
    }
}

impl Default for DynamoDbStreamsService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for DynamoDbStreamsService {
    fn service_name(&self) -> &str {
        "dynamodbstreams"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://streams\.dynamodb\..*\.amazonaws\.com",
            r"https?://streams\.dynamodb\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl DynamoDbStreamsService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        // Prefer the region from SigV4 Authorization header since
        // extract_region_from_uri mis-parses streams.dynamodb.<region>.amazonaws.com
        // (returns "dynamodb" instead of the actual region).
        let region = winterbaume_core::auth::extract_region_from_headers(&request.headers)
            .unwrap_or_else(|| winterbaume_core::auth::extract_region_from_uri(&request.uri));
        let account_id = DEFAULT_ACCOUNT_ID;

        // Extract action from X-Amz-Target header
        // Format: "DynamoDBStreams_20120810.ListStreams"
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

        if serde_json::from_slice::<Value>(&request.body).is_err() {
            return json_error_response(400, "SerializationException", "Invalid JSON body");
        }
        let body_bytes: &[u8] = &request.body;

        match action.as_str() {
            "ListStreams" => {
                self.handle_list_streams(account_id, &region, body_bytes)
                    .await
            }
            "DescribeStream" => {
                self.handle_describe_stream(account_id, &region, body_bytes)
                    .await
            }
            "GetShardIterator" => {
                self.handle_get_shard_iterator(account_id, &region, body_bytes)
                    .await
            }
            "GetRecords" => {
                self.handle_get_records(account_id, &region, body_bytes)
                    .await
            }
            _ => json_error_response(
                400,
                "UnknownOperationException",
                &format!("Unknown operation: {action}"),
            ),
        }
    }

    async fn handle_list_streams(
        &self,
        account_id: &str,
        region: &str,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_streams_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let table_name = input.table_name.as_deref();

        let Some(ref backend) = self.dynamodb_backend else {
            return wire::serialize_list_streams_response(&wire::ListStreamsOutput {
                streams: Some(vec![]),
                last_evaluated_stream_arn: None,
            });
        };

        match backend
            .list_streams(
                account_id.to_string(),
                region.to_string(),
                table_name.map(|s| s.to_string()),
            )
            .await
        {
            Ok(streams) => {
                let stream_list: Vec<wire::Stream> = streams
                    .iter()
                    .map(|s| wire::Stream {
                        stream_arn: Some(s.stream_arn.clone()),
                        stream_label: Some(s.stream_label.clone()),
                        table_name: Some(s.table_name.clone()),
                    })
                    .collect();

                wire::serialize_list_streams_response(&wire::ListStreamsOutput {
                    streams: Some(stream_list),
                    last_evaluated_stream_arn: None,
                })
            }
            Err(e) => dynamodbstreams_error_response(&DynamoDbStreamsError::BackendError(e)),
        }
    }

    async fn handle_describe_stream(
        &self,
        account_id: &str,
        region: &str,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_stream_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.stream_arn.is_empty() {
            return json_error_response(400, "ValidationException", "StreamArn is required");
        }
        let stream_arn = input.stream_arn.clone();

        let Some(ref backend) = self.dynamodb_backend else {
            return json_error_response(
                400,
                "ResourceNotFoundException",
                &format!("Requested resource not found: Stream: {stream_arn} not found"),
            );
        };

        match backend
            .describe_stream_by_arn(
                account_id.to_string(),
                region.to_string(),
                stream_arn.clone(),
            )
            .await
        {
            Ok(summary) => {
                let shard_id = format!("shardId-{:012}", 0);
                let key_schema: Vec<wire::KeySchemaElement> = summary
                    .key_schema
                    .iter()
                    .map(|k| wire::KeySchemaElement {
                        attribute_name: Some(k.attribute_name.clone()),
                        key_type: Some(k.key_type.clone()),
                    })
                    .collect();

                wire::serialize_describe_stream_response(&wire::DescribeStreamOutput {
                    stream_description: Some(wire::StreamDescription {
                        stream_arn: Some(summary.stream_arn),
                        stream_label: Some(summary.stream_label),
                        stream_status: Some("ENABLED".to_string()),
                        stream_view_type: Some(summary.stream_view_type),
                        table_name: Some(summary.table_name),
                        key_schema: Some(key_schema),
                        shards: Some(vec![wire::Shard {
                            shard_id: Some(shard_id),
                            sequence_number_range: Some(wire::SequenceNumberRange {
                                starting_sequence_number: Some("000000000000000000001".to_string()),
                                ending_sequence_number: None,
                            }),
                            parent_shard_id: None,
                        }]),
                        creation_request_date_time: Some(1.0),
                        last_evaluated_shard_id: None,
                    }),
                })
            }
            Err(e) => dynamodbstreams_error_response(&DynamoDbStreamsError::BackendError(e)),
        }
    }

    async fn handle_get_shard_iterator(
        &self,
        account_id: &str,
        region: &str,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_shard_iterator_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.stream_arn.is_empty() {
            return json_error_response(400, "ValidationException", "StreamArn is required");
        }
        let stream_arn = input.stream_arn.as_str();

        if input.shard_id.is_empty() {
            return json_error_response(400, "ValidationException", "ShardId is required");
        }
        let shard_id = input.shard_id.as_str();

        // Validate the stream exists (if we have a backend)
        if let Some(ref backend) = self.dynamodb_backend {
            if let Err(e) = backend
                .describe_stream_by_arn(
                    account_id.to_string(),
                    region.to_string(),
                    stream_arn.to_string(),
                )
                .await
            {
                return dynamodbstreams_error_response(&DynamoDbStreamsError::BackendError(e));
            }
        }

        let iterator_type = if input.shard_iterator_type.is_empty() {
            "TRIM_HORIZON"
        } else {
            input.shard_iterator_type.as_str()
        };

        // next_sequence_number is the first sequence number we want to read
        // (inclusive).  For TRIM_HORIZON that's 0 (all records), for LATEST
        // it's u64::MAX so only future writes are visible.
        let next_sequence_number: u64 = match iterator_type {
            "LATEST" => u64::MAX,
            "AFTER_SEQUENCE_NUMBER" => input
                .sequence_number
                .as_deref()
                .and_then(|s| s.parse::<u64>().ok())
                .map(|n| n + 1)
                .unwrap_or(0),
            "AT_SEQUENCE_NUMBER" => input
                .sequence_number
                .as_deref()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0),
            _ => 0, // TRIM_HORIZON
        };

        let iterator_id = format!("iter-{}-{}-{}", stream_arn, shard_id, uuid::Uuid::new_v4());

        let state = self.state.get(account_id, region);
        let mut s = state.write().await;
        s.shard_iterators.insert(
            iterator_id.clone(),
            ShardIteratorState {
                stream_arn: stream_arn.to_string(),
                shard_id: shard_id.to_string(),
                next_sequence_number,
            },
        );

        wire::serialize_get_shard_iterator_response(&wire::GetShardIteratorOutput {
            shard_iterator: Some(iterator_id),
        })
    }

    async fn handle_get_records(
        &self,
        account_id: &str,
        region: &str,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_records_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.shard_iterator.is_empty() {
            return json_error_response(400, "ValidationException", "ShardIterator is required");
        }
        let shard_iterator = input.shard_iterator.as_str();

        let limit: Option<usize> = input.limit.map(|n| n as usize);

        // Read current iterator state
        let (stream_arn, next_sequence_number) = {
            let state = self.state.get(account_id, region);
            let s = state.read().await;
            match s.shard_iterators.get(shard_iterator) {
                Some(it) => (it.stream_arn.clone(), it.next_sequence_number),
                None => {
                    return dynamodbstreams_error_response(
                        &DynamoDbStreamsError::ExpiredIteratorException,
                    );
                }
            }
        };

        // Fetch records from DynamoDB backend
        let (change_records, new_next_seq) = match &self.dynamodb_backend {
            Some(backend) => {
                match backend
                    .get_stream_records(
                        account_id.to_string(),
                        region.to_string(),
                        stream_arn.clone(),
                        next_sequence_number,
                        limit,
                    )
                    .await
                {
                    Ok(r) => r,
                    Err(e) => {
                        return dynamodbstreams_error_response(
                            &DynamoDbStreamsError::BackendError(e),
                        );
                    }
                }
            }
            None => (vec![], next_sequence_number),
        };

        // Update iterator position
        {
            let state = self.state.get(account_id, region);
            let mut s = state.write().await;
            if let Some(it) = s.shard_iterators.get_mut(shard_iterator) {
                it.next_sequence_number = new_next_seq;
            }
        }

        // Build next shard iterator (reuse same ID, position updated above)
        let next_shard_iterator = Some(shard_iterator.to_string());

        // Convert StreamChangeRecord -> wire::Record
        let records: Vec<wire::Record> = change_records
            .iter()
            .map(|r| stream_change_record_to_wire(r, region))
            .collect();

        wire::serialize_get_records_response(&wire::GetRecordsOutput {
            records: Some(records),
            next_shard_iterator,
        })
    }
}

fn dynamodbstreams_error_response(err: &DynamoDbStreamsError) -> MockResponse {
    let (status, error_type): (u16, &str) = match err {
        DynamoDbStreamsError::ExpiredIteratorException => (400, "ExpiredIteratorException"),
        DynamoDbStreamsError::BackendError(e) => match e {
            DynamoDbError::ResourceInUse(_) => (
                400,
                "com.amazonaws.dynamodb.v20120810#ResourceInUseException",
            ),
            DynamoDbError::NoHashKey => {
                (400, "com.amazonaws.dynamodb.v20120810#ValidationException")
            }
            DynamoDbError::MissingKey(_) => {
                (400, "com.amazonaws.dynamodb.v20120810#ValidationException")
            }
            DynamoDbError::QueryConditionMissedKey => {
                (400, "com.amazonaws.dynamodb.v20120810#ValidationException")
            }
            DynamoDbError::ResourceNotFound(_) => (
                400,
                "com.amazonaws.dynamodb.v20120810#ResourceNotFoundException",
            ),
            DynamoDbError::BackupNotFound(_) => (
                400,
                "com.amazonaws.dynamodb.v20120810#BackupNotFoundException",
            ),
            DynamoDbError::TableAlreadyExists(_) => (
                400,
                "com.amazonaws.dynamodb.v20120810#TableAlreadyExistsException",
            ),
            DynamoDbError::SourceTableNotFound(_) => (
                400,
                "com.amazonaws.dynamodb.v20120810#SourceTableNotFoundException",
            ),
            DynamoDbError::GlobalTableAlreadyExists(_) => (
                400,
                "com.amazonaws.dynamodb.v20120810#GlobalTableAlreadyExistsException",
            ),
            DynamoDbError::GlobalTableNotFound(_) => (
                400,
                "com.amazonaws.dynamodb.v20120810#GlobalTableNotFoundException",
            ),
            DynamoDbError::KinesisDestinationNotFound { .. } => (
                400,
                "com.amazonaws.dynamodb.v20120810#ResourceNotFoundException",
            ),
            DynamoDbError::TableNotFound(_) => (
                400,
                "com.amazonaws.dynamodb.v20120810#TableNotFoundException",
            ),
            DynamoDbError::TableNotFoundByArn(_) => (
                400,
                "com.amazonaws.dynamodb.v20120810#TableNotFoundException",
            ),
            DynamoDbError::StreamNotFound(_) => (
                400,
                "com.amazonaws.dynamodb.v20120810#ResourceNotFoundException",
            ),
            DynamoDbError::ConditionalCheckFailed => (
                400,
                "com.amazonaws.dynamodb.v20120810#ConditionalCheckFailedException",
            ),
            DynamoDbError::ExportNotFound(_) => (
                400,
                "com.amazonaws.dynamodb.v20120810#ExportNotFoundException",
            ),
            DynamoDbError::ImportNotFound(_) => (
                400,
                "com.amazonaws.dynamodb.v20120810#ImportNotFoundException",
            ),
            DynamoDbError::InternalError(_) => (500, "InternalError"),
        },
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

/// Convert a `StreamChangeRecord` (from the DynamoDB backend) to the wire
/// `Record` type expected by the DynamoDB Streams response protocol.
fn stream_change_record_to_wire(r: &StreamChangeRecord, region: &str) -> wire::Record {
    let keys = item_to_wire_attrs(&r.keys);
    let new_image = r.new_image.as_ref().map(item_to_wire_attrs);
    let old_image = r.old_image.as_ref().map(item_to_wire_attrs);

    wire::Record {
        aws_region: Some(region.to_string()),
        event_i_d: Some(r.event_id.clone()),
        event_name: Some(r.event_name.clone()),
        event_source: Some("aws:dynamodb".to_string()),
        event_version: Some("1.1".to_string()),
        user_identity: None,
        dynamodb: Some(wire::StreamRecord {
            approximate_creation_date_time: Some(r.approximate_creation_date_time),
            sequence_number: Some(r.sequence_number.to_string()),
            size_bytes: None,
            stream_view_type: None,
            keys: Some(keys),
            new_image,
            old_image,
        }),
    }
}

/// Convert a DynamoDB `Item` (`HashMap<String, AttributeValue>`) to the wire
/// `AttributeValue` map used in stream records.
///
/// `winterbaume_dynamodb::AttributeValue` and `wire::AttributeValue` share the
/// same DynamoDB JSON serialization format, so we go through JSON to convert.
fn item_to_wire_attrs(item: &winterbaume_dynamodb::Item) -> HashMap<String, wire::AttributeValue> {
    item.iter()
        .filter_map(|(k, v)| {
            serde_json::to_value(v)
                .ok()
                .and_then(|jv| serde_json::from_value::<wire::AttributeValue>(jv).ok())
                .map(|av| (k.clone(), av))
        })
        .collect()
}
