use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
    StatefulService,
};

use crate::state::{LogsError, LogsState};
use crate::types;
use crate::views::LogsStateView;
use crate::wire;

pub struct CloudWatchLogsService {
    pub(crate) state: Arc<BackendState<LogsState>>,
    pub(crate) notifier: StateChangeNotifier<LogsStateView>,
}

impl CloudWatchLogsService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for CloudWatchLogsService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for CloudWatchLogsService {
    fn service_name(&self) -> &str {
        "logs"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://logs\..*\.amazonaws\.com",
            r"https?://logs\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl CloudWatchLogsService {
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

        // Validate the body is well-formed JSON up-front; the typed deserialisers in
        // `wire` re-parse the bytes per operation.
        if serde_json::from_slice::<Value>(&request.body).is_err() {
            return json_error_response(400, "SerializationException", "Invalid JSON body");
        }
        let body_bytes: &[u8] = &request.body;

        let state = self.state.get(account_id, &region);

        let response = match action.as_str() {
            // Log Groups
            "CreateLogGroup" => {
                self.handle_create_log_group(&state, body_bytes, account_id, &region)
                    .await
            }
            "DeleteLogGroup" => self.handle_delete_log_group(&state, body_bytes).await,
            "DescribeLogGroups" => self.handle_describe_log_groups(&state, body_bytes).await,
            // Log Streams
            "CreateLogStream" => {
                self.handle_create_log_stream(&state, body_bytes, account_id, &region)
                    .await
            }
            "DeleteLogStream" => self.handle_delete_log_stream(&state, body_bytes).await,
            "DescribeLogStreams" => self.handle_describe_log_streams(&state, body_bytes).await,
            // Log Events
            "PutLogEvents" => self.handle_put_log_events(&state, body_bytes).await,
            "GetLogEvents" => self.handle_get_log_events(&state, body_bytes).await,
            "FilterLogEvents" => self.handle_filter_log_events(&state, body_bytes).await,
            // Retention Policy
            "PutRetentionPolicy" => self.handle_put_retention_policy(&state, body_bytes).await,
            "DeleteRetentionPolicy" => {
                self.handle_delete_retention_policy(&state, body_bytes)
                    .await
            }
            // Metric Filters
            "PutMetricFilter" => self.handle_put_metric_filter(&state, body_bytes).await,
            "DeleteMetricFilter" => self.handle_delete_metric_filter(&state, body_bytes).await,
            "DescribeMetricFilters" => {
                self.handle_describe_metric_filters(&state, body_bytes)
                    .await
            }
            // Subscription Filters
            "PutSubscriptionFilter" => {
                self.handle_put_subscription_filter(&state, body_bytes)
                    .await
            }
            "DeleteSubscriptionFilter" => {
                self.handle_delete_subscription_filter(&state, body_bytes)
                    .await
            }
            "DescribeSubscriptionFilters" => {
                self.handle_describe_subscription_filters(&state, body_bytes)
                    .await
            }
            // Resource Policies
            "PutResourcePolicy" => self.handle_put_resource_policy(&state, body_bytes).await,
            "DeleteResourcePolicy" => self.handle_delete_resource_policy(&state, body_bytes).await,
            "DescribeResourcePolicies" => self.handle_describe_resource_policies(&state).await,
            // Destinations
            "PutDestination" => {
                self.handle_put_destination(&state, body_bytes, account_id, &region)
                    .await
            }
            "PutDestinationPolicy" => self.handle_put_destination_policy(&state, body_bytes).await,
            "DeleteDestination" => self.handle_delete_destination(&state, body_bytes).await,
            "DescribeDestinations" => self.handle_describe_destinations(&state, body_bytes).await,
            // Export Tasks
            "CreateExportTask" => self.handle_create_export_task(&state, body_bytes).await,
            "CancelExportTask" => self.handle_cancel_export_task(&state, body_bytes).await,
            "DescribeExportTasks" => self.handle_describe_export_tasks(&state, body_bytes).await,
            // Queries
            "StartQuery" => self.handle_start_query(&state, body_bytes).await,
            "GetQueryResults" => self.handle_get_query_results(&state, body_bytes).await,
            "DescribeQueries" => self.handle_describe_queries(&state, body_bytes).await,
            // Tags (legacy log group)
            "TagLogGroup" => self.handle_tag_log_group(&state, body_bytes).await,
            "UntagLogGroup" => self.handle_untag_log_group(&state, body_bytes).await,
            "ListTagsLogGroup" => self.handle_list_tags_log_group(&state, body_bytes).await,
            // Tags (ARN-based)
            "TagResource" => self.handle_tag_resource(&state, body_bytes).await,
            "UntagResource" => self.handle_untag_resource(&state, body_bytes).await,
            "ListTagsForResource" => self.handle_list_tags_for_resource(&state, body_bytes).await,
            // Delivery Sources
            "PutDeliverySource" => self.handle_put_delivery_source(&state, body_bytes).await,
            "GetDeliverySource" => self.handle_get_delivery_source(&state, body_bytes).await,
            "DeleteDeliverySource" => self.handle_delete_delivery_source(&state, body_bytes).await,
            "DescribeDeliverySources" => self.handle_describe_delivery_sources(&state).await,
            // Delivery Destinations
            "PutDeliveryDestination" => {
                self.handle_put_delivery_destination(&state, body_bytes)
                    .await
            }
            "GetDeliveryDestination" => {
                self.handle_get_delivery_destination(&state, body_bytes)
                    .await
            }
            "DeleteDeliveryDestination" => {
                self.handle_delete_delivery_destination(&state, body_bytes)
                    .await
            }
            "DescribeDeliveryDestinations" => {
                self.handle_describe_delivery_destinations(&state).await
            }
            // Delivery Destination Policy
            "PutDeliveryDestinationPolicy" => {
                self.handle_put_delivery_destination_policy(&state, body_bytes)
                    .await
            }
            "GetDeliveryDestinationPolicy" => {
                self.handle_get_delivery_destination_policy(&state, body_bytes)
                    .await
            }
            "DeleteDeliveryDestinationPolicy" => {
                self.handle_delete_delivery_destination_policy(&state, body_bytes)
                    .await
            }
            // Deliveries
            "CreateDelivery" => self.handle_create_delivery(&state, body_bytes).await,
            "GetDelivery" => self.handle_get_delivery(&state, body_bytes).await,
            "DeleteDelivery" => self.handle_delete_delivery(&state, body_bytes).await,
            "DescribeDeliveries" => self.handle_describe_deliveries(&state).await,
            "UpdateDeliveryConfiguration" => {
                self.handle_update_delivery_configuration(&state, body_bytes)
                    .await
            }
            // Account Policies
            "PutAccountPolicy" => {
                self.handle_put_account_policy(&state, body_bytes, account_id)
                    .await
            }
            "DeleteAccountPolicy" => self.handle_delete_account_policy(&state, body_bytes).await,
            "DescribeAccountPolicies" => {
                self.handle_describe_account_policies(&state, body_bytes)
                    .await
            }
            // Data Protection Policies
            "PutDataProtectionPolicy" => {
                self.handle_put_data_protection_policy(&state, body_bytes)
                    .await
            }
            "GetDataProtectionPolicy" => {
                self.handle_get_data_protection_policy(&state, body_bytes)
                    .await
            }
            "DeleteDataProtectionPolicy" => {
                self.handle_delete_data_protection_policy(&state, body_bytes)
                    .await
            }
            // Index Policies
            "PutIndexPolicy" => self.handle_put_index_policy(&state, body_bytes).await,
            "DeleteIndexPolicy" => self.handle_delete_index_policy(&state, body_bytes).await,
            "DescribeFieldIndexes" => self.handle_describe_field_indexes(&state).await,
            "DescribeIndexPolicies" => {
                self.handle_describe_index_policies(&state, body_bytes)
                    .await
            }
            // Query Definitions
            "PutQueryDefinition" => self.handle_put_query_definition(&state, body_bytes).await,
            "DeleteQueryDefinition" => {
                self.handle_delete_query_definition(&state, body_bytes)
                    .await
            }
            "DescribeQueryDefinitions" => {
                self.handle_describe_query_definitions(&state, body_bytes)
                    .await
            }
            // Log Anomaly Detectors
            "CreateLogAnomalyDetector" => {
                self.handle_create_log_anomaly_detector(&state, body_bytes, account_id, &region)
                    .await
            }
            "DeleteLogAnomalyDetector" => {
                self.handle_delete_log_anomaly_detector(&state, body_bytes)
                    .await
            }
            "GetLogAnomalyDetector" => {
                self.handle_get_log_anomaly_detector(&state, body_bytes)
                    .await
            }
            "ListLogAnomalyDetectors" => {
                self.handle_list_log_anomaly_detectors(&state, body_bytes)
                    .await
            }
            "UpdateLogAnomalyDetector" => {
                self.handle_update_log_anomaly_detector(&state, body_bytes)
                    .await
            }
            // Anomalies
            "ListAnomalies" => self.handle_list_anomalies(&state, body_bytes).await,
            "UpdateAnomaly" => self.handle_update_anomaly(&state, body_bytes).await,
            // Integrations
            "PutIntegration" => self.handle_put_integration(&state, body_bytes).await,
            "GetIntegration" => self.handle_get_integration(&state, body_bytes).await,
            "DeleteIntegration" => self.handle_delete_integration(&state, body_bytes).await,
            "ListIntegrations" => self.handle_list_integrations(&state).await,
            // Scheduled Queries
            "CreateScheduledQuery" => {
                self.handle_create_scheduled_query(&state, body_bytes, account_id, &region)
                    .await
            }
            "DeleteScheduledQuery" => self.handle_delete_scheduled_query(&state, body_bytes).await,
            "GetScheduledQuery" => self.handle_get_scheduled_query(&state, body_bytes).await,
            "ListScheduledQueries" => self.handle_list_scheduled_queries(&state).await,
            "UpdateScheduledQuery" => self.handle_update_scheduled_query(&state, body_bytes).await,
            "GetScheduledQueryHistory" => self.handle_get_scheduled_query_history(body_bytes).await,
            // Transformers
            "PutTransformer" => self.handle_put_transformer(&state, body_bytes).await,
            "GetTransformer" => self.handle_get_transformer(&state, body_bytes).await,
            "DeleteTransformer" => self.handle_delete_transformer(&state, body_bytes).await,
            "TestTransformer" => self.handle_test_transformer(body_bytes).await,
            // Import Tasks
            "CreateImportTask" => self.handle_create_import_task(&state, body_bytes).await,
            "CancelImportTask" => self.handle_cancel_import_task(&state, body_bytes).await,
            "DescribeImportTasks" => self.handle_describe_import_tasks(&state, body_bytes).await,
            "DescribeImportTaskBatches" => {
                self.handle_describe_import_task_batches(body_bytes).await
            }
            // KMS
            "AssociateKmsKey" => self.handle_associate_kms_key(&state, body_bytes).await,
            "DisassociateKmsKey" => self.handle_disassociate_kms_key(&state, body_bytes).await,
            // S3 Table Integration
            "AssociateSourceToS3TableIntegration" => {
                self.handle_associate_source_to_s3_table_integration().await
            }
            "DisassociateSourceFromS3TableIntegration" => {
                self.handle_disassociate_source_from_s3_table_integration()
                    .await
            }
            "ListSourcesForS3TableIntegration" => {
                self.handle_list_sources_for_s3_table_integration().await
            }
            // Log query helpers
            "GetLogGroupFields" => self.handle_get_log_group_fields().await,
            "GetLogFields" => self.handle_get_log_fields().await,
            "GetLogObject" => self.handle_get_log_object().await,
            "GetLogRecord" => self.handle_get_log_record().await,
            "StopQuery" => self.handle_stop_query(&state, body_bytes).await,
            "TestMetricFilter" => self.handle_test_metric_filter().await,
            // Other
            "DescribeConfigurationTemplates" => {
                self.handle_describe_configuration_templates().await
            }
            "ListAggregateLogGroupSummaries" => {
                self.handle_list_aggregate_log_group_summaries().await
            }
            "ListGroups" => self.handle_list_log_groups(&state, body_bytes).await,
            "ListLogGroups" => self.handle_list_log_groups(&state, body_bytes).await,
            "ListLogGroupsForQuery" => self.handle_list_log_groups_for_query().await,
            "PutBearerTokenAuthentication" => self.handle_put_bearer_token_authentication().await,
            "PutLogGroupDeletionProtection" => {
                self.handle_put_log_group_deletion_protection(&state, body_bytes)
                    .await
            }
            "StartLiveTail" => self.handle_start_live_tail().await,
            _ => json_error_response(
                400,
                "InvalidAction",
                &format!("Could not find operation {action} for Logs"),
            ),
        };

        if response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    // ========== Log Groups ==========

    async fn handle_create_log_group(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_log_group_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.log_group_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'logGroupName'");
        }
        let tags = input.tags.unwrap_or_default();

        let mut state = state.write().await;
        match state.create_log_group(&input.log_group_name, account_id, region, tags) {
            Ok(_) => wire::serialize_create_log_group_response(),
            Err(e) => logs_error_response(&e),
        }
    }

