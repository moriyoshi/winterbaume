use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, StatefulService,
    default_account_id,
};

use crate::model;
use crate::state::{ChatbotError, ChatbotState};
use crate::views::ChatbotStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct ChatbotService {
    pub(crate) state: Arc<BackendState<ChatbotState>>,
    pub(crate) notifier: StateChangeNotifier<ChatbotStateView>,
}

impl ChatbotService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for ChatbotService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for ChatbotService {
    fn service_name(&self) -> &str {
        "chatbot"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://chatbot\..*\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl ChatbotService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let method = request.method.as_str();

        #[allow(clippy::result_large_err)]
        let parse_json_body = |body: &[u8]| -> Result<Value, MockResponse> {
            if body.is_empty() {
                return Ok(json!({}));
            }
            serde_json::from_slice(body)
                .map_err(|_| rest_json_error(400, "InvalidRequestException", "Invalid JSON body"))
        };

        let body = match parse_json_body(&request.body) {
            Ok(v) => v,
            Err(e) => return e,
        };

        let response = match (method, path.as_str()) {
            // Slack
            ("POST", "/create-slack-channel-configuration") => {
                self.handle_create_slack_channel_configuration(&state, &body, account_id, &region)
                    .await
            }
            ("POST", "/describe-slack-channel-configurations") => {
                self.handle_describe_slack_channel_configurations(&state, &body)
                    .await
            }
            ("POST", "/update-slack-channel-configuration") => {
                self.handle_update_slack_channel_configuration(&state, &body)
                    .await
            }
            ("POST", "/delete-slack-channel-configuration") => {
                self.handle_delete_slack_channel_configuration(&state, &body)
                    .await
            }
            // Chime
            ("POST", "/create-chime-webhook-configuration") => {
                self.handle_create_chime_webhook_configuration(&state, &body, account_id, &region)
                    .await
            }
            ("POST", "/describe-chime-webhook-configurations") => {
                self.handle_describe_chime_webhook_configurations(&state, &body)
                    .await
            }
            ("POST", "/delete-chime-webhook-configuration") => {
                self.handle_delete_chime_webhook_configuration(&state, &body)
                    .await
            }
            // Teams
            ("POST", "/create-ms-teams-channel-configuration") => {
                self.handle_create_microsoft_teams_channel_configuration(
                    &state, &body, account_id, &region,
                )
                .await
            }
            ("POST", "/list-ms-teams-channel-configurations") => {
                self.handle_list_microsoft_teams_channel_configurations(&state, &body)
                    .await
            }
            ("POST", "/delete-ms-teams-channel-configuration") => {
                self.handle_delete_microsoft_teams_channel_configuration(&state, &body)
                    .await
            }
            // --- Unimplemented operations ---
            // --- Unimplemented operations (auto-generated stubs) ---
            // POST /associate-to-configuration => AssociateToConfiguration (not implemented)
            // POST /create-custom-action => CreateCustomAction (not implemented)
            // POST /delete-custom-action => DeleteCustomAction (not implemented)
            // POST /delete-ms-teams-configured-teams => DeleteMicrosoftTeamsConfiguredTeam (not implemented)
            // POST /delete-ms-teams-user-identity => DeleteMicrosoftTeamsUserIdentity (not implemented)
            // POST /delete-slack-user-identity => DeleteSlackUserIdentity (not implemented)
            // POST /delete-slack-workspace-authorization => DeleteSlackWorkspaceAuthorization (not implemented)
            // POST /describe-slack-user-identities => DescribeSlackUserIdentities (not implemented)
            // POST /describe-slack-workspaces => DescribeSlackWorkspaces (not implemented)
            // POST /disassociate-from-configuration => DisassociateFromConfiguration (not implemented)
            // POST /get-account-preferences => GetAccountPreferences (not implemented)
            // POST /get-custom-action => GetCustomAction (not implemented)
            // FIX(terraform-e2e): GetMicrosoftTeamsChannelConfiguration
            ("POST", "/get-ms-teams-channel-configuration") => {
                self.handle_get_microsoft_teams_channel_configuration(&state, &body)
                    .await
            }
            // FIX(terraform-e2e): UpdateMicrosoftTeamsChannelConfiguration
            ("POST", "/update-ms-teams-channel-configuration") => {
                self.handle_update_microsoft_teams_channel_configuration(&state, &body)
                    .await
            }
            // FIX(terraform-e2e): ListTagsForResource
            ("POST", "/list-tags-for-resource") => {
                self.handle_list_tags_for_resource(&state, &body).await
            }
            // FIX(terraform-e2e): TagResource
            ("POST", "/tag-resource") => self.handle_tag_resource(&state, &body).await,
            // FIX(terraform-e2e): UntagResource
            ("POST", "/untag-resource") => self.handle_untag_resource(&state, &body).await,
            // (GetMicrosoftTeamsChannelConfiguration now implemented above)
            // POST /list-associations => ListAssociations (not implemented)
            // POST /list-custom-actions => ListCustomActions (not implemented)
            // POST /list-ms-teams-configured-teams => ListMicrosoftTeamsConfiguredTeams (not implemented)
            // POST /list-ms-teams-user-identities => ListMicrosoftTeamsUserIdentities (not implemented)
            // (ListTagsForResource, TagResource, UntagResource now implemented above)
            // POST /update-account-preferences => UpdateAccountPreferences (not implemented)
            // POST /update-chime-webhook-configuration => UpdateChimeWebhookConfiguration (not implemented)
            // POST /update-custom-action => UpdateCustomAction (not implemented)
            // (GetMicrosoftTeamsChannelConfiguration, UpdateMicrosoftTeamsChannelConfiguration now implemented above)

            // 24 unimplemented operations above
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        };

        let is_mutating = (method == "POST")
            && !matches!(
                path.as_str(),
                "/describe-slack-channel-configurations"
                    | "/describe-chime-webhook-configurations"
                    | "/list-ms-teams-channel-configurations"
                    | "/get-ms-teams-channel-configuration"
                    | "/list-tags-for-resource"
            )
            && response.status / 100 == 2;
        if is_mutating {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_create_slack_channel_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<ChatbotState>>,
        body: &Value,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let req: model::CreateSlackChannelConfigurationRequest =
            match serde_json::from_value(body.clone()) {
                Ok(r) => r,
                Err(e) => return rest_json_error(400, "InvalidRequestException", &e.to_string()),
            };
        let tags = tags_from_model(req.tags.as_deref());
        let mut state = state.write().await;
        match state.create_slack_channel_configuration(
            account_id,
            region,
            &req.configuration_name,
            &req.slack_team_id,
            &req.slack_channel_id,
            req.slack_channel_name,
            &req.iam_role_arn,
            req.sns_topic_arns.unwrap_or_default(),
            req.logging_level,
            req.guardrail_policy_arns.unwrap_or_default(),
            req.user_authorization_required,
            tags,
        ) {
            Ok(config) => wire::serialize_create_slack_channel_configuration_response(
                &model::CreateSlackChannelConfigurationResult {
                    channel_configuration: Some(slack_config_to_model(&config)),
                },
            ),
            Err(e) => chatbot_error_response(&e),
        }
    }

    async fn handle_describe_slack_channel_configurations(
        &self,
        state: &Arc<tokio::sync::RwLock<ChatbotState>>,
        body: &Value,
    ) -> MockResponse {
        let filter_arn = body
            .get("ChatConfigurationArn")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let state = state.read().await;
        let configs: Vec<model::SlackChannelConfiguration> = state
            .describe_slack_channel_configurations(filter_arn.as_deref())
            .into_iter()
            .map(slack_config_to_model)
            .collect();
        wire::serialize_describe_slack_channel_configurations_response(
            &model::DescribeSlackChannelConfigurationsResult {
                slack_channel_configurations: Some(configs),
                ..Default::default()
            },
        )
    }

    async fn handle_update_slack_channel_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<ChatbotState>>,
        body: &Value,
    ) -> MockResponse {
        let req: model::UpdateSlackChannelConfigurationRequest =
            match serde_json::from_value(body.clone()) {
                Ok(r) => r,
                Err(e) => return rest_json_error(400, "InvalidRequestException", &e.to_string()),
            };
        let mut state = state.write().await;
        match state.update_slack_channel_configuration(
            &req.chat_configuration_arn,
            &req.slack_channel_id,
            req.slack_channel_name,
            req.iam_role_arn,
            req.sns_topic_arns,
            req.logging_level,
            req.guardrail_policy_arns,
            req.user_authorization_required,
        ) {
            Ok(config) => wire::serialize_update_slack_channel_configuration_response(
                &model::UpdateSlackChannelConfigurationResult {
                    channel_configuration: Some(slack_config_to_model(&config)),
                },
            ),
            Err(e) => chatbot_error_response(&e),
        }
    }

