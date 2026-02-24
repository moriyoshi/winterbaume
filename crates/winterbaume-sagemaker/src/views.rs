//! Serde-compatible view types for SageMaker state snapshots.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::SageMakerService;
use crate::state::SageMakerState;
use crate::types::Endpoint;
use crate::types::TrainingJob;
use crate::types::{
    App, AutoMLJobV2, Cluster, ClusterNode, CompilationJob, Domain, EndpointConfig, Experiment,
    FeatureGroup, HyperParameterTuningJob, JobDefinition, LifecycleScript, Model, ModelCard,
    ModelPackage, ModelPackageGroup, NotebookInstance, NotebookInstanceLifecycleConfig, Pipeline,
    PipelineExecution, PipelineParameter, ProcessingJob, Space, TagPair, TransformJob, Trial,
    TrialComponent, UserProfile,
};

// ---------------------------------------------------------------------------
// Helper: timestamp serialisation
// ---------------------------------------------------------------------------

fn dt_to_str(dt: DateTime<Utc>) -> String {
    dt.to_rfc3339()
}

fn str_to_dt(s: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(s)
        .map(|d| d.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now())
}

// ---------------------------------------------------------------------------
// Helper: serialisable tag pair
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagPairView {
    pub key: String,
    pub value: String,
}

impl From<&TagPair> for TagPairView {
    fn from(t: &TagPair) -> Self {
        Self {
            key: t.key.clone(),
            value: t.value.clone(),
        }
    }
}

impl From<TagPairView> for TagPair {
    fn from(t: TagPairView) -> Self {
        Self {
            key: t.key,
            value: t.value,
        }
    }
}

fn tags_to_view(tags: &[TagPair]) -> Vec<TagPairView> {
    tags.iter().map(TagPairView::from).collect()
}

fn tags_from_view(tags: Vec<TagPairView>) -> Vec<TagPair> {
    tags.into_iter().map(TagPair::from).collect()
}

// ---------------------------------------------------------------------------
// Top-level view
// ---------------------------------------------------------------------------

/// Serialisable view of the entire SageMaker state for one (account, region).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SagemakerStateView {
    #[serde(default)]
    pub notebook_instances: HashMap<String, NotebookInstanceView>,
    #[serde(default)]
    pub models: HashMap<String, ModelView>,
    #[serde(default)]
    pub endpoint_configs: HashMap<String, EndpointConfigView>,
    #[serde(default)]
    pub endpoints: HashMap<String, EndpointView>,
    #[serde(default)]
    pub training_jobs: HashMap<String, TrainingJobView>,
    #[serde(default)]
    pub processing_jobs: HashMap<String, ProcessingJobView>,
    #[serde(default)]
    pub transform_jobs: HashMap<String, TransformJobView>,
    #[serde(default)]
    pub hyper_parameter_tuning_jobs: HashMap<String, HyperParameterTuningJobView>,
    #[serde(default)]
    pub compilation_jobs: HashMap<String, CompilationJobView>,
    #[serde(default)]
    pub auto_ml_jobs_v2: HashMap<String, AutoMLJobV2View>,
    #[serde(default)]
    pub domains: HashMap<String, DomainView>,
    #[serde(default)]
    pub pipelines: HashMap<String, PipelineView>,
    #[serde(default)]
    pub experiments: HashMap<String, ExperimentView>,
    #[serde(default)]
    pub trials: HashMap<String, TrialView>,
    #[serde(default)]
    pub trial_components: HashMap<String, TrialComponentView>,
    #[serde(default)]
    pub feature_groups: HashMap<String, FeatureGroupView>,
    #[serde(default)]
    pub clusters: HashMap<String, ClusterView>,
    #[serde(default)]
    pub cluster_nodes: HashMap<String, Vec<ClusterNodeView>>,
    #[serde(default)]
    pub data_quality_job_definitions: HashMap<String, JobDefinitionView>,
    #[serde(default)]
    pub model_quality_job_definitions: HashMap<String, JobDefinitionView>,
    #[serde(default)]
    pub model_bias_job_definitions: HashMap<String, JobDefinitionView>,
    #[serde(default)]
    pub model_explainability_job_definitions: HashMap<String, JobDefinitionView>,
    #[serde(default)]
    pub model_cards: HashMap<String, ModelCardView>,
    #[serde(default)]
    pub model_packages: HashMap<String, ModelPackageView>,
    #[serde(default)]
    pub model_package_groups: HashMap<String, ModelPackageGroupView>,
    #[serde(default)]
    pub notebook_instance_lifecycle_configs: HashMap<String, NotebookInstanceLifecycleConfigView>,
    #[serde(default)]
    pub user_profiles: HashMap<String, UserProfileView>,
    #[serde(default)]
    pub spaces: HashMap<String, SpaceView>,
    #[serde(default)]
    pub apps: HashMap<String, AppView>,
    /// Tags keyed by resource ARN.
    #[serde(default)]
    pub tags: HashMap<String, Vec<TagPairView>>,
}

