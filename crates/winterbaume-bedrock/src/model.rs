//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-bedrock

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDeleteEvaluationJobRequest {
    #[serde(rename = "jobIdentifiers")]
    #[serde(default)]
    pub job_identifiers: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDeleteEvaluationJobResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<BatchDeleteEvaluationJobError>>,
    #[serde(rename = "evaluationJobs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_jobs: Option<Vec<BatchDeleteEvaluationJobItem>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDeleteEvaluationJobError {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "jobIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_identifier: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDeleteEvaluationJobItem {
    #[serde(rename = "jobIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_identifier: Option<String>,
    #[serde(rename = "jobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelAutomatedReasoningPolicyBuildWorkflowRequest {
    #[serde(rename = "buildWorkflowId")]
    #[serde(default)]
    pub build_workflow_id: String,
    #[serde(rename = "policyArn")]
    #[serde(default)]
    pub policy_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelAutomatedReasoningPolicyBuildWorkflowResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAutomatedReasoningPolicyRequest {
    #[serde(rename = "clientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "kmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "policyDefinition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_definition: Option<AutomatedReasoningPolicyDefinition>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningPolicyDefinition {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<AutomatedReasoningPolicyDefinitionRule>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<AutomatedReasoningPolicyDefinitionType>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<Vec<AutomatedReasoningPolicyDefinitionVariable>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningPolicyDefinitionRule {
    #[serde(rename = "alternateExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternate_expression: Option<String>,
    #[serde(default)]
    pub expression: String,
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningPolicyDefinitionType {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub values: Vec<AutomatedReasoningPolicyDefinitionTypeValue>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningPolicyDefinitionTypeValue {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningPolicyDefinitionVariable {
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Tag {
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAutomatedReasoningPolicyResponse {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "definitionHash")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition_hash: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "policyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_arn: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAutomatedReasoningPolicyTestCaseRequest {
    #[serde(rename = "clientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "confidenceThreshold")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence_threshold: Option<f64>,
    #[serde(rename = "expectedAggregatedFindingsResult")]
    #[serde(default)]
    pub expected_aggregated_findings_result: String,
    #[serde(rename = "guardContent")]
    #[serde(default)]
    pub guard_content: String,
    #[serde(rename = "policyArn")]
    #[serde(default)]
    pub policy_arn: String,
    #[serde(rename = "queryContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_content: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAutomatedReasoningPolicyTestCaseResponse {
    #[serde(rename = "policyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_arn: Option<String>,
    #[serde(rename = "testCaseId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_case_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAutomatedReasoningPolicyVersionRequest {
    #[serde(rename = "clientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "lastUpdatedDefinitionHash")]
    #[serde(default)]
    pub last_updated_definition_hash: String,
    #[serde(rename = "policyArn")]
    #[serde(default)]
    pub policy_arn: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAutomatedReasoningPolicyVersionResponse {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "definitionHash")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition_hash: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "policyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCustomModelDeploymentRequest {
    #[serde(rename = "clientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "modelArn")]
    #[serde(default)]
    pub model_arn: String,
    #[serde(rename = "modelDeploymentName")]
    #[serde(default)]
    pub model_deployment_name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCustomModelDeploymentResponse {
    #[serde(rename = "customModelDeploymentArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_model_deployment_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCustomModelRequest {
    #[serde(rename = "clientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "modelKmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_kms_key_arn: Option<String>,
    #[serde(rename = "modelName")]
    #[serde(default)]
    pub model_name: String,
    #[serde(rename = "modelSourceConfig")]
    #[serde(default)]
    pub model_source_config: ModelDataSource,
    #[serde(rename = "modelTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_tags: Option<Vec<Tag>>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModelDataSource {
    #[serde(rename = "s3DataSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_data_source: Option<S3DataSource>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3DataSource {
    #[serde(rename = "s3Uri")]
    #[serde(default)]
    pub s3_uri: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCustomModelResponse {
    #[serde(rename = "modelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateEvaluationJobRequest {
    #[serde(rename = "applicationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_type: Option<String>,
    #[serde(rename = "clientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "customerEncryptionKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_encryption_key_id: Option<String>,
    #[serde(rename = "evaluationConfig")]
    #[serde(default)]
    pub evaluation_config: EvaluationConfig,
    #[serde(rename = "inferenceConfig")]
    #[serde(default)]
    pub inference_config: EvaluationInferenceConfig,
    #[serde(rename = "jobDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_description: Option<String>,
    #[serde(rename = "jobName")]
    #[serde(default)]
    pub job_name: String,
    #[serde(rename = "jobTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_tags: Option<Vec<Tag>>,
    #[serde(rename = "outputDataConfig")]
    #[serde(default)]
    pub output_data_config: EvaluationOutputDataConfig,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    pub role_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationConfig {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automated: Option<AutomatedEvaluationConfig>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub human: Option<HumanEvaluationConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedEvaluationConfig {
    #[serde(rename = "customMetricConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_metric_config: Option<AutomatedEvaluationCustomMetricConfig>,
    #[serde(rename = "datasetMetricConfigs")]
    #[serde(default)]
    pub dataset_metric_configs: Vec<EvaluationDatasetMetricConfig>,
    #[serde(rename = "evaluatorModelConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluator_model_config: Option<EvaluatorModelConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedEvaluationCustomMetricConfig {
    #[serde(rename = "customMetrics")]
    #[serde(default)]
    pub custom_metrics: Vec<AutomatedEvaluationCustomMetricSource>,
    #[serde(rename = "evaluatorModelConfig")]
    #[serde(default)]
    pub evaluator_model_config: CustomMetricEvaluatorModelConfig,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedEvaluationCustomMetricSource {
    #[serde(rename = "customMetricDefinition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_metric_definition: Option<CustomMetricDefinition>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomMetricDefinition {
    #[serde(default)]
    pub instructions: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "ratingScale")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rating_scale: Option<Vec<RatingScaleItem>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RatingScaleItem {
    #[serde(default)]
    pub definition: String,
    #[serde(default)]
    pub value: RatingScaleItemValue,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RatingScaleItemValue {
    #[serde(rename = "floatValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub float_value: Option<f32>,
    #[serde(rename = "stringValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomMetricEvaluatorModelConfig {
    #[serde(rename = "bedrockEvaluatorModels")]
    #[serde(default)]
    pub bedrock_evaluator_models: Vec<CustomMetricBedrockEvaluatorModel>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomMetricBedrockEvaluatorModel {
    #[serde(rename = "modelIdentifier")]
    #[serde(default)]
    pub model_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationDatasetMetricConfig {
    #[serde(default)]
    pub dataset: EvaluationDataset,
    #[serde(rename = "metricNames")]
    #[serde(default)]
    pub metric_names: Vec<String>,
    #[serde(rename = "taskType")]
    #[serde(default)]
    pub task_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationDataset {
    #[serde(rename = "datasetLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_location: Option<EvaluationDatasetLocation>,
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationDatasetLocation {
    #[serde(rename = "s3Uri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluatorModelConfig {
    #[serde(rename = "bedrockEvaluatorModels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bedrock_evaluator_models: Option<Vec<BedrockEvaluatorModel>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BedrockEvaluatorModel {
    #[serde(rename = "modelIdentifier")]
    #[serde(default)]
    pub model_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HumanEvaluationConfig {
    #[serde(rename = "customMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_metrics: Option<Vec<HumanEvaluationCustomMetric>>,
    #[serde(rename = "datasetMetricConfigs")]
    #[serde(default)]
    pub dataset_metric_configs: Vec<EvaluationDatasetMetricConfig>,
    #[serde(rename = "humanWorkflowConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub human_workflow_config: Option<HumanWorkflowConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HumanEvaluationCustomMetric {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "ratingMethod")]
    #[serde(default)]
    pub rating_method: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HumanWorkflowConfig {
    #[serde(rename = "flowDefinitionArn")]
    #[serde(default)]
    pub flow_definition_arn: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationInferenceConfig {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub models: Option<Vec<EvaluationModelConfig>>,
    #[serde(rename = "ragConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rag_configs: Option<Vec<RAGConfig>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationModelConfig {
    #[serde(rename = "bedrockModel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bedrock_model: Option<EvaluationBedrockModel>,
    #[serde(rename = "precomputedInferenceSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precomputed_inference_source: Option<EvaluationPrecomputedInferenceSource>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationBedrockModel {
    #[serde(rename = "inferenceParams")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_params: Option<String>,
    #[serde(rename = "modelIdentifier")]
    #[serde(default)]
    pub model_identifier: String,
    #[serde(rename = "performanceConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_config: Option<PerformanceConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PerformanceConfiguration {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latency: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationPrecomputedInferenceSource {
    #[serde(rename = "inferenceSourceIdentifier")]
    #[serde(default)]
    pub inference_source_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RAGConfig {
    #[serde(rename = "knowledgeBaseConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knowledge_base_config: Option<KnowledgeBaseConfig>,
    #[serde(rename = "precomputedRagSourceConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precomputed_rag_source_config: Option<EvaluationPrecomputedRagSourceConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KnowledgeBaseConfig {
    #[serde(rename = "retrieveAndGenerateConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieve_and_generate_config: Option<RetrieveAndGenerateConfiguration>,
    #[serde(rename = "retrieveConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieve_config: Option<RetrieveConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RetrieveAndGenerateConfiguration {
    #[serde(rename = "externalSourcesConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_sources_configuration: Option<ExternalSourcesRetrieveAndGenerateConfiguration>,
    #[serde(rename = "knowledgeBaseConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knowledge_base_configuration: Option<KnowledgeBaseRetrieveAndGenerateConfiguration>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExternalSourcesRetrieveAndGenerateConfiguration {
    #[serde(rename = "generationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_configuration: Option<ExternalSourcesGenerationConfiguration>,
    #[serde(rename = "modelArn")]
    #[serde(default)]
    pub model_arn: String,
    #[serde(default)]
    pub sources: Vec<ExternalSource>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExternalSourcesGenerationConfiguration {
    #[serde(rename = "additionalModelRequestFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_model_request_fields:
        Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "guardrailConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guardrail_configuration: Option<GuardrailConfiguration>,
    #[serde(rename = "kbInferenceConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kb_inference_config: Option<KbInferenceConfig>,
    #[serde(rename = "promptTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_template: Option<PromptTemplate>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GuardrailConfiguration {
    #[serde(rename = "guardrailId")]
    #[serde(default)]
    pub guardrail_id: String,
    #[serde(rename = "guardrailVersion")]
    #[serde(default)]
    pub guardrail_version: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KbInferenceConfig {
    #[serde(rename = "textInferenceConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_inference_config: Option<TextInferenceConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TextInferenceConfig {
    #[serde(rename = "maxTokens")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i32>,
    #[serde(rename = "stopSequences")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_sequences: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(rename = "topP")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PromptTemplate {
    #[serde(rename = "textPromptTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_prompt_template: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExternalSource {
    #[serde(rename = "byteContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub byte_content: Option<ByteContentDoc>,
    #[serde(rename = "s3Location")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_location: Option<S3ObjectDoc>,
    #[serde(rename = "sourceType")]
    #[serde(default)]
    pub source_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ByteContentDoc {
    #[serde(rename = "contentType")]
    #[serde(default)]
    pub content_type: String,
    #[serde(default)]
    pub data: String,
    #[serde(default)]
    pub identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3ObjectDoc {
    #[serde(default)]
    pub uri: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KnowledgeBaseRetrieveAndGenerateConfiguration {
    #[serde(rename = "generationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_configuration: Option<GenerationConfiguration>,
    #[serde(rename = "knowledgeBaseId")]
    #[serde(default)]
    pub knowledge_base_id: String,
    #[serde(rename = "modelArn")]
    #[serde(default)]
    pub model_arn: String,
    #[serde(rename = "orchestrationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orchestration_configuration: Option<OrchestrationConfiguration>,
    #[serde(rename = "retrievalConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieval_configuration: Option<KnowledgeBaseRetrievalConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GenerationConfiguration {
    #[serde(rename = "additionalModelRequestFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_model_request_fields:
        Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "guardrailConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guardrail_configuration: Option<GuardrailConfiguration>,
    #[serde(rename = "kbInferenceConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kb_inference_config: Option<KbInferenceConfig>,
    #[serde(rename = "promptTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_template: Option<PromptTemplate>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OrchestrationConfiguration {
    #[serde(rename = "queryTransformationConfiguration")]
    #[serde(default)]
    pub query_transformation_configuration: QueryTransformationConfiguration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueryTransformationConfiguration {
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KnowledgeBaseRetrievalConfiguration {
    #[serde(rename = "vectorSearchConfiguration")]
    #[serde(default)]
    pub vector_search_configuration: KnowledgeBaseVectorSearchConfiguration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KnowledgeBaseVectorSearchConfiguration {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<RetrievalFilter>,
    #[serde(rename = "implicitFilterConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub implicit_filter_configuration: Option<ImplicitFilterConfiguration>,
    #[serde(rename = "numberOfResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_results: Option<i32>,
    #[serde(rename = "overrideSearchType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_search_type: Option<String>,
    #[serde(rename = "rerankingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reranking_configuration: Option<VectorSearchRerankingConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RetrievalFilter {
    #[serde(rename = "andAll")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and_all: Option<Vec<RetrievalFilter>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub equals: Option<FilterAttribute>,
    #[serde(rename = "greaterThan")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub greater_than: Option<FilterAttribute>,
    #[serde(rename = "greaterThanOrEquals")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub greater_than_or_equals: Option<FilterAttribute>,
    #[serde(rename = "in")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#in: Option<FilterAttribute>,
    #[serde(rename = "lessThan")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub less_than: Option<FilterAttribute>,
    #[serde(rename = "lessThanOrEquals")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub less_than_or_equals: Option<FilterAttribute>,
    #[serde(rename = "listContains")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_contains: Option<FilterAttribute>,
    #[serde(rename = "notEquals")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_equals: Option<FilterAttribute>,
    #[serde(rename = "notIn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_in: Option<FilterAttribute>,
    #[serde(rename = "orAll")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub or_all: Option<Vec<RetrievalFilter>>,
    #[serde(rename = "startsWith")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starts_with: Option<FilterAttribute>,
    #[serde(rename = "stringContains")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_contains: Option<FilterAttribute>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FilterAttribute {
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    pub value: serde_json::Value,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImplicitFilterConfiguration {
    #[serde(rename = "metadataAttributes")]
    #[serde(default)]
    pub metadata_attributes: Vec<MetadataAttributeSchema>,
    #[serde(rename = "modelArn")]
    #[serde(default)]
    pub model_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MetadataAttributeSchema {
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub key: String,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VectorSearchRerankingConfiguration {
    #[serde(rename = "bedrockRerankingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bedrock_reranking_configuration: Option<VectorSearchBedrockRerankingConfiguration>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VectorSearchBedrockRerankingConfiguration {
    #[serde(rename = "metadataConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_configuration: Option<MetadataConfigurationForReranking>,
    #[serde(rename = "modelConfiguration")]
    #[serde(default)]
    pub model_configuration: VectorSearchBedrockRerankingModelConfiguration,
    #[serde(rename = "numberOfRerankedResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_reranked_results: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MetadataConfigurationForReranking {
    #[serde(rename = "selectionMode")]
    #[serde(default)]
    pub selection_mode: String,
    #[serde(rename = "selectiveModeConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective_mode_configuration: Option<RerankingMetadataSelectiveModeConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RerankingMetadataSelectiveModeConfiguration {
    #[serde(rename = "fieldsToExclude")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields_to_exclude: Option<Vec<FieldForReranking>>,
    #[serde(rename = "fieldsToInclude")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields_to_include: Option<Vec<FieldForReranking>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FieldForReranking {
    #[serde(rename = "fieldName")]
    #[serde(default)]
    pub field_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VectorSearchBedrockRerankingModelConfiguration {
    #[serde(rename = "additionalModelRequestFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_model_request_fields:
        Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "modelArn")]
    #[serde(default)]
    pub model_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RetrieveConfig {
    #[serde(rename = "knowledgeBaseId")]
    #[serde(default)]
    pub knowledge_base_id: String,
    #[serde(rename = "knowledgeBaseRetrievalConfiguration")]
    #[serde(default)]
    pub knowledge_base_retrieval_configuration: KnowledgeBaseRetrievalConfiguration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationPrecomputedRagSourceConfig {
    #[serde(rename = "retrieveAndGenerateSourceConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieve_and_generate_source_config:
        Option<EvaluationPrecomputedRetrieveAndGenerateSourceConfig>,
    #[serde(rename = "retrieveSourceConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieve_source_config: Option<EvaluationPrecomputedRetrieveSourceConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationPrecomputedRetrieveAndGenerateSourceConfig {
    #[serde(rename = "ragSourceIdentifier")]
    #[serde(default)]
    pub rag_source_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationPrecomputedRetrieveSourceConfig {
    #[serde(rename = "ragSourceIdentifier")]
    #[serde(default)]
    pub rag_source_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationOutputDataConfig {
    #[serde(rename = "s3Uri")]
    #[serde(default)]
    pub s3_uri: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateEvaluationJobResponse {
    #[serde(rename = "jobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFoundationModelAgreementRequest {
    #[serde(rename = "modelId")]
    #[serde(default)]
    pub model_id: String,
    #[serde(rename = "offerToken")]
    #[serde(default)]
    pub offer_token: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFoundationModelAgreementResponse {
    #[serde(rename = "modelId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateGuardrailRequest {
    #[serde(rename = "automatedReasoningPolicyConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automated_reasoning_policy_config: Option<GuardrailAutomatedReasoningPolicyConfig>,
    #[serde(rename = "blockedInputMessaging")]
    #[serde(default)]
    pub blocked_input_messaging: String,
    #[serde(rename = "blockedOutputsMessaging")]
    #[serde(default)]
    pub blocked_outputs_messaging: String,
    #[serde(rename = "clientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "contentPolicyConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_policy_config: Option<GuardrailContentPolicyConfig>,
    #[serde(rename = "contextualGroundingPolicyConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contextual_grounding_policy_config: Option<GuardrailContextualGroundingPolicyConfig>,
    #[serde(rename = "crossRegionConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_region_config: Option<GuardrailCrossRegionConfig>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "kmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "sensitiveInformationPolicyConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensitive_information_policy_config: Option<GuardrailSensitiveInformationPolicyConfig>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "topicPolicyConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_policy_config: Option<GuardrailTopicPolicyConfig>,
    #[serde(rename = "wordPolicyConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub word_policy_config: Option<GuardrailWordPolicyConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GuardrailAutomatedReasoningPolicyConfig {
    #[serde(rename = "confidenceThreshold")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence_threshold: Option<f64>,
    #[serde(default)]
    pub policies: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GuardrailContentPolicyConfig {
    #[serde(rename = "filtersConfig")]
    #[serde(default)]
    pub filters_config: Vec<GuardrailContentFilterConfig>,
    #[serde(rename = "tierConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier_config: Option<GuardrailContentFiltersTierConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GuardrailContentFilterConfig {
    #[serde(rename = "inputAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_action: Option<String>,
    #[serde(rename = "inputEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_enabled: Option<bool>,
    #[serde(rename = "inputModalities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_modalities: Option<Vec<String>>,
    #[serde(rename = "inputStrength")]
    #[serde(default)]
    pub input_strength: String,
    #[serde(rename = "outputAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_action: Option<String>,
    #[serde(rename = "outputEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_enabled: Option<bool>,
    #[serde(rename = "outputModalities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_modalities: Option<Vec<String>>,
    #[serde(rename = "outputStrength")]
    #[serde(default)]
    pub output_strength: String,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GuardrailContentFiltersTierConfig {
    #[serde(rename = "tierName")]
    #[serde(default)]
    pub tier_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GuardrailContextualGroundingPolicyConfig {
    #[serde(rename = "filtersConfig")]
    #[serde(default)]
    pub filters_config: Vec<GuardrailContextualGroundingFilterConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GuardrailContextualGroundingFilterConfig {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default)]
    pub threshold: f64,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GuardrailCrossRegionConfig {
    #[serde(rename = "guardrailProfileIdentifier")]
    #[serde(default)]
    pub guardrail_profile_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GuardrailSensitiveInformationPolicyConfig {
    #[serde(rename = "piiEntitiesConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pii_entities_config: Option<Vec<GuardrailPiiEntityConfig>>,
    #[serde(rename = "regexesConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regexes_config: Option<Vec<GuardrailRegexConfig>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GuardrailPiiEntityConfig {
    #[serde(default)]
    pub action: String,
    #[serde(rename = "inputAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_action: Option<String>,
    #[serde(rename = "inputEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_enabled: Option<bool>,
    #[serde(rename = "outputAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_action: Option<String>,
    #[serde(rename = "outputEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_enabled: Option<bool>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GuardrailRegexConfig {
    #[serde(default)]
    pub action: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "inputAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_action: Option<String>,
    #[serde(rename = "inputEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_enabled: Option<bool>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "outputAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_action: Option<String>,
    #[serde(rename = "outputEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_enabled: Option<bool>,
    #[serde(default)]
    pub pattern: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GuardrailTopicPolicyConfig {
    #[serde(rename = "tierConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier_config: Option<GuardrailTopicsTierConfig>,
    #[serde(rename = "topicsConfig")]
    #[serde(default)]
    pub topics_config: Vec<GuardrailTopicConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GuardrailTopicsTierConfig {
    #[serde(rename = "tierName")]
    #[serde(default)]
    pub tier_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GuardrailTopicConfig {
    #[serde(default)]
    pub definition: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub examples: Option<Vec<String>>,
    #[serde(rename = "inputAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_action: Option<String>,
    #[serde(rename = "inputEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_enabled: Option<bool>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "outputAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_action: Option<String>,
    #[serde(rename = "outputEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_enabled: Option<bool>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GuardrailWordPolicyConfig {
    #[serde(rename = "managedWordListsConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_word_lists_config: Option<Vec<GuardrailManagedWordsConfig>>,
    #[serde(rename = "wordsConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub words_config: Option<Vec<GuardrailWordConfig>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GuardrailManagedWordsConfig {
    #[serde(rename = "inputAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_action: Option<String>,
    #[serde(rename = "inputEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_enabled: Option<bool>,
    #[serde(rename = "outputAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_action: Option<String>,
    #[serde(rename = "outputEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_enabled: Option<bool>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GuardrailWordConfig {
    #[serde(rename = "inputAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_action: Option<String>,
    #[serde(rename = "inputEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_enabled: Option<bool>,
    #[serde(rename = "outputAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_action: Option<String>,
    #[serde(rename = "outputEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_enabled: Option<bool>,
    #[serde(default)]
    pub text: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateGuardrailResponse {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "guardrailArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guardrail_arn: Option<String>,
    #[serde(rename = "guardrailId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guardrail_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateGuardrailVersionRequest {
    #[serde(rename = "clientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "guardrailIdentifier")]
    #[serde(default)]
    pub guardrail_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateGuardrailVersionResponse {
    #[serde(rename = "guardrailId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guardrail_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateInferenceProfileRequest {
    #[serde(rename = "clientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "inferenceProfileName")]
    #[serde(default)]
    pub inference_profile_name: String,
    #[serde(rename = "modelSource")]
    #[serde(default)]
    pub model_source: InferenceProfileModelSource,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InferenceProfileModelSource {
    #[serde(rename = "copyFrom")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_from: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateInferenceProfileResponse {
    #[serde(rename = "inferenceProfileArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_profile_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateMarketplaceModelEndpointRequest {
    #[serde(rename = "acceptEula")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_eula: Option<bool>,
    #[serde(rename = "clientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "endpointConfig")]
    #[serde(default)]
    pub endpoint_config: EndpointConfig,
    #[serde(rename = "endpointName")]
    #[serde(default)]
    pub endpoint_name: String,
    #[serde(rename = "modelSourceIdentifier")]
    #[serde(default)]
    pub model_source_identifier: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EndpointConfig {
    #[serde(rename = "sageMaker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sage_maker: Option<SageMakerEndpoint>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SageMakerEndpoint {
    #[serde(rename = "executionRole")]
    #[serde(default)]
    pub execution_role: String,
    #[serde(rename = "initialInstanceCount")]
    #[serde(default)]
    pub initial_instance_count: i32,
    #[serde(rename = "instanceType")]
    #[serde(default)]
    pub instance_type: String,
    #[serde(rename = "kmsEncryptionKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_encryption_key: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc: Option<VpcConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VpcConfig {
    #[serde(rename = "securityGroupIds")]
    #[serde(default)]
    pub security_group_ids: Vec<String>,
    #[serde(rename = "subnetIds")]
    #[serde(default)]
    pub subnet_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateMarketplaceModelEndpointResponse {
    #[serde(rename = "marketplaceModelEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketplace_model_endpoint: Option<MarketplaceModelEndpoint>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MarketplaceModelEndpoint {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "endpointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_arn: Option<String>,
    #[serde(rename = "endpointConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_config: Option<EndpointConfig>,
    #[serde(rename = "endpointStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_status: Option<String>,
    #[serde(rename = "endpointStatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_status_message: Option<String>,
    #[serde(rename = "modelSourceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_source_identifier: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateModelCopyJobRequest {
    #[serde(rename = "clientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "modelKmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_kms_key_id: Option<String>,
    #[serde(rename = "sourceModelArn")]
    #[serde(default)]
    pub source_model_arn: String,
    #[serde(rename = "targetModelName")]
    #[serde(default)]
    pub target_model_name: String,
    #[serde(rename = "targetModelTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_model_tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateModelCopyJobResponse {
    #[serde(rename = "jobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateModelCustomizationJobRequest {
    #[serde(rename = "baseModelIdentifier")]
    #[serde(default)]
    pub base_model_identifier: String,
    #[serde(rename = "clientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "customModelKmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_model_kms_key_id: Option<String>,
    #[serde(rename = "customModelName")]
    #[serde(default)]
    pub custom_model_name: String,
    #[serde(rename = "customModelTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_model_tags: Option<Vec<Tag>>,
    #[serde(rename = "customizationConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customization_config: Option<CustomizationConfig>,
    #[serde(rename = "customizationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customization_type: Option<String>,
    #[serde(rename = "hyperParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hyper_parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "jobName")]
    #[serde(default)]
    pub job_name: String,
    #[serde(rename = "jobTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_tags: Option<Vec<Tag>>,
    #[serde(rename = "outputDataConfig")]
    #[serde(default)]
    pub output_data_config: OutputDataConfig,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(rename = "trainingDataConfig")]
    #[serde(default)]
    pub training_data_config: TrainingDataConfig,
    #[serde(rename = "validationDataConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_data_config: Option<ValidationDataConfig>,
    #[serde(rename = "vpcConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomizationConfig {
    #[serde(rename = "distillationConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distillation_config: Option<DistillationConfig>,
    #[serde(rename = "rftConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rft_config: Option<RFTConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DistillationConfig {
    #[serde(rename = "teacherModelConfig")]
    #[serde(default)]
    pub teacher_model_config: TeacherModelConfig,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TeacherModelConfig {
    #[serde(rename = "maxResponseLengthForInference")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_response_length_for_inference: Option<i32>,
    #[serde(rename = "teacherModelIdentifier")]
    #[serde(default)]
    pub teacher_model_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RFTConfig {
    #[serde(rename = "graderConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grader_config: Option<GraderConfig>,
    #[serde(rename = "hyperParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hyper_parameters: Option<RFTHyperParameters>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GraderConfig {
    #[serde(rename = "lambdaGrader")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_grader: Option<LambdaGraderConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LambdaGraderConfig {
    #[serde(rename = "lambdaArn")]
    #[serde(default)]
    pub lambda_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RFTHyperParameters {
    #[serde(rename = "batchSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<i32>,
    #[serde(rename = "epochCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub epoch_count: Option<i32>,
    #[serde(rename = "evalInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eval_interval: Option<i32>,
    #[serde(rename = "inferenceMaxTokens")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_max_tokens: Option<i32>,
    #[serde(rename = "learningRate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub learning_rate: Option<f32>,
    #[serde(rename = "maxPromptLength")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_prompt_length: Option<i32>,
    #[serde(rename = "reasoningEffort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_effort: Option<String>,
    #[serde(rename = "trainingSamplePerPrompt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_sample_per_prompt: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OutputDataConfig {
    #[serde(rename = "s3Uri")]
    #[serde(default)]
    pub s3_uri: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TrainingDataConfig {
    #[serde(rename = "invocationLogsConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invocation_logs_config: Option<InvocationLogsConfig>,
    #[serde(rename = "s3Uri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InvocationLogsConfig {
    #[serde(rename = "invocationLogSource")]
    #[serde(default)]
    pub invocation_log_source: InvocationLogSource,
    #[serde(rename = "requestMetadataFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_metadata_filters: Option<RequestMetadataFilters>,
    #[serde(rename = "usePromptResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_prompt_response: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InvocationLogSource {
    #[serde(rename = "s3Uri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RequestMetadataFilters {
    #[serde(rename = "andAll")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and_all: Option<Vec<RequestMetadataBaseFilters>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub equals: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "notEquals")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_equals: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "orAll")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub or_all: Option<Vec<RequestMetadataBaseFilters>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RequestMetadataBaseFilters {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub equals: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "notEquals")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_equals: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ValidationDataConfig {
    #[serde(default)]
    pub validators: Vec<Validator>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Validator {
    #[serde(rename = "s3Uri")]
    #[serde(default)]
    pub s3_uri: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateModelCustomizationJobResponse {
    #[serde(rename = "jobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateModelImportJobRequest {
    #[serde(rename = "clientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "importedModelKmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imported_model_kms_key_id: Option<String>,
    #[serde(rename = "importedModelName")]
    #[serde(default)]
    pub imported_model_name: String,
    #[serde(rename = "importedModelTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imported_model_tags: Option<Vec<Tag>>,
    #[serde(rename = "jobName")]
    #[serde(default)]
    pub job_name: String,
    #[serde(rename = "jobTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_tags: Option<Vec<Tag>>,
    #[serde(rename = "modelDataSource")]
    #[serde(default)]
    pub model_data_source: ModelDataSource,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(rename = "vpcConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateModelImportJobResponse {
    #[serde(rename = "jobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateModelInvocationJobRequest {
    #[serde(rename = "clientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "inputDataConfig")]
    #[serde(default)]
    pub input_data_config: ModelInvocationJobInputDataConfig,
    #[serde(rename = "jobName")]
    #[serde(default)]
    pub job_name: String,
    #[serde(rename = "modelId")]
    #[serde(default)]
    pub model_id: String,
    #[serde(rename = "modelInvocationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_invocation_type: Option<String>,
    #[serde(rename = "outputDataConfig")]
    #[serde(default)]
    pub output_data_config: ModelInvocationJobOutputDataConfig,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "timeoutDurationInHours")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_duration_in_hours: Option<i32>,
    #[serde(rename = "vpcConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModelInvocationJobInputDataConfig {
    #[serde(rename = "s3InputDataConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_input_data_config: Option<ModelInvocationJobS3InputDataConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModelInvocationJobS3InputDataConfig {
    #[serde(rename = "s3BucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_owner: Option<String>,
    #[serde(rename = "s3InputFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_input_format: Option<String>,
    #[serde(rename = "s3Uri")]
    #[serde(default)]
    pub s3_uri: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModelInvocationJobOutputDataConfig {
    #[serde(rename = "s3OutputDataConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_output_data_config: Option<ModelInvocationJobS3OutputDataConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModelInvocationJobS3OutputDataConfig {
    #[serde(rename = "s3BucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_owner: Option<String>,
    #[serde(rename = "s3EncryptionKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_encryption_key_id: Option<String>,
    #[serde(rename = "s3Uri")]
    #[serde(default)]
    pub s3_uri: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateModelInvocationJobResponse {
    #[serde(rename = "jobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePromptRouterRequest {
    #[serde(rename = "clientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "fallbackModel")]
    #[serde(default)]
    pub fallback_model: PromptRouterTargetModel,
    #[serde(default)]
    pub models: Vec<PromptRouterTargetModel>,
    #[serde(rename = "promptRouterName")]
    #[serde(default)]
    pub prompt_router_name: String,
    #[serde(rename = "routingCriteria")]
    #[serde(default)]
    pub routing_criteria: RoutingCriteria,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PromptRouterTargetModel {
    #[serde(rename = "modelArn")]
    #[serde(default)]
    pub model_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RoutingCriteria {
    #[serde(rename = "responseQualityDifference")]
    #[serde(default)]
    pub response_quality_difference: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePromptRouterResponse {
    #[serde(rename = "promptRouterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_router_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateProvisionedModelThroughputRequest {
    #[serde(rename = "clientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "commitmentDuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment_duration: Option<String>,
    #[serde(rename = "modelId")]
    #[serde(default)]
    pub model_id: String,
    #[serde(rename = "modelUnits")]
    #[serde(default)]
    pub model_units: i32,
    #[serde(rename = "provisionedModelName")]
    #[serde(default)]
    pub provisioned_model_name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateProvisionedModelThroughputResponse {
    #[serde(rename = "provisionedModelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_model_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAutomatedReasoningPolicyBuildWorkflowRequest {
    #[serde(rename = "buildWorkflowId")]
    #[serde(default)]
    pub build_workflow_id: String,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    pub last_updated_at: String,
    #[serde(rename = "policyArn")]
    #[serde(default)]
    pub policy_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAutomatedReasoningPolicyBuildWorkflowResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAutomatedReasoningPolicyRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    #[serde(rename = "policyArn")]
    #[serde(default)]
    pub policy_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAutomatedReasoningPolicyResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAutomatedReasoningPolicyTestCaseRequest {
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    pub last_updated_at: String,
    #[serde(rename = "policyArn")]
    #[serde(default)]
    pub policy_arn: String,
    #[serde(rename = "testCaseId")]
    #[serde(default)]
    pub test_case_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAutomatedReasoningPolicyTestCaseResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCustomModelDeploymentRequest {
    #[serde(rename = "customModelDeploymentIdentifier")]
    #[serde(default)]
    pub custom_model_deployment_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCustomModelDeploymentResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCustomModelRequest {
    #[serde(rename = "modelIdentifier")]
    #[serde(default)]
    pub model_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCustomModelResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteEnforcedGuardrailConfigurationRequest {
    #[serde(rename = "configId")]
    #[serde(default)]
    pub config_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteEnforcedGuardrailConfigurationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFoundationModelAgreementRequest {
    #[serde(rename = "modelId")]
    #[serde(default)]
    pub model_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFoundationModelAgreementResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteGuardrailRequest {
    #[serde(rename = "guardrailIdentifier")]
    #[serde(default)]
    pub guardrail_identifier: String,
    #[serde(rename = "guardrailVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guardrail_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteGuardrailResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteImportedModelRequest {
    #[serde(rename = "modelIdentifier")]
    #[serde(default)]
    pub model_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteImportedModelResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteInferenceProfileRequest {
    #[serde(rename = "inferenceProfileIdentifier")]
    #[serde(default)]
    pub inference_profile_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteInferenceProfileResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteMarketplaceModelEndpointRequest {
    #[serde(rename = "endpointArn")]
    #[serde(default)]
    pub endpoint_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteMarketplaceModelEndpointResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteModelInvocationLoggingConfigurationRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteModelInvocationLoggingConfigurationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePromptRouterRequest {
    #[serde(rename = "promptRouterArn")]
    #[serde(default)]
    pub prompt_router_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePromptRouterResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteProvisionedModelThroughputRequest {
    #[serde(rename = "provisionedModelId")]
    #[serde(default)]
    pub provisioned_model_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteProvisionedModelThroughputResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteResourcePolicyRequest {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteResourcePolicyResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeregisterMarketplaceModelEndpointRequest {
    #[serde(rename = "endpointArn")]
    #[serde(default)]
    pub endpoint_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeregisterMarketplaceModelEndpointResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExportAutomatedReasoningPolicyVersionRequest {
    #[serde(rename = "policyArn")]
    #[serde(default)]
    pub policy_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExportAutomatedReasoningPolicyVersionResponse {
    #[serde(rename = "policyDefinition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_definition: Option<AutomatedReasoningPolicyDefinition>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAutomatedReasoningPolicyAnnotationsRequest {
    #[serde(rename = "buildWorkflowId")]
    #[serde(default)]
    pub build_workflow_id: String,
    #[serde(rename = "policyArn")]
    #[serde(default)]
    pub policy_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAutomatedReasoningPolicyAnnotationsResponse {
    #[serde(rename = "annotationSetHash")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation_set_hash: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Vec<AutomatedReasoningPolicyAnnotation>>,
    #[serde(rename = "buildWorkflowId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_workflow_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "policyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_arn: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningPolicyAnnotation {
    #[serde(rename = "addRule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_rule: Option<AutomatedReasoningPolicyAddRuleAnnotation>,
    #[serde(rename = "addRuleFromNaturalLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_rule_from_natural_language:
        Option<AutomatedReasoningPolicyAddRuleFromNaturalLanguageAnnotation>,
    #[serde(rename = "addType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_type: Option<AutomatedReasoningPolicyAddTypeAnnotation>,
    #[serde(rename = "addVariable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_variable: Option<AutomatedReasoningPolicyAddVariableAnnotation>,
    #[serde(rename = "deleteRule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_rule: Option<AutomatedReasoningPolicyDeleteRuleAnnotation>,
    #[serde(rename = "deleteType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_type: Option<AutomatedReasoningPolicyDeleteTypeAnnotation>,
    #[serde(rename = "deleteVariable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_variable: Option<AutomatedReasoningPolicyDeleteVariableAnnotation>,
    #[serde(rename = "ingestContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingest_content: Option<AutomatedReasoningPolicyIngestContentAnnotation>,
    #[serde(rename = "updateFromRulesFeedback")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_from_rules_feedback:
        Option<AutomatedReasoningPolicyUpdateFromRuleFeedbackAnnotation>,
    #[serde(rename = "updateFromScenarioFeedback")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_from_scenario_feedback:
        Option<AutomatedReasoningPolicyUpdateFromScenarioFeedbackAnnotation>,
    #[serde(rename = "updateRule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_rule: Option<AutomatedReasoningPolicyUpdateRuleAnnotation>,
    #[serde(rename = "updateType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_type: Option<AutomatedReasoningPolicyUpdateTypeAnnotation>,
    #[serde(rename = "updateVariable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_variable: Option<AutomatedReasoningPolicyUpdateVariableAnnotation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningPolicyAddRuleAnnotation {
    #[serde(default)]
    pub expression: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningPolicyAddRuleFromNaturalLanguageAnnotation {
    #[serde(rename = "naturalLanguage")]
    #[serde(default)]
    pub natural_language: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningPolicyAddTypeAnnotation {
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub values: Vec<AutomatedReasoningPolicyDefinitionTypeValue>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningPolicyAddVariableAnnotation {
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningPolicyDeleteRuleAnnotation {
    #[serde(rename = "ruleId")]
    #[serde(default)]
    pub rule_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningPolicyDeleteTypeAnnotation {
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningPolicyDeleteVariableAnnotation {
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningPolicyIngestContentAnnotation {
    #[serde(default)]
    pub content: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningPolicyUpdateFromRuleFeedbackAnnotation {
    #[serde(default)]
    pub feedback: String,
    #[serde(rename = "ruleIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningPolicyUpdateFromScenarioFeedbackAnnotation {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback: Option<String>,
    #[serde(rename = "ruleIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_ids: Option<Vec<String>>,
    #[serde(rename = "scenarioExpression")]
    #[serde(default)]
    pub scenario_expression: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningPolicyUpdateRuleAnnotation {
    #[serde(default)]
    pub expression: String,
    #[serde(rename = "ruleId")]
    #[serde(default)]
    pub rule_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningPolicyUpdateTypeAnnotation {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "newName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_name: Option<String>,
    #[serde(default)]
    pub values: Vec<AutomatedReasoningPolicyTypeValueAnnotation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningPolicyTypeValueAnnotation {
    #[serde(rename = "addTypeValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_type_value: Option<AutomatedReasoningPolicyAddTypeValue>,
    #[serde(rename = "deleteTypeValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_type_value: Option<AutomatedReasoningPolicyDeleteTypeValue>,
    #[serde(rename = "updateTypeValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_type_value: Option<AutomatedReasoningPolicyUpdateTypeValue>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningPolicyAddTypeValue {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningPolicyDeleteTypeValue {
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningPolicyUpdateTypeValue {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "newValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_value: Option<String>,
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningPolicyUpdateVariableAnnotation {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "newName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAutomatedReasoningPolicyBuildWorkflowRequest {
    #[serde(rename = "buildWorkflowId")]
    #[serde(default)]
    pub build_workflow_id: String,
    #[serde(rename = "policyArn")]
    #[serde(default)]
    pub policy_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAutomatedReasoningPolicyBuildWorkflowResponse {
    #[serde(rename = "buildWorkflowId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_workflow_id: Option<String>,
    #[serde(rename = "buildWorkflowType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_workflow_type: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "documentContentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_content_type: Option<String>,
    #[serde(rename = "documentDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_description: Option<String>,
    #[serde(rename = "documentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_name: Option<String>,
    #[serde(rename = "policyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAutomatedReasoningPolicyBuildWorkflowResultAssetsRequest {
    #[serde(rename = "assetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<String>,
    #[serde(rename = "assetType")]
    #[serde(default)]
    pub asset_type: String,
    #[serde(rename = "buildWorkflowId")]
    #[serde(default)]
    pub build_workflow_id: String,
    #[serde(rename = "policyArn")]
    #[serde(default)]
    pub policy_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAutomatedReasoningPolicyBuildWorkflowResultAssetsResponse {
    #[serde(rename = "buildWorkflowAssets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_workflow_assets: Option<AutomatedReasoningPolicyBuildResultAssets>,
    #[serde(rename = "buildWorkflowId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_workflow_id: Option<String>,
    #[serde(rename = "policyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningPolicyBuildResultAssets {
    #[serde(rename = "assetManifest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_manifest: Option<AutomatedReasoningPolicyBuildResultAssetManifest>,
    #[serde(rename = "buildLog")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_log: Option<AutomatedReasoningPolicyBuildLog>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<AutomatedReasoningPolicySourceDocument>,
    #[serde(rename = "fidelityReport")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fidelity_report: Option<AutomatedReasoningPolicyFidelityReport>,
    #[serde(rename = "generatedTestCases")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_test_cases: Option<AutomatedReasoningPolicyGeneratedTestCases>,
    #[serde(rename = "policyDefinition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_definition: Option<AutomatedReasoningPolicyDefinition>,
    #[serde(rename = "policyScenarios")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_scenarios: Option<AutomatedReasoningPolicyScenarios>,
    #[serde(rename = "qualityReport")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_report: Option<AutomatedReasoningPolicyDefinitionQualityReport>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningPolicyBuildResultAssetManifest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<AutomatedReasoningPolicyBuildResultAssetManifestEntry>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningPolicyBuildResultAssetManifestEntry {
    #[serde(rename = "assetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<String>,
    #[serde(rename = "assetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_name: Option<String>,
    #[serde(rename = "assetType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningPolicyBuildLog {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<AutomatedReasoningPolicyBuildLogEntry>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningPolicyBuildLogEntry {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation: Option<AutomatedReasoningPolicyAnnotation>,
    #[serde(rename = "buildSteps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_steps: Option<Vec<AutomatedReasoningPolicyBuildStep>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningPolicyBuildStep {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<AutomatedReasoningPolicyBuildStepContext>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<AutomatedReasoningPolicyBuildStepMessage>>,
    #[serde(rename = "priorElement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prior_element: Option<AutomatedReasoningPolicyDefinitionElement>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningPolicyBuildStepContext {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mutation: Option<AutomatedReasoningPolicyMutation>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub planning: Option<AutomatedReasoningPolicyPlanning>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningPolicyMutation {
    #[serde(rename = "addRule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_rule: Option<AutomatedReasoningPolicyAddRuleMutation>,
    #[serde(rename = "addType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_type: Option<AutomatedReasoningPolicyAddTypeMutation>,
    #[serde(rename = "addVariable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_variable: Option<AutomatedReasoningPolicyAddVariableMutation>,
    #[serde(rename = "deleteRule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_rule: Option<AutomatedReasoningPolicyDeleteRuleMutation>,
    #[serde(rename = "deleteType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_type: Option<AutomatedReasoningPolicyDeleteTypeMutation>,
    #[serde(rename = "deleteVariable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_variable: Option<AutomatedReasoningPolicyDeleteVariableMutation>,
    #[serde(rename = "updateRule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_rule: Option<AutomatedReasoningPolicyUpdateRuleMutation>,
    #[serde(rename = "updateType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_type: Option<AutomatedReasoningPolicyUpdateTypeMutation>,
    #[serde(rename = "updateVariable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_variable: Option<AutomatedReasoningPolicyUpdateVariableMutation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningPolicyAddRuleMutation {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule: Option<AutomatedReasoningPolicyDefinitionRule>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningPolicyAddTypeMutation {
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<AutomatedReasoningPolicyDefinitionType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningPolicyAddVariableMutation {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variable: Option<AutomatedReasoningPolicyDefinitionVariable>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningPolicyDeleteRuleMutation {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningPolicyDeleteTypeMutation {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningPolicyDeleteVariableMutation {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningPolicyUpdateRuleMutation {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule: Option<AutomatedReasoningPolicyDefinitionRule>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningPolicyUpdateTypeMutation {
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<AutomatedReasoningPolicyDefinitionType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningPolicyUpdateVariableMutation {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variable: Option<AutomatedReasoningPolicyDefinitionVariable>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningPolicyPlanning {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningPolicyBuildStepMessage {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "messageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningPolicyDefinitionElement {
    #[serde(rename = "policyDefinitionRule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_definition_rule: Option<AutomatedReasoningPolicyDefinitionRule>,
    #[serde(rename = "policyDefinitionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_definition_type: Option<AutomatedReasoningPolicyDefinitionType>,
    #[serde(rename = "policyDefinitionVariable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_definition_variable: Option<AutomatedReasoningPolicyDefinitionVariable>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningPolicySourceDocument {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<String>,
    #[serde(rename = "documentContentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_content_type: Option<String>,
    #[serde(rename = "documentDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_description: Option<String>,
    #[serde(rename = "documentHash")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_hash: Option<String>,
    #[serde(rename = "documentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningPolicyFidelityReport {
    #[serde(rename = "accuracyScore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accuracy_score: Option<f64>,
    #[serde(rename = "coverageScore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coverage_score: Option<f64>,
    #[serde(rename = "documentSources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_sources: Option<Vec<AutomatedReasoningPolicyReportSourceDocument>>,
    #[serde(rename = "ruleReports")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_reports: Option<std::collections::HashMap<String, AutomatedReasoningPolicyRuleReport>>,
    #[serde(rename = "variableReports")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variable_reports:
        Option<std::collections::HashMap<String, AutomatedReasoningPolicyVariableReport>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningPolicyReportSourceDocument {
    #[serde(rename = "atomicStatements")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub atomic_statements: Option<Vec<AutomatedReasoningPolicyAtomicStatement>>,
    #[serde(rename = "documentContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_content: Option<Vec<AutomatedReasoningPolicyAnnotatedChunk>>,
    #[serde(rename = "documentHash")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_hash: Option<String>,
    #[serde(rename = "documentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
    #[serde(rename = "documentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningPolicyAtomicStatement {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<AutomatedReasoningPolicyStatementLocation>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningPolicyStatementLocation {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines: Option<Vec<i32>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningPolicyAnnotatedChunk {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<AutomatedReasoningPolicyAnnotatedContent>>,
    #[serde(rename = "pageNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningPolicyAnnotatedContent {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<AutomatedReasoningPolicyAnnotatedLine>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningPolicyAnnotatedLine {
    #[serde(rename = "lineNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_number: Option<i32>,
    #[serde(rename = "lineText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_text: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningPolicyRuleReport {
    #[serde(rename = "accuracyJustification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accuracy_justification: Option<String>,
    #[serde(rename = "accuracyScore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accuracy_score: Option<f64>,
    #[serde(rename = "groundingJustifications")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grounding_justifications: Option<Vec<String>>,
    #[serde(rename = "groundingStatements")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grounding_statements: Option<Vec<AutomatedReasoningPolicyStatementReference>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningPolicyStatementReference {
    #[serde(rename = "documentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
    #[serde(rename = "statementId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningPolicyVariableReport {
    #[serde(rename = "accuracyJustification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accuracy_justification: Option<String>,
    #[serde(rename = "accuracyScore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accuracy_score: Option<f64>,
    #[serde(rename = "groundingJustifications")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grounding_justifications: Option<Vec<String>>,
    #[serde(rename = "groundingStatements")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grounding_statements: Option<Vec<AutomatedReasoningPolicyStatementReference>>,
    #[serde(rename = "policyVariable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_variable: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningPolicyGeneratedTestCases {
    #[serde(rename = "generatedTestCases")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_test_cases: Option<Vec<AutomatedReasoningPolicyGeneratedTestCase>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningPolicyGeneratedTestCase {
    #[serde(rename = "expectedAggregatedFindingsResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_aggregated_findings_result: Option<String>,
    #[serde(rename = "guardContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guard_content: Option<String>,
    #[serde(rename = "queryContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_content: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningPolicyScenarios {
    #[serde(rename = "policyScenarios")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_scenarios: Option<Vec<AutomatedReasoningPolicyScenario>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningPolicyScenario {
    #[serde(rename = "alternateExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternate_expression: Option<String>,
    #[serde(rename = "expectedResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_result: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(rename = "ruleIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningPolicyDefinitionQualityReport {
    #[serde(rename = "conflictingRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflicting_rules: Option<Vec<String>>,
    #[serde(rename = "disjointRuleSets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disjoint_rule_sets: Option<Vec<AutomatedReasoningPolicyDisjointRuleSet>>,
    #[serde(rename = "ruleCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_count: Option<i32>,
    #[serde(rename = "typeCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_count: Option<i32>,
    #[serde(rename = "unusedTypeValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unused_type_values: Option<Vec<AutomatedReasoningPolicyDefinitionTypeValuePair>>,
    #[serde(rename = "unusedTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unused_types: Option<Vec<String>>,
    #[serde(rename = "unusedVariables")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unused_variables: Option<Vec<String>>,
    #[serde(rename = "variableCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variable_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningPolicyDisjointRuleSet {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningPolicyDefinitionTypeValuePair {
    #[serde(rename = "typeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
    #[serde(rename = "valueName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAutomatedReasoningPolicyNextScenarioRequest {
    #[serde(rename = "buildWorkflowId")]
    #[serde(default)]
    pub build_workflow_id: String,
    #[serde(rename = "policyArn")]
    #[serde(default)]
    pub policy_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAutomatedReasoningPolicyNextScenarioResponse {
    #[serde(rename = "policyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<AutomatedReasoningPolicyScenario>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAutomatedReasoningPolicyRequest {
    #[serde(rename = "policyArn")]
    #[serde(default)]
    pub policy_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAutomatedReasoningPolicyResponse {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "definitionHash")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition_hash: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "kmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "policyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_arn: Option<String>,
    #[serde(rename = "policyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAutomatedReasoningPolicyTestCaseRequest {
    #[serde(rename = "policyArn")]
    #[serde(default)]
    pub policy_arn: String,
    #[serde(rename = "testCaseId")]
    #[serde(default)]
    pub test_case_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAutomatedReasoningPolicyTestCaseResponse {
    #[serde(rename = "policyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_arn: Option<String>,
    #[serde(rename = "testCase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_case: Option<AutomatedReasoningPolicyTestCase>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningPolicyTestCase {
    #[serde(rename = "confidenceThreshold")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence_threshold: Option<f64>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "expectedAggregatedFindingsResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_aggregated_findings_result: Option<String>,
    #[serde(rename = "guardContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guard_content: Option<String>,
    #[serde(rename = "queryContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_content: Option<String>,
    #[serde(rename = "testCaseId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_case_id: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAutomatedReasoningPolicyTestResultRequest {
    #[serde(rename = "buildWorkflowId")]
    #[serde(default)]
    pub build_workflow_id: String,
    #[serde(rename = "policyArn")]
    #[serde(default)]
    pub policy_arn: String,
    #[serde(rename = "testCaseId")]
    #[serde(default)]
    pub test_case_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAutomatedReasoningPolicyTestResultResponse {
    #[serde(rename = "testResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_result: Option<AutomatedReasoningPolicyTestResult>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningPolicyTestResult {
    #[serde(rename = "aggregatedTestFindingsResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregated_test_findings_result: Option<String>,
    #[serde(rename = "policyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_arn: Option<String>,
    #[serde(rename = "testCase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_case: Option<AutomatedReasoningPolicyTestCase>,
    #[serde(rename = "testFindings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_findings: Option<Vec<AutomatedReasoningCheckFinding>>,
    #[serde(rename = "testRunResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_run_result: Option<String>,
    #[serde(rename = "testRunStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_run_status: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningCheckFinding {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impossible: Option<AutomatedReasoningCheckImpossibleFinding>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalid: Option<AutomatedReasoningCheckInvalidFinding>,
    #[serde(rename = "noTranslations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_translations: Option<AutomatedReasoningCheckNoTranslationsFinding>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub satisfiable: Option<AutomatedReasoningCheckSatisfiableFinding>,
    #[serde(rename = "tooComplex")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub too_complex: Option<AutomatedReasoningCheckTooComplexFinding>,
    #[serde(rename = "translationAmbiguous")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translation_ambiguous: Option<AutomatedReasoningCheckTranslationAmbiguousFinding>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid: Option<AutomatedReasoningCheckValidFinding>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningCheckImpossibleFinding {
    #[serde(rename = "contradictingRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contradicting_rules: Option<Vec<AutomatedReasoningCheckRule>>,
    #[serde(rename = "logicWarning")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logic_warning: Option<AutomatedReasoningCheckLogicWarning>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translation: Option<AutomatedReasoningCheckTranslation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningCheckRule {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "policyVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_version_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningCheckLogicWarning {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claims: Option<Vec<AutomatedReasoningLogicStatement>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premises: Option<Vec<AutomatedReasoningLogicStatement>>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningLogicStatement {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logic: Option<String>,
    #[serde(rename = "naturalLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub natural_language: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningCheckTranslation {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claims: Option<Vec<AutomatedReasoningLogicStatement>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premises: Option<Vec<AutomatedReasoningLogicStatement>>,
    #[serde(rename = "untranslatedClaims")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub untranslated_claims: Option<Vec<AutomatedReasoningCheckInputTextReference>>,
    #[serde(rename = "untranslatedPremises")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub untranslated_premises: Option<Vec<AutomatedReasoningCheckInputTextReference>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningCheckInputTextReference {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningCheckInvalidFinding {
    #[serde(rename = "contradictingRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contradicting_rules: Option<Vec<AutomatedReasoningCheckRule>>,
    #[serde(rename = "logicWarning")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logic_warning: Option<AutomatedReasoningCheckLogicWarning>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translation: Option<AutomatedReasoningCheckTranslation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningCheckNoTranslationsFinding {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningCheckSatisfiableFinding {
    #[serde(rename = "claimsFalseScenario")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claims_false_scenario: Option<AutomatedReasoningCheckScenario>,
    #[serde(rename = "claimsTrueScenario")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claims_true_scenario: Option<AutomatedReasoningCheckScenario>,
    #[serde(rename = "logicWarning")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logic_warning: Option<AutomatedReasoningCheckLogicWarning>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translation: Option<AutomatedReasoningCheckTranslation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningCheckScenario {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statements: Option<Vec<AutomatedReasoningLogicStatement>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningCheckTooComplexFinding {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningCheckTranslationAmbiguousFinding {
    #[serde(rename = "differenceScenarios")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub difference_scenarios: Option<Vec<AutomatedReasoningCheckScenario>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<AutomatedReasoningCheckTranslationOption>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningCheckTranslationOption {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translations: Option<Vec<AutomatedReasoningCheckTranslation>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningCheckValidFinding {
    #[serde(rename = "claimsTrueScenario")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claims_true_scenario: Option<AutomatedReasoningCheckScenario>,
    #[serde(rename = "logicWarning")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logic_warning: Option<AutomatedReasoningCheckLogicWarning>,
    #[serde(rename = "supportingRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supporting_rules: Option<Vec<AutomatedReasoningCheckRule>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translation: Option<AutomatedReasoningCheckTranslation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCustomModelDeploymentRequest {
    #[serde(rename = "customModelDeploymentIdentifier")]
    #[serde(default)]
    pub custom_model_deployment_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCustomModelDeploymentResponse {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "customModelDeploymentArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_model_deployment_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "failureMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<String>,
    #[serde(rename = "modelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_arn: Option<String>,
    #[serde(rename = "modelDeploymentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_deployment_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "updateDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_details: Option<CustomModelDeploymentUpdateDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomModelDeploymentUpdateDetails {
    #[serde(rename = "modelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_arn: Option<String>,
    #[serde(rename = "updateStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCustomModelRequest {
    #[serde(rename = "modelIdentifier")]
    #[serde(default)]
    pub model_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCustomModelResponse {
    #[serde(rename = "baseModelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_model_arn: Option<String>,
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(rename = "customizationConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customization_config: Option<CustomizationConfig>,
    #[serde(rename = "customizationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customization_type: Option<String>,
    #[serde(rename = "failureMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
    #[serde(rename = "hyperParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hyper_parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "jobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_arn: Option<String>,
    #[serde(rename = "jobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "modelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_arn: Option<String>,
    #[serde(rename = "modelKmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_kms_key_arn: Option<String>,
    #[serde(rename = "modelName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_name: Option<String>,
    #[serde(rename = "modelStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_status: Option<String>,
    #[serde(rename = "outputDataConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_data_config: Option<OutputDataConfig>,
    #[serde(rename = "trainingDataConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_data_config: Option<TrainingDataConfig>,
    #[serde(rename = "trainingMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_metrics: Option<TrainingMetrics>,
    #[serde(rename = "validationDataConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_data_config: Option<ValidationDataConfig>,
    #[serde(rename = "validationMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_metrics: Option<Vec<ValidatorMetric>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TrainingMetrics {
    #[serde(rename = "trainingLoss")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_loss: Option<f32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ValidatorMetric {
    #[serde(rename = "validationLoss")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_loss: Option<f32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetEvaluationJobRequest {
    #[serde(rename = "jobIdentifier")]
    #[serde(default)]
    pub job_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetEvaluationJobResponse {
    #[serde(rename = "applicationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_type: Option<String>,
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(rename = "customerEncryptionKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_encryption_key_id: Option<String>,
    #[serde(rename = "evaluationConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_config: Option<EvaluationConfig>,
    #[serde(rename = "failureMessages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_messages: Option<Vec<String>>,
    #[serde(rename = "inferenceConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_config: Option<EvaluationInferenceConfig>,
    #[serde(rename = "jobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_arn: Option<String>,
    #[serde(rename = "jobDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_description: Option<String>,
    #[serde(rename = "jobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "jobType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_type: Option<String>,
    #[serde(rename = "lastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<String>,
    #[serde(rename = "outputDataConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_data_config: Option<EvaluationOutputDataConfig>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFoundationModelAvailabilityRequest {
    #[serde(rename = "modelId")]
    #[serde(default)]
    pub model_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFoundationModelAvailabilityResponse {
    #[serde(rename = "agreementAvailability")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agreement_availability: Option<AgreementAvailability>,
    #[serde(rename = "authorizationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_status: Option<String>,
    #[serde(rename = "entitlementAvailability")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entitlement_availability: Option<String>,
    #[serde(rename = "modelId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
    #[serde(rename = "regionAvailability")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_availability: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AgreementAvailability {
    #[serde(rename = "errorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFoundationModelRequest {
    #[serde(rename = "modelIdentifier")]
    #[serde(default)]
    pub model_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFoundationModelResponse {
    #[serde(rename = "modelDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_details: Option<FoundationModelDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FoundationModelDetails {
    #[serde(rename = "customizationsSupported")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customizations_supported: Option<Vec<String>>,
    #[serde(rename = "inferenceTypesSupported")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_types_supported: Option<Vec<String>>,
    #[serde(rename = "inputModalities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_modalities: Option<Vec<String>>,
    #[serde(rename = "modelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_arn: Option<String>,
    #[serde(rename = "modelId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
    #[serde(rename = "modelLifecycle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_lifecycle: Option<FoundationModelLifecycle>,
    #[serde(rename = "modelName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_name: Option<String>,
    #[serde(rename = "outputModalities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_modalities: Option<Vec<String>>,
    #[serde(rename = "providerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,
    #[serde(rename = "responseStreamingSupported")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_streaming_supported: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FoundationModelLifecycle {
    #[serde(rename = "endOfLifeTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_of_life_time: Option<String>,
    #[serde(rename = "legacyTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legacy_time: Option<String>,
    #[serde(rename = "publicExtendedAccessTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_extended_access_time: Option<String>,
    #[serde(rename = "startOfLifeTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_of_life_time: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetGuardrailRequest {
    #[serde(rename = "guardrailIdentifier")]
    #[serde(default)]
    pub guardrail_identifier: String,
    #[serde(rename = "guardrailVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guardrail_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetGuardrailResponse {
    #[serde(rename = "automatedReasoningPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automated_reasoning_policy: Option<GuardrailAutomatedReasoningPolicy>,
    #[serde(rename = "blockedInputMessaging")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked_input_messaging: Option<String>,
    #[serde(rename = "blockedOutputsMessaging")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked_outputs_messaging: Option<String>,
    #[serde(rename = "contentPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_policy: Option<GuardrailContentPolicy>,
    #[serde(rename = "contextualGroundingPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contextual_grounding_policy: Option<GuardrailContextualGroundingPolicy>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "crossRegionDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_region_details: Option<GuardrailCrossRegionDetails>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "failureRecommendations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_recommendations: Option<Vec<String>>,
    #[serde(rename = "guardrailArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guardrail_arn: Option<String>,
    #[serde(rename = "guardrailId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guardrail_id: Option<String>,
    #[serde(rename = "kmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "sensitiveInformationPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensitive_information_policy: Option<GuardrailSensitiveInformationPolicy>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusReasons")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reasons: Option<Vec<String>>,
    #[serde(rename = "topicPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_policy: Option<GuardrailTopicPolicy>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "wordPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub word_policy: Option<GuardrailWordPolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GuardrailAutomatedReasoningPolicy {
    #[serde(rename = "confidenceThreshold")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence_threshold: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GuardrailContentPolicy {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<GuardrailContentFilter>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<GuardrailContentFiltersTier>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GuardrailContentFilter {
    #[serde(rename = "inputAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_action: Option<String>,
    #[serde(rename = "inputEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_enabled: Option<bool>,
    #[serde(rename = "inputModalities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_modalities: Option<Vec<String>>,
    #[serde(rename = "inputStrength")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_strength: Option<String>,
    #[serde(rename = "outputAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_action: Option<String>,
    #[serde(rename = "outputEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_enabled: Option<bool>,
    #[serde(rename = "outputModalities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_modalities: Option<Vec<String>>,
    #[serde(rename = "outputStrength")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_strength: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GuardrailContentFiltersTier {
    #[serde(rename = "tierName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GuardrailContextualGroundingPolicy {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<GuardrailContextualGroundingFilter>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GuardrailContextualGroundingFilter {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<f64>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GuardrailCrossRegionDetails {
    #[serde(rename = "guardrailProfileArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guardrail_profile_arn: Option<String>,
    #[serde(rename = "guardrailProfileId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guardrail_profile_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GuardrailSensitiveInformationPolicy {
    #[serde(rename = "piiEntities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pii_entities: Option<Vec<GuardrailPiiEntity>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regexes: Option<Vec<GuardrailRegex>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GuardrailPiiEntity {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "inputAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_action: Option<String>,
    #[serde(rename = "inputEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_enabled: Option<bool>,
    #[serde(rename = "outputAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_action: Option<String>,
    #[serde(rename = "outputEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_enabled: Option<bool>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GuardrailRegex {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "inputAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_action: Option<String>,
    #[serde(rename = "inputEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_enabled: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "outputAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_action: Option<String>,
    #[serde(rename = "outputEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_enabled: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GuardrailTopicPolicy {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<GuardrailTopicsTier>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<GuardrailTopic>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GuardrailTopicsTier {
    #[serde(rename = "tierName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GuardrailTopic {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub examples: Option<Vec<String>>,
    #[serde(rename = "inputAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_action: Option<String>,
    #[serde(rename = "inputEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_enabled: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "outputAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_action: Option<String>,
    #[serde(rename = "outputEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_enabled: Option<bool>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GuardrailWordPolicy {
    #[serde(rename = "managedWordLists")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_word_lists: Option<Vec<GuardrailManagedWords>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub words: Option<Vec<GuardrailWord>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GuardrailManagedWords {
    #[serde(rename = "inputAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_action: Option<String>,
    #[serde(rename = "inputEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_enabled: Option<bool>,
    #[serde(rename = "outputAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_action: Option<String>,
    #[serde(rename = "outputEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_enabled: Option<bool>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GuardrailWord {
    #[serde(rename = "inputAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_action: Option<String>,
    #[serde(rename = "inputEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_enabled: Option<bool>,
    #[serde(rename = "outputAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_action: Option<String>,
    #[serde(rename = "outputEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_enabled: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetImportedModelRequest {
    #[serde(rename = "modelIdentifier")]
    #[serde(default)]
    pub model_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetImportedModelResponse {
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(rename = "customModelUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_model_units: Option<CustomModelUnits>,
    #[serde(rename = "instructSupported")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instruct_supported: Option<bool>,
    #[serde(rename = "jobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_arn: Option<String>,
    #[serde(rename = "jobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "modelArchitecture")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_architecture: Option<String>,
    #[serde(rename = "modelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_arn: Option<String>,
    #[serde(rename = "modelDataSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_data_source: Option<ModelDataSource>,
    #[serde(rename = "modelKmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_kms_key_arn: Option<String>,
    #[serde(rename = "modelName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomModelUnits {
    #[serde(rename = "customModelUnitsPerModelCopy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_model_units_per_model_copy: Option<i32>,
    #[serde(rename = "customModelUnitsVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_model_units_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetInferenceProfileRequest {
    #[serde(rename = "inferenceProfileIdentifier")]
    #[serde(default)]
    pub inference_profile_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetInferenceProfileResponse {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "inferenceProfileArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_profile_arn: Option<String>,
    #[serde(rename = "inferenceProfileId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_profile_id: Option<String>,
    #[serde(rename = "inferenceProfileName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_profile_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub models: Option<Vec<InferenceProfileModel>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InferenceProfileModel {
    #[serde(rename = "modelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMarketplaceModelEndpointRequest {
    #[serde(rename = "endpointArn")]
    #[serde(default)]
    pub endpoint_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMarketplaceModelEndpointResponse {
    #[serde(rename = "marketplaceModelEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketplace_model_endpoint: Option<MarketplaceModelEndpoint>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetModelCopyJobRequest {
    #[serde(rename = "jobArn")]
    #[serde(default)]
    pub job_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetModelCopyJobResponse {
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(rename = "failureMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
    #[serde(rename = "jobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_arn: Option<String>,
    #[serde(rename = "sourceAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_account_id: Option<String>,
    #[serde(rename = "sourceModelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_model_arn: Option<String>,
    #[serde(rename = "sourceModelName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_model_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "targetModelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_model_arn: Option<String>,
    #[serde(rename = "targetModelKmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_model_kms_key_arn: Option<String>,
    #[serde(rename = "targetModelName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_model_name: Option<String>,
    #[serde(rename = "targetModelTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_model_tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetModelCustomizationJobRequest {
    #[serde(rename = "jobIdentifier")]
    #[serde(default)]
    pub job_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetModelCustomizationJobResponse {
    #[serde(rename = "baseModelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_model_arn: Option<String>,
    #[serde(rename = "clientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(rename = "customizationConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customization_config: Option<CustomizationConfig>,
    #[serde(rename = "customizationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customization_type: Option<String>,
    #[serde(rename = "endTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(rename = "failureMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
    #[serde(rename = "hyperParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hyper_parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "jobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_arn: Option<String>,
    #[serde(rename = "jobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "lastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<String>,
    #[serde(rename = "outputDataConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_data_config: Option<OutputDataConfig>,
    #[serde(rename = "outputModelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_model_arn: Option<String>,
    #[serde(rename = "outputModelKmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_model_kms_key_arn: Option<String>,
    #[serde(rename = "outputModelName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_model_name: Option<String>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_details: Option<StatusDetails>,
    #[serde(rename = "trainingDataConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_data_config: Option<TrainingDataConfig>,
    #[serde(rename = "trainingMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_metrics: Option<TrainingMetrics>,
    #[serde(rename = "validationDataConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_data_config: Option<ValidationDataConfig>,
    #[serde(rename = "validationMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_metrics: Option<Vec<ValidatorMetric>>,
    #[serde(rename = "vpcConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StatusDetails {
    #[serde(rename = "dataProcessingDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_processing_details: Option<DataProcessingDetails>,
    #[serde(rename = "trainingDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_details: Option<TrainingDetails>,
    #[serde(rename = "validationDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_details: Option<ValidationDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataProcessingDetails {
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(rename = "lastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TrainingDetails {
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(rename = "lastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ValidationDetails {
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(rename = "lastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetModelImportJobRequest {
    #[serde(rename = "jobIdentifier")]
    #[serde(default)]
    pub job_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetModelImportJobResponse {
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(rename = "endTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(rename = "failureMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
    #[serde(rename = "importedModelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imported_model_arn: Option<String>,
    #[serde(rename = "importedModelKmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imported_model_kms_key_arn: Option<String>,
    #[serde(rename = "importedModelName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imported_model_name: Option<String>,
    #[serde(rename = "jobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_arn: Option<String>,
    #[serde(rename = "jobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "lastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<String>,
    #[serde(rename = "modelDataSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_data_source: Option<ModelDataSource>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "vpcConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetModelInvocationJobRequest {
    #[serde(rename = "jobIdentifier")]
    #[serde(default)]
    pub job_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetModelInvocationJobResponse {
    #[serde(rename = "clientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "endTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(rename = "errorRecordCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_record_count: Option<i64>,
    #[serde(rename = "inputDataConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_data_config: Option<ModelInvocationJobInputDataConfig>,
    #[serde(rename = "jobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_arn: Option<String>,
    #[serde(rename = "jobExpirationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_expiration_time: Option<String>,
    #[serde(rename = "jobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "lastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "modelId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
    #[serde(rename = "modelInvocationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_invocation_type: Option<String>,
    #[serde(rename = "outputDataConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_data_config: Option<ModelInvocationJobOutputDataConfig>,
    #[serde(rename = "processedRecordCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processed_record_count: Option<i64>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "submitTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time: Option<String>,
    #[serde(rename = "successRecordCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_record_count: Option<i64>,
    #[serde(rename = "timeoutDurationInHours")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_duration_in_hours: Option<i32>,
    #[serde(rename = "totalRecordCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_record_count: Option<i64>,
    #[serde(rename = "vpcConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetModelInvocationLoggingConfigurationRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetModelInvocationLoggingConfigurationResponse {
    #[serde(rename = "loggingConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_config: Option<LoggingConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LoggingConfig {
    #[serde(rename = "audioDataDeliveryEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_data_delivery_enabled: Option<bool>,
    #[serde(rename = "cloudWatchConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_config: Option<CloudWatchConfig>,
    #[serde(rename = "embeddingDataDeliveryEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding_data_delivery_enabled: Option<bool>,
    #[serde(rename = "imageDataDeliveryEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_data_delivery_enabled: Option<bool>,
    #[serde(rename = "s3Config")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_config: Option<S3Config>,
    #[serde(rename = "textDataDeliveryEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_data_delivery_enabled: Option<bool>,
    #[serde(rename = "videoDataDeliveryEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_data_delivery_enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CloudWatchConfig {
    #[serde(rename = "largeDataDeliveryS3Config")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_data_delivery_s3_config: Option<S3Config>,
    #[serde(rename = "logGroupName")]
    #[serde(default)]
    pub log_group_name: String,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    pub role_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3Config {
    #[serde(rename = "bucketName")]
    #[serde(default)]
    pub bucket_name: String,
    #[serde(rename = "keyPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPromptRouterRequest {
    #[serde(rename = "promptRouterArn")]
    #[serde(default)]
    pub prompt_router_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPromptRouterResponse {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "fallbackModel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback_model: Option<PromptRouterTargetModel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub models: Option<Vec<PromptRouterTargetModel>>,
    #[serde(rename = "promptRouterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_router_arn: Option<String>,
    #[serde(rename = "promptRouterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_router_name: Option<String>,
    #[serde(rename = "routingCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_criteria: Option<RoutingCriteria>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetProvisionedModelThroughputRequest {
    #[serde(rename = "provisionedModelId")]
    #[serde(default)]
    pub provisioned_model_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetProvisionedModelThroughputResponse {
    #[serde(rename = "commitmentDuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment_duration: Option<String>,
    #[serde(rename = "commitmentExpirationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment_expiration_time: Option<String>,
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(rename = "desiredModelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_model_arn: Option<String>,
    #[serde(rename = "desiredModelUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_model_units: Option<i32>,
    #[serde(rename = "failureMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
    #[serde(rename = "foundationModelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foundation_model_arn: Option<String>,
    #[serde(rename = "lastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<String>,
    #[serde(rename = "modelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_arn: Option<String>,
    #[serde(rename = "modelUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_units: Option<i32>,
    #[serde(rename = "provisionedModelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_model_arn: Option<String>,
    #[serde(rename = "provisionedModelName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_model_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourcePolicyRequest {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourcePolicyResponse {
    #[serde(rename = "resourcePolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetUseCaseForModelAccessRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetUseCaseForModelAccessResponse {
    #[serde(rename = "formData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form_data: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAutomatedReasoningPoliciesRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "policyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAutomatedReasoningPoliciesResponse {
    #[serde(rename = "automatedReasoningPolicySummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automated_reasoning_policy_summaries: Option<Vec<AutomatedReasoningPolicySummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningPolicySummary {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "policyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_arn: Option<String>,
    #[serde(rename = "policyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAutomatedReasoningPolicyBuildWorkflowsRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "policyArn")]
    #[serde(default)]
    pub policy_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAutomatedReasoningPolicyBuildWorkflowsResponse {
    #[serde(rename = "automatedReasoningPolicyBuildWorkflowSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automated_reasoning_policy_build_workflow_summaries:
        Option<Vec<AutomatedReasoningPolicyBuildWorkflowSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningPolicyBuildWorkflowSummary {
    #[serde(rename = "buildWorkflowId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_workflow_id: Option<String>,
    #[serde(rename = "buildWorkflowType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_workflow_type: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "policyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAutomatedReasoningPolicyTestCasesRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "policyArn")]
    #[serde(default)]
    pub policy_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAutomatedReasoningPolicyTestCasesResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "testCases")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_cases: Option<Vec<AutomatedReasoningPolicyTestCase>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAutomatedReasoningPolicyTestResultsRequest {
    #[serde(rename = "buildWorkflowId")]
    #[serde(default)]
    pub build_workflow_id: String,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "policyArn")]
    #[serde(default)]
    pub policy_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAutomatedReasoningPolicyTestResultsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "testResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_results: Option<Vec<AutomatedReasoningPolicyTestResult>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCustomModelDeploymentsRequest {
    #[serde(rename = "createdAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<String>,
    #[serde(rename = "createdBefore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<String>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "modelArnEquals")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_arn_equals: Option<String>,
    #[serde(rename = "nameContains")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "sortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(rename = "sortOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    #[serde(rename = "statusEquals")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_equals: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCustomModelDeploymentsResponse {
    #[serde(rename = "modelDeploymentSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_deployment_summaries: Option<Vec<CustomModelDeploymentSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomModelDeploymentSummary {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "customModelDeploymentArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_model_deployment_arn: Option<String>,
    #[serde(rename = "customModelDeploymentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_model_deployment_name: Option<String>,
    #[serde(rename = "failureMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<String>,
    #[serde(rename = "modelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCustomModelsRequest {
    #[serde(rename = "baseModelArnEquals")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_model_arn_equals: Option<String>,
    #[serde(rename = "creationTimeAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_after: Option<String>,
    #[serde(rename = "creationTimeBefore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_before: Option<String>,
    #[serde(rename = "foundationModelArnEquals")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foundation_model_arn_equals: Option<String>,
    #[serde(rename = "isOwned")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_owned: Option<bool>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "modelStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_status: Option<String>,
    #[serde(rename = "nameContains")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "sortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(rename = "sortOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCustomModelsResponse {
    #[serde(rename = "modelSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_summaries: Option<Vec<CustomModelSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomModelSummary {
    #[serde(rename = "baseModelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_model_arn: Option<String>,
    #[serde(rename = "baseModelName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_model_name: Option<String>,
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(rename = "customizationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customization_type: Option<String>,
    #[serde(rename = "modelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_arn: Option<String>,
    #[serde(rename = "modelName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_name: Option<String>,
    #[serde(rename = "modelStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_status: Option<String>,
    #[serde(rename = "ownerAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_account_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListEnforcedGuardrailsConfigurationRequest {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListEnforcedGuardrailsConfigurationResponse {
    #[serde(rename = "guardrailsConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guardrails_config: Option<Vec<AccountEnforcedGuardrailOutputConfiguration>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccountEnforcedGuardrailOutputConfiguration {
    #[serde(rename = "configId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_id: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "createdBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "guardrailArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guardrail_arn: Option<String>,
    #[serde(rename = "guardrailId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guardrail_id: Option<String>,
    #[serde(rename = "guardrailVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guardrail_version: Option<String>,
    #[serde(rename = "inputTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_tags: Option<String>,
    #[serde(rename = "modelEnforcement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_enforcement: Option<ModelEnforcement>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(rename = "selectiveContentGuarding")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective_content_guarding: Option<SelectiveContentGuarding>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "updatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModelEnforcement {
    #[serde(rename = "excludedModels")]
    #[serde(default)]
    pub excluded_models: Vec<String>,
    #[serde(rename = "includedModels")]
    #[serde(default)]
    pub included_models: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SelectiveContentGuarding {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListEvaluationJobsRequest {
    #[serde(rename = "applicationTypeEquals")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_type_equals: Option<String>,
    #[serde(rename = "creationTimeAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_after: Option<String>,
    #[serde(rename = "creationTimeBefore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_before: Option<String>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nameContains")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "sortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(rename = "sortOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    #[serde(rename = "statusEquals")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_equals: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListEvaluationJobsResponse {
    #[serde(rename = "jobSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_summaries: Option<Vec<EvaluationSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationSummary {
    #[serde(rename = "applicationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_type: Option<String>,
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(rename = "customMetricsEvaluatorModelIdentifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_metrics_evaluator_model_identifiers: Option<Vec<String>>,
    #[serde(rename = "evaluationTaskTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_task_types: Option<Vec<String>>,
    #[serde(rename = "evaluatorModelIdentifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluator_model_identifiers: Option<Vec<String>>,
    #[serde(rename = "inferenceConfigSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_config_summary: Option<EvaluationInferenceConfigSummary>,
    #[serde(rename = "jobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_arn: Option<String>,
    #[serde(rename = "jobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "jobType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_type: Option<String>,
    #[serde(rename = "modelIdentifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_identifiers: Option<Vec<String>>,
    #[serde(rename = "ragIdentifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rag_identifiers: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationInferenceConfigSummary {
    #[serde(rename = "modelConfigSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_config_summary: Option<EvaluationModelConfigSummary>,
    #[serde(rename = "ragConfigSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rag_config_summary: Option<EvaluationRagConfigSummary>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationModelConfigSummary {
    #[serde(rename = "bedrockModelIdentifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bedrock_model_identifiers: Option<Vec<String>>,
    #[serde(rename = "precomputedInferenceSourceIdentifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precomputed_inference_source_identifiers: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationRagConfigSummary {
    #[serde(rename = "bedrockKnowledgeBaseIdentifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bedrock_knowledge_base_identifiers: Option<Vec<String>>,
    #[serde(rename = "precomputedRagSourceIdentifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precomputed_rag_source_identifiers: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFoundationModelAgreementOffersRequest {
    #[serde(rename = "modelId")]
    #[serde(default)]
    pub model_id: String,
    #[serde(rename = "offerType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offer_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFoundationModelAgreementOffersResponse {
    #[serde(rename = "modelId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offers: Option<Vec<Offer>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Offer {
    #[serde(rename = "offerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offer_id: Option<String>,
    #[serde(rename = "offerToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offer_token: Option<String>,
    #[serde(rename = "termDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub term_details: Option<TermDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TermDetails {
    #[serde(rename = "legalTerm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_term: Option<LegalTerm>,
    #[serde(rename = "supportTerm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_term: Option<SupportTerm>,
    #[serde(rename = "usageBasedPricingTerm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_based_pricing_term: Option<PricingTerm>,
    #[serde(rename = "validityTerm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity_term: Option<ValidityTerm>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LegalTerm {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SupportTerm {
    #[serde(rename = "refundPolicyDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_policy_description: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PricingTerm {
    #[serde(rename = "rateCard")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_card: Option<Vec<DimensionalPriceRate>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DimensionalPriceRate {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ValidityTerm {
    #[serde(rename = "agreementDuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agreement_duration: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFoundationModelsRequest {
    #[serde(rename = "byCustomizationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_customization_type: Option<String>,
    #[serde(rename = "byInferenceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_inference_type: Option<String>,
    #[serde(rename = "byOutputModality")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_output_modality: Option<String>,
    #[serde(rename = "byProvider")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_provider: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFoundationModelsResponse {
    #[serde(rename = "modelSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_summaries: Option<Vec<FoundationModelSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FoundationModelSummary {
    #[serde(rename = "customizationsSupported")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customizations_supported: Option<Vec<String>>,
    #[serde(rename = "inferenceTypesSupported")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_types_supported: Option<Vec<String>>,
    #[serde(rename = "inputModalities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_modalities: Option<Vec<String>>,
    #[serde(rename = "modelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_arn: Option<String>,
    #[serde(rename = "modelId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
    #[serde(rename = "modelLifecycle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_lifecycle: Option<FoundationModelLifecycle>,
    #[serde(rename = "modelName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_name: Option<String>,
    #[serde(rename = "outputModalities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_modalities: Option<Vec<String>>,
    #[serde(rename = "providerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,
    #[serde(rename = "responseStreamingSupported")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_streaming_supported: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListGuardrailsRequest {
    #[serde(rename = "guardrailIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guardrail_identifier: Option<String>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListGuardrailsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guardrails: Option<Vec<GuardrailSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GuardrailSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "crossRegionDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_region_details: Option<GuardrailCrossRegionDetails>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListImportedModelsRequest {
    #[serde(rename = "creationTimeAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_after: Option<String>,
    #[serde(rename = "creationTimeBefore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_before: Option<String>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nameContains")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "sortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(rename = "sortOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListImportedModelsResponse {
    #[serde(rename = "modelSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_summaries: Option<Vec<ImportedModelSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportedModelSummary {
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(rename = "instructSupported")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instruct_supported: Option<bool>,
    #[serde(rename = "modelArchitecture")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_architecture: Option<String>,
    #[serde(rename = "modelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_arn: Option<String>,
    #[serde(rename = "modelName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListInferenceProfilesRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "typeEquals")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_equals: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListInferenceProfilesResponse {
    #[serde(rename = "inferenceProfileSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_profile_summaries: Option<Vec<InferenceProfileSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InferenceProfileSummary {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "inferenceProfileArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_profile_arn: Option<String>,
    #[serde(rename = "inferenceProfileId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_profile_id: Option<String>,
    #[serde(rename = "inferenceProfileName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_profile_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub models: Option<Vec<InferenceProfileModel>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListMarketplaceModelEndpointsRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "modelSourceEquals")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_source_equals: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListMarketplaceModelEndpointsResponse {
    #[serde(rename = "marketplaceModelEndpoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketplace_model_endpoints: Option<Vec<MarketplaceModelEndpointSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MarketplaceModelEndpointSummary {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "endpointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_arn: Option<String>,
    #[serde(rename = "modelSourceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_source_identifier: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListModelCopyJobsRequest {
    #[serde(rename = "creationTimeAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_after: Option<String>,
    #[serde(rename = "creationTimeBefore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_before: Option<String>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "sortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(rename = "sortOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    #[serde(rename = "sourceAccountEquals")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_account_equals: Option<String>,
    #[serde(rename = "sourceModelArnEquals")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_model_arn_equals: Option<String>,
    #[serde(rename = "statusEquals")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_equals: Option<String>,
    #[serde(rename = "targetModelNameContains")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_model_name_contains: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListModelCopyJobsResponse {
    #[serde(rename = "modelCopyJobSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_copy_job_summaries: Option<Vec<ModelCopyJobSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModelCopyJobSummary {
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(rename = "failureMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
    #[serde(rename = "jobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_arn: Option<String>,
    #[serde(rename = "sourceAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_account_id: Option<String>,
    #[serde(rename = "sourceModelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_model_arn: Option<String>,
    #[serde(rename = "sourceModelName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_model_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "targetModelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_model_arn: Option<String>,
    #[serde(rename = "targetModelKmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_model_kms_key_arn: Option<String>,
    #[serde(rename = "targetModelName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_model_name: Option<String>,
    #[serde(rename = "targetModelTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_model_tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListModelCustomizationJobsRequest {
    #[serde(rename = "creationTimeAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_after: Option<String>,
    #[serde(rename = "creationTimeBefore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_before: Option<String>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nameContains")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "sortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(rename = "sortOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    #[serde(rename = "statusEquals")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_equals: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListModelCustomizationJobsResponse {
    #[serde(rename = "modelCustomizationJobSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_customization_job_summaries: Option<Vec<ModelCustomizationJobSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModelCustomizationJobSummary {
    #[serde(rename = "baseModelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_model_arn: Option<String>,
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(rename = "customModelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_model_arn: Option<String>,
    #[serde(rename = "customModelName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_model_name: Option<String>,
    #[serde(rename = "customizationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customization_type: Option<String>,
    #[serde(rename = "endTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(rename = "jobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_arn: Option<String>,
    #[serde(rename = "jobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "lastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_details: Option<StatusDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListModelImportJobsRequest {
    #[serde(rename = "creationTimeAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_after: Option<String>,
    #[serde(rename = "creationTimeBefore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_before: Option<String>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nameContains")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "sortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(rename = "sortOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    #[serde(rename = "statusEquals")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_equals: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListModelImportJobsResponse {
    #[serde(rename = "modelImportJobSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_import_job_summaries: Option<Vec<ModelImportJobSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModelImportJobSummary {
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(rename = "endTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(rename = "importedModelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imported_model_arn: Option<String>,
    #[serde(rename = "importedModelName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imported_model_name: Option<String>,
    #[serde(rename = "jobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_arn: Option<String>,
    #[serde(rename = "jobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "lastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListModelInvocationJobsRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nameContains")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "sortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(rename = "sortOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    #[serde(rename = "statusEquals")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_equals: Option<String>,
    #[serde(rename = "submitTimeAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time_after: Option<String>,
    #[serde(rename = "submitTimeBefore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time_before: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListModelInvocationJobsResponse {
    #[serde(rename = "invocationJobSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invocation_job_summaries: Option<Vec<ModelInvocationJobSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModelInvocationJobSummary {
    #[serde(rename = "clientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "endTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(rename = "errorRecordCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_record_count: Option<i64>,
    #[serde(rename = "inputDataConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_data_config: Option<ModelInvocationJobInputDataConfig>,
    #[serde(rename = "jobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_arn: Option<String>,
    #[serde(rename = "jobExpirationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_expiration_time: Option<String>,
    #[serde(rename = "jobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "lastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "modelId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
    #[serde(rename = "modelInvocationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_invocation_type: Option<String>,
    #[serde(rename = "outputDataConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_data_config: Option<ModelInvocationJobOutputDataConfig>,
    #[serde(rename = "processedRecordCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processed_record_count: Option<i64>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "submitTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time: Option<String>,
    #[serde(rename = "successRecordCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_record_count: Option<i64>,
    #[serde(rename = "timeoutDurationInHours")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_duration_in_hours: Option<i32>,
    #[serde(rename = "totalRecordCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_record_count: Option<i64>,
    #[serde(rename = "vpcConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPromptRoutersRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPromptRoutersResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "promptRouterSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_router_summaries: Option<Vec<PromptRouterSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PromptRouterSummary {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "fallbackModel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback_model: Option<PromptRouterTargetModel>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub models: Option<Vec<PromptRouterTargetModel>>,
    #[serde(rename = "promptRouterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_router_arn: Option<String>,
    #[serde(rename = "promptRouterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_router_name: Option<String>,
    #[serde(rename = "routingCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_criteria: Option<RoutingCriteria>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListProvisionedModelThroughputsRequest {
    #[serde(rename = "creationTimeAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_after: Option<String>,
    #[serde(rename = "creationTimeBefore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_before: Option<String>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "modelArnEquals")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_arn_equals: Option<String>,
    #[serde(rename = "nameContains")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "sortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(rename = "sortOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    #[serde(rename = "statusEquals")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_equals: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListProvisionedModelThroughputsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "provisionedModelSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_model_summaries: Option<Vec<ProvisionedModelSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProvisionedModelSummary {
    #[serde(rename = "commitmentDuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment_duration: Option<String>,
    #[serde(rename = "commitmentExpirationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment_expiration_time: Option<String>,
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(rename = "desiredModelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_model_arn: Option<String>,
    #[serde(rename = "desiredModelUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_model_units: Option<i32>,
    #[serde(rename = "foundationModelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foundation_model_arn: Option<String>,
    #[serde(rename = "lastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<String>,
    #[serde(rename = "modelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_arn: Option<String>,
    #[serde(rename = "modelUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_units: Option<i32>,
    #[serde(rename = "provisionedModelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_model_arn: Option<String>,
    #[serde(rename = "provisionedModelName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_model_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "resourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutEnforcedGuardrailConfigurationRequest {
    #[serde(rename = "configId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_id: Option<String>,
    #[serde(rename = "guardrailInferenceConfig")]
    #[serde(default)]
    pub guardrail_inference_config: AccountEnforcedGuardrailInferenceInputConfiguration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccountEnforcedGuardrailInferenceInputConfiguration {
    #[serde(rename = "guardrailIdentifier")]
    #[serde(default)]
    pub guardrail_identifier: String,
    #[serde(rename = "guardrailVersion")]
    #[serde(default)]
    pub guardrail_version: String,
    #[serde(rename = "modelEnforcement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_enforcement: Option<ModelEnforcement>,
    #[serde(rename = "selectiveContentGuarding")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective_content_guarding: Option<SelectiveContentGuarding>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutEnforcedGuardrailConfigurationResponse {
    #[serde(rename = "configId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_id: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "updatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutModelInvocationLoggingConfigurationRequest {
    #[serde(rename = "loggingConfig")]
    #[serde(default)]
    pub logging_config: LoggingConfig,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutModelInvocationLoggingConfigurationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutResourcePolicyRequest {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "resourcePolicy")]
    #[serde(default)]
    pub resource_policy: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutResourcePolicyResponse {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutUseCaseForModelAccessRequest {
    #[serde(rename = "formData")]
    #[serde(default)]
    pub form_data: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutUseCaseForModelAccessResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterMarketplaceModelEndpointRequest {
    #[serde(rename = "endpointIdentifier")]
    #[serde(default)]
    pub endpoint_identifier: String,
    #[serde(rename = "modelSourceIdentifier")]
    #[serde(default)]
    pub model_source_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterMarketplaceModelEndpointResponse {
    #[serde(rename = "marketplaceModelEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketplace_model_endpoint: Option<MarketplaceModelEndpoint>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartAutomatedReasoningPolicyBuildWorkflowRequest {
    #[serde(rename = "buildWorkflowType")]
    #[serde(default)]
    pub build_workflow_type: String,
    #[serde(rename = "clientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "policyArn")]
    #[serde(default)]
    pub policy_arn: String,
    #[serde(rename = "sourceContent")]
    #[serde(default)]
    pub source_content: AutomatedReasoningPolicyBuildWorkflowSource,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningPolicyBuildWorkflowSource {
    #[serde(rename = "policyDefinition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_definition: Option<AutomatedReasoningPolicyDefinition>,
    #[serde(rename = "workflowContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_content: Option<AutomatedReasoningPolicyWorkflowTypeContent>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningPolicyWorkflowTypeContent {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents: Option<Vec<AutomatedReasoningPolicyBuildWorkflowDocument>>,
    #[serde(rename = "generateFidelityReportContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generate_fidelity_report_content:
        Option<AutomatedReasoningPolicyGenerateFidelityReportContent>,
    #[serde(rename = "policyRepairAssets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_repair_assets: Option<AutomatedReasoningPolicyBuildWorkflowRepairContent>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningPolicyBuildWorkflowDocument {
    #[serde(default)]
    pub document: String,
    #[serde(rename = "documentContentType")]
    #[serde(default)]
    pub document_content_type: String,
    #[serde(rename = "documentDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_description: Option<String>,
    #[serde(rename = "documentName")]
    #[serde(default)]
    pub document_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningPolicyGenerateFidelityReportContent {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents: Option<Vec<AutomatedReasoningPolicyBuildWorkflowDocument>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedReasoningPolicyBuildWorkflowRepairContent {
    #[serde(default)]
    pub annotations: Vec<AutomatedReasoningPolicyAnnotation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartAutomatedReasoningPolicyBuildWorkflowResponse {
    #[serde(rename = "buildWorkflowId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_workflow_id: Option<String>,
    #[serde(rename = "policyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartAutomatedReasoningPolicyTestWorkflowRequest {
    #[serde(rename = "buildWorkflowId")]
    #[serde(default)]
    pub build_workflow_id: String,
    #[serde(rename = "clientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "policyArn")]
    #[serde(default)]
    pub policy_arn: String,
    #[serde(rename = "testCaseIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_case_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartAutomatedReasoningPolicyTestWorkflowResponse {
    #[serde(rename = "policyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopEvaluationJobRequest {
    #[serde(rename = "jobIdentifier")]
    #[serde(default)]
    pub job_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopEvaluationJobResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopModelCustomizationJobRequest {
    #[serde(rename = "jobIdentifier")]
    #[serde(default)]
    pub job_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopModelCustomizationJobResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopModelInvocationJobRequest {
    #[serde(rename = "jobIdentifier")]
    #[serde(default)]
    pub job_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopModelInvocationJobResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "resourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "resourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
    #[serde(rename = "tagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAutomatedReasoningPolicyAnnotationsRequest {
    #[serde(default)]
    pub annotations: Vec<AutomatedReasoningPolicyAnnotation>,
    #[serde(rename = "buildWorkflowId")]
    #[serde(default)]
    pub build_workflow_id: String,
    #[serde(rename = "lastUpdatedAnnotationSetHash")]
    #[serde(default)]
    pub last_updated_annotation_set_hash: String,
    #[serde(rename = "policyArn")]
    #[serde(default)]
    pub policy_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAutomatedReasoningPolicyAnnotationsResponse {
    #[serde(rename = "annotationSetHash")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation_set_hash: Option<String>,
    #[serde(rename = "buildWorkflowId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_workflow_id: Option<String>,
    #[serde(rename = "policyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_arn: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAutomatedReasoningPolicyRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "policyArn")]
    #[serde(default)]
    pub policy_arn: String,
    #[serde(rename = "policyDefinition")]
    #[serde(default)]
    pub policy_definition: AutomatedReasoningPolicyDefinition,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAutomatedReasoningPolicyResponse {
    #[serde(rename = "definitionHash")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition_hash: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "policyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_arn: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAutomatedReasoningPolicyTestCaseRequest {
    #[serde(rename = "clientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "confidenceThreshold")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence_threshold: Option<f64>,
    #[serde(rename = "expectedAggregatedFindingsResult")]
    #[serde(default)]
    pub expected_aggregated_findings_result: String,
    #[serde(rename = "guardContent")]
    #[serde(default)]
    pub guard_content: String,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    pub last_updated_at: String,
    #[serde(rename = "policyArn")]
    #[serde(default)]
    pub policy_arn: String,
    #[serde(rename = "queryContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_content: Option<String>,
    #[serde(rename = "testCaseId")]
    #[serde(default)]
    pub test_case_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAutomatedReasoningPolicyTestCaseResponse {
    #[serde(rename = "policyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_arn: Option<String>,
    #[serde(rename = "testCaseId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_case_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCustomModelDeploymentRequest {
    #[serde(rename = "customModelDeploymentIdentifier")]
    #[serde(default)]
    pub custom_model_deployment_identifier: String,
    #[serde(rename = "modelArn")]
    #[serde(default)]
    pub model_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCustomModelDeploymentResponse {
    #[serde(rename = "customModelDeploymentArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_model_deployment_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateGuardrailRequest {
    #[serde(rename = "automatedReasoningPolicyConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automated_reasoning_policy_config: Option<GuardrailAutomatedReasoningPolicyConfig>,
    #[serde(rename = "blockedInputMessaging")]
    #[serde(default)]
    pub blocked_input_messaging: String,
    #[serde(rename = "blockedOutputsMessaging")]
    #[serde(default)]
    pub blocked_outputs_messaging: String,
    #[serde(rename = "contentPolicyConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_policy_config: Option<GuardrailContentPolicyConfig>,
    #[serde(rename = "contextualGroundingPolicyConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contextual_grounding_policy_config: Option<GuardrailContextualGroundingPolicyConfig>,
    #[serde(rename = "crossRegionConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_region_config: Option<GuardrailCrossRegionConfig>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "guardrailIdentifier")]
    #[serde(default)]
    pub guardrail_identifier: String,
    #[serde(rename = "kmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "sensitiveInformationPolicyConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensitive_information_policy_config: Option<GuardrailSensitiveInformationPolicyConfig>,
    #[serde(rename = "topicPolicyConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_policy_config: Option<GuardrailTopicPolicyConfig>,
    #[serde(rename = "wordPolicyConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub word_policy_config: Option<GuardrailWordPolicyConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateGuardrailResponse {
    #[serde(rename = "guardrailArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guardrail_arn: Option<String>,
    #[serde(rename = "guardrailId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guardrail_id: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateMarketplaceModelEndpointRequest {
    #[serde(rename = "clientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "endpointArn")]
    #[serde(default)]
    pub endpoint_arn: String,
    #[serde(rename = "endpointConfig")]
    #[serde(default)]
    pub endpoint_config: EndpointConfig,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateMarketplaceModelEndpointResponse {
    #[serde(rename = "marketplaceModelEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketplace_model_endpoint: Option<MarketplaceModelEndpoint>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateProvisionedModelThroughputRequest {
    #[serde(rename = "desiredModelId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_model_id: Option<String>,
    #[serde(rename = "desiredProvisionedModelName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_provisioned_model_name: Option<String>,
    #[serde(rename = "provisionedModelId")]
    #[serde(default)]
    pub provisioned_model_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateProvisionedModelThroughputResponse {}
