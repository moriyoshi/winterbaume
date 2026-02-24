use std::collections::HashMap;

use chrono::{DateTime, Utc};

/// Represents a SageMaker notebook instance.
#[derive(Debug, Clone)]
pub struct NotebookInstance {
    pub notebook_instance_name: String,
    pub notebook_instance_arn: String,
    pub notebook_instance_status: String,
    pub instance_type: String,
    pub role_arn: String,
    pub creation_time: DateTime<Utc>,
    pub last_modified_time: DateTime<Utc>,
    pub direct_internet_access: String,
    pub volume_size_in_gb: i64,
    pub root_access: String,
    pub url: String,
    pub instance_metadata_service_configuration: Option<serde_json::Value>,
}

/// Represents a SageMaker model.
#[derive(Debug, Clone)]
pub struct Model {
    pub model_name: String,
    pub model_arn: String,
    pub execution_role_arn: String,
    pub creation_time: DateTime<Utc>,
    pub tags: Vec<TagPair>,
    pub container: Option<serde_json::Value>,
    pub primary_container: Option<serde_json::Value>,
    pub inference_execution_config: Option<serde_json::Value>,
    pub vpc_config: Option<serde_json::Value>,
}

/// Represents a SageMaker endpoint configuration.
#[derive(Debug, Clone)]
pub struct EndpointConfig {
    pub endpoint_config_name: String,
    pub endpoint_config_arn: String,
    pub creation_time: DateTime<Utc>,
    pub tags: Vec<TagPair>,
    pub production_variants: Option<serde_json::Value>,
    pub async_inference_config: Option<serde_json::Value>,
    pub data_capture_config: Option<serde_json::Value>,
}

/// Represents a SageMaker endpoint.
#[derive(Debug, Clone)]
pub struct Endpoint {
    pub endpoint_name: String,
    pub endpoint_arn: String,
    pub endpoint_config_name: String,
    pub endpoint_status: String,
    pub creation_time: DateTime<Utc>,
    pub last_modified_time: DateTime<Utc>,
    pub tags: Vec<TagPair>,
}

/// Represents a SageMaker training job.
#[derive(Debug, Clone)]
pub struct TrainingJob {
    pub training_job_name: String,
    pub training_job_arn: String,
    pub training_job_status: String,
    pub secondary_status: String,
    pub role_arn: String,
    pub creation_time: DateTime<Utc>,
    pub last_modified_time: DateTime<Utc>,
    pub tags: Vec<TagPair>,
}

/// Represents a SageMaker processing job.
#[derive(Debug, Clone)]
pub struct ProcessingJob {
    pub processing_job_name: String,
    pub processing_job_arn: String,
    pub processing_job_status: String,
    pub role_arn: String,
    pub creation_time: DateTime<Utc>,
    pub last_modified_time: DateTime<Utc>,
    pub tags: Vec<TagPair>,
}

/// Represents a SageMaker transform job.
#[derive(Debug, Clone)]
pub struct TransformJob {
    pub transform_job_name: String,
    pub transform_job_arn: String,
    pub transform_job_status: String,
    pub model_name: String,
    pub creation_time: DateTime<Utc>,
    pub last_modified_time: DateTime<Utc>,
    pub tags: Vec<TagPair>,
}

/// Represents a SageMaker hyperparameter tuning job.
#[derive(Debug, Clone)]
pub struct HyperParameterTuningJob {
    pub job_name: String,
    pub job_arn: String,
    pub job_status: String,
    pub creation_time: DateTime<Utc>,
    pub last_modified_time: DateTime<Utc>,
    pub tags: Vec<TagPair>,
}

/// Represents a SageMaker compilation job.
#[derive(Debug, Clone)]
pub struct CompilationJob {
    pub compilation_job_name: String,
    pub compilation_job_arn: String,
    pub compilation_job_status: String,
    pub creation_time: DateTime<Utc>,
    pub last_modified_time: DateTime<Utc>,
    pub tags: Vec<TagPair>,
}

