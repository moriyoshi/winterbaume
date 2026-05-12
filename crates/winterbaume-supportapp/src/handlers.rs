use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, default_account_id,
    extract_path, rest_json_error,
};

use crate::state::{ChannelConfig, SupportAppError, SupportAppState};
use crate::views::SupportAppStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct SupportAppService {
    pub(crate) state: Arc<BackendState<SupportAppState>>,
    pub(crate) notifier: StateChangeNotifier<SupportAppStateView>,
}

impl SupportAppService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for SupportAppService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for SupportAppService {
    fn service_name(&self) -> &str {
        "supportapp"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://supportapp\..*\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

type SharedState = Arc<tokio::sync::RwLock<SupportAppState>>;

impl SupportAppService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let path = path.trim_start_matches('/');
        let path = path.trim_start_matches("control/");

        let body: Value = if request.body.is_empty() {
            json!({})
        } else {
            match serde_json::from_slice(&request.body) {
                Ok(v) => v,
                Err(_) => return rest_json_error(400, "BadRequestException", "Invalid JSON body"),
            }
        };

        let response = match path {
            "create-slack-channel-configuration" => self.handle_create_channel(&state, &body).await,
            "delete-slack-channel-configuration" => self.handle_delete_channel(&state, &body).await,
            "delete-slack-workspace-configuration" => {
                self.handle_delete_workspace(&state, &body).await
            }
            "get-account-alias" => self.handle_get_account_alias(&state).await,
            "list-slack-channel-configurations" => self.handle_list_channels(&state).await,
            "list-slack-workspace-configurations" => self.handle_list_workspaces(&state).await,
            "put-account-alias" => self.handle_put_account_alias(&state, &body).await,
            "delete-account-alias" => self.handle_delete_account_alias(&state).await,
            "register-slack-workspace-for-organization" => {
                self.handle_register_workspace(&state, &body).await
            }
            "update-slack-channel-configuration" => self.handle_update_channel(&state, &body).await,
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        };

        use winterbaume_core::StatefulService;
        let read_only = matches!(
            path,
            "get-account-alias"
                | "list-slack-channel-configurations"
                | "list-slack-workspace-configurations"
        );
        if response.status / 100 == 2 && !read_only {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_create_channel(&self, state: &SharedState, body: &Value) -> MockResponse {
        let channel_id = match require_str(body, "channelId") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let team_id = match require_str(body, "teamId") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let channel_role_arn = match require_str(body, "channelRoleArn") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let notify_on_case_severity = match require_str(body, "notifyOnCaseSeverity") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let cfg = ChannelConfig {
            channel_id,
            channel_name: body
                .get("channelName")
                .and_then(|v| v.as_str())
                .map(String::from),
            channel_role_arn,
            team_id,
            notify_on_case_severity,
            notify_on_add_correspondence_to_case: body
                .get("notifyOnAddCorrespondenceToCase")
                .and_then(|v| v.as_bool()),
            notify_on_create_or_reopen_case: body
                .get("notifyOnCreateOrReopenCase")
                .and_then(|v| v.as_bool()),
            notify_on_resolve_case: body.get("notifyOnResolveCase").and_then(|v| v.as_bool()),
        };
        let mut state = state.write().await;
        match state.create_channel(cfg) {
            Ok(_) => wire::serialize_create_slack_channel_configuration_response(
                &wire::CreateSlackChannelConfigurationResult {},
            ),
            Err(e) => err_response(&e),
        }
    }

    async fn handle_delete_channel(&self, state: &SharedState, body: &Value) -> MockResponse {
        let channel_id = match require_str(body, "channelId") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let team_id = match require_str(body, "teamId") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let mut state = state.write().await;
        match state.delete_channel(&team_id, &channel_id) {
            Ok(()) => wire::serialize_delete_slack_channel_configuration_response(
                &wire::DeleteSlackChannelConfigurationResult {},
            ),
            Err(e) => err_response(&e),
        }
    }

    async fn handle_delete_workspace(&self, state: &SharedState, body: &Value) -> MockResponse {
        let team_id = match require_str(body, "teamId") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let mut state = state.write().await;
        match state.delete_workspace(&team_id) {
            Ok(()) => wire::serialize_delete_slack_workspace_configuration_response(
                &wire::DeleteSlackWorkspaceConfigurationResult {},
            ),
            Err(e) => err_response(&e),
        }
    }

    async fn handle_list_channels(&self, state: &SharedState) -> MockResponse {
        let state = state.read().await;
        let configs: Vec<wire::SlackChannelConfiguration> = state
            .list_channels()
            .into_iter()
            .map(|c| wire::SlackChannelConfiguration {
                channel_id: Some(c.channel_id.clone()),
                channel_name: c.channel_name.clone(),
                channel_role_arn: Some(c.channel_role_arn.clone()),
                notify_on_add_correspondence_to_case: c.notify_on_add_correspondence_to_case,
                notify_on_case_severity: Some(c.notify_on_case_severity.clone()),
                notify_on_create_or_reopen_case: c.notify_on_create_or_reopen_case,
                notify_on_resolve_case: c.notify_on_resolve_case,
                team_id: Some(c.team_id.clone()),
            })
            .collect();
        wire::serialize_list_slack_channel_configurations_response(
            &wire::ListSlackChannelConfigurationsResult {
                next_token: None,
                slack_channel_configurations: Some(configs),
            },
        )
    }

    async fn handle_list_workspaces(&self, state: &SharedState) -> MockResponse {
        let state = state.read().await;
        let configs: Vec<wire::SlackWorkspaceConfiguration> = state
            .list_workspaces()
            .into_iter()
            .map(|w| wire::SlackWorkspaceConfiguration {
                allow_organization_member_account: w.allow_organization_member_account,
                team_id: Some(w.team_id.clone()),
                team_name: w.team_name.clone(),
            })
            .collect();
        wire::serialize_list_slack_workspace_configurations_response(
            &wire::ListSlackWorkspaceConfigurationsResult {
                next_token: None,
                slack_workspace_configurations: Some(configs),
            },
        )
    }

    async fn handle_get_account_alias(&self, state: &SharedState) -> MockResponse {
        let state = state.read().await;
        wire::serialize_get_account_alias_response(&wire::GetAccountAliasResult {
            account_alias: state.account_alias.clone(),
        })
    }

    async fn handle_put_account_alias(&self, state: &SharedState, body: &Value) -> MockResponse {
        let alias = match require_str(body, "accountAlias") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let mut state = state.write().await;
        state.put_account_alias(alias);
        wire::serialize_put_account_alias_response(&wire::PutAccountAliasResult {})
    }

    async fn handle_delete_account_alias(&self, state: &SharedState) -> MockResponse {
        let mut state = state.write().await;
        state.delete_account_alias();
        wire::serialize_delete_account_alias_response(&wire::DeleteAccountAliasResult {})
    }

    async fn handle_register_workspace(&self, state: &SharedState, body: &Value) -> MockResponse {
        let team_id = match require_str(body, "teamId") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let mut state = state.write().await;
        let ws = state.register_workspace_for_organization(&team_id);
        wire::serialize_register_slack_workspace_for_organization_response(
            &wire::RegisterSlackWorkspaceForOrganizationResult {
                account_type: Some("PAYER".to_string()),
                team_id: Some(ws.team_id.clone()),
                team_name: ws.team_name.clone(),
            },
        )
    }

    async fn handle_update_channel(&self, state: &SharedState, body: &Value) -> MockResponse {
        let channel_id = match require_str(body, "channelId") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let team_id = match require_str(body, "teamId") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let new_name = body
            .get("channelName")
            .and_then(|v| v.as_str())
            .map(String::from);
        let new_role = body
            .get("channelRoleArn")
            .and_then(|v| v.as_str())
            .map(String::from);
        let new_severity = body
            .get("notifyOnCaseSeverity")
            .and_then(|v| v.as_str())
            .map(String::from);
        let new_add = body
            .get("notifyOnAddCorrespondenceToCase")
            .and_then(|v| v.as_bool());
        let new_create = body
            .get("notifyOnCreateOrReopenCase")
            .and_then(|v| v.as_bool());
        let new_resolve = body.get("notifyOnResolveCase").and_then(|v| v.as_bool());
        let mut state = state.write().await;
        match state.update_channel(&team_id, &channel_id, |c| {
            if let Some(n) = new_name {
                c.channel_name = Some(n);
            }
            if let Some(r) = new_role {
                c.channel_role_arn = r;
            }
            if let Some(s) = new_severity {
                c.notify_on_case_severity = s;
            }
            if let Some(b) = new_add {
                c.notify_on_add_correspondence_to_case = Some(b);
            }
            if let Some(b) = new_create {
                c.notify_on_create_or_reopen_case = Some(b);
            }
            if let Some(b) = new_resolve {
                c.notify_on_resolve_case = Some(b);
            }
        }) {
            Ok(c) => wire::serialize_update_slack_channel_configuration_response(
                &wire::UpdateSlackChannelConfigurationResult {
                    channel_id: Some(c.channel_id.clone()),
                    channel_name: c.channel_name.clone(),
                    channel_role_arn: Some(c.channel_role_arn.clone()),
                    notify_on_add_correspondence_to_case: c.notify_on_add_correspondence_to_case,
                    notify_on_case_severity: Some(c.notify_on_case_severity.clone()),
                    notify_on_create_or_reopen_case: c.notify_on_create_or_reopen_case,
                    notify_on_resolve_case: c.notify_on_resolve_case,
                    team_id: Some(c.team_id.clone()),
                },
            ),
            Err(e) => err_response(&e),
        }
    }
}

fn require_str(body: &Value, field: &str) -> Result<String, MockResponse> {
    body.get(field)
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .map(String::from)
        .ok_or_else(|| rest_json_error(400, "ValidationException", &format!("{field} is required")))
}

fn err_response(err: &SupportAppError) -> MockResponse {
    let (status, error_type) = match err {
        SupportAppError::ChannelNotFound { .. } | SupportAppError::WorkspaceNotFound { .. } => {
            (404, "ResourceNotFoundException")
        }
        SupportAppError::ChannelAlreadyExists { .. } => (409, "ConflictException"),
        SupportAppError::Validation { .. } => (400, "ValidationException"),
    };
    let body = json!({"message": err.to_string()});
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}
