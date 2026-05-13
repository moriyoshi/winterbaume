use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, default_account_id,
};

use crate::state::{EventsError, EventsState, PutEventsEntry};
use crate::types::{Tag, Target};
use crate::views::EventsStateView;
use crate::wire;

/// EventBridge service handler that processes awsJson1.1 protocol requests.
pub struct EventBridgeService {
    pub(crate) state: Arc<BackendState<EventsState>>,
    pub(crate) notifier: StateChangeNotifier<EventsStateView>,
}

impl EventBridgeService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for EventBridgeService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for EventBridgeService {
    fn service_name(&self) -> &str {
        "events"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://events\..*\.amazonaws\.com",
            r"https?://events\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl EventBridgeService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();

        // Extract action from X-Amz-Target header
        // Format: "AWSEvents.PutRule"
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
            "PutRule" => {
                self.handle_put_rule(&state, body_bytes, account_id, &region)
                    .await
            }
            "DeleteRule" => self.handle_delete_rule(&state, body_bytes).await,
            "DescribeRule" => self.handle_describe_rule(&state, body_bytes).await,
            "ListRules" => self.handle_list_rules(&state, body_bytes).await,
            "PutTargets" => self.handle_put_targets(&state, body_bytes).await,
            "RemoveTargets" => self.handle_remove_targets(&state, body_bytes).await,
            "ListTargetsByRule" => self.handle_list_targets_by_rule(&state, body_bytes).await,
            "PutEvents" => self.handle_put_events(&state, body_bytes).await,
            "CreateEventBus" => {
                self.handle_create_event_bus(&state, body_bytes, account_id, &region)
                    .await
            }
            "DeleteEventBus" => self.handle_delete_event_bus(&state, body_bytes).await,
            "DescribeEventBus" => {
                self.handle_describe_event_bus(&state, body_bytes, account_id, &region)
                    .await
            }
            "ListEventBuses" => self.handle_list_event_buses(&state, body_bytes).await,
            "PutPermission" => {
                self.handle_put_permission(&state, body_bytes, account_id, &region)
                    .await
            }
            "RemovePermission" => self.handle_remove_permission(&state, body_bytes).await,
            "CreateArchive" => {
                self.handle_create_archive(&state, body_bytes, account_id, &region)
                    .await
            }
            "DeleteArchive" => self.handle_delete_archive(&state, body_bytes).await,
            "DescribeArchive" => self.handle_describe_archive(&state, body_bytes).await,
            "UpdateArchive" => self.handle_update_archive(&state, body_bytes).await,
            "ListArchives" => self.handle_list_archives(&state, body_bytes).await,
            "CreateConnection" => {
                self.handle_create_connection(&state, body_bytes, account_id, &region)
                    .await
            }
            "DeleteConnection" => self.handle_delete_connection(&state, body_bytes).await,
            "DescribeConnection" => self.handle_describe_connection(&state, body_bytes).await,
            "UpdateConnection" => self.handle_update_connection(&state, body_bytes).await,
            "ListConnections" => self.handle_list_connections(&state, body_bytes).await,
            "CreateApiDestination" => {
                self.handle_create_api_destination(&state, body_bytes, account_id, &region)
                    .await
            }
            "DeleteApiDestination" => self.handle_delete_api_destination(&state, body_bytes).await,
            "DescribeApiDestination" => {
                self.handle_describe_api_destination(&state, body_bytes)
                    .await
            }
            "UpdateApiDestination" => self.handle_update_api_destination(&state, body_bytes).await,
            "ListApiDestinations" => self.handle_list_api_destinations(&state, body_bytes).await,
            "StartReplay" => {
                self.handle_start_replay(&state, body_bytes, account_id, &region)
                    .await
            }
            "CancelReplay" => self.handle_cancel_replay(&state, body_bytes).await,
            "DescribeReplay" => self.handle_describe_replay(&state, body_bytes).await,
            "ListReplays" => self.handle_list_replays(&state, body_bytes).await,
            "CreatePartnerEventSource" => {
                self.handle_create_partner_event_source(&state, body_bytes, account_id, &region)
                    .await
            }
            "DeletePartnerEventSource" => {
                self.handle_delete_partner_event_source(&state, body_bytes)
                    .await
            }
            "DescribeEventSource" => self.handle_describe_event_source(&state, body_bytes).await,
            "DescribePartnerEventSource" => {
                self.handle_describe_partner_event_source(&state, body_bytes)
                    .await
            }
            "DisableRule" => self.handle_disable_rule(&state, body_bytes).await,
            "EnableRule" => self.handle_enable_rule(&state, body_bytes).await,
            "ListRuleNamesByTarget" => {
                self.handle_list_rule_names_by_target(&state, body_bytes)
                    .await
            }
            "TagResource" => self.handle_tag_resource(&state, body_bytes).await,
            "UntagResource" => self.handle_untag_resource(&state, body_bytes).await,
            "ListTagsForResource" => self.handle_list_tags_for_resource(&state, body_bytes).await,
            "PutPartnerEvents" => self.handle_put_partner_events(&state, body_bytes).await,
            "TestEventPattern" => self.handle_test_event_pattern(&state, body_bytes).await,
            "ActivateEventSource" => self.handle_activate_event_source(&state, body_bytes).await,
            "DeactivateEventSource" => {
                self.handle_deactivate_event_source(&state, body_bytes)
                    .await
            }
            "ListEventSources" => self.handle_list_event_sources(&state, body_bytes).await,
            "ListPartnerEventSourceAccounts" => {
                self.handle_list_partner_event_source_accounts(&state, body_bytes)
                    .await
            }
            "ListPartnerEventSources" => {
                self.handle_list_partner_event_sources(&state, body_bytes)
                    .await
            }
            "DeauthorizeConnection" => self.handle_deauthorize_connection(&state, body_bytes).await,
            "CreateEndpoint" => {
                self.handle_create_endpoint(&state, body_bytes, account_id, &region)
                    .await
            }
            "DeleteEndpoint" => self.handle_delete_endpoint(&state, body_bytes).await,
            "DescribeEndpoint" => self.handle_describe_endpoint(&state, body_bytes).await,
            "ListEndpoints" => self.handle_list_endpoints(&state, body_bytes).await,
            "UpdateEndpoint" => self.handle_update_endpoint(&state, body_bytes).await,
            "UpdateEventBus" => {
                self.handle_update_event_bus(&state, body_bytes, account_id, &region)
                    .await
            }
            _ => json_error_response(
                400,
                "InvalidAction",
                &format!("Could not find operation {action} for EventBridge"),
            ),
        };
        if response.status / 100 == 2 {
            use winterbaume_core::StatefulService;
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_put_rule(
        &self,
        state: &Arc<tokio::sync::RwLock<EventsState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_put_rule_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Name'");
        }
        let event_pattern = input.event_pattern.as_deref();
        let schedule_expression = input.schedule_expression.as_deref();
        let rule_state = input.state.as_deref();
        let description = input.description.as_deref();
        let event_bus_name = input.event_bus_name.as_deref();

        let mut state = state.write().await;
        match state.put_rule(
            &input.name,
            account_id,
            region,
            event_pattern,
            schedule_expression,
            rule_state,
            description,
            event_bus_name,
        ) {
            Ok(rule) => wire::serialize_put_rule_response(&wire::PutRuleResponse {
                rule_arn: Some(rule.arn.clone()),
            }),
            Err(e) => events_error_response(&e),
        }
    }

