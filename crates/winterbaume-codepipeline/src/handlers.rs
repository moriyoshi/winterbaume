use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, default_account_id,
    json_error_response,
};

use crate::state::{CodePipelineError, CodePipelineState};
use crate::types::ArtifactDetailsData;
use crate::views::CodePipelineStateView;
use crate::wire;

pub struct CodePipelineService {
    pub(crate) state: Arc<BackendState<CodePipelineState>>,
    pub(crate) notifier: StateChangeNotifier<CodePipelineStateView>,
}

impl CodePipelineService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for CodePipelineService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for CodePipelineService {
    fn service_name(&self) -> &str {
        "codepipeline"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://codepipeline\..*\.amazonaws\.com",
            r"https?://codepipeline\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl CodePipelineService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();

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

        match action.as_str() {
            "CreatePipeline" => {
                self.handle_create_pipeline(&state, body_bytes, account_id, &region)
                    .await
            }
            "GetPipeline" => self.handle_get_pipeline(&state, body_bytes).await,
            "DeletePipeline" => self.handle_delete_pipeline(&state, body_bytes).await,
            "ListPipelines" => self.handle_list_pipelines(&state).await,
            "AcknowledgeJob" => self.handle_acknowledge_job(&state, body_bytes).await,
            "AcknowledgeThirdPartyJob" => {
                self.handle_acknowledge_third_party_job(&state, body_bytes)
                    .await
            }
            "CreateCustomActionType" => {
                self.handle_create_custom_action_type(&state, body_bytes)
                    .await
            }
            "DeleteCustomActionType" => {
                self.handle_delete_custom_action_type(&state, body_bytes)
                    .await
            }
            "DeleteWebhook" => self.handle_delete_webhook(&state, body_bytes).await,
            "DeregisterWebhookWithThirdParty" => {
                self.handle_deregister_webhook_with_third_party(&state, body_bytes)
                    .await
            }
            "DisableStageTransition" => {
                self.handle_disable_stage_transition(&state, body_bytes)
                    .await
            }
            "EnableStageTransition" => {
                self.handle_enable_stage_transition(&state, body_bytes)
                    .await
            }
            "GetActionType" => self.handle_get_action_type(&state, body_bytes).await,
            "GetJobDetails" => self.handle_get_job_details(&state, body_bytes).await,
            "GetPipelineExecution" => self.handle_get_pipeline_execution(&state, body_bytes).await,
            "GetPipelineState" => self.handle_get_pipeline_state(&state, body_bytes).await,
            "GetThirdPartyJobDetails" => {
                self.handle_get_third_party_job_details(&state, body_bytes)
                    .await
            }
            "ListActionExecutions" => self.handle_list_action_executions(&state, body_bytes).await,
            "ListActionTypes" => self.handle_list_action_types(&state, body_bytes).await,
            "ListDeployActionExecutionTargets" => {
                self.handle_list_deploy_action_execution_targets(&state, body_bytes)
                    .await
            }
            "ListPipelineExecutions" => {
                self.handle_list_pipeline_executions(&state, body_bytes)
                    .await
            }
            "ListRuleExecutions" => self.handle_list_rule_executions(&state, body_bytes).await,
            "ListRuleTypes" => self.handle_list_rule_types(&state).await,
            "ListTagsForResource" => {
                self.handle_list_tags_for_resource(&state, body_bytes, account_id)
                    .await
            }
            "ListWebhooks" => self.handle_list_webhooks(&state).await,
            "OverrideStageCondition" => {
                self.handle_override_stage_condition(&state, body_bytes)
                    .await
            }
            "PollForJobs" => self.handle_poll_for_jobs(&state, body_bytes).await,
            "PollForThirdPartyJobs" => self.handle_poll_for_third_party_jobs(&state).await,
            "PutActionRevision" => self.handle_put_action_revision(&state, body_bytes).await,
            "PutApprovalResult" => self.handle_put_approval_result(&state, body_bytes).await,
            "PutJobFailureResult" => self.handle_put_job_failure_result(&state, body_bytes).await,
            "PutJobSuccessResult" => self.handle_put_job_success_result(&state, body_bytes).await,
            "PutThirdPartyJobFailureResult" => {
                self.handle_put_third_party_job_failure_result(&state).await
            }
            "PutThirdPartyJobSuccessResult" => {
                self.handle_put_third_party_job_success_result(&state).await
            }
            "PutWebhook" => {
                self.handle_put_webhook(&state, body_bytes, account_id, &region)
                    .await
            }
            "RegisterWebhookWithThirdParty" => {
                self.handle_register_webhook_with_third_party(&state, body_bytes)
                    .await
            }
            "RetryStageExecution" => self.handle_retry_stage_execution(&state, body_bytes).await,
            "RollbackStage" => self.handle_rollback_stage(&state, body_bytes).await,
            "StartPipelineExecution" => {
                self.handle_start_pipeline_execution(&state, body_bytes)
                    .await
            }
            "StopPipelineExecution" => {
                self.handle_stop_pipeline_execution(&state, body_bytes)
                    .await
            }
            "TagResource" => {
                self.handle_tag_resource(&state, body_bytes, account_id)
                    .await
            }
            "UntagResource" => {
                self.handle_untag_resource(&state, body_bytes, account_id)
                    .await
            }
            "UpdateActionType" => self.handle_update_action_type(&state, body_bytes).await,
            "UpdatePipeline" => {
                self.handle_update_pipeline(&state, body_bytes, account_id)
                    .await
            }
            _ => json_error_response(400, "InvalidAction", &format!("Unknown operation {action}")),
        }
    }

