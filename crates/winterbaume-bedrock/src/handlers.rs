#[allow(unused_imports)]
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::json;
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, default_account_id,
};

use crate::state::{BedrockError, BedrockState};
use crate::types::*;
use crate::views::BedrockStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct BedrockService {
    pub(crate) state: Arc<BackendState<BedrockState>>,
    pub(crate) notifier: StateChangeNotifier<BedrockStateView>,
}

impl BedrockService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for BedrockService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for BedrockService {
    fn service_name(&self) -> &str {
        "bedrock"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://bedrock\.(.+)\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl BedrockService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let method = request.method.as_str();

        let segments: Vec<&str> = path.trim_start_matches('/').split('/').collect();

        let raw_query = extract_query(&request.uri);
        let query_map: HashMap<String, String> = winterbaume_core::parse_query_string(&raw_query);

        // Bedrock control plane routes:
        // GET    /foundation-models                                    - ListFoundationModels
        // GET    /foundation-models/{modelIdentifier}                  - GetFoundationModel
        // POST   /model-customization-jobs                             - CreateModelCustomizationJob
        // GET    /model-customization-jobs                             - ListModelCustomizationJobs
        // GET    /model-customization-jobs/{jobIdentifier}             - GetModelCustomizationJob
        // POST   /model-customization-jobs/{jobIdentifier}/stop        - StopModelCustomizationJob
        // GET    /custom-models                                        - ListCustomModels
        // GET    /custom-models/{modelIdentifier}                      - GetCustomModel
        // DELETE /custom-models/{modelIdentifier}                      - DeleteCustomModel
        // GET    /logging/modelinvocations                             - GetModelInvocationLoggingConfiguration
        // PUT    /logging/modelinvocations                             - PutModelInvocationLoggingConfiguration
        // DELETE /logging/modelinvocations                             - DeleteModelInvocationLoggingConfiguration
        // POST   /listTagsForResource                                  - ListTagsForResource
        // POST   /tagResource                                          - TagResource
        // POST   /untagResource                                        - UntagResource

        let response = match (method, segments.as_slice()) {
            // GET /foundation-models - ListFoundationModels
            ("GET", ["foundation-models"]) => {
                self.handle_list_foundation_models(&state, account_id, &region)
                    .await
            }
            // GET /foundation-models/{modelIdentifier} - GetFoundationModel
            ("GET", ["foundation-models", _model_id, ..]) => {
                let model_id = percent_decode(&segments[1..].join("/"));
                let labels: &[(&str, &str)] = &[("modelIdentifier", model_id.as_str())];
                self.handle_get_foundation_model(
                    &state, &request, labels, &query_map, account_id, &region,
                )
                .await
            }
            // POST /model-customization-jobs/{jobIdentifier}/stop - StopModelCustomizationJob
            ("POST", [_, _, "stop"]) if segments[0] == "model-customization-jobs" => {
                let job_id = percent_decode(segments[1]);
                let labels: &[(&str, &str)] = &[("jobIdentifier", job_id.as_str())];
                self.handle_stop_model_customization_job(&state, &request, labels, &query_map)
                    .await
            }
            // POST /model-customization-jobs - CreateModelCustomizationJob
            ("POST", ["model-customization-jobs"]) => {
                self.handle_create_model_customization_job(
                    &state,
                    &request,
                    &[],
                    &query_map,
                    account_id,
                    &region,
                )
                .await
            }
            // GET /model-customization-jobs - ListModelCustomizationJobs
            ("GET", ["model-customization-jobs"]) => {
                self.handle_list_model_customization_jobs(&state).await
            }
            // GET /model-customization-jobs/{jobIdentifier} - GetModelCustomizationJob
            ("GET", ["model-customization-jobs", ..]) if segments.len() >= 2 => {
                let job_id = percent_decode(segments[1]);
                let labels: &[(&str, &str)] = &[("jobIdentifier", job_id.as_str())];
                self.handle_get_model_customization_job(&state, &request, labels, &query_map)
                    .await
            }
            // GET /custom-models - ListCustomModels
            ("GET", ["custom-models"]) => self.handle_list_custom_models(&state, account_id).await,
            // GET /custom-models/{modelIdentifier} - GetCustomModel
            ("GET", ["custom-models", ..]) if segments.len() >= 2 => {
                let model_id = percent_decode(segments[1]);
                let labels: &[(&str, &str)] = &[("modelIdentifier", model_id.as_str())];
                self.handle_get_custom_model(&state, &request, labels, &query_map)
                    .await
            }
            // DELETE /custom-models/{modelIdentifier} - DeleteCustomModel
            ("DELETE", ["custom-models", ..]) if segments.len() >= 2 => {
                let model_id = percent_decode(segments[1]);
                let labels: &[(&str, &str)] = &[("modelIdentifier", model_id.as_str())];
                self.handle_delete_custom_model(&state, &request, labels, &query_map)
                    .await
            }
            // GET /logging/modelinvocations - GetModelInvocationLoggingConfiguration
            ("GET", ["logging", "modelinvocations"]) => {
                self.handle_get_model_invocation_logging_configuration(&state)
                    .await
            }
            // PUT /logging/modelinvocations - PutModelInvocationLoggingConfiguration
            ("PUT", ["logging", "modelinvocations"]) => {
                self.handle_put_model_invocation_logging_configuration(
                    &state,
                    &request,
                    &[],
                    &query_map,
                )
                .await
            }
            // DELETE /logging/modelinvocations - DeleteModelInvocationLoggingConfiguration
            ("DELETE", ["logging", "modelinvocations"]) => {
                self.handle_delete_model_invocation_logging_configuration(&state)
                    .await
            }
            // POST /listTagsForResource - ListTagsForResource
            ("POST", ["listTagsForResource"]) => {
                self.handle_list_tags_for_resource(&state, &request, &[], &query_map)
                    .await
            }
            // POST /tagResource - TagResource
            ("POST", ["tagResource"]) => {
                self.handle_tag_resource(&state, &request, &[], &query_map)
                    .await
            }
            // POST /untagResource - UntagResource
            ("POST", ["untagResource"]) => {
                self.handle_untag_resource(&state, &request, &[], &query_map)
                    .await
            }
            // POST /guardrails - CreateGuardrail
            ("POST", ["guardrails"]) => {
                self.handle_create_guardrail(&state, &request, &[], &query_map, account_id, &region)
                    .await
            }
            // POST /guardrails/{guardrailIdentifier} - CreateGuardrailVersion
            ("POST", ["guardrails", _]) => {
                let id = percent_decode(segments[1]);
                let labels: &[(&str, &str)] = &[("guardrailIdentifier", id.as_str())];
                self.handle_create_guardrail_version(&state, &request, labels, &query_map)
                    .await
            }
            // GET /guardrails/{guardrailIdentifier} - GetGuardrail
            ("GET", ["guardrails", _]) => {
                let id = percent_decode(segments[1]);
                let labels: &[(&str, &str)] = &[("guardrailIdentifier", id.as_str())];
                self.handle_get_guardrail(&state, &request, labels, &query_map)
                    .await
            }
            // GET /guardrails - ListGuardrails
            ("GET", ["guardrails"]) => self.handle_list_guardrails(&state).await,
            // PUT /guardrails/{guardrailIdentifier} - UpdateGuardrail
            ("PUT", ["guardrails", _]) => {
                let id = percent_decode(segments[1]);
                let labels: &[(&str, &str)] = &[("guardrailIdentifier", id.as_str())];
                self.handle_update_guardrail(&state, &request, labels, &query_map)
                    .await
            }
            // DELETE /guardrails/{guardrailIdentifier} - DeleteGuardrail
            ("DELETE", ["guardrails", _]) => {
                let id = percent_decode(segments[1]);
                let labels: &[(&str, &str)] = &[("guardrailIdentifier", id.as_str())];
                self.handle_delete_guardrail(&state, &request, labels, &query_map)
                    .await
            }
            // POST /provisioned-model-throughput - CreateProvisionedModelThroughput
            ("POST", ["provisioned-model-throughput"]) => {
                self.handle_create_provisioned_model_throughput(
                    &state,
                    &request,
                    &[],
                    &query_map,
                    account_id,
                    &region,
                )
                .await
            }
            // GET /provisioned-model-throughput/{provisionedModelId} - GetProvisionedModelThroughput
            ("GET", ["provisioned-model-throughput", _]) => {
                let id = percent_decode(segments[1]);
                let labels: &[(&str, &str)] = &[("provisionedModelId", id.as_str())];
                self.handle_get_provisioned_model_throughput(&state, &request, labels, &query_map)
                    .await
            }
            // GET /provisioned-model-throughputs - ListProvisionedModelThroughputs
            ("GET", ["provisioned-model-throughputs"]) => {
                self.handle_list_provisioned_model_throughputs(&state).await
            }
            // PATCH /provisioned-model-throughput/{provisionedModelId} - UpdateProvisionedModelThroughput
            ("PATCH", ["provisioned-model-throughput", _]) => {
                let id = percent_decode(segments[1]);
                let labels: &[(&str, &str)] = &[("provisionedModelId", id.as_str())];
                self.handle_update_provisioned_model_throughput(
                    &state, &request, labels, &query_map,
                )
                .await
            }
            // DELETE /provisioned-model-throughput/{provisionedModelId} - DeleteProvisionedModelThroughput
            ("DELETE", ["provisioned-model-throughput", _]) => {
                let id = percent_decode(segments[1]);
                let labels: &[(&str, &str)] = &[("provisionedModelId", id.as_str())];
                self.handle_delete_provisioned_model_throughput(
                    &state, &request, labels, &query_map,
                )
                .await
            }
            // POST /inference-profiles - CreateInferenceProfile
            ("POST", ["inference-profiles"]) => {
                self.handle_create_inference_profile(
                    &state,
                    &request,
                    &[],
                    &query_map,
                    account_id,
                    &region,
                )
                .await
            }
            // GET /inference-profiles/{inferenceProfileIdentifier} - GetInferenceProfile
            ("GET", ["inference-profiles", _]) => {
                let id = percent_decode(segments[1]);
                let labels: &[(&str, &str)] = &[("inferenceProfileIdentifier", id.as_str())];
                self.handle_get_inference_profile(&state, &request, labels, &query_map)
                    .await
            }
            // GET /inference-profiles - ListInferenceProfiles
            ("GET", ["inference-profiles"]) => self.handle_list_inference_profiles(&state).await,
            // DELETE /inference-profiles/{inferenceProfileIdentifier} - DeleteInferenceProfile
            ("DELETE", ["inference-profiles", _]) => {
                let id = percent_decode(segments[1]);
                let labels: &[(&str, &str)] = &[("inferenceProfileIdentifier", id.as_str())];
                self.handle_delete_inference_profile(&state, &request, labels, &query_map)
                    .await
            }
            // POST /prompt-routers - CreatePromptRouter
            ("POST", ["prompt-routers"]) => {
                self.handle_create_prompt_router(
                    &state,
                    &request,
                    &[],
                    &query_map,
                    account_id,
                    &region,
                )
                .await
            }
            // GET /prompt-routers/{promptRouterArn} - GetPromptRouter
            ("GET", ["prompt-routers", ..]) if segments.len() >= 2 => {
                let arn = percent_decode(&segments[1..].join("/"));
                let labels: &[(&str, &str)] = &[("promptRouterArn", arn.as_str())];
                self.handle_get_prompt_router(&state, &request, labels, &query_map)
                    .await
            }
            // GET /prompt-routers - ListPromptRouters
            ("GET", ["prompt-routers"]) => self.handle_list_prompt_routers(&state).await,
            // DELETE /prompt-routers/{promptRouterArn} - DeletePromptRouter
            ("DELETE", ["prompt-routers", ..]) if segments.len() >= 2 => {
                let arn = percent_decode(&segments[1..].join("/"));
                let labels: &[(&str, &str)] = &[("promptRouterArn", arn.as_str())];
                self.handle_delete_prompt_router(&state, &request, labels, &query_map)
                    .await
            }
            // POST /evaluation-jobs - CreateEvaluationJob
            ("POST", ["evaluation-jobs"]) => {
                self.handle_create_evaluation_job(
                    &state,
                    &request,
                    &[],
                    &query_map,
                    account_id,
                    &region,
                )
                .await
            }
            // GET /evaluation-jobs/{jobIdentifier} - GetEvaluationJob
            ("GET", ["evaluation-jobs", _]) => {
                let id = percent_decode(segments[1]);
                let labels: &[(&str, &str)] = &[("jobIdentifier", id.as_str())];
                self.handle_get_evaluation_job(&state, &request, labels, &query_map)
                    .await
            }
            // GET /evaluation-jobs - ListEvaluationJobs
            ("GET", ["evaluation-jobs"]) => self.handle_list_evaluation_jobs(&state).await,
            // POST /evaluation-job/{jobIdentifier}/stop - StopEvaluationJob
            ("POST", ["evaluation-job", _, "stop"]) => {
                let id = percent_decode(segments[1]);
                let labels: &[(&str, &str)] = &[("jobIdentifier", id.as_str())];
                self.handle_stop_evaluation_job(&state, &request, labels, &query_map)
                    .await
            }
            // POST /model-invocation-job - CreateModelInvocationJob
            ("POST", ["model-invocation-job"]) => {
                self.handle_create_model_invocation_job(
                    &state,
                    &request,
                    &[],
                    &query_map,
                    account_id,
                    &region,
                )
                .await
            }
            // GET /model-invocation-job/{jobIdentifier} - GetModelInvocationJob
            ("GET", ["model-invocation-job", _]) => {
                let id = percent_decode(segments[1]);
                let labels: &[(&str, &str)] = &[("jobIdentifier", id.as_str())];
                self.handle_get_model_invocation_job(&state, &request, labels, &query_map)
                    .await
            }
            // GET /model-invocation-jobs - ListModelInvocationJobs
            ("GET", ["model-invocation-jobs"]) => {
                self.handle_list_model_invocation_jobs(&state).await
            }
            // POST /model-invocation-job/{jobIdentifier}/stop - StopModelInvocationJob
            ("POST", ["model-invocation-job", _, "stop"]) => {
                let id = percent_decode(segments[1]);
                let labels: &[(&str, &str)] = &[("jobIdentifier", id.as_str())];
                self.handle_stop_model_invocation_job(&state, &request, labels, &query_map)
                    .await
            }
            // POST /model-import-jobs - CreateModelImportJob
            ("POST", ["model-import-jobs"]) => {
                self.handle_create_model_import_job(
                    &state,
                    &request,
                    &[],
                    &query_map,
                    account_id,
                    &region,
                )
                .await
            }
            // GET /model-import-jobs/{jobIdentifier} - GetModelImportJob
            ("GET", ["model-import-jobs", _]) => {
                let id = percent_decode(segments[1]);
                let labels: &[(&str, &str)] = &[("jobIdentifier", id.as_str())];
                self.handle_get_model_import_job(&state, &request, labels, &query_map)
                    .await
            }
            // GET /model-import-jobs - ListModelImportJobs
            ("GET", ["model-import-jobs"]) => self.handle_list_model_import_jobs(&state).await,
            // POST /model-copy-jobs - CreateModelCopyJob
            ("POST", ["model-copy-jobs"]) => {
                self.handle_create_model_copy_job(
                    &state,
                    &request,
                    &[],
                    &query_map,
                    account_id,
                    &region,
                )
                .await
            }
            // GET /model-copy-jobs/{jobArn} - GetModelCopyJob
            ("GET", ["model-copy-jobs", ..]) if segments.len() >= 2 => {
                let arn = percent_decode(&segments[1..].join("/"));
                let labels: &[(&str, &str)] = &[("jobArn", arn.as_str())];
                self.handle_get_model_copy_job(&state, &request, labels, &query_map)
                    .await
            }
            // GET /model-copy-jobs - ListModelCopyJobs
            ("GET", ["model-copy-jobs"]) => self.handle_list_model_copy_jobs(&state).await,
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        };

        response
    }

