//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-kinesisanalyticsv2

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddApplicationCloudWatchLoggingOptionRequest {
    #[serde(rename = "ApplicationName")]
    #[serde(default)]
    pub application_name: String,
    #[serde(rename = "CloudWatchLoggingOption")]
    #[serde(default)]
    pub cloud_watch_logging_option: CloudWatchLoggingOption,
    #[serde(rename = "ConditionalToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional_token: Option<String>,
    #[serde(rename = "CurrentApplicationVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_application_version_id: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CloudWatchLoggingOption {
    #[serde(rename = "LogStreamARN")]
    #[serde(default)]
    pub log_stream_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddApplicationCloudWatchLoggingOptionResponse {
    #[serde(rename = "ApplicationARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_a_r_n: Option<String>,
    #[serde(rename = "ApplicationVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_version_id: Option<i64>,
    #[serde(rename = "CloudWatchLoggingOptionDescriptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_option_descriptions: Option<Vec<CloudWatchLoggingOptionDescription>>,
    #[serde(rename = "OperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CloudWatchLoggingOptionDescription {
    #[serde(rename = "CloudWatchLoggingOptionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_option_id: Option<String>,
    #[serde(rename = "LogStreamARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_stream_a_r_n: Option<String>,
    #[serde(rename = "RoleARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_a_r_n: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddApplicationInputProcessingConfigurationRequest {
    #[serde(rename = "ApplicationName")]
    #[serde(default)]
    pub application_name: String,
    #[serde(rename = "CurrentApplicationVersionId")]
    #[serde(default)]
    pub current_application_version_id: i64,
    #[serde(rename = "InputId")]
    #[serde(default)]
    pub input_id: String,
    #[serde(rename = "InputProcessingConfiguration")]
    #[serde(default)]
    pub input_processing_configuration: InputProcessingConfiguration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InputProcessingConfiguration {
    #[serde(rename = "InputLambdaProcessor")]
    #[serde(default)]
    pub input_lambda_processor: InputLambdaProcessor,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InputLambdaProcessor {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddApplicationInputProcessingConfigurationResponse {
    #[serde(rename = "ApplicationARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_a_r_n: Option<String>,
    #[serde(rename = "ApplicationVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_version_id: Option<i64>,
    #[serde(rename = "InputId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_id: Option<String>,
    #[serde(rename = "InputProcessingConfigurationDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_processing_configuration_description: Option<InputProcessingConfigurationDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InputProcessingConfigurationDescription {
    #[serde(rename = "InputLambdaProcessorDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_lambda_processor_description: Option<InputLambdaProcessorDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InputLambdaProcessorDescription {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_a_r_n: Option<String>,
    #[serde(rename = "RoleARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_a_r_n: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddApplicationInputRequest {
    #[serde(rename = "ApplicationName")]
    #[serde(default)]
    pub application_name: String,
    #[serde(rename = "CurrentApplicationVersionId")]
    #[serde(default)]
    pub current_application_version_id: i64,
    #[serde(rename = "Input")]
    #[serde(default)]
    pub input: Input,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Input {
    #[serde(rename = "InputParallelism")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_parallelism: Option<InputParallelism>,
    #[serde(rename = "InputProcessingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_processing_configuration: Option<InputProcessingConfiguration>,
    #[serde(rename = "InputSchema")]
    #[serde(default)]
    pub input_schema: SourceSchema,
    #[serde(rename = "KinesisFirehoseInput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_firehose_input: Option<KinesisFirehoseInput>,
    #[serde(rename = "KinesisStreamsInput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_streams_input: Option<KinesisStreamsInput>,
    #[serde(rename = "NamePrefix")]
    #[serde(default)]
    pub name_prefix: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InputParallelism {
    #[serde(rename = "Count")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SourceSchema {
    #[serde(rename = "RecordColumns")]
    #[serde(default)]
    pub record_columns: Vec<RecordColumn>,
    #[serde(rename = "RecordEncoding")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_encoding: Option<String>,
    #[serde(rename = "RecordFormat")]
    #[serde(default)]
    pub record_format: RecordFormat,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecordColumn {
    #[serde(rename = "Mapping")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mapping: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "SqlType")]
    #[serde(default)]
    pub sql_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecordFormat {
    #[serde(rename = "MappingParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mapping_parameters: Option<MappingParameters>,
    #[serde(rename = "RecordFormatType")]
    #[serde(default)]
    pub record_format_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MappingParameters {
    #[serde(rename = "CSVMappingParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_s_v_mapping_parameters: Option<CSVMappingParameters>,
    #[serde(rename = "JSONMappingParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub j_s_o_n_mapping_parameters: Option<JSONMappingParameters>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CSVMappingParameters {
    #[serde(rename = "RecordColumnDelimiter")]
    #[serde(default)]
    pub record_column_delimiter: String,
    #[serde(rename = "RecordRowDelimiter")]
    #[serde(default)]
    pub record_row_delimiter: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JSONMappingParameters {
    #[serde(rename = "RecordRowPath")]
    #[serde(default)]
    pub record_row_path: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KinesisFirehoseInput {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KinesisStreamsInput {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddApplicationInputResponse {
    #[serde(rename = "ApplicationARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_a_r_n: Option<String>,
    #[serde(rename = "ApplicationVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_version_id: Option<i64>,
    #[serde(rename = "InputDescriptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_descriptions: Option<Vec<InputDescription>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InputDescription {
    #[serde(rename = "InAppStreamNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_app_stream_names: Option<Vec<String>>,
    #[serde(rename = "InputId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_id: Option<String>,
    #[serde(rename = "InputParallelism")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_parallelism: Option<InputParallelism>,
    #[serde(rename = "InputProcessingConfigurationDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_processing_configuration_description: Option<InputProcessingConfigurationDescription>,
    #[serde(rename = "InputSchema")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_schema: Option<SourceSchema>,
    #[serde(rename = "InputStartingPositionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_starting_position_configuration: Option<InputStartingPositionConfiguration>,
    #[serde(rename = "KinesisFirehoseInputDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_firehose_input_description: Option<KinesisFirehoseInputDescription>,
    #[serde(rename = "KinesisStreamsInputDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_streams_input_description: Option<KinesisStreamsInputDescription>,
    #[serde(rename = "NamePrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InputStartingPositionConfiguration {
    #[serde(rename = "InputStartingPosition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_starting_position: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KinesisFirehoseInputDescription {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_a_r_n: Option<String>,
    #[serde(rename = "RoleARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_a_r_n: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KinesisStreamsInputDescription {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_a_r_n: Option<String>,
    #[serde(rename = "RoleARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_a_r_n: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddApplicationOutputRequest {
    #[serde(rename = "ApplicationName")]
    #[serde(default)]
    pub application_name: String,
    #[serde(rename = "CurrentApplicationVersionId")]
    #[serde(default)]
    pub current_application_version_id: i64,
    #[serde(rename = "Output")]
    #[serde(default)]
    pub output: Output,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Output {
    #[serde(rename = "DestinationSchema")]
    #[serde(default)]
    pub destination_schema: DestinationSchema,
    #[serde(rename = "KinesisFirehoseOutput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_firehose_output: Option<KinesisFirehoseOutput>,
    #[serde(rename = "KinesisStreamsOutput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_streams_output: Option<KinesisStreamsOutput>,
    #[serde(rename = "LambdaOutput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_output: Option<LambdaOutput>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DestinationSchema {
    #[serde(rename = "RecordFormatType")]
    #[serde(default)]
    pub record_format_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KinesisFirehoseOutput {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KinesisStreamsOutput {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LambdaOutput {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddApplicationOutputResponse {
    #[serde(rename = "ApplicationARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_a_r_n: Option<String>,
    #[serde(rename = "ApplicationVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_version_id: Option<i64>,
    #[serde(rename = "OutputDescriptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_descriptions: Option<Vec<OutputDescription>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OutputDescription {
    #[serde(rename = "DestinationSchema")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_schema: Option<DestinationSchema>,
    #[serde(rename = "KinesisFirehoseOutputDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_firehose_output_description: Option<KinesisFirehoseOutputDescription>,
    #[serde(rename = "KinesisStreamsOutputDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_streams_output_description: Option<KinesisStreamsOutputDescription>,
    #[serde(rename = "LambdaOutputDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_output_description: Option<LambdaOutputDescription>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "OutputId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KinesisFirehoseOutputDescription {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_a_r_n: Option<String>,
    #[serde(rename = "RoleARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_a_r_n: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KinesisStreamsOutputDescription {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_a_r_n: Option<String>,
    #[serde(rename = "RoleARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_a_r_n: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LambdaOutputDescription {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_a_r_n: Option<String>,
    #[serde(rename = "RoleARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_a_r_n: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddApplicationReferenceDataSourceRequest {
    #[serde(rename = "ApplicationName")]
    #[serde(default)]
    pub application_name: String,
    #[serde(rename = "CurrentApplicationVersionId")]
    #[serde(default)]
    pub current_application_version_id: i64,
    #[serde(rename = "ReferenceDataSource")]
    #[serde(default)]
    pub reference_data_source: ReferenceDataSource,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReferenceDataSource {
    #[serde(rename = "ReferenceSchema")]
    #[serde(default)]
    pub reference_schema: SourceSchema,
    #[serde(rename = "S3ReferenceDataSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_reference_data_source: Option<S3ReferenceDataSource>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3ReferenceDataSource {
    #[serde(rename = "BucketARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_a_r_n: Option<String>,
    #[serde(rename = "FileKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_key: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddApplicationReferenceDataSourceResponse {
    #[serde(rename = "ApplicationARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_a_r_n: Option<String>,
    #[serde(rename = "ApplicationVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_version_id: Option<i64>,
    #[serde(rename = "ReferenceDataSourceDescriptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_data_source_descriptions: Option<Vec<ReferenceDataSourceDescription>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReferenceDataSourceDescription {
    #[serde(rename = "ReferenceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
    #[serde(rename = "ReferenceSchema")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_schema: Option<SourceSchema>,
    #[serde(rename = "S3ReferenceDataSourceDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_reference_data_source_description: Option<S3ReferenceDataSourceDescription>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3ReferenceDataSourceDescription {
    #[serde(rename = "BucketARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_a_r_n: Option<String>,
    #[serde(rename = "FileKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_key: Option<String>,
    #[serde(rename = "ReferenceRoleARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_role_a_r_n: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddApplicationVpcConfigurationRequest {
    #[serde(rename = "ApplicationName")]
    #[serde(default)]
    pub application_name: String,
    #[serde(rename = "ConditionalToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional_token: Option<String>,
    #[serde(rename = "CurrentApplicationVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_application_version_id: Option<i64>,
    #[serde(rename = "VpcConfiguration")]
    #[serde(default)]
    pub vpc_configuration: VpcConfiguration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VpcConfiguration {
    #[serde(rename = "SecurityGroupIds")]
    #[serde(default)]
    pub security_group_ids: Vec<String>,
    #[serde(rename = "SubnetIds")]
    #[serde(default)]
    pub subnet_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddApplicationVpcConfigurationResponse {
    #[serde(rename = "ApplicationARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_a_r_n: Option<String>,
    #[serde(rename = "ApplicationVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_version_id: Option<i64>,
    #[serde(rename = "OperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    #[serde(rename = "VpcConfigurationDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_configuration_description: Option<VpcConfigurationDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VpcConfigurationDescription {
    #[serde(rename = "SecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(rename = "SubnetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    #[serde(rename = "VpcConfigurationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_configuration_id: Option<String>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateApplicationPresignedUrlRequest {
    #[serde(rename = "ApplicationName")]
    #[serde(default)]
    pub application_name: String,
    #[serde(rename = "SessionExpirationDurationInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_expiration_duration_in_seconds: Option<i64>,
    #[serde(rename = "UrlType")]
    #[serde(default)]
    pub url_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateApplicationPresignedUrlResponse {
    #[serde(rename = "AuthorizedUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateApplicationRequest {
    #[serde(rename = "ApplicationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_configuration: Option<ApplicationConfiguration>,
    #[serde(rename = "ApplicationDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_description: Option<String>,
    #[serde(rename = "ApplicationMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_mode: Option<String>,
    #[serde(rename = "ApplicationName")]
    #[serde(default)]
    pub application_name: String,
    #[serde(rename = "CloudWatchLoggingOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_options: Option<Vec<CloudWatchLoggingOption>>,
    #[serde(rename = "RuntimeEnvironment")]
    #[serde(default)]
    pub runtime_environment: String,
    #[serde(rename = "ServiceExecutionRole")]
    #[serde(default)]
    pub service_execution_role: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApplicationConfiguration {
    #[serde(rename = "ApplicationCodeConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_code_configuration: Option<ApplicationCodeConfiguration>,
    #[serde(rename = "ApplicationEncryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_encryption_configuration: Option<ApplicationEncryptionConfiguration>,
    #[serde(rename = "ApplicationSnapshotConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_snapshot_configuration: Option<ApplicationSnapshotConfiguration>,
    #[serde(rename = "ApplicationSystemRollbackConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_system_rollback_configuration: Option<ApplicationSystemRollbackConfiguration>,
    #[serde(rename = "EnvironmentProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_properties: Option<EnvironmentProperties>,
    #[serde(rename = "FlinkApplicationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flink_application_configuration: Option<FlinkApplicationConfiguration>,
    #[serde(rename = "SqlApplicationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sql_application_configuration: Option<SqlApplicationConfiguration>,
    #[serde(rename = "VpcConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_configurations: Option<Vec<VpcConfiguration>>,
    #[serde(rename = "ZeppelinApplicationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zeppelin_application_configuration: Option<ZeppelinApplicationConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApplicationCodeConfiguration {
    #[serde(rename = "CodeContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_content: Option<CodeContent>,
    #[serde(rename = "CodeContentType")]
    #[serde(default)]
    pub code_content_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CodeContent {
    #[serde(rename = "S3ContentLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_content_location: Option<S3ContentLocation>,
    #[serde(rename = "TextContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_content: Option<String>,
    #[serde(rename = "ZipFileContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip_file_content: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3ContentLocation {
    #[serde(rename = "BucketARN")]
    #[serde(default)]
    pub bucket_a_r_n: String,
    #[serde(rename = "FileKey")]
    #[serde(default)]
    pub file_key: String,
    #[serde(rename = "ObjectVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApplicationEncryptionConfiguration {
    #[serde(rename = "KeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    #[serde(rename = "KeyType")]
    #[serde(default)]
    pub key_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApplicationSnapshotConfiguration {
    #[serde(rename = "SnapshotsEnabled")]
    #[serde(default)]
    pub snapshots_enabled: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApplicationSystemRollbackConfiguration {
    #[serde(rename = "RollbackEnabled")]
    #[serde(default)]
    pub rollback_enabled: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnvironmentProperties {
    #[serde(rename = "PropertyGroups")]
    #[serde(default)]
    pub property_groups: Vec<PropertyGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PropertyGroup {
    #[serde(rename = "PropertyGroupId")]
    #[serde(default)]
    pub property_group_id: String,
    #[serde(rename = "PropertyMap")]
    #[serde(default)]
    pub property_map: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FlinkApplicationConfiguration {
    #[serde(rename = "CheckpointConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkpoint_configuration: Option<CheckpointConfiguration>,
    #[serde(rename = "MonitoringConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_configuration: Option<MonitoringConfiguration>,
    #[serde(rename = "ParallelismConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallelism_configuration: Option<ParallelismConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CheckpointConfiguration {
    #[serde(rename = "CheckpointInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkpoint_interval: Option<i64>,
    #[serde(rename = "CheckpointingEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkpointing_enabled: Option<bool>,
    #[serde(rename = "ConfigurationType")]
    #[serde(default)]
    pub configuration_type: String,
    #[serde(rename = "MinPauseBetweenCheckpoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_pause_between_checkpoints: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MonitoringConfiguration {
    #[serde(rename = "ConfigurationType")]
    #[serde(default)]
    pub configuration_type: String,
    #[serde(rename = "LogLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_level: Option<String>,
    #[serde(rename = "MetricsLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics_level: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ParallelismConfiguration {
    #[serde(rename = "AutoScalingEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_enabled: Option<bool>,
    #[serde(rename = "ConfigurationType")]
    #[serde(default)]
    pub configuration_type: String,
    #[serde(rename = "Parallelism")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallelism: Option<i32>,
    #[serde(rename = "ParallelismPerKPU")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallelism_per_k_p_u: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SqlApplicationConfiguration {
    #[serde(rename = "Inputs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<Input>>,
    #[serde(rename = "Outputs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<Output>>,
    #[serde(rename = "ReferenceDataSources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_data_sources: Option<Vec<ReferenceDataSource>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ZeppelinApplicationConfiguration {
    #[serde(rename = "CatalogConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_configuration: Option<CatalogConfiguration>,
    #[serde(rename = "CustomArtifactsConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_artifacts_configuration: Option<Vec<CustomArtifactConfiguration>>,
    #[serde(rename = "DeployAsApplicationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deploy_as_application_configuration: Option<DeployAsApplicationConfiguration>,
    #[serde(rename = "MonitoringConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_configuration: Option<ZeppelinMonitoringConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CatalogConfiguration {
    #[serde(rename = "GlueDataCatalogConfiguration")]
    #[serde(default)]
    pub glue_data_catalog_configuration: GlueDataCatalogConfiguration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GlueDataCatalogConfiguration {
    #[serde(rename = "DatabaseARN")]
    #[serde(default)]
    pub database_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomArtifactConfiguration {
    #[serde(rename = "ArtifactType")]
    #[serde(default)]
    pub artifact_type: String,
    #[serde(rename = "MavenReference")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maven_reference: Option<MavenReference>,
    #[serde(rename = "S3ContentLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_content_location: Option<S3ContentLocation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MavenReference {
    #[serde(rename = "ArtifactId")]
    #[serde(default)]
    pub artifact_id: String,
    #[serde(rename = "GroupId")]
    #[serde(default)]
    pub group_id: String,
    #[serde(rename = "Version")]
    #[serde(default)]
    pub version: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeployAsApplicationConfiguration {
    #[serde(rename = "S3ContentLocation")]
    #[serde(default)]
    pub s3_content_location: S3ContentBaseLocation,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3ContentBaseLocation {
    #[serde(rename = "BasePath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_path: Option<String>,
    #[serde(rename = "BucketARN")]
    #[serde(default)]
    pub bucket_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ZeppelinMonitoringConfiguration {
    #[serde(rename = "LogLevel")]
    #[serde(default)]
    pub log_level: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Tag {
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateApplicationResponse {
    #[serde(rename = "ApplicationDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_detail: Option<ApplicationDetail>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApplicationDetail {
    #[serde(rename = "ApplicationARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_a_r_n: Option<String>,
    #[serde(rename = "ApplicationConfigurationDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_configuration_description: Option<ApplicationConfigurationDescription>,
    #[serde(rename = "ApplicationDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_description: Option<String>,
    #[serde(rename = "ApplicationMaintenanceConfigurationDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_maintenance_configuration_description:
        Option<ApplicationMaintenanceConfigurationDescription>,
    #[serde(rename = "ApplicationMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_mode: Option<String>,
    #[serde(rename = "ApplicationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    #[serde(rename = "ApplicationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_status: Option<String>,
    #[serde(rename = "ApplicationVersionCreateTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_version_create_timestamp: Option<f64>,
    #[serde(rename = "ApplicationVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_version_id: Option<i64>,
    #[serde(rename = "ApplicationVersionRolledBackFrom")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_version_rolled_back_from: Option<i64>,
    #[serde(rename = "ApplicationVersionRolledBackTo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_version_rolled_back_to: Option<i64>,
    #[serde(rename = "ApplicationVersionUpdatedFrom")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_version_updated_from: Option<i64>,
    #[serde(rename = "CloudWatchLoggingOptionDescriptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_option_descriptions: Option<Vec<CloudWatchLoggingOptionDescription>>,
    #[serde(rename = "ConditionalToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional_token: Option<String>,
    #[serde(rename = "CreateTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_timestamp: Option<f64>,
    #[serde(rename = "LastUpdateTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_timestamp: Option<f64>,
    #[serde(rename = "RuntimeEnvironment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_environment: Option<String>,
    #[serde(rename = "ServiceExecutionRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_execution_role: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApplicationConfigurationDescription {
    #[serde(rename = "ApplicationCodeConfigurationDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_code_configuration_description: Option<ApplicationCodeConfigurationDescription>,
    #[serde(rename = "ApplicationEncryptionConfigurationDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_encryption_configuration_description:
        Option<ApplicationEncryptionConfigurationDescription>,
    #[serde(rename = "ApplicationSnapshotConfigurationDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_snapshot_configuration_description:
        Option<ApplicationSnapshotConfigurationDescription>,
    #[serde(rename = "ApplicationSystemRollbackConfigurationDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_system_rollback_configuration_description:
        Option<ApplicationSystemRollbackConfigurationDescription>,
    #[serde(rename = "EnvironmentPropertyDescriptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_property_descriptions: Option<EnvironmentPropertyDescriptions>,
    #[serde(rename = "FlinkApplicationConfigurationDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flink_application_configuration_description:
        Option<FlinkApplicationConfigurationDescription>,
    #[serde(rename = "RunConfigurationDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_configuration_description: Option<RunConfigurationDescription>,
    #[serde(rename = "SqlApplicationConfigurationDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sql_application_configuration_description: Option<SqlApplicationConfigurationDescription>,
    #[serde(rename = "VpcConfigurationDescriptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_configuration_descriptions: Option<Vec<VpcConfigurationDescription>>,
    #[serde(rename = "ZeppelinApplicationConfigurationDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zeppelin_application_configuration_description:
        Option<ZeppelinApplicationConfigurationDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApplicationCodeConfigurationDescription {
    #[serde(rename = "CodeContentDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_content_description: Option<CodeContentDescription>,
    #[serde(rename = "CodeContentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_content_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CodeContentDescription {
    #[serde(rename = "CodeMD5")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_m_d5: Option<String>,
    #[serde(rename = "CodeSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_size: Option<i64>,
    #[serde(rename = "S3ApplicationCodeLocationDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_application_code_location_description: Option<S3ApplicationCodeLocationDescription>,
    #[serde(rename = "TextContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_content: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3ApplicationCodeLocationDescription {
    #[serde(rename = "BucketARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_a_r_n: Option<String>,
    #[serde(rename = "FileKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_key: Option<String>,
    #[serde(rename = "ObjectVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApplicationEncryptionConfigurationDescription {
    #[serde(rename = "KeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    #[serde(rename = "KeyType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApplicationSnapshotConfigurationDescription {
    #[serde(rename = "SnapshotsEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshots_enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApplicationSystemRollbackConfigurationDescription {
    #[serde(rename = "RollbackEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollback_enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnvironmentPropertyDescriptions {
    #[serde(rename = "PropertyGroupDescriptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_group_descriptions: Option<Vec<PropertyGroup>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FlinkApplicationConfigurationDescription {
    #[serde(rename = "CheckpointConfigurationDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkpoint_configuration_description: Option<CheckpointConfigurationDescription>,
    #[serde(rename = "JobPlanDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_plan_description: Option<String>,
    #[serde(rename = "MonitoringConfigurationDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_configuration_description: Option<MonitoringConfigurationDescription>,
    #[serde(rename = "ParallelismConfigurationDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallelism_configuration_description: Option<ParallelismConfigurationDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CheckpointConfigurationDescription {
    #[serde(rename = "CheckpointInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkpoint_interval: Option<i64>,
    #[serde(rename = "CheckpointingEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkpointing_enabled: Option<bool>,
    #[serde(rename = "ConfigurationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_type: Option<String>,
    #[serde(rename = "MinPauseBetweenCheckpoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_pause_between_checkpoints: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MonitoringConfigurationDescription {
    #[serde(rename = "ConfigurationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_type: Option<String>,
    #[serde(rename = "LogLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_level: Option<String>,
    #[serde(rename = "MetricsLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics_level: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ParallelismConfigurationDescription {
    #[serde(rename = "AutoScalingEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_enabled: Option<bool>,
    #[serde(rename = "ConfigurationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_type: Option<String>,
    #[serde(rename = "CurrentParallelism")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_parallelism: Option<i32>,
    #[serde(rename = "Parallelism")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallelism: Option<i32>,
    #[serde(rename = "ParallelismPerKPU")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallelism_per_k_p_u: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RunConfigurationDescription {
    #[serde(rename = "ApplicationRestoreConfigurationDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_restore_configuration_description: Option<ApplicationRestoreConfiguration>,
    #[serde(rename = "FlinkRunConfigurationDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flink_run_configuration_description: Option<FlinkRunConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApplicationRestoreConfiguration {
    #[serde(rename = "ApplicationRestoreType")]
    #[serde(default)]
    pub application_restore_type: String,
    #[serde(rename = "SnapshotName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FlinkRunConfiguration {
    #[serde(rename = "AllowNonRestoredState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_non_restored_state: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SqlApplicationConfigurationDescription {
    #[serde(rename = "InputDescriptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_descriptions: Option<Vec<InputDescription>>,
    #[serde(rename = "OutputDescriptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_descriptions: Option<Vec<OutputDescription>>,
    #[serde(rename = "ReferenceDataSourceDescriptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_data_source_descriptions: Option<Vec<ReferenceDataSourceDescription>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ZeppelinApplicationConfigurationDescription {
    #[serde(rename = "CatalogConfigurationDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_configuration_description: Option<CatalogConfigurationDescription>,
    #[serde(rename = "CustomArtifactsConfigurationDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_artifacts_configuration_description:
        Option<Vec<CustomArtifactConfigurationDescription>>,
    #[serde(rename = "DeployAsApplicationConfigurationDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deploy_as_application_configuration_description:
        Option<DeployAsApplicationConfigurationDescription>,
    #[serde(rename = "MonitoringConfigurationDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_configuration_description: Option<ZeppelinMonitoringConfigurationDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CatalogConfigurationDescription {
    #[serde(rename = "GlueDataCatalogConfigurationDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glue_data_catalog_configuration_description:
        Option<GlueDataCatalogConfigurationDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GlueDataCatalogConfigurationDescription {
    #[serde(rename = "DatabaseARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_a_r_n: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomArtifactConfigurationDescription {
    #[serde(rename = "ArtifactType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_type: Option<String>,
    #[serde(rename = "MavenReferenceDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maven_reference_description: Option<MavenReference>,
    #[serde(rename = "S3ContentLocationDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_content_location_description: Option<S3ContentLocation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeployAsApplicationConfigurationDescription {
    #[serde(rename = "S3ContentLocationDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_content_location_description: Option<S3ContentBaseLocationDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3ContentBaseLocationDescription {
    #[serde(rename = "BasePath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_path: Option<String>,
    #[serde(rename = "BucketARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_a_r_n: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ZeppelinMonitoringConfigurationDescription {
    #[serde(rename = "LogLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_level: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApplicationMaintenanceConfigurationDescription {
    #[serde(rename = "ApplicationMaintenanceWindowEndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_maintenance_window_end_time: Option<String>,
    #[serde(rename = "ApplicationMaintenanceWindowStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_maintenance_window_start_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateApplicationSnapshotRequest {
    #[serde(rename = "ApplicationName")]
    #[serde(default)]
    pub application_name: String,
    #[serde(rename = "SnapshotName")]
    #[serde(default)]
    pub snapshot_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateApplicationSnapshotResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteApplicationCloudWatchLoggingOptionRequest {
    #[serde(rename = "ApplicationName")]
    #[serde(default)]
    pub application_name: String,
    #[serde(rename = "CloudWatchLoggingOptionId")]
    #[serde(default)]
    pub cloud_watch_logging_option_id: String,
    #[serde(rename = "ConditionalToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional_token: Option<String>,
    #[serde(rename = "CurrentApplicationVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_application_version_id: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteApplicationCloudWatchLoggingOptionResponse {
    #[serde(rename = "ApplicationARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_a_r_n: Option<String>,
    #[serde(rename = "ApplicationVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_version_id: Option<i64>,
    #[serde(rename = "CloudWatchLoggingOptionDescriptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_option_descriptions: Option<Vec<CloudWatchLoggingOptionDescription>>,
    #[serde(rename = "OperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteApplicationInputProcessingConfigurationRequest {
    #[serde(rename = "ApplicationName")]
    #[serde(default)]
    pub application_name: String,
    #[serde(rename = "CurrentApplicationVersionId")]
    #[serde(default)]
    pub current_application_version_id: i64,
    #[serde(rename = "InputId")]
    #[serde(default)]
    pub input_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteApplicationInputProcessingConfigurationResponse {
    #[serde(rename = "ApplicationARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_a_r_n: Option<String>,
    #[serde(rename = "ApplicationVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_version_id: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteApplicationOutputRequest {
    #[serde(rename = "ApplicationName")]
    #[serde(default)]
    pub application_name: String,
    #[serde(rename = "CurrentApplicationVersionId")]
    #[serde(default)]
    pub current_application_version_id: i64,
    #[serde(rename = "OutputId")]
    #[serde(default)]
    pub output_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteApplicationOutputResponse {
    #[serde(rename = "ApplicationARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_a_r_n: Option<String>,
    #[serde(rename = "ApplicationVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_version_id: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteApplicationReferenceDataSourceRequest {
    #[serde(rename = "ApplicationName")]
    #[serde(default)]
    pub application_name: String,
    #[serde(rename = "CurrentApplicationVersionId")]
    #[serde(default)]
    pub current_application_version_id: i64,
    #[serde(rename = "ReferenceId")]
    #[serde(default)]
    pub reference_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteApplicationReferenceDataSourceResponse {
    #[serde(rename = "ApplicationARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_a_r_n: Option<String>,
    #[serde(rename = "ApplicationVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_version_id: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteApplicationRequest {
    #[serde(rename = "ApplicationName")]
    #[serde(default)]
    pub application_name: String,
    #[serde(rename = "CreateTimestamp")]
    #[serde(default)]
    pub create_timestamp: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteApplicationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteApplicationSnapshotRequest {
    #[serde(rename = "ApplicationName")]
    #[serde(default)]
    pub application_name: String,
    #[serde(rename = "SnapshotCreationTimestamp")]
    #[serde(default)]
    pub snapshot_creation_timestamp: f64,
    #[serde(rename = "SnapshotName")]
    #[serde(default)]
    pub snapshot_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteApplicationSnapshotResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteApplicationVpcConfigurationRequest {
    #[serde(rename = "ApplicationName")]
    #[serde(default)]
    pub application_name: String,
    #[serde(rename = "ConditionalToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional_token: Option<String>,
    #[serde(rename = "CurrentApplicationVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_application_version_id: Option<i64>,
    #[serde(rename = "VpcConfigurationId")]
    #[serde(default)]
    pub vpc_configuration_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteApplicationVpcConfigurationResponse {
    #[serde(rename = "ApplicationARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_a_r_n: Option<String>,
    #[serde(rename = "ApplicationVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_version_id: Option<i64>,
    #[serde(rename = "OperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeApplicationOperationRequest {
    #[serde(rename = "ApplicationName")]
    #[serde(default)]
    pub application_name: String,
    #[serde(rename = "OperationId")]
    #[serde(default)]
    pub operation_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeApplicationOperationResponse {
    #[serde(rename = "ApplicationOperationInfoDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_operation_info_details: Option<ApplicationOperationInfoDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApplicationOperationInfoDetails {
    #[serde(rename = "ApplicationVersionChangeDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_version_change_details: Option<ApplicationVersionChangeDetails>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "Operation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(rename = "OperationFailureDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_failure_details: Option<OperationFailureDetails>,
    #[serde(rename = "OperationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_status: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApplicationVersionChangeDetails {
    #[serde(rename = "ApplicationVersionUpdatedFrom")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_version_updated_from: Option<i64>,
    #[serde(rename = "ApplicationVersionUpdatedTo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_version_updated_to: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OperationFailureDetails {
    #[serde(rename = "ErrorInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_info: Option<ErrorInfo>,
    #[serde(rename = "RollbackOperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollback_operation_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ErrorInfo {
    #[serde(rename = "ErrorString")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_string: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeApplicationRequest {
    #[serde(rename = "ApplicationName")]
    #[serde(default)]
    pub application_name: String,
    #[serde(rename = "IncludeAdditionalDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_additional_details: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeApplicationResponse {
    #[serde(rename = "ApplicationDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_detail: Option<ApplicationDetail>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeApplicationSnapshotRequest {
    #[serde(rename = "ApplicationName")]
    #[serde(default)]
    pub application_name: String,
    #[serde(rename = "SnapshotName")]
    #[serde(default)]
    pub snapshot_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeApplicationSnapshotResponse {
    #[serde(rename = "SnapshotDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_details: Option<SnapshotDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SnapshotDetails {
    #[serde(rename = "ApplicationEncryptionConfigurationDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_encryption_configuration_description:
        Option<ApplicationEncryptionConfigurationDescription>,
    #[serde(rename = "ApplicationVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_version_id: Option<i64>,
    #[serde(rename = "RuntimeEnvironment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_environment: Option<String>,
    #[serde(rename = "SnapshotCreationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_creation_timestamp: Option<f64>,
    #[serde(rename = "SnapshotName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_name: Option<String>,
    #[serde(rename = "SnapshotStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeApplicationVersionRequest {
    #[serde(rename = "ApplicationName")]
    #[serde(default)]
    pub application_name: String,
    #[serde(rename = "ApplicationVersionId")]
    #[serde(default)]
    pub application_version_id: i64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeApplicationVersionResponse {
    #[serde(rename = "ApplicationVersionDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_version_detail: Option<ApplicationDetail>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DiscoverInputSchemaRequest {
    #[serde(rename = "InputProcessingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_processing_configuration: Option<InputProcessingConfiguration>,
    #[serde(rename = "InputStartingPositionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_starting_position_configuration: Option<InputStartingPositionConfiguration>,
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_a_r_n: Option<String>,
    #[serde(rename = "S3Configuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_configuration: Option<S3Configuration>,
    #[serde(rename = "ServiceExecutionRole")]
    #[serde(default)]
    pub service_execution_role: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3Configuration {
    #[serde(rename = "BucketARN")]
    #[serde(default)]
    pub bucket_a_r_n: String,
    #[serde(rename = "FileKey")]
    #[serde(default)]
    pub file_key: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DiscoverInputSchemaResponse {
    #[serde(rename = "InputSchema")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_schema: Option<SourceSchema>,
    #[serde(rename = "ParsedInputRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parsed_input_records: Option<Vec<Vec<String>>>,
    #[serde(rename = "ProcessedInputRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processed_input_records: Option<Vec<String>>,
    #[serde(rename = "RawInputRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_input_records: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListApplicationOperationsRequest {
    #[serde(rename = "ApplicationName")]
    #[serde(default)]
    pub application_name: String,
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Operation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(rename = "OperationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListApplicationOperationsResponse {
    #[serde(rename = "ApplicationOperationInfoList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_operation_info_list: Option<Vec<ApplicationOperationInfo>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApplicationOperationInfo {
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "Operation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(rename = "OperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    #[serde(rename = "OperationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_status: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListApplicationSnapshotsRequest {
    #[serde(rename = "ApplicationName")]
    #[serde(default)]
    pub application_name: String,
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListApplicationSnapshotsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SnapshotSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_summaries: Option<Vec<SnapshotDetails>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListApplicationVersionsRequest {
    #[serde(rename = "ApplicationName")]
    #[serde(default)]
    pub application_name: String,
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListApplicationVersionsResponse {
    #[serde(rename = "ApplicationVersionSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_version_summaries: Option<Vec<ApplicationVersionSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApplicationVersionSummary {
    #[serde(rename = "ApplicationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_status: Option<String>,
    #[serde(rename = "ApplicationVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_version_id: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListApplicationsRequest {
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListApplicationsResponse {
    #[serde(rename = "ApplicationSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_summaries: Option<Vec<ApplicationSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApplicationSummary {
    #[serde(rename = "ApplicationARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_a_r_n: Option<String>,
    #[serde(rename = "ApplicationMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_mode: Option<String>,
    #[serde(rename = "ApplicationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    #[serde(rename = "ApplicationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_status: Option<String>,
    #[serde(rename = "ApplicationVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_version_id: Option<i64>,
    #[serde(rename = "RuntimeEnvironment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_environment: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RollbackApplicationRequest {
    #[serde(rename = "ApplicationName")]
    #[serde(default)]
    pub application_name: String,
    #[serde(rename = "CurrentApplicationVersionId")]
    #[serde(default)]
    pub current_application_version_id: i64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RollbackApplicationResponse {
    #[serde(rename = "ApplicationDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_detail: Option<ApplicationDetail>,
    #[serde(rename = "OperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartApplicationRequest {
    #[serde(rename = "ApplicationName")]
    #[serde(default)]
    pub application_name: String,
    #[serde(rename = "RunConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_configuration: Option<RunConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RunConfiguration {
    #[serde(rename = "ApplicationRestoreConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_restore_configuration: Option<ApplicationRestoreConfiguration>,
    #[serde(rename = "FlinkRunConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flink_run_configuration: Option<FlinkRunConfiguration>,
    #[serde(rename = "SqlRunConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sql_run_configurations: Option<Vec<SqlRunConfiguration>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SqlRunConfiguration {
    #[serde(rename = "InputId")]
    #[serde(default)]
    pub input_id: String,
    #[serde(rename = "InputStartingPositionConfiguration")]
    #[serde(default)]
    pub input_starting_position_configuration: InputStartingPositionConfiguration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartApplicationResponse {
    #[serde(rename = "OperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopApplicationRequest {
    #[serde(rename = "ApplicationName")]
    #[serde(default)]
    pub application_name: String,
    #[serde(rename = "Force")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopApplicationResponse {
    #[serde(rename = "OperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateApplicationMaintenanceConfigurationRequest {
    #[serde(rename = "ApplicationMaintenanceConfigurationUpdate")]
    #[serde(default)]
    pub application_maintenance_configuration_update: ApplicationMaintenanceConfigurationUpdate,
    #[serde(rename = "ApplicationName")]
    #[serde(default)]
    pub application_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApplicationMaintenanceConfigurationUpdate {
    #[serde(rename = "ApplicationMaintenanceWindowStartTimeUpdate")]
    #[serde(default)]
    pub application_maintenance_window_start_time_update: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateApplicationMaintenanceConfigurationResponse {
    #[serde(rename = "ApplicationARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_a_r_n: Option<String>,
    #[serde(rename = "ApplicationMaintenanceConfigurationDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_maintenance_configuration_description:
        Option<ApplicationMaintenanceConfigurationDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateApplicationRequest {
    #[serde(rename = "ApplicationConfigurationUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_configuration_update: Option<ApplicationConfigurationUpdate>,
    #[serde(rename = "ApplicationName")]
    #[serde(default)]
    pub application_name: String,
    #[serde(rename = "CloudWatchLoggingOptionUpdates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_option_updates: Option<Vec<CloudWatchLoggingOptionUpdate>>,
    #[serde(rename = "ConditionalToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional_token: Option<String>,
    #[serde(rename = "CurrentApplicationVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_application_version_id: Option<i64>,
    #[serde(rename = "RunConfigurationUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_configuration_update: Option<RunConfigurationUpdate>,
    #[serde(rename = "RuntimeEnvironmentUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_environment_update: Option<String>,
    #[serde(rename = "ServiceExecutionRoleUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_execution_role_update: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApplicationConfigurationUpdate {
    #[serde(rename = "ApplicationCodeConfigurationUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_code_configuration_update: Option<ApplicationCodeConfigurationUpdate>,
    #[serde(rename = "ApplicationEncryptionConfigurationUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_encryption_configuration_update:
        Option<ApplicationEncryptionConfigurationUpdate>,
    #[serde(rename = "ApplicationSnapshotConfigurationUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_snapshot_configuration_update: Option<ApplicationSnapshotConfigurationUpdate>,
    #[serde(rename = "ApplicationSystemRollbackConfigurationUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_system_rollback_configuration_update:
        Option<ApplicationSystemRollbackConfigurationUpdate>,
    #[serde(rename = "EnvironmentPropertyUpdates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_property_updates: Option<EnvironmentPropertyUpdates>,
    #[serde(rename = "FlinkApplicationConfigurationUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flink_application_configuration_update: Option<FlinkApplicationConfigurationUpdate>,
    #[serde(rename = "SqlApplicationConfigurationUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sql_application_configuration_update: Option<SqlApplicationConfigurationUpdate>,
    #[serde(rename = "VpcConfigurationUpdates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_configuration_updates: Option<Vec<VpcConfigurationUpdate>>,
    #[serde(rename = "ZeppelinApplicationConfigurationUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zeppelin_application_configuration_update: Option<ZeppelinApplicationConfigurationUpdate>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApplicationCodeConfigurationUpdate {
    #[serde(rename = "CodeContentTypeUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_content_type_update: Option<String>,
    #[serde(rename = "CodeContentUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_content_update: Option<CodeContentUpdate>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CodeContentUpdate {
    #[serde(rename = "S3ContentLocationUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_content_location_update: Option<S3ContentLocationUpdate>,
    #[serde(rename = "TextContentUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_content_update: Option<String>,
    #[serde(rename = "ZipFileContentUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip_file_content_update: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3ContentLocationUpdate {
    #[serde(rename = "BucketARNUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_a_r_n_update: Option<String>,
    #[serde(rename = "FileKeyUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_key_update: Option<String>,
    #[serde(rename = "ObjectVersionUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_version_update: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApplicationEncryptionConfigurationUpdate {
    #[serde(rename = "KeyIdUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id_update: Option<String>,
    #[serde(rename = "KeyTypeUpdate")]
    #[serde(default)]
    pub key_type_update: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApplicationSnapshotConfigurationUpdate {
    #[serde(rename = "SnapshotsEnabledUpdate")]
    #[serde(default)]
    pub snapshots_enabled_update: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApplicationSystemRollbackConfigurationUpdate {
    #[serde(rename = "RollbackEnabledUpdate")]
    #[serde(default)]
    pub rollback_enabled_update: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnvironmentPropertyUpdates {
    #[serde(rename = "PropertyGroups")]
    #[serde(default)]
    pub property_groups: Vec<PropertyGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FlinkApplicationConfigurationUpdate {
    #[serde(rename = "CheckpointConfigurationUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkpoint_configuration_update: Option<CheckpointConfigurationUpdate>,
    #[serde(rename = "MonitoringConfigurationUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_configuration_update: Option<MonitoringConfigurationUpdate>,
    #[serde(rename = "ParallelismConfigurationUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallelism_configuration_update: Option<ParallelismConfigurationUpdate>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CheckpointConfigurationUpdate {
    #[serde(rename = "CheckpointIntervalUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkpoint_interval_update: Option<i64>,
    #[serde(rename = "CheckpointingEnabledUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkpointing_enabled_update: Option<bool>,
    #[serde(rename = "ConfigurationTypeUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_type_update: Option<String>,
    #[serde(rename = "MinPauseBetweenCheckpointsUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_pause_between_checkpoints_update: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MonitoringConfigurationUpdate {
    #[serde(rename = "ConfigurationTypeUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_type_update: Option<String>,
    #[serde(rename = "LogLevelUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_level_update: Option<String>,
    #[serde(rename = "MetricsLevelUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics_level_update: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ParallelismConfigurationUpdate {
    #[serde(rename = "AutoScalingEnabledUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_enabled_update: Option<bool>,
    #[serde(rename = "ConfigurationTypeUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_type_update: Option<String>,
    #[serde(rename = "ParallelismPerKPUUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallelism_per_k_p_u_update: Option<i32>,
    #[serde(rename = "ParallelismUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallelism_update: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SqlApplicationConfigurationUpdate {
    #[serde(rename = "InputUpdates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_updates: Option<Vec<InputUpdate>>,
    #[serde(rename = "OutputUpdates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_updates: Option<Vec<OutputUpdate>>,
    #[serde(rename = "ReferenceDataSourceUpdates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_data_source_updates: Option<Vec<ReferenceDataSourceUpdate>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InputUpdate {
    #[serde(rename = "InputId")]
    #[serde(default)]
    pub input_id: String,
    #[serde(rename = "InputParallelismUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_parallelism_update: Option<InputParallelismUpdate>,
    #[serde(rename = "InputProcessingConfigurationUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_processing_configuration_update: Option<InputProcessingConfigurationUpdate>,
    #[serde(rename = "InputSchemaUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_schema_update: Option<InputSchemaUpdate>,
    #[serde(rename = "KinesisFirehoseInputUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_firehose_input_update: Option<KinesisFirehoseInputUpdate>,
    #[serde(rename = "KinesisStreamsInputUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_streams_input_update: Option<KinesisStreamsInputUpdate>,
    #[serde(rename = "NamePrefixUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_prefix_update: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InputParallelismUpdate {
    #[serde(rename = "CountUpdate")]
    #[serde(default)]
    pub count_update: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InputProcessingConfigurationUpdate {
    #[serde(rename = "InputLambdaProcessorUpdate")]
    #[serde(default)]
    pub input_lambda_processor_update: InputLambdaProcessorUpdate,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InputLambdaProcessorUpdate {
    #[serde(rename = "ResourceARNUpdate")]
    #[serde(default)]
    pub resource_a_r_n_update: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InputSchemaUpdate {
    #[serde(rename = "RecordColumnUpdates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_column_updates: Option<Vec<RecordColumn>>,
    #[serde(rename = "RecordEncodingUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_encoding_update: Option<String>,
    #[serde(rename = "RecordFormatUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_format_update: Option<RecordFormat>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KinesisFirehoseInputUpdate {
    #[serde(rename = "ResourceARNUpdate")]
    #[serde(default)]
    pub resource_a_r_n_update: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KinesisStreamsInputUpdate {
    #[serde(rename = "ResourceARNUpdate")]
    #[serde(default)]
    pub resource_a_r_n_update: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OutputUpdate {
    #[serde(rename = "DestinationSchemaUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_schema_update: Option<DestinationSchema>,
    #[serde(rename = "KinesisFirehoseOutputUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_firehose_output_update: Option<KinesisFirehoseOutputUpdate>,
    #[serde(rename = "KinesisStreamsOutputUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_streams_output_update: Option<KinesisStreamsOutputUpdate>,
    #[serde(rename = "LambdaOutputUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_output_update: Option<LambdaOutputUpdate>,
    #[serde(rename = "NameUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_update: Option<String>,
    #[serde(rename = "OutputId")]
    #[serde(default)]
    pub output_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KinesisFirehoseOutputUpdate {
    #[serde(rename = "ResourceARNUpdate")]
    #[serde(default)]
    pub resource_a_r_n_update: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KinesisStreamsOutputUpdate {
    #[serde(rename = "ResourceARNUpdate")]
    #[serde(default)]
    pub resource_a_r_n_update: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LambdaOutputUpdate {
    #[serde(rename = "ResourceARNUpdate")]
    #[serde(default)]
    pub resource_a_r_n_update: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReferenceDataSourceUpdate {
    #[serde(rename = "ReferenceId")]
    #[serde(default)]
    pub reference_id: String,
    #[serde(rename = "ReferenceSchemaUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_schema_update: Option<SourceSchema>,
    #[serde(rename = "S3ReferenceDataSourceUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_reference_data_source_update: Option<S3ReferenceDataSourceUpdate>,
    #[serde(rename = "TableNameUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name_update: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3ReferenceDataSourceUpdate {
    #[serde(rename = "BucketARNUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_a_r_n_update: Option<String>,
    #[serde(rename = "FileKeyUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_key_update: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VpcConfigurationUpdate {
    #[serde(rename = "SecurityGroupIdUpdates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_id_updates: Option<Vec<String>>,
    #[serde(rename = "SubnetIdUpdates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id_updates: Option<Vec<String>>,
    #[serde(rename = "VpcConfigurationId")]
    #[serde(default)]
    pub vpc_configuration_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ZeppelinApplicationConfigurationUpdate {
    #[serde(rename = "CatalogConfigurationUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_configuration_update: Option<CatalogConfigurationUpdate>,
    #[serde(rename = "CustomArtifactsConfigurationUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_artifacts_configuration_update: Option<Vec<CustomArtifactConfiguration>>,
    #[serde(rename = "DeployAsApplicationConfigurationUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deploy_as_application_configuration_update: Option<DeployAsApplicationConfigurationUpdate>,
    #[serde(rename = "MonitoringConfigurationUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_configuration_update: Option<ZeppelinMonitoringConfigurationUpdate>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CatalogConfigurationUpdate {
    #[serde(rename = "GlueDataCatalogConfigurationUpdate")]
    #[serde(default)]
    pub glue_data_catalog_configuration_update: GlueDataCatalogConfigurationUpdate,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GlueDataCatalogConfigurationUpdate {
    #[serde(rename = "DatabaseARNUpdate")]
    #[serde(default)]
    pub database_a_r_n_update: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeployAsApplicationConfigurationUpdate {
    #[serde(rename = "S3ContentLocationUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_content_location_update: Option<S3ContentBaseLocationUpdate>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3ContentBaseLocationUpdate {
    #[serde(rename = "BasePathUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_path_update: Option<String>,
    #[serde(rename = "BucketARNUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_a_r_n_update: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ZeppelinMonitoringConfigurationUpdate {
    #[serde(rename = "LogLevelUpdate")]
    #[serde(default)]
    pub log_level_update: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CloudWatchLoggingOptionUpdate {
    #[serde(rename = "CloudWatchLoggingOptionId")]
    #[serde(default)]
    pub cloud_watch_logging_option_id: String,
    #[serde(rename = "LogStreamARNUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_stream_a_r_n_update: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RunConfigurationUpdate {
    #[serde(rename = "ApplicationRestoreConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_restore_configuration: Option<ApplicationRestoreConfiguration>,
    #[serde(rename = "FlinkRunConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flink_run_configuration: Option<FlinkRunConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateApplicationResponse {
    #[serde(rename = "ApplicationDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_detail: Option<ApplicationDetail>,
    #[serde(rename = "OperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
}