    async fn handle_create_pipeline(
        &self,
        state: &Arc<tokio::sync::RwLock<CodePipelineState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_pipeline_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.pipeline.name.is_empty() {
            return json_error_response(
                400,
                "InvalidStructureException",
                "pipeline.name is required",
            );
        }
        let name = input.pipeline.name.as_str();
        let role_arn = input.pipeline.role_arn.as_str();
        let stages = serde_json::to_value(&input.pipeline.stages).unwrap_or(json!([]));

        // Extract top-level tags from the request body (not from pipeline sub-object)
        let initial_tags: HashMap<String, String> = input
            .tags
            .unwrap_or_default()
            .into_iter()
            .map(|t| (t.key, t.value))
            .collect();

        let mut state = state.write().await;
        match state.create_pipeline(name, role_arn, stages, account_id, region, initial_tags) {
            Ok(p) => wire::serialize_create_pipeline_response(&wire::CreatePipelineOutput {
                pipeline: Some(pipeline_to_declaration(p)),
                tags: None,
            }),
            Err(e) => codepipeline_error_response(&e),
        }
    }

    async fn handle_get_pipeline(
        &self,
        state: &Arc<tokio::sync::RwLock<CodePipelineState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_pipeline_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "name is required");
        }
        let name = input.name.as_str();

        let state = state.read().await;
        match state.get_pipeline(name) {
            Ok(p) => wire::serialize_get_pipeline_response(&wire::GetPipelineOutput {
                pipeline: Some(pipeline_to_declaration(p)),
                metadata: None,
            }),
            Err(e) => codepipeline_error_response(&e),
        }
    }

    async fn handle_delete_pipeline(
        &self,
        state: &Arc<tokio::sync::RwLock<CodePipelineState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_pipeline_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "name is required");
        }

        let mut state = state.write().await;
        state.delete_pipeline(&input.name);
        wire::serialize_delete_pipeline_response()
    }

    async fn handle_list_pipelines(
        &self,
        state: &Arc<tokio::sync::RwLock<CodePipelineState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let pipelines = state.list_pipelines();
        let entries: Vec<wire::PipelineSummary> = pipelines
            .iter()
            .map(|p| wire::PipelineSummary {
                name: Some(p.name.clone()),
                version: Some(p.version),
                created: Some(p.created.timestamp() as f64),
                updated: Some(p.updated.timestamp() as f64),
                ..Default::default()
            })
            .collect();

        wire::serialize_list_pipelines_response(&wire::ListPipelinesOutput {
            pipelines: Some(entries),
            next_token: None,
        })
    }

    async fn handle_update_pipeline(
        &self,
        state: &Arc<tokio::sync::RwLock<CodePipelineState>>,
        body: &[u8],
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_update_pipeline_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.pipeline.name.is_empty() {
            return json_error_response(
                400,
                "InvalidStructureException",
                "pipeline.name is required",
            );
        }
        let name = input.pipeline.name.as_str();
        let role_arn = if input.pipeline.role_arn.is_empty() {
            None
        } else {
            Some(input.pipeline.role_arn.as_str())
        };
        let stages = serde_json::to_value(&input.pipeline.stages).ok();

        let mut state = state.write().await;
        match state.update_pipeline(name, role_arn, stages, account_id) {
            Ok(p) => wire::serialize_update_pipeline_response(&wire::UpdatePipelineOutput {
                pipeline: Some(pipeline_to_declaration(p)),
            }),
            Err(e) => codepipeline_error_response(&e),
        }
    }

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<CodePipelineState>>,
        body: &[u8],
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(400, "ValidationException", "resourceArn is required");
        }
        let tags: HashMap<String, String> =
            input.tags.into_iter().map(|t| (t.key, t.value)).collect();

        let mut state = state.write().await;
        match state.tag_resource(&input.resource_arn, tags, account_id) {
            Ok(()) => wire::serialize_tag_resource_response(&wire::TagResourceOutput {}),
            Err(e) => codepipeline_error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<CodePipelineState>>,
        body: &[u8],
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(400, "ValidationException", "resourceArn is required");
        }

        let mut state = state.write().await;
        match state.untag_resource(&input.resource_arn, &input.tag_keys, account_id) {
            Ok(()) => wire::serialize_untag_resource_response(&wire::UntagResourceOutput {}),
            Err(e) => codepipeline_error_response(&e),
        }
    }

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<CodePipelineState>>,
        body: &[u8],
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(400, "ValidationException", "resourceArn is required");
        }

        let state = state.read().await;
        match state.list_tags_for_resource(&input.resource_arn, account_id) {
            Ok(tags) => {
                let tag_list: Vec<wire::Tag> = tags
                    .iter()
                    .map(|(k, v)| wire::Tag {
                        key: k.clone(),
                        value: v.clone(),
                    })
                    .collect();
                wire::serialize_list_tags_for_resource_response(&wire::ListTagsForResourceOutput {
                    tags: Some(tag_list),
                    next_token: None,
                })
            }
            Err(e) => codepipeline_error_response(&e),
        }
    }

    // ---- Job acknowledgement ----

    async fn handle_acknowledge_job(
        &self,
        state: &Arc<tokio::sync::RwLock<CodePipelineState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_acknowledge_job_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.job_id.is_empty() {
            return json_error_response(400, "ValidationException", "jobId is required");
        }
        if input.nonce.is_empty() {
            return json_error_response(400, "ValidationException", "nonce is required");
        }

        let mut state = state.write().await;
        match state.acknowledge_job(&input.job_id, &input.nonce) {
            Ok(status) => wire::serialize_acknowledge_job_response(&wire::AcknowledgeJobOutput {
                status: Some(status.to_string()),
            }),
            Err(e) => codepipeline_error_response(&e),
        }
    }

    // Third-party job acknowledgement uses the same job store. The third-party
    // concept is essentially the same as regular jobs from a mock perspective.
    async fn handle_acknowledge_third_party_job(
        &self,
        state: &Arc<tokio::sync::RwLock<CodePipelineState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_acknowledge_third_party_job_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.job_id.is_empty() {
            return json_error_response(400, "ValidationException", "jobId is required");
        }
        if input.nonce.is_empty() {
            return json_error_response(400, "ValidationException", "nonce is required");
        }

        let mut state = state.write().await;
        match state.acknowledge_job(&input.job_id, &input.nonce) {
            Ok(status) => wire::serialize_acknowledge_third_party_job_response(
                &wire::AcknowledgeThirdPartyJobOutput {
                    status: Some(status.to_string()),
                },
            ),
            Err(e) => codepipeline_error_response(&e),
        }
    }

    // ---- Custom Action Types ----

    async fn handle_create_custom_action_type(
        &self,
        state: &Arc<tokio::sync::RwLock<CodePipelineState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_create_custom_action_type_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.category.is_empty() {
            return json_error_response(400, "ValidationException", "category is required");
        }
        if input.provider.is_empty() {
            return json_error_response(400, "ValidationException", "provider is required");
        }
        if input.version.is_empty() {
            return json_error_response(400, "ValidationException", "version is required");
        }

        let settings = input
            .settings
            .as_ref()
            .and_then(|v| serde_json::to_value(v).ok());
        let config_props = input
            .configuration_properties
            .as_ref()
            .and_then(|v| serde_json::to_value(v).ok());

        let input_artifact_details = ArtifactDetailsData {
            minimum_count: input.input_artifact_details.minimum_count,
            maximum_count: input.input_artifact_details.maximum_count,
        };
        let output_artifact_details = ArtifactDetailsData {
            minimum_count: input.output_artifact_details.minimum_count,
            maximum_count: input.output_artifact_details.maximum_count,
        };

        let tags: HashMap<String, String> = input
            .tags
            .unwrap_or_default()
            .into_iter()
            .map(|t| (t.key, t.value))
            .collect();

        let mut state = state.write().await;
        match state.create_custom_action_type(
            &input.category,
            &input.provider,
            &input.version,
            settings,
            config_props,
            input_artifact_details,
            output_artifact_details,
            tags,
        ) {
            Ok(a) => wire::serialize_create_custom_action_type_response(
                &wire::CreateCustomActionTypeOutput {
                    action_type: Some(custom_action_to_wire(a)),
                    tags: None,
                },
            ),
            Err(e) => codepipeline_error_response(&e),
        }
    }

    async fn handle_delete_custom_action_type(
        &self,
        state: &Arc<tokio::sync::RwLock<CodePipelineState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_custom_action_type_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };

        let mut state = state.write().await;
        match state.delete_custom_action_type(&input.category, &input.provider, &input.version) {
            Ok(()) => wire::serialize_delete_custom_action_type_response(),
            Err(e) => codepipeline_error_response(&e),
        }
    }

    async fn handle_get_action_type(
        &self,
        state: &Arc<tokio::sync::RwLock<CodePipelineState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_action_type_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let category = input.category.as_str();
        let provider = input.provider.as_str();
        let version = input.version.as_str();

        // GetActionType (v2-style) uses ActionTypeDeclaration. For custom types,
        // owner must be "Custom".
        let _ = input.owner.as_str();
        let state = state.read().await;
        match state.get_custom_action_type(category, provider, version) {
            Ok(a) => {
                let decl = wire::ActionTypeDeclaration {
                    description: None,
                    executor: Default::default(),
                    id: wire::ActionTypeIdentifier {
                        category: category.to_string(),
                        owner: "Custom".to_string(),
                        provider: provider.to_string(),
                        version: version.to_string(),
                    },
                    input_artifact_details: wire::ActionTypeArtifactDetails {
                        minimum_count: a.input_artifact_details.minimum_count,
                        maximum_count: a.input_artifact_details.maximum_count,
                    },
                    output_artifact_details: wire::ActionTypeArtifactDetails {
                        minimum_count: a.output_artifact_details.minimum_count,
                        maximum_count: a.output_artifact_details.maximum_count,
                    },
                    permissions: None,
                    properties: None,
                    urls: None,
                };
                wire::serialize_get_action_type_response(&wire::GetActionTypeOutput {
                    action_type: Some(decl),
                })
            }
            Err(e) => codepipeline_error_response(&e),
        }
    }

    async fn handle_list_action_types(
        &self,
        state: &Arc<tokio::sync::RwLock<CodePipelineState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_action_types_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let owner_filter = input.action_owner_filter.as_deref();

        let state = state.read().await;
        let action_types = state.list_custom_action_types(owner_filter);
        let entries: Vec<wire::ActionType> = action_types
            .iter()
            .map(|a| custom_action_to_wire(a))
            .collect();

        wire::serialize_list_action_types_response(&wire::ListActionTypesOutput {
            action_types: Some(entries),
            next_token: None,
        })
    }

    async fn handle_update_action_type(
        &self,
        state: &Arc<tokio::sync::RwLock<CodePipelineState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_action_type_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let action_type = &input.action_type;
        let category = action_type.id.category.as_str();
        let provider = action_type.id.provider.as_str();
        let version = action_type.id.version.as_str();

        let input_artifact_details = ArtifactDetailsData {
            minimum_count: action_type.input_artifact_details.minimum_count,
            maximum_count: action_type.input_artifact_details.maximum_count,
        };
        let output_artifact_details = ArtifactDetailsData {
            minimum_count: action_type.output_artifact_details.minimum_count,
            maximum_count: action_type.output_artifact_details.maximum_count,
        };

        // ActionTypeDeclaration does not carry `settings` and uses `properties` for
        // configuration. Preserve the prior shape: settings is None, config from properties.
        let settings = None;
        let config_props = action_type
            .properties
            .as_ref()
            .and_then(|v| serde_json::to_value(v).ok());

        let mut state = state.write().await;
        match state.update_action_type(
            category,
            provider,
            version,
            settings,
            config_props,
            input_artifact_details,
            output_artifact_details,
        ) {
            Ok(()) => wire::serialize_update_action_type_response(),
            Err(e) => codepipeline_error_response(&e),
        }
    }

    // ---- Webhooks ----

    async fn handle_put_webhook(
        &self,
        state: &Arc<tokio::sync::RwLock<CodePipelineState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_put_webhook_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.webhook.name.is_empty() {
            return json_error_response(400, "ValidationException", "webhook.name is required");
        }
        let name = input.webhook.name.clone();
        let webhook_def = serde_json::to_value(&input.webhook).unwrap_or(Value::Null);
        let tags: HashMap<String, String> = input
            .tags
            .unwrap_or_default()
            .into_iter()
            .map(|t| (t.key, t.value))
            .collect();

        let mut state = state.write().await;
        let wh = state.put_webhook(&name, webhook_def, tags, account_id, region);
        let item = webhook_to_wire(wh);
        wire::serialize_put_webhook_response(&wire::PutWebhookOutput {
            webhook: Some(item),
        })
    }

    async fn handle_delete_webhook(
        &self,
        state: &Arc<tokio::sync::RwLock<CodePipelineState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_webhook_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "name is required");
        }

        let mut state = state.write().await;
        match state.delete_webhook(&input.name) {
            Ok(()) => wire::serialize_delete_webhook_response(&wire::DeleteWebhookOutput {}),
            Err(e) => codepipeline_error_response(&e),
        }
    }

    async fn handle_list_webhooks(
        &self,
        state: &Arc<tokio::sync::RwLock<CodePipelineState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let webhooks = state.list_webhooks();
        let items: Vec<wire::ListWebhookItem> =
            webhooks.iter().map(|w| webhook_to_wire(w)).collect();

        wire::serialize_list_webhooks_response(&wire::ListWebhooksOutput {
            webhooks: Some(items),
            next_token: None,
        })
    }

    async fn handle_register_webhook_with_third_party(
        &self,
        state: &Arc<tokio::sync::RwLock<CodePipelineState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_register_webhook_with_third_party_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let name = input.webhook_name.as_deref().unwrap_or("");

        let mut state = state.write().await;
        match state.register_webhook_with_third_party(name) {
            Ok(()) => wire::serialize_register_webhook_with_third_party_response(
                &wire::RegisterWebhookWithThirdPartyOutput {},
            ),
            Err(e) => codepipeline_error_response(&e),
        }
    }

    async fn handle_deregister_webhook_with_third_party(
        &self,
        state: &Arc<tokio::sync::RwLock<CodePipelineState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_deregister_webhook_with_third_party_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let name = input.webhook_name.as_deref().unwrap_or("");

        let mut state = state.write().await;
        match state.deregister_webhook_with_third_party(name) {
            Ok(()) => wire::serialize_deregister_webhook_with_third_party_response(
                &wire::DeregisterWebhookWithThirdPartyOutput {},
            ),
            Err(e) => codepipeline_error_response(&e),
        }
    }

    // ---- Stage Transitions ----

    async fn handle_disable_stage_transition(
        &self,
        state: &Arc<tokio::sync::RwLock<CodePipelineState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_disable_stage_transition_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.pipeline_name.is_empty() {
            return json_error_response(400, "ValidationException", "pipelineName is required");
        }
        if input.stage_name.is_empty() {
            return json_error_response(400, "ValidationException", "stageName is required");
        }
        if input.transition_type.is_empty() {
            return json_error_response(400, "ValidationException", "transitionType is required");
        }

        let mut state = state.write().await;
        match state.disable_stage_transition(
            &input.pipeline_name,
            &input.stage_name,
            &input.transition_type,
            &input.reason,
        ) {
            Ok(()) => wire::serialize_disable_stage_transition_response(),
            Err(e) => codepipeline_error_response(&e),
        }
    }

    async fn handle_enable_stage_transition(
        &self,
        state: &Arc<tokio::sync::RwLock<CodePipelineState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_enable_stage_transition_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.pipeline_name.is_empty() {
            return json_error_response(400, "ValidationException", "pipelineName is required");
        }
        if input.stage_name.is_empty() {
            return json_error_response(400, "ValidationException", "stageName is required");
        }
        if input.transition_type.is_empty() {
            return json_error_response(400, "ValidationException", "transitionType is required");
        }

        let mut state = state.write().await;
        match state.enable_stage_transition(
            &input.pipeline_name,
            &input.stage_name,
            &input.transition_type,
        ) {
            Ok(()) => wire::serialize_enable_stage_transition_response(),
            Err(e) => codepipeline_error_response(&e),
        }
    }

    // ---- Job Details ----

    async fn handle_get_job_details(
        &self,
        state: &Arc<tokio::sync::RwLock<CodePipelineState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_job_details_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.job_id.is_empty() {
            return json_error_response(400, "ValidationException", "jobId is required");
        }

        let state = state.read().await;
        match state.get_job_details(&input.job_id) {
            Ok(job) => wire::serialize_get_job_details_response(&wire::GetJobDetailsOutput {
                job_details: Some(wire::JobDetails {
                    id: Some(job.id.clone()),
                    account_id: Some(job.account_id.clone()),
                    data: None,
                }),
            }),
            Err(e) => codepipeline_error_response(&e),
        }
    }

    // ---- Pipeline Execution ----

    async fn handle_get_pipeline_execution(
        &self,
        state: &Arc<tokio::sync::RwLock<CodePipelineState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_pipeline_execution_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.pipeline_name.is_empty() {
            return json_error_response(400, "ValidationException", "pipelineName is required");
        }
        if input.pipeline_execution_id.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "pipelineExecutionId is required",
            );
        }

        let state = state.read().await;
        match state.get_pipeline_execution(&input.pipeline_name, &input.pipeline_execution_id) {
            Ok(exec) => {
                wire::serialize_get_pipeline_execution_response(&wire::GetPipelineExecutionOutput {
                    pipeline_execution: Some(pipeline_execution_to_wire(exec)),
                })
            }
            Err(e) => codepipeline_error_response(&e),
        }
    }

    async fn handle_get_pipeline_state(
        &self,
        state: &Arc<tokio::sync::RwLock<CodePipelineState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_pipeline_state_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "name is required");
        }

        let state = state.read().await;
        match state.get_pipeline(&input.name) {
            Ok(p) => {
                // Parse stage names from the pipeline stages value.
                let stage_states: Vec<wire::StageState> = p
                    .stages
                    .as_array()
                    .map(|stages| {
                        stages
                            .iter()
                            .map(|s| wire::StageState {
                                stage_name: s
                                    .get("name")
                                    .and_then(|v| v.as_str())
                                    .map(String::from),
                                ..Default::default()
                            })
                            .collect()
                    })
                    .unwrap_or_default();

                wire::serialize_get_pipeline_state_response(&wire::GetPipelineStateOutput {
                    pipeline_name: Some(p.name.clone()),
                    pipeline_version: Some(p.version),
                    stage_states: Some(stage_states),
                    created: Some(p.created.timestamp() as f64),
                    updated: Some(p.updated.timestamp() as f64),
                })
            }
            Err(e) => codepipeline_error_response(&e),
        }
    }

    async fn handle_get_third_party_job_details(
        &self,
        state: &Arc<tokio::sync::RwLock<CodePipelineState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_third_party_job_details_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.job_id.is_empty() {
            return json_error_response(400, "ValidationException", "jobId is required");
        }

        let state = state.read().await;
        match state.get_job_details(&input.job_id) {
            Ok(job) => wire::serialize_get_third_party_job_details_response(
                &wire::GetThirdPartyJobDetailsOutput {
                    job_details: Some(wire::ThirdPartyJobDetails {
                        id: Some(job.id.clone()),
                        nonce: Some(job.nonce.clone()),
                        data: None,
                    }),
                },
            ),
            Err(e) => codepipeline_error_response(&e),
        }
    }

    // ---- Action/Rule Execution lists ----

    async fn handle_list_action_executions(
        &self,
        state: &Arc<tokio::sync::RwLock<CodePipelineState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_action_executions_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.pipeline_name.is_empty() {
            return json_error_response(400, "ValidationException", "pipelineName is required");
        }
        let pipeline_name = input.pipeline_name.as_str();

        let state = state.read().await;
        // Verify pipeline exists.
        if state.get_pipeline(pipeline_name).is_err() {
            return codepipeline_error_response(&CodePipelineError::PipelineNotFound {
                name: pipeline_name.to_string(),
                account_id: winterbaume_core::default_account_id().to_string(),
            });
        }

        // The mock does not track individual action executions inside a pipeline execution,
        // so we return an empty list.
        wire::serialize_list_action_executions_response(&wire::ListActionExecutionsOutput {
            action_execution_details: Some(vec![]),
            next_token: None,
        })
    }

    async fn handle_list_deploy_action_execution_targets(
        &self,
        state: &Arc<tokio::sync::RwLock<CodePipelineState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_deploy_action_execution_targets_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let pipeline_name = input.pipeline_name.as_deref();

        if let Some(name) = pipeline_name {
            let state = state.read().await;
            if state.get_pipeline(name).is_err() {
                return codepipeline_error_response(&CodePipelineError::PipelineNotFound {
                    name: name.to_string(),
                    account_id: winterbaume_core::default_account_id().to_string(),
                });
            }
        }

        wire::serialize_list_deploy_action_execution_targets_response(
            &wire::ListDeployActionExecutionTargetsOutput {
                targets: Some(vec![]),
                next_token: None,
            },
        )
    }

    async fn handle_list_pipeline_executions(
        &self,
        state: &Arc<tokio::sync::RwLock<CodePipelineState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_pipeline_executions_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.pipeline_name.is_empty() {
            return json_error_response(400, "ValidationException", "pipelineName is required");
        }

        let state = state.read().await;
        match state.list_pipeline_executions(&input.pipeline_name) {
            Ok(executions) => {
                let summaries: Vec<wire::PipelineExecutionSummary> = executions
                    .iter()
                    .map(|e| wire::PipelineExecutionSummary {
                        pipeline_execution_id: Some(e.pipeline_execution_id.clone()),
                        status: Some(e.status.clone()),
                        start_time: Some(e.start_time.timestamp() as f64),
                        last_update_time: Some(e.last_update_time.timestamp() as f64),
                        execution_mode: e.execution_mode.clone(),
                        execution_type: e.execution_type.clone(),
                        ..Default::default()
                    })
                    .collect();
                wire::serialize_list_pipeline_executions_response(
                    &wire::ListPipelineExecutionsOutput {
                        pipeline_execution_summaries: Some(summaries),
                        next_token: None,
                    },
                )
            }
            Err(e) => codepipeline_error_response(&e),
        }
    }

    async fn handle_list_rule_executions(
        &self,
        state: &Arc<tokio::sync::RwLock<CodePipelineState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_rule_executions_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.pipeline_name.is_empty() {
            return json_error_response(400, "ValidationException", "pipelineName is required");
        }
        let pipeline_name = input.pipeline_name.as_str();

        let state = state.read().await;
        if state.get_pipeline(pipeline_name).is_err() {
            return codepipeline_error_response(&CodePipelineError::PipelineNotFound {
                name: pipeline_name.to_string(),
                account_id: winterbaume_core::default_account_id().to_string(),
            });
        }

        // The mock does not track rule executions; return an empty list.
        wire::serialize_list_rule_executions_response(&wire::ListRuleExecutionsOutput {
            rule_execution_details: Some(vec![]),
            next_token: None,
        })
    }

    async fn handle_list_rule_types(
        &self,
        _state: &Arc<tokio::sync::RwLock<CodePipelineState>>,
    ) -> MockResponse {
        // Rule types are built-in AWS types; the mock returns an empty list.
        wire::serialize_list_rule_types_response(&wire::ListRuleTypesOutput {
            rule_types: Some(vec![]),
        })
    }

    // ---- Stage Condition Override ----

    async fn handle_override_stage_condition(
        &self,
        state: &Arc<tokio::sync::RwLock<CodePipelineState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_override_stage_condition_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.pipeline_name.is_empty() {
            return json_error_response(400, "ValidationException", "pipelineName is required");
        }
        if input.pipeline_execution_id.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "pipelineExecutionId is required",
            );
        }
        if input.stage_name.is_empty() {
            return json_error_response(400, "ValidationException", "stageName is required");
        }
        if input.condition_type.is_empty() {
            return json_error_response(400, "ValidationException", "conditionType is required");
        }

        let mut state = state.write().await;
        match state.override_stage_condition(
            &input.pipeline_name,
            &input.pipeline_execution_id,
            &input.stage_name,
            &input.condition_type,
        ) {
            Ok(()) => wire::serialize_override_stage_condition_response(),
            Err(e) => codepipeline_error_response(&e),
        }
    }

    // ---- Job Polling ----

    async fn handle_poll_for_jobs(
        &self,
        state: &Arc<tokio::sync::RwLock<CodePipelineState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_poll_for_jobs_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let action_type_id = serde_json::to_value(&input.action_type_id).unwrap_or(Value::Null);

        let state = state.read().await;
        let jobs = state.poll_for_jobs(&action_type_id);
        let wire_jobs: Vec<wire::Job> = jobs
            .iter()
            .map(|j| wire::Job {
                id: Some(j.id.clone()),
                account_id: Some(j.account_id.clone()),
                nonce: Some(j.nonce.clone()),
                data: None,
            })
            .collect();

        wire::serialize_poll_for_jobs_response(&wire::PollForJobsOutput {
            jobs: Some(wire_jobs),
        })
    }

    async fn handle_poll_for_third_party_jobs(
        &self,
        state: &Arc<tokio::sync::RwLock<CodePipelineState>>,
    ) -> MockResponse {
        // Third-party jobs share the same job store; return empty (no third-party
        // jobs are ever created in the mock without explicit insertion).
        let state = state.read().await;
        let jobs = state.poll_for_jobs(&Value::Null);
        let wire_jobs: Vec<wire::ThirdPartyJob> = jobs
            .iter()
            .map(|j| wire::ThirdPartyJob {
                job_id: Some(j.id.clone()),
                client_id: Some(j.account_id.clone()),
            })
            .collect();

        wire::serialize_poll_for_third_party_jobs_response(&wire::PollForThirdPartyJobsOutput {
            jobs: Some(wire_jobs),
        })
    }

    // ---- Action Revision / Approval ----

    async fn handle_put_action_revision(
        &self,
        state: &Arc<tokio::sync::RwLock<CodePipelineState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_put_action_revision_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.pipeline_name.is_empty() {
            return json_error_response(400, "ValidationException", "pipelineName is required");
        }

        let mut state = state.write().await;
        match state.put_action_revision(&input.pipeline_name, &input.stage_name, &input.action_name)
        {
            Ok(exec_id) => {
                wire::serialize_put_action_revision_response(&wire::PutActionRevisionOutput {
                    new_revision: Some(true),
                    pipeline_execution_id: exec_id,
                })
            }
            Err(e) => codepipeline_error_response(&e),
        }
    }

    async fn handle_put_approval_result(
        &self,
        state: &Arc<tokio::sync::RwLock<CodePipelineState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_put_approval_result_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.pipeline_name.is_empty() {
            return json_error_response(400, "ValidationException", "pipelineName is required");
        }

        let mut state = state.write().await;
        match state.put_approval_result(&input.pipeline_name, &input.stage_name, &input.action_name)
        {
            Ok(()) => {
                let now = chrono::Utc::now().timestamp() as f64;
                wire::serialize_put_approval_result_response(&wire::PutApprovalResultOutput {
                    approved_at: Some(now),
                })
            }
            Err(e) => codepipeline_error_response(&e),
        }
    }

    // ---- Job Result Reporting ----

    async fn handle_put_job_failure_result(
        &self,
        state: &Arc<tokio::sync::RwLock<CodePipelineState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_put_job_failure_result_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.job_id.is_empty() {
            return json_error_response(400, "ValidationException", "jobId is required");
        }

        let mut state = state.write().await;
        match state.put_job_failure_result(&input.job_id) {
            Ok(()) => wire::serialize_put_job_failure_result_response(),
            Err(e) => codepipeline_error_response(&e),
        }
    }

    async fn handle_put_job_success_result(
        &self,
        state: &Arc<tokio::sync::RwLock<CodePipelineState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_put_job_success_result_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.job_id.is_empty() {
            return json_error_response(400, "ValidationException", "jobId is required");
        }

        let mut state = state.write().await;
        match state.put_job_success_result(&input.job_id) {
            Ok(()) => wire::serialize_put_job_success_result_response(),
            Err(e) => codepipeline_error_response(&e),
        }
    }

    // Third-party job result reporting. In the mock these map to the same job store
    // as regular jobs. The AWS API requires a clientToken but the mock accepts any.
    async fn handle_put_third_party_job_failure_result(
        &self,
        state: &Arc<tokio::sync::RwLock<CodePipelineState>>,
    ) -> MockResponse {
        let _ = state;
        // Without a real third-party job mechanism, we just accept the call.
        wire::serialize_put_third_party_job_failure_result_response()
    }

    async fn handle_put_third_party_job_success_result(
        &self,
        state: &Arc<tokio::sync::RwLock<CodePipelineState>>,
    ) -> MockResponse {
        let _ = state;
        // Without a real third-party job mechanism, we just accept the call.
        wire::serialize_put_third_party_job_success_result_response()
    }

    // ---- Pipeline Execution Control ----

    async fn handle_retry_stage_execution(
        &self,
        state: &Arc<tokio::sync::RwLock<CodePipelineState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_retry_stage_execution_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.pipeline_name.is_empty() {
            return json_error_response(400, "ValidationException", "pipelineName is required");
        }
        if input.pipeline_execution_id.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "pipelineExecutionId is required",
            );
        }
        let retry_mode = if input.retry_mode.is_empty() {
            "FAILED_ACTIONS"
        } else {
            input.retry_mode.as_str()
        };

        let mut state = state.write().await;
        match state.retry_stage_execution(
            &input.pipeline_name,
            &input.pipeline_execution_id,
            &input.stage_name,
            retry_mode,
        ) {
            Ok(exec_id) => {
                wire::serialize_retry_stage_execution_response(&wire::RetryStageExecutionOutput {
                    pipeline_execution_id: Some(exec_id),
                })
            }
            Err(e) => codepipeline_error_response(&e),
        }
    }

    async fn handle_rollback_stage(
        &self,
        state: &Arc<tokio::sync::RwLock<CodePipelineState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_rollback_stage_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.pipeline_name.is_empty() {
            return json_error_response(400, "ValidationException", "pipelineName is required");
        }
        if input.target_pipeline_execution_id.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "targetPipelineExecutionId is required",
            );
        }

        let mut state = state.write().await;
        match state.rollback_stage(
            &input.pipeline_name,
            &input.stage_name,
            &input.target_pipeline_execution_id,
        ) {
            Ok(exec_id) => wire::serialize_rollback_stage_response(&wire::RollbackStageOutput {
                pipeline_execution_id: Some(exec_id),
            }),
            Err(e) => codepipeline_error_response(&e),
        }
    }

    async fn handle_start_pipeline_execution(
        &self,
        state: &Arc<tokio::sync::RwLock<CodePipelineState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_start_pipeline_execution_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "name is required");
        }

        let variables: Vec<Value> = input
            .variables
            .as_ref()
            .map(|v| {
                v.iter()
                    .filter_map(|x| serde_json::to_value(x).ok())
                    .collect()
            })
            .unwrap_or_default();
        let source_revisions: Vec<Value> = input
            .source_revisions
            .as_ref()
            .map(|v| {
                v.iter()
                    .filter_map(|x| serde_json::to_value(x).ok())
                    .collect()
            })
            .unwrap_or_default();

        let mut state = state.write().await;
        match state.start_pipeline_execution(&input.name, variables, source_revisions, None, None) {
            Ok(execution_id) => wire::serialize_start_pipeline_execution_response(
                &wire::StartPipelineExecutionOutput {
                    pipeline_execution_id: Some(execution_id),
                },
            ),
            Err(e) => codepipeline_error_response(&e),
        }
    }

    async fn handle_stop_pipeline_execution(
        &self,
        state: &Arc<tokio::sync::RwLock<CodePipelineState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_stop_pipeline_execution_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.pipeline_name.is_empty() {
            return json_error_response(400, "ValidationException", "pipelineName is required");
        }
        if input.pipeline_execution_id.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "pipelineExecutionId is required",
            );
        }
        let abandon = input.abandon.unwrap_or(false);
        let reason = input.reason.as_deref();

        let mut state = state.write().await;
        match state.stop_pipeline_execution(
            &input.pipeline_name,
            &input.pipeline_execution_id,
            abandon,
            reason,
        ) {
            Ok(exec_id) => wire::serialize_stop_pipeline_execution_response(
                &wire::StopPipelineExecutionOutput {
                    pipeline_execution_id: Some(exec_id),
                },
            ),
            Err(e) => codepipeline_error_response(&e),
        }
    }
}

