use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, StatefulService,
    default_account_id,
};

use crate::state::{JobDefinitionType, SageMakerError, SageMakerState};
use crate::types::*;
use crate::views::SagemakerStateView;
use crate::wire;

/// SageMaker service handler that processes awsJson1.1 protocol requests.
pub struct SageMakerService {
    pub(crate) state: Arc<BackendState<SageMakerState>>,
    pub(crate) notifier: StateChangeNotifier<SagemakerStateView>,
}

impl SageMakerService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for SageMakerService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for SageMakerService {
    fn service_name(&self) -> &str {
        "sagemaker"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://api\.sagemaker\.(.+)\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

// ============================================================
// Helper macros and functions
// ============================================================

fn get_str<'a>(body: &'a Value, key: &str) -> Option<&'a str> {
    body.get(key).and_then(|v| v.as_str())
}

fn require_str<'a>(body: &'a Value, key: &str) -> Result<&'a str, MockResponse> {
    get_str(body, key)
        .ok_or_else(|| json_error_response(400, "ValidationException", &format!("Missing '{key}'")))
}

fn get_tags_from_body(body: &Value) -> Vec<TagPair> {
    body.get("Tags")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|t| {
                    let key = t.get("Key")?.as_str()?.to_string();
                    let value = t.get("Value")?.as_str()?.to_string();
                    Some(TagPair { key, value })
                })
                .collect()
        })
        .unwrap_or_default()
}

fn get_tag_keys_from_body(body: &Value) -> Vec<String> {
    body.get("TagKeys")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default()
}

fn tags_to_wire(tags: &[TagPair]) -> Vec<wire::Tag> {
    tags.iter()
        .map(|t| wire::Tag {
            key: t.key.clone(),
            value: t.value.clone(),
        })
        .collect()
}