    async fn handle_delete_rule(
        &self,
        state: &Arc<tokio::sync::RwLock<EventsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_rule_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Name'");
        }

        let mut state = state.write().await;
        match state.delete_rule(&input.name) {
            Ok(()) => wire::serialize_delete_rule_response(),
            Err(e) => events_error_response(&e),
        }
    }

    async fn handle_describe_rule(
        &self,
        state: &Arc<tokio::sync::RwLock<EventsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_rule_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Name'");
        }

        let state = state.read().await;
        match state.describe_rule(&input.name) {
            Ok(rule) => wire::serialize_describe_rule_response(&wire::DescribeRuleResponse {
                name: Some(rule.name.clone()),
                arn: Some(rule.arn.clone()),
                state: Some(rule.state.as_str().to_string()),
                event_bus_name: Some(rule.event_bus_name.clone()),
                event_pattern: rule.event_pattern.clone(),
                schedule_expression: rule.schedule_expression.clone(),
                description: rule.description.clone(),
                ..Default::default()
            }),
            Err(e) => events_error_response(&e),
        }
    }

    async fn handle_list_rules(
        &self,
        state: &Arc<tokio::sync::RwLock<EventsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_rules_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let name_prefix = input.name_prefix.as_deref();

        let state = state.read().await;
        let rules = state.list_rules(name_prefix);
        let entries: Vec<wire::Rule> = rules.iter().map(|r| to_wire_rule(r)).collect();

        wire::serialize_list_rules_response(&wire::ListRulesResponse {
            rules: Some(entries),
            next_token: None,
        })
    }

    async fn handle_put_targets(
        &self,
        state: &Arc<tokio::sync::RwLock<EventsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_put_targets_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.rule.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Rule'");
        }

        let targets: Vec<Target> = input
            .targets
            .into_iter()
            .filter(|t| !t.id.is_empty() && !t.arn.is_empty())
            .map(|t| Target {
                id: t.id,
                arn: t.arn,
                input: t.input,
                input_path: t.input_path,
            })
            .collect();

        let mut state = state.write().await;
        match state.put_targets(&input.rule, targets) {
            Ok(()) => wire::serialize_put_targets_response(&wire::PutTargetsResponse {
                failed_entry_count: Some(0),
                failed_entries: Some(vec![]),
            }),
            Err(e) => events_error_response(&e),
        }
    }

    async fn handle_remove_targets(
        &self,
        state: &Arc<tokio::sync::RwLock<EventsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_remove_targets_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.rule.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Rule'");
        }

        let mut state = state.write().await;
        match state.remove_targets(&input.rule, &input.ids) {
            Ok(()) => wire::serialize_remove_targets_response(&wire::RemoveTargetsResponse {
                failed_entry_count: Some(0),
                failed_entries: Some(vec![]),
            }),
            Err(e) => events_error_response(&e),
        }
    }

    async fn handle_list_targets_by_rule(
        &self,
        state: &Arc<tokio::sync::RwLock<EventsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_targets_by_rule_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.rule.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Rule'");
        }

        let state = state.read().await;
        match state.list_targets_by_rule(&input.rule) {
            Ok(targets) => {
                let entries: Vec<wire::Target> = targets
                    .iter()
                    .map(|t| wire::Target {
                        id: t.id.clone(),
                        arn: t.arn.clone(),
                        input: t.input.clone(),
                        input_path: t.input_path.clone(),
                        ..Default::default()
                    })
                    .collect();
                wire::serialize_list_targets_by_rule_response(&wire::ListTargetsByRuleResponse {
                    targets: Some(entries),
                    next_token: None,
                })
            }
            Err(e) => events_error_response(&e),
        }
    }

    async fn handle_put_events(
        &self,
        state: &Arc<tokio::sync::RwLock<EventsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_put_events_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let entries: Vec<PutEventsEntry> = input
            .entries
            .into_iter()
            .map(|e| PutEventsEntry {
                source: e.source,
                detail_type: e.detail_type,
                detail: e.detail,
            })
            .collect();

        let state = state.read().await;
        let results = state.put_events(&entries);

        let result_entries: Vec<wire::PutEventsResultEntry> = results
            .iter()
            .map(|r| wire::PutEventsResultEntry {
                event_id: Some(r.event_id.clone()),
                ..Default::default()
            })
            .collect();

        wire::serialize_put_events_response(&wire::PutEventsResponse {
            failed_entry_count: Some(0),
            entries: Some(result_entries),
        })
    }
    // --- EventBus operations ---

    async fn handle_create_event_bus(
        &self,
        state: &Arc<tokio::sync::RwLock<EventsState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_event_bus_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Name'");
        }

        let mut state = state.write().await;
        match state.create_event_bus(&input.name, account_id, region) {
            Ok(bus) => wire::serialize_create_event_bus_response(&wire::CreateEventBusResponse {
                event_bus_arn: Some(bus.arn.clone()),
                ..Default::default()
            }),
            Err(e) => events_error_response(&e),
        }
    }

    async fn handle_delete_event_bus(
        &self,
        state: &Arc<tokio::sync::RwLock<EventsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_event_bus_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Name'");
        }

        let mut state = state.write().await;
        match state.delete_event_bus(&input.name) {
            Ok(()) => wire::serialize_delete_event_bus_response(),
            Err(e) => events_error_response(&e),
        }
    }

    async fn handle_describe_event_bus(
        &self,
        state: &Arc<tokio::sync::RwLock<EventsState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_event_bus_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let name = input.name.as_deref().unwrap_or("default");

        let mut state = state.write().await;
        state.ensure_default_bus(account_id, region);
        match state.describe_event_bus(name) {
            Ok(bus) => {
                wire::serialize_describe_event_bus_response(&wire::DescribeEventBusResponse {
                    name: Some(bus.name.clone()),
                    arn: Some(bus.arn.clone()),
                    policy: bus.policy.clone(),
                    ..Default::default()
                })
            }
            Err(e) => events_error_response(&e),
        }
    }

    async fn handle_list_event_buses(
        &self,
        state: &Arc<tokio::sync::RwLock<EventsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_event_buses_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let name_prefix = input.name_prefix.as_deref();

        let state = state.read().await;
        let buses = state.list_event_buses(name_prefix);
        let entries: Vec<wire::EventBus> = buses
            .iter()
            .map(|b| wire::EventBus {
                name: Some(b.name.clone()),
                arn: Some(b.arn.clone()),
                policy: b.policy.clone(),
                ..Default::default()
            })
            .collect();

        wire::serialize_list_event_buses_response(&wire::ListEventBusesResponse {
            event_buses: Some(entries),
            next_token: None,
        })
    }

    async fn handle_put_permission(
        &self,
        state: &Arc<tokio::sync::RwLock<EventsState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_put_permission_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let event_bus_name = input.event_bus_name.as_deref().unwrap_or("default");
        let policy = input.policy.as_deref();
        let action = input.action.as_deref();
        let principal = input.principal.as_deref();
        let statement_id = input.statement_id.as_deref();

        let mut state = state.write().await;
        state.ensure_default_bus(account_id, region);
        match state.put_permission(event_bus_name, policy, action, principal, statement_id) {
            Ok(()) => wire::serialize_put_permission_response(),
            Err(e) => events_error_response(&e),
        }
    }

    async fn handle_remove_permission(
        &self,
        state: &Arc<tokio::sync::RwLock<EventsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_remove_permission_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let event_bus_name = input.event_bus_name.as_deref().unwrap_or("default");
        let statement_id = input.statement_id.as_deref();
        let remove_all = input.remove_all_permissions.unwrap_or(false);

        let mut state = state.write().await;
        match state.remove_permission(event_bus_name, statement_id, remove_all) {
            Ok(()) => wire::serialize_remove_permission_response(),
            Err(e) => events_error_response(&e),
        }
    }

    // --- Archive operations ---

    async fn handle_create_archive(
        &self,
        state: &Arc<tokio::sync::RwLock<EventsState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_archive_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.archive_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ArchiveName'");
        }
        if input.event_source_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'EventSourceArn'");
        }
        let description = input.description.as_deref();
        let event_pattern = input.event_pattern.as_deref();
        let retention_days = input.retention_days.map(i64::from).unwrap_or(0);

        let mut state = state.write().await;
        match state.create_archive(
            &input.archive_name,
            account_id,
            region,
            &input.event_source_arn,
            description,
            event_pattern,
            retention_days,
        ) {
            Ok(archive) => wire::serialize_create_archive_response(&wire::CreateArchiveResponse {
                archive_arn: Some(archive.arn.clone()),
                creation_time: Some(archive.creation_time),
                state: Some(archive.state.clone()),
                ..Default::default()
            }),
            Err(e) => events_error_response(&e),
        }
    }

    async fn handle_delete_archive(
        &self,
        state: &Arc<tokio::sync::RwLock<EventsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_archive_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.archive_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ArchiveName'");
        }

        let mut state = state.write().await;
        match state.delete_archive(&input.archive_name) {
            Ok(()) => wire::serialize_delete_archive_response(&wire::DeleteArchiveResponse {}),
            Err(e) => events_error_response(&e),
        }
    }

    async fn handle_describe_archive(
        &self,
        state: &Arc<tokio::sync::RwLock<EventsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_archive_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.archive_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ArchiveName'");
        }

        let state = state.read().await;
        match state.describe_archive(&input.archive_name) {
            Ok(archive) => {
                wire::serialize_describe_archive_response(&wire::DescribeArchiveResponse {
                    archive_arn: Some(archive.arn.clone()),
                    archive_name: Some(archive.name.clone()),
                    event_source_arn: Some(archive.event_source_arn.clone()),
                    description: archive.description.clone(),
                    event_pattern: archive.event_pattern.clone(),
                    retention_days: Some(archive.retention_days as i32),
                    state: Some(archive.state.clone()),
                    size_bytes: Some(archive.size_bytes),
                    creation_time: Some(archive.creation_time),
                    ..Default::default()
                })
            }
            Err(e) => events_error_response(&e),
        }
    }

    async fn handle_update_archive(
        &self,
        state: &Arc<tokio::sync::RwLock<EventsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_archive_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.archive_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ArchiveName'");
        }
        let description = input.description.as_deref();
        let event_pattern = input.event_pattern.as_deref();
        let retention_days = input.retention_days.map(i64::from);

        let mut state = state.write().await;
        match state.update_archive(
            &input.archive_name,
            description,
            event_pattern,
            retention_days,
        ) {
            Ok(archive) => wire::serialize_update_archive_response(&wire::UpdateArchiveResponse {
                archive_arn: Some(archive.arn.clone()),
                creation_time: Some(archive.creation_time),
                state: Some(archive.state.clone()),
                ..Default::default()
            }),
            Err(e) => events_error_response(&e),
        }
    }

    async fn handle_list_archives(
        &self,
        state: &Arc<tokio::sync::RwLock<EventsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_archives_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let name_prefix = input.name_prefix.as_deref();

        let state = state.read().await;
        let archives = state.list_archives(name_prefix);
        let entries: Vec<wire::Archive> = archives
            .iter()
            .map(|a| wire::Archive {
                archive_name: Some(a.name.clone()),
                event_source_arn: Some(a.event_source_arn.clone()),
                state: Some(a.state.clone()),
                retention_days: Some(a.retention_days as i32),
                size_bytes: Some(a.size_bytes),
                creation_time: Some(a.creation_time),
                ..Default::default()
            })
            .collect();

        wire::serialize_list_archives_response(&wire::ListArchivesResponse {
            archives: Some(entries),
            next_token: None,
        })
    }

    // --- Connection operations ---

    async fn handle_create_connection(
        &self,
        state: &Arc<tokio::sync::RwLock<EventsState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_connection_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Name'");
        }
        if input.authorization_type.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'AuthorizationType'");
        }
        let description = input.description.as_deref();
        let auth_parameters =
            serde_json::to_string(&input.auth_parameters).unwrap_or_else(|_| "{}".to_string());

        let mut state = state.write().await;
        match state.create_connection(
            &input.name,
            account_id,
            region,
            description,
            &input.authorization_type,
            &auth_parameters,
        ) {
            Ok(conn) => {
                wire::serialize_create_connection_response(&wire::CreateConnectionResponse {
                    connection_arn: Some(conn.arn.clone()),
                    connection_state: Some(conn.state.clone()),
                    creation_time: Some(conn.creation_time),
                    ..Default::default()
                })
            }
            Err(e) => events_error_response(&e),
        }
    }

    async fn handle_delete_connection(
        &self,
        state: &Arc<tokio::sync::RwLock<EventsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_connection_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Name'");
        }

        let mut state = state.write().await;
        match state.remove_connection(&input.name) {
            Ok(conn) => {
                wire::serialize_delete_connection_response(&wire::DeleteConnectionResponse {
                    connection_arn: Some(conn.arn.clone()),
                    connection_state: Some("DELETING".to_string()),
                    creation_time: Some(conn.creation_time),
                    ..Default::default()
                })
            }
            Err(e) => events_error_response(&e),
        }
    }

    async fn handle_describe_connection(
        &self,
        state: &Arc<tokio::sync::RwLock<EventsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_connection_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Name'");
        }

        let state = state.read().await;
        match state.describe_connection(&input.name) {
            Ok(conn) => {
                wire::serialize_describe_connection_response(&wire::DescribeConnectionResponse {
                    name: Some(conn.name.clone()),
                    description: conn.description.clone(),
                    connection_arn: Some(conn.arn.clone()),
                    connection_state: Some(conn.state.clone()),
                    authorization_type: Some(conn.authorization_type.clone()),
                    creation_time: Some(conn.creation_time),
                    ..Default::default()
                })
            }
            Err(e) => events_error_response(&e),
        }
    }

    async fn handle_update_connection(
        &self,
        state: &Arc<tokio::sync::RwLock<EventsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_connection_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Name'");
        }
        let description = input.description.as_deref();
        let authorization_type = input.authorization_type.as_deref();
        let auth_parameters = input
            .auth_parameters
            .as_ref()
            .and_then(|p| serde_json::to_string(p).ok());

        let mut state = state.write().await;
        match state.update_connection(
            &input.name,
            description,
            authorization_type,
            auth_parameters.as_deref(),
        ) {
            Ok(conn) => {
                wire::serialize_update_connection_response(&wire::UpdateConnectionResponse {
                    connection_arn: Some(conn.arn.clone()),
                    connection_state: Some(conn.state.clone()),
                    creation_time: Some(conn.creation_time),
                    ..Default::default()
                })
            }
            Err(e) => events_error_response(&e),
        }
    }

    async fn handle_list_connections(
        &self,
        state: &Arc<tokio::sync::RwLock<EventsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_connections_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let name_prefix = input.name_prefix.as_deref();

        let state = state.read().await;
        let connections = state.list_connections(name_prefix);
        let entries: Vec<wire::Connection> = connections
            .iter()
            .map(|c| wire::Connection {
                name: Some(c.name.clone()),
                connection_arn: Some(c.arn.clone()),
                connection_state: Some(c.state.clone()),
                authorization_type: Some(c.authorization_type.clone()),
                creation_time: Some(c.creation_time),
                ..Default::default()
            })
            .collect();

        wire::serialize_list_connections_response(&wire::ListConnectionsResponse {
            connections: Some(entries),
            next_token: None,
        })
    }

    // --- ApiDestination operations ---

    async fn handle_create_api_destination(
        &self,
        state: &Arc<tokio::sync::RwLock<EventsState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_api_destination_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Name'");
        }
        if input.connection_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ConnectionArn'");
        }
        if input.invocation_endpoint.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'InvocationEndpoint'");
        }
        if input.http_method.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'HttpMethod'");
        }
        let description = input.description.as_deref();
        let invocation_rate_limit = input.invocation_rate_limit_per_second.map(i64::from);

        let mut state = state.write().await;
        match state.create_api_destination(
            &input.name,
            account_id,
            region,
            description,
            &input.connection_arn,
            &input.invocation_endpoint,
            &input.http_method,
            invocation_rate_limit,
        ) {
            Ok(dest) => wire::serialize_create_api_destination_response(
                &wire::CreateApiDestinationResponse {
                    api_destination_arn: Some(dest.arn.clone()),
                    api_destination_state: Some(dest.state.clone()),
                    creation_time: Some(dest.creation_time),
                    ..Default::default()
                },
            ),
            Err(e) => events_error_response(&e),
        }
    }

    async fn handle_delete_api_destination(
        &self,
        state: &Arc<tokio::sync::RwLock<EventsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_api_destination_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Name'");
        }

        let mut state = state.write().await;
        match state.delete_api_destination(&input.name) {
            Ok(()) => wire::serialize_delete_api_destination_response(
                &wire::DeleteApiDestinationResponse {},
            ),
            Err(e) => events_error_response(&e),
        }
    }

    async fn handle_describe_api_destination(
        &self,
        state: &Arc<tokio::sync::RwLock<EventsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_api_destination_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Name'");
        }

        let state = state.read().await;
        match state.describe_api_destination(&input.name) {
            Ok(dest) => wire::serialize_describe_api_destination_response(
                &wire::DescribeApiDestinationResponse {
                    name: Some(dest.name.clone()),
                    description: dest.description.clone(),
                    api_destination_arn: Some(dest.arn.clone()),
                    api_destination_state: Some(dest.state.clone()),
                    connection_arn: Some(dest.connection_arn.clone()),
                    invocation_endpoint: Some(dest.invocation_endpoint.clone()),
                    http_method: Some(dest.http_method.clone()),
                    invocation_rate_limit_per_second: Some(dest.invocation_rate_limit as i32),
                    creation_time: Some(dest.creation_time),
                    ..Default::default()
                },
            ),
            Err(e) => events_error_response(&e),
        }
    }

    async fn handle_update_api_destination(
        &self,
        state: &Arc<tokio::sync::RwLock<EventsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_api_destination_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Name'");
        }
        let description = input.description.as_deref();
        let connection_arn = input.connection_arn.as_deref();
        let invocation_endpoint = input.invocation_endpoint.as_deref();
        let http_method = input.http_method.as_deref();
        let invocation_rate_limit = input.invocation_rate_limit_per_second.map(i64::from);

        let mut state = state.write().await;
        match state.update_api_destination(
            &input.name,
            description,
            connection_arn,
            invocation_endpoint,
            http_method,
            invocation_rate_limit,
        ) {
            Ok(dest) => wire::serialize_update_api_destination_response(
                &wire::UpdateApiDestinationResponse {
                    api_destination_arn: Some(dest.arn.clone()),
                    api_destination_state: Some(dest.state.clone()),
                    creation_time: Some(dest.creation_time),
                    ..Default::default()
                },
            ),
            Err(e) => events_error_response(&e),
        }
    }

    async fn handle_list_api_destinations(
        &self,
        state: &Arc<tokio::sync::RwLock<EventsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_api_destinations_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let name_prefix = input.name_prefix.as_deref();

        let state = state.read().await;
        let dests = state.list_api_destinations(name_prefix);
        let entries: Vec<wire::ApiDestination> = dests
            .iter()
            .map(|d| wire::ApiDestination {
                name: Some(d.name.clone()),
                api_destination_arn: Some(d.arn.clone()),
                api_destination_state: Some(d.state.clone()),
                connection_arn: Some(d.connection_arn.clone()),
                invocation_endpoint: Some(d.invocation_endpoint.clone()),
                http_method: Some(d.http_method.clone()),
                invocation_rate_limit_per_second: Some(d.invocation_rate_limit as i32),
                creation_time: Some(d.creation_time),
                ..Default::default()
            })
            .collect();

        wire::serialize_list_api_destinations_response(&wire::ListApiDestinationsResponse {
            api_destinations: Some(entries),
            next_token: None,
        })
    }

    // --- Replay operations ---

    async fn handle_start_replay(
        &self,
        state: &Arc<tokio::sync::RwLock<EventsState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_start_replay_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.replay_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ReplayName'");
        }
        if input.event_source_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'EventSourceArn'");
        }
        let description = input.description.as_deref();
        let start_time = input.event_start_time;
        let end_time = input.event_end_time;
        let destination = input.destination.arn.as_str();

        let mut state = state.write().await;
        match state.start_replay(
            &input.replay_name,
            account_id,
            region,
            &input.event_source_arn,
            description,
            start_time,
            end_time,
            destination,
        ) {
            Ok(replay) => wire::serialize_start_replay_response(&wire::StartReplayResponse {
                replay_arn: Some(replay.arn.clone()),
                state: Some("STARTING".to_string()),
                replay_start_time: Some(replay.replay_start_time),
                ..Default::default()
            }),
            Err(e) => events_error_response(&e),
        }
    }

    async fn handle_cancel_replay(
        &self,
        state: &Arc<tokio::sync::RwLock<EventsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_cancel_replay_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.replay_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ReplayName'");
        }

        let mut state = state.write().await;
        match state.cancel_replay(&input.replay_name) {
            Ok(result) => wire::serialize_cancel_replay_response(&wire::CancelReplayResponse {
                replay_arn: Some(result.replay_arn),
                state: Some(result.state),
                ..Default::default()
            }),
            Err(e) => events_error_response(&e),
        }
    }

    async fn handle_describe_replay(
        &self,
        state: &Arc<tokio::sync::RwLock<EventsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_replay_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.replay_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ReplayName'");
        }

        let state = state.read().await;
        match state.describe_replay(&input.replay_name) {
            Ok(replay) => wire::serialize_describe_replay_response(&wire::DescribeReplayResponse {
                replay_name: Some(replay.name.clone()),
                replay_arn: Some(replay.arn.clone()),
                description: replay.description.clone(),
                event_source_arn: Some(replay.event_source_arn.clone()),
                state: Some(replay.state.clone()),
                event_start_time: Some(replay.start_time),
                event_end_time: Some(replay.end_time),
                replay_start_time: Some(replay.replay_start_time),
                replay_end_time: Some(replay.replay_end_time),
                destination: Some(wire::ReplayDestination {
                    arn: replay.destination.clone(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            Err(e) => events_error_response(&e),
        }
    }

    async fn handle_list_replays(
        &self,
        state: &Arc<tokio::sync::RwLock<EventsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_replays_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let name_prefix = input.name_prefix.as_deref();

        let state = state.read().await;
        let replays = state.list_replays(name_prefix);
        let entries: Vec<wire::Replay> = replays
            .iter()
            .map(|r| wire::Replay {
                replay_name: Some(r.name.clone()),
                event_source_arn: Some(r.event_source_arn.clone()),
                state: Some(r.state.clone()),
                event_start_time: Some(r.start_time),
                event_end_time: Some(r.end_time),
                replay_start_time: Some(r.replay_start_time),
                replay_end_time: Some(r.replay_end_time),
                ..Default::default()
            })
            .collect();

        wire::serialize_list_replays_response(&wire::ListReplaysResponse {
            replays: Some(entries),
            next_token: None,
        })
    }

    // --- PartnerEventSource operations ---

    async fn handle_create_partner_event_source(
        &self,
        state: &Arc<tokio::sync::RwLock<EventsState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_partner_event_source_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Name'");
        }

        let mut state = state.write().await;
        match state.create_partner_event_source(&input.name, account_id, region) {
            Ok(source) => wire::serialize_create_partner_event_source_response(
                &wire::CreatePartnerEventSourceResponse {
                    event_source_arn: Some(source.arn.clone()),
                },
            ),
            Err(e) => events_error_response(&e),
        }
    }

    async fn handle_delete_partner_event_source(
        &self,
        state: &Arc<tokio::sync::RwLock<EventsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_partner_event_source_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Name'");
        }

        let mut state = state.write().await;
        match state.delete_partner_event_source(&input.name) {
            Ok(()) => wire::serialize_delete_partner_event_source_response(),
            Err(e) => events_error_response(&e),
        }
    }

    async fn handle_describe_event_source(
        &self,
        state: &Arc<tokio::sync::RwLock<EventsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_event_source_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Name'");
        }

        let state = state.read().await;
        match state.describe_event_source(&input.name) {
            Ok(source) => {
                wire::serialize_describe_event_source_response(&wire::DescribeEventSourceResponse {
                    name: Some(source.name.clone()),
                    arn: Some(source.arn.clone()),
                    state: Some("ACTIVE".to_string()),
                    ..Default::default()
                })
            }
            Err(e) => events_error_response(&e),
        }
    }

    async fn handle_describe_partner_event_source(
        &self,
        state: &Arc<tokio::sync::RwLock<EventsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_partner_event_source_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Name'");
        }

        let state = state.read().await;
        match state.describe_partner_event_source(&input.name) {
            Ok(source) => wire::serialize_describe_partner_event_source_response(
                &wire::DescribePartnerEventSourceResponse {
                    name: Some(source.name.clone()),
                    arn: Some(source.arn.clone()),
                },
            ),
            Err(e) => events_error_response(&e),
        }
    }

    // --- Rule enable/disable ---

    async fn handle_disable_rule(
        &self,
        state: &Arc<tokio::sync::RwLock<EventsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_disable_rule_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Name'");
        }

        let mut state = state.write().await;
        match state.disable_rule(&input.name) {
            Ok(()) => wire::serialize_disable_rule_response(),
            Err(e) => events_error_response(&e),
        }
    }

    async fn handle_enable_rule(
        &self,
        state: &Arc<tokio::sync::RwLock<EventsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_enable_rule_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Name'");
        }

        let mut state = state.write().await;
        match state.enable_rule(&input.name) {
            Ok(()) => wire::serialize_enable_rule_response(),
            Err(e) => events_error_response(&e),
        }
    }

    // --- ListRuleNamesByTarget ---

    async fn handle_list_rule_names_by_target(
        &self,
        state: &Arc<tokio::sync::RwLock<EventsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_rule_names_by_target_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.target_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'TargetArn'");
        }

        let state = state.read().await;
        let rule_names = state.list_rule_names_by_target(&input.target_arn);

        wire::serialize_list_rule_names_by_target_response(&wire::ListRuleNamesByTargetResponse {
            rule_names: Some(rule_names),
            next_token: None,
        })
    }

    // --- Tag operations ---

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<EventsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_a_r_n.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ResourceARN'");
        }

        let tags: Vec<Tag> = input
            .tags
            .into_iter()
            .map(|t| Tag {
                key: t.key,
                value: t.value,
            })
            .collect();

        let mut state = state.write().await;
        state.tag_resource(&input.resource_a_r_n, tags);
        wire::serialize_tag_resource_response(&wire::TagResourceResponse {})
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<EventsState>>,
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
        state.untag_resource(&input.resource_a_r_n, &input.tag_keys);
        wire::serialize_untag_resource_response(&wire::UntagResourceResponse {})
    }

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<EventsState>>,
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
        let tags = state.list_tags_for_resource(&input.resource_a_r_n);
        let wire_tags: Vec<wire::Tag> = tags
            .iter()
            .map(|t| wire::Tag {
                key: t.key.clone(),
                value: t.value.clone(),
            })
            .collect();

        wire::serialize_list_tags_for_resource_response(&wire::ListTagsForResourceResponse {
            tags: Some(wire_tags),
        })
    }

    // --- PutPartnerEvents ---

    async fn handle_put_partner_events(
        &self,
        state: &Arc<tokio::sync::RwLock<EventsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_put_partner_events_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let entries: Vec<PutEventsEntry> = input
            .entries
            .into_iter()
            .map(|e| PutEventsEntry {
                source: e.source,
                detail_type: e.detail_type,
                detail: e.detail,
            })
            .collect();

        let state = state.read().await;
        let results = state.put_partner_events(&entries);

        let result_entries: Vec<wire::PutPartnerEventsResultEntry> = results
            .iter()
            .map(|r| wire::PutPartnerEventsResultEntry {
                event_id: Some(r.event_id.clone()),
                ..Default::default()
            })
            .collect();

        wire::serialize_put_partner_events_response(&wire::PutPartnerEventsResponse {
            failed_entry_count: Some(0),
            entries: Some(result_entries),
        })
    }

    // --- TestEventPattern ---

    async fn handle_test_event_pattern(
        &self,
        state: &Arc<tokio::sync::RwLock<EventsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_test_event_pattern_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.event_pattern.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'EventPattern'");
        }
        if input.event.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Event'");
        }

        let state = state.read().await;
        let result = state.test_event_pattern(&input.event_pattern, &input.event);

        wire::serialize_test_event_pattern_response(&wire::TestEventPatternResponse {
            result: Some(result),
        })
    }

    // --- ActivateEventSource / DeactivateEventSource ---

    async fn handle_activate_event_source(
        &self,
        state: &Arc<tokio::sync::RwLock<EventsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_activate_event_source_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Name'");
        }

        let mut state = state.write().await;
        match state.activate_event_source(&input.name) {
            Ok(()) => wire::serialize_activate_event_source_response(),
            Err(e) => events_error_response(&e),
        }
    }

    async fn handle_deactivate_event_source(
        &self,
        state: &Arc<tokio::sync::RwLock<EventsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_deactivate_event_source_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Name'");
        }

        let mut state = state.write().await;
        match state.deactivate_event_source(&input.name) {
            Ok(()) => wire::serialize_deactivate_event_source_response(),
            Err(e) => events_error_response(&e),
        }
    }

    // --- ListEventSources ---

    async fn handle_list_event_sources(
        &self,
        state: &Arc<tokio::sync::RwLock<EventsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_event_sources_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let name_prefix = input.name_prefix.as_deref();

        let state = state.read().await;
        let sources = state.list_event_sources(name_prefix);
        let entries: Vec<wire::EventSource> = sources
            .iter()
            .map(|s| wire::EventSource {
                name: Some(s.name.clone()),
                arn: Some(s.arn.clone()),
                state: Some(s.state.clone()),
                ..Default::default()
            })
            .collect();

        wire::serialize_list_event_sources_response(&wire::ListEventSourcesResponse {
            event_sources: Some(entries),
            next_token: None,
        })
    }

    // --- ListPartnerEventSourceAccounts ---

    async fn handle_list_partner_event_source_accounts(
        &self,
        _state: &Arc<tokio::sync::RwLock<EventsState>>,
        _body: &[u8],
    ) -> MockResponse {
        // Partner event source accounts require real Organizations cross-account state;
        // returns empty list in mock.
        wire::serialize_list_partner_event_source_accounts_response(
            &wire::ListPartnerEventSourceAccountsResponse {
                partner_event_source_accounts: Some(vec![]),
                next_token: None,
            },
        )
    }

    // --- ListPartnerEventSources ---

    async fn handle_list_partner_event_sources(
        &self,
        state: &Arc<tokio::sync::RwLock<EventsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_partner_event_sources_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let name_prefix = if input.name_prefix.is_empty() {
            None
        } else {
            Some(input.name_prefix.as_str())
        };

        let state = state.read().await;
        let sources = state.list_partner_event_sources(name_prefix);
        let entries: Vec<wire::PartnerEventSource> = sources
            .iter()
            .map(|s| wire::PartnerEventSource {
                name: Some(s.name.clone()),
                arn: Some(s.arn.clone()),
            })
            .collect();

        wire::serialize_list_partner_event_sources_response(
            &wire::ListPartnerEventSourcesResponse {
                partner_event_sources: Some(entries),
                next_token: None,
            },
        )
    }

    // --- DeauthorizeConnection ---

    async fn handle_deauthorize_connection(
        &self,
        state: &Arc<tokio::sync::RwLock<EventsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_deauthorize_connection_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Name'");
        }

        let mut state = state.write().await;
        match state.deauthorize_connection(&input.name) {
            Ok(conn) => wire::serialize_deauthorize_connection_response(
                &wire::DeauthorizeConnectionResponse {
                    connection_arn: Some(conn.arn.clone()),
                    connection_state: Some(conn.state.clone()),
                    creation_time: Some(conn.creation_time),
                    ..Default::default()
                },
            ),
            Err(e) => events_error_response(&e),
        }
    }

    // --- Endpoint operations ---

    async fn handle_create_endpoint(
        &self,
        state: &Arc<tokio::sync::RwLock<EventsState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_endpoint_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Name'");
        }
        let description = input.description.as_deref();
        let routing_config = serde_json::to_string(&input.routing_config).ok();
        let replication_config = input
            .replication_config
            .as_ref()
            .and_then(|r| serde_json::to_string(r).ok());
        let event_buses: Vec<String> = input
            .event_buses
            .into_iter()
            .filter(|b| !b.event_bus_arn.is_empty())
            .map(|b| b.event_bus_arn)
            .collect();
        let role_arn = input.role_arn.as_deref();

        let mut state = state.write().await;
        match state.create_endpoint(
            &input.name,
            account_id,
            region,
            description,
            routing_config.as_deref(),
            replication_config.as_deref(),
            event_buses,
            role_arn,
        ) {
            Ok(ep) => {
                let event_buses: Vec<wire::EndpointEventBus> = ep
                    .event_buses
                    .iter()
                    .map(|arn| wire::EndpointEventBus {
                        event_bus_arn: arn.clone(),
                    })
                    .collect();
                wire::serialize_create_endpoint_response(&wire::CreateEndpointResponse {
                    arn: Some(ep.arn.clone()),
                    name: Some(ep.name.clone()),
                    state: Some(ep.state.clone()),
                    event_buses: Some(event_buses),
                    role_arn: ep.role_arn.clone(),
                    ..Default::default()
                })
            }
            Err(e) => events_error_response(&e),
        }
    }

    async fn handle_delete_endpoint(
        &self,
        state: &Arc<tokio::sync::RwLock<EventsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_endpoint_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Name'");
        }

        let mut state = state.write().await;
        match state.delete_endpoint(&input.name) {
            Ok(()) => wire::serialize_delete_endpoint_response(&wire::DeleteEndpointResponse {}),
            Err(e) => events_error_response(&e),
        }
    }

    async fn handle_describe_endpoint(
        &self,
        state: &Arc<tokio::sync::RwLock<EventsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_endpoint_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Name'");
        }

        let state = state.read().await;
        match state.describe_endpoint(&input.name) {
            Ok(ep) => {
                let event_buses: Vec<wire::EndpointEventBus> = ep
                    .event_buses
                    .iter()
                    .map(|arn| wire::EndpointEventBus {
                        event_bus_arn: arn.clone(),
                    })
                    .collect();
                wire::serialize_describe_endpoint_response(&wire::DescribeEndpointResponse {
                    arn: Some(ep.arn.clone()),
                    name: Some(ep.name.clone()),
                    description: ep.description.clone(),
                    state: Some(ep.state.clone()),
                    endpoint_id: Some(ep.endpoint_id.clone()),
                    endpoint_url: Some(ep.endpoint_url.clone()),
                    event_buses: Some(event_buses),
                    role_arn: ep.role_arn.clone(),
                    creation_time: Some(ep.creation_time),
                    last_modified_time: Some(ep.last_modified_time),
                    ..Default::default()
                })
            }
            Err(e) => events_error_response(&e),
        }
    }

    async fn handle_list_endpoints(
        &self,
        state: &Arc<tokio::sync::RwLock<EventsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_endpoints_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let name_prefix = input.name_prefix.as_deref();

        let state = state.read().await;
        let endpoints = state.list_endpoints(name_prefix);
        let entries: Vec<wire::Endpoint> = endpoints
            .iter()
            .map(|ep| {
                let event_buses: Vec<wire::EndpointEventBus> = ep
                    .event_buses
                    .iter()
                    .map(|arn| wire::EndpointEventBus {
                        event_bus_arn: arn.clone(),
                    })
                    .collect();
                wire::Endpoint {
                    arn: Some(ep.arn.clone()),
                    name: Some(ep.name.clone()),
                    description: ep.description.clone(),
                    state: Some(ep.state.clone()),
                    endpoint_id: Some(ep.endpoint_id.clone()),
                    endpoint_url: Some(ep.endpoint_url.clone()),
                    event_buses: Some(event_buses),
                    role_arn: ep.role_arn.clone(),
                    creation_time: Some(ep.creation_time),
                    last_modified_time: Some(ep.last_modified_time),
                    ..Default::default()
                }
            })
            .collect();

        wire::serialize_list_endpoints_response(&wire::ListEndpointsResponse {
            endpoints: Some(entries),
            next_token: None,
        })
    }

    async fn handle_update_endpoint(
        &self,
        state: &Arc<tokio::sync::RwLock<EventsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_endpoint_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Name'");
        }
        let description = input.description.as_deref();
        let routing_config = input
            .routing_config
            .as_ref()
            .and_then(|r| serde_json::to_string(r).ok());
        let replication_config = input
            .replication_config
            .as_ref()
            .and_then(|r| serde_json::to_string(r).ok());
        let event_buses: Option<Vec<String>> = input.event_buses.map(|v| {
            v.into_iter()
                .filter(|b| !b.event_bus_arn.is_empty())
                .map(|b| b.event_bus_arn)
                .collect()
        });
        let role_arn = input.role_arn.as_deref();

        let mut state = state.write().await;
        match state.update_endpoint(
            &input.name,
            description,
            routing_config.as_deref(),
            replication_config.as_deref(),
            event_buses,
            role_arn,
        ) {
            Ok(ep) => {
                let event_buses: Vec<wire::EndpointEventBus> = ep
                    .event_buses
                    .iter()
                    .map(|arn| wire::EndpointEventBus {
                        event_bus_arn: arn.clone(),
                    })
                    .collect();
                wire::serialize_update_endpoint_response(&wire::UpdateEndpointResponse {
                    arn: Some(ep.arn.clone()),
                    name: Some(ep.name.clone()),
                    state: Some(ep.state.clone()),
                    endpoint_id: Some(ep.endpoint_id.clone()),
                    endpoint_url: Some(ep.endpoint_url.clone()),
                    event_buses: Some(event_buses),
                    role_arn: ep.role_arn.clone(),
                    ..Default::default()
                })
            }
            Err(e) => events_error_response(&e),
        }
    }

    // --- UpdateEventBus ---

    async fn handle_update_event_bus(
        &self,
        state: &Arc<tokio::sync::RwLock<EventsState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_update_event_bus_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let name = input.name.as_deref().unwrap_or("default");
        let description = input.description.as_deref();
        let kms_key_identifier = input.kms_key_identifier.as_deref();

        let mut state = state.write().await;
        state.ensure_default_bus(account_id, region);
        match state.update_event_bus(name, description, kms_key_identifier) {
            Ok(bus) => wire::serialize_update_event_bus_response(&wire::UpdateEventBusResponse {
                arn: Some(bus.arn.clone()),
                name: Some(bus.name.clone()),
                description: bus.description.clone(),
                kms_key_identifier: bus.kms_key_identifier.clone(),
                ..Default::default()
            }),
            Err(e) => events_error_response(&e),
        }
    }
}

