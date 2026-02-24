//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-firehose

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDeliveryStreamInput {
    #[serde(rename = "AmazonOpenSearchServerlessDestinationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_open_search_serverless_destination_configuration:
        Option<AmazonOpenSearchServerlessDestinationConfiguration>,
    #[serde(rename = "AmazonopensearchserviceDestinationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazonopensearchservice_destination_configuration:
        Option<AmazonopensearchserviceDestinationConfiguration>,
    #[serde(rename = "DatabaseSourceConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_source_configuration: Option<DatabaseSourceConfiguration>,
    #[serde(rename = "DeliveryStreamEncryptionConfigurationInput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_stream_encryption_configuration_input:
        Option<DeliveryStreamEncryptionConfigurationInput>,
    #[serde(rename = "DeliveryStreamName")]
    #[serde(default)]
    pub delivery_stream_name: String,
    #[serde(rename = "DeliveryStreamType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_stream_type: Option<String>,
    #[serde(rename = "DirectPutSourceConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_put_source_configuration: Option<DirectPutSourceConfiguration>,
    #[serde(rename = "ElasticsearchDestinationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elasticsearch_destination_configuration: Option<ElasticsearchDestinationConfiguration>,
    #[serde(rename = "ExtendedS3DestinationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_s3_destination_configuration: Option<ExtendedS3DestinationConfiguration>,
    #[serde(rename = "HttpEndpointDestinationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_endpoint_destination_configuration: Option<HttpEndpointDestinationConfiguration>,
    #[serde(rename = "IcebergDestinationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iceberg_destination_configuration: Option<IcebergDestinationConfiguration>,
    #[serde(rename = "KinesisStreamSourceConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_stream_source_configuration: Option<KinesisStreamSourceConfiguration>,
    #[serde(rename = "MSKSourceConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_s_k_source_configuration: Option<MSKSourceConfiguration>,
    #[serde(rename = "RedshiftDestinationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redshift_destination_configuration: Option<RedshiftDestinationConfiguration>,
    #[serde(rename = "S3DestinationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_destination_configuration: Option<S3DestinationConfiguration>,
    #[serde(rename = "SnowflakeDestinationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snowflake_destination_configuration: Option<SnowflakeDestinationConfiguration>,
    #[serde(rename = "SplunkDestinationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splunk_destination_configuration: Option<SplunkDestinationConfiguration>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AmazonOpenSearchServerlessDestinationConfiguration {
    #[serde(rename = "BufferingHints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffering_hints: Option<AmazonOpenSearchServerlessBufferingHints>,
    #[serde(rename = "CloudWatchLoggingOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,
    #[serde(rename = "CollectionEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_endpoint: Option<String>,
    #[serde(rename = "IndexName")]
    #[serde(default)]
    pub index_name: String,
    #[serde(rename = "ProcessingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_configuration: Option<ProcessingConfiguration>,
    #[serde(rename = "RetryOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_options: Option<AmazonOpenSearchServerlessRetryOptions>,
    #[serde(rename = "RoleARN")]
    #[serde(default)]
    pub role_a_r_n: String,
    #[serde(rename = "S3BackupMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_backup_mode: Option<String>,
    #[serde(rename = "S3Configuration")]
    #[serde(default)]
    pub s3_configuration: S3DestinationConfiguration,
    #[serde(rename = "VpcConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_configuration: Option<VpcConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AmazonOpenSearchServerlessBufferingHints {
    #[serde(rename = "IntervalInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_in_seconds: Option<i32>,
    #[serde(rename = "SizeInMBs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_in_m_bs: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CloudWatchLoggingOptions {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "LogGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_name: Option<String>,
    #[serde(rename = "LogStreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_stream_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProcessingConfiguration {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "Processors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processors: Option<Vec<Processor>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Processor {
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<ProcessorParameter>>,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProcessorParameter {
    #[serde(rename = "ParameterName")]
    #[serde(default)]
    pub parameter_name: String,
    #[serde(rename = "ParameterValue")]
    #[serde(default)]
    pub parameter_value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AmazonOpenSearchServerlessRetryOptions {
    #[serde(rename = "DurationInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_in_seconds: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3DestinationConfiguration {
    #[serde(rename = "BucketARN")]
    #[serde(default)]
    pub bucket_a_r_n: String,
    #[serde(rename = "BufferingHints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffering_hints: Option<BufferingHints>,
    #[serde(rename = "CloudWatchLoggingOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,
    #[serde(rename = "CompressionFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compression_format: Option<String>,
    #[serde(rename = "EncryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
    #[serde(rename = "ErrorOutputPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_output_prefix: Option<String>,
    #[serde(rename = "Prefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "RoleARN")]
    #[serde(default)]
    pub role_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BufferingHints {
    #[serde(rename = "IntervalInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_in_seconds: Option<i32>,
    #[serde(rename = "SizeInMBs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_in_m_bs: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EncryptionConfiguration {
    #[serde(rename = "KMSEncryptionConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k_m_s_encryption_config: Option<KMSEncryptionConfig>,
    #[serde(rename = "NoEncryptionConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_encryption_config: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KMSEncryptionConfig {
    #[serde(rename = "AWSKMSKeyARN")]
    #[serde(default)]
    pub a_w_s_k_m_s_key_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VpcConfiguration {
    #[serde(rename = "RoleARN")]
    #[serde(default)]
    pub role_a_r_n: String,
    #[serde(rename = "SecurityGroupIds")]
    #[serde(default)]
    pub security_group_ids: Vec<String>,
    #[serde(rename = "SubnetIds")]
    #[serde(default)]
    pub subnet_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AmazonopensearchserviceDestinationConfiguration {
    #[serde(rename = "BufferingHints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffering_hints: Option<AmazonopensearchserviceBufferingHints>,
    #[serde(rename = "CloudWatchLoggingOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,
    #[serde(rename = "ClusterEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_endpoint: Option<String>,
    #[serde(rename = "DocumentIdOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_id_options: Option<DocumentIdOptions>,
    #[serde(rename = "DomainARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_a_r_n: Option<String>,
    #[serde(rename = "IndexName")]
    #[serde(default)]
    pub index_name: String,
    #[serde(rename = "IndexRotationPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_rotation_period: Option<String>,
    #[serde(rename = "ProcessingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_configuration: Option<ProcessingConfiguration>,
    #[serde(rename = "RetryOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_options: Option<AmazonopensearchserviceRetryOptions>,
    #[serde(rename = "RoleARN")]
    #[serde(default)]
    pub role_a_r_n: String,
    #[serde(rename = "S3BackupMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_backup_mode: Option<String>,
    #[serde(rename = "S3Configuration")]
    #[serde(default)]
    pub s3_configuration: S3DestinationConfiguration,
    #[serde(rename = "TypeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
    #[serde(rename = "VpcConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_configuration: Option<VpcConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AmazonopensearchserviceBufferingHints {
    #[serde(rename = "IntervalInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_in_seconds: Option<i32>,
    #[serde(rename = "SizeInMBs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_in_m_bs: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DocumentIdOptions {
    #[serde(rename = "DefaultDocumentIdFormat")]
    #[serde(default)]
    pub default_document_id_format: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AmazonopensearchserviceRetryOptions {
    #[serde(rename = "DurationInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_in_seconds: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DatabaseSourceConfiguration {
    #[serde(rename = "Columns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<DatabaseColumnList>,
    #[serde(rename = "DatabaseSourceAuthenticationConfiguration")]
    #[serde(default)]
    pub database_source_authentication_configuration: DatabaseSourceAuthenticationConfiguration,
    #[serde(rename = "DatabaseSourceVPCConfiguration")]
    #[serde(default)]
    pub database_source_v_p_c_configuration: DatabaseSourceVPCConfiguration,
    #[serde(rename = "Databases")]
    #[serde(default)]
    pub databases: DatabaseList,
    #[serde(rename = "Endpoint")]
    #[serde(default)]
    pub endpoint: String,
    #[serde(rename = "Port")]
    #[serde(default)]
    pub port: i32,
    #[serde(rename = "SSLMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_l_mode: Option<String>,
    #[serde(rename = "SnapshotWatermarkTable")]
    #[serde(default)]
    pub snapshot_watermark_table: String,
    #[serde(rename = "SurrogateKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surrogate_keys: Option<Vec<String>>,
    #[serde(rename = "Tables")]
    #[serde(default)]
    pub tables: DatabaseTableList,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DatabaseColumnList {
    #[serde(rename = "Exclude")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude: Option<Vec<String>>,
    #[serde(rename = "Include")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DatabaseSourceAuthenticationConfiguration {
    #[serde(rename = "SecretsManagerConfiguration")]
    #[serde(default)]
    pub secrets_manager_configuration: SecretsManagerConfiguration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SecretsManagerConfiguration {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    pub enabled: bool,
    #[serde(rename = "RoleARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_a_r_n: Option<String>,
    #[serde(rename = "SecretARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_a_r_n: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DatabaseSourceVPCConfiguration {
    #[serde(rename = "VpcEndpointServiceName")]
    #[serde(default)]
    pub vpc_endpoint_service_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DatabaseList {
    #[serde(rename = "Exclude")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude: Option<Vec<String>>,
    #[serde(rename = "Include")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DatabaseTableList {
    #[serde(rename = "Exclude")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude: Option<Vec<String>>,
    #[serde(rename = "Include")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeliveryStreamEncryptionConfigurationInput {
    #[serde(rename = "KeyARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_a_r_n: Option<String>,
    #[serde(rename = "KeyType")]
    #[serde(default)]
    pub key_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DirectPutSourceConfiguration {
    #[serde(rename = "ThroughputHintInMBs")]
    #[serde(default)]
    pub throughput_hint_in_m_bs: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ElasticsearchDestinationConfiguration {
    #[serde(rename = "BufferingHints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffering_hints: Option<ElasticsearchBufferingHints>,
    #[serde(rename = "CloudWatchLoggingOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,
    #[serde(rename = "ClusterEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_endpoint: Option<String>,
    #[serde(rename = "DocumentIdOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_id_options: Option<DocumentIdOptions>,
    #[serde(rename = "DomainARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_a_r_n: Option<String>,
    #[serde(rename = "IndexName")]
    #[serde(default)]
    pub index_name: String,
    #[serde(rename = "IndexRotationPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_rotation_period: Option<String>,
    #[serde(rename = "ProcessingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_configuration: Option<ProcessingConfiguration>,
    #[serde(rename = "RetryOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_options: Option<ElasticsearchRetryOptions>,
    #[serde(rename = "RoleARN")]
    #[serde(default)]
    pub role_a_r_n: String,
    #[serde(rename = "S3BackupMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_backup_mode: Option<String>,
    #[serde(rename = "S3Configuration")]
    #[serde(default)]
    pub s3_configuration: S3DestinationConfiguration,
    #[serde(rename = "TypeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
    #[serde(rename = "VpcConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_configuration: Option<VpcConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ElasticsearchBufferingHints {
    #[serde(rename = "IntervalInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_in_seconds: Option<i32>,
    #[serde(rename = "SizeInMBs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_in_m_bs: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ElasticsearchRetryOptions {
    #[serde(rename = "DurationInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_in_seconds: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExtendedS3DestinationConfiguration {
    #[serde(rename = "BucketARN")]
    #[serde(default)]
    pub bucket_a_r_n: String,
    #[serde(rename = "BufferingHints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffering_hints: Option<BufferingHints>,
    #[serde(rename = "CloudWatchLoggingOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,
    #[serde(rename = "CompressionFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compression_format: Option<String>,
    #[serde(rename = "CustomTimeZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_time_zone: Option<String>,
    #[serde(rename = "DataFormatConversionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_format_conversion_configuration: Option<DataFormatConversionConfiguration>,
    #[serde(rename = "DynamicPartitioningConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_partitioning_configuration: Option<DynamicPartitioningConfiguration>,
    #[serde(rename = "EncryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
    #[serde(rename = "ErrorOutputPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_output_prefix: Option<String>,
    #[serde(rename = "FileExtension")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_extension: Option<String>,
    #[serde(rename = "Prefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "ProcessingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_configuration: Option<ProcessingConfiguration>,
    #[serde(rename = "RoleARN")]
    #[serde(default)]
    pub role_a_r_n: String,
    #[serde(rename = "S3BackupConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_backup_configuration: Option<S3DestinationConfiguration>,
    #[serde(rename = "S3BackupMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_backup_mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataFormatConversionConfiguration {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "InputFormatConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_format_configuration: Option<InputFormatConfiguration>,
    #[serde(rename = "OutputFormatConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format_configuration: Option<OutputFormatConfiguration>,
    #[serde(rename = "SchemaConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_configuration: Option<SchemaConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InputFormatConfiguration {
    #[serde(rename = "Deserializer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deserializer: Option<Deserializer>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Deserializer {
    #[serde(rename = "HiveJsonSerDe")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hive_json_ser_de: Option<HiveJsonSerDe>,
    #[serde(rename = "OpenXJsonSerDe")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_x_json_ser_de: Option<OpenXJsonSerDe>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HiveJsonSerDe {
    #[serde(rename = "TimestampFormats")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_formats: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OpenXJsonSerDe {
    #[serde(rename = "CaseInsensitive")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub case_insensitive: Option<bool>,
    #[serde(rename = "ColumnToJsonKeyMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_to_json_key_mappings: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ConvertDotsInJsonKeysToUnderscores")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub convert_dots_in_json_keys_to_underscores: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OutputFormatConfiguration {
    #[serde(rename = "Serializer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serializer: Option<Serializer>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Serializer {
    #[serde(rename = "OrcSerDe")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orc_ser_de: Option<OrcSerDe>,
    #[serde(rename = "ParquetSerDe")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parquet_ser_de: Option<ParquetSerDe>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OrcSerDe {
    #[serde(rename = "BlockSizeBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_size_bytes: Option<i32>,
    #[serde(rename = "BloomFilterColumns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bloom_filter_columns: Option<Vec<String>>,
    #[serde(rename = "BloomFilterFalsePositiveProbability")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bloom_filter_false_positive_probability: Option<f64>,
    #[serde(rename = "Compression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compression: Option<String>,
    #[serde(rename = "DictionaryKeyThreshold")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dictionary_key_threshold: Option<f64>,
    #[serde(rename = "EnablePadding")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_padding: Option<bool>,
    #[serde(rename = "FormatVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format_version: Option<String>,
    #[serde(rename = "PaddingTolerance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding_tolerance: Option<f64>,
    #[serde(rename = "RowIndexStride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_index_stride: Option<i32>,
    #[serde(rename = "StripeSizeBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stripe_size_bytes: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ParquetSerDe {
    #[serde(rename = "BlockSizeBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_size_bytes: Option<i32>,
    #[serde(rename = "Compression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compression: Option<String>,
    #[serde(rename = "EnableDictionaryCompression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_dictionary_compression: Option<bool>,
    #[serde(rename = "MaxPaddingBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_padding_bytes: Option<i32>,
    #[serde(rename = "PageSizeBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size_bytes: Option<i32>,
    #[serde(rename = "WriterVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub writer_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SchemaConfiguration {
    #[serde(rename = "CatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(rename = "Region")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "RoleARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_a_r_n: Option<String>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    #[serde(rename = "VersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DynamicPartitioningConfiguration {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "RetryOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_options: Option<RetryOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RetryOptions {
    #[serde(rename = "DurationInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_in_seconds: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HttpEndpointDestinationConfiguration {
    #[serde(rename = "BufferingHints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffering_hints: Option<HttpEndpointBufferingHints>,
    #[serde(rename = "CloudWatchLoggingOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,
    #[serde(rename = "EndpointConfiguration")]
    #[serde(default)]
    pub endpoint_configuration: HttpEndpointConfiguration,
    #[serde(rename = "ProcessingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_configuration: Option<ProcessingConfiguration>,
    #[serde(rename = "RequestConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_configuration: Option<HttpEndpointRequestConfiguration>,
    #[serde(rename = "RetryOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_options: Option<HttpEndpointRetryOptions>,
    #[serde(rename = "RoleARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_a_r_n: Option<String>,
    #[serde(rename = "S3BackupMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_backup_mode: Option<String>,
    #[serde(rename = "S3Configuration")]
    #[serde(default)]
    pub s3_configuration: S3DestinationConfiguration,
    #[serde(rename = "SecretsManagerConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_configuration: Option<SecretsManagerConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HttpEndpointBufferingHints {
    #[serde(rename = "IntervalInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_in_seconds: Option<i32>,
    #[serde(rename = "SizeInMBs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_in_m_bs: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HttpEndpointConfiguration {
    #[serde(rename = "AccessKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Url")]
    #[serde(default)]
    pub url: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HttpEndpointRequestConfiguration {
    #[serde(rename = "CommonAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common_attributes: Option<Vec<HttpEndpointCommonAttribute>>,
    #[serde(rename = "ContentEncoding")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_encoding: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HttpEndpointCommonAttribute {
    #[serde(rename = "AttributeName")]
    #[serde(default)]
    pub attribute_name: String,
    #[serde(rename = "AttributeValue")]
    #[serde(default)]
    pub attribute_value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HttpEndpointRetryOptions {
    #[serde(rename = "DurationInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_in_seconds: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IcebergDestinationConfiguration {
    #[serde(rename = "AppendOnly")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub append_only: Option<bool>,
    #[serde(rename = "BufferingHints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffering_hints: Option<BufferingHints>,
    #[serde(rename = "CatalogConfiguration")]
    #[serde(default)]
    pub catalog_configuration: CatalogConfiguration,
    #[serde(rename = "CloudWatchLoggingOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,
    #[serde(rename = "DestinationTableConfigurationList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_table_configuration_list: Option<Vec<DestinationTableConfiguration>>,
    #[serde(rename = "ProcessingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_configuration: Option<ProcessingConfiguration>,
    #[serde(rename = "RetryOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_options: Option<RetryOptions>,
    #[serde(rename = "RoleARN")]
    #[serde(default)]
    pub role_a_r_n: String,
    #[serde(rename = "S3BackupMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_backup_mode: Option<String>,
    #[serde(rename = "S3Configuration")]
    #[serde(default)]
    pub s3_configuration: S3DestinationConfiguration,
    #[serde(rename = "SchemaEvolutionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_evolution_configuration: Option<SchemaEvolutionConfiguration>,
    #[serde(rename = "TableCreationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_creation_configuration: Option<TableCreationConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CatalogConfiguration {
    #[serde(rename = "CatalogARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_a_r_n: Option<String>,
    #[serde(rename = "WarehouseLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warehouse_location: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DestinationTableConfiguration {
    #[serde(rename = "DestinationDatabaseName")]
    #[serde(default)]
    pub destination_database_name: String,
    #[serde(rename = "DestinationTableName")]
    #[serde(default)]
    pub destination_table_name: String,
    #[serde(rename = "PartitionSpec")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_spec: Option<PartitionSpec>,
    #[serde(rename = "S3ErrorOutputPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_error_output_prefix: Option<String>,
    #[serde(rename = "UniqueKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_keys: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PartitionSpec {
    #[serde(rename = "Identity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<Vec<PartitionField>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PartitionField {
    #[serde(rename = "SourceName")]
    #[serde(default)]
    pub source_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SchemaEvolutionConfiguration {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    pub enabled: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TableCreationConfiguration {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    pub enabled: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KinesisStreamSourceConfiguration {
    #[serde(rename = "KinesisStreamARN")]
    #[serde(default)]
    pub kinesis_stream_a_r_n: String,
    #[serde(rename = "RoleARN")]
    #[serde(default)]
    pub role_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MSKSourceConfiguration {
    #[serde(rename = "AuthenticationConfiguration")]
    #[serde(default)]
    pub authentication_configuration: AuthenticationConfiguration,
    #[serde(rename = "MSKClusterARN")]
    #[serde(default)]
    pub m_s_k_cluster_a_r_n: String,
    #[serde(rename = "ReadFromTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_from_timestamp: Option<f64>,
    #[serde(rename = "TopicName")]
    #[serde(default)]
    pub topic_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuthenticationConfiguration {
    #[serde(rename = "Connectivity")]
    #[serde(default)]
    pub connectivity: String,
    #[serde(rename = "RoleARN")]
    #[serde(default)]
    pub role_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RedshiftDestinationConfiguration {
    #[serde(rename = "CloudWatchLoggingOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,
    #[serde(rename = "ClusterJDBCURL")]
    #[serde(default)]
    pub cluster_j_d_b_c_u_r_l: String,
    #[serde(rename = "CopyCommand")]
    #[serde(default)]
    pub copy_command: CopyCommand,
    #[serde(rename = "Password")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "ProcessingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_configuration: Option<ProcessingConfiguration>,
    #[serde(rename = "RetryOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_options: Option<RedshiftRetryOptions>,
    #[serde(rename = "RoleARN")]
    #[serde(default)]
    pub role_a_r_n: String,
    #[serde(rename = "S3BackupConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_backup_configuration: Option<S3DestinationConfiguration>,
    #[serde(rename = "S3BackupMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_backup_mode: Option<String>,
    #[serde(rename = "S3Configuration")]
    #[serde(default)]
    pub s3_configuration: S3DestinationConfiguration,
    #[serde(rename = "SecretsManagerConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_configuration: Option<SecretsManagerConfiguration>,
    #[serde(rename = "Username")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CopyCommand {
    #[serde(rename = "CopyOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_options: Option<String>,
    #[serde(rename = "DataTableColumns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_table_columns: Option<String>,
    #[serde(rename = "DataTableName")]
    #[serde(default)]
    pub data_table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RedshiftRetryOptions {
    #[serde(rename = "DurationInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_in_seconds: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SnowflakeDestinationConfiguration {
    #[serde(rename = "AccountUrl")]
    #[serde(default)]
    pub account_url: String,
    #[serde(rename = "BufferingHints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffering_hints: Option<SnowflakeBufferingHints>,
    #[serde(rename = "CloudWatchLoggingOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,
    #[serde(rename = "ContentColumnName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_column_name: Option<String>,
    #[serde(rename = "DataLoadingOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_loading_option: Option<String>,
    #[serde(rename = "Database")]
    #[serde(default)]
    pub database: String,
    #[serde(rename = "KeyPassphrase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_passphrase: Option<String>,
    #[serde(rename = "MetaDataColumnName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta_data_column_name: Option<String>,
    #[serde(rename = "PrivateKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
    #[serde(rename = "ProcessingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_configuration: Option<ProcessingConfiguration>,
    #[serde(rename = "RetryOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_options: Option<SnowflakeRetryOptions>,
    #[serde(rename = "RoleARN")]
    #[serde(default)]
    pub role_a_r_n: String,
    #[serde(rename = "S3BackupMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_backup_mode: Option<String>,
    #[serde(rename = "S3Configuration")]
    #[serde(default)]
    pub s3_configuration: S3DestinationConfiguration,
    #[serde(rename = "Schema")]
    #[serde(default)]
    pub schema: String,
    #[serde(rename = "SecretsManagerConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_configuration: Option<SecretsManagerConfiguration>,
    #[serde(rename = "SnowflakeRoleConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snowflake_role_configuration: Option<SnowflakeRoleConfiguration>,
    #[serde(rename = "SnowflakeVpcConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snowflake_vpc_configuration: Option<SnowflakeVpcConfiguration>,
    #[serde(rename = "Table")]
    #[serde(default)]
    pub table: String,
    #[serde(rename = "User")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SnowflakeBufferingHints {
    #[serde(rename = "IntervalInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_in_seconds: Option<i32>,
    #[serde(rename = "SizeInMBs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_in_m_bs: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SnowflakeRetryOptions {
    #[serde(rename = "DurationInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_in_seconds: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SnowflakeRoleConfiguration {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "SnowflakeRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snowflake_role: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SnowflakeVpcConfiguration {
    #[serde(rename = "PrivateLinkVpceId")]
    #[serde(default)]
    pub private_link_vpce_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SplunkDestinationConfiguration {
    #[serde(rename = "BufferingHints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffering_hints: Option<SplunkBufferingHints>,
    #[serde(rename = "CloudWatchLoggingOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,
    #[serde(rename = "HECAcknowledgmentTimeoutInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h_e_c_acknowledgment_timeout_in_seconds: Option<i32>,
    #[serde(rename = "HECEndpoint")]
    #[serde(default)]
    pub h_e_c_endpoint: String,
    #[serde(rename = "HECEndpointType")]
    #[serde(default)]
    pub h_e_c_endpoint_type: String,
    #[serde(rename = "HECToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h_e_c_token: Option<String>,
    #[serde(rename = "ProcessingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_configuration: Option<ProcessingConfiguration>,
    #[serde(rename = "RetryOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_options: Option<SplunkRetryOptions>,
    #[serde(rename = "S3BackupMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_backup_mode: Option<String>,
    #[serde(rename = "S3Configuration")]
    #[serde(default)]
    pub s3_configuration: S3DestinationConfiguration,
    #[serde(rename = "SecretsManagerConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_configuration: Option<SecretsManagerConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SplunkBufferingHints {
    #[serde(rename = "IntervalInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_in_seconds: Option<i32>,
    #[serde(rename = "SizeInMBs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_in_m_bs: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SplunkRetryOptions {
    #[serde(rename = "DurationInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_in_seconds: Option<i32>,
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
pub struct CreateDeliveryStreamOutput {
    #[serde(rename = "DeliveryStreamARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_stream_a_r_n: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDeliveryStreamInput {
    #[serde(rename = "AllowForceDelete")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_force_delete: Option<bool>,
    #[serde(rename = "DeliveryStreamName")]
    #[serde(default)]
    pub delivery_stream_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDeliveryStreamOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDeliveryStreamInput {
    #[serde(rename = "DeliveryStreamName")]
    #[serde(default)]
    pub delivery_stream_name: String,
    #[serde(rename = "ExclusiveStartDestinationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_start_destination_id: Option<String>,
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDeliveryStreamOutput {
    #[serde(rename = "DeliveryStreamDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_stream_description: Option<DeliveryStreamDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeliveryStreamDescription {
    #[serde(rename = "CreateTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_timestamp: Option<f64>,
    #[serde(rename = "DeliveryStreamARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_stream_a_r_n: Option<String>,
    #[serde(rename = "DeliveryStreamEncryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_stream_encryption_configuration: Option<DeliveryStreamEncryptionConfiguration>,
    #[serde(rename = "DeliveryStreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_stream_name: Option<String>,
    #[serde(rename = "DeliveryStreamStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_stream_status: Option<String>,
    #[serde(rename = "DeliveryStreamType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_stream_type: Option<String>,
    #[serde(rename = "Destinations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<DestinationDescription>>,
    #[serde(rename = "FailureDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_description: Option<FailureDescription>,
    #[serde(rename = "HasMoreDestinations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more_destinations: Option<bool>,
    #[serde(rename = "LastUpdateTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_timestamp: Option<f64>,
    #[serde(rename = "Source")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<SourceDescription>,
    #[serde(rename = "VersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeliveryStreamEncryptionConfiguration {
    #[serde(rename = "FailureDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_description: Option<FailureDescription>,
    #[serde(rename = "KeyARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_a_r_n: Option<String>,
    #[serde(rename = "KeyType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_type: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FailureDescription {
    #[serde(rename = "Details")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DestinationDescription {
    #[serde(rename = "AmazonOpenSearchServerlessDestinationDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_open_search_serverless_destination_description:
        Option<AmazonOpenSearchServerlessDestinationDescription>,
    #[serde(rename = "AmazonopensearchserviceDestinationDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazonopensearchservice_destination_description:
        Option<AmazonopensearchserviceDestinationDescription>,
    #[serde(rename = "DestinationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_id: Option<String>,
    #[serde(rename = "ElasticsearchDestinationDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elasticsearch_destination_description: Option<ElasticsearchDestinationDescription>,
    #[serde(rename = "ExtendedS3DestinationDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_s3_destination_description: Option<ExtendedS3DestinationDescription>,
    #[serde(rename = "HttpEndpointDestinationDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_endpoint_destination_description: Option<HttpEndpointDestinationDescription>,
    #[serde(rename = "IcebergDestinationDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iceberg_destination_description: Option<IcebergDestinationDescription>,
    #[serde(rename = "RedshiftDestinationDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redshift_destination_description: Option<RedshiftDestinationDescription>,
    #[serde(rename = "S3DestinationDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_destination_description: Option<S3DestinationDescription>,
    #[serde(rename = "SnowflakeDestinationDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snowflake_destination_description: Option<SnowflakeDestinationDescription>,
    #[serde(rename = "SplunkDestinationDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splunk_destination_description: Option<SplunkDestinationDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AmazonOpenSearchServerlessDestinationDescription {
    #[serde(rename = "BufferingHints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffering_hints: Option<AmazonOpenSearchServerlessBufferingHints>,
    #[serde(rename = "CloudWatchLoggingOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,
    #[serde(rename = "CollectionEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_endpoint: Option<String>,
    #[serde(rename = "IndexName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,
    #[serde(rename = "ProcessingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_configuration: Option<ProcessingConfiguration>,
    #[serde(rename = "RetryOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_options: Option<AmazonOpenSearchServerlessRetryOptions>,
    #[serde(rename = "RoleARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_a_r_n: Option<String>,
    #[serde(rename = "S3BackupMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_backup_mode: Option<String>,
    #[serde(rename = "S3DestinationDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_destination_description: Option<S3DestinationDescription>,
    #[serde(rename = "VpcConfigurationDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_configuration_description: Option<VpcConfigurationDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3DestinationDescription {
    #[serde(rename = "BucketARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_a_r_n: Option<String>,
    #[serde(rename = "BufferingHints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffering_hints: Option<BufferingHints>,
    #[serde(rename = "CloudWatchLoggingOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,
    #[serde(rename = "CompressionFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compression_format: Option<String>,
    #[serde(rename = "EncryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
    #[serde(rename = "ErrorOutputPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_output_prefix: Option<String>,
    #[serde(rename = "Prefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "RoleARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_a_r_n: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VpcConfigurationDescription {
    #[serde(rename = "RoleARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_a_r_n: Option<String>,
    #[serde(rename = "SecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(rename = "SubnetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AmazonopensearchserviceDestinationDescription {
    #[serde(rename = "BufferingHints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffering_hints: Option<AmazonopensearchserviceBufferingHints>,
    #[serde(rename = "CloudWatchLoggingOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,
    #[serde(rename = "ClusterEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_endpoint: Option<String>,
    #[serde(rename = "DocumentIdOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_id_options: Option<DocumentIdOptions>,
    #[serde(rename = "DomainARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_a_r_n: Option<String>,
    #[serde(rename = "IndexName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,
    #[serde(rename = "IndexRotationPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_rotation_period: Option<String>,
    #[serde(rename = "ProcessingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_configuration: Option<ProcessingConfiguration>,
    #[serde(rename = "RetryOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_options: Option<AmazonopensearchserviceRetryOptions>,
    #[serde(rename = "RoleARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_a_r_n: Option<String>,
    #[serde(rename = "S3BackupMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_backup_mode: Option<String>,
    #[serde(rename = "S3DestinationDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_destination_description: Option<S3DestinationDescription>,
    #[serde(rename = "TypeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
    #[serde(rename = "VpcConfigurationDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_configuration_description: Option<VpcConfigurationDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ElasticsearchDestinationDescription {
    #[serde(rename = "BufferingHints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffering_hints: Option<ElasticsearchBufferingHints>,
    #[serde(rename = "CloudWatchLoggingOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,
    #[serde(rename = "ClusterEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_endpoint: Option<String>,
    #[serde(rename = "DocumentIdOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_id_options: Option<DocumentIdOptions>,
    #[serde(rename = "DomainARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_a_r_n: Option<String>,
    #[serde(rename = "IndexName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,
    #[serde(rename = "IndexRotationPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_rotation_period: Option<String>,
    #[serde(rename = "ProcessingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_configuration: Option<ProcessingConfiguration>,
    #[serde(rename = "RetryOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_options: Option<ElasticsearchRetryOptions>,
    #[serde(rename = "RoleARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_a_r_n: Option<String>,
    #[serde(rename = "S3BackupMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_backup_mode: Option<String>,
    #[serde(rename = "S3DestinationDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_destination_description: Option<S3DestinationDescription>,
    #[serde(rename = "TypeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
    #[serde(rename = "VpcConfigurationDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_configuration_description: Option<VpcConfigurationDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExtendedS3DestinationDescription {
    #[serde(rename = "BucketARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_a_r_n: Option<String>,
    #[serde(rename = "BufferingHints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffering_hints: Option<BufferingHints>,
    #[serde(rename = "CloudWatchLoggingOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,
    #[serde(rename = "CompressionFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compression_format: Option<String>,
    #[serde(rename = "CustomTimeZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_time_zone: Option<String>,
    #[serde(rename = "DataFormatConversionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_format_conversion_configuration: Option<DataFormatConversionConfiguration>,
    #[serde(rename = "DynamicPartitioningConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_partitioning_configuration: Option<DynamicPartitioningConfiguration>,
    #[serde(rename = "EncryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
    #[serde(rename = "ErrorOutputPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_output_prefix: Option<String>,
    #[serde(rename = "FileExtension")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_extension: Option<String>,
    #[serde(rename = "Prefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "ProcessingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_configuration: Option<ProcessingConfiguration>,
    #[serde(rename = "RoleARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_a_r_n: Option<String>,
    #[serde(rename = "S3BackupDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_backup_description: Option<S3DestinationDescription>,
    #[serde(rename = "S3BackupMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_backup_mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HttpEndpointDestinationDescription {
    #[serde(rename = "BufferingHints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffering_hints: Option<HttpEndpointBufferingHints>,
    #[serde(rename = "CloudWatchLoggingOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,
    #[serde(rename = "EndpointConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_configuration: Option<HttpEndpointDescription>,
    #[serde(rename = "ProcessingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_configuration: Option<ProcessingConfiguration>,
    #[serde(rename = "RequestConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_configuration: Option<HttpEndpointRequestConfiguration>,
    #[serde(rename = "RetryOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_options: Option<HttpEndpointRetryOptions>,
    #[serde(rename = "RoleARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_a_r_n: Option<String>,
    #[serde(rename = "S3BackupMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_backup_mode: Option<String>,
    #[serde(rename = "S3DestinationDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_destination_description: Option<S3DestinationDescription>,
    #[serde(rename = "SecretsManagerConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_configuration: Option<SecretsManagerConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HttpEndpointDescription {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Url")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IcebergDestinationDescription {
    #[serde(rename = "AppendOnly")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub append_only: Option<bool>,
    #[serde(rename = "BufferingHints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffering_hints: Option<BufferingHints>,
    #[serde(rename = "CatalogConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_configuration: Option<CatalogConfiguration>,
    #[serde(rename = "CloudWatchLoggingOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,
    #[serde(rename = "DestinationTableConfigurationList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_table_configuration_list: Option<Vec<DestinationTableConfiguration>>,
    #[serde(rename = "ProcessingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_configuration: Option<ProcessingConfiguration>,
    #[serde(rename = "RetryOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_options: Option<RetryOptions>,
    #[serde(rename = "RoleARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_a_r_n: Option<String>,
    #[serde(rename = "S3BackupMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_backup_mode: Option<String>,
    #[serde(rename = "S3DestinationDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_destination_description: Option<S3DestinationDescription>,
    #[serde(rename = "SchemaEvolutionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_evolution_configuration: Option<SchemaEvolutionConfiguration>,
    #[serde(rename = "TableCreationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_creation_configuration: Option<TableCreationConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RedshiftDestinationDescription {
    #[serde(rename = "CloudWatchLoggingOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,
    #[serde(rename = "ClusterJDBCURL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_j_d_b_c_u_r_l: Option<String>,
    #[serde(rename = "CopyCommand")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_command: Option<CopyCommand>,
    #[serde(rename = "ProcessingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_configuration: Option<ProcessingConfiguration>,
    #[serde(rename = "RetryOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_options: Option<RedshiftRetryOptions>,
    #[serde(rename = "RoleARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_a_r_n: Option<String>,
    #[serde(rename = "S3BackupDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_backup_description: Option<S3DestinationDescription>,
    #[serde(rename = "S3BackupMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_backup_mode: Option<String>,
    #[serde(rename = "S3DestinationDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_destination_description: Option<S3DestinationDescription>,
    #[serde(rename = "SecretsManagerConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_configuration: Option<SecretsManagerConfiguration>,
    #[serde(rename = "Username")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SnowflakeDestinationDescription {
    #[serde(rename = "AccountUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_url: Option<String>,
    #[serde(rename = "BufferingHints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffering_hints: Option<SnowflakeBufferingHints>,
    #[serde(rename = "CloudWatchLoggingOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,
    #[serde(rename = "ContentColumnName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_column_name: Option<String>,
    #[serde(rename = "DataLoadingOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_loading_option: Option<String>,
    #[serde(rename = "Database")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    #[serde(rename = "MetaDataColumnName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta_data_column_name: Option<String>,
    #[serde(rename = "ProcessingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_configuration: Option<ProcessingConfiguration>,
    #[serde(rename = "RetryOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_options: Option<SnowflakeRetryOptions>,
    #[serde(rename = "RoleARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_a_r_n: Option<String>,
    #[serde(rename = "S3BackupMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_backup_mode: Option<String>,
    #[serde(rename = "S3DestinationDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_destination_description: Option<S3DestinationDescription>,
    #[serde(rename = "Schema")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[serde(rename = "SecretsManagerConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_configuration: Option<SecretsManagerConfiguration>,
    #[serde(rename = "SnowflakeRoleConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snowflake_role_configuration: Option<SnowflakeRoleConfiguration>,
    #[serde(rename = "SnowflakeVpcConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snowflake_vpc_configuration: Option<SnowflakeVpcConfiguration>,
    #[serde(rename = "Table")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<String>,
    #[serde(rename = "User")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SplunkDestinationDescription {
    #[serde(rename = "BufferingHints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffering_hints: Option<SplunkBufferingHints>,
    #[serde(rename = "CloudWatchLoggingOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,
    #[serde(rename = "HECAcknowledgmentTimeoutInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h_e_c_acknowledgment_timeout_in_seconds: Option<i32>,
    #[serde(rename = "HECEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h_e_c_endpoint: Option<String>,
    #[serde(rename = "HECEndpointType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h_e_c_endpoint_type: Option<String>,
    #[serde(rename = "HECToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h_e_c_token: Option<String>,
    #[serde(rename = "ProcessingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_configuration: Option<ProcessingConfiguration>,
    #[serde(rename = "RetryOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_options: Option<SplunkRetryOptions>,
    #[serde(rename = "S3BackupMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_backup_mode: Option<String>,
    #[serde(rename = "S3DestinationDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_destination_description: Option<S3DestinationDescription>,
    #[serde(rename = "SecretsManagerConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_configuration: Option<SecretsManagerConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SourceDescription {
    #[serde(rename = "DatabaseSourceDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_source_description: Option<DatabaseSourceDescription>,
    #[serde(rename = "DirectPutSourceDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_put_source_description: Option<DirectPutSourceDescription>,
    #[serde(rename = "KinesisStreamSourceDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_stream_source_description: Option<KinesisStreamSourceDescription>,
    #[serde(rename = "MSKSourceDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_s_k_source_description: Option<MSKSourceDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DatabaseSourceDescription {
    #[serde(rename = "Columns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<DatabaseColumnList>,
    #[serde(rename = "DatabaseSourceAuthenticationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_source_authentication_configuration:
        Option<DatabaseSourceAuthenticationConfiguration>,
    #[serde(rename = "DatabaseSourceVPCConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_source_v_p_c_configuration: Option<DatabaseSourceVPCConfiguration>,
    #[serde(rename = "Databases")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub databases: Option<DatabaseList>,
    #[serde(rename = "Endpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "SSLMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_l_mode: Option<String>,
    #[serde(rename = "SnapshotInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_info: Option<Vec<DatabaseSnapshotInfo>>,
    #[serde(rename = "SnapshotWatermarkTable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_watermark_table: Option<String>,
    #[serde(rename = "SurrogateKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surrogate_keys: Option<Vec<String>>,
    #[serde(rename = "Tables")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tables: Option<DatabaseTableList>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DatabaseSnapshotInfo {
    #[serde(rename = "FailureDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_description: Option<FailureDescription>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "RequestTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_timestamp: Option<f64>,
    #[serde(rename = "RequestedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_by: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Table")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DirectPutSourceDescription {
    #[serde(rename = "ThroughputHintInMBs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throughput_hint_in_m_bs: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KinesisStreamSourceDescription {
    #[serde(rename = "DeliveryStartTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_start_timestamp: Option<f64>,
    #[serde(rename = "KinesisStreamARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_stream_a_r_n: Option<String>,
    #[serde(rename = "RoleARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_a_r_n: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MSKSourceDescription {
    #[serde(rename = "AuthenticationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_configuration: Option<AuthenticationConfiguration>,
    #[serde(rename = "DeliveryStartTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_start_timestamp: Option<f64>,
    #[serde(rename = "MSKClusterARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_s_k_cluster_a_r_n: Option<String>,
    #[serde(rename = "ReadFromTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_from_timestamp: Option<f64>,
    #[serde(rename = "TopicName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDeliveryStreamsInput {
    #[serde(rename = "DeliveryStreamType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_stream_type: Option<String>,
    #[serde(rename = "ExclusiveStartDeliveryStreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_start_delivery_stream_name: Option<String>,
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDeliveryStreamsOutput {
    #[serde(rename = "DeliveryStreamNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_stream_names: Option<Vec<String>>,
    #[serde(rename = "HasMoreDeliveryStreams")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more_delivery_streams: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForDeliveryStreamInput {
    #[serde(rename = "DeliveryStreamName")]
    #[serde(default)]
    pub delivery_stream_name: String,
    #[serde(rename = "ExclusiveStartTagKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_start_tag_key: Option<String>,
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForDeliveryStreamOutput {
    #[serde(rename = "HasMoreTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more_tags: Option<bool>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutRecordBatchInput {
    #[serde(rename = "DeliveryStreamName")]
    #[serde(default)]
    pub delivery_stream_name: String,
    #[serde(rename = "Records")]
    #[serde(default)]
    pub records: Vec<Record>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Record {
    #[serde(rename = "Data")]
    #[serde(default)]
    pub data: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutRecordBatchOutput {
    #[serde(rename = "Encrypted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    #[serde(rename = "FailedPutCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_put_count: Option<i32>,
    #[serde(rename = "RequestResponses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_responses: Option<Vec<PutRecordBatchResponseEntry>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutRecordBatchResponseEntry {
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "RecordId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutRecordInput {
    #[serde(rename = "DeliveryStreamName")]
    #[serde(default)]
    pub delivery_stream_name: String,
    #[serde(rename = "Record")]
    #[serde(default)]
    pub record: Record,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutRecordOutput {
    #[serde(rename = "Encrypted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    #[serde(rename = "RecordId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartDeliveryStreamEncryptionInput {
    #[serde(rename = "DeliveryStreamEncryptionConfigurationInput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_stream_encryption_configuration_input:
        Option<DeliveryStreamEncryptionConfigurationInput>,
    #[serde(rename = "DeliveryStreamName")]
    #[serde(default)]
    pub delivery_stream_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartDeliveryStreamEncryptionOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopDeliveryStreamEncryptionInput {
    #[serde(rename = "DeliveryStreamName")]
    #[serde(default)]
    pub delivery_stream_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopDeliveryStreamEncryptionOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagDeliveryStreamInput {
    #[serde(rename = "DeliveryStreamName")]
    #[serde(default)]
    pub delivery_stream_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagDeliveryStreamOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagDeliveryStreamInput {
    #[serde(rename = "DeliveryStreamName")]
    #[serde(default)]
    pub delivery_stream_name: String,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagDeliveryStreamOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDestinationInput {
    #[serde(rename = "AmazonOpenSearchServerlessDestinationUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_open_search_serverless_destination_update:
        Option<AmazonOpenSearchServerlessDestinationUpdate>,
    #[serde(rename = "AmazonopensearchserviceDestinationUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazonopensearchservice_destination_update:
        Option<AmazonopensearchserviceDestinationUpdate>,
    #[serde(rename = "CurrentDeliveryStreamVersionId")]
    #[serde(default)]
    pub current_delivery_stream_version_id: String,
    #[serde(rename = "DeliveryStreamName")]
    #[serde(default)]
    pub delivery_stream_name: String,
    #[serde(rename = "DestinationId")]
    #[serde(default)]
    pub destination_id: String,
    #[serde(rename = "ElasticsearchDestinationUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elasticsearch_destination_update: Option<ElasticsearchDestinationUpdate>,
    #[serde(rename = "ExtendedS3DestinationUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_s3_destination_update: Option<ExtendedS3DestinationUpdate>,
    #[serde(rename = "HttpEndpointDestinationUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_endpoint_destination_update: Option<HttpEndpointDestinationUpdate>,
    #[serde(rename = "IcebergDestinationUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iceberg_destination_update: Option<IcebergDestinationUpdate>,
    #[serde(rename = "RedshiftDestinationUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redshift_destination_update: Option<RedshiftDestinationUpdate>,
    #[serde(rename = "S3DestinationUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_destination_update: Option<S3DestinationUpdate>,
    #[serde(rename = "SnowflakeDestinationUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snowflake_destination_update: Option<SnowflakeDestinationUpdate>,
    #[serde(rename = "SplunkDestinationUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splunk_destination_update: Option<SplunkDestinationUpdate>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AmazonOpenSearchServerlessDestinationUpdate {
    #[serde(rename = "BufferingHints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffering_hints: Option<AmazonOpenSearchServerlessBufferingHints>,
    #[serde(rename = "CloudWatchLoggingOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,
    #[serde(rename = "CollectionEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_endpoint: Option<String>,
    #[serde(rename = "IndexName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,
    #[serde(rename = "ProcessingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_configuration: Option<ProcessingConfiguration>,
    #[serde(rename = "RetryOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_options: Option<AmazonOpenSearchServerlessRetryOptions>,
    #[serde(rename = "RoleARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_a_r_n: Option<String>,
    #[serde(rename = "S3Update")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_update: Option<S3DestinationUpdate>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3DestinationUpdate {
    #[serde(rename = "BucketARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_a_r_n: Option<String>,
    #[serde(rename = "BufferingHints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffering_hints: Option<BufferingHints>,
    #[serde(rename = "CloudWatchLoggingOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,
    #[serde(rename = "CompressionFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compression_format: Option<String>,
    #[serde(rename = "EncryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
    #[serde(rename = "ErrorOutputPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_output_prefix: Option<String>,
    #[serde(rename = "Prefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "RoleARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_a_r_n: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AmazonopensearchserviceDestinationUpdate {
    #[serde(rename = "BufferingHints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffering_hints: Option<AmazonopensearchserviceBufferingHints>,
    #[serde(rename = "CloudWatchLoggingOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,
    #[serde(rename = "ClusterEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_endpoint: Option<String>,
    #[serde(rename = "DocumentIdOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_id_options: Option<DocumentIdOptions>,
    #[serde(rename = "DomainARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_a_r_n: Option<String>,
    #[serde(rename = "IndexName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,
    #[serde(rename = "IndexRotationPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_rotation_period: Option<String>,
    #[serde(rename = "ProcessingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_configuration: Option<ProcessingConfiguration>,
    #[serde(rename = "RetryOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_options: Option<AmazonopensearchserviceRetryOptions>,
    #[serde(rename = "RoleARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_a_r_n: Option<String>,
    #[serde(rename = "S3Update")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_update: Option<S3DestinationUpdate>,
    #[serde(rename = "TypeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ElasticsearchDestinationUpdate {
    #[serde(rename = "BufferingHints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffering_hints: Option<ElasticsearchBufferingHints>,
    #[serde(rename = "CloudWatchLoggingOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,
    #[serde(rename = "ClusterEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_endpoint: Option<String>,
    #[serde(rename = "DocumentIdOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_id_options: Option<DocumentIdOptions>,
    #[serde(rename = "DomainARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_a_r_n: Option<String>,
    #[serde(rename = "IndexName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,
    #[serde(rename = "IndexRotationPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_rotation_period: Option<String>,
    #[serde(rename = "ProcessingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_configuration: Option<ProcessingConfiguration>,
    #[serde(rename = "RetryOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_options: Option<ElasticsearchRetryOptions>,
    #[serde(rename = "RoleARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_a_r_n: Option<String>,
    #[serde(rename = "S3Update")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_update: Option<S3DestinationUpdate>,
    #[serde(rename = "TypeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExtendedS3DestinationUpdate {
    #[serde(rename = "BucketARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_a_r_n: Option<String>,
    #[serde(rename = "BufferingHints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffering_hints: Option<BufferingHints>,
    #[serde(rename = "CloudWatchLoggingOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,
    #[serde(rename = "CompressionFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compression_format: Option<String>,
    #[serde(rename = "CustomTimeZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_time_zone: Option<String>,
    #[serde(rename = "DataFormatConversionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_format_conversion_configuration: Option<DataFormatConversionConfiguration>,
    #[serde(rename = "DynamicPartitioningConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_partitioning_configuration: Option<DynamicPartitioningConfiguration>,
    #[serde(rename = "EncryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
    #[serde(rename = "ErrorOutputPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_output_prefix: Option<String>,
    #[serde(rename = "FileExtension")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_extension: Option<String>,
    #[serde(rename = "Prefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "ProcessingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_configuration: Option<ProcessingConfiguration>,
    #[serde(rename = "RoleARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_a_r_n: Option<String>,
    #[serde(rename = "S3BackupMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_backup_mode: Option<String>,
    #[serde(rename = "S3BackupUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_backup_update: Option<S3DestinationUpdate>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HttpEndpointDestinationUpdate {
    #[serde(rename = "BufferingHints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffering_hints: Option<HttpEndpointBufferingHints>,
    #[serde(rename = "CloudWatchLoggingOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,
    #[serde(rename = "EndpointConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_configuration: Option<HttpEndpointConfiguration>,
    #[serde(rename = "ProcessingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_configuration: Option<ProcessingConfiguration>,
    #[serde(rename = "RequestConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_configuration: Option<HttpEndpointRequestConfiguration>,
    #[serde(rename = "RetryOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_options: Option<HttpEndpointRetryOptions>,
    #[serde(rename = "RoleARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_a_r_n: Option<String>,
    #[serde(rename = "S3BackupMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_backup_mode: Option<String>,
    #[serde(rename = "S3Update")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_update: Option<S3DestinationUpdate>,
    #[serde(rename = "SecretsManagerConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_configuration: Option<SecretsManagerConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IcebergDestinationUpdate {
    #[serde(rename = "AppendOnly")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub append_only: Option<bool>,
    #[serde(rename = "BufferingHints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffering_hints: Option<BufferingHints>,
    #[serde(rename = "CatalogConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_configuration: Option<CatalogConfiguration>,
    #[serde(rename = "CloudWatchLoggingOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,
    #[serde(rename = "DestinationTableConfigurationList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_table_configuration_list: Option<Vec<DestinationTableConfiguration>>,
    #[serde(rename = "ProcessingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_configuration: Option<ProcessingConfiguration>,
    #[serde(rename = "RetryOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_options: Option<RetryOptions>,
    #[serde(rename = "RoleARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_a_r_n: Option<String>,
    #[serde(rename = "S3BackupMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_backup_mode: Option<String>,
    #[serde(rename = "S3Configuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_configuration: Option<S3DestinationConfiguration>,
    #[serde(rename = "SchemaEvolutionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_evolution_configuration: Option<SchemaEvolutionConfiguration>,
    #[serde(rename = "TableCreationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_creation_configuration: Option<TableCreationConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RedshiftDestinationUpdate {
    #[serde(rename = "CloudWatchLoggingOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,
    #[serde(rename = "ClusterJDBCURL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_j_d_b_c_u_r_l: Option<String>,
    #[serde(rename = "CopyCommand")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_command: Option<CopyCommand>,
    #[serde(rename = "Password")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "ProcessingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_configuration: Option<ProcessingConfiguration>,
    #[serde(rename = "RetryOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_options: Option<RedshiftRetryOptions>,
    #[serde(rename = "RoleARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_a_r_n: Option<String>,
    #[serde(rename = "S3BackupMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_backup_mode: Option<String>,
    #[serde(rename = "S3BackupUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_backup_update: Option<S3DestinationUpdate>,
    #[serde(rename = "S3Update")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_update: Option<S3DestinationUpdate>,
    #[serde(rename = "SecretsManagerConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_configuration: Option<SecretsManagerConfiguration>,
    #[serde(rename = "Username")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SnowflakeDestinationUpdate {
    #[serde(rename = "AccountUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_url: Option<String>,
    #[serde(rename = "BufferingHints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffering_hints: Option<SnowflakeBufferingHints>,
    #[serde(rename = "CloudWatchLoggingOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,
    #[serde(rename = "ContentColumnName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_column_name: Option<String>,
    #[serde(rename = "DataLoadingOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_loading_option: Option<String>,
    #[serde(rename = "Database")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    #[serde(rename = "KeyPassphrase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_passphrase: Option<String>,
    #[serde(rename = "MetaDataColumnName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta_data_column_name: Option<String>,
    #[serde(rename = "PrivateKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
    #[serde(rename = "ProcessingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_configuration: Option<ProcessingConfiguration>,
    #[serde(rename = "RetryOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_options: Option<SnowflakeRetryOptions>,
    #[serde(rename = "RoleARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_a_r_n: Option<String>,
    #[serde(rename = "S3BackupMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_backup_mode: Option<String>,
    #[serde(rename = "S3Update")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_update: Option<S3DestinationUpdate>,
    #[serde(rename = "Schema")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[serde(rename = "SecretsManagerConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_configuration: Option<SecretsManagerConfiguration>,
    #[serde(rename = "SnowflakeRoleConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snowflake_role_configuration: Option<SnowflakeRoleConfiguration>,
    #[serde(rename = "Table")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<String>,
    #[serde(rename = "User")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SplunkDestinationUpdate {
    #[serde(rename = "BufferingHints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffering_hints: Option<SplunkBufferingHints>,
    #[serde(rename = "CloudWatchLoggingOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,
    #[serde(rename = "HECAcknowledgmentTimeoutInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h_e_c_acknowledgment_timeout_in_seconds: Option<i32>,
    #[serde(rename = "HECEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h_e_c_endpoint: Option<String>,
    #[serde(rename = "HECEndpointType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h_e_c_endpoint_type: Option<String>,
    #[serde(rename = "HECToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h_e_c_token: Option<String>,
    #[serde(rename = "ProcessingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_configuration: Option<ProcessingConfiguration>,
    #[serde(rename = "RetryOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_options: Option<SplunkRetryOptions>,
    #[serde(rename = "S3BackupMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_backup_mode: Option<String>,
    #[serde(rename = "S3Update")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_update: Option<S3DestinationUpdate>,
    #[serde(rename = "SecretsManagerConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_configuration: Option<SecretsManagerConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDestinationOutput {}