    async fn handle_delete_slack_channel_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<ChatbotState>>,
        body: &Value,
    ) -> MockResponse {
        let arn = match body.get("ChatConfigurationArn").and_then(|v| v.as_str()) {
            Some(a) => a.to_string(),
            None => {
                return rest_json_error(
                    400,
                    "InvalidRequestException",
                    "Missing 'ChatConfigurationArn'",
                );
            }
        };
        let mut state = state.write().await;
        match state.delete_slack_channel_configuration(&arn) {
            Ok(()) => wire::serialize_delete_slack_channel_configuration_response(
                &model::DeleteSlackChannelConfigurationResult {},
            ),
            Err(e) => chatbot_error_response(&e),
        }
    }

    async fn handle_create_chime_webhook_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<ChatbotState>>,
        body: &Value,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let req: model::CreateChimeWebhookConfigurationRequest =
            match serde_json::from_value(body.clone()) {
                Ok(r) => r,
                Err(e) => return rest_json_error(400, "InvalidRequestException", &e.to_string()),
            };
        let tags = tags_from_model(req.tags.as_deref());
        let mut state = state.write().await;
        match state.create_chime_webhook_configuration(
            account_id,
            region,
            &req.configuration_name,
            &req.webhook_url,
            &req.webhook_description,
            &req.iam_role_arn,
            req.sns_topic_arns,
            req.logging_level,
            tags,
        ) {
            Ok(config) => wire::serialize_create_chime_webhook_configuration_response(
                &model::CreateChimeWebhookConfigurationResult {
                    webhook_configuration: Some(chime_config_to_model(&config)),
                },
            ),
            Err(e) => chatbot_error_response(&e),
        }
    }

    async fn handle_describe_chime_webhook_configurations(
        &self,
        state: &Arc<tokio::sync::RwLock<ChatbotState>>,
        body: &Value,
    ) -> MockResponse {
        let filter_arn = body
            .get("ChatConfigurationArn")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let state = state.read().await;
        let configs: Vec<model::ChimeWebhookConfiguration> = state
            .describe_chime_webhook_configurations(filter_arn.as_deref())
            .into_iter()
            .map(chime_config_to_model)
            .collect();
        wire::serialize_describe_chime_webhook_configurations_response(
            &model::DescribeChimeWebhookConfigurationsResult {
                webhook_configurations: Some(configs),
                ..Default::default()
            },
        )
    }

    async fn handle_delete_chime_webhook_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<ChatbotState>>,
        body: &Value,
    ) -> MockResponse {
        let arn = match body.get("ChatConfigurationArn").and_then(|v| v.as_str()) {
            Some(a) => a.to_string(),
            None => {
                return rest_json_error(
                    400,
                    "InvalidRequestException",
                    "Missing 'ChatConfigurationArn'",
                );
            }
        };
        let mut state = state.write().await;
        match state.delete_chime_webhook_configuration(&arn) {
            Ok(()) => wire::serialize_delete_chime_webhook_configuration_response(
                &model::DeleteChimeWebhookConfigurationResult {},
            ),
            Err(e) => chatbot_error_response(&e),
        }
    }

    async fn handle_create_microsoft_teams_channel_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<ChatbotState>>,
        body: &Value,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let req: model::CreateTeamsChannelConfigurationRequest =
            match serde_json::from_value(body.clone()) {
                Ok(r) => r,
                Err(e) => return rest_json_error(400, "InvalidRequestException", &e.to_string()),
            };
        let tags = tags_from_model(req.tags.as_deref());
        let mut state = state.write().await;
        match state.create_microsoft_teams_channel_configuration(
            account_id,
            region,
            &req.configuration_name,
            &req.team_id,
            req.team_name,
            &req.tenant_id,
            &req.channel_id,
            req.channel_name,
            &req.iam_role_arn,
            req.sns_topic_arns.unwrap_or_default(),
            req.logging_level,
            req.guardrail_policy_arns.unwrap_or_default(),
            req.user_authorization_required,
            tags,
        ) {
            Ok(config) => wire::serialize_create_microsoft_teams_channel_configuration_response(
                &model::CreateTeamsChannelConfigurationResult {
                    channel_configuration: Some(teams_config_to_model(&config)),
                },
            ),
            Err(e) => chatbot_error_response(&e),
        }
    }

    async fn handle_list_microsoft_teams_channel_configurations(
        &self,
        state: &Arc<tokio::sync::RwLock<ChatbotState>>,
        body: &Value,
    ) -> MockResponse {
        let filter_arn = body
            .get("ChatConfigurationArn")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let state = state.read().await;
        let configs: Vec<model::TeamsChannelConfiguration> = state
            .list_microsoft_teams_channel_configurations(filter_arn.as_deref())
            .into_iter()
            .map(teams_config_to_model)
            .collect();
        wire::serialize_list_microsoft_teams_channel_configurations_response(
            &model::ListTeamsChannelConfigurationsResult {
                team_channel_configurations: Some(configs),
                ..Default::default()
            },
        )
    }

    async fn handle_delete_microsoft_teams_channel_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<ChatbotState>>,
        body: &Value,
    ) -> MockResponse {
        let arn = match body.get("ChatConfigurationArn").and_then(|v| v.as_str()) {
            Some(a) => a.to_string(),
            None => {
                return rest_json_error(
                    400,
                    "InvalidRequestException",
                    "Missing 'ChatConfigurationArn'",
                );
            }
        };
        let mut state = state.write().await;
        match state.delete_microsoft_teams_channel_configuration(&arn) {
            Ok(()) => wire::serialize_delete_microsoft_teams_channel_configuration_response(
                &model::DeleteTeamsChannelConfigurationResult {},
            ),
            Err(e) => chatbot_error_response(&e),
        }
    }

    // FIX(terraform-e2e) handlers below

    async fn handle_get_microsoft_teams_channel_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<ChatbotState>>,
        body: &Value,
    ) -> MockResponse {
        let req: model::GetTeamsChannelConfigurationRequest =
            match serde_json::from_value(body.clone()) {
                Ok(r) => r,
                Err(e) => return rest_json_error(400, "InvalidRequestException", &e.to_string()),
            };
        let state = state.read().await;
        match state.get_microsoft_teams_channel_configuration(&req.chat_configuration_arn) {
            Ok(config) => wire::serialize_get_microsoft_teams_channel_configuration_response(
                &model::GetTeamsChannelConfigurationResult {
                    channel_configuration: Some(teams_config_to_model(config)),
                },
            ),
            Err(e) => chatbot_error_response(&e),
        }
    }

    async fn handle_update_microsoft_teams_channel_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<ChatbotState>>,
        body: &Value,
    ) -> MockResponse {
        let req: model::UpdateTeamsChannelConfigurationRequest =
            match serde_json::from_value(body.clone()) {
                Ok(r) => r,
                Err(e) => return rest_json_error(400, "InvalidRequestException", &e.to_string()),
            };
        let mut state = state.write().await;
        match state.update_microsoft_teams_channel_configuration(
            &req.chat_configuration_arn,
            &req.channel_id,
            req.channel_name,
            req.iam_role_arn,
            req.sns_topic_arns,
            req.logging_level,
            req.guardrail_policy_arns,
            req.user_authorization_required,
        ) {
            Ok(config) => wire::serialize_update_microsoft_teams_channel_configuration_response(
                &model::UpdateTeamsChannelConfigurationResult {
                    channel_configuration: Some(teams_config_to_model(&config)),
                },
            ),
            Err(e) => chatbot_error_response(&e),
        }
    }

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<ChatbotState>>,
        body: &Value,
    ) -> MockResponse {
        let req: model::ListTagsForResourceRequest = match serde_json::from_value(body.clone()) {
            Ok(r) => r,
            Err(e) => return rest_json_error(400, "InvalidRequestException", &e.to_string()),
        };
        let state = state.read().await;
        match state.list_tags_for_resource(&req.resource_a_r_n) {
            Ok(tags) => {
                let tag_list: Vec<model::Tag> = tags
                    .into_iter()
                    .map(|(k, v)| model::Tag {
                        tag_key: k,
                        tag_value: v,
                    })
                    .collect();
                wire::serialize_list_tags_for_resource_response(
                    &model::ListTagsForResourceResponse {
                        tags: Some(tag_list),
                    },
                )
            }
            Err(e) => chatbot_error_response(&e),
        }
    }

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<ChatbotState>>,
        body: &Value,
    ) -> MockResponse {
        let req: model::TagResourceRequest = match serde_json::from_value(body.clone()) {
            Ok(r) => r,
            Err(e) => return rest_json_error(400, "InvalidRequestException", &e.to_string()),
        };
        let tags = tags_from_model(Some(&req.tags));
        let mut state = state.write().await;
        match state.tag_resource(&req.resource_a_r_n, tags) {
            Ok(()) => wire::serialize_tag_resource_response(&model::TagResourceResponse {}),
            Err(e) => chatbot_error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<ChatbotState>>,
        body: &Value,
    ) -> MockResponse {
        let req: model::UntagResourceRequest = match serde_json::from_value(body.clone()) {
            Ok(r) => r,
            Err(e) => return rest_json_error(400, "InvalidRequestException", &e.to_string()),
        };
        let mut state = state.write().await;
        match state.untag_resource(&req.resource_a_r_n, &req.tag_keys) {
            Ok(()) => wire::serialize_untag_resource_response(&model::UntagResourceResponse {}),
            Err(e) => chatbot_error_response(&e),
        }
    }
}