    async fn handle_delete_log_group(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_log_group_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.log_group_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'logGroupName'");
        }

        let mut state = state.write().await;
        match state.delete_log_group(&input.log_group_name) {
            Ok(()) => wire::serialize_delete_log_group_response(),
            Err(e) => logs_error_response(&e),
        }
    }

    async fn handle_describe_log_groups(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_log_groups_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let prefix = input.log_group_name_prefix.as_deref();
        let limit = input.limit.map(|v| v as usize);
        let next_token = input.next_token.as_deref();

        let state = state.read().await;
        let mut groups = state.describe_log_groups(prefix);
        groups.sort_by(|a, b| a.name.cmp(&b.name));

        // Pagination: skip past the token
        if let Some(token) = next_token {
            let pos = groups.iter().position(|g| g.name == token);
            match pos {
                Some(idx) => {
                    groups = groups.into_iter().skip(idx + 1).collect();
                }
                None => {
                    // Invalid token returns empty
                    return wire::serialize_describe_log_groups_response(
                        &wire::DescribeLogGroupsResponse {
                            log_groups: Some(vec![]),
                            next_token: None,
                        },
                    );
                }
            }
        }

        let result_token = if let Some(lim) = limit {
            if groups.len() > lim {
                let token = groups[lim - 1].name.clone();
                groups.truncate(lim);
                Some(token)
            } else {
                None
            }
        } else {
            None
        };

        let entries: Vec<wire::LogGroup> = groups
            .iter()
            .map(|g| {
                // logGroupArn is the ARN without :* suffix
                let log_group_arn = g.arn.strip_suffix(":*").unwrap_or(&g.arn).to_string();
                wire::LogGroup {
                    log_group_name: Some(g.name.clone()),
                    arn: Some(g.arn.clone()),
                    log_group_arn: Some(log_group_arn),
                    creation_time: Some(g.creation_time),
                    stored_bytes: Some(0),
                    retention_in_days: g.retention_in_days,
                    ..Default::default()
                }
            })
            .collect();

        wire::serialize_describe_log_groups_response(&wire::DescribeLogGroupsResponse {
            log_groups: Some(entries),
            next_token: result_token,
        })
    }

    // ========== Log Streams ==========

    async fn handle_create_log_stream(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_log_stream_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.log_group_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'logGroupName'");
        }
        if input.log_stream_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'logStreamName'");
        }

        let mut state = state.write().await;
        match state.create_log_stream(
            &input.log_group_name,
            &input.log_stream_name,
            account_id,
            region,
        ) {
            Ok(()) => wire::serialize_create_log_stream_response(),
            Err(e) => logs_error_response(&e),
        }
    }

    async fn handle_delete_log_stream(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_log_stream_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.log_group_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'logGroupName'");
        }
        if input.log_stream_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'logStreamName'");
        }

        let mut state = state.write().await;
        match state.delete_log_stream(&input.log_group_name, &input.log_stream_name) {
            Ok(()) => wire::serialize_delete_log_stream_response(),
            Err(e) => logs_error_response(&e),
        }
    }

    async fn handle_describe_log_streams(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_log_streams_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let group_name = match input.log_group_name.as_deref() {
            Some(n) if !n.is_empty() => n,
            _ => {
                return json_error_response(400, "ValidationException", "Missing 'logGroupName'");
            }
        };
        let prefix = input.log_stream_name_prefix.as_deref();
        let limit = input.limit.map(|v| v as usize);
        let next_token = input.next_token.as_deref();

        let state = state.read().await;
        match state.describe_log_streams(group_name, prefix) {
            Ok(mut streams) => {
                streams.sort_by(|a, b| a.name.cmp(&b.name));

                // Pagination: token format is "groupName@lastStreamName"
                if let Some(token) = next_token {
                    if let Some((_group, last_stream)) = token.split_once('@') {
                        let pos = streams.iter().position(|s| s.name == last_stream);
                        match pos {
                            Some(idx) => {
                                streams = streams.into_iter().skip(idx + 1).collect();
                            }
                            None => {
                                streams = vec![];
                            }
                        }
                    } else {
                        streams = vec![];
                    }
                }

                let result_token = if let Some(lim) = limit {
                    if streams.len() > lim {
                        let token = format!("{}@{}", group_name, streams[lim - 1].name);
                        streams.truncate(lim);
                        Some(token)
                    } else {
                        None
                    }
                } else {
                    None
                };

                let entries: Vec<wire::LogStream> = streams
                    .iter()
                    .map(|s| wire::LogStream {
                        log_stream_name: Some(s.name.clone()),
                        arn: Some(s.arn.clone()),
                        creation_time: Some(s.creation_time),
                        upload_sequence_token: Some(s.upload_sequence_token.clone()),
                        stored_bytes: Some(0),
                        first_event_timestamp: s.first_event_timestamp,
                        last_event_timestamp: s.last_event_timestamp,
                        ..Default::default()
                    })
                    .collect();

                wire::serialize_describe_log_streams_response(&wire::DescribeLogStreamsResponse {
                    log_streams: Some(entries),
                    next_token: result_token,
                })
            }
            Err(e) => logs_error_response(&e),
        }
    }

    // ========== Log Events ==========

    async fn handle_put_log_events(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_put_log_events_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.log_group_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'logGroupName'");
        }
        if input.log_stream_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'logStreamName'");
        }

        let events: Vec<(i64, String)> = input
            .log_events
            .into_iter()
            .map(|e| (e.timestamp, e.message))
            .collect();

        if events.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "Log events batch must not be empty",
            );
        }

        let mut state = state.write().await;
        match state.put_log_events(&input.log_group_name, &input.log_stream_name, events) {
            Ok(token) => wire::serialize_put_log_events_response(&wire::PutLogEventsResponse {
                next_sequence_token: Some(token),
                rejected_entity_info: None,
                rejected_log_events_info: None,
            }),
            Err(e) => logs_error_response(&e),
        }
    }

    async fn handle_get_log_events(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_log_events_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let group_name = match input.log_group_name.as_deref() {
            Some(n) if !n.is_empty() => n,
            _ => {
                return json_error_response(400, "ValidationException", "Missing 'logGroupName'");
            }
        };
        if input.log_stream_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'logStreamName'");
        }

        let start_time = input.start_time;
        let end_time = input.end_time;
        let limit = input.limit.map(|v| v as usize);

        let state = state.read().await;
        match state.get_log_events(
            group_name,
            &input.log_stream_name,
            start_time,
            end_time,
            limit,
        ) {
            Ok(events) => {
                let entries: Vec<wire::OutputLogEvent> = events
                    .iter()
                    .map(|e| wire::OutputLogEvent {
                        timestamp: Some(e.timestamp),
                        message: Some(e.message.clone()),
                        ingestion_time: Some(e.ingestion_time),
                    })
                    .collect();

                wire::serialize_get_log_events_response(&wire::GetLogEventsResponse {
                    events: Some(entries),
                    next_forward_token: Some("f/next".to_string()),
                    next_backward_token: Some("b/next".to_string()),
                })
            }
            Err(e) => logs_error_response(&e),
        }
    }

    async fn handle_filter_log_events(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_filter_log_events_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let log_group_name = match input.log_group_name.as_deref() {
            Some(n) if !n.is_empty() => n,
            _ => {
                return json_error_response(400, "ValidationException", "Missing 'logGroupName'");
            }
        };

        let log_stream_names = input.log_stream_names;
        let filter_pattern = input.filter_pattern.as_deref();
        let start_time = input.start_time;
        let end_time = input.end_time;
        let limit = input.limit.map(|v| v as usize);

        let state = state.read().await;
        match state.filter_log_events(
            log_group_name,
            log_stream_names.as_deref(),
            filter_pattern,
            start_time,
            end_time,
            limit,
        ) {
            Ok(events) => {
                let entries: Vec<wire::FilteredLogEvent> = events
                    .iter()
                    .map(|e| wire::FilteredLogEvent {
                        event_id: Some(e.event_id.clone()),
                        ingestion_time: Some(e.ingestion_time),
                        log_stream_name: Some(e.log_stream_name.clone()),
                        message: Some(e.message.clone()),
                        timestamp: Some(e.timestamp),
                    })
                    .collect();

                wire::serialize_filter_log_events_response(&wire::FilterLogEventsResponse {
                    events: Some(entries),
                    next_token: None,
                    searched_log_streams: None,
                })
            }
            Err(e) => logs_error_response(&e),
        }
    }

    // ========== Retention Policy ==========

    async fn handle_put_retention_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_put_retention_policy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.log_group_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'logGroupName'");
        }

        let mut state = state.write().await;
        match state.put_retention_policy(&input.log_group_name, input.retention_in_days) {
            Ok(()) => wire::serialize_put_retention_policy_response(),
            Err(e) => logs_error_response(&e),
        }
    }

    async fn handle_delete_retention_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_retention_policy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.log_group_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'logGroupName'");
        }

        let mut state = state.write().await;
        match state.delete_retention_policy(&input.log_group_name) {
            Ok(()) => wire::serialize_delete_retention_policy_response(),
            Err(e) => logs_error_response(&e),
        }
    }

    // ========== Metric Filters ==========

    async fn handle_put_metric_filter(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_put_metric_filter_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.log_group_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'logGroupName'");
        }
        if input.filter_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'filterName'");
        }

        let metric_transformations: Vec<types::MetricTransformation> = input
            .metric_transformations
            .into_iter()
            .map(|t| types::MetricTransformation {
                metric_namespace: t.metric_namespace,
                metric_name: t.metric_name,
                metric_value: t.metric_value,
            })
            .collect();

        let now = chrono::Utc::now().timestamp_millis();
        let filter = types::MetricFilter {
            filter_name: input.filter_name,
            log_group_name: input.log_group_name,
            filter_pattern: input.filter_pattern,
            metric_transformations,
            creation_time: now,
        };

        let mut state = state.write().await;
        match state.put_metric_filter(filter) {
            Ok(()) => wire::serialize_put_metric_filter_response(),
            Err(e) => logs_error_response(&e),
        }
    }

    async fn handle_delete_metric_filter(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_metric_filter_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.log_group_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'logGroupName'");
        }
        if input.filter_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'filterName'");
        }

        let mut state = state.write().await;
        match state.delete_metric_filter(&input.log_group_name, &input.filter_name) {
            Ok(()) => wire::serialize_delete_metric_filter_response(),
            Err(e) => logs_error_response(&e),
        }
    }

    async fn handle_describe_metric_filters(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_metric_filters_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let log_group_name = input.log_group_name.as_deref();
        let filter_name_prefix = input.filter_name_prefix.as_deref();
        let metric_name = input.metric_name.as_deref();
        let metric_namespace = input.metric_namespace.as_deref();

        let state = state.read().await;
        let filters = state.describe_metric_filters(
            log_group_name,
            filter_name_prefix,
            metric_name,
            metric_namespace,
        );

        let entries: Vec<wire::MetricFilter> = filters
            .iter()
            .map(|f| wire::MetricFilter {
                filter_name: Some(f.filter_name.clone()),
                log_group_name: Some(f.log_group_name.clone()),
                filter_pattern: Some(f.filter_pattern.clone()),
                creation_time: Some(f.creation_time),
                metric_transformations: Some(
                    f.metric_transformations
                        .iter()
                        .map(|t| wire::MetricTransformation {
                            metric_namespace: t.metric_namespace.clone(),
                            metric_name: t.metric_name.clone(),
                            metric_value: t.metric_value.clone(),
                            ..Default::default()
                        })
                        .collect(),
                ),
                ..Default::default()
            })
            .collect();

        wire::serialize_describe_metric_filters_response(&wire::DescribeMetricFiltersResponse {
            metric_filters: Some(entries),
            next_token: None,
        })
    }

    // ========== Subscription Filters ==========

    async fn handle_put_subscription_filter(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_put_subscription_filter_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.log_group_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'logGroupName'");
        }
        if input.filter_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'filterName'");
        }
        if input.destination_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'destinationArn'");
        }

        let now = chrono::Utc::now().timestamp_millis();
        let filter = types::SubscriptionFilter {
            filter_name: input.filter_name,
            log_group_name: input.log_group_name,
            filter_pattern: input.filter_pattern,
            destination_arn: input.destination_arn,
            role_arn: input.role_arn,
            creation_time: now,
        };

        let mut state = state.write().await;
        match state.put_subscription_filter(filter) {
            Ok(()) => wire::serialize_put_subscription_filter_response(),
            Err(e) => logs_error_response(&e),
        }
    }

    async fn handle_delete_subscription_filter(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_subscription_filter_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.log_group_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'logGroupName'");
        }
        if input.filter_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'filterName'");
        }

        let mut state = state.write().await;
        match state.delete_subscription_filter(&input.log_group_name, &input.filter_name) {
            Ok(()) => wire::serialize_delete_subscription_filter_response(),
            Err(e) => logs_error_response(&e),
        }
    }

    async fn handle_describe_subscription_filters(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_subscription_filters_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.log_group_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'logGroupName'");
        }
        let filter_name_prefix = input.filter_name_prefix.as_deref();

        let state = state.read().await;
        match state.describe_subscription_filters(&input.log_group_name, filter_name_prefix) {
            Ok(filters) => {
                let entries: Vec<wire::SubscriptionFilter> = filters
                    .iter()
                    .map(|f| wire::SubscriptionFilter {
                        filter_name: Some(f.filter_name.clone()),
                        log_group_name: Some(f.log_group_name.clone()),
                        filter_pattern: Some(f.filter_pattern.clone()),
                        destination_arn: Some(f.destination_arn.clone()),
                        role_arn: f.role_arn.clone(),
                        creation_time: Some(f.creation_time),
                        ..Default::default()
                    })
                    .collect();

                wire::serialize_describe_subscription_filters_response(
                    &wire::DescribeSubscriptionFiltersResponse {
                        subscription_filters: Some(entries),
                        next_token: None,
                    },
                )
            }
            Err(e) => logs_error_response(&e),
        }
    }

    // ========== Resource Policies ==========

    async fn handle_put_resource_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_put_resource_policy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let policy_name = match input.policy_name.as_deref() {
            Some(n) if !n.is_empty() => n,
            _ => {
                return json_error_response(400, "ValidationException", "Missing 'policyName'");
            }
        };
        let policy_document = match input.policy_document.as_deref() {
            Some(d) if !d.is_empty() => d,
            _ => {
                return json_error_response(400, "ValidationException", "Missing 'policyDocument'");
            }
        };

        let mut state = state.write().await;
        let policy = state.put_resource_policy(policy_name, policy_document);
        wire::serialize_put_resource_policy_response(&wire::PutResourcePolicyResponse {
            resource_policy: Some(wire::ResourcePolicy {
                policy_name: Some(policy.policy_name.clone()),
                policy_document: Some(policy.policy_document.clone()),
                last_updated_time: Some(policy.last_updated_time),
                ..Default::default()
            }),
            ..Default::default()
        })
    }

    async fn handle_delete_resource_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_resource_policy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let policy_name = match input.policy_name.as_deref() {
            Some(n) if !n.is_empty() => n,
            _ => {
                return json_error_response(400, "ValidationException", "Missing 'policyName'");
            }
        };

        let mut state = state.write().await;
        match state.delete_resource_policy(policy_name) {
            Ok(()) => wire::serialize_delete_resource_policy_response(),
            Err(e) => logs_error_response(&e),
        }
    }

    async fn handle_describe_resource_policies(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let policies = state.describe_resource_policies();

        let entries: Vec<wire::ResourcePolicy> = policies
            .iter()
            .map(|p| wire::ResourcePolicy {
                policy_name: Some(p.policy_name.clone()),
                policy_document: Some(p.policy_document.clone()),
                last_updated_time: Some(p.last_updated_time),
                ..Default::default()
            })
            .collect();

        wire::serialize_describe_resource_policies_response(
            &wire::DescribeResourcePoliciesResponse {
                resource_policies: Some(entries),
                next_token: None,
            },
        )
    }

    // ========== Destinations ==========

    async fn handle_put_destination(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_put_destination_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.destination_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'destinationName'");
        }
        if input.target_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'targetArn'");
        }
        if input.role_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'roleArn'");
        }
        let tags = input.tags.unwrap_or_default();

        let mut state = state.write().await;
        let dest = state.put_destination(
            &input.destination_name,
            &input.target_arn,
            &input.role_arn,
            account_id,
            region,
            tags,
        );
        wire::serialize_put_destination_response(&wire::PutDestinationResponse {
            destination: Some(wire::Destination {
                destination_name: Some(dest.destination_name.clone()),
                target_arn: Some(dest.target_arn.clone()),
                role_arn: Some(dest.role_arn.clone()),
                arn: Some(dest.arn.clone()),
                creation_time: Some(dest.creation_time),
                access_policy: dest.access_policy.clone(),
            }),
        })
    }

    async fn handle_put_destination_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_put_destination_policy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.destination_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'destinationName'");
        }
        if input.access_policy.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'accessPolicy'");
        }

        let mut state = state.write().await;
        match state.put_destination_policy(&input.destination_name, &input.access_policy) {
            Ok(()) => wire::serialize_put_destination_policy_response(),
            Err(e) => logs_error_response(&e),
        }
    }

    async fn handle_delete_destination(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_destination_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.destination_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'destinationName'");
        }

        let mut state = state.write().await;
        match state.delete_destination(&input.destination_name) {
            Ok(()) => wire::serialize_delete_destination_response(),
            Err(e) => logs_error_response(&e),
        }
    }

    async fn handle_describe_destinations(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_destinations_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let prefix = input.destination_name_prefix.as_deref();

        let state = state.read().await;
        let destinations = state.describe_destinations(prefix);

        let entries: Vec<wire::Destination> = destinations
            .iter()
            .map(|d| wire::Destination {
                destination_name: Some(d.destination_name.clone()),
                target_arn: Some(d.target_arn.clone()),
                role_arn: Some(d.role_arn.clone()),
                arn: Some(d.arn.clone()),
                creation_time: Some(d.creation_time),
                access_policy: d.access_policy.clone(),
            })
            .collect();

        wire::serialize_describe_destinations_response(&wire::DescribeDestinationsResponse {
            destinations: Some(entries),
            next_token: None,
        })
    }

    // ========== Export Tasks ==========

    async fn handle_create_export_task(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_create_export_task_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.log_group_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'logGroupName'");
        }
        if input.destination.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'destination'");
        }

        let mut state = state.write().await;
        match state.create_export_task(
            input.task_name.as_deref(),
            &input.log_group_name,
            &input.destination,
            input.from,
            input.to,
        ) {
            Ok(task_id) => {
                wire::serialize_create_export_task_response(&wire::CreateExportTaskResponse {
                    task_id: Some(task_id),
                })
            }
            Err(e) => logs_error_response(&e),
        }
    }

    async fn handle_cancel_export_task(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_cancel_export_task_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.task_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'taskId'");
        }

        let mut state = state.write().await;
        match state.cancel_export_task(&input.task_id) {
            Ok(()) => wire::serialize_cancel_export_task_response(),
            Err(e) => logs_error_response(&e),
        }
    }

    async fn handle_describe_export_tasks(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_export_tasks_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let task_id = input.task_id.as_deref();

        let state = state.read().await;
        match state.describe_export_tasks(task_id) {
            Ok(tasks) => {
                let entries: Vec<wire::ExportTask> = tasks
                    .iter()
                    .map(|t| wire::ExportTask {
                        task_id: Some(t.task_id.clone()),
                        task_name: t.task_name.clone(),
                        log_group_name: Some(t.log_group_name.clone()),
                        destination: Some(t.destination.clone()),
                        from: Some(t.from_time),
                        to: Some(t.to_time),
                        status: Some(wire::ExportTaskStatus {
                            code: Some(t.status.clone()),
                            message: None,
                        }),
                        ..Default::default()
                    })
                    .collect();

                wire::serialize_describe_export_tasks_response(&wire::DescribeExportTasksResponse {
                    export_tasks: Some(entries),
                    next_token: None,
                })
            }
            Err(e) => logs_error_response(&e),
        }
    }

    // ========== Queries ==========

    async fn handle_start_query(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_start_query_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let log_group_name = match input.log_group_name.as_deref() {
            Some(n) if !n.is_empty() => n,
            _ => {
                return json_error_response(400, "ValidationException", "Missing 'logGroupName'");
            }
        };
        if input.query_string.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'queryString'");
        }

        let mut state = state.write().await;
        match state.start_query(
            log_group_name,
            &input.query_string,
            input.start_time,
            input.end_time,
        ) {
            Ok(query_id) => wire::serialize_start_query_response(&wire::StartQueryResponse {
                query_id: Some(query_id),
            }),
            Err(e) => logs_error_response(&e),
        }
    }

    async fn handle_get_query_results(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_query_results_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.query_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'queryId'");
        }

        let state = state.read().await;
        match state.get_query_results(&input.query_id) {
            Ok(query) => {
                wire::serialize_get_query_results_response(&wire::GetQueryResultsResponse {
                    status: Some(query.status.clone()),
                    results: Some(Vec::new()),
                    statistics: Some(wire::QueryStatistics {
                        records_matched: Some(0.0),
                        records_scanned: Some(0.0),
                        bytes_scanned: Some(0.0),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
            }
            Err(e) => logs_error_response(&e),
        }
    }

    async fn handle_describe_queries(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_queries_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let log_group_name = input.log_group_name.as_deref();

        let state = state.read().await;
        let queries = state.describe_queries(log_group_name);

        let entries: Vec<wire::QueryInfo> = queries
            .iter()
            .map(|q| wire::QueryInfo {
                query_id: Some(q.query_id.clone()),
                query_string: Some(q.query_string.clone()),
                status: Some(q.status.clone()),
                create_time: Some(q.create_time),
                log_group_name: Some(q.log_group_name.clone()),
                ..Default::default()
            })
            .collect();

        wire::serialize_describe_queries_response(&wire::DescribeQueriesResponse {
            queries: Some(entries),
            next_token: None,
        })
    }

    // ========== Tags (legacy log group) ==========

    async fn handle_tag_log_group(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_tag_log_group_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.log_group_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'logGroupName'");
        }

        let mut state = state.write().await;
        match state.tag_log_group(&input.log_group_name, input.tags) {
            Ok(()) => wire::serialize_tag_log_group_response(),
            Err(e) => logs_error_response(&e),
        }
    }

    async fn handle_untag_log_group(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_untag_log_group_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.log_group_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'logGroupName'");
        }

        let mut state = state.write().await;
        match state.untag_log_group(&input.log_group_name, &input.tags) {
            Ok(()) => wire::serialize_untag_log_group_response(),
            Err(e) => logs_error_response(&e),
        }
    }

    async fn handle_list_tags_log_group(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_log_group_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.log_group_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'logGroupName'");
        }

        let state = state.read().await;
        match state.list_tags_log_group(&input.log_group_name) {
            Ok(tags) => {
                wire::serialize_list_tags_log_group_response(&wire::ListTagsLogGroupResponse {
                    tags: Some(tags.clone()),
                })
            }
            Err(e) => logs_error_response(&e),
        }
    }

    // ========== Tags (ARN-based) ==========

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'resourceArn'");
        }

        let mut state = state.write().await;
        match state.tag_resource(&input.resource_arn, input.tags) {
            Ok(()) => wire::serialize_tag_resource_response(),
            Err(e) => logs_error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'resourceArn'");
        }

        let mut state = state.write().await;
        match state.untag_resource(&input.resource_arn, &input.tag_keys) {
            Ok(()) => wire::serialize_untag_resource_response(),
            Err(e) => logs_error_response(&e),
        }
    }

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'resourceArn'");
        }

        let state = state.read().await;
        match state.list_tags_for_resource(&input.resource_arn) {
            Ok(tags) => wire::serialize_list_tags_for_resource_response(
                &wire::ListTagsForResourceResponse {
                    tags: Some(tags.clone()),
                },
            ),
            Err(e) => logs_error_response(&e),
        }
    }

    // ========== Delivery Sources ==========

    async fn handle_put_delivery_source(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_put_delivery_source_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'name'");
        }
        if input.resource_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'resourceArn'");
        }
        let log_type = if input.log_type.is_empty() {
            None
        } else {
            Some(input.log_type)
        };

        let source = types::DeliverySource {
            name: input.name.clone(),
            log_type: log_type.clone(),
            service: None,
            resource_arns: vec![input.resource_arn.clone()],
        };

        let mut state = state.write().await;
        state.put_delivery_source(source);

        wire::serialize_put_delivery_source_response(&wire::PutDeliverySourceResponse {
            delivery_source: Some(wire::DeliverySource {
                name: Some(input.name),
                log_type,
                resource_arns: Some(vec![input.resource_arn]),
                ..Default::default()
            }),
        })
    }

    async fn handle_get_delivery_source(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_delivery_source_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'name'");
        }

        let state = state.read().await;
        match state.get_delivery_source(&input.name) {
            Ok(src) => {
                wire::serialize_get_delivery_source_response(&wire::GetDeliverySourceResponse {
                    delivery_source: Some(wire::DeliverySource {
                        name: Some(src.name.clone()),
                        log_type: src.log_type.clone(),
                        service: src.service.clone(),
                        resource_arns: Some(src.resource_arns.clone()),
                        ..Default::default()
                    }),
                })
            }
            Err(e) => logs_error_response(&e),
        }
    }

    async fn handle_delete_delivery_source(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_delivery_source_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'name'");
        }

        let mut state = state.write().await;
        match state.delete_delivery_source(&input.name) {
            Ok(()) => wire::serialize_delete_delivery_source_response(),
            Err(e) => logs_error_response(&e),
        }
    }

    async fn handle_describe_delivery_sources(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let sources = state.describe_delivery_sources();

        let entries: Vec<wire::DeliverySource> = sources
            .iter()
            .map(|s| wire::DeliverySource {
                name: Some(s.name.clone()),
                log_type: s.log_type.clone(),
                service: s.service.clone(),
                resource_arns: Some(s.resource_arns.clone()),
                ..Default::default()
            })
            .collect();

        wire::serialize_describe_delivery_sources_response(&wire::DescribeDeliverySourcesResponse {
            delivery_sources: Some(entries),
            next_token: None,
        })
    }

    // ========== Delivery Destinations ==========

    async fn handle_put_delivery_destination(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_put_delivery_destination_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'name'");
        }
        let output_format = input.output_format;

        let delivery_destination_configuration =
            input.delivery_destination_configuration.map(|c| {
                types::DeliveryDestinationConfiguration {
                    destination_resource_arn: c.destination_resource_arn,
                }
            });

        let dest = types::DeliveryDestination {
            name: input.name.clone(),
            arn: format!(
                "arn:aws:logs:us-east-1:123456789012:delivery-destination:{}",
                input.name
            ),
            delivery_destination_type: None,
            output_format: output_format.clone(),
            delivery_destination_configuration: delivery_destination_configuration.clone(),
        };

        let mut state = state.write().await;
        state.put_delivery_destination(dest);

        wire::serialize_put_delivery_destination_response(&wire::PutDeliveryDestinationResponse {
            delivery_destination: Some(wire::DeliveryDestination {
                name: Some(input.name),
                output_format,
                delivery_destination_configuration: delivery_destination_configuration.map(|c| {
                    wire::DeliveryDestinationConfiguration {
                        destination_resource_arn: c.destination_resource_arn,
                    }
                }),
                ..Default::default()
            }),
        })
    }

    async fn handle_get_delivery_destination(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_delivery_destination_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'name'");
        }

        let state = state.read().await;
        match state.get_delivery_destination(&input.name) {
            Ok(dest) => wire::serialize_get_delivery_destination_response(
                &wire::GetDeliveryDestinationResponse {
                    delivery_destination: Some(wire::DeliveryDestination {
                        name: Some(dest.name.clone()),
                        arn: Some(dest.arn.clone()),
                        output_format: dest.output_format.clone(),
                        delivery_destination_configuration: dest
                            .delivery_destination_configuration
                            .as_ref()
                            .map(|c| wire::DeliveryDestinationConfiguration {
                                destination_resource_arn: c.destination_resource_arn.clone(),
                            }),
                        ..Default::default()
                    }),
                },
            ),
            Err(e) => logs_error_response(&e),
        }
    }

    async fn handle_delete_delivery_destination(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_delivery_destination_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'name'");
        }

        let mut state = state.write().await;
        match state.delete_delivery_destination(&input.name) {
            Ok(()) => wire::serialize_delete_delivery_destination_response(),
            Err(e) => logs_error_response(&e),
        }
    }

    async fn handle_describe_delivery_destinations(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let destinations = state.describe_delivery_destinations();

        let entries: Vec<wire::DeliveryDestination> = destinations
            .iter()
            .map(|d| wire::DeliveryDestination {
                name: Some(d.name.clone()),
                arn: Some(d.arn.clone()),
                output_format: d.output_format.clone(),
                delivery_destination_configuration: d
                    .delivery_destination_configuration
                    .as_ref()
                    .map(|c| wire::DeliveryDestinationConfiguration {
                        destination_resource_arn: c.destination_resource_arn.clone(),
                    }),
                ..Default::default()
            })
            .collect();

        wire::serialize_describe_delivery_destinations_response(
            &wire::DescribeDeliveryDestinationsResponse {
                delivery_destinations: Some(entries),
                next_token: None,
            },
        )
    }

    // ========== Delivery Destination Policy ==========

    async fn handle_put_delivery_destination_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_put_delivery_destination_policy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.delivery_destination_name.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing 'deliveryDestinationName'",
            );
        }
        if input.delivery_destination_policy.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing 'deliveryDestinationPolicy'",
            );
        }

        let mut state = state.write().await;
        match state.put_delivery_destination_policy(
            &input.delivery_destination_name,
            &input.delivery_destination_policy,
        ) {
            Ok(()) => wire::serialize_put_delivery_destination_policy_response(
                &wire::PutDeliveryDestinationPolicyResponse {
                    policy: Some(wire::Policy {
                        delivery_destination_policy: Some(input.delivery_destination_policy),
                    }),
                },
            ),
            Err(e) => logs_error_response(&e),
        }
    }

    async fn handle_get_delivery_destination_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_delivery_destination_policy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.delivery_destination_name.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing 'deliveryDestinationName'",
            );
        }

        let state = state.read().await;
        match state.get_delivery_destination_policy(&input.delivery_destination_name) {
            Ok(p) => wire::serialize_get_delivery_destination_policy_response(
                &wire::GetDeliveryDestinationPolicyResponse {
                    policy: Some(wire::Policy {
                        delivery_destination_policy: Some(p.policy.clone()),
                    }),
                },
            ),
            Err(e) => logs_error_response(&e),
        }
    }

    async fn handle_delete_delivery_destination_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_delivery_destination_policy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.delivery_destination_name.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing 'deliveryDestinationName'",
            );
        }

        let mut state = state.write().await;
        match state.delete_delivery_destination_policy(&input.delivery_destination_name) {
            Ok(()) => wire::serialize_delete_delivery_destination_policy_response(),
            Err(e) => logs_error_response(&e),
        }
    }

    // ========== Deliveries ==========

    async fn handle_create_delivery(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_create_delivery_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.delivery_source_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'deliverySourceName'");
        }
        if input.delivery_destination_arn.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing 'deliveryDestinationArn'",
            );
        }
        let tags = input.tags.unwrap_or_default();

        // The destination ARN contains the destination name after the last colon
        let dest_name = input
            .delivery_destination_arn
            .rsplit(':')
            .next()
            .unwrap_or(&input.delivery_destination_arn)
            .to_string();

        let mut state = state.write().await;
        match state.create_delivery(&input.delivery_source_name, &dest_name, tags) {
            Ok(delivery) => {
                wire::serialize_create_delivery_response(&wire::CreateDeliveryResponse {
                    delivery: Some(wire::Delivery {
                        id: Some(delivery.id.clone()),
                        delivery_source_name: Some(delivery.source.clone()),
                        delivery_destination_arn: Some(input.delivery_destination_arn),
                        tags: if delivery.tags.is_empty() {
                            None
                        } else {
                            Some(delivery.tags.clone())
                        },
                        ..Default::default()
                    }),
                })
            }
            Err(e) => logs_error_response(&e),
        }
    }

    async fn handle_get_delivery(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_delivery_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'id'");
        }

        let state = state.read().await;
        match state.get_delivery(&input.id) {
            Ok(delivery) => wire::serialize_get_delivery_response(&wire::GetDeliveryResponse {
                delivery: Some(wire::Delivery {
                    id: Some(delivery.id.clone()),
                    delivery_source_name: Some(delivery.source.clone()),
                    tags: if delivery.tags.is_empty() {
                        None
                    } else {
                        Some(delivery.tags.clone())
                    },
                    ..Default::default()
                }),
            }),
            Err(e) => logs_error_response(&e),
        }
    }

    async fn handle_delete_delivery(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_delivery_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'id'");
        }

        let mut state = state.write().await;
        match state.delete_delivery(&input.id) {
            Ok(()) => wire::serialize_delete_delivery_response(),
            Err(e) => logs_error_response(&e),
        }
    }

    async fn handle_describe_deliveries(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let deliveries = state.describe_deliveries();

        let entries: Vec<wire::Delivery> = deliveries
            .iter()
            .map(|d| wire::Delivery {
                id: Some(d.id.clone()),
                delivery_source_name: Some(d.source.clone()),
                tags: if d.tags.is_empty() {
                    None
                } else {
                    Some(d.tags.clone())
                },
                ..Default::default()
            })
            .collect();

        wire::serialize_describe_deliveries_response(&wire::DescribeDeliveriesResponse {
            deliveries: Some(entries),
            next_token: None,
        })
    }

    // ========== UpdateDeliveryConfiguration ==========

    async fn handle_update_delivery_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_delivery_configuration_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'id'");
        }

        let state = state.read().await;
        if state.get_delivery(&input.id).is_err() {
            return json_error_response(
                400,
                "ResourceNotFoundException",
                "The specified delivery does not exist.",
            );
        }

        wire::serialize_update_delivery_configuration_response(
            &wire::UpdateDeliveryConfigurationResponse {},
        )
    }

    // ========== Account Policies ==========

    async fn handle_put_account_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_put_account_policy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.policy_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'policyName'");
        }
        if input.policy_document.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'policyDocument'");
        }
        if input.policy_type.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'policyType'");
        }
        let scope = input.scope;
        let selection_criteria = input.selection_criteria;

        let now = chrono::Utc::now().timestamp_millis();
        let policy = types::AccountPolicy {
            policy_name: input.policy_name.clone(),
            policy_document: input.policy_document.clone(),
            policy_type: input.policy_type.clone(),
            scope: scope.clone(),
            selection_criteria: selection_criteria.clone(),
            account_id: account_id.to_string(),
            last_updated_time: now,
        };

        let mut state = state.write().await;
        state.put_account_policy(policy);

        wire::serialize_put_account_policy_response(&wire::PutAccountPolicyResponse {
            account_policy: Some(wire::AccountPolicy {
                policy_name: Some(input.policy_name),
                policy_document: Some(input.policy_document),
                policy_type: Some(input.policy_type),
                scope,
                selection_criteria,
                account_id: Some(account_id.to_string()),
                last_updated_time: Some(now),
            }),
        })
    }

    async fn handle_delete_account_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_account_policy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.policy_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'policyName'");
        }
        if input.policy_type.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'policyType'");
        }

        let mut state = state.write().await;
        match state.delete_account_policy(&input.policy_name, &input.policy_type) {
            Ok(()) => wire::serialize_delete_account_policy_response(),
            Err(e) => logs_error_response(&e),
        }
    }

    async fn handle_describe_account_policies(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_account_policies_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.policy_type.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'policyType'");
        }
        let policy_name = input.policy_name.as_deref();

        let state = state.read().await;
        let policies = state.describe_account_policies(&input.policy_type, policy_name);

        let entries: Vec<wire::AccountPolicy> = policies
            .iter()
            .map(|p| wire::AccountPolicy {
                policy_name: Some(p.policy_name.clone()),
                policy_document: Some(p.policy_document.clone()),
                policy_type: Some(p.policy_type.clone()),
                scope: p.scope.clone(),
                selection_criteria: p.selection_criteria.clone(),
                account_id: Some(p.account_id.clone()),
                last_updated_time: Some(p.last_updated_time),
            })
            .collect();

        wire::serialize_describe_account_policies_response(&wire::DescribeAccountPoliciesResponse {
            account_policies: Some(entries),
            next_token: None,
        })
    }

    // ========== Data Protection Policies ==========

    async fn handle_put_data_protection_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_put_data_protection_policy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.log_group_identifier.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'logGroupIdentifier'");
        }
        if input.policy_document.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'policyDocument'");
        }

        let now = chrono::Utc::now().timestamp_millis();
        let mut state = state.write().await;
        match state.put_data_protection_policy(&input.log_group_identifier, &input.policy_document)
        {
            Ok(()) => wire::serialize_put_data_protection_policy_response(
                &wire::PutDataProtectionPolicyResponse {
                    log_group_identifier: Some(input.log_group_identifier),
                    policy_document: Some(input.policy_document),
                    last_updated_time: Some(now),
                },
            ),
            Err(e) => logs_error_response(&e),
        }
    }

    async fn handle_get_data_protection_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_data_protection_policy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.log_group_identifier.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'logGroupIdentifier'");
        }

        let now = chrono::Utc::now().timestamp_millis();
        let state = state.read().await;
        match state.get_data_protection_policy(&input.log_group_identifier) {
            Ok(doc) => wire::serialize_get_data_protection_policy_response(
                &wire::GetDataProtectionPolicyResponse {
                    log_group_identifier: Some(input.log_group_identifier),
                    policy_document: doc.map(|s| s.to_string()),
                    last_updated_time: Some(now),
                },
            ),
            Err(e) => logs_error_response(&e),
        }
    }

    async fn handle_delete_data_protection_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_data_protection_policy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.log_group_identifier.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'logGroupIdentifier'");
        }

        let mut state = state.write().await;
        match state.delete_data_protection_policy(&input.log_group_identifier) {
            Ok(()) => wire::serialize_delete_data_protection_policy_response(),
            Err(e) => logs_error_response(&e),
        }
    }

    // ========== Index Policies ==========

    async fn handle_put_index_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_put_index_policy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.log_group_identifier.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'logGroupIdentifier'");
        }
        if input.policy_document.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'policyDocument'");
        }

        let mut state = state.write().await;
        match state.put_index_policy(&input.log_group_identifier, &input.policy_document) {
            Ok(policy) => {
                wire::serialize_put_index_policy_response(&wire::PutIndexPolicyResponse {
                    index_policy: Some(wire::IndexPolicy {
                        log_group_identifier: Some(policy.log_group_identifier.clone()),
                        policy_document: Some(policy.policy_document.clone()),
                        policy_name: Some(policy.policy_name.clone()),
                        last_update_time: Some(policy.last_update_time),
                        source: None,
                    }),
                })
            }
            Err(e) => logs_error_response(&e),
        }
    }

    async fn handle_delete_index_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_index_policy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.log_group_identifier.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'logGroupIdentifier'");
        }

        let mut state = state.write().await;
        match state.delete_index_policy(&input.log_group_identifier) {
            Ok(()) => {
                wire::serialize_delete_index_policy_response(&wire::DeleteIndexPolicyResponse {})
            }
            Err(e) => logs_error_response(&e),
        }
    }

    // STUB[no-engine]: DescribeFieldIndexes returns field index metadata derived from
    //   real log ingestion; the mock has no field index engine; always returns empty list.
    async fn handle_describe_field_indexes(
        &self,
        _state: &Arc<tokio::sync::RwLock<LogsState>>,
    ) -> MockResponse {
        wire::serialize_describe_field_indexes_response(&wire::DescribeFieldIndexesResponse {
            field_indexes: Some(vec![]),
            next_token: None,
        })
    }

    async fn handle_describe_index_policies(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_index_policies_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let log_group_identifiers = input.log_group_identifiers;

        let state = state.read().await;
        let policies = state.describe_index_policies(&log_group_identifiers);

        let entries: Vec<wire::IndexPolicy> = policies
            .iter()
            .map(|p| wire::IndexPolicy {
                log_group_identifier: Some(p.log_group_identifier.clone()),
                policy_document: Some(p.policy_document.clone()),
                policy_name: Some(p.policy_name.clone()),
                last_update_time: Some(p.last_update_time),
                source: None,
            })
            .collect();

        wire::serialize_describe_index_policies_response(&wire::DescribeIndexPoliciesResponse {
            index_policies: Some(entries),
            next_token: None,
        })
    }

    // ========== Query Definitions ==========

    async fn handle_put_query_definition(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_put_query_definition_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'name'");
        }
        if input.query_string.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'queryString'");
        }
        let query_definition_id = input
            .query_definition_id
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let log_group_names = input.log_group_names.unwrap_or_default();

        let now = chrono::Utc::now().timestamp_millis();
        let qd = types::QueryDefinition {
            query_definition_id: query_definition_id.clone(),
            name: input.name,
            query_string: input.query_string,
            log_group_names,
            last_modified: now,
        };

        let mut state = state.write().await;
        let id = state.put_query_definition(qd);

        wire::serialize_put_query_definition_response(&wire::PutQueryDefinitionResponse {
            query_definition_id: Some(id),
        })
    }

    async fn handle_delete_query_definition(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_query_definition_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.query_definition_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'queryDefinitionId'");
        }

        let mut state = state.write().await;
        match state.delete_query_definition(&input.query_definition_id) {
            Ok(success) => wire::serialize_delete_query_definition_response(
                &wire::DeleteQueryDefinitionResponse {
                    success: Some(success),
                },
            ),
            Err(e) => logs_error_response(&e),
        }
    }

    async fn handle_describe_query_definitions(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_query_definitions_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let prefix = input.query_definition_name_prefix.as_deref();

        let state = state.read().await;
        let defs = state.describe_query_definitions(prefix);

        let entries: Vec<wire::QueryDefinition> = defs
            .iter()
            .map(|qd| wire::QueryDefinition {
                query_definition_id: Some(qd.query_definition_id.clone()),
                name: Some(qd.name.clone()),
                query_string: Some(qd.query_string.clone()),
                log_group_names: Some(qd.log_group_names.clone()),
                last_modified: Some(qd.last_modified),
                ..Default::default()
            })
            .collect();

        wire::serialize_describe_query_definitions_response(
            &wire::DescribeQueryDefinitionsResponse {
                query_definitions: Some(entries),
                next_token: None,
            },
        )
    }

    // ========== Log Anomaly Detectors ==========

    async fn handle_create_log_anomaly_detector(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_log_anomaly_detector_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let log_group_arn_list = input.log_group_arn_list;
        let detector_name = input.detector_name.unwrap_or_else(|| "default".to_string());
        let evaluation_frequency = input.evaluation_frequency;
        let filter_pattern = input.filter_pattern;
        let anomaly_visibility_time = input.anomaly_visibility_time;
        let kms_key_id = input.kms_key_id;

        let now = chrono::Utc::now().timestamp_millis();
        let id = uuid::Uuid::new_v4().to_string();
        let arn = format!("arn:aws:logs:{region}:{account_id}:anomaly-detector:{id}");

        let detector = types::LogAnomalyDetector {
            anomaly_detector_arn: arn.clone(),
            detector_name,
            log_group_arn_list,
            evaluation_frequency,
            filter_pattern,
            anomaly_visibility_time,
            status: "INITIALIZING".to_string(),
            creation_time: now,
            last_modified_time: now,
            kms_key_id,
        };

        let mut state = state.write().await;
        state.create_log_anomaly_detector(detector);

        wire::serialize_create_log_anomaly_detector_response(
            &wire::CreateLogAnomalyDetectorResponse {
                anomaly_detector_arn: Some(arn),
            },
        )
    }

    async fn handle_delete_log_anomaly_detector(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_log_anomaly_detector_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.anomaly_detector_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'anomalyDetectorArn'");
        }

        let mut state = state.write().await;
        match state.delete_log_anomaly_detector(&input.anomaly_detector_arn) {
            Ok(()) => wire::serialize_delete_log_anomaly_detector_response(),
            Err(e) => logs_error_response(&e),
        }
    }

    async fn handle_get_log_anomaly_detector(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_log_anomaly_detector_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.anomaly_detector_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'anomalyDetectorArn'");
        }

        let state = state.read().await;
        match state.get_log_anomaly_detector(&input.anomaly_detector_arn) {
            Ok(d) => wire::serialize_get_log_anomaly_detector_response(
                &wire::GetLogAnomalyDetectorResponse {
                    detector_name: Some(d.detector_name.clone()),
                    log_group_arn_list: Some(d.log_group_arn_list.clone()),
                    evaluation_frequency: d.evaluation_frequency.clone(),
                    filter_pattern: d.filter_pattern.clone(),
                    anomaly_visibility_time: d.anomaly_visibility_time,
                    anomaly_detector_status: Some(d.status.clone()),
                    creation_time_stamp: Some(d.creation_time),
                    last_modified_time_stamp: Some(d.last_modified_time),
                    kms_key_id: d.kms_key_id.clone(),
                },
            ),
            Err(e) => logs_error_response(&e),
        }
    }

    async fn handle_list_log_anomaly_detectors(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_log_anomaly_detectors_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let filter_log_group_arn = input.filter_log_group_arn.as_deref();

        let state = state.read().await;
        let detectors = state.list_log_anomaly_detectors(filter_log_group_arn);

        let entries: Vec<wire::AnomalyDetector> = detectors
            .iter()
            .map(|d| wire::AnomalyDetector {
                anomaly_detector_arn: Some(d.anomaly_detector_arn.clone()),
                detector_name: Some(d.detector_name.clone()),
                log_group_arn_list: Some(d.log_group_arn_list.clone()),
                evaluation_frequency: d.evaluation_frequency.clone(),
                filter_pattern: d.filter_pattern.clone(),
                anomaly_visibility_time: d.anomaly_visibility_time,
                anomaly_detector_status: Some(d.status.clone()),
                creation_time_stamp: Some(d.creation_time),
                last_modified_time_stamp: Some(d.last_modified_time),
                kms_key_id: d.kms_key_id.clone(),
            })
            .collect();

        wire::serialize_list_log_anomaly_detectors_response(
            &wire::ListLogAnomalyDetectorsResponse {
                anomaly_detectors: Some(entries),
                next_token: None,
            },
        )
    }

    async fn handle_update_log_anomaly_detector(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_log_anomaly_detector_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.anomaly_detector_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'anomalyDetectorArn'");
        }
        let evaluation_frequency = input.evaluation_frequency.as_deref();
        let filter_pattern = input.filter_pattern.as_deref();
        let anomaly_visibility_time = input.anomaly_visibility_time;
        let enabled = Some(input.enabled);

        let mut state = state.write().await;
        match state.update_log_anomaly_detector(
            &input.anomaly_detector_arn,
            evaluation_frequency,
            filter_pattern,
            anomaly_visibility_time,
            enabled,
        ) {
            Ok(()) => wire::serialize_update_log_anomaly_detector_response(),
            Err(e) => logs_error_response(&e),
        }
    }

    // ========== Anomalies ==========

    async fn handle_list_anomalies(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_anomalies_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let anomaly_detector_arn = input.anomaly_detector_arn.as_deref();

        let state = state.read().await;
        let anomalies = state.list_anomalies(anomaly_detector_arn);

        let entries: Vec<wire::Anomaly> = anomalies
            .iter()
            .map(|a| wire::Anomaly {
                anomaly_id: Some(a.anomaly_id.clone()),
                anomaly_detector_arn: Some(a.anomaly_detector_arn.clone()),
                description: Some(a.description.clone()),
                first_seen: Some(a.first_seen),
                last_seen: Some(a.last_seen),
                active: Some(!a.suppressed),
                ..Default::default()
            })
            .collect();

        wire::serialize_list_anomalies_response(&wire::ListAnomaliesResponse {
            anomalies: Some(entries),
            next_token: None,
        })
    }

    async fn handle_update_anomaly(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_anomaly_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.anomaly_detector_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'anomalyDetectorArn'");
        }
        let anomaly_id = input.anomaly_id.unwrap_or_default();
        let suppression_type = input.suppression_type.as_deref();
        let suppression_period_value = input
            .suppression_period
            .as_ref()
            .and_then(|p| p.value)
            .map(|v| v as i64);

        let mut state = state.write().await;
        match state.update_anomaly(
            &anomaly_id,
            &input.anomaly_detector_arn,
            suppression_type,
            suppression_period_value,
        ) {
            Ok(()) => wire::serialize_update_anomaly_response(),
            Err(e) => logs_error_response(&e),
        }
    }

    // ========== Integrations ==========

    async fn handle_put_integration(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_put_integration_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.integration_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'integrationName'");
        }
        let integration_type = if input.integration_type.is_empty() {
            "OPENSEARCH".to_string()
        } else {
            input.integration_type
        };
        let resource_config = serde_json::to_value(&input.resource_config).unwrap_or(Value::Null);

        let integration = types::Integration {
            integration_name: input.integration_name.clone(),
            integration_type: integration_type.clone(),
            resource_config,
            status: "ACTIVE".to_string(),
        };

        let mut state = state.write().await;
        let _saved = state.put_integration(integration);

        wire::serialize_put_integration_response(&wire::PutIntegrationResponse {
            integration_name: Some(input.integration_name),
            integration_status: Some("ACTIVE".to_string()),
        })
    }

    async fn handle_get_integration(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_integration_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.integration_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'integrationName'");
        }

        let state = state.read().await;
        match state.get_integration(&input.integration_name) {
            Ok(i) => wire::serialize_get_integration_response(&wire::GetIntegrationResponse {
                integration_name: Some(i.integration_name.clone()),
                integration_status: Some(i.status.clone()),
                integration_type: Some(i.integration_type.clone()),
                ..Default::default()
            }),
            Err(e) => logs_error_response(&e),
        }
    }

    async fn handle_delete_integration(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_integration_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.integration_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'integrationName'");
        }

        let mut state = state.write().await;
        match state.delete_integration(&input.integration_name) {
            Ok(()) => {
                wire::serialize_delete_integration_response(&wire::DeleteIntegrationResponse {})
            }
            Err(e) => logs_error_response(&e),
        }
    }

    async fn handle_list_integrations(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let integrations = state.list_integrations();

        let entries: Vec<wire::IntegrationSummary> = integrations
            .iter()
            .map(|i| wire::IntegrationSummary {
                integration_name: Some(i.integration_name.clone()),
                integration_status: Some(i.status.clone()),
                integration_type: Some(i.integration_type.clone()),
            })
            .collect();

        wire::serialize_list_integrations_response(&wire::ListIntegrationsResponse {
            integration_summaries: Some(entries),
        })
    }

    // ========== Scheduled Queries ==========

    async fn handle_create_scheduled_query(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_scheduled_query_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'name'");
        }
        if input.query_string.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'queryString'");
        }
        let schedule_expression = if input.schedule_expression.is_empty() {
            "cron(0 * * * ? *)".to_string()
        } else {
            input.schedule_expression
        };
        let log_group_name = input
            .log_group_identifiers
            .as_ref()
            .and_then(|v| v.first())
            .cloned()
            .unwrap_or_default();

        let now = chrono::Utc::now().timestamp_millis();
        let id = uuid::Uuid::new_v4().to_string();
        let arn = format!("arn:aws:logs:{region}:{account_id}:scheduled-query:{id}");

        let sq = types::ScheduledQuery {
            scheduled_query_id: arn.clone(),
            name: input.name,
            query_string: input.query_string,
            log_group_name,
            schedule_expression,
            status: "ENABLED".to_string(),
            creation_time: now,
        };

        let mut state = state.write().await;
        state.create_scheduled_query(sq);

        wire::serialize_create_scheduled_query_response(&wire::CreateScheduledQueryResponse {
            scheduled_query_arn: Some(arn),
            state: Some("ENABLED".to_string()),
        })
    }

    async fn handle_delete_scheduled_query(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_scheduled_query_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.identifier.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'identifier'");
        }

        let mut state = state.write().await;
        match state.delete_scheduled_query(&input.identifier) {
            Ok(()) => wire::serialize_delete_scheduled_query_response(
                &wire::DeleteScheduledQueryResponse {},
            ),
            Err(e) => logs_error_response(&e),
        }
    }

    async fn handle_get_scheduled_query(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_scheduled_query_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.identifier.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'identifier'");
        }

        let state = state.read().await;
        match state.get_scheduled_query(&input.identifier) {
            Ok(sq) => {
                wire::serialize_get_scheduled_query_response(&wire::GetScheduledQueryResponse {
                    scheduled_query_arn: Some(sq.scheduled_query_id.clone()),
                    name: Some(sq.name.clone()),
                    query_string: Some(sq.query_string.clone()),
                    schedule_expression: Some(sq.schedule_expression.clone()),
                    state: Some(sq.status.clone()),
                    creation_time: Some(sq.creation_time),
                    ..Default::default()
                })
            }
            Err(e) => logs_error_response(&e),
        }
    }

    async fn handle_list_scheduled_queries(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let queries = state.list_scheduled_queries();

        let entries: Vec<wire::ScheduledQuerySummary> = queries
            .iter()
            .map(|sq| wire::ScheduledQuerySummary {
                scheduled_query_arn: Some(sq.scheduled_query_id.clone()),
                name: Some(sq.name.clone()),
                schedule_expression: Some(sq.schedule_expression.clone()),
                state: Some(sq.status.clone()),
                creation_time: Some(sq.creation_time),
                ..Default::default()
            })
            .collect();

        wire::serialize_list_scheduled_queries_response(&wire::ListScheduledQueriesResponse {
            scheduled_queries: Some(entries),
            next_token: None,
        })
    }

    async fn handle_update_scheduled_query(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_scheduled_query_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.identifier.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'identifier'");
        }
        let enabled = input.state.as_deref().map(|s| s == "ENABLED");

        let mut state = state.write().await;
        match state.update_scheduled_query(&input.identifier, enabled) {
            Ok(()) => wire::serialize_update_scheduled_query_response(
                &wire::UpdateScheduledQueryResponse {
                    ..Default::default()
                },
            ),
            Err(e) => logs_error_response(&e),
        }
    }

    // STUB[no-engine]: GetScheduledQueryHistory requires a real execution history log;
    //   the mock does not record scheduled query run history; always returns empty trigger list.
    async fn handle_get_scheduled_query_history(&self, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_get_scheduled_query_history_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let scheduled_query_arn = if input.identifier.is_empty() {
            None
        } else {
            Some(input.identifier)
        };
        wire::serialize_get_scheduled_query_history_response(
            &wire::GetScheduledQueryHistoryResponse {
                scheduled_query_arn,
                trigger_history: Some(vec![]),
                next_token: None,
                name: None,
            },
        )
    }

    // ========== Transformers ==========

    async fn handle_put_transformer(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_put_transformer_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.log_group_identifier.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'logGroupIdentifier'");
        }
        let transformers: Vec<Value> = input
            .transformer_config
            .iter()
            .map(|p| serde_json::to_value(p).unwrap_or(Value::Null))
            .collect();

        let mut state = state.write().await;
        match state.put_transformer(&input.log_group_identifier, transformers) {
            Ok(()) => wire::serialize_put_transformer_response(),
            Err(e) => logs_error_response(&e),
        }
    }

    async fn handle_get_transformer(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_transformer_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.log_group_identifier.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'logGroupIdentifier'");
        }

        let state = state.read().await;
        match state.get_transformer(&input.log_group_identifier) {
            Ok(t) => wire::serialize_get_transformer_response(&wire::GetTransformerResponse {
                log_group_identifier: Some(t.log_group_identifier.clone()),
                creation_time: Some(t.creation_time),
                last_modified_time: Some(t.last_modified_time),
                transformer_config: None,
            }),
            Err(e) => logs_error_response(&e),
        }
    }

    async fn handle_delete_transformer(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_transformer_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.log_group_identifier.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'logGroupIdentifier'");
        }

        let mut state = state.write().await;
        match state.delete_transformer(&input.log_group_identifier) {
            Ok(()) => wire::serialize_delete_transformer_response(),
            Err(e) => logs_error_response(&e),
        }
    }

    async fn handle_test_transformer(&self, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_test_transformer_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let log_events: Vec<wire::TransformedLogRecord> = input
            .log_event_messages
            .iter()
            .enumerate()
            .map(|(i, msg)| wire::TransformedLogRecord {
                event_number: Some(i as i64),
                event_message: Some(msg.clone()),
                transformed_event_message: Some(msg.clone()),
            })
            .collect();

        wire::serialize_test_transformer_response(&wire::TestTransformerResponse {
            transformed_logs: Some(log_events),
        })
    }

    // ========== Import Tasks ==========

    async fn handle_create_import_task(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_create_import_task_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        // Smithy CreateImportTask uses importRoleArn / importSourceArn / importFilter.
        // We treat importSourceArn as the resource being imported and synthesise a log group
        // name from a sentinel since the Smithy model does not include logGroupName/from/to.
        let source = serde_json::to_value(&input).unwrap_or(Value::Null);
        let from_time = input
            .import_filter
            .as_ref()
            .and_then(|f| f.start_event_time)
            .unwrap_or(0);
        let to_time = input
            .import_filter
            .as_ref()
            .and_then(|f| f.end_event_time)
            .unwrap_or(0);
        let log_group_name = input.import_source_arn.clone();

        let now = chrono::Utc::now().timestamp_millis();
        let task_id = uuid::Uuid::new_v4().to_string();
        let destination_arn =
            format!("arn:aws:logs:us-east-1:123456789012:log-group:{log_group_name}");

        let task = types::ImportTask {
            task_id: task_id.clone(),
            task_name: None,
            log_group_name,
            from_time,
            to_time,
            source,
            status: "PENDING".to_string(),
            creation_time: now,
        };

        let mut state = state.write().await;
        state.create_import_task(task);

        wire::serialize_create_import_task_response(&wire::CreateImportTaskResponse {
            import_id: Some(task_id),
            creation_time: Some(now),
            import_destination_arn: Some(destination_arn),
        })
    }

    async fn handle_cancel_import_task(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_cancel_import_task_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.import_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'importId'");
        }

        let now = chrono::Utc::now().timestamp_millis();
        let mut state = state.write().await;
        match state.cancel_import_task(&input.import_id) {
            Ok(()) => {
                wire::serialize_cancel_import_task_response(&wire::CancelImportTaskResponse {
                    import_id: Some(input.import_id),
                    import_status: Some("CANCELLED".to_string()),
                    last_updated_time: Some(now),
                    creation_time: None,
                    import_statistics: None,
                })
            }
            Err(e) => logs_error_response(&e),
        }
    }

    async fn handle_describe_import_tasks(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_import_tasks_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let task_id = input.import_id.as_deref();

        let state = state.read().await;
        let tasks = state.describe_import_tasks(task_id);

        let entries: Vec<wire::Import> = tasks
            .iter()
            .map(|t| wire::Import {
                import_id: Some(t.task_id.clone()),
                import_status: Some(t.status.clone()),
                creation_time: Some(t.creation_time),
                import_destination_arn: Some(format!(
                    "arn:aws:logs:us-east-1:123456789012:log-group:{}",
                    t.log_group_name
                )),
                ..Default::default()
            })
            .collect();

        wire::serialize_describe_import_tasks_response(&wire::DescribeImportTasksResponse {
            imports: Some(entries),
            next_token: None,
        })
    }

    // STUB[no-engine]: DescribeImportTaskBatches requires real import batch tracking;
    //   the mock records import tasks but not their internal batch decomposition; always returns empty.
    async fn handle_describe_import_task_batches(&self, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_describe_import_task_batches_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let import_id = if input.import_id.is_empty() {
            None
        } else {
            Some(input.import_id)
        };
        wire::serialize_describe_import_task_batches_response(
            &wire::DescribeImportTaskBatchesResponse {
                import_id,
                import_batches: Some(vec![]),
                import_source_arn: None,
                next_token: None,
            },
        )
    }

    // ========== KMS ==========

    async fn handle_associate_kms_key(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_associate_kms_key_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let log_group_name = match input
            .log_group_name
            .as_deref()
            .or(input.resource_identifier.as_deref())
        {
            Some(n) if !n.is_empty() => n,
            _ => {
                return json_error_response(
                    400,
                    "ValidationException",
                    "Missing 'logGroupName' or 'resourceIdentifier'",
                );
            }
        };
        if input.kms_key_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'kmsKeyId'");
        }

        let mut state = state.write().await;
        match state.associate_kms_key(log_group_name, &input.kms_key_id) {
            Ok(()) => wire::serialize_associate_kms_key_response(),
            Err(e) => logs_error_response(&e),
        }
    }

    async fn handle_disassociate_kms_key(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_disassociate_kms_key_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let log_group_name = match input
            .log_group_name
            .as_deref()
            .or(input.resource_identifier.as_deref())
        {
            Some(n) if !n.is_empty() => n,
            _ => {
                return json_error_response(
                    400,
                    "ValidationException",
                    "Missing 'logGroupName' or 'resourceIdentifier'",
                );
            }
        };

        let mut state = state.write().await;
        match state.disassociate_kms_key(log_group_name) {
            Ok(()) => wire::serialize_disassociate_kms_key_response(),
            Err(e) => logs_error_response(&e),
        }
    }

    // ========== S3 Table Integration ==========

    // STUB[s3-integration]: S3 Table Integration requires real S3 bucket metadata and
    //   cross-service table associations that cannot exist in a single-account mock.
    async fn handle_associate_source_to_s3_table_integration(&self) -> MockResponse {
        let id = uuid::Uuid::new_v4().to_string();
        wire::serialize_associate_source_to_s3_table_integration_response(
            &wire::AssociateSourceToS3TableIntegrationResponse {
                identifier: Some(id),
            },
        )
    }

    // STUB[s3-integration]: S3 Table Integration requires real S3 bucket metadata and
    //   cross-service table associations that cannot exist in a single-account mock.
    async fn handle_disassociate_source_from_s3_table_integration(&self) -> MockResponse {
        wire::serialize_disassociate_source_from_s3_table_integration_response(
            &wire::DisassociateSourceFromS3TableIntegrationResponse { identifier: None },
        )
    }

    // STUB[s3-integration]: S3 Table Integration requires real S3 bucket metadata and
    //   cross-service table associations that cannot exist in a single-account mock.
    async fn handle_list_sources_for_s3_table_integration(&self) -> MockResponse {
        wire::serialize_list_sources_for_s3_table_integration_response(
            &wire::ListSourcesForS3TableIntegrationResponse {
                sources: Some(vec![]),
                next_token: None,
            },
        )
    }

    // ========== Log Query Helpers ==========

    // STUB[no-engine]: Log field discovery requires execution against real indexed log data;
    //   the mock stores raw events but has no field extraction or schema inference engine.
    async fn handle_get_log_group_fields(&self) -> MockResponse {
        wire::serialize_get_log_group_fields_response(&wire::GetLogGroupFieldsResponse {
            log_group_fields: Some(vec![]),
        })
    }

    // STUB[no-engine]: Log field discovery requires execution against real indexed log data;
    //   the mock has no field extraction engine.
    async fn handle_get_log_fields(&self) -> MockResponse {
        wire::serialize_get_log_fields_response(&wire::GetLogFieldsResponse {
            log_fields: Some(vec![]),
        })
    }

    // STUB[no-engine]: GetLogObject requires access to real indexed CloudWatch log storage;
    //   the mock has no object-retrieval engine.
    async fn handle_get_log_object(&self) -> MockResponse {
        wire::serialize_get_log_object_response(&wire::GetLogObjectResponse { field_stream: None })
    }

    // STUB[no-engine]: GetLogRecord requires access to real indexed CloudWatch Logs Insights data;
    //   the mock stores raw events but has no record-lookup engine.
    async fn handle_get_log_record(&self) -> MockResponse {
        wire::serialize_get_log_record_response(&wire::GetLogRecordResponse {
            log_record: Some(HashMap::new()),
        })
    }

    async fn handle_stop_query(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_stop_query_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.query_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'queryId'");
        }

        let mut state = state.write().await;
        match state.stop_query(&input.query_id) {
            Ok(success) => wire::serialize_stop_query_response(&wire::StopQueryResponse {
                success: Some(success),
            }),
            Err(e) => logs_error_response(&e),
        }
    }

    // STUB[no-engine]: TestMetricFilter requires a real metric filter evaluation engine
    //   to match log events against a filter pattern; always returns empty matches.
    async fn handle_test_metric_filter(&self) -> MockResponse {
        wire::serialize_test_metric_filter_response(&wire::TestMetricFilterResponse {
            matches: Some(vec![]),
        })
    }

    // ========== Other ==========

    async fn handle_describe_configuration_templates(&self) -> MockResponse {
        wire::serialize_describe_configuration_templates_response(
            &wire::DescribeConfigurationTemplatesResponse {
                configuration_templates: Some(vec![]),
                next_token: None,
            },
        )
    }

    // STUB[no-telemetry]: Aggregate log group summaries are driven by real cross-account
    //   telemetry data that the mock does not have; always returns empty list.
    async fn handle_list_aggregate_log_group_summaries(&self) -> MockResponse {
        wire::serialize_list_aggregate_log_group_summaries_response(
            &wire::ListAggregateLogGroupSummariesResponse {
                aggregate_log_group_summaries: Some(vec![]),
                next_token: None,
            },
        )
    }

    async fn handle_list_log_groups(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_log_groups_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let pattern = input.log_group_name_pattern.as_deref();

        let state = state.read().await;
        let mut groups = state.describe_log_groups(None);
        groups.sort_by(|a, b| a.name.cmp(&b.name));

        if let Some(pat) = pattern {
            groups.retain(|g| g.name.contains(pat));
        }

        let entries: Vec<wire::LogGroupSummary> = groups
            .iter()
            .map(|g| {
                let log_group_arn = g.arn.strip_suffix(":*").unwrap_or(&g.arn).to_string();
                wire::LogGroupSummary {
                    log_group_name: Some(g.name.clone()),
                    log_group_arn: Some(log_group_arn),
                    log_group_class: None,
                }
            })
            .collect();

        wire::serialize_list_log_groups_response(&wire::ListLogGroupsResponse {
            log_groups: Some(entries),
            next_token: None,
        })
    }

    // STUB[no-engine]: ListLogGroupsForQuery requires tracking which log groups were
    //   scanned by a completed Insights query; the mock query engine does not record this.
    async fn handle_list_log_groups_for_query(&self) -> MockResponse {
        wire::serialize_list_log_groups_for_query_response(&wire::ListLogGroupsForQueryResponse {
            log_group_identifiers: Some(vec![]),
            next_token: None,
        })
    }

    // STUB[no-auth]: The mock has no authentication layer; PutBearerTokenAuthentication
    //   is a no-op that returns a void success response.
    async fn handle_put_bearer_token_authentication(&self) -> MockResponse {
        wire::serialize_put_bearer_token_authentication_response()
    }

    async fn handle_put_log_group_deletion_protection(
        &self,
        state: &Arc<tokio::sync::RwLock<LogsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_put_log_group_deletion_protection_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.log_group_identifier.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'logGroupName'");
        }

        let mut state = state.write().await;
        match state.put_log_group_deletion_protection(
            &input.log_group_identifier,
            input.deletion_protection_enabled,
        ) {
            Ok(()) => wire::serialize_put_log_group_deletion_protection_response(),
            Err(e) => logs_error_response(&e),
        }
    }

    // STUB[no-engine]: StartLiveTail requires a real streaming connection and live event
    //   forwarding; returns a minimal session-start response without actual event streaming.
    async fn handle_start_live_tail(&self) -> MockResponse {
        let session_id = uuid::Uuid::new_v4().to_string();
        wire::serialize_start_live_tail_response(&wire::StartLiveTailResponse {
            response_stream: Some(wire::StartLiveTailResponseStream {
                session_start: Some(wire::LiveTailSessionStart {
                    session_id: Some(session_id),
                    ..Default::default()
                }),
                ..Default::default()
            }),
        })
    }
}

fn logs_error_response(err: &LogsError) -> MockResponse {
    let (status, error_type) = match err {
        LogsError::ResourceNotFound(_) => (400u16, "ResourceNotFoundException"),
        LogsError::AlreadyExists(_) => (400u16, "ResourceAlreadyExistsException"),
        LogsError::InvalidParameter(_) => (400u16, "InvalidParameterException"),
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