fn pipeline_to_declaration(p: &crate::types::Pipeline) -> wire::PipelineDeclaration {
    let stages: Vec<wire::StageDeclaration> =
        serde_json::from_value(p.stages.clone()).unwrap_or_default();
    wire::PipelineDeclaration {
        name: p.name.clone(),
        role_arn: p.role_arn.clone(),
        stages,
        version: Some(p.version),
        ..Default::default()
    }
}

fn custom_action_to_wire(a: &crate::types::CustomActionType) -> wire::ActionType {
    let config_props: Option<Vec<wire::ActionConfigurationProperty>> = a
        .configuration_properties
        .as_ref()
        .and_then(|v| serde_json::from_value(v.clone()).ok());
    let settings: Option<wire::ActionTypeSettings> = a
        .settings
        .as_ref()
        .and_then(|v| serde_json::from_value(v.clone()).ok());

    wire::ActionType {
        id: Some(wire::ActionTypeId {
            category: a.category.clone(),
            owner: "Custom".to_string(),
            provider: a.provider.clone(),
            version: a.version.clone(),
        }),
        settings,
        action_configuration_properties: config_props,
        input_artifact_details: Some(wire::ArtifactDetails {
            minimum_count: a.input_artifact_details.minimum_count,
            maximum_count: a.input_artifact_details.maximum_count,
        }),
        output_artifact_details: Some(wire::ArtifactDetails {
            minimum_count: a.output_artifact_details.minimum_count,
            maximum_count: a.output_artifact_details.maximum_count,
        }),
    }
}

