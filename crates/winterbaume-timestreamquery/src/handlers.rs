use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, StatefulService,
    default_account_id,
};

use crate::model;
use crate::state::{TimestreamQueryError, TimestreamQueryState};
use crate::views::TimestreamQueryStateView;
use crate::wire;

/// Timestream Query service handler that processes awsJson1.0 protocol requests.
pub struct TimestreamQueryService {
    pub(crate) state: Arc<BackendState<TimestreamQueryState>>,
    pub(crate) notifier: StateChangeNotifier<TimestreamQueryStateView>,
}

impl TimestreamQueryService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for TimestreamQueryService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for TimestreamQueryService {
    fn service_name(&self) -> &str {
        "timestreamquery"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://query\.timestream\.(.+)\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl TimestreamQueryService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();

        // Extract action from X-Amz-Target header
        // Format: "Timestream_20181101.Query"
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

        // Validate that body parses as JSON; typed deserialisers re-parse per operation.
        if serde_json::from_slice::<Value>(&request.body).is_err() {
            return json_error_response(400, "SerializationException", "Invalid JSON body");
        }
        let body_bytes: &[u8] = &request.body;

        let state = self.state.get(account_id, &region);

        let response = match action.as_str() {
            "Query" => self.handle_query(&state, body_bytes).await,
            "DescribeEndpoints" => self.handle_describe_endpoints(&region).await,
            "CreateScheduledQuery" => {
                self.handle_create_scheduled_query(&state, body_bytes, account_id, &region)
                    .await
            }
            "ListScheduledQueries" => self.handle_list_scheduled_queries(&state).await,
            "DescribeScheduledQuery" => {
                self.handle_describe_scheduled_query(&state, body_bytes)
                    .await
            }
            "DeleteScheduledQuery" => self.handle_delete_scheduled_query(&state, body_bytes).await,
            "UpdateScheduledQuery" => self.handle_update_scheduled_query(&state, body_bytes).await,
            "CancelQuery" => self.handle_cancel_query(&state, body_bytes).await,
            "DescribeAccountSettings" => self.handle_describe_account_settings(&state).await,
            "UpdateAccountSettings" => {
                self.handle_update_account_settings(&state, body_bytes)
                    .await
            }
            "ExecuteScheduledQuery" => {
                self.handle_execute_scheduled_query(&state, body_bytes)
                    .await
            }
            "PrepareQuery" => self.handle_prepare_query(&state, body_bytes).await,
            "ListTagsForResource" => self.handle_list_tags_for_resource(&state, body_bytes).await,
            "TagResource" => self.handle_tag_resource(&state, body_bytes).await,
            "UntagResource" => self.handle_untag_resource(&state, body_bytes).await,
            _ => json_error_response(
                400,
                "InvalidAction",
                &format!("Could not find operation {action} for Timestream Query"),
            ),
        };
        if response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_query(
        &self,
        state: &Arc<tokio::sync::RwLock<TimestreamQueryState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_query_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.query_string.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'QueryString'");
        }
        let query_string = input.query_string.as_str();

        let state = state.read().await;
        match state.query(query_string) {
            Ok((columns, rows, query_id)) => {
                let response = model::QueryResponse {
                    query_id: Some(query_id),
                    column_info: Some(columns.iter().map(column_info_to_wire).collect()),
                    rows: Some(rows.iter().map(row_to_wire).collect()),
                    ..Default::default()
                };
                wire::serialize_query_response(&response)
            }
            Err(e) => tsq_error_response(&e),
        }
    }

    async fn handle_describe_endpoints(&self, region: &str) -> MockResponse {
        let state_lock = self.state.get(default_account_id(), region);
        let state = state_lock.read().await;
        let endpoints = state.describe_endpoints(region);

        let response = model::DescribeEndpointsResponse {
            endpoints: Some(
                endpoints
                    .iter()
                    .map(|(address, cache_period)| model::Endpoint {
                        address: Some(address.clone()),
                        cache_period_in_minutes: Some(*cache_period),
                    })
                    .collect(),
            ),
        };
        wire::serialize_describe_endpoints_response(&response)
    }