// ---------------------------------------------------------------------------
// Per-resource views
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelView {
    pub model_name: String,
    pub model_arn: String,
    pub execution_role_arn: String,
    pub creation_time: String,
    #[serde(default)]
    pub tags: Vec<TagPairView>,
    /// Raw JSON for `container` blocks.
    #[serde(default)]
    pub container: Option<serde_json::Value>,
    /// Raw JSON for `primary_container` block.
    #[serde(default)]
    pub primary_container: Option<serde_json::Value>,
    /// Raw JSON for `inference_execution_config` block.
    #[serde(default)]
    pub inference_execution_config: Option<serde_json::Value>,
    /// Raw JSON for `vpc_config` block.
    #[serde(default)]
    pub vpc_config: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointConfigView {
    pub endpoint_config_name: String,
    pub endpoint_config_arn: String,
    pub creation_time: String,
    #[serde(default)]
    pub tags: Vec<TagPairView>,
    /// Raw JSON for `production_variants` blocks.
    #[serde(default)]
    pub production_variants: Option<serde_json::Value>,
    /// Raw JSON for `async_inference_config` block.
    #[serde(default)]
    pub async_inference_config: Option<serde_json::Value>,
    /// Raw JSON for `data_capture_config` block.
    #[serde(default)]
    pub data_capture_config: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointView {
    pub endpoint_name: String,
    pub endpoint_arn: String,
    pub endpoint_config_name: String,
    pub endpoint_status: String,
    pub creation_time: String,
    pub last_modified_time: String,
    #[serde(default)]
    pub tags: Vec<TagPairView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingJobView {
    pub training_job_name: String,
    pub training_job_arn: String,
    pub training_job_status: String,
    pub secondary_status: String,
    pub role_arn: String,
    pub creation_time: String,
    pub last_modified_time: String,
    #[serde(default)]
    pub tags: Vec<TagPairView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingJobView {
    pub processing_job_name: String,
    pub processing_job_arn: String,
    pub processing_job_status: String,
    pub role_arn: String,
    pub creation_time: String,
    pub last_modified_time: String,
    #[serde(default)]
    pub tags: Vec<TagPairView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformJobView {
    pub transform_job_name: String,
    pub transform_job_arn: String,
    pub transform_job_status: String,
    pub model_name: String,
    pub creation_time: String,
    pub last_modified_time: String,
    #[serde(default)]
    pub tags: Vec<TagPairView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilationJobView {
    pub compilation_job_name: String,
    pub compilation_job_arn: String,
    pub compilation_job_status: String,
    pub creation_time: String,
    pub last_modified_time: String,
    #[serde(default)]
    pub tags: Vec<TagPairView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainView {
    pub domain_id: String,
    pub domain_name: String,
    pub domain_arn: String,
    pub status: String,
    pub creation_time: String,
    pub last_modified_time: String,
    #[serde(default)]
    pub tags: Vec<TagPairView>,
    pub vpc_id: Option<String>,
    #[serde(default)]
    pub subnet_ids: Vec<String>,
    pub app_network_access_type: Option<String>,
    pub auth_mode: Option<String>,
    pub kms_key_id: Option<String>,
    pub home_efs_file_system_id: Option<String>,
    #[serde(default)]
    pub security_group_ids: Vec<String>,
    pub url: Option<String>,
    /// Raw JSON for `default_space_settings` block.
    #[serde(default)]
    pub default_space_settings: Option<serde_json::Value>,
    /// Raw JSON for `domain_settings` block.
    #[serde(default)]
    pub domain_settings: Option<serde_json::Value>,
    /// Raw JSON for `retention_policy` block.
    #[serde(default)]
    pub retention_policy: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineParameterView {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineExecutionView {
    pub pipeline_execution_arn: String,
    pub pipeline_execution_status: String,
    pub pipeline_execution_description: Option<String>,
    pub creation_time: String,
    pub last_modified_time: String,
    #[serde(default)]
    pub parameters: Vec<PipelineParameterView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineView {
    pub pipeline_name: String,
    pub pipeline_arn: String,
    pub pipeline_display_name: Option<String>,
    pub pipeline_description: Option<String>,
    pub pipeline_definition: Option<String>,
    pub role_arn: String,
    pub creation_time: String,
    pub last_modified_time: String,
    #[serde(default)]
    pub executions: Vec<PipelineExecutionView>,
    #[serde(default)]
    pub tags: Vec<TagPairView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentView {
    pub experiment_name: String,
    pub experiment_arn: String,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub creation_time: String,
    pub last_modified_time: String,
    #[serde(default)]
    pub tags: Vec<TagPairView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrialView {
    pub trial_name: String,
    pub trial_arn: String,
    pub experiment_name: String,
    pub display_name: Option<String>,
    pub creation_time: String,
    pub last_modified_time: String,
    #[serde(default)]
    pub trial_component_names: Vec<String>,
    #[serde(default)]
    pub tags: Vec<TagPairView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrialComponentView {
    pub trial_component_name: String,
    pub trial_component_arn: String,
    pub display_name: Option<String>,
    pub status: String,
    pub creation_time: String,
    pub last_modified_time: String,
    #[serde(default)]
    pub tags: Vec<TagPairView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureGroupView {
    pub feature_group_name: String,
    pub feature_group_arn: String,
    pub feature_group_status: String,
    pub creation_time: String,
    pub last_modified_time: String,
    #[serde(default)]
    pub tags: Vec<TagPairView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelPackageView {
    pub model_package_name: String,
    pub model_package_arn: String,
    pub model_package_status: String,
    pub model_package_description: Option<String>,
    pub model_approval_status: Option<String>,
    pub creation_time: String,
    pub last_modified_time: String,
    #[serde(default)]
    pub tags: Vec<TagPairView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelPackageGroupView {
    pub model_package_group_name: String,
    pub model_package_group_arn: String,
    pub model_package_group_status: String,
    pub model_package_group_description: Option<String>,
    pub creation_time: String,
    pub next_version: u32,
    #[serde(default)]
    pub tags: Vec<TagPairView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfileView {
    pub domain_id: String,
    pub user_profile_name: String,
    pub user_profile_arn: String,
    pub status: String,
    pub creation_time: String,
    pub last_modified_time: String,
    #[serde(default)]
    pub tags: Vec<TagPairView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpaceView {
    pub domain_id: String,
    pub space_name: String,
    pub space_arn: String,
    pub status: String,
    pub creation_time: String,
    pub last_modified_time: String,
    #[serde(default)]
    pub tags: Vec<TagPairView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppView {
    pub domain_id: String,
    pub user_profile_name: Option<String>,
    pub space_name: Option<String>,
    pub app_type: String,
    pub app_name: String,
    pub app_arn: String,
    pub status: String,
    pub creation_time: String,
    #[serde(default)]
    pub tags: Vec<TagPairView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotebookInstanceView {
    pub notebook_instance_name: String,
    pub notebook_instance_arn: String,
    pub notebook_instance_status: String,
    pub instance_type: String,
    pub role_arn: String,
    pub creation_time: String,
    pub last_modified_time: String,
    pub direct_internet_access: String,
    pub volume_size_in_gb: i64,
    pub root_access: String,
    pub url: String,
    /// Raw JSON for `instance_metadata_service_configuration` block.
    #[serde(default)]
    pub instance_metadata_service_configuration: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HyperParameterTuningJobView {
    pub job_name: String,
    pub job_arn: String,
    pub job_status: String,
    pub creation_time: String,
    pub last_modified_time: String,
    #[serde(default)]
    pub tags: Vec<TagPairView>,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoMLJobV2View {
    pub auto_ml_job_name: String,
    pub auto_ml_job_arn: String,
    pub auto_ml_job_status: String,
    pub creation_time: String,
    pub last_modified_time: String,
    #[serde(default)]
    pub tags: Vec<TagPairView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterView {
    pub cluster_name: String,
    pub cluster_arn: String,
    pub cluster_status: String,
    pub creation_time: String,
    #[serde(default)]
    pub tags: Vec<TagPairView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterNodeView {
    pub instance_group_name: String,
    pub instance_id: String,
    pub instance_status: String,
    pub instance_type: String,
    pub launch_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobDefinitionView {
    pub job_definition_name: String,
    pub job_definition_arn: String,
    pub role_arn: String,
    pub creation_time: String,
    #[serde(default)]
    pub tags: Vec<TagPairView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelCardView {
    pub model_card_name: String,
    pub model_card_arn: String,
    pub model_card_status: String,
    pub content: String,
    pub model_card_version: i32,
    pub creation_time: String,
    pub last_modified_time: String,
    #[serde(default)]
    pub tags: Vec<TagPairView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifecycleScriptView {
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotebookInstanceLifecycleConfigView {
    pub name: String,
    pub arn: String,
    #[serde(default)]
    pub on_create: Vec<LifecycleScriptView>,
    #[serde(default)]
    pub on_start: Vec<LifecycleScriptView>,
    pub creation_time: String,
    pub last_modified_time: String,
}

// ---------------------------------------------------------------------------
// From<&SageMakerState> for SagemakerStateView
// ---------------------------------------------------------------------------

impl From<&SageMakerState> for SagemakerStateView {
    fn from(s: &SageMakerState) -> Self {
        let models = s
            .models
            .iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    ModelView {
                        model_name: v.model_name.clone(),
                        model_arn: v.model_arn.clone(),
                        execution_role_arn: v.execution_role_arn.clone(),
                        creation_time: dt_to_str(v.creation_time),
                        tags: tags_to_view(&v.tags),
                        container: v.container.clone(),
                        primary_container: v.primary_container.clone(),
                        inference_execution_config: v.inference_execution_config.clone(),
                        vpc_config: v.vpc_config.clone(),
                    },
                )
            })
            .collect();

        let endpoint_configs = s
            .endpoint_configs
            .iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    EndpointConfigView {
                        endpoint_config_name: v.endpoint_config_name.clone(),
                        endpoint_config_arn: v.endpoint_config_arn.clone(),
                        creation_time: dt_to_str(v.creation_time),
                        tags: tags_to_view(&v.tags),
                        production_variants: v.production_variants.clone(),
                        async_inference_config: v.async_inference_config.clone(),
                        data_capture_config: v.data_capture_config.clone(),
                    },
                )
            })
            .collect();

        let endpoints = s
            .endpoints
            .iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    EndpointView {
                        endpoint_name: v.endpoint_name.clone(),
                        endpoint_arn: v.endpoint_arn.clone(),
                        endpoint_config_name: v.endpoint_config_name.clone(),
                        endpoint_status: v.endpoint_status.clone(),
                        creation_time: dt_to_str(v.creation_time),
                        last_modified_time: dt_to_str(v.last_modified_time),
                        tags: tags_to_view(&v.tags),
                    },
                )
            })
            .collect();

        let training_jobs = s
            .training_jobs
            .iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    TrainingJobView {
                        training_job_name: v.training_job_name.clone(),
                        training_job_arn: v.training_job_arn.clone(),
                        training_job_status: v.training_job_status.clone(),
                        secondary_status: v.secondary_status.clone(),
                        role_arn: v.role_arn.clone(),
                        creation_time: dt_to_str(v.creation_time),
                        last_modified_time: dt_to_str(v.last_modified_time),
                        tags: tags_to_view(&v.tags),
                    },
                )
            })
            .collect();

        let processing_jobs = s
            .processing_jobs
            .iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    ProcessingJobView {
                        processing_job_name: v.processing_job_name.clone(),
                        processing_job_arn: v.processing_job_arn.clone(),
                        processing_job_status: v.processing_job_status.clone(),
                        role_arn: v.role_arn.clone(),
                        creation_time: dt_to_str(v.creation_time),
                        last_modified_time: dt_to_str(v.last_modified_time),
                        tags: tags_to_view(&v.tags),
                    },
                )
            })
            .collect();

        let transform_jobs = s
            .transform_jobs
            .iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    TransformJobView {
                        transform_job_name: v.transform_job_name.clone(),
                        transform_job_arn: v.transform_job_arn.clone(),
                        transform_job_status: v.transform_job_status.clone(),
                        model_name: v.model_name.clone(),
                        creation_time: dt_to_str(v.creation_time),
                        last_modified_time: dt_to_str(v.last_modified_time),
                        tags: tags_to_view(&v.tags),
                    },
                )
            })
            .collect();

        let compilation_jobs = s
            .compilation_jobs
            .iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    CompilationJobView {
                        compilation_job_name: v.compilation_job_name.clone(),
                        compilation_job_arn: v.compilation_job_arn.clone(),
                        compilation_job_status: v.compilation_job_status.clone(),
                        creation_time: dt_to_str(v.creation_time),
                        last_modified_time: dt_to_str(v.last_modified_time),
                        tags: tags_to_view(&v.tags),
                    },
                )
            })
            .collect();

        let domains = s
            .domains
            .iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    DomainView {
                        domain_id: v.domain_id.clone(),
                        domain_name: v.domain_name.clone(),
                        domain_arn: v.domain_arn.clone(),
                        status: v.status.clone(),
                        creation_time: dt_to_str(v.creation_time),
                        last_modified_time: dt_to_str(v.last_modified_time),
                        tags: tags_to_view(&v.tags),
                        vpc_id: v.vpc_id.clone(),
                        subnet_ids: v.subnet_ids.clone(),
                        app_network_access_type: v.app_network_access_type.clone(),
                        auth_mode: v.auth_mode.clone(),
                        kms_key_id: v.kms_key_id.clone(),
                        home_efs_file_system_id: v.home_efs_file_system_id.clone(),
                        security_group_ids: v.security_group_ids.clone(),
                        url: v.url.clone(),
                        default_space_settings: v.default_space_settings.clone(),
                        domain_settings: v.domain_settings.clone(),
                        retention_policy: v.retention_policy.clone(),
                    },
                )
            })
            .collect();

        let pipelines = s
            .pipelines
            .iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    PipelineView {
                        pipeline_name: v.pipeline_name.clone(),
                        pipeline_arn: v.pipeline_arn.clone(),
                        pipeline_display_name: v.pipeline_display_name.clone(),
                        pipeline_description: v.pipeline_description.clone(),
                        pipeline_definition: v.pipeline_definition.clone(),
                        role_arn: v.role_arn.clone(),
                        creation_time: dt_to_str(v.creation_time),
                        last_modified_time: dt_to_str(v.last_modified_time),
                        executions: v
                            .executions
                            .iter()
                            .map(|e| PipelineExecutionView {
                                pipeline_execution_arn: e.pipeline_execution_arn.clone(),
                                pipeline_execution_status: e.pipeline_execution_status.clone(),
                                pipeline_execution_description: e
                                    .pipeline_execution_description
                                    .clone(),
                                creation_time: dt_to_str(e.creation_time),
                                last_modified_time: dt_to_str(e.last_modified_time),
                                parameters: e
                                    .parameters
                                    .iter()
                                    .map(|p| PipelineParameterView {
                                        name: p.name.clone(),
                                        value: p.value.clone(),
                                    })
                                    .collect(),
                            })
                            .collect(),
                        tags: tags_to_view(&v.tags),
                    },
                )
            })
            .collect();

        let experiments = s
            .experiments
            .iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    ExperimentView {
                        experiment_name: v.experiment_name.clone(),
                        experiment_arn: v.experiment_arn.clone(),
                        display_name: v.display_name.clone(),
                        description: v.description.clone(),
                        creation_time: dt_to_str(v.creation_time),
                        last_modified_time: dt_to_str(v.last_modified_time),
                        tags: tags_to_view(&v.tags),
                    },
                )
            })
            .collect();

        let trials = s
            .trials
            .iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    TrialView {
                        trial_name: v.trial_name.clone(),
                        trial_arn: v.trial_arn.clone(),
                        experiment_name: v.experiment_name.clone(),
                        display_name: v.display_name.clone(),
                        creation_time: dt_to_str(v.creation_time),
                        last_modified_time: dt_to_str(v.last_modified_time),
                        trial_component_names: v.trial_component_names.clone(),
                        tags: tags_to_view(&v.tags),
                    },
                )
            })
            .collect();

        let trial_components = s
            .trial_components
            .iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    TrialComponentView {
                        trial_component_name: v.trial_component_name.clone(),
                        trial_component_arn: v.trial_component_arn.clone(),
                        display_name: v.display_name.clone(),
                        status: v.status.clone(),
                        creation_time: dt_to_str(v.creation_time),
                        last_modified_time: dt_to_str(v.last_modified_time),
                        tags: tags_to_view(&v.tags),
                    },
                )
            })
            .collect();

        let feature_groups = s
            .feature_groups
            .iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    FeatureGroupView {
                        feature_group_name: v.feature_group_name.clone(),
                        feature_group_arn: v.feature_group_arn.clone(),
                        feature_group_status: v.feature_group_status.clone(),
                        creation_time: dt_to_str(v.creation_time),
                        last_modified_time: dt_to_str(v.last_modified_time),
                        tags: tags_to_view(&v.tags),
                    },
                )
            })
            .collect();

        let model_packages = s
            .model_packages
            .iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    ModelPackageView {
                        model_package_name: v.model_package_name.clone(),
                        model_package_arn: v.model_package_arn.clone(),
                        model_package_status: v.model_package_status.clone(),
                        model_package_description: v.model_package_description.clone(),
                        model_approval_status: v.model_approval_status.clone(),
                        creation_time: dt_to_str(v.creation_time),
                        last_modified_time: dt_to_str(v.last_modified_time),
                        tags: tags_to_view(&v.tags),
                    },
                )
            })
            .collect();

        let model_package_groups = s
            .model_package_groups
            .iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    ModelPackageGroupView {
                        model_package_group_name: v.model_package_group_name.clone(),
                        model_package_group_arn: v.model_package_group_arn.clone(),
                        model_package_group_status: v.model_package_group_status.clone(),
                        model_package_group_description: v.model_package_group_description.clone(),
                        creation_time: dt_to_str(v.creation_time),
                        next_version: v.next_version,
                        tags: tags_to_view(&v.tags),
                    },
                )
            })
            .collect();

        let tags = s
            .tags
            .iter()
            .map(|(arn, tag_vec)| (arn.clone(), tags_to_view(tag_vec)))
            .collect();

        let user_profiles = s
            .user_profiles
            .iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    UserProfileView {
                        domain_id: v.domain_id.clone(),
                        user_profile_name: v.user_profile_name.clone(),
                        user_profile_arn: v.user_profile_arn.clone(),
                        status: v.status.clone(),
                        creation_time: dt_to_str(v.creation_time),
                        last_modified_time: dt_to_str(v.last_modified_time),
                        tags: tags_to_view(&v.tags),
                    },
                )
            })
            .collect();

        let spaces = s
            .spaces
            .iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    SpaceView {
                        domain_id: v.domain_id.clone(),
                        space_name: v.space_name.clone(),
                        space_arn: v.space_arn.clone(),
                        status: v.status.clone(),
                        creation_time: dt_to_str(v.creation_time),
                        last_modified_time: dt_to_str(v.last_modified_time),
                        tags: tags_to_view(&v.tags),
                    },
                )
            })
            .collect();

        let apps = s
            .apps
            .iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    AppView {
                        domain_id: v.domain_id.clone(),
                        user_profile_name: v.user_profile_name.clone(),
                        space_name: v.space_name.clone(),
                        app_type: v.app_type.clone(),
                        app_name: v.app_name.clone(),
                        app_arn: v.app_arn.clone(),
                        status: v.status.clone(),
                        creation_time: dt_to_str(v.creation_time),
                        tags: tags_to_view(&v.tags),
                    },
                )
            })
            .collect();

        let notebook_instances = s
            .notebook_instances
            .iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    NotebookInstanceView {
                        notebook_instance_name: v.notebook_instance_name.clone(),
                        notebook_instance_arn: v.notebook_instance_arn.clone(),
                        notebook_instance_status: v.notebook_instance_status.clone(),
                        instance_type: v.instance_type.clone(),
                        role_arn: v.role_arn.clone(),
                        creation_time: dt_to_str(v.creation_time),
                        last_modified_time: dt_to_str(v.last_modified_time),
                        direct_internet_access: v.direct_internet_access.clone(),
                        volume_size_in_gb: v.volume_size_in_gb,
                        root_access: v.root_access.clone(),
                        url: v.url.clone(),
                        instance_metadata_service_configuration: v
                            .instance_metadata_service_configuration
                            .clone(),
                    },
                )
            })
            .collect();

        let hyper_parameter_tuning_jobs = s
            .hyper_parameter_tuning_jobs
            .iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    HyperParameterTuningJobView {
                        job_name: v.job_name.clone(),
                        job_arn: v.job_arn.clone(),
                        job_status: v.job_status.clone(),
                        creation_time: dt_to_str(v.creation_time),
                        last_modified_time: dt_to_str(v.last_modified_time),
                        tags: tags_to_view(&v.tags),
                    },
                )
            })
            .collect();

        let auto_ml_jobs_v2 = s
            .auto_ml_jobs_v2
            .iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    AutoMLJobV2View {
                        auto_ml_job_name: v.auto_ml_job_name.clone(),
                        auto_ml_job_arn: v.auto_ml_job_arn.clone(),
                        auto_ml_job_status: v.auto_ml_job_status.clone(),
                        creation_time: dt_to_str(v.creation_time),
                        last_modified_time: dt_to_str(v.last_modified_time),
                        tags: tags_to_view(&v.tags),
                    },
                )
            })
            .collect();

        let clusters = s
            .clusters
            .iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    ClusterView {
                        cluster_name: v.cluster_name.clone(),
                        cluster_arn: v.cluster_arn.clone(),
                        cluster_status: v.cluster_status.clone(),
                        creation_time: dt_to_str(v.creation_time),
                        tags: tags_to_view(&v.tags),
                    },
                )
            })
            .collect();

        let cluster_nodes = s
            .cluster_nodes
            .iter()
            .map(|(k, nodes)| {
                (
                    k.clone(),
                    nodes
                        .iter()
                        .map(|n| ClusterNodeView {
                            instance_group_name: n.instance_group_name.clone(),
                            instance_id: n.instance_id.clone(),
                            instance_status: n.instance_status.clone(),
                            instance_type: n.instance_type.clone(),
                            launch_time: dt_to_str(n.launch_time),
                        })
                        .collect(),
                )
            })
            .collect();

        let fn_job_def_to_view =
            |(k, v): (&String, &JobDefinition)| -> (String, JobDefinitionView) {
                (
                    k.clone(),
                    JobDefinitionView {
                        job_definition_name: v.job_definition_name.clone(),
                        job_definition_arn: v.job_definition_arn.clone(),
                        role_arn: v.role_arn.clone(),
                        creation_time: dt_to_str(v.creation_time),
                        tags: tags_to_view(&v.tags),
                    },
                )
            };

        let data_quality_job_definitions = s
            .data_quality_job_definitions
            .iter()
            .map(&fn_job_def_to_view)
            .collect();

        let model_quality_job_definitions = s
            .model_quality_job_definitions
            .iter()
            .map(&fn_job_def_to_view)
            .collect();

        let model_bias_job_definitions = s
            .model_bias_job_definitions
            .iter()
            .map(&fn_job_def_to_view)
            .collect();

        let model_explainability_job_definitions = s
            .model_explainability_job_definitions
            .iter()
            .map(&fn_job_def_to_view)
            .collect();

        let model_cards = s
            .model_cards
            .iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    ModelCardView {
                        model_card_name: v.model_card_name.clone(),
                        model_card_arn: v.model_card_arn.clone(),
                        model_card_status: v.model_card_status.clone(),
                        content: v.content.clone(),
                        model_card_version: v.model_card_version,
                        creation_time: dt_to_str(v.creation_time),
                        last_modified_time: dt_to_str(v.last_modified_time),
                        tags: tags_to_view(&v.tags),
                    },
                )
            })
            .collect();

        let notebook_instance_lifecycle_configs = s
            .notebook_instance_lifecycle_configs
            .iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    NotebookInstanceLifecycleConfigView {
                        name: v.name.clone(),
                        arn: v.arn.clone(),
                        on_create: v
                            .on_create
                            .iter()
                            .map(|sc| LifecycleScriptView {
                                content: sc.content.clone(),
                            })
                            .collect(),
                        on_start: v
                            .on_start
                            .iter()
                            .map(|sc| LifecycleScriptView {
                                content: sc.content.clone(),
                            })
                            .collect(),
                        creation_time: dt_to_str(v.creation_time),
                        last_modified_time: dt_to_str(v.last_modified_time),
                    },
                )
            })
            .collect();

        SagemakerStateView {
            notebook_instances,
            models,
            endpoint_configs,
            endpoints,
            training_jobs,
            processing_jobs,
            transform_jobs,
            hyper_parameter_tuning_jobs,
            compilation_jobs,
            auto_ml_jobs_v2,
            domains,
            pipelines,
            experiments,
            trials,
            trial_components,
            feature_groups,
            clusters,
            cluster_nodes,
            data_quality_job_definitions,
            model_quality_job_definitions,
            model_bias_job_definitions,
            model_explainability_job_definitions,
            model_cards,
            model_packages,
            model_package_groups,
            notebook_instance_lifecycle_configs,
            user_profiles,
            spaces,
            apps,
            tags,
        }
    }
}