fn webhook_to_wire(w: &crate::types::Webhook) -> wire::ListWebhookItem {
    let definition: wire::WebhookDefinition =
        serde_json::from_value(w.definition.clone()).unwrap_or_default();
    let tags: Vec<wire::Tag> = w
        .tags
        .iter()
        .map(|(k, v)| wire::Tag {
            key: k.clone(),
            value: v.clone(),
        })
        .collect();

    wire::ListWebhookItem {
        arn: Some(w.arn.clone()),
        definition: Some(definition),
        url: Some(w.url.clone()),
        tags: if tags.is_empty() { None } else { Some(tags) },
        error_code: None,
        error_message: None,
        last_triggered: None,
    }
}

fn pipeline_execution_to_wire(exec: &crate::types::PipelineExecution) -> wire::PipelineExecution {
    wire::PipelineExecution {
        pipeline_execution_id: Some(exec.pipeline_execution_id.clone()),
        pipeline_name: Some(exec.pipeline_name.clone()),
        pipeline_version: Some(exec.pipeline_version),
        status: Some(exec.status.clone()),
        status_summary: exec.status_summary.clone(),
        execution_mode: exec.execution_mode.clone(),
        execution_type: exec.execution_type.clone(),
        artifact_revisions: None,
        rollback_metadata: None,
        trigger: None,
        variables: None,
    }
}