fn slack_config_to_model(c: &crate::types::SlackConfig) -> model::SlackChannelConfiguration {
    model::SlackChannelConfiguration {
        chat_configuration_arn: Some(c.arn.clone()),
        configuration_name: Some(c.configuration_name.clone()),
        slack_team_id: Some(c.slack_team_id.clone()),
        slack_channel_id: Some(c.slack_channel_id.clone()),
        slack_channel_name: c.slack_channel_name.clone(),
        iam_role_arn: Some(c.iam_role_arn.clone()),
        sns_topic_arns: Some(c.sns_topic_arns.clone()),
        logging_level: c.logging_level.clone(),
        guardrail_policy_arns: Some(c.guardrail_policy_arns.clone()),
        user_authorization_required: c.user_authorization_required,
        state: Some("ENABLED".to_string()),
        ..Default::default()
    }
}

fn chime_config_to_model(c: &crate::types::ChimeConfig) -> model::ChimeWebhookConfiguration {
    model::ChimeWebhookConfiguration {
        chat_configuration_arn: Some(c.arn.clone()),
        configuration_name: Some(c.configuration_name.clone()),
        iam_role_arn: Some(c.iam_role_arn.clone()),
        sns_topic_arns: Some(c.sns_topic_arns.clone()),
        logging_level: c.logging_level.clone(),
        webhook_description: Some(c.webhook_description.clone()),
        state: Some("ENABLED".to_string()),
        ..Default::default()
    }
}

