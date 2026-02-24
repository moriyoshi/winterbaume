//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-forecast

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAutoPredictorRequest {
    #[serde(rename = "DataConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_config: Option<DataConfig>,
    #[serde(rename = "EncryptionConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_config: Option<EncryptionConfig>,
    #[serde(rename = "ExplainPredictor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explain_predictor: Option<bool>,
    #[serde(rename = "ForecastDimensions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast_dimensions: Option<Vec<String>>,
    #[serde(rename = "ForecastFrequency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast_frequency: Option<String>,
    #[serde(rename = "ForecastHorizon")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast_horizon: Option<i32>,
    #[serde(rename = "ForecastTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast_types: Option<Vec<String>>,
    #[serde(rename = "MonitorConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor_config: Option<MonitorConfig>,
    #[serde(rename = "OptimizationMetric")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optimization_metric: Option<String>,
    #[serde(rename = "PredictorName")]
    #[serde(default)]
    pub predictor_name: String,
    #[serde(rename = "ReferencePredictorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_predictor_arn: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "TimeAlignmentBoundary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_alignment_boundary: Option<TimeAlignmentBoundary>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataConfig {
    #[serde(rename = "AdditionalDatasets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_datasets: Option<Vec<AdditionalDataset>>,
    #[serde(rename = "AttributeConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_configs: Option<Vec<AttributeConfig>>,
    #[serde(rename = "DatasetGroupArn")]
    #[serde(default)]
    pub dataset_group_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdditionalDataset {
    #[serde(rename = "Configuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttributeConfig {
    #[serde(rename = "AttributeName")]
    #[serde(default)]
    pub attribute_name: String,
    #[serde(rename = "Transformations")]
    #[serde(default)]
    pub transformations: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EncryptionConfig {
    #[serde(rename = "KMSKeyArn")]
    #[serde(default)]
    pub k_m_s_key_arn: String,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    pub role_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MonitorConfig {
    #[serde(rename = "MonitorName")]
    #[serde(default)]
    pub monitor_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Tag {
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TimeAlignmentBoundary {
    #[serde(rename = "DayOfMonth")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_of_month: Option<i32>,
    #[serde(rename = "DayOfWeek")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_of_week: Option<String>,
    #[serde(rename = "Hour")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hour: Option<i32>,
    #[serde(rename = "Month")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAutoPredictorResponse {
    #[serde(rename = "PredictorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predictor_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDatasetGroupRequest {
    #[serde(rename = "DatasetArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_arns: Option<Vec<String>>,
    #[serde(rename = "DatasetGroupName")]
    #[serde(default)]
    pub dataset_group_name: String,
    #[serde(rename = "Domain")]
    #[serde(default)]
    pub domain: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDatasetGroupResponse {
    #[serde(rename = "DatasetGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_group_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDatasetImportJobRequest {
    #[serde(rename = "DataSource")]
    #[serde(default)]
    pub data_source: DataSource,
    #[serde(rename = "DatasetArn")]
    #[serde(default)]
    pub dataset_arn: String,
    #[serde(rename = "DatasetImportJobName")]
    #[serde(default)]
    pub dataset_import_job_name: String,
    #[serde(rename = "Format")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(rename = "GeolocationFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geolocation_format: Option<String>,
    #[serde(rename = "ImportMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_mode: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "TimeZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    #[serde(rename = "TimestampFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_format: Option<String>,
    #[serde(rename = "UseGeolocationForTimeZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_geolocation_for_time_zone: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataSource {
    #[serde(rename = "S3Config")]
    #[serde(default)]
    pub s3_config: S3Config,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3Config {
    #[serde(rename = "KMSKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k_m_s_key_arn: Option<String>,
    #[serde(rename = "Path")]
    #[serde(default)]
    pub path: String,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    pub role_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDatasetImportJobResponse {
    #[serde(rename = "DatasetImportJobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_import_job_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDatasetRequest {
    #[serde(rename = "DataFrequency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_frequency: Option<String>,
    #[serde(rename = "DatasetName")]
    #[serde(default)]
    pub dataset_name: String,
    #[serde(rename = "DatasetType")]
    #[serde(default)]
    pub dataset_type: String,
    #[serde(rename = "Domain")]
    #[serde(default)]
    pub domain: String,
    #[serde(rename = "EncryptionConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_config: Option<EncryptionConfig>,
    #[serde(rename = "Schema")]
    #[serde(default)]
    pub schema: Schema,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Schema {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<SchemaAttribute>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SchemaAttribute {
    #[serde(rename = "AttributeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_name: Option<String>,
    #[serde(rename = "AttributeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDatasetResponse {
    #[serde(rename = "DatasetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateExplainabilityExportRequest {
    #[serde(rename = "Destination")]
    #[serde(default)]
    pub destination: DataDestination,
    #[serde(rename = "ExplainabilityArn")]
    #[serde(default)]
    pub explainability_arn: String,
    #[serde(rename = "ExplainabilityExportName")]
    #[serde(default)]
    pub explainability_export_name: String,
    #[serde(rename = "Format")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataDestination {
    #[serde(rename = "S3Config")]
    #[serde(default)]
    pub s3_config: S3Config,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateExplainabilityExportResponse {
    #[serde(rename = "ExplainabilityExportArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explainability_export_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateExplainabilityRequest {
    #[serde(rename = "DataSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source: Option<DataSource>,
    #[serde(rename = "EnableVisualization")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_visualization: Option<bool>,
    #[serde(rename = "EndDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date_time: Option<String>,
    #[serde(rename = "ExplainabilityConfig")]
    #[serde(default)]
    pub explainability_config: ExplainabilityConfig,
    #[serde(rename = "ExplainabilityName")]
    #[serde(default)]
    pub explainability_name: String,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "Schema")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<Schema>,
    #[serde(rename = "StartDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date_time: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExplainabilityConfig {
    #[serde(rename = "TimePointGranularity")]
    #[serde(default)]
    pub time_point_granularity: String,
    #[serde(rename = "TimeSeriesGranularity")]
    #[serde(default)]
    pub time_series_granularity: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateExplainabilityResponse {
    #[serde(rename = "ExplainabilityArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explainability_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateForecastExportJobRequest {
    #[serde(rename = "Destination")]
    #[serde(default)]
    pub destination: DataDestination,
    #[serde(rename = "ForecastArn")]
    #[serde(default)]
    pub forecast_arn: String,
    #[serde(rename = "ForecastExportJobName")]
    #[serde(default)]
    pub forecast_export_job_name: String,
    #[serde(rename = "Format")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateForecastExportJobResponse {
    #[serde(rename = "ForecastExportJobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast_export_job_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateForecastRequest {
    #[serde(rename = "ForecastName")]
    #[serde(default)]
    pub forecast_name: String,
    #[serde(rename = "ForecastTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast_types: Option<Vec<String>>,
    #[serde(rename = "PredictorArn")]
    #[serde(default)]
    pub predictor_arn: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "TimeSeriesSelector")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_series_selector: Option<TimeSeriesSelector>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TimeSeriesSelector {
    #[serde(rename = "TimeSeriesIdentifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_series_identifiers: Option<TimeSeriesIdentifiers>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TimeSeriesIdentifiers {
    #[serde(rename = "DataSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source: Option<DataSource>,
    #[serde(rename = "Format")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(rename = "Schema")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<Schema>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateForecastResponse {
    #[serde(rename = "ForecastArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateMonitorRequest {
    #[serde(rename = "MonitorName")]
    #[serde(default)]
    pub monitor_name: String,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateMonitorResponse {
    #[serde(rename = "MonitorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePredictorBacktestExportJobRequest {
    #[serde(rename = "Destination")]
    #[serde(default)]
    pub destination: DataDestination,
    #[serde(rename = "Format")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(rename = "PredictorArn")]
    #[serde(default)]
    pub predictor_arn: String,
    #[serde(rename = "PredictorBacktestExportJobName")]
    #[serde(default)]
    pub predictor_backtest_export_job_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePredictorBacktestExportJobResponse {
    #[serde(rename = "PredictorBacktestExportJobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predictor_backtest_export_job_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePredictorRequest {
    #[serde(rename = "AlgorithmArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm_arn: Option<String>,
    #[serde(rename = "AutoMLOverrideStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_m_l_override_strategy: Option<String>,
    #[serde(rename = "EncryptionConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_config: Option<EncryptionConfig>,
    #[serde(rename = "EvaluationParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_parameters: Option<EvaluationParameters>,
    #[serde(rename = "FeaturizationConfig")]
    #[serde(default)]
    pub featurization_config: FeaturizationConfig,
    #[serde(rename = "ForecastHorizon")]
    #[serde(default)]
    pub forecast_horizon: i32,
    #[serde(rename = "ForecastTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast_types: Option<Vec<String>>,
    #[serde(rename = "HPOConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h_p_o_config: Option<HyperParameterTuningJobConfig>,
    #[serde(rename = "InputDataConfig")]
    #[serde(default)]
    pub input_data_config: InputDataConfig,
    #[serde(rename = "OptimizationMetric")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optimization_metric: Option<String>,
    #[serde(rename = "PerformAutoML")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perform_auto_m_l: Option<bool>,
    #[serde(rename = "PerformHPO")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perform_h_p_o: Option<bool>,
    #[serde(rename = "PredictorName")]
    #[serde(default)]
    pub predictor_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "TrainingParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_parameters: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationParameters {
    #[serde(rename = "BackTestWindowOffset")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back_test_window_offset: Option<i32>,
    #[serde(rename = "NumberOfBacktestWindows")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_backtest_windows: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FeaturizationConfig {
    #[serde(rename = "Featurizations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub featurizations: Option<Vec<Featurization>>,
    #[serde(rename = "ForecastDimensions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast_dimensions: Option<Vec<String>>,
    #[serde(rename = "ForecastFrequency")]
    #[serde(default)]
    pub forecast_frequency: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Featurization {
    #[serde(rename = "AttributeName")]
    #[serde(default)]
    pub attribute_name: String,
    #[serde(rename = "FeaturizationPipeline")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub featurization_pipeline: Option<Vec<FeaturizationMethod>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FeaturizationMethod {
    #[serde(rename = "FeaturizationMethodName")]
    #[serde(default)]
    pub featurization_method_name: String,
    #[serde(rename = "FeaturizationMethodParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub featurization_method_parameters: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HyperParameterTuningJobConfig {
    #[serde(rename = "ParameterRanges")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_ranges: Option<ParameterRanges>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ParameterRanges {
    #[serde(rename = "CategoricalParameterRanges")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categorical_parameter_ranges: Option<Vec<CategoricalParameterRange>>,
    #[serde(rename = "ContinuousParameterRanges")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuous_parameter_ranges: Option<Vec<ContinuousParameterRange>>,
    #[serde(rename = "IntegerParameterRanges")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integer_parameter_ranges: Option<Vec<IntegerParameterRange>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CategoricalParameterRange {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Values")]
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContinuousParameterRange {
    #[serde(rename = "MaxValue")]
    #[serde(default)]
    pub max_value: f64,
    #[serde(rename = "MinValue")]
    #[serde(default)]
    pub min_value: f64,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "ScalingType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IntegerParameterRange {
    #[serde(rename = "MaxValue")]
    #[serde(default)]
    pub max_value: i32,
    #[serde(rename = "MinValue")]
    #[serde(default)]
    pub min_value: i32,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "ScalingType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InputDataConfig {
    #[serde(rename = "DatasetGroupArn")]
    #[serde(default)]
    pub dataset_group_arn: String,
    #[serde(rename = "SupplementaryFeatures")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplementary_features: Option<Vec<SupplementaryFeature>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SupplementaryFeature {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePredictorResponse {
    #[serde(rename = "PredictorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predictor_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateWhatIfAnalysisRequest {
    #[serde(rename = "ForecastArn")]
    #[serde(default)]
    pub forecast_arn: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "TimeSeriesSelector")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_series_selector: Option<TimeSeriesSelector>,
    #[serde(rename = "WhatIfAnalysisName")]
    #[serde(default)]
    pub what_if_analysis_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateWhatIfAnalysisResponse {
    #[serde(rename = "WhatIfAnalysisArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub what_if_analysis_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateWhatIfForecastExportRequest {
    #[serde(rename = "Destination")]
    #[serde(default)]
    pub destination: DataDestination,
    #[serde(rename = "Format")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "WhatIfForecastArns")]
    #[serde(default)]
    pub what_if_forecast_arns: Vec<String>,
    #[serde(rename = "WhatIfForecastExportName")]
    #[serde(default)]
    pub what_if_forecast_export_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateWhatIfForecastExportResponse {
    #[serde(rename = "WhatIfForecastExportArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub what_if_forecast_export_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateWhatIfForecastRequest {
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "TimeSeriesReplacementsDataSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_series_replacements_data_source: Option<TimeSeriesReplacementsDataSource>,
    #[serde(rename = "TimeSeriesTransformations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_series_transformations: Option<Vec<TimeSeriesTransformation>>,
    #[serde(rename = "WhatIfAnalysisArn")]
    #[serde(default)]
    pub what_if_analysis_arn: String,
    #[serde(rename = "WhatIfForecastName")]
    #[serde(default)]
    pub what_if_forecast_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TimeSeriesReplacementsDataSource {
    #[serde(rename = "Format")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(rename = "S3Config")]
    #[serde(default)]
    pub s3_config: S3Config,
    #[serde(rename = "Schema")]
    #[serde(default)]
    pub schema: Schema,
    #[serde(rename = "TimestampFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_format: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TimeSeriesTransformation {
    #[serde(rename = "Action")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<Action>,
    #[serde(rename = "TimeSeriesConditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_series_conditions: Option<Vec<TimeSeriesCondition>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Action {
    #[serde(rename = "AttributeName")]
    #[serde(default)]
    pub attribute_name: String,
    #[serde(rename = "Operation")]
    #[serde(default)]
    pub operation: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TimeSeriesCondition {
    #[serde(rename = "AttributeName")]
    #[serde(default)]
    pub attribute_name: String,
    #[serde(rename = "AttributeValue")]
    #[serde(default)]
    pub attribute_value: String,
    #[serde(rename = "Condition")]
    #[serde(default)]
    pub condition: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateWhatIfForecastResponse {
    #[serde(rename = "WhatIfForecastArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub what_if_forecast_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDatasetGroupRequest {
    #[serde(rename = "DatasetGroupArn")]
    #[serde(default)]
    pub dataset_group_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDatasetImportJobRequest {
    #[serde(rename = "DatasetImportJobArn")]
    #[serde(default)]
    pub dataset_import_job_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDatasetRequest {
    #[serde(rename = "DatasetArn")]
    #[serde(default)]
    pub dataset_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteExplainabilityExportRequest {
    #[serde(rename = "ExplainabilityExportArn")]
    #[serde(default)]
    pub explainability_export_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteExplainabilityRequest {
    #[serde(rename = "ExplainabilityArn")]
    #[serde(default)]
    pub explainability_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteForecastExportJobRequest {
    #[serde(rename = "ForecastExportJobArn")]
    #[serde(default)]
    pub forecast_export_job_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteForecastRequest {
    #[serde(rename = "ForecastArn")]
    #[serde(default)]
    pub forecast_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteMonitorRequest {
    #[serde(rename = "MonitorArn")]
    #[serde(default)]
    pub monitor_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePredictorBacktestExportJobRequest {
    #[serde(rename = "PredictorBacktestExportJobArn")]
    #[serde(default)]
    pub predictor_backtest_export_job_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePredictorRequest {
    #[serde(rename = "PredictorArn")]
    #[serde(default)]
    pub predictor_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteResourceTreeRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteWhatIfAnalysisRequest {
    #[serde(rename = "WhatIfAnalysisArn")]
    #[serde(default)]
    pub what_if_analysis_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteWhatIfForecastExportRequest {
    #[serde(rename = "WhatIfForecastExportArn")]
    #[serde(default)]
    pub what_if_forecast_export_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteWhatIfForecastRequest {
    #[serde(rename = "WhatIfForecastArn")]
    #[serde(default)]
    pub what_if_forecast_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAutoPredictorRequest {
    #[serde(rename = "PredictorArn")]
    #[serde(default)]
    pub predictor_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAutoPredictorResponse {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "DataConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_config: Option<DataConfig>,
    #[serde(rename = "DatasetImportJobArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_import_job_arns: Option<Vec<String>>,
    #[serde(rename = "EncryptionConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_config: Option<EncryptionConfig>,
    #[serde(rename = "EstimatedTimeRemainingInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_time_remaining_in_minutes: Option<i64>,
    #[serde(rename = "ExplainabilityInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explainability_info: Option<ExplainabilityInfo>,
    #[serde(rename = "ForecastDimensions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast_dimensions: Option<Vec<String>>,
    #[serde(rename = "ForecastFrequency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast_frequency: Option<String>,
    #[serde(rename = "ForecastHorizon")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast_horizon: Option<i32>,
    #[serde(rename = "ForecastTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast_types: Option<Vec<String>>,
    #[serde(rename = "LastModificationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modification_time: Option<f64>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "MonitorInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor_info: Option<MonitorInfo>,
    #[serde(rename = "OptimizationMetric")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optimization_metric: Option<String>,
    #[serde(rename = "PredictorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predictor_arn: Option<String>,
    #[serde(rename = "PredictorName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predictor_name: Option<String>,
    #[serde(rename = "ReferencePredictorSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_predictor_summary: Option<ReferencePredictorSummary>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TimeAlignmentBoundary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_alignment_boundary: Option<TimeAlignmentBoundary>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExplainabilityInfo {
    #[serde(rename = "ExplainabilityArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explainability_arn: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MonitorInfo {
    #[serde(rename = "MonitorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor_arn: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReferencePredictorSummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDatasetGroupRequest {
    #[serde(rename = "DatasetGroupArn")]
    #[serde(default)]
    pub dataset_group_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDatasetGroupResponse {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "DatasetArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_arns: Option<Vec<String>>,
    #[serde(rename = "DatasetGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_group_arn: Option<String>,
    #[serde(rename = "DatasetGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_group_name: Option<String>,
    #[serde(rename = "Domain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "LastModificationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modification_time: Option<f64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDatasetImportJobRequest {
    #[serde(rename = "DatasetImportJobArn")]
    #[serde(default)]
    pub dataset_import_job_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDatasetImportJobResponse {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "DataSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_size: Option<f64>,
    #[serde(rename = "DataSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source: Option<DataSource>,
    #[serde(rename = "DatasetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_arn: Option<String>,
    #[serde(rename = "DatasetImportJobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_import_job_arn: Option<String>,
    #[serde(rename = "DatasetImportJobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_import_job_name: Option<String>,
    #[serde(rename = "EstimatedTimeRemainingInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_time_remaining_in_minutes: Option<i64>,
    #[serde(rename = "FieldStatistics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_statistics: Option<std::collections::HashMap<String, Statistics>>,
    #[serde(rename = "Format")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(rename = "GeolocationFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geolocation_format: Option<String>,
    #[serde(rename = "ImportMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_mode: Option<String>,
    #[serde(rename = "LastModificationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modification_time: Option<f64>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TimeZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    #[serde(rename = "TimestampFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_format: Option<String>,
    #[serde(rename = "UseGeolocationForTimeZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_geolocation_for_time_zone: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Statistics {
    #[serde(rename = "Avg")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg: Option<f64>,
    #[serde(rename = "Count")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(rename = "CountDistinct")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_distinct: Option<i32>,
    #[serde(rename = "CountDistinctLong")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_distinct_long: Option<i64>,
    #[serde(rename = "CountLong")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_long: Option<i64>,
    #[serde(rename = "CountNan")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_nan: Option<i32>,
    #[serde(rename = "CountNanLong")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_nan_long: Option<i64>,
    #[serde(rename = "CountNull")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_null: Option<i32>,
    #[serde(rename = "CountNullLong")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_null_long: Option<i64>,
    #[serde(rename = "Max")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<String>,
    #[serde(rename = "Min")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<String>,
    #[serde(rename = "Stddev")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stddev: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDatasetRequest {
    #[serde(rename = "DatasetArn")]
    #[serde(default)]
    pub dataset_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDatasetResponse {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "DataFrequency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_frequency: Option<String>,
    #[serde(rename = "DatasetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_arn: Option<String>,
    #[serde(rename = "DatasetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_name: Option<String>,
    #[serde(rename = "DatasetType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_type: Option<String>,
    #[serde(rename = "Domain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "EncryptionConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_config: Option<EncryptionConfig>,
    #[serde(rename = "LastModificationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modification_time: Option<f64>,
    #[serde(rename = "Schema")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<Schema>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeExplainabilityExportRequest {
    #[serde(rename = "ExplainabilityExportArn")]
    #[serde(default)]
    pub explainability_export_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeExplainabilityExportResponse {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "Destination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<DataDestination>,
    #[serde(rename = "ExplainabilityArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explainability_arn: Option<String>,
    #[serde(rename = "ExplainabilityExportArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explainability_export_arn: Option<String>,
    #[serde(rename = "ExplainabilityExportName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explainability_export_name: Option<String>,
    #[serde(rename = "Format")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(rename = "LastModificationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modification_time: Option<f64>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeExplainabilityRequest {
    #[serde(rename = "ExplainabilityArn")]
    #[serde(default)]
    pub explainability_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeExplainabilityResponse {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "DataSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source: Option<DataSource>,
    #[serde(rename = "EnableVisualization")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_visualization: Option<bool>,
    #[serde(rename = "EndDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date_time: Option<String>,
    #[serde(rename = "EstimatedTimeRemainingInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_time_remaining_in_minutes: Option<i64>,
    #[serde(rename = "ExplainabilityArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explainability_arn: Option<String>,
    #[serde(rename = "ExplainabilityConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explainability_config: Option<ExplainabilityConfig>,
    #[serde(rename = "ExplainabilityName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explainability_name: Option<String>,
    #[serde(rename = "LastModificationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modification_time: Option<f64>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "Schema")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<Schema>,
    #[serde(rename = "StartDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date_time: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeForecastExportJobRequest {
    #[serde(rename = "ForecastExportJobArn")]
    #[serde(default)]
    pub forecast_export_job_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeForecastExportJobResponse {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "Destination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<DataDestination>,
    #[serde(rename = "ForecastArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast_arn: Option<String>,
    #[serde(rename = "ForecastExportJobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast_export_job_arn: Option<String>,
    #[serde(rename = "ForecastExportJobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast_export_job_name: Option<String>,
    #[serde(rename = "Format")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(rename = "LastModificationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modification_time: Option<f64>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeForecastRequest {
    #[serde(rename = "ForecastArn")]
    #[serde(default)]
    pub forecast_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeForecastResponse {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "DatasetGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_group_arn: Option<String>,
    #[serde(rename = "EstimatedTimeRemainingInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_time_remaining_in_minutes: Option<i64>,
    #[serde(rename = "ForecastArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast_arn: Option<String>,
    #[serde(rename = "ForecastName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast_name: Option<String>,
    #[serde(rename = "ForecastTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast_types: Option<Vec<String>>,
    #[serde(rename = "LastModificationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modification_time: Option<f64>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "PredictorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predictor_arn: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TimeSeriesSelector")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_series_selector: Option<TimeSeriesSelector>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeMonitorRequest {
    #[serde(rename = "MonitorArn")]
    #[serde(default)]
    pub monitor_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeMonitorResponse {
    #[serde(rename = "Baseline")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline: Option<Baseline>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "EstimatedEvaluationTimeRemainingInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_evaluation_time_remaining_in_minutes: Option<i64>,
    #[serde(rename = "LastEvaluationState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_evaluation_state: Option<String>,
    #[serde(rename = "LastEvaluationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_evaluation_time: Option<f64>,
    #[serde(rename = "LastModificationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modification_time: Option<f64>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "MonitorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor_arn: Option<String>,
    #[serde(rename = "MonitorName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor_name: Option<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Baseline {
    #[serde(rename = "PredictorBaseline")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predictor_baseline: Option<PredictorBaseline>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PredictorBaseline {
    #[serde(rename = "BaselineMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_metrics: Option<Vec<BaselineMetric>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BaselineMetric {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribePredictorBacktestExportJobRequest {
    #[serde(rename = "PredictorBacktestExportJobArn")]
    #[serde(default)]
    pub predictor_backtest_export_job_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribePredictorBacktestExportJobResponse {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "Destination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<DataDestination>,
    #[serde(rename = "Format")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(rename = "LastModificationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modification_time: Option<f64>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "PredictorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predictor_arn: Option<String>,
    #[serde(rename = "PredictorBacktestExportJobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predictor_backtest_export_job_arn: Option<String>,
    #[serde(rename = "PredictorBacktestExportJobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predictor_backtest_export_job_name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribePredictorRequest {
    #[serde(rename = "PredictorArn")]
    #[serde(default)]
    pub predictor_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribePredictorResponse {
    #[serde(rename = "AlgorithmArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm_arn: Option<String>,
    #[serde(rename = "AutoMLAlgorithmArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_m_l_algorithm_arns: Option<Vec<String>>,
    #[serde(rename = "AutoMLOverrideStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_m_l_override_strategy: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "DatasetImportJobArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_import_job_arns: Option<Vec<String>>,
    #[serde(rename = "EncryptionConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_config: Option<EncryptionConfig>,
    #[serde(rename = "EstimatedTimeRemainingInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_time_remaining_in_minutes: Option<i64>,
    #[serde(rename = "EvaluationParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_parameters: Option<EvaluationParameters>,
    #[serde(rename = "FeaturizationConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub featurization_config: Option<FeaturizationConfig>,
    #[serde(rename = "ForecastHorizon")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast_horizon: Option<i32>,
    #[serde(rename = "ForecastTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast_types: Option<Vec<String>>,
    #[serde(rename = "HPOConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h_p_o_config: Option<HyperParameterTuningJobConfig>,
    #[serde(rename = "InputDataConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_data_config: Option<InputDataConfig>,
    #[serde(rename = "IsAutoPredictor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_auto_predictor: Option<bool>,
    #[serde(rename = "LastModificationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modification_time: Option<f64>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "OptimizationMetric")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optimization_metric: Option<String>,
    #[serde(rename = "PerformAutoML")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perform_auto_m_l: Option<bool>,
    #[serde(rename = "PerformHPO")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perform_h_p_o: Option<bool>,
    #[serde(rename = "PredictorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predictor_arn: Option<String>,
    #[serde(rename = "PredictorExecutionDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predictor_execution_details: Option<PredictorExecutionDetails>,
    #[serde(rename = "PredictorName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predictor_name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TrainingParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_parameters: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PredictorExecutionDetails {
    #[serde(rename = "PredictorExecutions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predictor_executions: Option<Vec<PredictorExecution>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PredictorExecution {
    #[serde(rename = "AlgorithmArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm_arn: Option<String>,
    #[serde(rename = "TestWindows")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_windows: Option<Vec<TestWindowSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TestWindowSummary {
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TestWindowEnd")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_window_end: Option<f64>,
    #[serde(rename = "TestWindowStart")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_window_start: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeWhatIfAnalysisRequest {
    #[serde(rename = "WhatIfAnalysisArn")]
    #[serde(default)]
    pub what_if_analysis_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeWhatIfAnalysisResponse {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "EstimatedTimeRemainingInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_time_remaining_in_minutes: Option<i64>,
    #[serde(rename = "ForecastArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast_arn: Option<String>,
    #[serde(rename = "LastModificationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modification_time: Option<f64>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TimeSeriesSelector")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_series_selector: Option<TimeSeriesSelector>,
    #[serde(rename = "WhatIfAnalysisArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub what_if_analysis_arn: Option<String>,
    #[serde(rename = "WhatIfAnalysisName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub what_if_analysis_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeWhatIfForecastExportRequest {
    #[serde(rename = "WhatIfForecastExportArn")]
    #[serde(default)]
    pub what_if_forecast_export_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeWhatIfForecastExportResponse {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "Destination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<DataDestination>,
    #[serde(rename = "EstimatedTimeRemainingInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_time_remaining_in_minutes: Option<i64>,
    #[serde(rename = "Format")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(rename = "LastModificationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modification_time: Option<f64>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "WhatIfForecastArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub what_if_forecast_arns: Option<Vec<String>>,
    #[serde(rename = "WhatIfForecastExportArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub what_if_forecast_export_arn: Option<String>,
    #[serde(rename = "WhatIfForecastExportName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub what_if_forecast_export_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeWhatIfForecastRequest {
    #[serde(rename = "WhatIfForecastArn")]
    #[serde(default)]
    pub what_if_forecast_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeWhatIfForecastResponse {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "EstimatedTimeRemainingInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_time_remaining_in_minutes: Option<i64>,
    #[serde(rename = "ForecastTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast_types: Option<Vec<String>>,
    #[serde(rename = "LastModificationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modification_time: Option<f64>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TimeSeriesReplacementsDataSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_series_replacements_data_source: Option<TimeSeriesReplacementsDataSource>,
    #[serde(rename = "TimeSeriesTransformations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_series_transformations: Option<Vec<TimeSeriesTransformation>>,
    #[serde(rename = "WhatIfAnalysisArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub what_if_analysis_arn: Option<String>,
    #[serde(rename = "WhatIfForecastArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub what_if_forecast_arn: Option<String>,
    #[serde(rename = "WhatIfForecastName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub what_if_forecast_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAccuracyMetricsRequest {
    #[serde(rename = "PredictorArn")]
    #[serde(default)]
    pub predictor_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAccuracyMetricsResponse {
    #[serde(rename = "AutoMLOverrideStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_m_l_override_strategy: Option<String>,
    #[serde(rename = "IsAutoPredictor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_auto_predictor: Option<bool>,
    #[serde(rename = "OptimizationMetric")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optimization_metric: Option<String>,
    #[serde(rename = "PredictorEvaluationResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predictor_evaluation_results: Option<Vec<EvaluationResult>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationResult {
    #[serde(rename = "AlgorithmArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm_arn: Option<String>,
    #[serde(rename = "TestWindows")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_windows: Option<Vec<WindowSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WindowSummary {
    #[serde(rename = "EvaluationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_type: Option<String>,
    #[serde(rename = "ItemCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_count: Option<i32>,
    #[serde(rename = "Metrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Metrics>,
    #[serde(rename = "TestWindowEnd")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_window_end: Option<f64>,
    #[serde(rename = "TestWindowStart")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_window_start: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Metrics {
    #[serde(rename = "AverageWeightedQuantileLoss")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_weighted_quantile_loss: Option<f64>,
    #[serde(rename = "ErrorMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_metrics: Option<Vec<ErrorMetric>>,
    #[serde(rename = "RMSE")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r_m_s_e: Option<f64>,
    #[serde(rename = "WeightedQuantileLosses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weighted_quantile_losses: Option<Vec<WeightedQuantileLoss>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ErrorMetric {
    #[serde(rename = "ForecastType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast_type: Option<String>,
    #[serde(rename = "MAPE")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_a_p_e: Option<f64>,
    #[serde(rename = "MASE")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_a_s_e: Option<f64>,
    #[serde(rename = "RMSE")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r_m_s_e: Option<f64>,
    #[serde(rename = "WAPE")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w_a_p_e: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WeightedQuantileLoss {
    #[serde(rename = "LossValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loss_value: Option<f64>,
    #[serde(rename = "Quantile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantile: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDatasetGroupsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDatasetGroupsResponse {
    #[serde(rename = "DatasetGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_groups: Option<Vec<DatasetGroupSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DatasetGroupSummary {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "DatasetGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_group_arn: Option<String>,
    #[serde(rename = "DatasetGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_group_name: Option<String>,
    #[serde(rename = "LastModificationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modification_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDatasetImportJobsRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Filter {
    #[serde(rename = "Condition")]
    #[serde(default)]
    pub condition: String,
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDatasetImportJobsResponse {
    #[serde(rename = "DatasetImportJobs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_import_jobs: Option<Vec<DatasetImportJobSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DatasetImportJobSummary {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "DataSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source: Option<DataSource>,
    #[serde(rename = "DatasetImportJobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_import_job_arn: Option<String>,
    #[serde(rename = "DatasetImportJobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_import_job_name: Option<String>,
    #[serde(rename = "ImportMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_mode: Option<String>,
    #[serde(rename = "LastModificationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modification_time: Option<f64>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDatasetsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDatasetsResponse {
    #[serde(rename = "Datasets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datasets: Option<Vec<DatasetSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DatasetSummary {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "DatasetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_arn: Option<String>,
    #[serde(rename = "DatasetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_name: Option<String>,
    #[serde(rename = "DatasetType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_type: Option<String>,
    #[serde(rename = "Domain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "LastModificationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modification_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListExplainabilitiesRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListExplainabilitiesResponse {
    #[serde(rename = "Explainabilities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explainabilities: Option<Vec<ExplainabilitySummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExplainabilitySummary {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "ExplainabilityArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explainability_arn: Option<String>,
    #[serde(rename = "ExplainabilityConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explainability_config: Option<ExplainabilityConfig>,
    #[serde(rename = "ExplainabilityName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explainability_name: Option<String>,
    #[serde(rename = "LastModificationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modification_time: Option<f64>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListExplainabilityExportsRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListExplainabilityExportsResponse {
    #[serde(rename = "ExplainabilityExports")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explainability_exports: Option<Vec<ExplainabilityExportSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExplainabilityExportSummary {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "Destination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<DataDestination>,
    #[serde(rename = "ExplainabilityExportArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explainability_export_arn: Option<String>,
    #[serde(rename = "ExplainabilityExportName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explainability_export_name: Option<String>,
    #[serde(rename = "LastModificationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modification_time: Option<f64>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListForecastExportJobsRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListForecastExportJobsResponse {
    #[serde(rename = "ForecastExportJobs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast_export_jobs: Option<Vec<ForecastExportJobSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ForecastExportJobSummary {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "Destination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<DataDestination>,
    #[serde(rename = "ForecastExportJobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast_export_job_arn: Option<String>,
    #[serde(rename = "ForecastExportJobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast_export_job_name: Option<String>,
    #[serde(rename = "LastModificationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modification_time: Option<f64>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListForecastsRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListForecastsResponse {
    #[serde(rename = "Forecasts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecasts: Option<Vec<ForecastSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ForecastSummary {
    #[serde(rename = "CreatedUsingAutoPredictor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_using_auto_predictor: Option<bool>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "DatasetGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_group_arn: Option<String>,
    #[serde(rename = "ForecastArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast_arn: Option<String>,
    #[serde(rename = "ForecastName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast_name: Option<String>,
    #[serde(rename = "LastModificationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modification_time: Option<f64>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "PredictorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predictor_arn: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListMonitorEvaluationsRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "MonitorArn")]
    #[serde(default)]
    pub monitor_arn: String,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListMonitorEvaluationsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PredictorMonitorEvaluations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predictor_monitor_evaluations: Option<Vec<PredictorMonitorEvaluation>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PredictorMonitorEvaluation {
    #[serde(rename = "EvaluationState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_state: Option<String>,
    #[serde(rename = "EvaluationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_time: Option<f64>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "MetricResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_results: Option<Vec<MetricResult>>,
    #[serde(rename = "MonitorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor_arn: Option<String>,
    #[serde(rename = "MonitorDataSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor_data_source: Option<MonitorDataSource>,
    #[serde(rename = "NumItemsEvaluated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_items_evaluated: Option<i64>,
    #[serde(rename = "PredictorEvent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predictor_event: Option<PredictorEvent>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "WindowEndDatetime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_end_datetime: Option<f64>,
    #[serde(rename = "WindowStartDatetime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_start_datetime: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MetricResult {
    #[serde(rename = "MetricName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    #[serde(rename = "MetricValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_value: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MonitorDataSource {
    #[serde(rename = "DatasetImportJobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_import_job_arn: Option<String>,
    #[serde(rename = "ForecastArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast_arn: Option<String>,
    #[serde(rename = "PredictorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predictor_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PredictorEvent {
    #[serde(rename = "Datetime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datetime: Option<f64>,
    #[serde(rename = "Detail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListMonitorsRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListMonitorsResponse {
    #[serde(rename = "Monitors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitors: Option<Vec<MonitorSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MonitorSummary {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "LastModificationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modification_time: Option<f64>,
    #[serde(rename = "MonitorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor_arn: Option<String>,
    #[serde(rename = "MonitorName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor_name: Option<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPredictorBacktestExportJobsRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPredictorBacktestExportJobsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PredictorBacktestExportJobs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predictor_backtest_export_jobs: Option<Vec<PredictorBacktestExportJobSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PredictorBacktestExportJobSummary {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "Destination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<DataDestination>,
    #[serde(rename = "LastModificationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modification_time: Option<f64>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "PredictorBacktestExportJobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predictor_backtest_export_job_arn: Option<String>,
    #[serde(rename = "PredictorBacktestExportJobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predictor_backtest_export_job_name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPredictorsRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPredictorsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Predictors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predictors: Option<Vec<PredictorSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PredictorSummary {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "DatasetGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_group_arn: Option<String>,
    #[serde(rename = "IsAutoPredictor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_auto_predictor: Option<bool>,
    #[serde(rename = "LastModificationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modification_time: Option<f64>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "PredictorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predictor_arn: Option<String>,
    #[serde(rename = "PredictorName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predictor_name: Option<String>,
    #[serde(rename = "ReferencePredictorSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_predictor_summary: Option<ReferencePredictorSummary>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListWhatIfAnalysesRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListWhatIfAnalysesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "WhatIfAnalyses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub what_if_analyses: Option<Vec<WhatIfAnalysisSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WhatIfAnalysisSummary {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "ForecastArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast_arn: Option<String>,
    #[serde(rename = "LastModificationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modification_time: Option<f64>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "WhatIfAnalysisArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub what_if_analysis_arn: Option<String>,
    #[serde(rename = "WhatIfAnalysisName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub what_if_analysis_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListWhatIfForecastExportsRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListWhatIfForecastExportsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "WhatIfForecastExports")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub what_if_forecast_exports: Option<Vec<WhatIfForecastExportSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WhatIfForecastExportSummary {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "Destination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<DataDestination>,
    #[serde(rename = "LastModificationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modification_time: Option<f64>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "WhatIfForecastArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub what_if_forecast_arns: Option<Vec<String>>,
    #[serde(rename = "WhatIfForecastExportArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub what_if_forecast_export_arn: Option<String>,
    #[serde(rename = "WhatIfForecastExportName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub what_if_forecast_export_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListWhatIfForecastsRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListWhatIfForecastsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "WhatIfForecasts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub what_if_forecasts: Option<Vec<WhatIfForecastSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WhatIfForecastSummary {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "LastModificationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modification_time: Option<f64>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "WhatIfAnalysisArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub what_if_analysis_arn: Option<String>,
    #[serde(rename = "WhatIfForecastArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub what_if_forecast_arn: Option<String>,
    #[serde(rename = "WhatIfForecastName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub what_if_forecast_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResumeResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDatasetGroupRequest {
    #[serde(rename = "DatasetArns")]
    #[serde(default)]
    pub dataset_arns: Vec<String>,
    #[serde(rename = "DatasetGroupArn")]
    #[serde(default)]
    pub dataset_group_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDatasetGroupResponse {}