/// Represents a SageMaker AutoML Job V2.
#[derive(Debug, Clone)]
#[allow(clippy::upper_case_acronyms)]
pub struct AutoMLJobV2 {
    pub auto_ml_job_name: String,
    pub auto_ml_job_arn: String,
    pub auto_ml_job_status: String,
    pub creation_time: DateTime<Utc>,
    pub last_modified_time: DateTime<Utc>,
    pub tags: Vec<TagPair>,
}

/// Represents a SageMaker experiment.
#[derive(Debug, Clone)]
pub struct Experiment {
    pub experiment_name: String,
    pub experiment_arn: String,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub creation_time: DateTime<Utc>,
    pub last_modified_time: DateTime<Utc>,
    pub tags: Vec<TagPair>,
}

/// Represents a SageMaker trial.
#[derive(Debug, Clone)]
pub struct Trial {
    pub trial_name: String,
    pub trial_arn: String,
    pub experiment_name: String,
    pub display_name: Option<String>,
    pub creation_time: DateTime<Utc>,
    pub last_modified_time: DateTime<Utc>,
    pub trial_component_names: Vec<String>,
    pub tags: Vec<TagPair>,
}

/// Represents a SageMaker trial component.
#[derive(Debug, Clone)]
pub struct TrialComponent {
    pub trial_component_name: String,
    pub trial_component_arn: String,
    pub display_name: Option<String>,
    pub status: String,
    pub creation_time: DateTime<Utc>,
    pub last_modified_time: DateTime<Utc>,
    pub tags: Vec<TagPair>,
}

/// Represents a SageMaker pipeline.
#[derive(Debug, Clone)]
pub struct Pipeline {
    pub pipeline_name: String,
    pub pipeline_arn: String,
    pub pipeline_display_name: Option<String>,
    pub pipeline_description: Option<String>,
    pub pipeline_definition: Option<String>,
    pub role_arn: String,
    pub creation_time: DateTime<Utc>,
    pub last_modified_time: DateTime<Utc>,
    pub executions: Vec<PipelineExecution>,
    pub tags: Vec<TagPair>,
}

/// Represents a SageMaker pipeline execution.
#[derive(Debug, Clone)]
pub struct PipelineExecution {
    pub pipeline_execution_arn: String,
    pub pipeline_execution_status: String,
    pub pipeline_execution_description: Option<String>,
    pub creation_time: DateTime<Utc>,
    pub last_modified_time: DateTime<Utc>,
    pub parameters: Vec<PipelineParameter>,
}

/// Represents a pipeline parameter.
#[derive(Debug, Clone)]
pub struct PipelineParameter {
    pub name: String,
    pub value: String,
}

/// Represents a SageMaker feature group.
#[derive(Debug, Clone)]
pub struct FeatureGroup {
    pub feature_group_name: String,
    pub feature_group_arn: String,
    pub feature_group_status: String,
    pub creation_time: DateTime<Utc>,
    pub last_modified_time: DateTime<Utc>,
    pub tags: Vec<TagPair>,
}

/// Represents a SageMaker domain.
#[derive(Debug, Clone)]
pub struct Domain {
    pub domain_id: String,
    pub domain_name: String,
    pub domain_arn: String,
    pub status: String,
    pub creation_time: DateTime<Utc>,
    pub last_modified_time: DateTime<Utc>,
    pub tags: Vec<TagPair>,
    pub vpc_id: Option<String>,
    pub subnet_ids: Vec<String>,
    pub app_network_access_type: Option<String>,
    pub auth_mode: Option<String>,
    pub kms_key_id: Option<String>,
    pub home_efs_file_system_id: Option<String>,
    pub security_group_ids: Vec<String>,
    pub url: Option<String>,
    pub default_space_settings: Option<serde_json::Value>,
    pub domain_settings: Option<serde_json::Value>,
    pub retention_policy: Option<serde_json::Value>,
}