    async fn handle_list_foundation_models(
        &self,
        state: &Arc<tokio::sync::RwLock<BedrockState>>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let state = state.read().await;
        let models = state.list_foundation_models(account_id, region);
        let model_summaries: Vec<wire::FoundationModelSummary> =
            models.iter().map(foundation_model_to_summary).collect();
        wire::serialize_list_foundation_models_response(&wire::ListFoundationModelsResponse {
            model_summaries: Some(model_summaries),
        })
    }

    async fn handle_get_foundation_model(
        &self,
        state: &Arc<tokio::sync::RwLock<BedrockState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_get_foundation_model_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_foundation_model(&input.model_identifier, account_id, region) {
            Ok(m) => {
                wire::serialize_get_foundation_model_response(&wire::GetFoundationModelResponse {
                    model_details: Some(foundation_model_to_detail(&m)),
                })
            }
            Err(e) => bedrock_error_response(&e),
        }
    }

    async fn handle_create_model_customization_job(
        &self,
        state: &Arc<tokio::sync::RwLock<BedrockState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_model_customization_job_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.job_name.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'jobName'");
        }
        if input.base_model_identifier.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'baseModelIdentifier'");
        }
        if input.custom_model_name.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'customModelName'");
        }
        if input.role_arn.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'roleArn'");
        }
        let customization_type = input.customization_type.as_deref().unwrap_or("FINE_TUNING");
        let training_data_config = TrainingDataConfig {
            s3_uri: input
                .training_data_config
                .s3_uri
                .clone()
                .unwrap_or_default(),
        };
        let output_data_config = OutputDataConfig {
            s3_uri: input.output_data_config.s3_uri.clone(),
        };
        let hyper_parameters = input.hyper_parameters.unwrap_or_default();

        let mut state = state.write().await;
        match state.create_model_customization_job(
            &input.job_name,
            &input.base_model_identifier,
            &input.custom_model_name,
            customization_type,
            &input.role_arn,
            training_data_config,
            output_data_config,
            hyper_parameters,
            account_id,
            region,
        ) {
            Ok(job) => wire::serialize_create_model_customization_job_response(
                &wire::CreateModelCustomizationJobResponse {
                    job_arn: Some(job.job_arn.clone()),
                },
            ),
            Err(e) => bedrock_error_response(&e),
        }
    }

    async fn handle_list_model_customization_jobs(
        &self,
        state: &Arc<tokio::sync::RwLock<BedrockState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let jobs = state.list_model_customization_jobs();
        let job_summaries: Vec<wire::ModelCustomizationJobSummary> = jobs
            .iter()
            .map(|j| wire::ModelCustomizationJobSummary {
                job_arn: Some(j.job_arn.clone()),
                job_name: Some(j.job_name.clone()),
                base_model_arn: Some(j.base_model_identifier.clone()),
                custom_model_name: Some(j.custom_model_name.clone()),
                customization_type: Some(j.customization_type.clone()),
                status: Some(j.status.clone()),
                creation_time: Some(j.creation_time.clone()),
                last_modified_time: Some(j.last_modified_time.clone()),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_model_customization_jobs_response(
            &wire::ListModelCustomizationJobsResponse {
                model_customization_job_summaries: Some(job_summaries),
                next_token: None,
            },
        )
    }
    async fn handle_get_model_customization_job(
        &self,
        state: &Arc<tokio::sync::RwLock<BedrockState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_get_model_customization_job_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let state = state.read().await;
        match state.get_model_customization_job(&input.job_identifier) {
            Ok(job) => wire::serialize_get_model_customization_job_response(
                &wire::GetModelCustomizationJobResponse {
                    job_arn: Some(job.job_arn.clone()),
                    job_name: Some(job.job_name.clone()),
                    base_model_arn: Some(job.base_model_identifier.clone()),
                    output_model_name: Some(job.custom_model_name.clone()),
                    customization_type: Some(job.customization_type.clone()),
                    role_arn: Some(job.role_arn.clone()),
                    status: Some(job.status.clone()),
                    creation_time: Some(job.creation_time.clone()),
                    last_modified_time: Some(job.last_modified_time.clone()),
                    training_data_config: Some(wire::TrainingDataConfig {
                        s3_uri: Some(job.training_data_config.s3_uri.clone()),
                        ..Default::default()
                    }),
                    output_data_config: Some(wire::OutputDataConfig {
                        s3_uri: job.output_data_config.s3_uri.clone(),
                    }),
                    hyper_parameters: Some(job.hyper_parameters.clone()),
                    validation_data_config: Some(wire::ValidationDataConfig { validators: vec![] }),
                    ..Default::default()
                },
            ),
            Err(e) => bedrock_error_response(&e),
        }
    }

    async fn handle_stop_model_customization_job(
        &self,
        state: &Arc<tokio::sync::RwLock<BedrockState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_stop_model_customization_job_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let mut state = state.write().await;
        match state.stop_model_customization_job(&input.job_identifier) {
            Ok(()) => wire::serialize_stop_model_customization_job_response(
                &wire::StopModelCustomizationJobResponse {},
            ),
            Err(e) => bedrock_error_response(&e),
        }
    }

    async fn handle_get_custom_model(
        &self,
        state: &Arc<tokio::sync::RwLock<BedrockState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_custom_model_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_custom_model(&input.model_identifier) {
            Ok(model) => wire::serialize_get_custom_model_response(&wire::GetCustomModelResponse {
                model_arn: Some(model.model_arn.clone()),
                model_name: Some(model.model_name.clone()),
                base_model_arn: Some(model.base_model_arn.clone()),
                customization_type: Some(model.customization_type.clone()),
                creation_time: Some(model.creation_time.clone()),
                job_arn: Some(model.job_arn.clone()),
                job_name: Some(model.job_name.clone()),
                training_data_config: Some(wire::TrainingDataConfig {
                    s3_uri: Some(model.training_data_config.s3_uri.clone()),
                    ..Default::default()
                }),
                output_data_config: Some(wire::OutputDataConfig {
                    s3_uri: model.output_data_config.s3_uri.clone(),
                }),
                hyper_parameters: Some(model.hyper_parameters.clone()),
                ..Default::default()
            }),
            Err(e) => bedrock_error_response(&e),
        }
    }

    async fn handle_list_custom_models(
        &self,
        state: &Arc<tokio::sync::RwLock<BedrockState>>,
        account_id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        let models = state.list_custom_models();
        let summaries: Vec<wire::CustomModelSummary> = models
            .iter()
            .map(|m| wire::CustomModelSummary {
                model_arn: Some(m.model_arn.clone()),
                model_name: Some(m.model_name.clone()),
                base_model_arn: Some(m.base_model_arn.clone()),
                base_model_name: Some(m.base_model_arn.clone()),
                customization_type: Some(m.customization_type.clone()),
                creation_time: Some(m.creation_time.clone()),
                owner_account_id: Some(account_id.to_string()),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_custom_models_response(&wire::ListCustomModelsResponse {
            model_summaries: Some(summaries),
            next_token: None,
        })
    }

    async fn handle_delete_custom_model(
        &self,
        state: &Arc<tokio::sync::RwLock<BedrockState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_custom_model_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.delete_custom_model(&input.model_identifier) {
            Ok(()) => {
                wire::serialize_delete_custom_model_response(&wire::DeleteCustomModelResponse {})
            }
            Err(e) => bedrock_error_response(&e),
        }
    }

    async fn handle_get_model_invocation_logging_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<BedrockState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let logging_config = state.get_logging_config().map(|c| wire::LoggingConfig {
            text_data_delivery_enabled: c.text_data_delivery_enabled,
            image_data_delivery_enabled: c.image_data_delivery_enabled,
            embedding_data_delivery_enabled: c.embedding_data_delivery_enabled,
            s3_config: c.s3_config.as_ref().map(|s| wire::S3Config {
                bucket_name: s.bucket_name.clone(),
                key_prefix: s.key_prefix.clone(),
            }),
            cloud_watch_config: c
                .cloud_watch_config
                .as_ref()
                .map(|cw| wire::CloudWatchConfig {
                    log_group_name: cw.log_group_name.clone(),
                    role_arn: cw.role_arn.clone(),
                    ..Default::default()
                }),
            ..Default::default()
        });
        wire::serialize_get_model_invocation_logging_configuration_response(
            &wire::GetModelInvocationLoggingConfigurationResponse { logging_config },
        )
    }

    async fn handle_put_model_invocation_logging_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<BedrockState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_put_model_invocation_logging_configuration_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let lc = &input.logging_config;
        let config = LoggingConfiguration {
            text_data_delivery_enabled: lc.text_data_delivery_enabled,
            image_data_delivery_enabled: lc.image_data_delivery_enabled,
            embedding_data_delivery_enabled: lc.embedding_data_delivery_enabled,
            s3_config: lc.s3_config.as_ref().map(|s| S3LogConfig {
                bucket_name: s.bucket_name.clone(),
                key_prefix: s.key_prefix.clone(),
            }),
            cloud_watch_config: lc
                .cloud_watch_config
                .as_ref()
                .map(|cw| CloudWatchLogConfig {
                    log_group_name: cw.log_group_name.clone(),
                    role_arn: cw.role_arn.clone(),
                }),
        };

        let mut state = state.write().await;
        state.put_logging_config(config);
        wire::serialize_put_model_invocation_logging_configuration_response(
            &wire::PutModelInvocationLoggingConfigurationResponse {},
        )
    }

    async fn handle_delete_model_invocation_logging_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<BedrockState>>,
    ) -> MockResponse {
        let mut state = state.write().await;
        state.delete_logging_config();
        wire::serialize_delete_model_invocation_logging_configuration_response(
            &wire::DeleteModelInvocationLoggingConfigurationResponse {},
        )
    }

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<BedrockState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.resource_a_r_n.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'resourceARN'");
        }
        let state = state.read().await;
        match state.list_tags_for_resource(&input.resource_a_r_n) {
            Ok(tags) => {
                let wire_tags: Vec<wire::Tag> = tags
                    .iter()
                    .map(|t| wire::Tag {
                        key: t.key.clone(),
                        value: t.value.clone(),
                    })
                    .collect();
                wire::serialize_list_tags_for_resource_response(
                    &wire::ListTagsForResourceResponse {
                        tags: Some(wire_tags),
                    },
                )
            }
            Err(e) => bedrock_error_response(&e),
        }
    }

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<BedrockState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.resource_a_r_n.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'resourceARN'");
        }
        let tags: Vec<ResourceTag> = input
            .tags
            .into_iter()
            .map(|t| ResourceTag {
                key: t.key,
                value: t.value,
            })
            .collect();

        let mut state = state.write().await;
        match state.tag_resource(&input.resource_a_r_n, tags) {
            Ok(()) => wire::serialize_tag_resource_response(&wire::TagResourceResponse {}),
            Err(e) => bedrock_error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<BedrockState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.resource_a_r_n.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'resourceARN'");
        }

        let mut state = state.write().await;
        match state.untag_resource(&input.resource_a_r_n, input.tag_keys) {
            Ok(()) => wire::serialize_untag_resource_response(&wire::UntagResourceResponse {}),
            Err(e) => bedrock_error_response(&e),
        }
    }

    // ---- Guardrails ----

    async fn handle_create_guardrail(
        &self,
        state: &Arc<tokio::sync::RwLock<BedrockState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_guardrail_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'name'");
        }
        let blocked_input = if input.blocked_input_messaging.is_empty() {
            "Blocked"
        } else {
            input.blocked_input_messaging.as_str()
        };
        let blocked_output = if input.blocked_outputs_messaging.is_empty() {
            "Blocked"
        } else {
            input.blocked_outputs_messaging.as_str()
        };
        let description = input.description.clone();
        let mut state = state.write().await;
        match state.create_guardrail(
            &input.name,
            description,
            blocked_input,
            blocked_output,
            account_id,
            region,
        ) {
            Ok(g) => wire::serialize_create_guardrail_response(&wire::CreateGuardrailResponse {
                guardrail_arn: Some(g.guardrail_arn.clone()),
                guardrail_id: Some(g.guardrail_id.clone()),
                version: Some(g.version.clone()),
                created_at: Some(g.created_at.clone()),
            }),
            Err(e) => bedrock_error_response(&e),
        }
    }

    async fn handle_create_guardrail_version(
        &self,
        state: &Arc<tokio::sync::RwLock<BedrockState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_guardrail_version_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.create_guardrail_version(&input.guardrail_identifier) {
            Ok(id) => wire::serialize_create_guardrail_version_response(
                &wire::CreateGuardrailVersionResponse {
                    guardrail_id: Some(id),
                    version: Some("1".to_string()),
                },
            ),
            Err(e) => bedrock_error_response(&e),
        }
    }

    async fn handle_get_guardrail(
        &self,
        state: &Arc<tokio::sync::RwLock<BedrockState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_guardrail_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_guardrail(&input.guardrail_identifier) {
            Ok(g) => wire::serialize_get_guardrail_response(&wire::GetGuardrailResponse {
                guardrail_arn: Some(g.guardrail_arn.clone()),
                guardrail_id: Some(g.guardrail_id.clone()),
                name: Some(g.name.clone()),
                description: g.description.clone(),
                status: Some(g.status.clone()),
                version: Some(g.version.clone()),
                created_at: Some(g.created_at.clone()),
                updated_at: Some(g.updated_at.clone()),
                blocked_input_messaging: Some(g.blocked_input_messaging.clone()),
                blocked_outputs_messaging: Some(g.blocked_outputs_messaging.clone()),
                ..Default::default()
            }),
            Err(e) => bedrock_error_response(&e),
        }
    }

    async fn handle_list_guardrails(
        &self,
        state: &Arc<tokio::sync::RwLock<BedrockState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let guardrails = state.list_guardrails();
        let summaries: Vec<wire::GuardrailSummary> = guardrails
            .iter()
            .map(|g| wire::GuardrailSummary {
                arn: Some(g.guardrail_arn.clone()),
                id: Some(g.guardrail_id.clone()),
                name: Some(g.name.clone()),
                description: g.description.clone(),
                status: Some(g.status.clone()),
                version: Some(g.version.clone()),
                created_at: Some(g.created_at.clone()),
                updated_at: Some(g.updated_at.clone()),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_guardrails_response(&wire::ListGuardrailsResponse {
            guardrails: Some(summaries),
            next_token: None,
        })
    }

    async fn handle_update_guardrail(
        &self,
        state: &Arc<tokio::sync::RwLock<BedrockState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_guardrail_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let name = if input.name.is_empty() {
            None
        } else {
            Some(input.name.as_str())
        };
        let description = input.description.clone();
        let blocked_input = if input.blocked_input_messaging.is_empty() {
            None
        } else {
            Some(input.blocked_input_messaging.as_str())
        };
        let blocked_output = if input.blocked_outputs_messaging.is_empty() {
            None
        } else {
            Some(input.blocked_outputs_messaging.as_str())
        };
        let mut state = state.write().await;
        match state.update_guardrail(
            &input.guardrail_identifier,
            name,
            description,
            blocked_input,
            blocked_output,
        ) {
            Ok(id) => wire::serialize_update_guardrail_response(&wire::UpdateGuardrailResponse {
                guardrail_id: Some(id),
                ..Default::default()
            }),
            Err(e) => bedrock_error_response(&e),
        }
    }

    async fn handle_delete_guardrail(
        &self,
        state: &Arc<tokio::sync::RwLock<BedrockState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_guardrail_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.delete_guardrail(&input.guardrail_identifier) {
            Ok(()) => wire::serialize_delete_guardrail_response(&wire::DeleteGuardrailResponse {}),
            Err(e) => bedrock_error_response(&e),
        }
    }

    // ---- Provisioned Model Throughput ----

    async fn handle_create_provisioned_model_throughput(
        &self,
        state: &Arc<tokio::sync::RwLock<BedrockState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_provisioned_model_throughput_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.model_id.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'modelId'");
        }
        if input.provisioned_model_name.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'provisionedModelName'");
        }
        let model_units = if input.model_units == 0 {
            1
        } else {
            input.model_units
        };
        let commitment_duration = input.commitment_duration.clone();
        let mut state = state.write().await;
        match state.create_provisioned_model_throughput(
            &input.model_id,
            &input.provisioned_model_name,
            model_units,
            commitment_duration,
            account_id,
            region,
        ) {
            Ok(p) => wire::serialize_create_provisioned_model_throughput_response(
                &wire::CreateProvisionedModelThroughputResponse {
                    provisioned_model_arn: Some(p.provisioned_model_arn.clone()),
                },
            ),
            Err(e) => bedrock_error_response(&e),
        }
    }

    async fn handle_get_provisioned_model_throughput(
        &self,
        state: &Arc<tokio::sync::RwLock<BedrockState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_provisioned_model_throughput_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_provisioned_model_throughput(&input.provisioned_model_id) {
            Ok(p) => wire::serialize_get_provisioned_model_throughput_response(
                &wire::GetProvisionedModelThroughputResponse {
                    provisioned_model_arn: Some(p.provisioned_model_arn.clone()),
                    provisioned_model_name: Some(p.provisioned_model_name.clone()),
                    model_arn: Some(p.model_arn.clone()),
                    desired_model_arn: Some(p.model_arn.clone()),
                    model_units: Some(p.model_units),
                    desired_model_units: Some(p.model_units),
                    status: Some(p.status.clone()),
                    commitment_duration: p.commitment_duration.clone(),
                    creation_time: Some(p.creation_time.clone()),
                    last_modified_time: Some(p.last_modified_time.clone()),
                    ..Default::default()
                },
            ),
            Err(e) => bedrock_error_response(&e),
        }
    }

    async fn handle_list_provisioned_model_throughputs(
        &self,
        state: &Arc<tokio::sync::RwLock<BedrockState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let provisioned = state.list_provisioned_model_throughputs();
        let summaries: Vec<wire::ProvisionedModelSummary> = provisioned
            .iter()
            .map(|p| wire::ProvisionedModelSummary {
                provisioned_model_arn: Some(p.provisioned_model_arn.clone()),
                provisioned_model_name: Some(p.provisioned_model_name.clone()),
                model_arn: Some(p.model_arn.clone()),
                desired_model_arn: Some(p.model_arn.clone()),
                model_units: Some(p.model_units),
                desired_model_units: Some(p.model_units),
                status: Some(p.status.clone()),
                commitment_duration: p.commitment_duration.clone(),
                creation_time: Some(p.creation_time.clone()),
                last_modified_time: Some(p.last_modified_time.clone()),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_provisioned_model_throughputs_response(
            &wire::ListProvisionedModelThroughputsResponse {
                provisioned_model_summaries: Some(summaries),
                next_token: None,
            },
        )
    }

    async fn handle_update_provisioned_model_throughput(
        &self,
        state: &Arc<tokio::sync::RwLock<BedrockState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_provisioned_model_throughput_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let desired_model_id = input.desired_model_id.as_deref();
        let desired_name = input.desired_provisioned_model_name.as_deref();
        let mut state = state.write().await;
        match state.update_provisioned_model_throughput(
            &input.provisioned_model_id,
            desired_model_id,
            desired_name,
        ) {
            Ok(()) => wire::serialize_update_provisioned_model_throughput_response(
                &wire::UpdateProvisionedModelThroughputResponse {},
            ),
            Err(e) => bedrock_error_response(&e),
        }
    }

    async fn handle_delete_provisioned_model_throughput(
        &self,
        state: &Arc<tokio::sync::RwLock<BedrockState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_provisioned_model_throughput_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.delete_provisioned_model_throughput(&input.provisioned_model_id) {
            Ok(()) => wire::serialize_delete_provisioned_model_throughput_response(
                &wire::DeleteProvisionedModelThroughputResponse {},
            ),
            Err(e) => bedrock_error_response(&e),
        }
    }

    // ---- Inference Profiles ----

    async fn handle_create_inference_profile(
        &self,
        state: &Arc<tokio::sync::RwLock<BedrockState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_inference_profile_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.inference_profile_name.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'inferenceProfileName'");
        }
        let model_source_arn = input.model_source.copy_from.as_deref().unwrap_or("");
        let description = input.description.clone();
        let mut state = state.write().await;
        match state.create_inference_profile(
            &input.inference_profile_name,
            description,
            model_source_arn,
            account_id,
            region,
        ) {
            Ok(p) => wire::serialize_create_inference_profile_response(
                &wire::CreateInferenceProfileResponse {
                    inference_profile_arn: Some(p.inference_profile_arn.clone()),
                    status: Some(p.status.clone()),
                },
            ),
            Err(e) => bedrock_error_response(&e),
        }
    }

    async fn handle_get_inference_profile(
        &self,
        state: &Arc<tokio::sync::RwLock<BedrockState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_inference_profile_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_inference_profile(&input.inference_profile_identifier) {
            Ok(p) => {
                wire::serialize_get_inference_profile_response(&wire::GetInferenceProfileResponse {
                    inference_profile_arn: Some(p.inference_profile_arn.clone()),
                    inference_profile_id: Some(p.inference_profile_id.clone()),
                    inference_profile_name: Some(p.inference_profile_name.clone()),
                    description: p.description.clone(),
                    status: Some(p.status.clone()),
                    r#type: Some(p.profile_type.clone()),
                    models: Some(vec![wire::InferenceProfileModel {
                        model_arn: Some(p.model_arn.clone()),
                    }]),
                    created_at: Some(p.created_at.clone()),
                    updated_at: Some(p.updated_at.clone()),
                })
            }
            Err(e) => bedrock_error_response(&e),
        }
    }

    async fn handle_list_inference_profiles(
        &self,
        state: &Arc<tokio::sync::RwLock<BedrockState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let profiles = state.list_inference_profiles();
        let summaries: Vec<wire::InferenceProfileSummary> = profiles
            .iter()
            .map(|p| wire::InferenceProfileSummary {
                inference_profile_arn: Some(p.inference_profile_arn.clone()),
                inference_profile_id: Some(p.inference_profile_id.clone()),
                inference_profile_name: Some(p.inference_profile_name.clone()),
                description: p.description.clone(),
                status: Some(p.status.clone()),
                r#type: Some(p.profile_type.clone()),
                models: Some(vec![wire::InferenceProfileModel {
                    model_arn: Some(p.model_arn.clone()),
                }]),
                created_at: Some(p.created_at.clone()),
                updated_at: Some(p.updated_at.clone()),
            })
            .collect();
        wire::serialize_list_inference_profiles_response(&wire::ListInferenceProfilesResponse {
            inference_profile_summaries: Some(summaries),
            next_token: None,
        })
    }

    async fn handle_delete_inference_profile(
        &self,
        state: &Arc<tokio::sync::RwLock<BedrockState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_inference_profile_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.delete_inference_profile(&input.inference_profile_identifier) {
            Ok(()) => wire::serialize_delete_inference_profile_response(
                &wire::DeleteInferenceProfileResponse {},
            ),
            Err(e) => bedrock_error_response(&e),
        }
    }

    // ---- Prompt Routers ----

    async fn handle_create_prompt_router(
        &self,
        state: &Arc<tokio::sync::RwLock<BedrockState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_prompt_router_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.prompt_router_name.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'promptRouterName'");
        }
        let fallback_model_arn = input.fallback_model.model_arn.clone();
        let models: Vec<String> = input.models.iter().map(|m| m.model_arn.clone()).collect();
        let routing_criteria = input.routing_criteria.response_quality_difference;
        let description = input.description.clone();
        let mut state = state.write().await;
        match state.create_prompt_router(
            &input.prompt_router_name,
            description,
            &fallback_model_arn,
            models,
            routing_criteria,
            account_id,
            region,
        ) {
            Ok(r) => {
                wire::serialize_create_prompt_router_response(&wire::CreatePromptRouterResponse {
                    prompt_router_arn: Some(r.prompt_router_arn.clone()),
                })
            }
            Err(e) => bedrock_error_response(&e),
        }
    }

    async fn handle_get_prompt_router(
        &self,
        state: &Arc<tokio::sync::RwLock<BedrockState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_prompt_router_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_prompt_router(&input.prompt_router_arn) {
            Ok(r) => {
                let models: Vec<wire::PromptRouterTargetModel> = r
                    .models
                    .iter()
                    .map(|m| wire::PromptRouterTargetModel {
                        model_arn: m.clone(),
                    })
                    .collect();
                wire::serialize_get_prompt_router_response(&wire::GetPromptRouterResponse {
                    prompt_router_arn: Some(r.prompt_router_arn.clone()),
                    prompt_router_name: Some(r.prompt_router_name.clone()),
                    description: r.description.clone(),
                    status: Some(r.status.clone()),
                    r#type: Some(r.router_type.clone()),
                    fallback_model: Some(wire::PromptRouterTargetModel {
                        model_arn: r.fallback_model_arn.clone(),
                    }),
                    models: Some(models),
                    routing_criteria: Some(wire::RoutingCriteria {
                        response_quality_difference: r.routing_criteria_response_quality_difference,
                    }),
                    created_at: Some(r.created_at.clone()),
                    updated_at: Some(r.updated_at.clone()),
                })
            }
            Err(e) => bedrock_error_response(&e),
        }
    }

    async fn handle_list_prompt_routers(
        &self,
        state: &Arc<tokio::sync::RwLock<BedrockState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let routers = state.list_prompt_routers();
        let summaries: Vec<wire::PromptRouterSummary> = routers
            .iter()
            .map(|r| {
                let models: Vec<wire::PromptRouterTargetModel> = r
                    .models
                    .iter()
                    .map(|m| wire::PromptRouterTargetModel {
                        model_arn: m.clone(),
                    })
                    .collect();
                wire::PromptRouterSummary {
                    prompt_router_arn: Some(r.prompt_router_arn.clone()),
                    prompt_router_name: Some(r.prompt_router_name.clone()),
                    description: r.description.clone(),
                    status: Some(r.status.clone()),
                    r#type: Some(r.router_type.clone()),
                    fallback_model: Some(wire::PromptRouterTargetModel {
                        model_arn: r.fallback_model_arn.clone(),
                    }),
                    models: Some(models),
                    routing_criteria: Some(wire::RoutingCriteria {
                        response_quality_difference: r.routing_criteria_response_quality_difference,
                    }),
                    created_at: Some(r.created_at.clone()),
                    updated_at: Some(r.updated_at.clone()),
                }
            })
            .collect();
        wire::serialize_list_prompt_routers_response(&wire::ListPromptRoutersResponse {
            prompt_router_summaries: Some(summaries),
            next_token: None,
        })
    }

    async fn handle_delete_prompt_router(
        &self,
        state: &Arc<tokio::sync::RwLock<BedrockState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_prompt_router_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.delete_prompt_router(&input.prompt_router_arn) {
            Ok(()) => {
                wire::serialize_delete_prompt_router_response(&wire::DeletePromptRouterResponse {})
            }
            Err(e) => bedrock_error_response(&e),
        }
    }

    // ---- Evaluation Jobs ----

    async fn handle_create_evaluation_job(
        &self,
        state: &Arc<tokio::sync::RwLock<BedrockState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_evaluation_job_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.job_name.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'jobName'");
        }
        let job_description = input.job_description.clone();
        let mut state = state.write().await;
        match state.create_evaluation_job(&input.job_name, job_description, account_id, region) {
            Ok(j) => {
                wire::serialize_create_evaluation_job_response(&wire::CreateEvaluationJobResponse {
                    job_arn: Some(j.job_arn.clone()),
                })
            }
            Err(e) => bedrock_error_response(&e),
        }
    }

    async fn handle_get_evaluation_job(
        &self,
        state: &Arc<tokio::sync::RwLock<BedrockState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_evaluation_job_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_evaluation_job(&input.job_identifier) {
            Ok(j) => wire::serialize_get_evaluation_job_response(&wire::GetEvaluationJobResponse {
                job_arn: Some(j.job_arn.clone()),
                job_name: Some(j.job_name.clone()),
                job_description: j.job_description.clone(),
                status: Some(j.status.clone()),
                creation_time: Some(j.creation_time.clone()),
                last_modified_time: Some(j.last_modified_time.clone()),
                ..Default::default()
            }),
            Err(e) => bedrock_error_response(&e),
        }
    }

    async fn handle_list_evaluation_jobs(
        &self,
        state: &Arc<tokio::sync::RwLock<BedrockState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let jobs = state.list_evaluation_jobs();
        let summaries: Vec<wire::EvaluationSummary> = jobs
            .iter()
            .map(|j| wire::EvaluationSummary {
                job_arn: Some(j.job_arn.clone()),
                job_name: Some(j.job_name.clone()),
                status: Some(j.status.clone()),
                creation_time: Some(j.creation_time.clone()),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_evaluation_jobs_response(&wire::ListEvaluationJobsResponse {
            job_summaries: Some(summaries),
            next_token: None,
        })
    }

    async fn handle_stop_evaluation_job(
        &self,
        state: &Arc<tokio::sync::RwLock<BedrockState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_stop_evaluation_job_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.stop_evaluation_job(&input.job_identifier) {
            Ok(()) => {
                wire::serialize_stop_evaluation_job_response(&wire::StopEvaluationJobResponse {})
            }
            Err(e) => bedrock_error_response(&e),
        }
    }

    // ---- Model Invocation Jobs ----

    async fn handle_create_model_invocation_job(
        &self,
        state: &Arc<tokio::sync::RwLock<BedrockState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input =
            match wire::deserialize_create_model_invocation_job_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        if input.job_name.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'jobName'");
        }
        if input.model_id.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'modelId'");
        }
        if input.role_arn.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'roleArn'");
        }
        let input_s3_uri = input
            .input_data_config
            .s3_input_data_config
            .as_ref()
            .map(|s| s.s3_uri.as_str())
            .unwrap_or("");
        let output_s3_uri = input
            .output_data_config
            .s3_output_data_config
            .as_ref()
            .map(|s| s.s3_uri.as_str())
            .unwrap_or("");
        let mut state = state.write().await;
        match state.create_model_invocation_job(
            &input.job_name,
            &input.model_id,
            &input.role_arn,
            input_s3_uri,
            output_s3_uri,
            account_id,
            region,
        ) {
            Ok(j) => wire::serialize_create_model_invocation_job_response(
                &wire::CreateModelInvocationJobResponse {
                    job_arn: Some(j.job_arn.clone()),
                },
            ),
            Err(e) => bedrock_error_response(&e),
        }
    }

    async fn handle_get_model_invocation_job(
        &self,
        state: &Arc<tokio::sync::RwLock<BedrockState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_model_invocation_job_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_model_invocation_job(&input.job_identifier) {
            Ok(j) => wire::serialize_get_model_invocation_job_response(
                &wire::GetModelInvocationJobResponse {
                    job_arn: Some(j.job_arn.clone()),
                    job_name: Some(j.job_name.clone()),
                    model_id: Some(j.model_id.clone()),
                    role_arn: Some(j.role_arn.clone()),
                    status: Some(j.status.clone()),
                    submit_time: Some(j.submit_time.clone()),
                    last_modified_time: Some(j.last_modified_time.clone()),
                    input_data_config: Some(wire::ModelInvocationJobInputDataConfig {
                        s3_input_data_config: Some(wire::ModelInvocationJobS3InputDataConfig {
                            s3_uri: j.input_s3_uri.clone(),
                            ..Default::default()
                        }),
                    }),
                    output_data_config: Some(wire::ModelInvocationJobOutputDataConfig {
                        s3_output_data_config: Some(wire::ModelInvocationJobS3OutputDataConfig {
                            s3_uri: j.output_s3_uri.clone(),
                            ..Default::default()
                        }),
                    }),
                    ..Default::default()
                },
            ),
            Err(e) => bedrock_error_response(&e),
        }
    }

    async fn handle_list_model_invocation_jobs(
        &self,
        state: &Arc<tokio::sync::RwLock<BedrockState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let jobs = state.list_model_invocation_jobs();
        let summaries: Vec<wire::ModelInvocationJobSummary> = jobs
            .iter()
            .map(|j| wire::ModelInvocationJobSummary {
                job_arn: Some(j.job_arn.clone()),
                job_name: Some(j.job_name.clone()),
                model_id: Some(j.model_id.clone()),
                role_arn: Some(j.role_arn.clone()),
                status: Some(j.status.clone()),
                submit_time: Some(j.submit_time.clone()),
                last_modified_time: Some(j.last_modified_time.clone()),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_model_invocation_jobs_response(
            &wire::ListModelInvocationJobsResponse {
                invocation_job_summaries: Some(summaries),
                next_token: None,
            },
        )
    }

    async fn handle_stop_model_invocation_job(
        &self,
        state: &Arc<tokio::sync::RwLock<BedrockState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_stop_model_invocation_job_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let mut state = state.write().await;
        match state.stop_model_invocation_job(&input.job_identifier) {
            Ok(()) => wire::serialize_stop_model_invocation_job_response(
                &wire::StopModelInvocationJobResponse {},
            ),
            Err(e) => bedrock_error_response(&e),
        }
    }

    // ---- Model Import Jobs ----

    async fn handle_create_model_import_job(
        &self,
        state: &Arc<tokio::sync::RwLock<BedrockState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_model_import_job_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.job_name.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'jobName'");
        }
        if input.imported_model_name.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'importedModelName'");
        }
        if input.role_arn.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'roleArn'");
        }
        let mut state = state.write().await;
        match state.create_model_import_job(
            &input.job_name,
            &input.imported_model_name,
            &input.role_arn,
            account_id,
            region,
        ) {
            Ok(j) => wire::serialize_create_model_import_job_response(
                &wire::CreateModelImportJobResponse {
                    job_arn: Some(j.job_arn.clone()),
                },
            ),
            Err(e) => bedrock_error_response(&e),
        }
    }

    async fn handle_get_model_import_job(
        &self,
        state: &Arc<tokio::sync::RwLock<BedrockState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_model_import_job_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_model_import_job(&input.job_identifier) {
            Ok(j) => {
                wire::serialize_get_model_import_job_response(&wire::GetModelImportJobResponse {
                    job_arn: Some(j.job_arn.clone()),
                    job_name: Some(j.job_name.clone()),
                    imported_model_name: Some(j.imported_model_name.clone()),
                    imported_model_arn: Some(j.imported_model_arn.clone()),
                    role_arn: Some(j.role_arn.clone()),
                    status: Some(j.status.clone()),
                    creation_time: Some(j.creation_time.clone()),
                    last_modified_time: Some(j.last_modified_time.clone()),
                    ..Default::default()
                })
            }
            Err(e) => bedrock_error_response(&e),
        }
    }

    async fn handle_list_model_import_jobs(
        &self,
        state: &Arc<tokio::sync::RwLock<BedrockState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let jobs = state.list_model_import_jobs();
        let summaries: Vec<wire::ModelImportJobSummary> = jobs
            .iter()
            .map(|j| wire::ModelImportJobSummary {
                job_arn: Some(j.job_arn.clone()),
                job_name: Some(j.job_name.clone()),
                imported_model_name: Some(j.imported_model_name.clone()),
                imported_model_arn: Some(j.imported_model_arn.clone()),
                status: Some(j.status.clone()),
                creation_time: Some(j.creation_time.clone()),
                last_modified_time: Some(j.last_modified_time.clone()),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_model_import_jobs_response(&wire::ListModelImportJobsResponse {
            model_import_job_summaries: Some(summaries),
            next_token: None,
        })
    }

    // ---- Model Copy Jobs ----

    async fn handle_create_model_copy_job(
        &self,
        state: &Arc<tokio::sync::RwLock<BedrockState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_model_copy_job_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.source_model_arn.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'sourceModelArn'");
        }
        if input.target_model_name.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'targetModelName'");
        }
        let source_account_id = account_id;
        let mut state = state.write().await;
        match state.create_model_copy_job(
            &input.source_model_arn,
            &input.target_model_name,
            source_account_id,
            account_id,
            region,
        ) {
            Ok(j) => {
                wire::serialize_create_model_copy_job_response(&wire::CreateModelCopyJobResponse {
                    job_arn: Some(j.job_arn.clone()),
                })
            }
            Err(e) => bedrock_error_response(&e),
        }
    }

    async fn handle_get_model_copy_job(
        &self,
        state: &Arc<tokio::sync::RwLock<BedrockState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_model_copy_job_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_model_copy_job(&input.job_arn) {
            Ok(j) => wire::serialize_get_model_copy_job_response(&wire::GetModelCopyJobResponse {
                job_arn: Some(j.job_arn.clone()),
                source_model_arn: Some(j.source_model_arn.clone()),
                source_account_id: Some(j.source_account_id.clone()),
                target_model_arn: Some(j.target_model_arn.clone()),
                target_model_name: Some(j.target_model_name.clone()),
                status: Some(j.status.clone()),
                creation_time: Some(j.creation_time.clone()),
                ..Default::default()
            }),
            Err(e) => bedrock_error_response(&e),
        }
    }

    async fn handle_list_model_copy_jobs(
        &self,
        state: &Arc<tokio::sync::RwLock<BedrockState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let jobs = state.list_model_copy_jobs();
        let summaries: Vec<wire::ModelCopyJobSummary> = jobs
            .iter()
            .map(|j| wire::ModelCopyJobSummary {
                job_arn: Some(j.job_arn.clone()),
                source_model_arn: Some(j.source_model_arn.clone()),
                source_account_id: Some(j.source_account_id.clone()),
                target_model_arn: Some(j.target_model_arn.clone()),
                target_model_name: Some(j.target_model_name.clone()),
                status: Some(j.status.clone()),
                creation_time: Some(j.creation_time.clone()),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_model_copy_jobs_response(&wire::ListModelCopyJobsResponse {
            model_copy_job_summaries: Some(summaries),
            next_token: None,
        })
    }
}

/// Convert a state FoundationModel to a wire FoundationModelSummary.
fn foundation_model_to_summary(m: &FoundationModel) -> wire::FoundationModelSummary {
    wire::FoundationModelSummary {
        model_id: Some(m.model_id.clone()),
        model_name: Some(m.model_name.clone()),
        provider_name: Some(m.provider_name.clone()),
        model_arn: Some(m.model_arn.clone()),
        input_modalities: Some(m.input_modalities.clone()),
        output_modalities: Some(m.output_modalities.clone()),
        response_streaming_supported: Some(m.response_streaming_supported),
        customizations_supported: Some(m.customizations_supported.clone()),
        inference_types_supported: Some(m.inference_types_supported.clone()),
        model_lifecycle: Some(wire::FoundationModelLifecycle {
            status: Some(m.model_lifecycle_status.clone()),
            ..Default::default()
        }),
        ..Default::default()
    }
}

/// Convert a state FoundationModel to a wire FoundationModelDetails.
fn foundation_model_to_detail(m: &FoundationModel) -> wire::FoundationModelDetails {
    wire::FoundationModelDetails {
        model_id: Some(m.model_id.clone()),
        model_name: Some(m.model_name.clone()),
        provider_name: Some(m.provider_name.clone()),
        model_arn: Some(m.model_arn.clone()),
        input_modalities: Some(m.input_modalities.clone()),
        output_modalities: Some(m.output_modalities.clone()),
        response_streaming_supported: Some(m.response_streaming_supported),
        customizations_supported: Some(m.customizations_supported.clone()),
        inference_types_supported: Some(m.inference_types_supported.clone()),
        model_lifecycle: Some(wire::FoundationModelLifecycle {
            status: Some(m.model_lifecycle_status.clone()),
            ..Default::default()
        }),
        ..Default::default()
    }
}

fn extract_query(uri: &str) -> String {
    if let Some(idx) = uri.find('?') {
        uri[idx + 1..].to_string()
    } else {
        String::new()
    }
}

fn extract_path(uri: &str) -> String {
    // Delegate to the shared core helper, which correctly strips the scheme
    // and host (including custom-endpoint hostnames like `127.0.0.1:PORT`)
    // before returning the path. The previous implementation only matched on
    // `amazonaws.com` and returned the entire URI for non-AWS endpoints,
    // causing dispatch to fail with 404 against the in-process mock server.
    winterbaume_core::extract_path(uri)
}

fn percent_decode(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut bytes = s.bytes();
    while let Some(b) = bytes.next() {
        match b {
            b'%' => {
                let hi = bytes.next().and_then(hex_val);
                let lo = bytes.next().and_then(hex_val);
                if let (Some(hi), Some(lo)) = (hi, lo) {
                    result.push((hi << 4 | lo) as char);
                }
            }
            b'+' => result.push(' '),
            _ => result.push(b as char),
        }
    }
    result
}

fn hex_val(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

fn bedrock_error_response(err: &BedrockError) -> MockResponse {
    let (status, error_type) = match err {
        BedrockError::ModelNotFound(_) => (404, "ResourceNotFoundException"),
        BedrockError::CustomizationJobNotFound(_) => (404, "ResourceNotFoundException"),
        BedrockError::CustomModelNotFound(_) => (404, "ResourceNotFoundException"),
        BedrockError::GuardrailNotFound(_) => (404, "ResourceNotFoundException"),
        BedrockError::ProvisionedModelNotFound(_) => (404, "ResourceNotFoundException"),
        BedrockError::InferenceProfileNotFound(_) => (404, "ResourceNotFoundException"),
        BedrockError::PromptRouterNotFound(_) => (404, "ResourceNotFoundException"),
        BedrockError::EvaluationJobNotFound(_) => (404, "ResourceNotFoundException"),
        BedrockError::ModelInvocationJobNotFound(_) => (404, "ResourceNotFoundException"),
        BedrockError::ModelImportJobNotFound(_) => (404, "ResourceNotFoundException"),
        BedrockError::ModelCopyJobNotFound(_) => (404, "ResourceNotFoundException"),
        BedrockError::CustomizationJobConflict(_) => (400, "ConflictException"),
        BedrockError::CustomizationJobNotInProgress(_) => (400, "ConflictException"),
        BedrockError::GuardrailConflict(_) => (400, "ConflictException"),
        BedrockError::ProvisionedModelConflict(_) => (400, "ConflictException"),
        BedrockError::InferenceProfileConflict(_) => (400, "ConflictException"),
        BedrockError::PromptRouterConflict(_) => (400, "ConflictException"),
        BedrockError::EvaluationJobConflict(_) => (400, "ConflictException"),
        BedrockError::ModelInvocationJobConflict(_) => (400, "ConflictException"),
    };
    let body = json!({
        "message": err.to_string(),
    });
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}

fn rest_json_error(status: u16, code: &str, message: &str) -> MockResponse {
    let body = json!({
        "message": message,
    });
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers.insert(X_AMZN_ERRORTYPE, code.parse().unwrap());
    resp
}