// ---------------------------------------------------------------------------
// From<SagemakerStateView> for SageMakerState  (reverse)
// ---------------------------------------------------------------------------

impl From<SagemakerStateView> for SageMakerState {
    fn from(v: SagemakerStateView) -> Self {
        let mut s = SageMakerState::default();

        for (k, mv) in v.models {
            s.models.insert(
                k,
                Model {
                    model_name: mv.model_name,
                    model_arn: mv.model_arn,
                    execution_role_arn: mv.execution_role_arn,
                    creation_time: str_to_dt(&mv.creation_time),
                    tags: tags_from_view(mv.tags),
                    container: mv.container,
                    primary_container: mv.primary_container,
                    inference_execution_config: mv.inference_execution_config,
                    vpc_config: mv.vpc_config,
                },
            );
        }

        for (k, ev) in v.endpoint_configs {
            s.endpoint_configs.insert(
                k,
                EndpointConfig {
                    endpoint_config_name: ev.endpoint_config_name,
                    endpoint_config_arn: ev.endpoint_config_arn,
                    creation_time: str_to_dt(&ev.creation_time),
                    tags: tags_from_view(ev.tags),
                    production_variants: ev.production_variants,
                    async_inference_config: ev.async_inference_config,
                    data_capture_config: ev.data_capture_config,
                },
            );
        }

        for (k, ev) in v.endpoints {
            s.endpoints.insert(
                k,
                Endpoint {
                    endpoint_name: ev.endpoint_name,
                    endpoint_arn: ev.endpoint_arn,
                    endpoint_config_name: ev.endpoint_config_name,
                    endpoint_status: ev.endpoint_status,
                    creation_time: str_to_dt(&ev.creation_time),
                    last_modified_time: str_to_dt(&ev.last_modified_time),
                    tags: tags_from_view(ev.tags),
                },
            );
        }

        for (k, tv) in v.training_jobs {
            s.training_jobs.insert(
                k,
                TrainingJob {
                    training_job_name: tv.training_job_name,
                    training_job_arn: tv.training_job_arn,
                    training_job_status: tv.training_job_status,
                    secondary_status: tv.secondary_status,
                    role_arn: tv.role_arn,
                    creation_time: str_to_dt(&tv.creation_time),
                    last_modified_time: str_to_dt(&tv.last_modified_time),
                    tags: tags_from_view(tv.tags),
                },
            );
        }

        for (k, pv) in v.processing_jobs {
            s.processing_jobs.insert(
                k,
                ProcessingJob {
                    processing_job_name: pv.processing_job_name,
                    processing_job_arn: pv.processing_job_arn,
                    processing_job_status: pv.processing_job_status,
                    role_arn: pv.role_arn,
                    creation_time: str_to_dt(&pv.creation_time),
                    last_modified_time: str_to_dt(&pv.last_modified_time),
                    tags: tags_from_view(pv.tags),
                },
            );
        }

        for (k, tv) in v.transform_jobs {
            s.transform_jobs.insert(
                k,
                TransformJob {
                    transform_job_name: tv.transform_job_name,
                    transform_job_arn: tv.transform_job_arn,
                    transform_job_status: tv.transform_job_status,
                    model_name: tv.model_name,
                    creation_time: str_to_dt(&tv.creation_time),
                    last_modified_time: str_to_dt(&tv.last_modified_time),
                    tags: tags_from_view(tv.tags),
                },
            );
        }

        for (k, cv) in v.compilation_jobs {
            s.compilation_jobs.insert(
                k,
                CompilationJob {
                    compilation_job_name: cv.compilation_job_name,
                    compilation_job_arn: cv.compilation_job_arn,
                    compilation_job_status: cv.compilation_job_status,
                    creation_time: str_to_dt(&cv.creation_time),
                    last_modified_time: str_to_dt(&cv.last_modified_time),
                    tags: tags_from_view(cv.tags),
                },
            );
        }

        for (k, dv) in v.domains {
            s.domains.insert(
                k,
                Domain {
                    domain_id: dv.domain_id,
                    domain_name: dv.domain_name,
                    domain_arn: dv.domain_arn,
                    status: dv.status,
                    creation_time: str_to_dt(&dv.creation_time),
                    last_modified_time: str_to_dt(&dv.last_modified_time),
                    tags: tags_from_view(dv.tags),
                    vpc_id: dv.vpc_id,
                    subnet_ids: dv.subnet_ids,
                    app_network_access_type: dv.app_network_access_type,
                    auth_mode: dv.auth_mode,
                    kms_key_id: dv.kms_key_id,
                    home_efs_file_system_id: dv.home_efs_file_system_id,
                    security_group_ids: dv.security_group_ids,
                    url: dv.url,
                    default_space_settings: dv.default_space_settings,
                    domain_settings: dv.domain_settings,
                    retention_policy: dv.retention_policy,
                },
            );
        }

        for (k, pv) in v.pipelines {
            s.pipelines.insert(
                k,
                Pipeline {
                    pipeline_name: pv.pipeline_name,
                    pipeline_arn: pv.pipeline_arn,
                    pipeline_display_name: pv.pipeline_display_name,
                    pipeline_description: pv.pipeline_description,
                    pipeline_definition: pv.pipeline_definition,
                    role_arn: pv.role_arn,
                    creation_time: str_to_dt(&pv.creation_time),
                    last_modified_time: str_to_dt(&pv.last_modified_time),
                    executions: pv
                        .executions
                        .into_iter()
                        .map(|e| PipelineExecution {
                            pipeline_execution_arn: e.pipeline_execution_arn,
                            pipeline_execution_status: e.pipeline_execution_status,
                            pipeline_execution_description: e.pipeline_execution_description,
                            creation_time: str_to_dt(&e.creation_time),
                            last_modified_time: str_to_dt(&e.last_modified_time),
                            parameters: e
                                .parameters
                                .into_iter()
                                .map(|p| PipelineParameter {
                                    name: p.name,
                                    value: p.value,
                                })
                                .collect(),
                        })
                        .collect(),
                    tags: tags_from_view(pv.tags),
                },
            );
        }

        for (k, ev) in v.experiments {
            s.experiments.insert(
                k,
                Experiment {
                    experiment_name: ev.experiment_name,
                    experiment_arn: ev.experiment_arn,
                    display_name: ev.display_name,
                    description: ev.description,
                    creation_time: str_to_dt(&ev.creation_time),
                    last_modified_time: str_to_dt(&ev.last_modified_time),
                    tags: tags_from_view(ev.tags),
                },
            );
        }

        for (k, tv) in v.trials {
            s.trials.insert(
                k,
                Trial {
                    trial_name: tv.trial_name,
                    trial_arn: tv.trial_arn,
                    experiment_name: tv.experiment_name,
                    display_name: tv.display_name,
                    creation_time: str_to_dt(&tv.creation_time),
                    last_modified_time: str_to_dt(&tv.last_modified_time),
                    trial_component_names: tv.trial_component_names,
                    tags: tags_from_view(tv.tags),
                },
            );
        }

        for (k, tv) in v.trial_components {
            s.trial_components.insert(
                k,
                TrialComponent {
                    trial_component_name: tv.trial_component_name,
                    trial_component_arn: tv.trial_component_arn,
                    display_name: tv.display_name,
                    status: tv.status,
                    creation_time: str_to_dt(&tv.creation_time),
                    last_modified_time: str_to_dt(&tv.last_modified_time),
                    tags: tags_from_view(tv.tags),
                },
            );
        }

        for (k, fv) in v.feature_groups {
            s.feature_groups.insert(
                k,
                FeatureGroup {
                    feature_group_name: fv.feature_group_name,
                    feature_group_arn: fv.feature_group_arn,
                    feature_group_status: fv.feature_group_status,
                    creation_time: str_to_dt(&fv.creation_time),
                    last_modified_time: str_to_dt(&fv.last_modified_time),
                    tags: tags_from_view(fv.tags),
                },
            );
        }

        for (k, mv) in v.model_packages {
            s.model_packages.insert(
                k,
                ModelPackage {
                    model_package_name: mv.model_package_name,
                    model_package_arn: mv.model_package_arn,
                    model_package_status: mv.model_package_status,
                    model_package_description: mv.model_package_description,
                    model_approval_status: mv.model_approval_status,
                    creation_time: str_to_dt(&mv.creation_time),
                    last_modified_time: str_to_dt(&mv.last_modified_time),
                    tags: tags_from_view(mv.tags),
                },
            );
        }

        for (k, mv) in v.model_package_groups {
            s.model_package_groups.insert(
                k,
                ModelPackageGroup {
                    model_package_group_name: mv.model_package_group_name,
                    model_package_group_arn: mv.model_package_group_arn,
                    model_package_group_status: mv.model_package_group_status,
                    model_package_group_description: mv.model_package_group_description,
                    creation_time: str_to_dt(&mv.creation_time),
                    tags: tags_from_view(mv.tags),
                    next_version: mv.next_version,
                },
            );
        }

        for (arn, tag_vec) in v.tags {
            s.tags.insert(arn, tags_from_view(tag_vec));
        }

        for (k, niv) in v.notebook_instances {
            s.notebook_instances.insert(
                k,
                NotebookInstance {
                    notebook_instance_name: niv.notebook_instance_name,
                    notebook_instance_arn: niv.notebook_instance_arn,
                    notebook_instance_status: niv.notebook_instance_status,
                    instance_type: niv.instance_type,
                    role_arn: niv.role_arn,
                    creation_time: str_to_dt(&niv.creation_time),
                    last_modified_time: str_to_dt(&niv.last_modified_time),
                    direct_internet_access: niv.direct_internet_access,
                    volume_size_in_gb: niv.volume_size_in_gb,
                    root_access: niv.root_access,
                    url: niv.url,
                    instance_metadata_service_configuration: niv
                        .instance_metadata_service_configuration,
                },
            );
        }

        for (k, hv) in v.hyper_parameter_tuning_jobs {
            s.hyper_parameter_tuning_jobs.insert(
                k,
                HyperParameterTuningJob {
                    job_name: hv.job_name,
                    job_arn: hv.job_arn,
                    job_status: hv.job_status,
                    creation_time: str_to_dt(&hv.creation_time),
                    last_modified_time: str_to_dt(&hv.last_modified_time),
                    tags: tags_from_view(hv.tags),
                },
            );
        }

        for (k, av) in v.auto_ml_jobs_v2 {
            s.auto_ml_jobs_v2.insert(
                k,
                AutoMLJobV2 {
                    auto_ml_job_name: av.auto_ml_job_name,
                    auto_ml_job_arn: av.auto_ml_job_arn,
                    auto_ml_job_status: av.auto_ml_job_status,
                    creation_time: str_to_dt(&av.creation_time),
                    last_modified_time: str_to_dt(&av.last_modified_time),
                    tags: tags_from_view(av.tags),
                },
            );
        }

        for (k, cv) in v.clusters {
            s.clusters.insert(
                k,
                Cluster {
                    cluster_name: cv.cluster_name,
                    cluster_arn: cv.cluster_arn,
                    cluster_status: cv.cluster_status,
                    creation_time: str_to_dt(&cv.creation_time),
                    tags: tags_from_view(cv.tags),
                },
            );
        }

        for (k, nodes_view) in v.cluster_nodes {
            s.cluster_nodes.insert(
                k,
                nodes_view
                    .into_iter()
                    .map(|nv| ClusterNode {
                        instance_group_name: nv.instance_group_name,
                        instance_id: nv.instance_id,
                        instance_status: nv.instance_status,
                        instance_type: nv.instance_type,
                        launch_time: str_to_dt(&nv.launch_time),
                    })
                    .collect(),
            );
        }

        let fn_view_to_job_def = |jv: JobDefinitionView| -> JobDefinition {
            JobDefinition {
                job_definition_name: jv.job_definition_name,
                job_definition_arn: jv.job_definition_arn,
                role_arn: jv.role_arn,
                creation_time: str_to_dt(&jv.creation_time),
                tags: tags_from_view(jv.tags),
            }
        };

        for (k, jv) in v.data_quality_job_definitions {
            s.data_quality_job_definitions
                .insert(k, fn_view_to_job_def(jv));
        }
        for (k, jv) in v.model_quality_job_definitions {
            s.model_quality_job_definitions
                .insert(k, fn_view_to_job_def(jv));
        }
        for (k, jv) in v.model_bias_job_definitions {
            s.model_bias_job_definitions
                .insert(k, fn_view_to_job_def(jv));
        }
        for (k, jv) in v.model_explainability_job_definitions {
            s.model_explainability_job_definitions
                .insert(k, fn_view_to_job_def(jv));
        }

        for (k, mcv) in v.model_cards {
            s.model_cards.insert(
                k,
                ModelCard {
                    model_card_name: mcv.model_card_name,
                    model_card_arn: mcv.model_card_arn,
                    model_card_status: mcv.model_card_status,
                    content: mcv.content,
                    model_card_version: mcv.model_card_version,
                    creation_time: str_to_dt(&mcv.creation_time),
                    last_modified_time: str_to_dt(&mcv.last_modified_time),
                    tags: tags_from_view(mcv.tags),
                },
            );
        }

        for (k, lcv) in v.notebook_instance_lifecycle_configs {
            s.notebook_instance_lifecycle_configs.insert(
                k,
                NotebookInstanceLifecycleConfig {
                    name: lcv.name,
                    arn: lcv.arn,
                    on_create: lcv
                        .on_create
                        .into_iter()
                        .map(|sv| LifecycleScript {
                            content: sv.content,
                        })
                        .collect(),
                    on_start: lcv
                        .on_start
                        .into_iter()
                        .map(|sv| LifecycleScript {
                            content: sv.content,
                        })
                        .collect(),
                    creation_time: str_to_dt(&lcv.creation_time),
                    last_modified_time: str_to_dt(&lcv.last_modified_time),
                },
            );
        }

        for (k, upv) in v.user_profiles {
            s.user_profiles.insert(
                k,
                UserProfile {
                    domain_id: upv.domain_id,
                    user_profile_name: upv.user_profile_name,
                    user_profile_arn: upv.user_profile_arn,
                    status: upv.status,
                    creation_time: str_to_dt(&upv.creation_time),
                    last_modified_time: str_to_dt(&upv.last_modified_time),
                    tags: tags_from_view(upv.tags),
                },
            );
        }

        for (k, sv) in v.spaces {
            s.spaces.insert(
                k,
                Space {
                    domain_id: sv.domain_id,
                    space_name: sv.space_name,
                    space_arn: sv.space_arn,
                    status: sv.status,
                    creation_time: str_to_dt(&sv.creation_time),
                    last_modified_time: str_to_dt(&sv.last_modified_time),
                    tags: tags_from_view(sv.tags),
                },
            );
        }

        for (k, av) in v.apps {
            s.apps.insert(
                k,
                App {
                    domain_id: av.domain_id,
                    user_profile_name: av.user_profile_name,
                    space_name: av.space_name,
                    app_type: av.app_type,
                    app_name: av.app_name,
                    app_arn: av.app_arn,
                    status: av.status,
                    creation_time: str_to_dt(&av.creation_time),
                    tags: tags_from_view(av.tags),
                },
            );
        }

        s
    }
}

// ---------------------------------------------------------------------------
// StatefulService implementation
// ---------------------------------------------------------------------------

impl StatefulService for SageMakerService {
    type StateView = SagemakerStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        SagemakerStateView::from(&*guard)
    }

    async fn restore(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let new_state = SageMakerState::from(view);
        {
            let state = self.state.get(account_id, region);
            *state.write().await = new_state;
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    async fn merge(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let incoming = SageMakerState::from(view);
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            guard.models.extend(incoming.models);
            guard.endpoint_configs.extend(incoming.endpoint_configs);
            guard.endpoints.extend(incoming.endpoints);
            guard.training_jobs.extend(incoming.training_jobs);
            guard.processing_jobs.extend(incoming.processing_jobs);
            guard.transform_jobs.extend(incoming.transform_jobs);
            guard.compilation_jobs.extend(incoming.compilation_jobs);
            guard.domains.extend(incoming.domains);
            guard.pipelines.extend(incoming.pipelines);
            guard.experiments.extend(incoming.experiments);
            guard.trials.extend(incoming.trials);
            guard.trial_components.extend(incoming.trial_components);
            guard.feature_groups.extend(incoming.feature_groups);
            guard.model_packages.extend(incoming.model_packages);
            guard
                .model_package_groups
                .extend(incoming.model_package_groups);
            guard.user_profiles.extend(incoming.user_profiles);
            guard.spaces.extend(incoming.spaces);
            guard.apps.extend(incoming.apps);
            guard.tags.extend(incoming.tags);
            guard.notebook_instances.extend(incoming.notebook_instances);
            guard
                .hyper_parameter_tuning_jobs
                .extend(incoming.hyper_parameter_tuning_jobs);
            guard.auto_ml_jobs_v2.extend(incoming.auto_ml_jobs_v2);
            guard.clusters.extend(incoming.clusters);
            guard.cluster_nodes.extend(incoming.cluster_nodes);
            guard
                .data_quality_job_definitions
                .extend(incoming.data_quality_job_definitions);
            guard
                .model_quality_job_definitions
                .extend(incoming.model_quality_job_definitions);
            guard
                .model_bias_job_definitions
                .extend(incoming.model_bias_job_definitions);
            guard
                .model_explainability_job_definitions
                .extend(incoming.model_explainability_job_definitions);
            guard.model_cards.extend(incoming.model_cards);
            guard
                .notebook_instance_lifecycle_configs
                .extend(incoming.notebook_instance_lifecycle_configs);
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
