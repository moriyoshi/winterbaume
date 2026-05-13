use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::json;
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, StatefulService,
    default_account_id,
};

use crate::state::{SchedulerError, SchedulerState};
use crate::types::*;
use crate::views::SchedulerStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct SchedulerService {
    pub(crate) state: Arc<BackendState<SchedulerState>>,
    pub(crate) notifier: StateChangeNotifier<SchedulerStateView>,
}

impl SchedulerService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for SchedulerService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for SchedulerService {
    fn service_name(&self) -> &str {
        "scheduler"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://scheduler\..*\.amazonaws\.com",
            r"https?://scheduler\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl SchedulerService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();
        let state = self.state.get(account_id, &region);

        let (path, query_string) = extract_path_and_query(&request.uri);
        let method = request.method.as_str();

        let segments: Vec<&str> = path
            .trim_start_matches('/')
            .split('/')
            .filter(|s| !s.is_empty())
            .collect();

        let query_map: HashMap<String, String> = parse_query_string(&query_string);

        // Routes:
        // POST   /schedules/{Name}               - CreateSchedule
        // GET    /schedules/{Name}?groupName=...  - GetSchedule
        // PUT    /schedules/{Name}                - UpdateSchedule
        // DELETE /schedules/{Name}?groupName=...  - DeleteSchedule
        // GET    /schedules                       - ListSchedules
        // POST   /schedule-groups/{Name}          - CreateScheduleGroup
        // GET    /schedule-groups/{Name}          - GetScheduleGroup
        // DELETE /schedule-groups/{Name}          - DeleteScheduleGroup
        // GET    /schedule-groups                 - ListScheduleGroups
        // GET    /tags/{ResourceArn+}             - ListTagsForResource
        // POST   /tags/{ResourceArn+}             - TagResource
        // DELETE /tags/{ResourceArn+}             - UntagResource

        if segments.is_empty() {
            return rest_json_error(404, "UnknownOperationException", "Not found");
        }

        let response = match segments[0] {
            "schedules" => match (method, segments.len()) {
                ("GET", 1) => {
                    self.handle_list_schedules(&state, account_id, &region, &request, &query_map)
                        .await
                }
                ("POST", 2) => {
                    let name_decoded = urlencoding::decode(segments[1]).unwrap_or_default();
                    let labels: &[(&str, &str)] = &[("Name", &name_decoded)];
                    self.handle_create_schedule(
                        &state, &request, labels, &query_map, account_id, &region,
                    )
                    .await
                }
                ("GET", 2) => {
                    let name_decoded = urlencoding::decode(segments[1]).unwrap_or_default();
                    let labels: &[(&str, &str)] = &[("Name", &name_decoded)];
                    self.handle_get_schedule(
                        &state, &request, labels, &query_map, account_id, &region,
                    )
                    .await
                }
                ("PUT", 2) => {
                    let name_decoded = urlencoding::decode(segments[1]).unwrap_or_default();
                    let labels: &[(&str, &str)] = &[("Name", &name_decoded)];
                    self.handle_update_schedule(
                        &state, &request, labels, &query_map, account_id, &region,
                    )
                    .await
                }
                ("DELETE", 2) => {
                    let name_decoded = urlencoding::decode(segments[1]).unwrap_or_default();
                    let labels: &[(&str, &str)] = &[("Name", &name_decoded)];
                    self.handle_delete_schedule(
                        &state, &request, labels, &query_map, account_id, &region,
                    )
                    .await
                }
                _ => rest_json_error(404, "UnknownOperationException", "Not found"),
            },
            "schedule-groups" => match (method, segments.len()) {
                ("GET", 1) => {
                    self.handle_list_schedule_groups(
                        &state, account_id, &region, &request, &query_map,
                    )
                    .await
                }
                ("POST", 2) => {
                    let name_decoded = urlencoding::decode(segments[1]).unwrap_or_default();
                    let labels: &[(&str, &str)] = &[("Name", &name_decoded)];
                    self.handle_create_schedule_group(
                        &state, &request, labels, &query_map, account_id, &region,
                    )
                    .await
                }
                ("GET", 2) => {
                    let name_decoded = urlencoding::decode(segments[1]).unwrap_or_default();
                    let labels: &[(&str, &str)] = &[("Name", &name_decoded)];
                    self.handle_get_schedule_group(
                        &state, &request, labels, &query_map, account_id, &region,
                    )
                    .await
                }
                ("DELETE", 2) => {
                    let name_decoded = urlencoding::decode(segments[1]).unwrap_or_default();
                    let labels: &[(&str, &str)] = &[("Name", &name_decoded)];
                    self.handle_delete_schedule_group(&state, &request, labels, &query_map)
                        .await
                }
                _ => rest_json_error(404, "UnknownOperationException", "Not found"),
            },
            "tags" => {
                // The resource ARN is everything after /tags/
                let resource_arn = segments[1..].join("/");
                let resource_arn = urlencoding::decode(&resource_arn)
                    .unwrap_or_default()
                    .to_string();
                let labels: &[(&str, &str)] = &[("ResourceArn", resource_arn.as_str())];
                match method {
                    "GET" => {
                        self.handle_list_tags_for_resource(&state, &request, labels, &query_map)
                            .await
                    }
                    "POST" => {
                        self.handle_tag_resource(&state, &request, labels, &query_map)
                            .await
                    }
                    "DELETE" => {
                        self.handle_untag_resource(&state, &request, labels, &query_string)
                            .await
                    }
                    _ => rest_json_error(404, "UnknownOperationException", "Not found"),
                }
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        };

        if response.status >= 200 && response.status < 300 {
            self.notify_state_changed(account_id, &region).await;
        }

        response
    }

    async fn handle_create_schedule(
        &self,
        state: &Arc<tokio::sync::RwLock<SchedulerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_schedule_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "SerializationException", &e),
        };