fn teams_config_to_model(c: &crate::types::TeamsConfig) -> model::TeamsChannelConfiguration {
    model::TeamsChannelConfiguration {
        chat_configuration_arn: Some(c.arn.clone()),
        configuration_name: Some(c.configuration_name.clone()),
        team_id: Some(c.team_id.clone()),
        team_name: c.team_name.clone(),
        tenant_id: Some(c.tenant_id.clone()),
        channel_id: Some(c.channel_id.clone()),
        channel_name: c.channel_name.clone(),
        iam_role_arn: Some(c.iam_role_arn.clone()),
        sns_topic_arns: Some(c.sns_topic_arns.clone()),
        logging_level: c.logging_level.clone(),
        guardrail_policy_arns: Some(c.guardrail_policy_arns.clone()),
        user_authorization_required: c.user_authorization_required,
        state: Some("ENABLED".to_string()),
        ..Default::default()
    }
}

fn tags_from_model(tags: Option<&[model::Tag]>) -> std::collections::HashMap<String, String> {
    tags.unwrap_or_default()
        .iter()
        .map(|t| (t.tag_key.clone(), t.tag_value.clone()))
        .collect()
}

fn chatbot_error_response(e: &ChatbotError) -> MockResponse {
    let (status, error_type) = match e {
        ChatbotError::ConfigurationNotFound(_) => (404, "InvalidRequestException"),
        ChatbotError::ResourceNotFound(_) => (404, "ResourceNotFoundException"),
    };
    rest_json_error(status, error_type, &e.to_string())
}

fn extract_path(uri: &str) -> String {
    if let Some(idx) = uri.find("amazonaws.com") {
        let after_host = &uri[idx + "amazonaws.com".len()..];
        if let Some(q) = after_host.find('?') {
            after_host[..q].to_string()
        } else {
            after_host.to_string()
        }
    } else {
        uri.split('?').next().unwrap_or(uri).to_string()
    }
}

fn rest_json_error(status: u16, code: &str, message: &str) -> MockResponse {
    let body = json!({
        "Message": message,
    });
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers.insert(X_AMZN_ERRORTYPE, code.parse().unwrap());
    resp
}