    async fn handle_create_scheduled_query(
        &self,
        state: &Arc<tokio::sync::RwLock<TimestreamQueryState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_scheduled_query_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Name'");
        }
        if input.query_string.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'QueryString'");
        }
        let name = input.name.as_str();
        let query_string = input.query_string.as_str();
        let schedule_expression = if input.schedule_configuration.schedule_expression.is_empty() {
            "rate(1 hour)"
        } else {
            input.schedule_configuration.schedule_expression.as_str()
        };
        let notification_topic_arn = input
            .notification_configuration
            .sns_configuration
            .topic_arn
            .as_str();
        let (target_database, target_table) = match &input.target_configuration {
            Some(tc) => (
                tc.timestream_configuration.database_name.as_str(),
                tc.timestream_configuration.table_name.as_str(),
            ),
            None => ("", ""),
        };
        let role_arn = input.scheduled_query_execution_role_arn.as_str();
        let s3_bucket_owned = input
            .error_report_configuration
            .s3_configuration
            .bucket_name
            .clone();
        let s3_bucket = if s3_bucket_owned.is_empty() {
            None
        } else {
            Some(s3_bucket_owned.as_str())
        };

        let mut state = state.write().await;
        match state.create_scheduled_query(
            account_id,
            region,
            name,
            query_string,
            schedule_expression,
            notification_topic_arn,
            target_database,
            target_table,
            role_arn,
            s3_bucket,
        ) {
            Ok(sq) => {
                let response = model::CreateScheduledQueryResponse {
                    arn: Some(sq.arn.clone()),
                };
                wire::serialize_create_scheduled_query_response(&response)
            }
            Err(e) => tsq_error_response(&e),
        }
    }

    async fn handle_list_scheduled_queries(
        &self,
        state: &Arc<tokio::sync::RwLock<TimestreamQueryState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let queries = state.list_scheduled_queries();

        let response = model::ListScheduledQueriesResponse {
            scheduled_queries: Some(
                queries
                    .iter()
                    .map(|sq| scheduled_query_to_wire(sq))
                    .collect(),
            ),
            next_token: None,
        };
        wire::serialize_list_scheduled_queries_response(&response)
    }

    async fn handle_describe_scheduled_query(
        &self,
        state: &Arc<tokio::sync::RwLock<TimestreamQueryState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_scheduled_query_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.scheduled_query_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ScheduledQueryArn'");
        }
        let arn = input.scheduled_query_arn.as_str();

        let state = state.read().await;
        match state.describe_scheduled_query(arn) {
            Ok(sq) => {
                let response = model::DescribeScheduledQueryResponse {
                    scheduled_query: Some(scheduled_query_description_to_wire(sq)),
                };
                wire::serialize_describe_scheduled_query_response(&response)
            }
            Err(e) => tsq_error_response(&e),
        }
    }

    async fn handle_delete_scheduled_query(
        &self,
        state: &Arc<tokio::sync::RwLock<TimestreamQueryState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_scheduled_query_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.scheduled_query_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ScheduledQueryArn'");
        }
        let arn = input.scheduled_query_arn.as_str();

        let mut state = state.write().await;
        match state.delete_scheduled_query(arn) {
            Ok(()) => wire::serialize_delete_scheduled_query_response(),
            Err(e) => tsq_error_response(&e),
        }
    }
    async fn handle_update_scheduled_query(
        &self,
        state: &Arc<tokio::sync::RwLock<TimestreamQueryState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_scheduled_query_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.scheduled_query_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ScheduledQueryArn'");
        }
        if input.state.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'State'");
        }
        let arn = input.scheduled_query_arn.as_str();
        let state_str = input.state.as_str();

        let new_state = match crate::types::ScheduledQueryState::from_str(state_str) {
            Some(s) => s,
            None => {
                return json_error_response(
                    400,
                    "ValidationException",
                    &format!("Invalid state '{state_str}'. Must be ENABLED or DISABLED"),
                );
            }
        };

        let mut state = state.write().await;
        match state.update_scheduled_query(arn, new_state) {
            Ok(()) => wire::serialize_update_scheduled_query_response(),
            Err(e) => tsq_error_response(&e),
        }
    }

    async fn handle_cancel_query(
        &self,
        state: &Arc<tokio::sync::RwLock<TimestreamQueryState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_cancel_query_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.query_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'QueryId'");
        }
        let query_id = input.query_id.as_str();

        let state = state.read().await;
        match state.cancel_query(query_id) {
            Ok(msg) => {
                let response = model::CancelQueryResponse {
                    cancellation_message: Some(msg),
                };
                wire::serialize_cancel_query_response(&response)
            }
            Err(e) => tsq_error_response(&e),
        }
    }

    async fn handle_describe_account_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<TimestreamQueryState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let settings = state.describe_account_settings();
        let response = model::DescribeAccountSettingsResponse {
            max_query_t_c_u: Some(settings.max_query_tcu),
            query_pricing_model: Some(settings.query_pricing_model.clone()),
            query_compute: None,
        };
        wire::serialize_describe_account_settings_response(&response)
    }

    async fn handle_update_account_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<TimestreamQueryState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_account_settings_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let max_query_tcu = input.max_query_t_c_u;
        let query_pricing_model = input.query_pricing_model.as_deref();

        let mut state = state.write().await;
        let settings = state.update_account_settings(max_query_tcu, query_pricing_model);
        let response = model::UpdateAccountSettingsResponse {
            max_query_t_c_u: Some(settings.max_query_tcu),
            query_pricing_model: Some(settings.query_pricing_model.clone()),
            query_compute: None,
        };
        wire::serialize_update_account_settings_response(&response)
    }

    async fn handle_execute_scheduled_query(
        &self,
        state: &Arc<tokio::sync::RwLock<TimestreamQueryState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_execute_scheduled_query_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.scheduled_query_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ScheduledQueryArn'");
        }
        let arn = input.scheduled_query_arn.as_str();

        let state = state.read().await;
        match state.execute_scheduled_query(arn) {
            Ok(()) => wire::serialize_execute_scheduled_query_response(),
            Err(e) => tsq_error_response(&e),
        }
    }

    async fn handle_prepare_query(
        &self,
        _state: &Arc<tokio::sync::RwLock<TimestreamQueryState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_prepare_query_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.query_string.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'QueryString'");
        }

        let response = model::PrepareQueryResponse {
            query_string: Some(input.query_string),
            columns: Some(vec![]),
            parameters: Some(vec![]),
        };
        wire::serialize_prepare_query_response(&response)
    }

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<TimestreamQueryState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_a_r_n.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ResourceARN'");
        }
        let resource_arn = input.resource_a_r_n.as_str();

        let state = state.read().await;
        let tags_map = state.list_tags_for_resource(resource_arn);
        let mut tags: Vec<model::Tag> = tags_map
            .into_iter()
            .map(|(k, v)| model::Tag { key: k, value: v })
            .collect();
        tags.sort_by(|a, b| a.key.cmp(&b.key));

        let response = model::ListTagsForResourceResponse {
            tags: Some(tags),
            next_token: None,
        };
        wire::serialize_list_tags_for_resource_response(&response)
    }

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<TimestreamQueryState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_a_r_n.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ResourceARN'");
        }
        let resource_arn = input.resource_a_r_n.as_str();

        let mut tags = std::collections::HashMap::new();
        for tag in input.tags {
            if !tag.key.is_empty() {
                tags.insert(tag.key, tag.value);
            }
        }

        let mut state = state.write().await;
        state.tag_resource(resource_arn, tags);
        let response = model::TagResourceResponse {};
        wire::serialize_tag_resource_response(&response)
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<TimestreamQueryState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_a_r_n.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ResourceARN'");
        }
        let resource_arn = input.resource_a_r_n.as_str();
        let tag_keys = input.tag_keys;

        let mut state = state.write().await;
        state.untag_resource(resource_arn, &tag_keys);
        let response = model::UntagResourceResponse {};
        wire::serialize_untag_resource_response(&response)
    }
}