        if input.schedule_expression.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing ScheduleExpression");
        }
        if input.flexible_time_window.mode.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing FlexibleTimeWindow");
        }

        let group_name = input.group_name.as_deref().unwrap_or("default");
        let ftw = FlexibleTimeWindow {
            mode: input.flexible_time_window.mode.clone(),
            maximum_window_in_minutes: input
                .flexible_time_window
                .maximum_window_in_minutes
                .map(|v| v as i64),
        };
        let target = ScheduleTarget {
            arn: input.target.arn.clone(),
            role_arn: input.target.role_arn.clone(),
            retry_policy: RetryPolicy::default(),
        };
        let start_date = input.start_date.map(|v| v.to_string());
        let end_date = input.end_date.map(|v| v.to_string());

        let mut state = state.write().await;
        match state.create_schedule(
            &input.name,
            group_name,
            account_id,
            region,
            &input.schedule_expression,
            ftw,
            target,
            input.state.as_deref(),
            input.description.as_deref(),
            input.action_after_completion.as_deref(),
            start_date.as_deref(),
            end_date.as_deref(),
        ) {
            Ok(arn) => wire::serialize_create_schedule_response(&wire::CreateScheduleOutput {
                schedule_arn: Some(arn),
            }),
            Err(e) => scheduler_error_response(&e),
        }
    }

    async fn handle_get_schedule(
        &self,
        state: &Arc<tokio::sync::RwLock<SchedulerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_get_schedule_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "SerializationException", &e),
        };
        let group_name = input.group_name.as_deref().unwrap_or("default");

        let mut state = state.write().await;
        match state.get_schedule(&input.name, group_name, account_id, region) {
            Ok(schedule) => {
                wire::serialize_get_schedule_response(&schedule_to_get_schedule_output(schedule))
            }
            Err(e) => scheduler_error_response(&e),
        }
    }

    async fn handle_update_schedule(
        &self,
        state: &Arc<tokio::sync::RwLock<SchedulerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_update_schedule_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "SerializationException", &e),
        };

        if input.schedule_expression.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing ScheduleExpression");
        }
        if input.flexible_time_window.mode.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing FlexibleTimeWindow");
        }

        let group_name = input.group_name.as_deref().unwrap_or("default");
        let ftw = FlexibleTimeWindow {
            mode: input.flexible_time_window.mode.clone(),
            maximum_window_in_minutes: input
                .flexible_time_window
                .maximum_window_in_minutes
                .map(|v| v as i64),
        };
        let target = ScheduleTarget {
            arn: input.target.arn.clone(),
            role_arn: input.target.role_arn.clone(),
            retry_policy: RetryPolicy::default(),
        };
        let start_date = input.start_date.map(|v| v.to_string());
        let end_date = input.end_date.map(|v| v.to_string());

        let mut state = state.write().await;
        match state.update_schedule(
            &input.name,
            group_name,
            account_id,
            region,
            &input.schedule_expression,
            ftw,
            target,
            input.state.as_deref(),
            input.description.as_deref(),
            input.action_after_completion.as_deref(),
            start_date.as_deref(),
            end_date.as_deref(),
        ) {
            Ok(arn) => wire::serialize_update_schedule_response(&wire::UpdateScheduleOutput {
                schedule_arn: Some(arn),
            }),
            Err(e) => scheduler_error_response(&e),
        }
    }

    async fn handle_delete_schedule(
        &self,
        state: &Arc<tokio::sync::RwLock<SchedulerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_schedule_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "SerializationException", &e),
        };
        let group_name = input.group_name.as_deref().unwrap_or("default");

        let mut state = state.write().await;
        match state.delete_schedule(&input.name, group_name, account_id, region) {
            Ok(()) => wire::serialize_delete_schedule_response(&wire::DeleteScheduleOutput {}),
            Err(e) => scheduler_error_response(&e),
        }
    }

    async fn handle_list_schedules(
        &self,
        state: &Arc<tokio::sync::RwLock<SchedulerState>>,
        account_id: &str,
        region: &str,
        request: &MockRequest,
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_schedules_request(request, &[], query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "SerializationException", &e),
        };
        let group_name = input.group_name.as_deref();
        let state_filter = input.state.as_deref();
        let name_prefix = input.name_prefix.as_deref();
        let max_results = input.max_results.map(|v| v as usize);
        let next_token = input.next_token.as_deref();

        let mut state = state.write().await;
        let (schedules, token) = state.list_schedules(
            account_id,
            region,
            group_name,
            state_filter,
            name_prefix,
            max_results,
            next_token,
        );

        let entries: Vec<wire::ScheduleSummary> = schedules
            .iter()
            .map(|s| wire::ScheduleSummary {
                arn: Some(s.arn.clone()),
                name: Some(s.name.clone()),
                group_name: Some(s.group_name.clone()),
                state: Some(s.state.as_str().to_string()),
                target: Some(wire::TargetSummary {
                    arn: Some(s.target.arn.clone()),
                }),
                creation_date: parse_epoch_opt(&s.creation_date),
                last_modification_date: parse_epoch_opt(&s.last_modification_date),
            })
            .collect();

        wire::serialize_list_schedules_response(&wire::ListSchedulesOutput {
            schedules: Some(entries),
            next_token: token,
        })
    }

    async fn handle_create_schedule_group(
        &self,
        state: &Arc<tokio::sync::RwLock<SchedulerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_schedule_group_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "SerializationException", &e),
        };
        let tags: Vec<Tag> = input
            .tags
            .unwrap_or_default()
            .into_iter()
            .map(|t| Tag {
                key: t.key,
                value: t.value,
            })
            .collect();

        let mut state = state.write().await;
        match state.create_schedule_group(&input.name, account_id, region, tags) {
            Ok(arn) => {
                wire::serialize_create_schedule_group_response(&wire::CreateScheduleGroupOutput {
                    schedule_group_arn: Some(arn),
                })
            }
            Err(e) => scheduler_error_response(&e),
        }
    }

    async fn handle_get_schedule_group(
        &self,
        state: &Arc<tokio::sync::RwLock<SchedulerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_get_schedule_group_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "SerializationException", &e),
        };
        let mut state = state.write().await;
        match state.get_schedule_group(&input.name, account_id, region) {
            Ok(group) => {
                wire::serialize_get_schedule_group_response(&wire::GetScheduleGroupOutput {
                    arn: Some(group.arn.clone()),
                    name: Some(group.name.clone()),
                    state: Some(group.state.clone()),
                    creation_date: parse_epoch_opt(&group.creation_date),
                    last_modification_date: parse_epoch_opt(&group.last_modification_date),
                })
            }
            Err(e) => scheduler_error_response(&e),
        }
    }

    async fn handle_delete_schedule_group(
        &self,
        state: &Arc<tokio::sync::RwLock<SchedulerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_schedule_group_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "SerializationException", &e),
        };
        let mut state = state.write().await;
        match state.delete_schedule_group(&input.name) {
            Ok(()) => {
                wire::serialize_delete_schedule_group_response(&wire::DeleteScheduleGroupOutput {})
            }
            Err(e) => scheduler_error_response(&e),
        }
    }

    async fn handle_list_schedule_groups(
        &self,
        state: &Arc<tokio::sync::RwLock<SchedulerState>>,
        account_id: &str,
        region: &str,
        request: &MockRequest,
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_schedule_groups_request(request, &[], query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "SerializationException", &e),
        };
        let name_prefix = input.name_prefix.as_deref();
        let max_results = input.max_results.map(|v| v as usize);
        let next_token = input.next_token.as_deref();

        let mut state = state.write().await;
        let (groups, token) =
            state.list_schedule_groups(account_id, region, name_prefix, max_results, next_token);

        let entries: Vec<wire::ScheduleGroupSummary> = groups
            .iter()
            .map(|g| wire::ScheduleGroupSummary {
                arn: Some(g.arn.clone()),
                name: Some(g.name.clone()),
                state: Some(g.state.clone()),
                creation_date: parse_epoch_opt(&g.creation_date),
                last_modification_date: parse_epoch_opt(&g.last_modification_date),
            })
            .collect();

        wire::serialize_list_schedule_groups_response(&wire::ListScheduleGroupsOutput {
            schedule_groups: Some(entries),
            next_token: token,
        })
    }

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<SchedulerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "SerializationException", &e),
        };
        let state = state.read().await;
        match state.list_tags_for_resource(&input.resource_arn) {
            Ok(tags) => {
                let tag_entries: Vec<wire::Tag> = tags
                    .iter()
                    .map(|t| wire::Tag {
                        key: t.key.clone(),
                        value: t.value.clone(),
                    })
                    .collect();
                wire::serialize_list_tags_for_resource_response(&wire::ListTagsForResourceOutput {
                    tags: Some(tag_entries),
                })
            }
            Err(e) => scheduler_error_response(&e),
        }
    }

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<SchedulerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "SerializationException", &e),
        };
        let tags: Vec<Tag> = input
            .tags
            .into_iter()
            .map(|t| Tag {
                key: t.key,
                value: t.value,
            })
            .collect();

        let mut state = state.write().await;
        match state.tag_resource(&input.resource_arn, tags) {
            Ok(()) => wire::serialize_tag_resource_response(&wire::TagResourceOutput {}),
            Err(e) => scheduler_error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<SchedulerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query_string: &str,
    ) -> MockResponse {
        // UntagResource takes TagKeys as a multi-valued query parameter; the generated
        // deserialiser only handles single values, so parse the multi-value list directly
        // and seed the input via labels.
        let resource_arn = labels
            .iter()
            .find(|(k, _)| *k == "ResourceArn")
            .map(|(_, v)| (*v).to_string())
            .unwrap_or_default();
        if resource_arn.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing ResourceArn");
        }
        // Validate the body parses (kept for parity with peer handlers).
        let _ = request;
        let params = parse_query_string_multi(query_string);
        let tag_keys: Vec<String> = params.get("TagKeys").cloned().unwrap_or_default();

        let mut state = state.write().await;
        match state.untag_resource(&resource_arn, &tag_keys) {
            Ok(()) => wire::serialize_untag_resource_response(&wire::UntagResourceOutput {}),
            Err(e) => scheduler_error_response(&e),
        }
    }
}

