use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::json;
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, StatefulService,
    default_account_id,
    protocol::common::{extract_path, extract_query_string, percent_decode},
};

use crate::model;
use crate::state::{CodeStarError, CodeStarNotificationsState, RuleRecord, TargetRecord};
use crate::views::CodeStarNotificationsStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct CodeStarNotificationsService {
    pub(crate) state: Arc<BackendState<CodeStarNotificationsState>>,
    pub(crate) notifier: StateChangeNotifier<CodeStarNotificationsStateView>,
}

impl CodeStarNotificationsService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for CodeStarNotificationsService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for CodeStarNotificationsService {
    fn service_name(&self) -> &str {
        "codestar-notifications"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://codestar-notifications\..*\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

type SharedState = Arc<tokio::sync::RwLock<CodeStarNotificationsState>>;

fn target_record_to_summary(t: &TargetRecord) -> model::TargetSummary {
    model::TargetSummary {
        target_address: Some(t.target_address.clone()),
        target_status: Some(t.target_status.clone()),
        target_type: Some(t.target_type.clone()),
    }
}

fn rule_to_describe_output(
    rule: &RuleRecord,
    tags: &HashMap<String, String>,
) -> model::DescribeNotificationRuleResult {
    let event_types: Vec<model::EventTypeSummary> = rule
        .event_type_ids
        .iter()
        .map(|id| model::EventTypeSummary {
            event_type_id: Some(id.clone()),
            ..Default::default()
        })
        .collect();
    let targets: Vec<model::TargetSummary> =
        rule.targets.iter().map(target_record_to_summary).collect();
    model::DescribeNotificationRuleResult {
        arn: Some(rule.arn.clone()),
        created_by: Some("mock".to_string()),
        created_timestamp: Some(rule.created_timestamp),
        detail_type: Some(rule.detail_type.clone()),
        event_types: Some(event_types),
        last_modified_timestamp: Some(rule.last_modified_timestamp),
        name: Some(rule.name.clone()),
        resource: Some(rule.resource.clone()),
        status: Some(rule.status.clone()),
        tags: Some(tags.clone()),
        targets: Some(targets),
    }
}

impl CodeStarNotificationsService {
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
        let query_str = extract_query_string(&request.uri);
        let query: HashMap<String, String> = winterbaume_core::parse_query_string(query_str);
        let method = request.method.as_str().to_string();

        // Validate JSON body up-front; the typed deserialisers in `wire` re-parse the bytes
        // per operation.
        if !request.body.is_empty()
            && serde_json::from_slice::<serde_json::Value>(&request.body).is_err()
        {
            return rest_json_error(400, "ValidationException", "Invalid JSON body");
        }

        let (response, mutating) = match (method.as_str(), segments.as_slice()) {
            ("POST", ["createNotificationRule"]) => (
                self.handle_create_rule(&state, &request, &[], &query, &region, account_id)
                    .await,
                true,
            ),
            ("POST", ["deleteNotificationRule"]) => (
                self.handle_delete_rule(&state, &request, &[], &query).await,
                true,
            ),
            ("POST", ["deleteTarget"]) => (
                self.handle_delete_target(&state, &request, &[], &query)
                    .await,
                true,
            ),
            ("POST", ["describeNotificationRule"]) => (
                self.handle_describe_rule(&state, &request, &[], &query)
                    .await,
                false,
            ),
            ("POST", ["listEventTypes"]) => (
                self.handle_list_event_types(&request, &[], &query).await,
                false,
            ),
            ("POST", ["listNotificationRules"]) => (
                self.handle_list_rules(&state, &request, &[], &query).await,
                false,
            ),
            ("POST", ["listTagsForResource"]) => (
                self.handle_list_tags(&state, &request, &[], &query).await,
                false,
            ),
            ("POST", ["listTargets"]) => (
                self.handle_list_targets(&state, &request, &[], &query)
                    .await,
                false,
            ),
            ("POST", ["subscribe"]) => (
                self.handle_subscribe(&state, &request, &[], &query).await,
                true,
            ),
            ("POST", ["tagResource"]) => (
                self.handle_tag_resource(&state, &request, &[], &query)
                    .await,
                true,
            ),
            ("POST", ["unsubscribe"]) => (
                self.handle_unsubscribe(&state, &request, &[], &query).await,
                true,
            ),
            ("POST", ["untagResource", arn]) => {
                let labels: &[(&str, &str)] = &[("Arn", arn)];
                (
                    self.handle_untag_resource(&state, &request, labels, &query)
                        .await,
                    true,
                )
            }
            ("POST", ["updateNotificationRule"]) => (
                self.handle_update_rule(&state, &request, &[], &query).await,
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

    async fn handle_create_rule(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_notification_rule_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return rest_json_error(400, "ValidationException", "Name is required");
        }
        let resource = input.resource;
        let detail_type = if input.detail_type.is_empty() {
            "BASIC".to_string()
        } else {
            input.detail_type
        };
        let status = input.status.unwrap_or_else(|| "ENABLED".to_string());
        let event_type_ids = input.event_type_ids;
        let targets: Vec<TargetRecord> = input
            .targets
            .into_iter()
            .filter_map(|t| {
                let address = t.target_address?;
                Some(TargetRecord {
                    target_address: address,
                    target_type: t.target_type.unwrap_or_else(|| "SNS".to_string()),
                    target_status: "ACTIVE".to_string(),
                })
            })
            .collect();
        let id = uuid::Uuid::new_v4().simple().to_string();
        let arn =
            format!("arn:aws:codestar-notifications:{region}:{account_id}:notificationrule/{id}");
        let now = chrono::Utc::now().timestamp() as f64;
        let rule = RuleRecord {
            arn: arn.clone(),
            name: input.name,
            resource,
            detail_type,
            event_type_ids,
            status,
            targets,
            created_timestamp: now,
            last_modified_timestamp: now,
        };
        let tag_map = input.tags.unwrap_or_default();
        let mut state = state.write().await;
        state.put_rule(rule);
        if !tag_map.is_empty() {
            state.tag_resource(&arn, tag_map);
        }
        wire::serialize_create_notification_rule_response(&model::CreateNotificationRuleResult {
            arn: Some(arn),
        })
    }

    async fn handle_delete_rule(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_notification_rule_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.arn.is_empty() {
            return rest_json_error(400, "ValidationException", "Arn is required");
        }
        let mut state = state.write().await;
        state.rules.remove(&input.arn);
        state.tags.remove(&input.arn);
        wire::serialize_delete_notification_rule_response(&model::DeleteNotificationRuleResult {
            arn: Some(input.arn),
        })
    }

    async fn handle_delete_target(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_target_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.target_address.is_empty() {
            return rest_json_error(400, "ValidationException", "TargetAddress is required");
        }
        let force = input.force_unsubscribe_all.unwrap_or(false);
        let mut state = state.write().await;
        state.delete_target(&input.target_address, force);
        wire::serialize_delete_target_response(&model::DeleteTargetResult {})
    }

    async fn handle_describe_rule(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_describe_notification_rule_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        if input.arn.is_empty() {
            return rest_json_error(400, "ValidationException", "Arn is required");
        }
        let state = state.read().await;
        match state.rules.get(&input.arn) {
            Some(rule) => {
                let tags = state.list_tags(&input.arn);
                let output = rule_to_describe_output(rule, &tags);
                wire::serialize_describe_notification_rule_response(&output)
            }
            None => err_response(&CodeStarError::NotFound { arn: input.arn }),
        }
    }

    async fn handle_list_event_types(
        &self,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_list_event_types_request(request, labels, query) {
            return rest_json_error(400, "ValidationException", &e);
        }
        let event_types = vec![
            model::EventTypeSummary {
                event_type_id: Some("codecommit-repository-pull-request-created".to_string()),
                event_type_name: Some("Pull request created".to_string()),
                resource_type: Some("CodeCommit".to_string()),
                service_name: Some("CodeCommit".to_string()),
            },
            model::EventTypeSummary {
                event_type_id: Some(
                    "codepipeline-pipeline-pipeline-execution-succeeded".to_string(),
                ),
                event_type_name: Some("Pipeline execution succeeded".to_string()),
                resource_type: Some("CodePipeline".to_string()),
                service_name: Some("CodePipeline".to_string()),
            },
        ];
        wire::serialize_list_event_types_response(&model::ListEventTypesResult {
            event_types: Some(event_types),
            next_token: None,
        })
    }

    async fn handle_list_rules(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_list_notification_rules_request(request, labels, query) {
            return rest_json_error(400, "ValidationException", &e);
        }
        let state = state.read().await;
        let summaries: Vec<model::NotificationRuleSummary> = state
            .list_rules()
            .into_iter()
            .map(|r| model::NotificationRuleSummary {
                id: Some(r.arn.split('/').next_back().unwrap_or("").to_string()),
                arn: Some(r.arn.clone()),
            })
            .collect();
        wire::serialize_list_notification_rules_response(&model::ListNotificationRulesResult {
            next_token: None,
            notification_rules: Some(summaries),
        })
    }

    async fn handle_list_tags(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.arn.is_empty() {
            return rest_json_error(400, "ValidationException", "Arn is required");
        }
        let state = state.read().await;
        let tags = state.list_tags(&input.arn);
        wire::serialize_list_tags_for_resource_response(&model::ListTagsForResourceResult {
            tags: Some(tags),
        })
    }

    async fn handle_list_targets(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_list_targets_request(request, labels, query) {
            return rest_json_error(400, "ValidationException", &e);
        }
        let state = state.read().await;
        let targets: Vec<model::TargetSummary> = state
            .list_targets()
            .iter()
            .map(target_record_to_summary)
            .collect();
        wire::serialize_list_targets_response(&model::ListTargetsResult {
            next_token: None,
            targets: Some(targets),
        })
    }

    async fn handle_subscribe(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_subscribe_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.arn.is_empty() {
            return rest_json_error(400, "ValidationException", "Arn is required");
        }
        let address = input.target.target_address.unwrap_or_default();
        let ttype = input
            .target
            .target_type
            .unwrap_or_else(|| "SNS".to_string());
        let mut state = state.write().await;
        let rule = match state.rules.get_mut(&input.arn) {
            Some(r) => r,
            None => return err_response(&CodeStarError::NotFound { arn: input.arn }),
        };
        if !rule.targets.iter().any(|t| t.target_address == address) {
            rule.targets.push(TargetRecord {
                target_address: address,
                target_type: ttype,
                target_status: "ACTIVE".to_string(),
            });
        }
        wire::serialize_subscribe_response(&model::SubscribeResult {
            arn: Some(input.arn),
        })
    }

    async fn handle_tag_resource(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.arn.is_empty() {
            return rest_json_error(400, "ValidationException", "Arn is required");
        }
        let mut state = state.write().await;
        state.tag_resource(&input.arn, input.tags);
        let tags = state.list_tags(&input.arn);
        wire::serialize_tag_resource_response(&model::TagResourceResult { tags: Some(tags) })
    }

    async fn handle_unsubscribe(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_unsubscribe_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.arn.is_empty() {
            return rest_json_error(400, "ValidationException", "Arn is required");
        }
        if input.target_address.is_empty() {
            return rest_json_error(400, "ValidationException", "TargetAddress is required");
        }
        let mut state = state.write().await;
        let rule = match state.rules.get_mut(&input.arn) {
            Some(r) => r,
            None => return err_response(&CodeStarError::NotFound { arn: input.arn }),
        };
        rule.targets
            .retain(|t| t.target_address != input.target_address);
        wire::serialize_unsubscribe_response(&model::UnsubscribeResult {
            arn: Some(input.arn),
        })
    }

    async fn handle_untag_resource(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        state.untag_resource(&input.arn, &input.tag_keys);
        wire::serialize_untag_resource_response(&model::UntagResourceResult {})
    }

    async fn handle_update_rule(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_notification_rule_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.arn.is_empty() {
            return rest_json_error(400, "ValidationException", "Arn is required");
        }
        let mut state = state.write().await;
        let rule = match state.rules.get_mut(&input.arn) {
            Some(r) => r,
            None => return err_response(&CodeStarError::NotFound { arn: input.arn }),
        };
        if let Some(name) = input.name {
            rule.name = name;
        }
        if let Some(status) = input.status {
            rule.status = status;
        }
        if let Some(detail) = input.detail_type {
            rule.detail_type = detail;
        }
        if let Some(ids) = input.event_type_ids {
            rule.event_type_ids = ids;
        }
        if let Some(targets) = input.targets {
            rule.targets = targets
                .into_iter()
                .filter_map(|t| {
                    let address = t.target_address?;
                    Some(TargetRecord {
                        target_address: address,
                        target_type: t.target_type.unwrap_or_else(|| "SNS".to_string()),
                        target_status: "ACTIVE".to_string(),
                    })
                })
                .collect();
        }
        rule.last_modified_timestamp = chrono::Utc::now().timestamp() as f64;
        wire::serialize_update_notification_rule_response(&model::UpdateNotificationRuleResult {})
    }
}

fn rest_json_error(status: u16, error_type: &str, message: &str) -> MockResponse {
    let body = json!({"__type": error_type, "Message": message});
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}

fn err_response(err: &CodeStarError) -> MockResponse {
    let (status, error_type) = match err {
        CodeStarError::NotFound { .. } => (404, "ResourceNotFoundException"),
        CodeStarError::Validation { .. } => (400, "ValidationException"),
    };
    rest_json_error(status, error_type, &err.to_string())
}
