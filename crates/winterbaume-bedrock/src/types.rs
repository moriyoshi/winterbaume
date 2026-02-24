use serde::{Deserialize, Serialize};

/// A foundation model available in Bedrock.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoundationModel {
    pub model_id: String,
    pub model_name: String,
    pub provider_name: String,
    pub model_arn: String,
    pub input_modalities: Vec<String>,
    pub output_modalities: Vec<String>,
    pub response_streaming_supported: bool,
    pub customizations_supported: Vec<String>,
    pub inference_types_supported: Vec<String>,
    pub model_lifecycle_status: String,
}

/// A model customization job.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelCustomizationJob {
    pub job_arn: String,
    pub job_name: String,
    pub base_model_identifier: String,
    pub custom_model_name: String,
    pub customization_type: String,
    pub role_arn: String,
    pub status: String,
    pub creation_time: String,
    pub last_modified_time: String,
    pub training_data_config: TrainingDataConfig,
    pub output_data_config: OutputDataConfig,
    pub hyper_parameters: std::collections::HashMap<String, String>,
}

/// Training data configuration for a customization job.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingDataConfig {
    pub s3_uri: String,
}

/// Output data configuration for a customization job.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputDataConfig {
    pub s3_uri: String,
}

/// A custom model created by a completed customization job.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomModel {
    pub model_arn: String,
    pub model_name: String,
    pub base_model_arn: String,
    pub customization_type: String,
    pub creation_time: String,
    pub job_arn: String,
    pub job_name: String,
    pub training_data_config: TrainingDataConfig,
    pub output_data_config: OutputDataConfig,
    pub hyper_parameters: std::collections::HashMap<String, String>,
}

/// Model invocation logging configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfiguration {
    pub text_data_delivery_enabled: Option<bool>,
    pub image_data_delivery_enabled: Option<bool>,
    pub embedding_data_delivery_enabled: Option<bool>,
    pub s3_config: Option<S3LogConfig>,
    pub cloud_watch_config: Option<CloudWatchLogConfig>,
}

/// S3 logging configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct S3LogConfig {
    pub bucket_name: String,
    pub key_prefix: Option<String>,
}

/// CloudWatch logging configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudWatchLogConfig {
    pub log_group_name: String,
    pub role_arn: String,
}

/// A resource tag.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceTag {
    pub key: String,
    pub value: String,
}

/// A Bedrock guardrail.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Guardrail {
    pub guardrail_id: String,
    pub guardrail_arn: String,
    pub name: String,
    pub description: Option<String>,
    pub status: String,
    pub version: String,
    pub created_at: String,
    pub updated_at: String,
    pub blocked_input_messaging: String,
    pub blocked_outputs_messaging: String,
    #[serde(default)]
    pub content_policy_config: Option<serde_json::Value>,
    #[serde(default)]
    pub contextual_grounding_policy_config: Option<serde_json::Value>,
    #[serde(default)]
    pub sensitive_information_policy_config: Option<serde_json::Value>,
    #[serde(default)]
    pub topic_policy_config: Option<serde_json::Value>,
    #[serde(default)]
    pub word_policy_config: Option<serde_json::Value>,
}

/// A provisioned model throughput resource.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvisionedModelThroughput {
    pub provisioned_model_arn: String,
    pub provisioned_model_name: String,
    pub model_arn: String,
    pub model_units: i32,
    pub status: String,
    pub commitment_duration: Option<String>,
    pub creation_time: String,
    pub last_modified_time: String,
}

/// A customer inference profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceProfile {
    pub inference_profile_arn: String,
    pub inference_profile_id: String,
    pub inference_profile_name: String,
    pub description: Option<String>,
    pub status: String,
    pub profile_type: String,
    pub model_arn: String,
    pub created_at: String,
    pub updated_at: String,
}

/// A prompt router.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptRouter {
    pub prompt_router_arn: String,
    pub prompt_router_name: String,
    pub description: Option<String>,
    pub status: String,
    pub router_type: String,
    pub fallback_model_arn: String,
    pub models: Vec<String>,
    pub routing_criteria_response_quality_difference: f64,
    pub created_at: String,
    pub updated_at: String,
}

/// An evaluation job.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationJob {
    pub job_arn: String,
    pub job_name: String,
    pub job_description: Option<String>,
    pub status: String,
    pub creation_time: String,
    pub last_modified_time: String,
}

/// A model invocation job.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInvocationJob {
    pub job_arn: String,
    pub job_name: String,
    pub model_id: String,
    pub role_arn: String,
    pub input_s3_uri: String,
    pub output_s3_uri: String,
    pub status: String,
    pub submit_time: String,
    pub last_modified_time: String,
}

/// A model import job.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelImportJob {
    pub job_arn: String,
    pub job_name: String,
    pub imported_model_name: String,
    pub imported_model_arn: String,
    pub role_arn: String,
    pub status: String,
    pub creation_time: String,
    pub last_modified_time: String,
}

/// A model copy job.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelCopyJob {
    pub job_arn: String,
    pub source_model_arn: String,
    pub source_account_id: String,
    pub target_model_arn: String,
    pub target_model_name: String,
    pub status: String,
    pub creation_time: String,
}