// --- Utility functions ---

fn schedule_to_get_schedule_output(schedule: &Schedule) -> wire::GetScheduleOutput {
    wire::GetScheduleOutput {
        arn: Some(schedule.arn.clone()),
        name: Some(schedule.name.clone()),
        group_name: Some(schedule.group_name.clone()),
        schedule_expression: Some(schedule.schedule_expression.clone()),
        flexible_time_window: Some(wire::FlexibleTimeWindow {
            mode: schedule.flexible_time_window.mode.clone(),
            maximum_window_in_minutes: schedule
                .flexible_time_window
                .maximum_window_in_minutes
                .map(|v| v as i32),
        }),
        target: Some(wire::Target {
            arn: schedule.target.arn.clone(),
            role_arn: schedule.target.role_arn.clone(),
            retry_policy: Some(wire::RetryPolicy {
                maximum_event_age_in_seconds: Some(
                    schedule.target.retry_policy.maximum_event_age_in_seconds as i32,
                ),
                maximum_retry_attempts: Some(
                    schedule.target.retry_policy.maximum_retry_attempts as i32,
                ),
            }),
            ..Default::default()
        }),
        state: Some(schedule.state.as_str().to_string()),
        creation_date: parse_epoch_opt(&schedule.creation_date),
        last_modification_date: parse_epoch_opt(&schedule.last_modification_date),
        description: schedule.description.clone(),
        action_after_completion: schedule.action_after_completion.clone(),
        start_date: schedule
            .start_date
            .as_ref()
            .and_then(|s| s.parse::<f64>().ok()),
        end_date: schedule
            .end_date
            .as_ref()
            .and_then(|s| s.parse::<f64>().ok()),
        ..Default::default()
    }
}