// --- Conversion helpers: state types -> wire model types ---

fn column_info_to_wire(c: &crate::types::ColumnInfo) -> model::ColumnInfo {
    model::ColumnInfo {
        name: Some(c.name.clone()),
        r#type: Some(Box::new(model::Type {
            scalar_type: Some(c.column_type.clone()),
            ..Default::default()
        })),
    }
}

fn datum_to_wire(d: &crate::types::Datum) -> model::Datum {
    if let Some(ref sv) = d.scalar_value {
        model::Datum {
            scalar_value: Some(sv.clone()),
            ..Default::default()
        }
    } else {
        model::Datum {
            null_value: Some(true),
            ..Default::default()
        }
    }
}

fn row_to_wire(r: &crate::types::Row) -> model::Row {
    model::Row {
        data: Some(r.data.iter().map(datum_to_wire).collect()),
    }
}

fn scheduled_query_to_wire(sq: &crate::types::ScheduledQuery) -> model::ScheduledQuery {
    model::ScheduledQuery {
        arn: Some(sq.arn.clone()),
        name: Some(sq.name.clone()),
        state: Some(sq.state.as_str().to_string()),
        creation_time: Some(sq.creation_time.timestamp() as f64),
        target_destination: Some(model::TargetDestination {
            timestream_destination: Some(model::TimestreamDestination {
                database_name: Some(sq.target_database.clone()),
                table_name: Some(sq.target_table.clone()),
            }),
        }),
        ..Default::default()
    }
}

fn scheduled_query_description_to_wire(
    sq: &crate::types::ScheduledQuery,
) -> model::ScheduledQueryDescription {
    model::ScheduledQueryDescription {
        arn: Some(sq.arn.clone()),
        name: Some(sq.name.clone()),
        query_string: Some(sq.query_string.clone()),
        state: Some(sq.state.as_str().to_string()),
        schedule_configuration: Some(model::ScheduleConfiguration {
            schedule_expression: sq.schedule_expression.clone(),
        }),
        notification_configuration: Some(model::NotificationConfiguration {
            sns_configuration: model::SnsConfiguration {
                topic_arn: sq.notification_topic_arn.clone(),
            },
        }),
        target_configuration: Some(model::TargetConfiguration {
            timestream_configuration: model::TimestreamConfiguration {
                database_name: sq.target_database.clone(),
                table_name: sq.target_table.clone(),
                ..Default::default()
            },
        }),
        scheduled_query_execution_role_arn: Some(sq.role_arn.clone()),
        creation_time: Some(sq.creation_time.timestamp() as f64),
        ..Default::default()
    }
}

// --- Utility functions ---

fn tsq_error_response(err: &TimestreamQueryError) -> MockResponse {
    let (status, error_type) = match err {
        TimestreamQueryError::EmptyQueryString => (400, "ValidationException"),
        TimestreamQueryError::ScheduledQueryAlreadyExists { .. } => (409, "ConflictException"),
        TimestreamQueryError::ScheduledQueryNotFound { .. } => (400, "ResourceNotFoundException"),
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