// --- Utility functions ---

fn to_wire_rule(rule: &crate::types::Rule) -> wire::Rule {
    wire::Rule {
        name: Some(rule.name.clone()),
        arn: Some(rule.arn.clone()),
        state: Some(rule.state.as_str().to_string()),
        event_bus_name: Some(rule.event_bus_name.clone()),
        event_pattern: rule.event_pattern.clone(),
        schedule_expression: rule.schedule_expression.clone(),
        description: rule.description.clone(),
        ..Default::default()
    }
}

fn events_error_response(err: &EventsError) -> MockResponse {
    let (status, error_type) = match err {
        EventsError::RuleHasTargets => (400, "ValidationException"),
        EventsError::EventBusAlreadyExists(_) => (400, "ResourceAlreadyExistsException"),
        EventsError::CannotDeleteDefaultBus => (400, "ValidationException"),
        EventsError::InvalidPermissionAction => (400, "ValidationException"),
        EventsError::EventBusHasNoPolicy => (400, "ResourceNotFoundException"),
        EventsError::StatementNotFound => (400, "ResourceNotFoundException"),
        EventsError::ArchiveAlreadyExists(_) => (400, "ResourceAlreadyExistsException"),
        EventsError::ConnectionAlreadyExists(_) => (400, "ResourceAlreadyExistsException"),
        EventsError::ConnectionNotFound(_) => (400, "ResourceNotFoundException"),
        EventsError::ApiDestinationAlreadyExists(_) => (400, "ResourceAlreadyExistsException"),
        EventsError::ApiDestinationNotFound(_) => (400, "ResourceNotFoundException"),
        EventsError::ReplayAlreadyExists(_) => (400, "ResourceAlreadyExistsException"),
        EventsError::ReplayInvalidState(_) => (400, "IllegalStatusException"),
        EventsError::PartnerEventSourceAlreadyExists(_) => (400, "ResourceAlreadyExistsException"),
        EventsError::EndpointAlreadyExists(_) => (400, "ResourceAlreadyExistsException"),
        EventsError::ResourceNotFound(_, _) => (400, "ResourceNotFoundException"),
    };
    json_error_response(status, error_type, &err.to_string())
}

fn json_error_response(status: u16, code: &str, message: &str) -> MockResponse {
    let body = json!({
        "__type": code,
        "message": message,
    });
    MockResponse::json(status, body.to_string())
}