fn extract_path_and_query(uri: &str) -> (String, String) {
    // Remove scheme and host
    let path_and_query = if let Some(pos) = uri.find("://") {
        let rest = &uri[pos + 3..];
        if let Some(pos) = rest.find('/') {
            &rest[pos..]
        } else {
            "/"
        }
    } else {
        uri
    };

    if let Some(pos) = path_and_query.find('?') {
        (
            path_and_query[..pos].to_string(),
            path_and_query[pos + 1..].to_string(),
        )
    } else {
        (path_and_query.to_string(), String::new())
    }
}

fn parse_query_string(query: &str) -> std::collections::HashMap<String, String> {
    let mut map = std::collections::HashMap::new();
    if query.is_empty() {
        return map;
    }
    for pair in query.split('&') {
        if let Some((k, v)) = pair.split_once('=') {
            let k = urlencoding::decode(k).unwrap_or_default().to_string();
            let v = urlencoding::decode(v).unwrap_or_default().to_string();
            map.insert(k, v);
        }
    }
    map
}

fn parse_query_string_multi(query: &str) -> std::collections::HashMap<String, Vec<String>> {
    let mut map: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();
    if query.is_empty() {
        return map;
    }
    for pair in query.split('&') {
        if let Some((k, v)) = pair.split_once('=') {
            let k = urlencoding::decode(k).unwrap_or_default().to_string();
            let v = urlencoding::decode(v).unwrap_or_default().to_string();
            map.entry(k).or_default().push(v);
        }
    }
    map
}

/// Parse an epoch seconds string (e.g. "1711012345.123") into an Option<f64>.
fn parse_epoch_opt(s: &str) -> Option<f64> {
    s.parse::<f64>().ok()
}

fn scheduler_error_response(err: &SchedulerError) -> MockResponse {
    let (status, error_type) = match err {
        SchedulerError::ScheduleGroupAlreadyExists(_) => (409, "ConflictException"),
        SchedulerError::ScheduleAlreadyExists(_) => (409, "ConflictException"),
        SchedulerError::ScheduleGroupNotFound(_) => (404, "ResourceNotFoundException"),
        SchedulerError::ScheduleNotFound(_) => (404, "ResourceNotFoundException"),
        SchedulerError::ResourceNotFound(_) => (404, "ResourceNotFoundException"),
    };
    let body = json!({
        "Message": err.to_string(),
    });
    let mut response = MockResponse::rest_json(status, body.to_string());
    response
        .headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    response
}

fn rest_json_error(status: u16, code: &str, message: &str) -> MockResponse {
    let body = json!({
        "Message": message,
    });
    let mut response = MockResponse::rest_json(status, body.to_string());
    response
        .headers
        .insert(X_AMZN_ERRORTYPE, code.parse().unwrap());
    response
}
