//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-personalize

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateBatchInferenceJobRequest {
    #[serde(rename = "batchInferenceJobConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_inference_job_config: Option<BatchInferenceJobConfig>,
    #[serde(rename = "batchInferenceJobMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_inference_job_mode: Option<String>,
    #[serde(rename = "filterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_arn: Option<String>,
    #[serde(rename = "jobInput")]
    #[serde(default)]
    pub job_input: BatchInferenceJobInput,
    #[serde(rename = "jobName")]
    #[serde(default)]
    pub job_name: String,
    #[serde(rename = "jobOutput")]
    #[serde(default)]
    pub job_output: BatchInferenceJobOutput,
    #[serde(rename = "numResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_results: Option<i32>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(rename = "solutionVersionArn")]
    #[serde(default)]
    pub solution_version_arn: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "themeGenerationConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_generation_config: Option<ThemeGenerationConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchInferenceJobConfig {
    #[serde(rename = "itemExplorationConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_exploration_config: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "rankingInfluence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranking_influence: Option<std::collections::HashMap<String, f64>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchInferenceJobInput {
    #[serde(rename = "s3DataSource")]
    #[serde(default)]
    pub s3_data_source: S3DataConfig,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3DataConfig {
    #[serde(rename = "kmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    #[serde(default)]
    pub path: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchInferenceJobOutput {
    #[serde(rename = "s3DataDestination")]
    #[serde(default)]
    pub s3_data_destination: S3DataConfig,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Tag {
    #[serde(rename = "tagKey")]
    #[serde(default)]
    pub tag_key: String,
    #[serde(rename = "tagValue")]
    #[serde(default)]
    pub tag_value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ThemeGenerationConfig {
    #[serde(rename = "fieldsForThemeGeneration")]
    #[serde(default)]
    pub fields_for_theme_generation: FieldsForThemeGeneration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FieldsForThemeGeneration {
    #[serde(rename = "itemName")]
    #[serde(default)]
    pub item_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateBatchInferenceJobResponse {
    #[serde(rename = "batchInferenceJobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_inference_job_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateBatchSegmentJobRequest {
    #[serde(rename = "filterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_arn: Option<String>,
    #[serde(rename = "jobInput")]
    #[serde(default)]
    pub job_input: BatchSegmentJobInput,
    #[serde(rename = "jobName")]
    #[serde(default)]
    pub job_name: String,
    #[serde(rename = "jobOutput")]
    #[serde(default)]
    pub job_output: BatchSegmentJobOutput,
    #[serde(rename = "numResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_results: Option<i32>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(rename = "solutionVersionArn")]
    #[serde(default)]
    pub solution_version_arn: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchSegmentJobInput {
    #[serde(rename = "s3DataSource")]
    #[serde(default)]
    pub s3_data_source: S3DataConfig,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchSegmentJobOutput {
    #[serde(rename = "s3DataDestination")]
    #[serde(default)]
    pub s3_data_destination: S3DataConfig,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateBatchSegmentJobResponse {
    #[serde(rename = "batchSegmentJobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_segment_job_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCampaignRequest {
    #[serde(rename = "campaignConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_config: Option<CampaignConfig>,
    #[serde(rename = "minProvisionedTPS")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_provisioned_t_p_s: Option<i32>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "solutionVersionArn")]
    #[serde(default)]
    pub solution_version_arn: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CampaignConfig {
    #[serde(rename = "enableMetadataWithRecommendations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_metadata_with_recommendations: Option<bool>,
    #[serde(rename = "itemExplorationConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_exploration_config: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "rankingInfluence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranking_influence: Option<std::collections::HashMap<String, f64>>,
    #[serde(rename = "syncWithLatestSolutionVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_with_latest_solution_version: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCampaignResponse {
    #[serde(rename = "campaignArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDataDeletionJobRequest {
    #[serde(rename = "dataSource")]
    #[serde(default)]
    pub data_source: DataSource,
    #[serde(rename = "datasetGroupArn")]
    #[serde(default)]
    pub dataset_group_arn: String,
    #[serde(rename = "jobName")]
    #[serde(default)]
    pub job_name: String,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataSource {
    #[serde(rename = "dataLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_location: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDataDeletionJobResponse {
    #[serde(rename = "dataDeletionJobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_deletion_job_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDatasetExportJobRequest {
    #[serde(rename = "datasetArn")]
    #[serde(default)]
    pub dataset_arn: String,
    #[serde(rename = "ingestionMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingestion_mode: Option<String>,
    #[serde(rename = "jobName")]
    #[serde(default)]
    pub job_name: String,
    #[serde(rename = "jobOutput")]
    #[serde(default)]
    pub job_output: DatasetExportJobOutput,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DatasetExportJobOutput {
    #[serde(rename = "s3DataDestination")]
    #[serde(default)]
    pub s3_data_destination: S3DataConfig,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDatasetExportJobResponse {
    #[serde(rename = "datasetExportJobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_export_job_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDatasetGroupRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "kmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDatasetGroupResponse {
    #[serde(rename = "datasetGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_group_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDatasetImportJobRequest {
    #[serde(rename = "dataSource")]
    #[serde(default)]
    pub data_source: DataSource,
    #[serde(rename = "datasetArn")]
    #[serde(default)]
    pub dataset_arn: String,
    #[serde(rename = "importMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_mode: Option<String>,
    #[serde(rename = "jobName")]
    #[serde(default)]
    pub job_name: String,
    #[serde(rename = "publishAttributionMetricsToS3")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_attribution_metrics_to_s3: Option<bool>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDatasetImportJobResponse {
    #[serde(rename = "datasetImportJobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_import_job_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDatasetRequest {
    #[serde(rename = "datasetGroupArn")]
    #[serde(default)]
    pub dataset_group_arn: String,
    #[serde(rename = "datasetType")]
    #[serde(default)]
    pub dataset_type: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "schemaArn")]
    #[serde(default)]
    pub schema_arn: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDatasetResponse {
    #[serde(rename = "datasetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateEventTrackerRequest {
    #[serde(rename = "datasetGroupArn")]
    #[serde(default)]
    pub dataset_group_arn: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateEventTrackerResponse {
    #[serde(rename = "eventTrackerArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_tracker_arn: Option<String>,
    #[serde(rename = "trackingId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFilterRequest {
    #[serde(rename = "datasetGroupArn")]
    #[serde(default)]
    pub dataset_group_arn: String,
    #[serde(rename = "filterExpression")]
    #[serde(default)]
    pub filter_expression: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFilterResponse {
    #[serde(rename = "filterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateMetricAttributionRequest {
    #[serde(rename = "datasetGroupArn")]
    #[serde(default)]
    pub dataset_group_arn: String,
    #[serde(default)]
    pub metrics: Vec<MetricAttribute>,
    #[serde(rename = "metricsOutputConfig")]
    #[serde(default)]
    pub metrics_output_config: MetricAttributionOutput,
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MetricAttribute {
    #[serde(rename = "eventType")]
    #[serde(default)]
    pub event_type: String,
    #[serde(default)]
    pub expression: String,
    #[serde(rename = "metricName")]
    #[serde(default)]
    pub metric_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MetricAttributionOutput {
    #[serde(rename = "roleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(rename = "s3DataDestination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_data_destination: Option<S3DataConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateMetricAttributionResponse {
    #[serde(rename = "metricAttributionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_attribution_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateRecommenderRequest {
    #[serde(rename = "datasetGroupArn")]
    #[serde(default)]
    pub dataset_group_arn: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "recipeArn")]
    #[serde(default)]
    pub recipe_arn: String,
    #[serde(rename = "recommenderConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommender_config: Option<RecommenderConfig>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecommenderConfig {
    #[serde(rename = "enableMetadataWithRecommendations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_metadata_with_recommendations: Option<bool>,
    #[serde(rename = "itemExplorationConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_exploration_config: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "minRecommendationRequestsPerSecond")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_recommendation_requests_per_second: Option<i32>,
    #[serde(rename = "trainingDataConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_data_config: Option<TrainingDataConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TrainingDataConfig {
    #[serde(rename = "excludedDatasetColumns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_dataset_columns: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "includedDatasetColumns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub included_dataset_columns: Option<std::collections::HashMap<String, Vec<String>>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateRecommenderResponse {
    #[serde(rename = "recommenderArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommender_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSchemaRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub schema: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSchemaResponse {
    #[serde(rename = "schemaArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSolutionRequest {
    #[serde(rename = "datasetGroupArn")]
    #[serde(default)]
    pub dataset_group_arn: String,
    #[serde(rename = "eventType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "performAutoML")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perform_auto_m_l: Option<bool>,
    #[serde(rename = "performAutoTraining")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perform_auto_training: Option<bool>,
    #[serde(rename = "performHPO")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perform_h_p_o: Option<bool>,
    #[serde(rename = "performIncrementalUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perform_incremental_update: Option<bool>,
    #[serde(rename = "recipeArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipe_arn: Option<String>,
    #[serde(rename = "solutionConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution_config: Option<SolutionConfig>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SolutionConfig {
    #[serde(rename = "algorithmHyperParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm_hyper_parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "autoMLConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_m_l_config: Option<AutoMLConfig>,
    #[serde(rename = "autoTrainingConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_training_config: Option<AutoTrainingConfig>,
    #[serde(rename = "eventValueThreshold")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_value_threshold: Option<String>,
    #[serde(rename = "eventsConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events_config: Option<EventsConfig>,
    #[serde(rename = "featureTransformationParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_transformation_parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "hpoConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hpo_config: Option<HPOConfig>,
    #[serde(rename = "optimizationObjective")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optimization_objective: Option<OptimizationObjective>,
    #[serde(rename = "trainingDataConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_data_config: Option<TrainingDataConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutoMLConfig {
    #[serde(rename = "metricName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    #[serde(rename = "recipeList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipe_list: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutoTrainingConfig {
    #[serde(rename = "schedulingExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduling_expression: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventsConfig {
    #[serde(rename = "eventParametersList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_parameters_list: Option<Vec<EventParameters>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventParameters {
    #[serde(rename = "eventType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    #[serde(rename = "eventValueThreshold")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_value_threshold: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HPOConfig {
    #[serde(rename = "algorithmHyperParameterRanges")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm_hyper_parameter_ranges: Option<HyperParameterRanges>,
    #[serde(rename = "hpoObjective")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hpo_objective: Option<HPOObjective>,
    #[serde(rename = "hpoResourceConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hpo_resource_config: Option<HPOResourceConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HyperParameterRanges {
    #[serde(rename = "categoricalHyperParameterRanges")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categorical_hyper_parameter_ranges: Option<Vec<CategoricalHyperParameterRange>>,
    #[serde(rename = "continuousHyperParameterRanges")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuous_hyper_parameter_ranges: Option<Vec<ContinuousHyperParameterRange>>,
    #[serde(rename = "integerHyperParameterRanges")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integer_hyper_parameter_ranges: Option<Vec<IntegerHyperParameterRange>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CategoricalHyperParameterRange {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContinuousHyperParameterRange {
    #[serde(rename = "maxValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_value: Option<f64>,
    #[serde(rename = "minValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_value: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IntegerHyperParameterRange {
    #[serde(rename = "maxValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_value: Option<i32>,
    #[serde(rename = "minValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_value: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HPOObjective {
    #[serde(rename = "metricName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    #[serde(rename = "metricRegex")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_regex: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HPOResourceConfig {
    #[serde(rename = "maxNumberOfTrainingJobs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_number_of_training_jobs: Option<String>,
    #[serde(rename = "maxParallelTrainingJobs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_parallel_training_jobs: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OptimizationObjective {
    #[serde(rename = "itemAttribute")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_attribute: Option<String>,
    #[serde(rename = "objectiveSensitivity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub objective_sensitivity: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSolutionResponse {
    #[serde(rename = "solutionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSolutionVersionRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "solutionArn")]
    #[serde(default)]
    pub solution_arn: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "trainingMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSolutionVersionResponse {
    #[serde(rename = "solutionVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution_version_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCampaignRequest {
    #[serde(rename = "campaignArn")]
    #[serde(default)]
    pub campaign_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDatasetGroupRequest {
    #[serde(rename = "datasetGroupArn")]
    #[serde(default)]
    pub dataset_group_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDatasetRequest {
    #[serde(rename = "datasetArn")]
    #[serde(default)]
    pub dataset_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteEventTrackerRequest {
    #[serde(rename = "eventTrackerArn")]
    #[serde(default)]
    pub event_tracker_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFilterRequest {
    #[serde(rename = "filterArn")]
    #[serde(default)]
    pub filter_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteMetricAttributionRequest {
    #[serde(rename = "metricAttributionArn")]
    #[serde(default)]
    pub metric_attribution_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRecommenderRequest {
    #[serde(rename = "recommenderArn")]
    #[serde(default)]
    pub recommender_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSchemaRequest {
    #[serde(rename = "schemaArn")]
    #[serde(default)]
    pub schema_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSolutionRequest {
    #[serde(rename = "solutionArn")]
    #[serde(default)]
    pub solution_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAlgorithmRequest {
    #[serde(rename = "algorithmArn")]
    #[serde(default)]
    pub algorithm_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAlgorithmResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<Algorithm>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Algorithm {
    #[serde(rename = "algorithmArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm_arn: Option<String>,
    #[serde(rename = "algorithmImage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm_image: Option<AlgorithmImage>,
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "defaultHyperParameterRanges")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_hyper_parameter_ranges: Option<DefaultHyperParameterRanges>,
    #[serde(rename = "defaultHyperParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_hyper_parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "defaultResourceConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_resource_config: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "trainingInputMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_input_mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AlgorithmImage {
    #[serde(rename = "dockerURI")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docker_u_r_i: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DefaultHyperParameterRanges {
    #[serde(rename = "categoricalHyperParameterRanges")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categorical_hyper_parameter_ranges: Option<Vec<DefaultCategoricalHyperParameterRange>>,
    #[serde(rename = "continuousHyperParameterRanges")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuous_hyper_parameter_ranges: Option<Vec<DefaultContinuousHyperParameterRange>>,
    #[serde(rename = "integerHyperParameterRanges")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integer_hyper_parameter_ranges: Option<Vec<DefaultIntegerHyperParameterRange>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DefaultCategoricalHyperParameterRange {
    #[serde(rename = "isTunable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_tunable: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DefaultContinuousHyperParameterRange {
    #[serde(rename = "isTunable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_tunable: Option<bool>,
    #[serde(rename = "maxValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_value: Option<f64>,
    #[serde(rename = "minValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_value: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DefaultIntegerHyperParameterRange {
    #[serde(rename = "isTunable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_tunable: Option<bool>,
    #[serde(rename = "maxValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_value: Option<i32>,
    #[serde(rename = "minValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_value: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeBatchInferenceJobRequest {
    #[serde(rename = "batchInferenceJobArn")]
    #[serde(default)]
    pub batch_inference_job_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeBatchInferenceJobResponse {
    #[serde(rename = "batchInferenceJob")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_inference_job: Option<BatchInferenceJob>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchInferenceJob {
    #[serde(rename = "batchInferenceJobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_inference_job_arn: Option<String>,
    #[serde(rename = "batchInferenceJobConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_inference_job_config: Option<BatchInferenceJobConfig>,
    #[serde(rename = "batchInferenceJobMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_inference_job_mode: Option<String>,
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "failureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "filterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_arn: Option<String>,
    #[serde(rename = "jobInput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_input: Option<BatchInferenceJobInput>,
    #[serde(rename = "jobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "jobOutput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_output: Option<BatchInferenceJobOutput>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(rename = "numResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_results: Option<i32>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "solutionVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution_version_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "themeGenerationConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_generation_config: Option<ThemeGenerationConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeBatchSegmentJobRequest {
    #[serde(rename = "batchSegmentJobArn")]
    #[serde(default)]
    pub batch_segment_job_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeBatchSegmentJobResponse {
    #[serde(rename = "batchSegmentJob")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_segment_job: Option<BatchSegmentJob>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchSegmentJob {
    #[serde(rename = "batchSegmentJobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_segment_job_arn: Option<String>,
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "failureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "filterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_arn: Option<String>,
    #[serde(rename = "jobInput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_input: Option<BatchSegmentJobInput>,
    #[serde(rename = "jobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "jobOutput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_output: Option<BatchSegmentJobOutput>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(rename = "numResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_results: Option<i32>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "solutionVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution_version_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeCampaignRequest {
    #[serde(rename = "campaignArn")]
    #[serde(default)]
    pub campaign_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeCampaignResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign: Option<Campaign>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Campaign {
    #[serde(rename = "campaignArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_arn: Option<String>,
    #[serde(rename = "campaignConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_config: Option<CampaignConfig>,
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "failureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(rename = "latestCampaignUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_campaign_update: Option<CampaignUpdateSummary>,
    #[serde(rename = "minProvisionedTPS")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_provisioned_t_p_s: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "solutionVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution_version_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CampaignUpdateSummary {
    #[serde(rename = "campaignConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_config: Option<CampaignConfig>,
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "failureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(rename = "minProvisionedTPS")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_provisioned_t_p_s: Option<i32>,
    #[serde(rename = "solutionVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution_version_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDataDeletionJobRequest {
    #[serde(rename = "dataDeletionJobArn")]
    #[serde(default)]
    pub data_deletion_job_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDataDeletionJobResponse {
    #[serde(rename = "dataDeletionJob")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_deletion_job: Option<DataDeletionJob>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataDeletionJob {
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "dataDeletionJobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_deletion_job_arn: Option<String>,
    #[serde(rename = "dataSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source: Option<DataSource>,
    #[serde(rename = "datasetGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_group_arn: Option<String>,
    #[serde(rename = "failureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "jobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(rename = "numDeleted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_deleted: Option<i32>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDatasetExportJobRequest {
    #[serde(rename = "datasetExportJobArn")]
    #[serde(default)]
    pub dataset_export_job_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDatasetExportJobResponse {
    #[serde(rename = "datasetExportJob")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_export_job: Option<DatasetExportJob>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DatasetExportJob {
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "datasetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_arn: Option<String>,
    #[serde(rename = "datasetExportJobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_export_job_arn: Option<String>,
    #[serde(rename = "failureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "ingestionMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingestion_mode: Option<String>,
    #[serde(rename = "jobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "jobOutput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_output: Option<DatasetExportJobOutput>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDatasetGroupRequest {
    #[serde(rename = "datasetGroupArn")]
    #[serde(default)]
    pub dataset_group_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDatasetGroupResponse {
    #[serde(rename = "datasetGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_group: Option<DatasetGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DatasetGroup {
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "datasetGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_group_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "failureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "kmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDatasetImportJobRequest {
    #[serde(rename = "datasetImportJobArn")]
    #[serde(default)]
    pub dataset_import_job_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDatasetImportJobResponse {
    #[serde(rename = "datasetImportJob")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_import_job: Option<DatasetImportJob>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DatasetImportJob {
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "dataSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source: Option<DataSource>,
    #[serde(rename = "datasetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_arn: Option<String>,
    #[serde(rename = "datasetImportJobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_import_job_arn: Option<String>,
    #[serde(rename = "failureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "importMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_mode: Option<String>,
    #[serde(rename = "jobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(rename = "publishAttributionMetricsToS3")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_attribution_metrics_to_s3: Option<bool>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDatasetRequest {
    #[serde(rename = "datasetArn")]
    #[serde(default)]
    pub dataset_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDatasetResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset: Option<Dataset>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Dataset {
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "datasetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_arn: Option<String>,
    #[serde(rename = "datasetGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_group_arn: Option<String>,
    #[serde(rename = "datasetType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_type: Option<String>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(rename = "latestDatasetUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_dataset_update: Option<DatasetUpdateSummary>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "schemaArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "trackingId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DatasetUpdateSummary {
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "failureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(rename = "schemaArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeEventTrackerRequest {
    #[serde(rename = "eventTrackerArn")]
    #[serde(default)]
    pub event_tracker_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeEventTrackerResponse {
    #[serde(rename = "eventTracker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_tracker: Option<EventTracker>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventTracker {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "datasetGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_group_arn: Option<String>,
    #[serde(rename = "eventTrackerArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_tracker_arn: Option<String>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "trackingId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeFeatureTransformationRequest {
    #[serde(rename = "featureTransformationArn")]
    #[serde(default)]
    pub feature_transformation_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeFeatureTransformationResponse {
    #[serde(rename = "featureTransformation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_transformation: Option<FeatureTransformation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FeatureTransformation {
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "defaultParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "featureTransformationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_transformation_arn: Option<String>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeFilterRequest {
    #[serde(rename = "filterArn")]
    #[serde(default)]
    pub filter_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeFilterResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Filter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Filter {
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "datasetGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_group_arn: Option<String>,
    #[serde(rename = "failureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "filterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_arn: Option<String>,
    #[serde(rename = "filterExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_expression: Option<String>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeMetricAttributionRequest {
    #[serde(rename = "metricAttributionArn")]
    #[serde(default)]
    pub metric_attribution_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeMetricAttributionResponse {
    #[serde(rename = "metricAttribution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_attribution: Option<MetricAttribution>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MetricAttribution {
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "datasetGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_group_arn: Option<String>,
    #[serde(rename = "failureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(rename = "metricAttributionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_attribution_arn: Option<String>,
    #[serde(rename = "metricsOutputConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics_output_config: Option<MetricAttributionOutput>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRecipeRequest {
    #[serde(rename = "recipeArn")]
    #[serde(default)]
    pub recipe_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRecipeResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipe: Option<Recipe>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Recipe {
    #[serde(rename = "algorithmArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm_arn: Option<String>,
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "featureTransformationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_transformation_arn: Option<String>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "recipeArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipe_arn: Option<String>,
    #[serde(rename = "recipeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipe_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRecommenderRequest {
    #[serde(rename = "recommenderArn")]
    #[serde(default)]
    pub recommender_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRecommenderResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommender: Option<Recommender>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Recommender {
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "datasetGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_group_arn: Option<String>,
    #[serde(rename = "failureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(rename = "latestRecommenderUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_recommender_update: Option<RecommenderUpdateSummary>,
    #[serde(rename = "modelMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_metrics: Option<std::collections::HashMap<String, f64>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "recipeArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipe_arn: Option<String>,
    #[serde(rename = "recommenderArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommender_arn: Option<String>,
    #[serde(rename = "recommenderConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommender_config: Option<RecommenderConfig>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecommenderUpdateSummary {
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "failureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(rename = "recommenderConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommender_config: Option<RecommenderConfig>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSchemaRequest {
    #[serde(rename = "schemaArn")]
    #[serde(default)]
    pub schema_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSchemaResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<DatasetSchema>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DatasetSchema {
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[serde(rename = "schemaArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSolutionRequest {
    #[serde(rename = "solutionArn")]
    #[serde(default)]
    pub solution_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSolutionResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution: Option<Solution>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Solution {
    #[serde(rename = "autoMLResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_m_l_result: Option<AutoMLResult>,
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "datasetGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_group_arn: Option<String>,
    #[serde(rename = "eventType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(rename = "latestSolutionUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_solution_update: Option<SolutionUpdateSummary>,
    #[serde(rename = "latestSolutionVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_solution_version: Option<SolutionVersionSummary>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "performAutoML")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perform_auto_m_l: Option<bool>,
    #[serde(rename = "performAutoTraining")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perform_auto_training: Option<bool>,
    #[serde(rename = "performHPO")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perform_h_p_o: Option<bool>,
    #[serde(rename = "performIncrementalUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perform_incremental_update: Option<bool>,
    #[serde(rename = "recipeArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipe_arn: Option<String>,
    #[serde(rename = "solutionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution_arn: Option<String>,
    #[serde(rename = "solutionConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution_config: Option<SolutionConfig>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutoMLResult {
    #[serde(rename = "bestRecipeArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub best_recipe_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SolutionUpdateSummary {
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "failureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(rename = "performAutoTraining")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perform_auto_training: Option<bool>,
    #[serde(rename = "performIncrementalUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perform_incremental_update: Option<bool>,
    #[serde(rename = "solutionUpdateConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution_update_config: Option<SolutionUpdateConfig>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SolutionUpdateConfig {
    #[serde(rename = "autoTrainingConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_training_config: Option<AutoTrainingConfig>,
    #[serde(rename = "eventsConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events_config: Option<EventsConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SolutionVersionSummary {
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "failureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(rename = "solutionVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution_version_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "trainingMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_mode: Option<String>,
    #[serde(rename = "trainingType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSolutionVersionRequest {
    #[serde(rename = "solutionVersionArn")]
    #[serde(default)]
    pub solution_version_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSolutionVersionResponse {
    #[serde(rename = "solutionVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution_version: Option<SolutionVersion>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SolutionVersion {
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "datasetGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_group_arn: Option<String>,
    #[serde(rename = "eventType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    #[serde(rename = "failureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "performAutoML")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perform_auto_m_l: Option<bool>,
    #[serde(rename = "performHPO")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perform_h_p_o: Option<bool>,
    #[serde(rename = "performIncrementalUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perform_incremental_update: Option<bool>,
    #[serde(rename = "recipeArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipe_arn: Option<String>,
    #[serde(rename = "solutionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution_arn: Option<String>,
    #[serde(rename = "solutionConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution_config: Option<SolutionConfig>,
    #[serde(rename = "solutionVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution_version_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "trainingHours")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_hours: Option<f64>,
    #[serde(rename = "trainingMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_mode: Option<String>,
    #[serde(rename = "trainingType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_type: Option<String>,
    #[serde(rename = "tunedHPOParams")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tuned_h_p_o_params: Option<TunedHPOParams>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TunedHPOParams {
    #[serde(rename = "algorithmHyperParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm_hyper_parameters: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSolutionMetricsRequest {
    #[serde(rename = "solutionVersionArn")]
    #[serde(default)]
    pub solution_version_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSolutionMetricsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<std::collections::HashMap<String, f64>>,
    #[serde(rename = "solutionVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution_version_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBatchInferenceJobsRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "solutionVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution_version_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBatchInferenceJobsResponse {
    #[serde(rename = "batchInferenceJobs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_inference_jobs: Option<Vec<BatchInferenceJobSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchInferenceJobSummary {
    #[serde(rename = "batchInferenceJobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_inference_job_arn: Option<String>,
    #[serde(rename = "batchInferenceJobMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_inference_job_mode: Option<String>,
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "failureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "jobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(rename = "solutionVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution_version_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBatchSegmentJobsRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "solutionVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution_version_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBatchSegmentJobsResponse {
    #[serde(rename = "batchSegmentJobs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_segment_jobs: Option<Vec<BatchSegmentJobSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchSegmentJobSummary {
    #[serde(rename = "batchSegmentJobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_segment_job_arn: Option<String>,
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "failureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "jobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(rename = "solutionVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution_version_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCampaignsRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "solutionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCampaignsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaigns: Option<Vec<CampaignSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CampaignSummary {
    #[serde(rename = "campaignArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_arn: Option<String>,
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "failureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDataDeletionJobsRequest {
    #[serde(rename = "datasetGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_group_arn: Option<String>,
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
pub struct ListDataDeletionJobsResponse {
    #[serde(rename = "dataDeletionJobs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_deletion_jobs: Option<Vec<DataDeletionJobSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataDeletionJobSummary {
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "dataDeletionJobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_deletion_job_arn: Option<String>,
    #[serde(rename = "datasetGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_group_arn: Option<String>,
    #[serde(rename = "failureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "jobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDatasetExportJobsRequest {
    #[serde(rename = "datasetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_arn: Option<String>,
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
pub struct ListDatasetExportJobsResponse {
    #[serde(rename = "datasetExportJobs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_export_jobs: Option<Vec<DatasetExportJobSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DatasetExportJobSummary {
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "datasetExportJobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_export_job_arn: Option<String>,
    #[serde(rename = "failureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "jobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDatasetGroupsRequest {
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
pub struct ListDatasetGroupsResponse {
    #[serde(rename = "datasetGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_groups: Option<Vec<DatasetGroupSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DatasetGroupSummary {
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "datasetGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_group_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "failureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDatasetImportJobsRequest {
    #[serde(rename = "datasetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_arn: Option<String>,
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
pub struct ListDatasetImportJobsResponse {
    #[serde(rename = "datasetImportJobs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_import_jobs: Option<Vec<DatasetImportJobSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DatasetImportJobSummary {
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "datasetImportJobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_import_job_arn: Option<String>,
    #[serde(rename = "failureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "importMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_mode: Option<String>,
    #[serde(rename = "jobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDatasetsRequest {
    #[serde(rename = "datasetGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_group_arn: Option<String>,
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
pub struct ListDatasetsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datasets: Option<Vec<DatasetSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DatasetSummary {
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "datasetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_arn: Option<String>,
    #[serde(rename = "datasetType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_type: Option<String>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListEventTrackersRequest {
    #[serde(rename = "datasetGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_group_arn: Option<String>,
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
pub struct ListEventTrackersResponse {
    #[serde(rename = "eventTrackers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_trackers: Option<Vec<EventTrackerSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventTrackerSummary {
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "eventTrackerArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_tracker_arn: Option<String>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFiltersRequest {
    #[serde(rename = "datasetGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_group_arn: Option<String>,
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
pub struct ListFiltersResponse {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<FilterSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FilterSummary {
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "datasetGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_group_arn: Option<String>,
    #[serde(rename = "failureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "filterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_arn: Option<String>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListMetricAttributionMetricsRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "metricAttributionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_attribution_arn: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListMetricAttributionMetricsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Vec<MetricAttribute>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListMetricAttributionsRequest {
    #[serde(rename = "datasetGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_group_arn: Option<String>,
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
pub struct ListMetricAttributionsResponse {
    #[serde(rename = "metricAttributions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_attributions: Option<Vec<MetricAttributionSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MetricAttributionSummary {
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "failureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(rename = "metricAttributionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_attribution_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRecipesRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "recipeProvider")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipe_provider: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRecipesResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipes: Option<Vec<RecipeSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecipeSummary {
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "recipeArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipe_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRecommendersRequest {
    #[serde(rename = "datasetGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_group_arn: Option<String>,
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
pub struct ListRecommendersResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommenders: Option<Vec<RecommenderSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecommenderSummary {
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "datasetGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_group_arn: Option<String>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "recipeArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipe_arn: Option<String>,
    #[serde(rename = "recommenderArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommender_arn: Option<String>,
    #[serde(rename = "recommenderConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommender_config: Option<RecommenderConfig>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSchemasRequest {
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
pub struct ListSchemasResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schemas: Option<Vec<DatasetSchemaSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DatasetSchemaSummary {
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "schemaArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSolutionVersionsRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "solutionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSolutionVersionsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "solutionVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution_versions: Option<Vec<SolutionVersionSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSolutionsRequest {
    #[serde(rename = "datasetGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_group_arn: Option<String>,
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
pub struct ListSolutionsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solutions: Option<Vec<SolutionSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SolutionSummary {
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "recipeArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipe_arn: Option<String>,
    #[serde(rename = "solutionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartRecommenderRequest {
    #[serde(rename = "recommenderArn")]
    #[serde(default)]
    pub recommender_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartRecommenderResponse {
    #[serde(rename = "recommenderArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommender_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopRecommenderRequest {
    #[serde(rename = "recommenderArn")]
    #[serde(default)]
    pub recommender_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopRecommenderResponse {
    #[serde(rename = "recommenderArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommender_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopSolutionVersionCreationRequest {
    #[serde(rename = "solutionVersionArn")]
    #[serde(default)]
    pub solution_version_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "tagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCampaignRequest {
    #[serde(rename = "campaignArn")]
    #[serde(default)]
    pub campaign_arn: String,
    #[serde(rename = "campaignConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_config: Option<CampaignConfig>,
    #[serde(rename = "minProvisionedTPS")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_provisioned_t_p_s: Option<i32>,
    #[serde(rename = "solutionVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution_version_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCampaignResponse {
    #[serde(rename = "campaignArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDatasetRequest {
    #[serde(rename = "datasetArn")]
    #[serde(default)]
    pub dataset_arn: String,
    #[serde(rename = "schemaArn")]
    #[serde(default)]
    pub schema_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDatasetResponse {
    #[serde(rename = "datasetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateMetricAttributionRequest {
    #[serde(rename = "addMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_metrics: Option<Vec<MetricAttribute>>,
    #[serde(rename = "metricAttributionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_attribution_arn: Option<String>,
    #[serde(rename = "metricsOutputConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics_output_config: Option<MetricAttributionOutput>,
    #[serde(rename = "removeMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_metrics: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateMetricAttributionResponse {
    #[serde(rename = "metricAttributionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_attribution_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRecommenderRequest {
    #[serde(rename = "recommenderArn")]
    #[serde(default)]
    pub recommender_arn: String,
    #[serde(rename = "recommenderConfig")]
    #[serde(default)]
    pub recommender_config: RecommenderConfig,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRecommenderResponse {
    #[serde(rename = "recommenderArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommender_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSolutionRequest {
    #[serde(rename = "performAutoTraining")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perform_auto_training: Option<bool>,
    #[serde(rename = "performIncrementalUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perform_incremental_update: Option<bool>,
    #[serde(rename = "solutionArn")]
    #[serde(default)]
    pub solution_arn: String,
    #[serde(rename = "solutionUpdateConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution_update_config: Option<SolutionUpdateConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSolutionResponse {
    #[serde(rename = "solutionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution_arn: Option<String>,
}