/// Represents a SageMaker cluster.
#[derive(Debug, Clone)]
pub struct Cluster {
    pub cluster_name: String,
    pub cluster_arn: String,
    pub cluster_status: String,
    pub creation_time: DateTime<Utc>,
    pub tags: Vec<TagPair>,
}

/// Represents a SageMaker cluster node.
#[derive(Debug, Clone)]
pub struct ClusterNode {
    pub instance_group_name: String,
    pub instance_id: String,
    pub instance_status: String,
    pub instance_type: String,
    pub launch_time: DateTime<Utc>,
}

/// Represents a monitoring/quality job definition (data quality, model quality, model bias, model explainability).
#[derive(Debug, Clone)]
pub struct JobDefinition {
    pub job_definition_name: String,
    pub job_definition_arn: String,
    pub role_arn: String,
    pub creation_time: DateTime<Utc>,
    pub tags: Vec<TagPair>,
}

/// Represents a SageMaker model card.
#[derive(Debug, Clone)]
pub struct ModelCard {
    pub model_card_name: String,
    pub model_card_arn: String,
    pub model_card_status: String,
    pub content: String,
    pub model_card_version: i32,
    pub creation_time: DateTime<Utc>,
    pub last_modified_time: DateTime<Utc>,
    pub tags: Vec<TagPair>,
}

/// Represents a SageMaker model package.
#[derive(Debug, Clone)]
pub struct ModelPackage {
    pub model_package_name: String,
    pub model_package_arn: String,
    pub model_package_status: String,
    pub model_package_description: Option<String>,
    pub model_approval_status: Option<String>,
    pub creation_time: DateTime<Utc>,
    pub last_modified_time: DateTime<Utc>,
    pub tags: Vec<TagPair>,
}

/// Represents a SageMaker model package group.
#[derive(Debug, Clone)]
pub struct ModelPackageGroup {
    pub model_package_group_name: String,
    pub model_package_group_arn: String,
    pub model_package_group_status: String,
    pub model_package_group_description: Option<String>,
    pub creation_time: DateTime<Utc>,
    pub tags: Vec<TagPair>,
    /// Auto-incrementing version counter for versioned (group-based) packages.
    pub next_version: u32,
}

/// Represents a SageMaker notebook instance lifecycle configuration.
#[derive(Debug, Clone)]
pub struct NotebookInstanceLifecycleConfig {
    pub name: String,
    pub arn: String,
    pub on_create: Vec<LifecycleScript>,
    pub on_start: Vec<LifecycleScript>,
    pub creation_time: DateTime<Utc>,
    pub last_modified_time: DateTime<Utc>,
}

/// A lifecycle script.
#[derive(Debug, Clone)]
pub struct LifecycleScript {
    pub content: String,
}

/// A tag pair.
#[derive(Debug, Clone)]
pub struct TagPair {
    pub key: String,
    pub value: String,
}

/// Represents a SageMaker user profile.
#[derive(Debug, Clone)]
pub struct UserProfile {
    pub domain_id: String,
    pub user_profile_name: String,
    pub user_profile_arn: String,
    pub status: String,
    pub creation_time: DateTime<Utc>,
    pub last_modified_time: DateTime<Utc>,
    pub tags: Vec<TagPair>,
}

/// Represents a SageMaker space.
#[derive(Debug, Clone)]
pub struct Space {
    pub domain_id: String,
    pub space_name: String,
    pub space_arn: String,
    pub status: String,
    pub creation_time: DateTime<Utc>,
    pub last_modified_time: DateTime<Utc>,
    pub tags: Vec<TagPair>,
}

/// Represents a SageMaker app.
#[derive(Debug, Clone)]
pub struct App {
    pub domain_id: String,
    pub user_profile_name: Option<String>,
    pub space_name: Option<String>,
    pub app_type: String,
    pub app_name: String,
    pub app_arn: String,
    pub status: String,
    pub creation_time: DateTime<Utc>,
    pub tags: Vec<TagPair>,
}

/// Mapping from resource ARN to tags (shared across all resource types).
pub type TagStore = HashMap<String, Vec<TagPair>>;