fn sagemaker_error_response(err: &SageMakerError) -> MockResponse {
    let (status, error_type) = match err {
        SageMakerError::NotFound { .. } => (400u16, "ValidationException"),
        SageMakerError::AlreadyExists { .. } => (400u16, "ValidationException"),
        SageMakerError::ResourceNotFound { .. } => (404u16, "ResourceNotFound"),
        SageMakerError::ModelPackageGroupNotFound => (400u16, "ValidationException"),
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

fn empty_json_response() -> MockResponse {
    MockResponse::json(200, "{}".to_string())
}

// ============================================================
// Dispatch
// ============================================================

impl SageMakerService {
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

        let body: Value = match serde_json::from_slice(&request.body) {
            Ok(v) => v,
            Err(_) => {
                return json_error_response(400, "SerializationException", "Invalid JSON body");
            }
        };

        let state = self.state.get(account_id, &region);

        let response = match action.as_str() {
            // Notebook Instances
            "CreateNotebookInstance" => {
                self.handle_create_notebook_instance(&state, &body, account_id, &region)
                    .await
            }
            "DescribeNotebookInstance" => {
                self.handle_describe_notebook_instance(&state, &body).await
            }
            "DeleteNotebookInstance" => self.handle_delete_notebook_instance(&state, &body).await,
            "ListNotebookInstances" => self.handle_list_notebook_instances(&state).await,
            "StartNotebookInstance" => self.handle_start_notebook_instance(&state, &body).await,
            "StopNotebookInstance" => self.handle_stop_notebook_instance(&state, &body).await,

            // Models
            "CreateModel" => {
                self.handle_create_model(&state, &body, account_id, &region)
                    .await
            }
            "DescribeModel" => self.handle_describe_model(&state, &body).await,
            "DeleteModel" => self.handle_delete_model(&state, &body).await,
            "ListModels" => self.handle_list_models(&state).await,

            // Endpoint Configs
            "CreateEndpointConfig" => {
                self.handle_create_endpoint_config(&state, &body, account_id, &region)
                    .await
            }
            "DescribeEndpointConfig" => self.handle_describe_endpoint_config(&state, &body).await,
            "DeleteEndpointConfig" => self.handle_delete_endpoint_config(&state, &body).await,
            "ListEndpointConfigs" => self.handle_list_endpoint_configs(&state).await,

            // Endpoints
            "CreateEndpoint" => {
                self.handle_create_endpoint(&state, &body, account_id, &region)
                    .await
            }
            "DescribeEndpoint" => self.handle_describe_endpoint(&state, &body).await,
            "DeleteEndpoint" => self.handle_delete_endpoint(&state, &body).await,
            "ListEndpoints" => self.handle_list_endpoints(&state).await,
            "UpdateEndpointWeightsAndCapacities" => {
                self.handle_update_endpoint_weights_and_capacities(&state, &body)
                    .await
            }

            // Training Jobs
            "CreateTrainingJob" => {
                self.handle_create_training_job(&state, &body, account_id, &region)
                    .await
            }
            "DescribeTrainingJob" => self.handle_describe_training_job(&state, &body).await,
            "StopTrainingJob" => self.handle_stop_training_job(&state, &body).await,
            "DeleteTrainingJob" => self.handle_delete_training_job(&state, &body).await,
            "ListTrainingJobs" => self.handle_list_training_jobs(&state).await,

            // Processing Jobs
            "CreateProcessingJob" => {
                self.handle_create_processing_job(&state, &body, account_id, &region)
                    .await
            }
            "DescribeProcessingJob" => self.handle_describe_processing_job(&state, &body).await,
            "StopProcessingJob" => self.handle_stop_processing_job(&state, &body).await,
            "DeleteProcessingJob" => self.handle_delete_processing_job(&state, &body).await,
            "ListProcessingJobs" => self.handle_list_processing_jobs(&state).await,

            // Transform Jobs
            "CreateTransformJob" => {
                self.handle_create_transform_job(&state, &body, account_id, &region)
                    .await
            }
            "DescribeTransformJob" => self.handle_describe_transform_job(&state, &body).await,
            "StopTransformJob" => self.handle_stop_transform_job(&state, &body).await,
            "ListTransformJobs" => self.handle_list_transform_jobs(&state).await,

            // HyperParameter Tuning Jobs
            "CreateHyperParameterTuningJob" => {
                self.handle_create_hyper_parameter_tuning_job(&state, &body, account_id, &region)
                    .await
            }
            "DescribeHyperParameterTuningJob" => {
                self.handle_describe_hyper_parameter_tuning_job(&state, &body)
                    .await
            }
            "DeleteHyperParameterTuningJob" => {
                self.handle_delete_hyper_parameter_tuning_job(&state, &body)
                    .await
            }
            "ListHyperParameterTuningJobs" => {
                self.handle_list_hyper_parameter_tuning_jobs(&state).await
            }

            // Compilation Jobs
            "CreateCompilationJob" => {
                self.handle_create_compilation_job(&state, &body, account_id, &region)
                    .await
            }
            "DescribeCompilationJob" => self.handle_describe_compilation_job(&state, &body).await,
            "DeleteCompilationJob" => self.handle_delete_compilation_job(&state, &body).await,
            "ListCompilationJobs" => self.handle_list_compilation_jobs(&state).await,

            // AutoML Jobs V2
            "CreateAutoMLJobV2" => {
                self.handle_create_auto_ml_job_v2(&state, &body, account_id, &region)
                    .await
            }
            "DescribeAutoMLJobV2" => self.handle_describe_auto_ml_job_v2(&state, &body).await,
            "StopAutoMLJob" => self.handle_stop_auto_ml_job(&state, &body).await,
            "ListAutoMLJobs" => self.handle_list_auto_ml_jobs(&state).await,

            // Experiments
            "CreateExperiment" => {
                self.handle_create_experiment(&state, &body, account_id, &region)
                    .await
            }
            "DescribeExperiment" => self.handle_describe_experiment(&state, &body).await,
            "UpdateExperiment" => self.handle_update_experiment(&state, &body).await,
            "DeleteExperiment" => self.handle_delete_experiment(&state, &body).await,
            "ListExperiments" => self.handle_list_experiments(&state).await,

            // Trials
            "CreateTrial" => {
                self.handle_create_trial(&state, &body, account_id, &region)
                    .await
            }
            "DescribeTrial" => self.handle_describe_trial(&state, &body).await,
            "UpdateTrial" => self.handle_update_trial(&state, &body).await,
            "DeleteTrial" => self.handle_delete_trial(&state, &body).await,
            "ListTrials" => self.handle_list_trials(&state, &body).await,

            // Trial Components
            "CreateTrialComponent" => {
                self.handle_create_trial_component(&state, &body, account_id, &region)
                    .await
            }
            "DescribeTrialComponent" => self.handle_describe_trial_component(&state, &body).await,
            "DeleteTrialComponent" => self.handle_delete_trial_component(&state, &body).await,
            "UpdateTrialComponent" => self.handle_update_trial_component(&state, &body).await,
            "ListTrialComponents" => self.handle_list_trial_components(&state, &body).await,
            "AssociateTrialComponent" => self.handle_associate_trial_component(&state, &body).await,
            "DisassociateTrialComponent" => {
                self.handle_disassociate_trial_component(&state, &body, account_id, &region)
                    .await
            }

            // Pipelines
            "CreatePipeline" => {
                self.handle_create_pipeline(&state, &body, account_id, &region)
                    .await
            }
            "DescribePipeline" => self.handle_describe_pipeline(&state, &body).await,
            "DescribePipelineDefinitionForExecution" => {
                self.handle_describe_pipeline_definition_for_execution(&state, &body)
                    .await
            }
            "DescribePipelineExecution" => {
                self.handle_describe_pipeline_execution(&state, &body).await
            }
            "UpdatePipeline" => self.handle_update_pipeline(&state, &body).await,
            "DeletePipeline" => self.handle_delete_pipeline(&state, &body).await,
            "ListPipelines" => self.handle_list_pipelines(&state).await,
            "StartPipelineExecution" => {
                self.handle_start_pipeline_execution(&state, &body, account_id, &region)
                    .await
            }
            "ListPipelineExecutions" => self.handle_list_pipeline_executions(&state, &body).await,
            "ListPipelineParametersForExecution" => {
                self.handle_list_pipeline_parameters_for_execution(&state, &body)
                    .await
            }

            // Feature Groups
            "CreateFeatureGroup" => {
                self.handle_create_feature_group(&state, &body, account_id, &region)
                    .await
            }
            "DescribeFeatureGroup" => self.handle_describe_feature_group(&state, &body).await,
            "DeleteFeatureGroup" => self.handle_delete_feature_group(&state, &body).await,
            "ListFeatureGroups" => self.handle_list_feature_groups(&state).await,

            // Domains
            "CreateDomain" => {
                self.handle_create_domain(&state, &body, account_id, &region)
                    .await
            }
            "DescribeDomain" => self.handle_describe_domain(&state, &body).await,
            "UpdateDomain" => self.handle_update_domain(&state, &body).await,
            "DeleteDomain" => self.handle_delete_domain(&state, &body).await,
            "ListDomains" => self.handle_list_domains(&state).await,

            // User Profiles
            "CreateUserProfile" => {
                self.handle_create_user_profile(&state, &body, account_id, &region)
                    .await
            }
            "DescribeUserProfile" => self.handle_describe_user_profile(&state, &body).await,
            "UpdateUserProfile" => {
                self.handle_update_user_profile(&state, &body, account_id, &region)
                    .await
            }
            "DeleteUserProfile" => self.handle_delete_user_profile(&state, &body).await,
            "ListUserProfiles" => self.handle_list_user_profiles(&state).await,

            // Spaces
            "CreateSpace" => {
                self.handle_create_space(&state, &body, account_id, &region)
                    .await
            }
            "DescribeSpace" => self.handle_describe_space(&state, &body).await,
            "UpdateSpace" => {
                self.handle_update_space(&state, &body, account_id, &region)
                    .await
            }
            "DeleteSpace" => self.handle_delete_space(&state, &body).await,
            "ListSpaces" => self.handle_list_spaces(&state).await,

            // Apps
            "CreateApp" => {
                self.handle_create_app(&state, &body, account_id, &region)
                    .await
            }
            "DescribeApp" => self.handle_describe_app(&state, &body).await,
            "DeleteApp" => self.handle_delete_app(&state, &body).await,
            "ListApps" => self.handle_list_apps(&state).await,

            // Clusters
            "CreateCluster" => {
                self.handle_create_cluster(&state, &body, account_id, &region)
                    .await
            }
            "DescribeCluster" => self.handle_describe_cluster(&state, &body).await,
            "DescribeClusterNode" => self.handle_describe_cluster_node(&state, &body).await,
            "DeleteCluster" => self.handle_delete_cluster(&state, &body).await,
            "ListClusters" => self.handle_list_clusters(&state).await,
            "ListClusterNodes" => self.handle_list_cluster_nodes(&state, &body).await,

            // Job Definitions
            "CreateDataQualityJobDefinition" => {
                self.handle_create_job_definition(
                    &state,
                    &body,
                    account_id,
                    &region,
                    JobDefinitionType::DataQuality,
                )
                .await
            }
            "DescribeDataQualityJobDefinition" => {
                self.handle_describe_job_definition(&state, &body, JobDefinitionType::DataQuality)
                    .await
            }
            "DeleteDataQualityJobDefinition" => {
                self.handle_delete_job_definition(&state, &body, JobDefinitionType::DataQuality)
                    .await
            }
            "ListDataQualityJobDefinitions" => {
                self.handle_list_job_definitions(&state, JobDefinitionType::DataQuality)
                    .await
            }
            "CreateModelQualityJobDefinition" => {
                self.handle_create_job_definition(
                    &state,
                    &body,
                    account_id,
                    &region,
                    JobDefinitionType::ModelQuality,
                )
                .await
            }
            "DescribeModelQualityJobDefinition" => {
                self.handle_describe_job_definition(&state, &body, JobDefinitionType::ModelQuality)
                    .await
            }
            "DeleteModelQualityJobDefinition" => {
                self.handle_delete_job_definition(&state, &body, JobDefinitionType::ModelQuality)
                    .await
            }
            "ListModelQualityJobDefinitions" => {
                self.handle_list_job_definitions(&state, JobDefinitionType::ModelQuality)
                    .await
            }
            "CreateModelBiasJobDefinition" => {
                self.handle_create_job_definition(
                    &state,
                    &body,
                    account_id,
                    &region,
                    JobDefinitionType::ModelBias,
                )
                .await
            }
            "DescribeModelBiasJobDefinition" => {
                self.handle_describe_job_definition(&state, &body, JobDefinitionType::ModelBias)
                    .await
            }
            "DeleteModelBiasJobDefinition" => {
                self.handle_delete_job_definition(&state, &body, JobDefinitionType::ModelBias)
                    .await
            }
            "ListModelBiasJobDefinitions" => {
                self.handle_list_job_definitions(&state, JobDefinitionType::ModelBias)
                    .await
            }
            "CreateModelExplainabilityJobDefinition" => {
                self.handle_create_job_definition(
                    &state,
                    &body,
                    account_id,
                    &region,
                    JobDefinitionType::ModelExplainability,
                )
                .await
            }
            "DescribeModelExplainabilityJobDefinition" => {
                self.handle_describe_job_definition(
                    &state,
                    &body,
                    JobDefinitionType::ModelExplainability,
                )
                .await
            }
            "DeleteModelExplainabilityJobDefinition" => {
                self.handle_delete_job_definition(
                    &state,
                    &body,
                    JobDefinitionType::ModelExplainability,
                )
                .await
            }
            "ListModelExplainabilityJobDefinitions" => {
                self.handle_list_job_definitions(&state, JobDefinitionType::ModelExplainability)
                    .await
            }

            // Model Cards
            "CreateModelCard" => {
                self.handle_create_model_card(&state, &body, account_id, &region)
                    .await
            }
            "DescribeModelCard" => self.handle_describe_model_card(&state, &body).await,
            "UpdateModelCard" => self.handle_update_model_card(&state, &body).await,
            "DeleteModelCard" => self.handle_delete_model_card(&state, &body).await,
            "ListModelCards" => self.handle_list_model_cards(&state).await,
            "ListModelCardVersions" => self.handle_list_model_card_versions(&state, &body).await,

            // Model Packages
            "CreateModelPackage" => {
                self.handle_create_model_package(&state, &body, account_id, &region)
                    .await
            }
            "DescribeModelPackage" => self.handle_describe_model_package(&state, &body).await,
            "UpdateModelPackage" => self.handle_update_model_package(&state, &body).await,
            "DeleteModelPackage" => self.handle_delete_model_package(&state, &body).await,
            "ListModelPackages" => self.handle_list_model_packages(&state).await,

            // Model Package Groups
            "CreateModelPackageGroup" => {
                self.handle_create_model_package_group(&state, &body, account_id, &region)
                    .await
            }
            "DescribeModelPackageGroup" => {
                self.handle_describe_model_package_group(&state, &body)
                    .await
            }
            "DeleteModelPackageGroup" => {
                self.handle_delete_model_package_group(&state, &body).await
            }
            "ListModelPackageGroups" => self.handle_list_model_package_groups(&state).await,

            // Endpoint (update)
            "UpdateEndpoint" => self.handle_update_endpoint(&state, &body).await,

            // Notebook Instance Lifecycle Configs
            "CreateNotebookInstanceLifecycleConfig" => {
                self.handle_create_notebook_instance_lifecycle_config(
                    &state, &body, account_id, &region,
                )
                .await
            }
            "DescribeNotebookInstanceLifecycleConfig" => {
                self.handle_describe_notebook_instance_lifecycle_config(&state, &body)
                    .await
            }
            "DeleteNotebookInstanceLifecycleConfig" => {
                self.handle_delete_notebook_instance_lifecycle_config(&state, &body)
                    .await
            }
            "ListNotebookInstanceLifecycleConfigs" => {
                self.handle_list_notebook_instance_lifecycle_configs(&state)
                    .await
            }
            "UpdateNotebookInstance" => self.handle_update_notebook_instance(&state, &body).await,

            // Tags
            "AddTags" => self.handle_add_tags(&state, &body).await,
            "DeleteTags" => self.handle_delete_tags(&state, &body).await,
            "ListTags" => self.handle_list_tags(&state, &body).await,

            // Search
            "Search" => self.handle_search(&state, &body).await,

            _ => json_error_response(
                400,
                "InvalidAction",
                &format!("Could not find operation {action} for SageMaker"),
            ),
        };

        if response.status >= 200 && response.status < 300 {
            self.notify_state_changed(account_id, &region).await;
        }

        response
    }

    // ============================================================
    // Notebook Instances
    // ============================================================

    async fn handle_create_notebook_instance(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let name = match require_str(body, "NotebookInstanceName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let instance_type = match require_str(body, "InstanceType") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let role_arn = match require_str(body, "RoleArn") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let volume_size_in_gb = body.get("VolumeSizeInGB").and_then(|v| v.as_i64());
        let direct_internet_access = get_str(body, "DirectInternetAccess");
        let root_access = get_str(body, "RootAccess");

        let mut state = state.write().await;
        match state.create_notebook_instance(
            account_id,
            region,
            name,
            instance_type,
            role_arn,
            volume_size_in_gb,
            direct_internet_access,
            root_access,
        ) {
            Ok(instance) => wire::serialize_create_notebook_instance_response(
                &wire::CreateNotebookInstanceOutput {
                    notebook_instance_arn: Some(instance.notebook_instance_arn.clone()),
                },
            ),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_describe_notebook_instance(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let name = match require_str(body, "NotebookInstanceName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let state = state.read().await;
        match state.describe_notebook_instance(name) {
            Ok(i) => wire::serialize_describe_notebook_instance_response(
                &notebook_instance_to_describe_output(i),
            ),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_delete_notebook_instance(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let name = match require_str(body, "NotebookInstanceName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let mut state = state.write().await;
        match state.delete_notebook_instance(name) {
            Ok(()) => wire::serialize_delete_notebook_instance_response(),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_list_notebook_instances(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let instances = state.list_notebook_instances();
        let entries: Vec<wire::NotebookInstanceSummary> = instances
            .iter()
            .map(|i| notebook_instance_to_summary(i))
            .collect();
        wire::serialize_list_notebook_instances_response(&wire::ListNotebookInstancesOutput {
            notebook_instances: Some(entries),
            ..Default::default()
        })
    }

    async fn handle_start_notebook_instance(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let name = match require_str(body, "NotebookInstanceName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let mut state = state.write().await;
        match state.start_notebook_instance(name) {
            Ok(()) => empty_json_response(),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_stop_notebook_instance(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let name = match require_str(body, "NotebookInstanceName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let mut state = state.write().await;
        match state.stop_notebook_instance(name) {
            Ok(()) => empty_json_response(),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    // ============================================================
    // Models
    // ============================================================

    async fn handle_create_model(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let name = match require_str(body, "ModelName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let role_arn = get_str(body, "ExecutionRoleArn")
            .unwrap_or("arn:aws:iam::123456789012:role/SageMakerRole");
        let mut state = state.write().await;
        match state.create_model(account_id, region, name, role_arn) {
            Ok(model) => wire::serialize_create_model_response(&wire::CreateModelOutput {
                model_arn: Some(model.model_arn.clone()),
            }),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_describe_model(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let name = match require_str(body, "ModelName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let state = state.read().await;
        match state.describe_model(name) {
            Ok(m) => wire::serialize_describe_model_response(&wire::DescribeModelOutput {
                model_name: Some(m.model_name.clone()),
                model_arn: Some(m.model_arn.clone()),
                execution_role_arn: Some(m.execution_role_arn.clone()),
                creation_time: Some(m.creation_time.timestamp() as f64),
                ..Default::default()
            }),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_delete_model(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let name = match require_str(body, "ModelName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let mut state = state.write().await;
        match state.delete_model(name) {
            Ok(()) => empty_json_response(),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_list_models(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let models = state.list_models();
        let entries: Vec<wire::ModelSummary> = models
            .iter()
            .map(|m| wire::ModelSummary {
                model_name: Some(m.model_name.clone()),
                model_arn: Some(m.model_arn.clone()),
                creation_time: Some(m.creation_time.timestamp() as f64),
            })
            .collect();
        wire::serialize_list_models_response(&wire::ListModelsOutput {
            models: Some(entries),
            ..Default::default()
        })
    }

    // ============================================================
    // Endpoint Configs
    // ============================================================

    async fn handle_create_endpoint_config(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let name = match require_str(body, "EndpointConfigName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let mut state = state.write().await;
        match state.create_endpoint_config(account_id, region, name) {
            Ok(ec) => {
                wire::serialize_create_endpoint_config_response(&wire::CreateEndpointConfigOutput {
                    endpoint_config_arn: Some(ec.endpoint_config_arn.clone()),
                })
            }
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_describe_endpoint_config(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let name = match require_str(body, "EndpointConfigName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let state = state.read().await;
        match state.describe_endpoint_config(name) {
            Ok(ec) => wire::serialize_describe_endpoint_config_response(
                &wire::DescribeEndpointConfigOutput {
                    endpoint_config_name: Some(ec.endpoint_config_name.clone()),
                    endpoint_config_arn: Some(ec.endpoint_config_arn.clone()),
                    creation_time: Some(ec.creation_time.timestamp() as f64),
                    ..Default::default()
                },
            ),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_delete_endpoint_config(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let name = match require_str(body, "EndpointConfigName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let mut state = state.write().await;
        match state.delete_endpoint_config(name) {
            Ok(()) => empty_json_response(),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_list_endpoint_configs(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let configs = state.list_endpoint_configs();
        let entries: Vec<wire::EndpointConfigSummary> = configs
            .iter()
            .map(|ec| wire::EndpointConfigSummary {
                endpoint_config_name: Some(ec.endpoint_config_name.clone()),
                endpoint_config_arn: Some(ec.endpoint_config_arn.clone()),
                creation_time: Some(ec.creation_time.timestamp() as f64),
            })
            .collect();
        wire::serialize_list_endpoint_configs_response(&wire::ListEndpointConfigsOutput {
            endpoint_configs: Some(entries),
            ..Default::default()
        })
    }

    // ============================================================
    // Endpoints
    // ============================================================

    async fn handle_create_endpoint(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let name = match require_str(body, "EndpointName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let config_name = match require_str(body, "EndpointConfigName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let mut state = state.write().await;
        match state.create_endpoint(account_id, region, name, config_name) {
            Ok(ep) => wire::serialize_create_endpoint_response(&wire::CreateEndpointOutput {
                endpoint_arn: Some(ep.endpoint_arn.clone()),
            }),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_describe_endpoint(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let name = match require_str(body, "EndpointName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let state = state.read().await;
        match state.describe_endpoint(name) {
            Ok(ep) => wire::serialize_describe_endpoint_response(&wire::DescribeEndpointOutput {
                endpoint_name: Some(ep.endpoint_name.clone()),
                endpoint_arn: Some(ep.endpoint_arn.clone()),
                endpoint_config_name: Some(ep.endpoint_config_name.clone()),
                endpoint_status: Some(ep.endpoint_status.clone()),
                creation_time: Some(ep.creation_time.timestamp() as f64),
                last_modified_time: Some(ep.last_modified_time.timestamp() as f64),
                ..Default::default()
            }),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_delete_endpoint(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let name = match require_str(body, "EndpointName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let mut state = state.write().await;
        match state.delete_endpoint(name) {
            Ok(()) => empty_json_response(),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_list_endpoints(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let eps = state.list_endpoints();
        let entries: Vec<wire::EndpointSummary> = eps
            .iter()
            .map(|ep| wire::EndpointSummary {
                endpoint_name: Some(ep.endpoint_name.clone()),
                endpoint_arn: Some(ep.endpoint_arn.clone()),
                endpoint_status: Some(ep.endpoint_status.clone()),
                creation_time: Some(ep.creation_time.timestamp() as f64),
                last_modified_time: Some(ep.last_modified_time.timestamp() as f64),
            })
            .collect();
        wire::serialize_list_endpoints_response(&wire::ListEndpointsOutput {
            endpoints: Some(entries),
            ..Default::default()
        })
    }

    async fn handle_update_endpoint_weights_and_capacities(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let name = match require_str(body, "EndpointName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let state = state.read().await;
        match state.update_endpoint_weights_and_capacities(name) {
            Ok(ep) => wire::serialize_update_endpoint_weights_and_capacities_response(
                &wire::UpdateEndpointWeightsAndCapacitiesOutput {
                    endpoint_arn: Some(ep.endpoint_arn.clone()),
                },
            ),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    // ============================================================
    // Training Jobs
    // ============================================================

    async fn handle_create_training_job(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let name = match require_str(body, "TrainingJobName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let role_arn = match require_str(body, "RoleArn") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let mut state = state.write().await;
        match state.create_training_job(account_id, region, name, role_arn) {
            Ok(job) => {
                wire::serialize_create_training_job_response(&wire::CreateTrainingJobResponse {
                    training_job_arn: Some(job.training_job_arn.clone()),
                })
            }
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_describe_training_job(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let name = match require_str(body, "TrainingJobName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let state = state.read().await;
        match state.describe_training_job(name) {
            Ok(job) => {
                wire::serialize_describe_training_job_response(&wire::DescribeTrainingJobResponse {
                    training_job_name: Some(job.training_job_name.clone()),
                    training_job_arn: Some(job.training_job_arn.clone()),
                    training_job_status: Some(job.training_job_status.clone()),
                    secondary_status: Some(job.secondary_status.clone()),
                    role_arn: Some(job.role_arn.clone()),
                    creation_time: Some(job.creation_time.timestamp() as f64),
                    last_modified_time: Some(job.last_modified_time.timestamp() as f64),
                    ..Default::default()
                })
            }
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_list_training_jobs(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let jobs = state.list_training_jobs();
        let entries: Vec<wire::TrainingJobSummary> = jobs
            .iter()
            .map(|j| wire::TrainingJobSummary {
                training_job_name: Some(j.training_job_name.clone()),
                training_job_arn: Some(j.training_job_arn.clone()),
                training_job_status: Some(j.training_job_status.clone()),
                creation_time: Some(j.creation_time.timestamp() as f64),
                last_modified_time: Some(j.last_modified_time.timestamp() as f64),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_training_jobs_response(&wire::ListTrainingJobsResponse {
            training_job_summaries: Some(entries),
            ..Default::default()
        })
    }

    // ============================================================
    // Processing Jobs
    // ============================================================

    async fn handle_create_processing_job(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let name = match require_str(body, "ProcessingJobName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let role_arn = match require_str(body, "RoleArn") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let mut state = state.write().await;
        match state.create_processing_job(account_id, region, name, role_arn) {
            Ok(job) => {
                wire::serialize_create_processing_job_response(&wire::CreateProcessingJobResponse {
                    processing_job_arn: Some(job.processing_job_arn.clone()),
                })
            }
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_describe_processing_job(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let name = match require_str(body, "ProcessingJobName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let state = state.read().await;
        match state.describe_processing_job(name) {
            Ok(job) => wire::serialize_describe_processing_job_response(
                &wire::DescribeProcessingJobResponse {
                    processing_job_name: Some(job.processing_job_name.clone()),
                    processing_job_arn: Some(job.processing_job_arn.clone()),
                    processing_job_status: Some(job.processing_job_status.clone()),
                    role_arn: Some(job.role_arn.clone()),
                    creation_time: Some(job.creation_time.timestamp() as f64),
                    last_modified_time: Some(job.last_modified_time.timestamp() as f64),
                    ..Default::default()
                },
            ),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_list_processing_jobs(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let jobs = state.list_processing_jobs();
        let entries: Vec<wire::ProcessingJobSummary> = jobs
            .iter()
            .map(|j| wire::ProcessingJobSummary {
                processing_job_name: Some(j.processing_job_name.clone()),
                processing_job_arn: Some(j.processing_job_arn.clone()),
                processing_job_status: Some(j.processing_job_status.clone()),
                creation_time: Some(j.creation_time.timestamp() as f64),
                last_modified_time: Some(j.last_modified_time.timestamp() as f64),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_processing_jobs_response(&wire::ListProcessingJobsResponse {
            processing_job_summaries: Some(entries),
            ..Default::default()
        })
    }

    // ============================================================
    // Transform Jobs
    // ============================================================

    async fn handle_create_transform_job(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let name = match require_str(body, "TransformJobName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let model_name = match require_str(body, "ModelName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let mut state = state.write().await;
        match state.create_transform_job(account_id, region, name, model_name) {
            Ok(job) => {
                wire::serialize_create_transform_job_response(&wire::CreateTransformJobResponse {
                    transform_job_arn: Some(job.transform_job_arn.clone()),
                })
            }
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_describe_transform_job(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let name = match require_str(body, "TransformJobName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let state = state.read().await;
        match state.describe_transform_job(name) {
            Ok(job) => wire::serialize_describe_transform_job_response(
                &wire::DescribeTransformJobResponse {
                    transform_job_name: Some(job.transform_job_name.clone()),
                    transform_job_arn: Some(job.transform_job_arn.clone()),
                    transform_job_status: Some(job.transform_job_status.clone()),
                    model_name: Some(job.model_name.clone()),
                    creation_time: Some(job.creation_time.timestamp() as f64),
                    ..Default::default()
                },
            ),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_list_transform_jobs(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let jobs = state.list_transform_jobs();
        let entries: Vec<wire::TransformJobSummary> = jobs
            .iter()
            .map(|j| wire::TransformJobSummary {
                transform_job_name: Some(j.transform_job_name.clone()),
                transform_job_arn: Some(j.transform_job_arn.clone()),
                transform_job_status: Some(j.transform_job_status.clone()),
                creation_time: Some(j.creation_time.timestamp() as f64),
                last_modified_time: Some(j.last_modified_time.timestamp() as f64),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_transform_jobs_response(&wire::ListTransformJobsResponse {
            transform_job_summaries: Some(entries),
            ..Default::default()
        })
    }

    // ============================================================
    // HyperParameter Tuning Jobs
    // ============================================================

    async fn handle_create_hyper_parameter_tuning_job(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let name = match require_str(body, "HyperParameterTuningJobName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let mut state = state.write().await;
        match state.create_hyper_parameter_tuning_job(account_id, region, name) {
            Ok(job) => wire::serialize_create_hyper_parameter_tuning_job_response(
                &wire::CreateHyperParameterTuningJobResponse {
                    hyper_parameter_tuning_job_arn: Some(job.job_arn.clone()),
                },
            ),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_describe_hyper_parameter_tuning_job(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let name = match require_str(body, "HyperParameterTuningJobName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let state = state.read().await;
        match state.describe_hyper_parameter_tuning_job(name) {
            Ok(job) => wire::serialize_describe_hyper_parameter_tuning_job_response(
                &wire::DescribeHyperParameterTuningJobResponse {
                    hyper_parameter_tuning_job_name: Some(job.job_name.clone()),
                    hyper_parameter_tuning_job_arn: Some(job.job_arn.clone()),
                    hyper_parameter_tuning_job_status: Some(job.job_status.clone()),
                    creation_time: Some(job.creation_time.timestamp() as f64),
                    last_modified_time: Some(job.last_modified_time.timestamp() as f64),
                    ..Default::default()
                },
            ),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_delete_hyper_parameter_tuning_job(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let name = match require_str(body, "HyperParameterTuningJobName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let mut state = state.write().await;
        match state.delete_hyper_parameter_tuning_job(name) {
            Ok(()) => empty_json_response(),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_list_hyper_parameter_tuning_jobs(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let jobs = state.list_hyper_parameter_tuning_jobs();
        let entries: Vec<wire::HyperParameterTuningJobSummary> = jobs
            .iter()
            .map(|j| wire::HyperParameterTuningJobSummary {
                hyper_parameter_tuning_job_name: Some(j.job_name.clone()),
                hyper_parameter_tuning_job_arn: Some(j.job_arn.clone()),
                hyper_parameter_tuning_job_status: Some(j.job_status.clone()),
                creation_time: Some(j.creation_time.timestamp() as f64),
                last_modified_time: Some(j.last_modified_time.timestamp() as f64),
                strategy: Some("Bayesian".to_string()),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_hyper_parameter_tuning_jobs_response(
            &wire::ListHyperParameterTuningJobsResponse {
                hyper_parameter_tuning_job_summaries: Some(entries),
                ..Default::default()
            },
        )
    }

    // ============================================================
    // Compilation Jobs
    // ============================================================

    async fn handle_create_compilation_job(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let name = match require_str(body, "CompilationJobName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let mut state = state.write().await;
        match state.create_compilation_job(account_id, region, name) {
            Ok(job) => wire::serialize_create_compilation_job_response(
                &wire::CreateCompilationJobResponse {
                    compilation_job_arn: Some(job.compilation_job_arn.clone()),
                },
            ),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_describe_compilation_job(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let name = match require_str(body, "CompilationJobName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let state = state.read().await;
        match state.describe_compilation_job(name) {
            Ok(job) => wire::serialize_describe_compilation_job_response(
                &wire::DescribeCompilationJobResponse {
                    compilation_job_name: Some(job.compilation_job_name.clone()),
                    compilation_job_arn: Some(job.compilation_job_arn.clone()),
                    compilation_job_status: Some(job.compilation_job_status.clone()),
                    creation_time: Some(job.creation_time.timestamp() as f64),
                    last_modified_time: Some(job.last_modified_time.timestamp() as f64),
                    ..Default::default()
                },
            ),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_delete_compilation_job(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let name = match require_str(body, "CompilationJobName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let mut state = state.write().await;
        match state.delete_compilation_job(name) {
            Ok(()) => empty_json_response(),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_list_compilation_jobs(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let jobs = state.list_compilation_jobs();
        let entries: Vec<wire::CompilationJobSummary> = jobs
            .iter()
            .map(|j| wire::CompilationJobSummary {
                compilation_job_name: Some(j.compilation_job_name.clone()),
                compilation_job_arn: Some(j.compilation_job_arn.clone()),
                compilation_job_status: Some(j.compilation_job_status.clone()),
                creation_time: Some(j.creation_time.timestamp() as f64),
                last_modified_time: Some(j.last_modified_time.timestamp() as f64),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_compilation_jobs_response(&wire::ListCompilationJobsResponse {
            compilation_job_summaries: Some(entries),
            ..Default::default()
        })
    }

    // ============================================================
    // AutoML Jobs V2
    // ============================================================

    async fn handle_create_auto_ml_job_v2(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let name = match require_str(body, "AutoMLJobName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let mut state = state.write().await;
        match state.create_auto_ml_job_v2(account_id, region, name) {
            Ok(job) => {
                wire::serialize_create_auto_m_l_job_v2_response(&wire::CreateAutoMLJobV2Response {
                    auto_m_l_job_arn: Some(job.auto_ml_job_arn.clone()),
                })
            }
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_describe_auto_ml_job_v2(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let name = match require_str(body, "AutoMLJobName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let state = state.read().await;
        match state.describe_auto_ml_job_v2(name) {
            Ok(job) => wire::serialize_describe_auto_m_l_job_v2_response(
                &wire::DescribeAutoMLJobV2Response {
                    auto_m_l_job_name: Some(job.auto_ml_job_name.clone()),
                    auto_m_l_job_arn: Some(job.auto_ml_job_arn.clone()),
                    auto_m_l_job_status: Some(job.auto_ml_job_status.clone()),
                    auto_m_l_job_secondary_status: Some("Starting".to_string()),
                    creation_time: Some(job.creation_time.timestamp() as f64),
                    last_modified_time: Some(job.last_modified_time.timestamp() as f64),
                    ..Default::default()
                },
            ),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_stop_auto_ml_job(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let name = match require_str(body, "AutoMLJobName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let mut state = state.write().await;
        match state.stop_auto_ml_job(name) {
            Ok(()) => empty_json_response(),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_list_auto_ml_jobs(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let jobs = state.list_auto_ml_jobs();
        let entries: Vec<wire::AutoMLJobSummary> = jobs
            .iter()
            .map(|j| wire::AutoMLJobSummary {
                auto_m_l_job_name: Some(j.auto_ml_job_name.clone()),
                auto_m_l_job_arn: Some(j.auto_ml_job_arn.clone()),
                auto_m_l_job_status: Some(j.auto_ml_job_status.clone()),
                auto_m_l_job_secondary_status: Some("Starting".to_string()),
                creation_time: Some(j.creation_time.timestamp() as f64),
                last_modified_time: Some(j.last_modified_time.timestamp() as f64),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_auto_m_l_jobs_response(&wire::ListAutoMLJobsResponse {
            auto_m_l_job_summaries: Some(entries),
            ..Default::default()
        })
    }

    // ============================================================
    // Experiments, Trials, Trial Components
    // ============================================================

    async fn handle_create_experiment(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let name = match require_str(body, "ExperimentName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let display_name = get_str(body, "DisplayName");
        let description = get_str(body, "Description");
        let mut state = state.write().await;
        match state.create_experiment(account_id, region, name, display_name, description) {
            Ok(exp) => {
                wire::serialize_create_experiment_response(&wire::CreateExperimentResponse {
                    experiment_arn: Some(exp.experiment_arn.clone()),
                })
            }
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_describe_experiment(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let name = match require_str(body, "ExperimentName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let state = state.read().await;
        match state.describe_experiment(name) {
            Ok(exp) => {
                wire::serialize_describe_experiment_response(&wire::DescribeExperimentResponse {
                    experiment_name: Some(exp.experiment_name.clone()),
                    experiment_arn: Some(exp.experiment_arn.clone()),
                    display_name: exp.display_name.clone(),
                    description: exp.description.clone(),
                    creation_time: Some(exp.creation_time.timestamp() as f64),
                    last_modified_time: Some(exp.last_modified_time.timestamp() as f64),
                    ..Default::default()
                })
            }
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_delete_experiment(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let name = match require_str(body, "ExperimentName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let mut state = state.write().await;
        match state.delete_experiment(name) {
            Ok(arn) => {
                wire::serialize_delete_experiment_response(&wire::DeleteExperimentResponse {
                    experiment_arn: Some(arn),
                })
            }
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_list_experiments(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let exps = state.list_experiments();
        let entries: Vec<wire::ExperimentSummary> = exps
            .iter()
            .map(|e| wire::ExperimentSummary {
                experiment_name: Some(e.experiment_name.clone()),
                experiment_arn: Some(e.experiment_arn.clone()),
                display_name: e.display_name.clone(),
                creation_time: Some(e.creation_time.timestamp() as f64),
                last_modified_time: Some(e.last_modified_time.timestamp() as f64),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_experiments_response(&wire::ListExperimentsResponse {
            experiment_summaries: Some(entries),
            ..Default::default()
        })
    }

    async fn handle_create_trial(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let name = match require_str(body, "TrialName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let exp_name = match require_str(body, "ExperimentName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let display_name = get_str(body, "DisplayName");
        let mut state = state.write().await;
        match state.create_trial(account_id, region, name, exp_name, display_name) {
            Ok(trial) => wire::serialize_create_trial_response(&wire::CreateTrialResponse {
                trial_arn: Some(trial.trial_arn.clone()),
            }),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_describe_trial(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let name = match require_str(body, "TrialName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let state = state.read().await;
        match state.describe_trial(name) {
            Ok(trial) => wire::serialize_describe_trial_response(&wire::DescribeTrialResponse {
                trial_name: Some(trial.trial_name.clone()),
                trial_arn: Some(trial.trial_arn.clone()),
                experiment_name: Some(trial.experiment_name.clone()),
                display_name: trial.display_name.clone(),
                creation_time: Some(trial.creation_time.timestamp() as f64),
                last_modified_time: Some(trial.last_modified_time.timestamp() as f64),
                ..Default::default()
            }),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_delete_trial(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let name = match require_str(body, "TrialName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let mut state = state.write().await;
        match state.delete_trial(name) {
            Ok(arn) => wire::serialize_delete_trial_response(&wire::DeleteTrialResponse {
                trial_arn: Some(arn),
            }),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_list_trials(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let tc_name_filter = get_str(body, "TrialComponentName");
        let state = state.read().await;
        let trials = state.list_trials_filtered(tc_name_filter);
        let entries: Vec<wire::TrialSummary> = trials
            .iter()
            .map(|t| wire::TrialSummary {
                trial_name: Some(t.trial_name.clone()),
                trial_arn: Some(t.trial_arn.clone()),
                display_name: t.display_name.clone(),
                creation_time: Some(t.creation_time.timestamp() as f64),
                last_modified_time: Some(t.last_modified_time.timestamp() as f64),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_trials_response(&wire::ListTrialsResponse {
            trial_summaries: Some(entries),
            ..Default::default()
        })
    }

    async fn handle_create_trial_component(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let name = match require_str(body, "TrialComponentName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let display_name = get_str(body, "DisplayName");
        let mut state = state.write().await;
        match state.create_trial_component(account_id, region, name, display_name) {
            Ok(tc) => wire::serialize_create_trial_component_response(
                &wire::CreateTrialComponentResponse {
                    trial_component_arn: Some(tc.trial_component_arn.clone()),
                },
            ),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_describe_trial_component(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let name = match require_str(body, "TrialComponentName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let state = state.read().await;
        match state.describe_trial_component(name) {
            Ok(tc) => wire::serialize_describe_trial_component_response(
                &wire::DescribeTrialComponentResponse {
                    trial_component_name: Some(tc.trial_component_name.clone()),
                    trial_component_arn: Some(tc.trial_component_arn.clone()),
                    display_name: tc.display_name.clone(),
                    creation_time: Some(tc.creation_time.timestamp() as f64),
                    last_modified_time: Some(tc.last_modified_time.timestamp() as f64),
                    ..Default::default()
                },
            ),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_delete_trial_component(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let name = match require_str(body, "TrialComponentName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let mut state = state.write().await;
        match state.delete_trial_component(name) {
            Ok(arn) => wire::serialize_delete_trial_component_response(
                &wire::DeleteTrialComponentResponse {
                    trial_component_arn: Some(arn),
                },
            ),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_update_trial_component(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let name = match require_str(body, "TrialComponentName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let display_name = get_str(body, "DisplayName");
        let mut state = state.write().await;
        match state.update_trial_component(name, display_name) {
            Ok(tc) => wire::serialize_update_trial_component_response(
                &wire::UpdateTrialComponentResponse {
                    trial_component_arn: Some(tc.trial_component_arn.clone()),
                    ..Default::default()
                },
            ),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_list_trial_components(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let trial_name_filter = get_str(body, "TrialName");
        let state = state.read().await;
        let tcs = state.list_trial_components_filtered(trial_name_filter);
        let entries: Vec<wire::TrialComponentSummary> = tcs
            .iter()
            .map(|tc| wire::TrialComponentSummary {
                trial_component_name: Some(tc.trial_component_name.clone()),
                trial_component_arn: Some(tc.trial_component_arn.clone()),
                display_name: tc.display_name.clone(),
                creation_time: Some(tc.creation_time.timestamp() as f64),
                last_modified_time: Some(tc.last_modified_time.timestamp() as f64),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_trial_components_response(&wire::ListTrialComponentsResponse {
            trial_component_summaries: Some(entries),
            ..Default::default()
        })
    }

    async fn handle_associate_trial_component(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let trial_name = match require_str(body, "TrialName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let tc_name = match require_str(body, "TrialComponentName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let mut state = state.write().await;
        match state.associate_trial_component(trial_name, tc_name) {
            Ok((trial_arn, tc_arn)) => wire::serialize_associate_trial_component_response(
                &wire::AssociateTrialComponentResponse {
                    trial_arn: Some(trial_arn),
                    trial_component_arn: Some(tc_arn),
                },
            ),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_disassociate_trial_component(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let trial_name = match require_str(body, "TrialName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let tc_name = match require_str(body, "TrialComponentName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let mut state = state.write().await;
        // DisassociateTrialComponent is idempotent — it succeeds even when trial or component doesn't exist.
        // Construct synthesized ARNs for non-existent resources (matching AWS/moto behavior).
        let trial_arn =
            format!("arn:aws:sagemaker:{region}:{account_id}:experiment-trial/{trial_name}");
        let tc_arn =
            format!("arn:aws:sagemaker:{region}:{account_id}:experiment-trial-component/{tc_name}");
        state.disassociate_trial_component(trial_name, tc_name);
        wire::serialize_disassociate_trial_component_response(
            &wire::DisassociateTrialComponentResponse {
                trial_arn: Some(trial_arn),
                trial_component_arn: Some(tc_arn),
            },
        )
    }

    // ============================================================
    // Pipelines
    // ============================================================

    async fn handle_create_pipeline(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let name = match require_str(body, "PipelineName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let role_arn = match require_str(body, "RoleArn") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let definition = get_str(body, "PipelineDefinition");
        let display_name = get_str(body, "PipelineDisplayName");
        let description = get_str(body, "PipelineDescription");
        let mut state = state.write().await;
        match state.create_pipeline(
            account_id,
            region,
            name,
            role_arn,
            definition,
            display_name,
            description,
        ) {
            Ok(p) => wire::serialize_create_pipeline_response(&wire::CreatePipelineResponse {
                pipeline_arn: Some(p.pipeline_arn.clone()),
            }),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_describe_pipeline(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let name = match require_str(body, "PipelineName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let state = state.read().await;
        match state.describe_pipeline(name) {
            Ok(p) => wire::serialize_describe_pipeline_response(&wire::DescribePipelineResponse {
                pipeline_name: Some(p.pipeline_name.clone()),
                pipeline_arn: Some(p.pipeline_arn.clone()),
                pipeline_display_name: p.pipeline_display_name.clone(),
                pipeline_description: p.pipeline_description.clone(),
                pipeline_definition: p.pipeline_definition.clone(),
                role_arn: Some(p.role_arn.clone()),
                creation_time: Some(p.creation_time.timestamp() as f64),
                last_modified_time: Some(p.last_modified_time.timestamp() as f64),
                pipeline_status: Some("Active".to_string()),
                ..Default::default()
            }),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_describe_pipeline_definition_for_execution(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let arn = match require_str(body, "PipelineExecutionArn") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let state = state.read().await;
        match state.describe_pipeline_definition_for_execution(arn) {
            Ok(def) => wire::serialize_describe_pipeline_definition_for_execution_response(
                &wire::DescribePipelineDefinitionForExecutionResponse {
                    pipeline_definition: def,
                    creation_time: None,
                },
            ),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_describe_pipeline_execution(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let arn = match require_str(body, "PipelineExecutionArn") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let state = state.read().await;
        match state.describe_pipeline_execution(arn) {
            Ok((pipeline, exec)) => wire::serialize_describe_pipeline_execution_response(
                &wire::DescribePipelineExecutionResponse {
                    pipeline_arn: Some(pipeline.pipeline_arn.clone()),
                    pipeline_execution_arn: Some(exec.pipeline_execution_arn.clone()),
                    pipeline_execution_status: Some(exec.pipeline_execution_status.clone()),
                    pipeline_execution_description: exec.pipeline_execution_description.clone(),
                    creation_time: Some(exec.creation_time.timestamp() as f64),
                    last_modified_time: Some(exec.last_modified_time.timestamp() as f64),
                    ..Default::default()
                },
            ),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_update_pipeline(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let name = match require_str(body, "PipelineName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let definition = get_str(body, "PipelineDefinition");
        let description = get_str(body, "PipelineDescription");
        let role_arn = get_str(body, "RoleArn");
        let mut state = state.write().await;
        match state.update_pipeline(name, definition, description, role_arn) {
            Ok(p) => wire::serialize_update_pipeline_response(&wire::UpdatePipelineResponse {
                pipeline_arn: Some(p.pipeline_arn.clone()),
                ..Default::default()
            }),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_delete_pipeline(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let name = match require_str(body, "PipelineName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let mut state = state.write().await;
        match state.delete_pipeline(name) {
            Ok(arn) => wire::serialize_delete_pipeline_response(&wire::DeletePipelineResponse {
                pipeline_arn: Some(arn),
            }),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_list_pipelines(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let pipelines = state.list_pipelines();
        let entries: Vec<wire::PipelineSummary> = pipelines
            .iter()
            .map(|p| wire::PipelineSummary {
                pipeline_name: Some(p.pipeline_name.clone()),
                pipeline_arn: Some(p.pipeline_arn.clone()),
                pipeline_display_name: p.pipeline_display_name.clone(),
                pipeline_description: p.pipeline_description.clone(),
                role_arn: Some(p.role_arn.clone()),
                creation_time: Some(p.creation_time.timestamp() as f64),
                last_modified_time: Some(p.last_modified_time.timestamp() as f64),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_pipelines_response(&wire::ListPipelinesResponse {
            pipeline_summaries: Some(entries),
            ..Default::default()
        })
    }

    async fn handle_start_pipeline_execution(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let name = match require_str(body, "PipelineName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let params = body
            .get("PipelineParameters")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|p| {
                        let name = p.get("Name")?.as_str()?.to_string();
                        let value = p.get("Value")?.as_str()?.to_string();
                        Some(PipelineParameter { name, value })
                    })
                    .collect()
            })
            .unwrap_or_default();
        let mut state = state.write().await;
        match state.start_pipeline_execution(name, account_id, region, params) {
            Ok(arn) => wire::serialize_start_pipeline_execution_response(
                &wire::StartPipelineExecutionResponse {
                    pipeline_execution_arn: Some(arn),
                },
            ),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_list_pipeline_executions(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let name = match require_str(body, "PipelineName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let state = state.read().await;
        match state.list_pipeline_executions(name) {
            Ok(execs) => {
                let entries: Vec<wire::PipelineExecutionSummary> = execs
                    .iter()
                    .map(|e| wire::PipelineExecutionSummary {
                        pipeline_execution_arn: Some(e.pipeline_execution_arn.clone()),
                        pipeline_execution_status: Some(e.pipeline_execution_status.clone()),
                        pipeline_execution_description: e.pipeline_execution_description.clone(),
                        start_time: Some(e.creation_time.timestamp() as f64),
                        ..Default::default()
                    })
                    .collect();
                wire::serialize_list_pipeline_executions_response(
                    &wire::ListPipelineExecutionsResponse {
                        pipeline_execution_summaries: Some(entries),
                        ..Default::default()
                    },
                )
            }
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_list_pipeline_parameters_for_execution(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let arn = match require_str(body, "PipelineExecutionArn") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let state = state.read().await;
        match state.list_pipeline_parameters_for_execution(arn) {
            Ok(params) => {
                let entries: Vec<wire::Parameter> = params
                    .iter()
                    .map(|p| wire::Parameter {
                        name: p.name.clone(),
                        value: p.value.clone(),
                    })
                    .collect();
                wire::serialize_list_pipeline_parameters_for_execution_response(
                    &wire::ListPipelineParametersForExecutionResponse {
                        pipeline_parameters: Some(entries),
                        ..Default::default()
                    },
                )
            }
            Err(e) => sagemaker_error_response(&e),
        }
    }

    // ============================================================
    // Feature Groups
    // ============================================================

    async fn handle_create_feature_group(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let name = match require_str(body, "FeatureGroupName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let mut state = state.write().await;
        match state.create_feature_group(account_id, region, name) {
            Ok(fg) => {
                wire::serialize_create_feature_group_response(&wire::CreateFeatureGroupResponse {
                    feature_group_arn: Some(fg.feature_group_arn.clone()),
                })
            }
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_describe_feature_group(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let name = match require_str(body, "FeatureGroupName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let state = state.read().await;
        match state.describe_feature_group(name) {
            Ok(fg) => wire::serialize_describe_feature_group_response(
                &wire::DescribeFeatureGroupResponse {
                    feature_group_name: Some(fg.feature_group_name.clone()),
                    feature_group_arn: Some(fg.feature_group_arn.clone()),
                    feature_group_status: Some(fg.feature_group_status.clone()),
                    creation_time: Some(fg.creation_time.timestamp() as f64),
                    last_modified_time: Some(fg.last_modified_time.timestamp() as f64),
                    ..Default::default()
                },
            ),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    // ============================================================
    // Domains
    // ============================================================

    async fn handle_create_domain(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let name = match require_str(body, "DomainName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let mut state = state.write().await;
        match state.create_domain(account_id, region, name) {
            Ok(d) => wire::serialize_create_domain_response(&wire::CreateDomainResponse {
                domain_arn: Some(d.domain_arn.clone()),
                ..Default::default()
            }),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_describe_domain(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let domain_id = match require_str(body, "DomainId") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let state = state.read().await;
        match state.describe_domain(domain_id) {
            Ok(d) => wire::serialize_describe_domain_response(&wire::DescribeDomainResponse {
                domain_id: Some(d.domain_id.clone()),
                domain_name: Some(d.domain_name.clone()),
                domain_arn: Some(d.domain_arn.clone()),
                status: Some(d.status.clone()),
                creation_time: Some(d.creation_time.timestamp() as f64),
                last_modified_time: Some(d.last_modified_time.timestamp() as f64),
                ..Default::default()
            }),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_delete_domain(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let domain_id = match require_str(body, "DomainId") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let mut state = state.write().await;
        match state.delete_domain(domain_id) {
            Ok(()) => empty_json_response(),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_list_domains(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let domains = state.list_domains();
        let entries: Vec<wire::DomainDetails> = domains
            .iter()
            .map(|d| wire::DomainDetails {
                domain_id: Some(d.domain_id.clone()),
                domain_name: Some(d.domain_name.clone()),
                domain_arn: Some(d.domain_arn.clone()),
                status: Some(d.status.clone()),
                creation_time: Some(d.creation_time.timestamp() as f64),
                last_modified_time: Some(d.last_modified_time.timestamp() as f64),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_domains_response(&wire::ListDomainsResponse {
            domains: Some(entries),
            ..Default::default()
        })
    }

    // ============================================================
    // Clusters
    // ============================================================

    async fn handle_create_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let name = match require_str(body, "ClusterName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let mut state = state.write().await;
        match state.create_cluster(account_id, region, name) {
            Ok(c) => wire::serialize_create_cluster_response(&wire::CreateClusterResponse {
                cluster_arn: Some(c.cluster_arn.clone()),
            }),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_describe_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let name = match require_str(body, "ClusterName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let state = state.read().await;
        match state.describe_cluster(name) {
            Ok(c) => wire::serialize_describe_cluster_response(&wire::DescribeClusterResponse {
                cluster_name: Some(c.cluster_name.clone()),
                cluster_arn: Some(c.cluster_arn.clone()),
                cluster_status: Some(c.cluster_status.clone()),
                creation_time: Some(c.creation_time.timestamp() as f64),
                ..Default::default()
            }),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_describe_cluster_node(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let cluster_name = match require_str(body, "ClusterName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let node_id = match require_str(body, "NodeId") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let state = state.read().await;
        match state.describe_cluster_node(cluster_name, node_id) {
            Ok(node) => {
                wire::serialize_describe_cluster_node_response(&wire::DescribeClusterNodeResponse {
                    node_details: Some(wire::ClusterNodeDetails {
                        instance_group_name: Some(node.instance_group_name.clone()),
                        instance_id: Some(node.instance_id.clone()),
                        instance_type: Some(node.instance_type.clone()),
                        launch_time: Some(node.launch_time.timestamp() as f64),
                        instance_status: Some(wire::ClusterInstanceStatusDetails {
                            status: Some(node.instance_status.clone()),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                })
            }
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_delete_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let name = match require_str(body, "ClusterName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let mut state = state.write().await;
        match state.delete_cluster(name) {
            Ok(()) => empty_json_response(),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_list_clusters(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let clusters = state.list_clusters();
        let entries: Vec<wire::ClusterSummary> = clusters
            .iter()
            .map(|c| wire::ClusterSummary {
                cluster_name: Some(c.cluster_name.clone()),
                cluster_arn: Some(c.cluster_arn.clone()),
                cluster_status: Some(c.cluster_status.clone()),
                creation_time: Some(c.creation_time.timestamp() as f64),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_clusters_response(&wire::ListClustersResponse {
            cluster_summaries: Some(entries),
            ..Default::default()
        })
    }

    async fn handle_list_cluster_nodes(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let cluster_name = match require_str(body, "ClusterName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let state = state.read().await;
        match state.list_cluster_nodes(cluster_name) {
            Ok(nodes) => {
                let entries: Vec<wire::ClusterNodeSummary> = nodes
                    .iter()
                    .map(|n| wire::ClusterNodeSummary {
                        instance_group_name: Some(n.instance_group_name.clone()),
                        instance_id: Some(n.instance_id.clone()),
                        instance_type: Some(n.instance_type.clone()),
                        launch_time: Some(n.launch_time.timestamp() as f64),
                        instance_status: Some(wire::ClusterInstanceStatusDetails {
                            status: Some(n.instance_status.clone()),
                            ..Default::default()
                        }),
                        ..Default::default()
                    })
                    .collect();
                wire::serialize_list_cluster_nodes_response(&wire::ListClusterNodesResponse {
                    cluster_node_summaries: Some(entries),
                    ..Default::default()
                })
            }
            Err(e) => sagemaker_error_response(&e),
        }
    }

    // ============================================================
    // Job Definitions (unified handler for 4 types)
    // ============================================================

    async fn handle_create_job_definition(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
        account_id: &str,
        region: &str,
        jd_type: JobDefinitionType,
    ) -> MockResponse {
        let name = match require_str(body, "JobDefinitionName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let role_arn = match require_str(body, "RoleArn") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let mut state = state.write().await;
        match state.create_job_definition(jd_type, account_id, region, name, role_arn) {
            Ok(arn) => match jd_type {
                JobDefinitionType::DataQuality => {
                    wire::serialize_create_data_quality_job_definition_response(
                        &wire::CreateDataQualityJobDefinitionResponse {
                            job_definition_arn: Some(arn),
                        },
                    )
                }
                JobDefinitionType::ModelQuality => {
                    wire::serialize_create_model_quality_job_definition_response(
                        &wire::CreateModelQualityJobDefinitionResponse {
                            job_definition_arn: Some(arn),
                        },
                    )
                }
                JobDefinitionType::ModelBias => {
                    wire::serialize_create_model_bias_job_definition_response(
                        &wire::CreateModelBiasJobDefinitionResponse {
                            job_definition_arn: Some(arn),
                        },
                    )
                }
                JobDefinitionType::ModelExplainability => {
                    wire::serialize_create_model_explainability_job_definition_response(
                        &wire::CreateModelExplainabilityJobDefinitionResponse {
                            job_definition_arn: Some(arn),
                        },
                    )
                }
            },
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_describe_job_definition(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
        jd_type: JobDefinitionType,
    ) -> MockResponse {
        let name = match require_str(body, "JobDefinitionName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let state = state.read().await;
        match state.describe_job_definition(jd_type, name) {
            Ok(jd) => match jd_type {
                JobDefinitionType::DataQuality => {
                    wire::serialize_describe_data_quality_job_definition_response(
                        &wire::DescribeDataQualityJobDefinitionResponse {
                            job_definition_name: Some(jd.job_definition_name.clone()),
                            job_definition_arn: Some(jd.job_definition_arn.clone()),
                            role_arn: Some(jd.role_arn.clone()),
                            creation_time: Some(jd.creation_time.timestamp() as f64),
                            ..Default::default()
                        },
                    )
                }
                JobDefinitionType::ModelQuality => {
                    wire::serialize_describe_model_quality_job_definition_response(
                        &wire::DescribeModelQualityJobDefinitionResponse {
                            job_definition_name: Some(jd.job_definition_name.clone()),
                            job_definition_arn: Some(jd.job_definition_arn.clone()),
                            role_arn: Some(jd.role_arn.clone()),
                            creation_time: Some(jd.creation_time.timestamp() as f64),
                            ..Default::default()
                        },
                    )
                }
                JobDefinitionType::ModelBias => {
                    wire::serialize_describe_model_bias_job_definition_response(
                        &wire::DescribeModelBiasJobDefinitionResponse {
                            job_definition_name: Some(jd.job_definition_name.clone()),
                            job_definition_arn: Some(jd.job_definition_arn.clone()),
                            role_arn: Some(jd.role_arn.clone()),
                            creation_time: Some(jd.creation_time.timestamp() as f64),
                            ..Default::default()
                        },
                    )
                }
                JobDefinitionType::ModelExplainability => {
                    wire::serialize_describe_model_explainability_job_definition_response(
                        &wire::DescribeModelExplainabilityJobDefinitionResponse {
                            job_definition_name: Some(jd.job_definition_name.clone()),
                            job_definition_arn: Some(jd.job_definition_arn.clone()),
                            role_arn: Some(jd.role_arn.clone()),
                            creation_time: Some(jd.creation_time.timestamp() as f64),
                            ..Default::default()
                        },
                    )
                }
            },
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_delete_job_definition(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
        jd_type: JobDefinitionType,
    ) -> MockResponse {
        let name = match require_str(body, "JobDefinitionName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let mut state = state.write().await;
        match state.delete_job_definition(jd_type, name) {
            Ok(()) => empty_json_response(),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_list_job_definitions(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        jd_type: JobDefinitionType,
    ) -> MockResponse {
        let state = state.read().await;
        let defs = state.list_job_definitions(jd_type);
        let entries: Vec<wire::MonitoringJobDefinitionSummary> = defs
            .iter()
            .map(|jd| wire::MonitoringJobDefinitionSummary {
                monitoring_job_definition_name: Some(jd.job_definition_name.clone()),
                monitoring_job_definition_arn: Some(jd.job_definition_arn.clone()),
                creation_time: Some(jd.creation_time.timestamp() as f64),
                ..Default::default()
            })
            .collect();

        match jd_type {
            JobDefinitionType::DataQuality => {
                wire::serialize_list_data_quality_job_definitions_response(
                    &wire::ListDataQualityJobDefinitionsResponse {
                        job_definition_summaries: Some(entries),
                        ..Default::default()
                    },
                )
            }
            JobDefinitionType::ModelQuality => {
                wire::serialize_list_model_quality_job_definitions_response(
                    &wire::ListModelQualityJobDefinitionsResponse {
                        job_definition_summaries: Some(entries),
                        ..Default::default()
                    },
                )
            }
            JobDefinitionType::ModelBias => {
                wire::serialize_list_model_bias_job_definitions_response(
                    &wire::ListModelBiasJobDefinitionsResponse {
                        job_definition_summaries: Some(entries),
                        ..Default::default()
                    },
                )
            }
            JobDefinitionType::ModelExplainability => {
                wire::serialize_list_model_explainability_job_definitions_response(
                    &wire::ListModelExplainabilityJobDefinitionsResponse {
                        job_definition_summaries: Some(entries),
                        ..Default::default()
                    },
                )
            }
        }
    }

    // ============================================================
    // Model Cards
    // ============================================================

    async fn handle_create_model_card(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let name = match require_str(body, "ModelCardName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let content = match require_str(body, "Content") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let status = match require_str(body, "ModelCardStatus") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let mut state = state.write().await;
        match state.create_model_card(account_id, region, name, content, status) {
            Ok(mc) => wire::serialize_create_model_card_response(&wire::CreateModelCardResponse {
                model_card_arn: Some(mc.model_card_arn.clone()),
            }),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_describe_model_card(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let name = match require_str(body, "ModelCardName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let state = state.read().await;
        match state.describe_model_card(name) {
            Ok(mc) => {
                wire::serialize_describe_model_card_response(&wire::DescribeModelCardResponse {
                    model_card_name: Some(mc.model_card_name.clone()),
                    model_card_arn: Some(mc.model_card_arn.clone()),
                    model_card_status: Some(mc.model_card_status.clone()),
                    content: Some(mc.content.clone()),
                    model_card_version: Some(mc.model_card_version),
                    creation_time: Some(mc.creation_time.timestamp() as f64),
                    last_modified_time: Some(mc.last_modified_time.timestamp() as f64),
                    ..Default::default()
                })
            }
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_update_model_card(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let name = match require_str(body, "ModelCardName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let content = get_str(body, "Content");
        let status = get_str(body, "ModelCardStatus");
        let mut state = state.write().await;
        match state.update_model_card(name, content, status) {
            Ok(mc) => wire::serialize_update_model_card_response(&wire::UpdateModelCardResponse {
                model_card_arn: Some(mc.model_card_arn.clone()),
            }),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_delete_model_card(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let name = match require_str(body, "ModelCardName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let mut state = state.write().await;
        match state.delete_model_card(name) {
            Ok(()) => empty_json_response(),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_list_model_cards(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let cards = state.list_model_cards();
        let entries: Vec<wire::ModelCardSummary> = cards
            .iter()
            .map(|mc| wire::ModelCardSummary {
                model_card_name: Some(mc.model_card_name.clone()),
                model_card_arn: Some(mc.model_card_arn.clone()),
                model_card_status: Some(mc.model_card_status.clone()),
                creation_time: Some(mc.creation_time.timestamp() as f64),
                last_modified_time: Some(mc.last_modified_time.timestamp() as f64),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_model_cards_response(&wire::ListModelCardsResponse {
            model_card_summaries: Some(entries),
            ..Default::default()
        })
    }

    async fn handle_list_model_card_versions(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let name = match require_str(body, "ModelCardName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let state = state.read().await;
        match state.describe_model_card(name) {
            Ok(mc) => {
                // Synthesise one summary entry per version (1 through current) since we
                // do not store full per-version history.
                let versions: Vec<wire::ModelCardVersionSummary> = (1..=mc.model_card_version)
                    .map(|v| wire::ModelCardVersionSummary {
                        model_card_name: Some(mc.model_card_name.clone()),
                        model_card_arn: Some(mc.model_card_arn.clone()),
                        model_card_status: Some(mc.model_card_status.clone()),
                        model_card_version: Some(v),
                        creation_time: Some(mc.creation_time.timestamp() as f64),
                        last_modified_time: Some(mc.last_modified_time.timestamp() as f64),
                        ..Default::default()
                    })
                    .collect();
                wire::serialize_list_model_card_versions_response(
                    &wire::ListModelCardVersionsResponse {
                        model_card_version_summary_list: Some(versions),
                        ..Default::default()
                    },
                )
            }
            Err(e) => sagemaker_error_response(&e),
        }
    }

    // ============================================================
    // Model Packages
    // ============================================================

    async fn handle_create_model_package(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let group_name = get_str(body, "ModelPackageGroupName");
        let package_name = get_str(body, "ModelPackageName");
        let desc = get_str(body, "ModelPackageDescription");
        let approval = get_str(body, "ModelApprovalStatus");
        let mut state = state.write().await;
        match (package_name, group_name) {
            (Some(_), Some(_)) => json_error_response(
                400,
                "ValidationException",
                "Both ModelPackageName and ModelPackageGroupName are provided in the input.",
            ),
            (None, Some(group)) => {
                // Versioned (group-based) model package
                match state
                    .create_versioned_model_package(account_id, region, group, desc, approval)
                {
                    Ok(mp) => wire::serialize_create_model_package_response(
                        &wire::CreateModelPackageOutput {
                            model_package_arn: Some(mp.model_package_arn.clone()),
                        },
                    ),
                    Err(e) => sagemaker_error_response(&e),
                }
            }
            (Some(name), None) => {
                // Named (standalone) model package
                match state.create_model_package(account_id, region, name, desc, approval) {
                    Ok(mp) => wire::serialize_create_model_package_response(
                        &wire::CreateModelPackageOutput {
                            model_package_arn: Some(mp.model_package_arn.clone()),
                        },
                    ),
                    Err(e) => sagemaker_error_response(&e),
                }
            }
            (None, None) => json_error_response(400, "ValidationException", "Missing ARN."),
        }
    }

    async fn handle_describe_model_package(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let raw_name = match require_str(body, "ModelPackageName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        // ModelPackageName can be a plain name or a full ARN (for versioned packages).
        let name = if let Some(idx) = raw_name.find(":model-package/") {
            &raw_name[idx + ":model-package/".len()..]
        } else {
            raw_name
        };
        let state = state.read().await;
        match state.describe_model_package(name) {
            Ok(mp) => {
                wire::serialize_describe_model_package_response(&wire::DescribeModelPackageOutput {
                    model_package_name: Some(mp.model_package_name.clone()),
                    model_package_arn: Some(mp.model_package_arn.clone()),
                    model_package_status: Some(mp.model_package_status.clone()),
                    model_package_description: mp.model_package_description.clone(),
                    model_approval_status: mp.model_approval_status.clone(),
                    creation_time: Some(mp.creation_time.timestamp() as f64),
                    last_modified_time: Some(mp.last_modified_time.timestamp() as f64),
                    ..Default::default()
                })
            }
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_update_model_package(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        // ModelPackageArn is the identifier for UpdateModelPackage.
        // ARN format: arn:aws:sagemaker:region:account:model-package/{name}
        //          or arn:aws:sagemaker:region:account:model-package/{group}/{version}
        let arn = match require_str(body, "ModelPackageArn") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let approval = get_str(body, "ModelApprovalStatus");
        // Extract everything after ":model-package/" as the lookup key.
        let name = if let Some(idx) = arn.find(":model-package/") {
            &arn[idx + ":model-package/".len()..]
        } else {
            arn.rsplit('/').next().unwrap_or(arn)
        };
        let mut state = state.write().await;
        match state.update_model_package(name, approval) {
            Ok(mp) => {
                wire::serialize_update_model_package_response(&wire::UpdateModelPackageOutput {
                    model_package_arn: Some(mp.model_package_arn.clone()),
                })
            }
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_list_model_packages(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let packages = state.list_model_packages();
        let entries: Vec<wire::ModelPackageSummary> = packages
            .iter()
            .map(|mp| wire::ModelPackageSummary {
                model_package_name: Some(mp.model_package_name.clone()),
                model_package_arn: Some(mp.model_package_arn.clone()),
                model_package_status: Some(mp.model_package_status.clone()),
                model_package_description: mp.model_package_description.clone(),
                model_approval_status: mp.model_approval_status.clone(),
                creation_time: Some(mp.creation_time.timestamp() as f64),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_model_packages_response(&wire::ListModelPackagesOutput {
            model_package_summary_list: Some(entries),
            ..Default::default()
        })
    }

    // ============================================================
    // Model Package Groups
    // ============================================================

    async fn handle_create_model_package_group(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let name = match require_str(body, "ModelPackageGroupName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let desc = get_str(body, "ModelPackageGroupDescription");
        let mut state = state.write().await;
        match state.create_model_package_group(account_id, region, name, desc) {
            Ok(mpg) => wire::serialize_create_model_package_group_response(
                &wire::CreateModelPackageGroupOutput {
                    model_package_group_arn: Some(mpg.model_package_group_arn.clone()),
                },
            ),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_describe_model_package_group(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let name = match require_str(body, "ModelPackageGroupName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let state = state.read().await;
        match state.describe_model_package_group(name) {
            Ok(mpg) => wire::serialize_describe_model_package_group_response(
                &wire::DescribeModelPackageGroupOutput {
                    model_package_group_name: Some(mpg.model_package_group_name.clone()),
                    model_package_group_arn: Some(mpg.model_package_group_arn.clone()),
                    model_package_group_status: Some(mpg.model_package_group_status.clone()),
                    model_package_group_description: mpg.model_package_group_description.clone(),
                    creation_time: Some(mpg.creation_time.timestamp() as f64),
                    ..Default::default()
                },
            ),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_list_model_package_groups(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let groups = state.list_model_package_groups();
        let entries: Vec<wire::ModelPackageGroupSummary> = groups
            .iter()
            .map(|mpg| wire::ModelPackageGroupSummary {
                model_package_group_name: Some(mpg.model_package_group_name.clone()),
                model_package_group_arn: Some(mpg.model_package_group_arn.clone()),
                model_package_group_status: Some(mpg.model_package_group_status.clone()),
                model_package_group_description: mpg.model_package_group_description.clone(),
                creation_time: Some(mpg.creation_time.timestamp() as f64),
            })
            .collect();
        wire::serialize_list_model_package_groups_response(&wire::ListModelPackageGroupsOutput {
            model_package_group_summary_list: Some(entries),
            ..Default::default()
        })
    }

    // ============================================================
    // Notebook Instance Lifecycle Configs
    // ============================================================

    async fn handle_create_notebook_instance_lifecycle_config(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let name = match require_str(body, "NotebookInstanceLifecycleConfigName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let mut state = state.write().await;
        match state.create_notebook_instance_lifecycle_config(account_id, region, name) {
            Ok(c) => wire::serialize_create_notebook_instance_lifecycle_config_response(
                &wire::CreateNotebookInstanceLifecycleConfigOutput {
                    notebook_instance_lifecycle_config_arn: Some(c.arn.clone()),
                },
            ),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_describe_notebook_instance_lifecycle_config(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let name = match require_str(body, "NotebookInstanceLifecycleConfigName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let state = state.read().await;
        match state.describe_notebook_instance_lifecycle_config(name) {
            Ok(c) => wire::serialize_describe_notebook_instance_lifecycle_config_response(
                &wire::DescribeNotebookInstanceLifecycleConfigOutput {
                    notebook_instance_lifecycle_config_name: Some(c.name.clone()),
                    notebook_instance_lifecycle_config_arn: Some(c.arn.clone()),
                    creation_time: Some(c.creation_time.timestamp() as f64),
                    last_modified_time: Some(c.last_modified_time.timestamp() as f64),
                    ..Default::default()
                },
            ),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_delete_notebook_instance_lifecycle_config(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let name = match require_str(body, "NotebookInstanceLifecycleConfigName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let mut state = state.write().await;
        match state.delete_notebook_instance_lifecycle_config(name) {
            Ok(()) => empty_json_response(),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    // ============================================================
    // Tags
    // ============================================================

    async fn handle_add_tags(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let resource_arn = match require_str(body, "ResourceArn") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let new_tags = get_tags_from_body(body);
        let mut state = state.write().await;
        let tags = state.add_tags(resource_arn, new_tags);
        wire::serialize_add_tags_response(&wire::AddTagsOutput {
            tags: Some(tags_to_wire(&tags)),
        })
    }

    async fn handle_delete_tags(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let resource_arn = match require_str(body, "ResourceArn") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let tag_keys = get_tag_keys_from_body(body);
        let mut state = state.write().await;
        state.delete_tags(resource_arn, &tag_keys);
        wire::serialize_delete_tags_response(&wire::DeleteTagsOutput {})
    }

    async fn handle_list_tags(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let resource_arn = match require_str(body, "ResourceArn") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let state = state.read().await;
        let tags = state.list_tags(resource_arn);
        wire::serialize_list_tags_response(&wire::ListTagsOutput {
            tags: Some(tags_to_wire(&tags)),
            ..Default::default()
        })
    }

    // ============================================================
    // Search
    // ============================================================

    // STUB[no-engine]: SageMaker Search requires indexing across resource types and
    //   filter/sort evaluation; always returns empty results.
    async fn handle_search(
        &self,
        _state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        _body: &Value,
    ) -> MockResponse {
        wire::serialize_search_response(&wire::SearchResponse {
            results: Some(Vec::new()),
            ..Default::default()
        })
    }

    // ============================================================
    // Update Domain
    // ============================================================

    async fn handle_update_domain(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let domain_id = match require_str(body, "DomainId") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let mut state = state.write().await;
        match state.update_domain(domain_id) {
            Ok(d) => wire::serialize_update_domain_response(&wire::UpdateDomainResponse {
                domain_arn: Some(d.domain_arn.clone()),
            }),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    // ============================================================
    // Update Endpoint
    // ============================================================

    async fn handle_update_endpoint(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let name = match require_str(body, "EndpointName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let endpoint_config_name = match require_str(body, "EndpointConfigName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let mut state = state.write().await;
        match state.update_endpoint(name, endpoint_config_name) {
            Ok(ep) => wire::serialize_update_endpoint_response(&wire::UpdateEndpointOutput {
                endpoint_arn: Some(ep.endpoint_arn.clone()),
            }),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    // ============================================================
    // Update Experiment / Trial
    // ============================================================

    async fn handle_update_experiment(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let name = match require_str(body, "ExperimentName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let display_name = get_str(body, "DisplayName");
        let description = get_str(body, "Description");
        let mut state = state.write().await;
        match state.update_experiment(name, display_name, description) {
            Ok(exp) => {
                wire::serialize_update_experiment_response(&wire::UpdateExperimentResponse {
                    experiment_arn: Some(exp.experiment_arn.clone()),
                })
            }
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_update_trial(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let name = match require_str(body, "TrialName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let display_name = get_str(body, "DisplayName");
        let mut state = state.write().await;
        match state.update_trial(name, display_name) {
            Ok(trial) => wire::serialize_update_trial_response(&wire::UpdateTrialResponse {
                trial_arn: Some(trial.trial_arn.clone()),
            }),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    // ============================================================
    // User Profiles
    // ============================================================

    async fn handle_create_user_profile(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let domain_id = match require_str(body, "DomainId") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let user_profile_name = match require_str(body, "UserProfileName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let mut state = state.write().await;
        match state.create_user_profile(account_id, region, domain_id, user_profile_name) {
            Ok(up) => {
                wire::serialize_create_user_profile_response(&wire::CreateUserProfileResponse {
                    user_profile_arn: Some(up.user_profile_arn.clone()),
                })
            }
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_describe_user_profile(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let domain_id = match require_str(body, "DomainId") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let user_profile_name = match require_str(body, "UserProfileName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let state = state.read().await;
        match state.describe_user_profile(domain_id, user_profile_name) {
            Ok(up) => {
                wire::serialize_describe_user_profile_response(&wire::DescribeUserProfileResponse {
                    domain_id: Some(up.domain_id.clone()),
                    user_profile_name: Some(up.user_profile_name.clone()),
                    user_profile_arn: Some(up.user_profile_arn.clone()),
                    status: Some(up.status.clone()),
                    creation_time: Some(up.creation_time.timestamp() as f64),
                    last_modified_time: Some(up.last_modified_time.timestamp() as f64),
                    ..Default::default()
                })
            }
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_update_user_profile(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
        _account_id: &str,
        _region: &str,
    ) -> MockResponse {
        let domain_id = match require_str(body, "DomainId") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let user_profile_name = match require_str(body, "UserProfileName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let mut state = state.write().await;
        match state.update_user_profile(domain_id, user_profile_name) {
            Ok(up) => {
                wire::serialize_update_user_profile_response(&wire::UpdateUserProfileResponse {
                    user_profile_arn: Some(up.user_profile_arn.clone()),
                })
            }
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_delete_user_profile(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let domain_id = match require_str(body, "DomainId") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let user_profile_name = match require_str(body, "UserProfileName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let mut state = state.write().await;
        match state.delete_user_profile(domain_id, user_profile_name) {
            Ok(()) => wire::serialize_delete_user_profile_response(),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_list_user_profiles(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let profiles = state.list_user_profiles();
        let entries: Vec<wire::UserProfileDetails> = profiles
            .iter()
            .map(|up| wire::UserProfileDetails {
                domain_id: Some(up.domain_id.clone()),
                user_profile_name: Some(up.user_profile_name.clone()),
                status: Some(up.status.clone()),
                creation_time: Some(up.creation_time.timestamp() as f64),
                last_modified_time: Some(up.last_modified_time.timestamp() as f64),
            })
            .collect();
        wire::serialize_list_user_profiles_response(&wire::ListUserProfilesResponse {
            user_profiles: Some(entries),
            ..Default::default()
        })
    }

    // ============================================================
    // Spaces
    // ============================================================

    async fn handle_create_space(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let domain_id = match require_str(body, "DomainId") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let space_name = match require_str(body, "SpaceName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let mut state = state.write().await;
        match state.create_space(account_id, region, domain_id, space_name) {
            Ok(s) => wire::serialize_create_space_response(&wire::CreateSpaceResponse {
                space_arn: Some(s.space_arn.clone()),
            }),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_describe_space(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let domain_id = match require_str(body, "DomainId") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let space_name = match require_str(body, "SpaceName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let state = state.read().await;
        match state.describe_space(domain_id, space_name) {
            Ok(s) => wire::serialize_describe_space_response(&wire::DescribeSpaceResponse {
                domain_id: Some(s.domain_id.clone()),
                space_name: Some(s.space_name.clone()),
                space_arn: Some(s.space_arn.clone()),
                status: Some(s.status.clone()),
                creation_time: Some(s.creation_time.timestamp() as f64),
                last_modified_time: Some(s.last_modified_time.timestamp() as f64),
                ..Default::default()
            }),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_update_space(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
        _account_id: &str,
        _region: &str,
    ) -> MockResponse {
        let domain_id = match require_str(body, "DomainId") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let space_name = match require_str(body, "SpaceName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let mut state = state.write().await;
        match state.update_space(domain_id, space_name) {
            Ok(s) => wire::serialize_update_space_response(&wire::UpdateSpaceResponse {
                space_arn: Some(s.space_arn.clone()),
            }),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_delete_space(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let domain_id = match require_str(body, "DomainId") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let space_name = match require_str(body, "SpaceName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let mut state = state.write().await;
        match state.delete_space(domain_id, space_name) {
            Ok(()) => wire::serialize_delete_space_response(),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_list_spaces(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let spaces = state.list_spaces();
        let entries: Vec<wire::SpaceDetails> = spaces
            .iter()
            .map(|s| wire::SpaceDetails {
                domain_id: Some(s.domain_id.clone()),
                space_name: Some(s.space_name.clone()),
                status: Some(s.status.clone()),
                creation_time: Some(s.creation_time.timestamp() as f64),
                last_modified_time: Some(s.last_modified_time.timestamp() as f64),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_spaces_response(&wire::ListSpacesResponse {
            spaces: Some(entries),
            ..Default::default()
        })
    }

    // ============================================================
    // Apps
    // ============================================================

    async fn handle_create_app(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let domain_id = match require_str(body, "DomainId") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let app_type = match require_str(body, "AppType") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let app_name = match require_str(body, "AppName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let user_profile_name = get_str(body, "UserProfileName");
        let space_name = get_str(body, "SpaceName");
        let mut state = state.write().await;
        match state.create_app(
            account_id,
            region,
            domain_id,
            user_profile_name,
            space_name,
            app_type,
            app_name,
        ) {
            Ok(app) => wire::serialize_create_app_response(&wire::CreateAppResponse {
                app_arn: Some(app.app_arn.clone()),
            }),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_describe_app(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let domain_id = match require_str(body, "DomainId") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let app_type = match require_str(body, "AppType") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let app_name = match require_str(body, "AppName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let user_profile_name = get_str(body, "UserProfileName");
        let space_name = get_str(body, "SpaceName");
        let state = state.read().await;
        match state.describe_app(domain_id, user_profile_name, space_name, app_type, app_name) {
            Ok(app) => wire::serialize_describe_app_response(&wire::DescribeAppResponse {
                domain_id: Some(app.domain_id.clone()),
                app_name: Some(app.app_name.clone()),
                app_type: Some(app.app_type.clone()),
                app_arn: Some(app.app_arn.clone()),
                status: Some(app.status.clone()),
                user_profile_name: app.user_profile_name.clone(),
                space_name: app.space_name.clone(),
                creation_time: Some(app.creation_time.timestamp() as f64),
                ..Default::default()
            }),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_delete_app(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let domain_id = match require_str(body, "DomainId") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let app_type = match require_str(body, "AppType") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let app_name = match require_str(body, "AppName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let user_profile_name = get_str(body, "UserProfileName");
        let space_name = get_str(body, "SpaceName");
        let mut state = state.write().await;
        match state.delete_app(domain_id, user_profile_name, space_name, app_type, app_name) {
            Ok(()) => wire::serialize_delete_app_response(),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_list_apps(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let apps = state.list_apps();
        let entries: Vec<wire::AppDetails> = apps
            .iter()
            .map(|app| wire::AppDetails {
                domain_id: Some(app.domain_id.clone()),
                app_name: Some(app.app_name.clone()),
                app_type: Some(app.app_type.clone()),
                status: Some(app.status.clone()),
                user_profile_name: app.user_profile_name.clone(),
                space_name: app.space_name.clone(),
                creation_time: Some(app.creation_time.timestamp() as f64),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_apps_response(&wire::ListAppsResponse {
            apps: Some(entries),
            ..Default::default()
        })
    }

    // ============================================================
    // Feature Groups (additional)
    // ============================================================

    async fn handle_delete_feature_group(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let name = match require_str(body, "FeatureGroupName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let mut state = state.write().await;
        match state.delete_feature_group(name) {
            Ok(()) => wire::serialize_delete_feature_group_response(),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_list_feature_groups(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let fgs = state.list_feature_groups();
        let entries: Vec<wire::FeatureGroupSummary> = fgs
            .iter()
            .map(|fg| wire::FeatureGroupSummary {
                feature_group_name: Some(fg.feature_group_name.clone()),
                feature_group_arn: Some(fg.feature_group_arn.clone()),
                feature_group_status: Some(fg.feature_group_status.clone()),
                creation_time: Some(fg.creation_time.timestamp() as f64),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_feature_groups_response(&wire::ListFeatureGroupsResponse {
            feature_group_summaries: Some(entries),
            ..Default::default()
        })
    }

    // ============================================================
    // Model Package (delete)
    // ============================================================

    async fn handle_delete_model_package(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let name = match require_str(body, "ModelPackageName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let mut state = state.write().await;
        match state.delete_model_package(name) {
            Ok(()) => wire::serialize_delete_model_package_response(),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    // ============================================================
    // Model Package Group (delete)
    // ============================================================

    async fn handle_delete_model_package_group(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let name = match require_str(body, "ModelPackageGroupName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let mut state = state.write().await;
        match state.delete_model_package_group(name) {
            Ok(()) => wire::serialize_delete_model_package_group_response(),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    // ============================================================
    // Training Job (stop/delete)
    // ============================================================

    async fn handle_stop_training_job(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let name = match require_str(body, "TrainingJobName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let mut state = state.write().await;
        match state.stop_training_job(name) {
            Ok(()) => wire::serialize_stop_training_job_response(),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_delete_training_job(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let name = match require_str(body, "TrainingJobName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let mut state = state.write().await;
        match state.delete_training_job(name) {
            Ok(()) => wire::serialize_delete_training_job_response(),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    // ============================================================
    // Processing Job (stop/delete)
    // ============================================================

    async fn handle_stop_processing_job(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let name = match require_str(body, "ProcessingJobName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let mut state = state.write().await;
        match state.stop_processing_job(name) {
            Ok(()) => wire::serialize_stop_processing_job_response(),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_delete_processing_job(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let name = match require_str(body, "ProcessingJobName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let mut state = state.write().await;
        match state.delete_processing_job(name) {
            Ok(()) => wire::serialize_delete_processing_job_response(),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    // ============================================================
    // Transform Job (stop)
    // ============================================================

    async fn handle_stop_transform_job(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let name = match require_str(body, "TransformJobName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let mut state = state.write().await;
        match state.stop_transform_job(name) {
            Ok(()) => wire::serialize_stop_transform_job_response(),
            Err(e) => sagemaker_error_response(&e),
        }
    }

    // ============================================================
    // Notebook Instance Lifecycle Configs (list/update)
    // ============================================================

    async fn handle_list_notebook_instance_lifecycle_configs(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let configs = state.list_notebook_instance_lifecycle_configs();
        let entries: Vec<wire::NotebookInstanceLifecycleConfigSummary> = configs
            .iter()
            .map(|c| wire::NotebookInstanceLifecycleConfigSummary {
                notebook_instance_lifecycle_config_name: Some(c.name.clone()),
                notebook_instance_lifecycle_config_arn: Some(c.arn.clone()),
                creation_time: Some(c.creation_time.timestamp() as f64),
                last_modified_time: Some(c.last_modified_time.timestamp() as f64),
            })
            .collect();
        wire::serialize_list_notebook_instance_lifecycle_configs_response(
            &wire::ListNotebookInstanceLifecycleConfigsOutput {
                notebook_instance_lifecycle_configs: Some(entries),
                ..Default::default()
            },
        )
    }

    async fn handle_update_notebook_instance(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerState>>,
        body: &Value,
    ) -> MockResponse {
        let name = match require_str(body, "NotebookInstanceName") {
            Ok(v) => v,
            Err(e) => return e,
        };
        let instance_type = get_str(body, "InstanceType");
        let role_arn = get_str(body, "RoleArn");
        let mut state = state.write().await;
        match state.update_notebook_instance(name, instance_type, role_arn) {
            Ok(()) => wire::serialize_update_notebook_instance_response(
                &wire::UpdateNotebookInstanceOutput {},
            ),
            Err(e) => sagemaker_error_response(&e),
        }
    }
}

// ============================================================
// Conversion helpers
// ============================================================

fn notebook_instance_to_describe_output(
    instance: &NotebookInstance,
) -> wire::DescribeNotebookInstanceOutput {
    wire::DescribeNotebookInstanceOutput {
        notebook_instance_arn: Some(instance.notebook_instance_arn.clone()),
        notebook_instance_name: Some(instance.notebook_instance_name.clone()),
        notebook_instance_status: Some(instance.notebook_instance_status.clone()),
        instance_type: Some(instance.instance_type.clone()),
        role_arn: Some(instance.role_arn.clone()),
        creation_time: Some(instance.creation_time.timestamp() as f64),
        last_modified_time: Some(instance.last_modified_time.timestamp() as f64),
        direct_internet_access: Some(instance.direct_internet_access.clone()),
        volume_size_in_g_b: Some(instance.volume_size_in_gb as i32),
        root_access: Some(instance.root_access.clone()),
        url: Some(instance.url.clone()),
        ..Default::default()
    }
}

fn notebook_instance_to_summary(instance: &NotebookInstance) -> wire::NotebookInstanceSummary {
    wire::NotebookInstanceSummary {
        notebook_instance_arn: Some(instance.notebook_instance_arn.clone()),
        notebook_instance_name: Some(instance.notebook_instance_name.clone()),
        notebook_instance_status: Some(instance.notebook_instance_status.clone()),
        instance_type: Some(instance.instance_type.clone()),
        creation_time: Some(instance.creation_time.timestamp() as f64),
        last_modified_time: Some(instance.last_modified_time.timestamp() as f64),
        url: Some(instance.url.clone()),
        ..Default::default()
    }
}