fn codepipeline_error_response(err: &CodePipelineError) -> MockResponse {
    let (status, error_type) = match err {
        CodePipelineError::PipelineAlreadyExists { .. } => (400, "InvalidStructureException"),
        CodePipelineError::PipelineNotFound { .. } => (400, "PipelineNotFoundException"),
        CodePipelineError::ResourceNotFound { .. } => (400, "ResourceNotFoundException"),
        CodePipelineError::ActionTypeAlreadyExists => (409, "InvalidStructureException"),
        CodePipelineError::ActionTypeNotFound => (400, "ActionNotFoundException"),
        CodePipelineError::WebhookNotFound { .. } => (400, "WebhookNotFoundException"),
        CodePipelineError::WebhookAlreadyExists { .. } => (400, "InvalidStructureException"),
        CodePipelineError::JobNotFound { .. } => (400, "JobNotFoundException"),
        CodePipelineError::InvalidNonce => (400, "InvalidNonceException"),
        CodePipelineError::PipelineExecutionNotFound => (400, "PipelineExecutionNotFoundException"),
        CodePipelineError::StageNotFound { .. } => (400, "StageNotFoundException"),
    };
    let body = json!({
        "__type": error_type,
        "message": err.to_string(),
    });
    MockResponse::json(status, body.to_string())
}
