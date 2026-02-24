//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-pipes

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePipeRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "DesiredState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_state: Option<String>,
    #[serde(rename = "Enrichment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enrichment: Option<String>,
    #[serde(rename = "EnrichmentParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enrichment_parameters: Option<PipeEnrichmentParameters>,
    #[serde(rename = "KmsKeyIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_identifier: Option<String>,
    #[serde(rename = "LogConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_configuration: Option<PipeLogConfigurationParameters>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(rename = "Source")]
    #[serde(default)]
    pub source: String,
    #[serde(rename = "SourceParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_parameters: Option<PipeSourceParameters>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Target")]
    #[serde(default)]
    pub target: String,
    #[serde(rename = "TargetParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_parameters: Option<PipeTargetParameters>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PipeEnrichmentParameters {
    #[serde(rename = "HttpParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_parameters: Option<PipeEnrichmentHttpParameters>,
    #[serde(rename = "InputTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_template: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PipeEnrichmentHttpParameters {
    #[serde(rename = "HeaderParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "PathParameterValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_parameter_values: Option<Vec<String>>,
    #[serde(rename = "QueryStringParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_string_parameters: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PipeLogConfigurationParameters {
    #[serde(rename = "CloudwatchLogsLogDestination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloudwatch_logs_log_destination: Option<CloudwatchLogsLogDestinationParameters>,
    #[serde(rename = "FirehoseLogDestination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firehose_log_destination: Option<FirehoseLogDestinationParameters>,
    #[serde(rename = "IncludeExecutionData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_execution_data: Option<Vec<String>>,
    #[serde(rename = "Level")]
    #[serde(default)]
    pub level: String,
    #[serde(rename = "S3LogDestination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_log_destination: Option<S3LogDestinationParameters>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CloudwatchLogsLogDestinationParameters {
    #[serde(rename = "LogGroupArn")]
    #[serde(default)]
    pub log_group_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FirehoseLogDestinationParameters {
    #[serde(rename = "DeliveryStreamArn")]
    #[serde(default)]
    pub delivery_stream_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3LogDestinationParameters {
    #[serde(rename = "BucketName")]
    #[serde(default)]
    pub bucket_name: String,
    #[serde(rename = "BucketOwner")]
    #[serde(default)]
    pub bucket_owner: String,
    #[serde(rename = "OutputFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
    #[serde(rename = "Prefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PipeSourceParameters {
    #[serde(rename = "ActiveMQBrokerParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_m_q_broker_parameters: Option<PipeSourceActiveMQBrokerParameters>,
    #[serde(rename = "DynamoDBStreamParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamo_d_b_stream_parameters: Option<PipeSourceDynamoDBStreamParameters>,
    #[serde(rename = "FilterCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_criteria: Option<FilterCriteria>,
    #[serde(rename = "KinesisStreamParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_stream_parameters: Option<PipeSourceKinesisStreamParameters>,
    #[serde(rename = "ManagedStreamingKafkaParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_streaming_kafka_parameters: Option<PipeSourceManagedStreamingKafkaParameters>,
    #[serde(rename = "RabbitMQBrokerParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rabbit_m_q_broker_parameters: Option<PipeSourceRabbitMQBrokerParameters>,
    #[serde(rename = "SelfManagedKafkaParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_managed_kafka_parameters: Option<PipeSourceSelfManagedKafkaParameters>,
    #[serde(rename = "SqsQueueParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sqs_queue_parameters: Option<PipeSourceSqsQueueParameters>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PipeSourceActiveMQBrokerParameters {
    #[serde(rename = "BatchSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<i32>,
    #[serde(rename = "Credentials")]
    #[serde(default)]
    pub credentials: MQBrokerAccessCredentials,
    #[serde(rename = "MaximumBatchingWindowInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_batching_window_in_seconds: Option<i32>,
    #[serde(rename = "QueueName")]
    #[serde(default)]
    pub queue_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MQBrokerAccessCredentials {
    #[serde(rename = "BasicAuth")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basic_auth: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PipeSourceDynamoDBStreamParameters {
    #[serde(rename = "BatchSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<i32>,
    #[serde(rename = "DeadLetterConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dead_letter_config: Option<DeadLetterConfig>,
    #[serde(rename = "MaximumBatchingWindowInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_batching_window_in_seconds: Option<i32>,
    #[serde(rename = "MaximumRecordAgeInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_record_age_in_seconds: Option<i32>,
    #[serde(rename = "MaximumRetryAttempts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_retry_attempts: Option<i32>,
    #[serde(rename = "OnPartialBatchItemFailure")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_partial_batch_item_failure: Option<String>,
    #[serde(rename = "ParallelizationFactor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallelization_factor: Option<i32>,
    #[serde(rename = "StartingPosition")]
    #[serde(default)]
    pub starting_position: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeadLetterConfig {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FilterCriteria {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Filter {
    #[serde(rename = "Pattern")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PipeSourceKinesisStreamParameters {
    #[serde(rename = "BatchSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<i32>,
    #[serde(rename = "DeadLetterConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dead_letter_config: Option<DeadLetterConfig>,
    #[serde(rename = "MaximumBatchingWindowInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_batching_window_in_seconds: Option<i32>,
    #[serde(rename = "MaximumRecordAgeInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_record_age_in_seconds: Option<i32>,
    #[serde(rename = "MaximumRetryAttempts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_retry_attempts: Option<i32>,
    #[serde(rename = "OnPartialBatchItemFailure")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_partial_batch_item_failure: Option<String>,
    #[serde(rename = "ParallelizationFactor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallelization_factor: Option<i32>,
    #[serde(rename = "StartingPosition")]
    #[serde(default)]
    pub starting_position: String,
    #[serde(rename = "StartingPositionTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_position_timestamp: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PipeSourceManagedStreamingKafkaParameters {
    #[serde(rename = "BatchSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<i32>,
    #[serde(rename = "ConsumerGroupID")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_group_i_d: Option<String>,
    #[serde(rename = "Credentials")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<MSKAccessCredentials>,
    #[serde(rename = "MaximumBatchingWindowInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_batching_window_in_seconds: Option<i32>,
    #[serde(rename = "StartingPosition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_position: Option<String>,
    #[serde(rename = "TopicName")]
    #[serde(default)]
    pub topic_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MSKAccessCredentials {
    #[serde(rename = "ClientCertificateTlsAuth")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_certificate_tls_auth: Option<String>,
    #[serde(rename = "SaslScram512Auth")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sasl_scram512_auth: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PipeSourceRabbitMQBrokerParameters {
    #[serde(rename = "BatchSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<i32>,
    #[serde(rename = "Credentials")]
    #[serde(default)]
    pub credentials: MQBrokerAccessCredentials,
    #[serde(rename = "MaximumBatchingWindowInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_batching_window_in_seconds: Option<i32>,
    #[serde(rename = "QueueName")]
    #[serde(default)]
    pub queue_name: String,
    #[serde(rename = "VirtualHost")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_host: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PipeSourceSelfManagedKafkaParameters {
    #[serde(rename = "AdditionalBootstrapServers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_bootstrap_servers: Option<Vec<String>>,
    #[serde(rename = "BatchSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<i32>,
    #[serde(rename = "ConsumerGroupID")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_group_i_d: Option<String>,
    #[serde(rename = "Credentials")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<SelfManagedKafkaAccessConfigurationCredentials>,
    #[serde(rename = "MaximumBatchingWindowInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_batching_window_in_seconds: Option<i32>,
    #[serde(rename = "ServerRootCaCertificate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_root_ca_certificate: Option<String>,
    #[serde(rename = "StartingPosition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_position: Option<String>,
    #[serde(rename = "TopicName")]
    #[serde(default)]
    pub topic_name: String,
    #[serde(rename = "Vpc")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc: Option<SelfManagedKafkaAccessConfigurationVpc>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SelfManagedKafkaAccessConfigurationCredentials {
    #[serde(rename = "BasicAuth")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basic_auth: Option<String>,
    #[serde(rename = "ClientCertificateTlsAuth")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_certificate_tls_auth: Option<String>,
    #[serde(rename = "SaslScram256Auth")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sasl_scram256_auth: Option<String>,
    #[serde(rename = "SaslScram512Auth")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sasl_scram512_auth: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SelfManagedKafkaAccessConfigurationVpc {
    #[serde(rename = "SecurityGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group: Option<Vec<String>>,
    #[serde(rename = "Subnets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnets: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PipeSourceSqsQueueParameters {
    #[serde(rename = "BatchSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<i32>,
    #[serde(rename = "MaximumBatchingWindowInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_batching_window_in_seconds: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PipeTargetParameters {
    #[serde(rename = "BatchJobParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_job_parameters: Option<PipeTargetBatchJobParameters>,
    #[serde(rename = "CloudWatchLogsParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs_parameters: Option<PipeTargetCloudWatchLogsParameters>,
    #[serde(rename = "EcsTaskParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecs_task_parameters: Option<PipeTargetEcsTaskParameters>,
    #[serde(rename = "EventBridgeEventBusParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bridge_event_bus_parameters: Option<PipeTargetEventBridgeEventBusParameters>,
    #[serde(rename = "HttpParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_parameters: Option<PipeTargetHttpParameters>,
    #[serde(rename = "InputTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_template: Option<String>,
    #[serde(rename = "KinesisStreamParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_stream_parameters: Option<PipeTargetKinesisStreamParameters>,
    #[serde(rename = "LambdaFunctionParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_function_parameters: Option<PipeTargetLambdaFunctionParameters>,
    #[serde(rename = "RedshiftDataParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redshift_data_parameters: Option<PipeTargetRedshiftDataParameters>,
    #[serde(rename = "SageMakerPipelineParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sage_maker_pipeline_parameters: Option<PipeTargetSageMakerPipelineParameters>,
    #[serde(rename = "SqsQueueParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sqs_queue_parameters: Option<PipeTargetSqsQueueParameters>,
    #[serde(rename = "StepFunctionStateMachineParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_function_state_machine_parameters: Option<PipeTargetStateMachineParameters>,
    #[serde(rename = "TimestreamParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestream_parameters: Option<PipeTargetTimestreamParameters>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PipeTargetBatchJobParameters {
    #[serde(rename = "ArrayProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub array_properties: Option<BatchArrayProperties>,
    #[serde(rename = "ContainerOverrides")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_overrides: Option<BatchContainerOverrides>,
    #[serde(rename = "DependsOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depends_on: Option<Vec<BatchJobDependency>>,
    #[serde(rename = "JobDefinition")]
    #[serde(default)]
    pub job_definition: String,
    #[serde(rename = "JobName")]
    #[serde(default)]
    pub job_name: String,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "RetryStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_strategy: Option<BatchRetryStrategy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchArrayProperties {
    #[serde(rename = "Size")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchContainerOverrides {
    #[serde(rename = "Command")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    #[serde(rename = "Environment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<Vec<BatchEnvironmentVariable>>,
    #[serde(rename = "InstanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    #[serde(rename = "ResourceRequirements")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_requirements: Option<Vec<BatchResourceRequirement>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchEnvironmentVariable {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchResourceRequirement {
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchJobDependency {
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchRetryStrategy {
    #[serde(rename = "Attempts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempts: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PipeTargetCloudWatchLogsParameters {
    #[serde(rename = "LogStreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_stream_name: Option<String>,
    #[serde(rename = "Timestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PipeTargetEcsTaskParameters {
    #[serde(rename = "CapacityProviderStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_provider_strategy: Option<Vec<CapacityProviderStrategyItem>>,
    #[serde(rename = "EnableECSManagedTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_e_c_s_managed_tags: Option<bool>,
    #[serde(rename = "EnableExecuteCommand")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_execute_command: Option<bool>,
    #[serde(rename = "Group")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(rename = "LaunchType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_type: Option<String>,
    #[serde(rename = "NetworkConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_configuration: Option<NetworkConfiguration>,
    #[serde(rename = "Overrides")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overrides: Option<EcsTaskOverride>,
    #[serde(rename = "PlacementConstraints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_constraints: Option<Vec<PlacementConstraint>>,
    #[serde(rename = "PlacementStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_strategy: Option<Vec<PlacementStrategy>>,
    #[serde(rename = "PlatformVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_version: Option<String>,
    #[serde(rename = "PropagateTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub propagate_tags: Option<String>,
    #[serde(rename = "ReferenceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "TaskCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_count: Option<i32>,
    #[serde(rename = "TaskDefinitionArn")]
    #[serde(default)]
    pub task_definition_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CapacityProviderStrategyItem {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base: Option<i32>,
    #[serde(rename = "capacityProvider")]
    #[serde(default)]
    pub capacity_provider: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NetworkConfiguration {
    #[serde(rename = "awsvpcConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub awsvpc_configuration: Option<AwsVpcConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsVpcConfiguration {
    #[serde(rename = "AssignPublicIp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assign_public_ip: Option<String>,
    #[serde(rename = "SecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,
    #[serde(rename = "Subnets")]
    #[serde(default)]
    pub subnets: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EcsTaskOverride {
    #[serde(rename = "ContainerOverrides")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_overrides: Option<Vec<EcsContainerOverride>>,
    #[serde(rename = "Cpu")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<String>,
    #[serde(rename = "EphemeralStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ephemeral_storage: Option<EcsEphemeralStorage>,
    #[serde(rename = "ExecutionRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role_arn: Option<String>,
    #[serde(rename = "InferenceAcceleratorOverrides")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_accelerator_overrides: Option<Vec<EcsInferenceAcceleratorOverride>>,
    #[serde(rename = "Memory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<String>,
    #[serde(rename = "TaskRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EcsContainerOverride {
    #[serde(rename = "Command")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    #[serde(rename = "Cpu")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<i32>,
    #[serde(rename = "Environment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<Vec<EcsEnvironmentVariable>>,
    #[serde(rename = "EnvironmentFiles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_files: Option<Vec<EcsEnvironmentFile>>,
    #[serde(rename = "Memory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<i32>,
    #[serde(rename = "MemoryReservation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_reservation: Option<i32>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ResourceRequirements")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_requirements: Option<Vec<EcsResourceRequirement>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EcsEnvironmentVariable {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EcsEnvironmentFile {
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EcsResourceRequirement {
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EcsEphemeralStorage {
    #[serde(rename = "sizeInGiB")]
    #[serde(default)]
    pub size_in_gi_b: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EcsInferenceAcceleratorOverride {
    #[serde(rename = "deviceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    #[serde(rename = "deviceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PlacementConstraint {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PlacementStrategy {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
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
pub struct PipeTargetEventBridgeEventBusParameters {
    #[serde(rename = "DetailType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail_type: Option<String>,
    #[serde(rename = "EndpointId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_id: Option<String>,
    #[serde(rename = "Resources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<String>>,
    #[serde(rename = "Source")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "Time")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PipeTargetHttpParameters {
    #[serde(rename = "HeaderParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "PathParameterValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_parameter_values: Option<Vec<String>>,
    #[serde(rename = "QueryStringParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_string_parameters: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PipeTargetKinesisStreamParameters {
    #[serde(rename = "PartitionKey")]
    #[serde(default)]
    pub partition_key: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PipeTargetLambdaFunctionParameters {
    #[serde(rename = "InvocationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invocation_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PipeTargetRedshiftDataParameters {
    #[serde(rename = "Database")]
    #[serde(default)]
    pub database: String,
    #[serde(rename = "DbUser")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_user: Option<String>,
    #[serde(rename = "SecretManagerArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_manager_arn: Option<String>,
    #[serde(rename = "Sqls")]
    #[serde(default)]
    pub sqls: Vec<String>,
    #[serde(rename = "StatementName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_name: Option<String>,
    #[serde(rename = "WithEvent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_event: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PipeTargetSageMakerPipelineParameters {
    #[serde(rename = "PipelineParameterList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_parameter_list: Option<Vec<SageMakerPipelineParameter>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SageMakerPipelineParameter {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PipeTargetSqsQueueParameters {
    #[serde(rename = "MessageDeduplicationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_deduplication_id: Option<String>,
    #[serde(rename = "MessageGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_group_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PipeTargetStateMachineParameters {
    #[serde(rename = "InvocationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invocation_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PipeTargetTimestreamParameters {
    #[serde(rename = "DimensionMappings")]
    #[serde(default)]
    pub dimension_mappings: Vec<DimensionMapping>,
    #[serde(rename = "EpochTimeUnit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub epoch_time_unit: Option<String>,
    #[serde(rename = "MultiMeasureMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_measure_mappings: Option<Vec<MultiMeasureMapping>>,
    #[serde(rename = "SingleMeasureMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_measure_mappings: Option<Vec<SingleMeasureMapping>>,
    #[serde(rename = "TimeFieldType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_field_type: Option<String>,
    #[serde(rename = "TimeValue")]
    #[serde(default)]
    pub time_value: String,
    #[serde(rename = "TimestampFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_format: Option<String>,
    #[serde(rename = "VersionValue")]
    #[serde(default)]
    pub version_value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DimensionMapping {
    #[serde(rename = "DimensionName")]
    #[serde(default)]
    pub dimension_name: String,
    #[serde(rename = "DimensionValue")]
    #[serde(default)]
    pub dimension_value: String,
    #[serde(rename = "DimensionValueType")]
    #[serde(default)]
    pub dimension_value_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MultiMeasureMapping {
    #[serde(rename = "MultiMeasureAttributeMappings")]
    #[serde(default)]
    pub multi_measure_attribute_mappings: Vec<MultiMeasureAttributeMapping>,
    #[serde(rename = "MultiMeasureName")]
    #[serde(default)]
    pub multi_measure_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MultiMeasureAttributeMapping {
    #[serde(rename = "MeasureValue")]
    #[serde(default)]
    pub measure_value: String,
    #[serde(rename = "MeasureValueType")]
    #[serde(default)]
    pub measure_value_type: String,
    #[serde(rename = "MultiMeasureAttributeName")]
    #[serde(default)]
    pub multi_measure_attribute_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SingleMeasureMapping {
    #[serde(rename = "MeasureName")]
    #[serde(default)]
    pub measure_name: String,
    #[serde(rename = "MeasureValue")]
    #[serde(default)]
    pub measure_value: String,
    #[serde(rename = "MeasureValueType")]
    #[serde(default)]
    pub measure_value_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePipeResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "CurrentState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_state: Option<String>,
    #[serde(rename = "DesiredState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_state: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePipeRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePipeResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "CurrentState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_state: Option<String>,
    #[serde(rename = "DesiredState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_state: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribePipeRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribePipeResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "CurrentState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_state: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "DesiredState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_state: Option<String>,
    #[serde(rename = "Enrichment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enrichment: Option<String>,
    #[serde(rename = "EnrichmentParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enrichment_parameters: Option<PipeEnrichmentParameters>,
    #[serde(rename = "KmsKeyIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_identifier: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "LogConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_configuration: Option<PipeLogConfiguration>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "Source")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "SourceParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_parameters: Option<PipeSourceParameters>,
    #[serde(rename = "StateReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Target")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(rename = "TargetParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_parameters: Option<PipeTargetParameters>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PipeLogConfiguration {
    #[serde(rename = "CloudwatchLogsLogDestination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloudwatch_logs_log_destination: Option<CloudwatchLogsLogDestination>,
    #[serde(rename = "FirehoseLogDestination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firehose_log_destination: Option<FirehoseLogDestination>,
    #[serde(rename = "IncludeExecutionData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_execution_data: Option<Vec<String>>,
    #[serde(rename = "Level")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    #[serde(rename = "S3LogDestination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_log_destination: Option<S3LogDestination>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CloudwatchLogsLogDestination {
    #[serde(rename = "LogGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FirehoseLogDestination {
    #[serde(rename = "DeliveryStreamArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_stream_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3LogDestination {
    #[serde(rename = "BucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<String>,
    #[serde(rename = "BucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_owner: Option<String>,
    #[serde(rename = "OutputFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
    #[serde(rename = "Prefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPipesRequest {
    #[serde(rename = "CurrentState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_state: Option<String>,
    #[serde(rename = "DesiredState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_state: Option<String>,
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "NamePrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_prefix: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SourcePrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_prefix: Option<String>,
    #[serde(rename = "TargetPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPipesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Pipes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipes: Option<Vec<Pipe>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Pipe {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "CurrentState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_state: Option<String>,
    #[serde(rename = "DesiredState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_state: Option<String>,
    #[serde(rename = "Enrichment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enrichment: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Source")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "StateReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<String>,
    #[serde(rename = "Target")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
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
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartPipeRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartPipeResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "CurrentState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_state: Option<String>,
    #[serde(rename = "DesiredState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_state: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopPipeRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopPipeResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "CurrentState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_state: Option<String>,
    #[serde(rename = "DesiredState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_state: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(default)]
    pub tags: std::collections::HashMap<String, String>,
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
pub struct UpdatePipeRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "DesiredState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_state: Option<String>,
    #[serde(rename = "Enrichment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enrichment: Option<String>,
    #[serde(rename = "EnrichmentParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enrichment_parameters: Option<PipeEnrichmentParameters>,
    #[serde(rename = "KmsKeyIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_identifier: Option<String>,
    #[serde(rename = "LogConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_configuration: Option<PipeLogConfigurationParameters>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(rename = "SourceParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_parameters: Option<UpdatePipeSourceParameters>,
    #[serde(rename = "Target")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(rename = "TargetParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_parameters: Option<PipeTargetParameters>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePipeSourceParameters {
    #[serde(rename = "ActiveMQBrokerParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_m_q_broker_parameters: Option<UpdatePipeSourceActiveMQBrokerParameters>,
    #[serde(rename = "DynamoDBStreamParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamo_d_b_stream_parameters: Option<UpdatePipeSourceDynamoDBStreamParameters>,
    #[serde(rename = "FilterCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_criteria: Option<FilterCriteria>,
    #[serde(rename = "KinesisStreamParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_stream_parameters: Option<UpdatePipeSourceKinesisStreamParameters>,
    #[serde(rename = "ManagedStreamingKafkaParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_streaming_kafka_parameters: Option<UpdatePipeSourceManagedStreamingKafkaParameters>,
    #[serde(rename = "RabbitMQBrokerParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rabbit_m_q_broker_parameters: Option<UpdatePipeSourceRabbitMQBrokerParameters>,
    #[serde(rename = "SelfManagedKafkaParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_managed_kafka_parameters: Option<UpdatePipeSourceSelfManagedKafkaParameters>,
    #[serde(rename = "SqsQueueParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sqs_queue_parameters: Option<UpdatePipeSourceSqsQueueParameters>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePipeSourceActiveMQBrokerParameters {
    #[serde(rename = "BatchSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<i32>,
    #[serde(rename = "Credentials")]
    #[serde(default)]
    pub credentials: MQBrokerAccessCredentials,
    #[serde(rename = "MaximumBatchingWindowInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_batching_window_in_seconds: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePipeSourceDynamoDBStreamParameters {
    #[serde(rename = "BatchSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<i32>,
    #[serde(rename = "DeadLetterConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dead_letter_config: Option<DeadLetterConfig>,
    #[serde(rename = "MaximumBatchingWindowInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_batching_window_in_seconds: Option<i32>,
    #[serde(rename = "MaximumRecordAgeInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_record_age_in_seconds: Option<i32>,
    #[serde(rename = "MaximumRetryAttempts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_retry_attempts: Option<i32>,
    #[serde(rename = "OnPartialBatchItemFailure")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_partial_batch_item_failure: Option<String>,
    #[serde(rename = "ParallelizationFactor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallelization_factor: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePipeSourceKinesisStreamParameters {
    #[serde(rename = "BatchSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<i32>,
    #[serde(rename = "DeadLetterConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dead_letter_config: Option<DeadLetterConfig>,
    #[serde(rename = "MaximumBatchingWindowInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_batching_window_in_seconds: Option<i32>,
    #[serde(rename = "MaximumRecordAgeInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_record_age_in_seconds: Option<i32>,
    #[serde(rename = "MaximumRetryAttempts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_retry_attempts: Option<i32>,
    #[serde(rename = "OnPartialBatchItemFailure")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_partial_batch_item_failure: Option<String>,
    #[serde(rename = "ParallelizationFactor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallelization_factor: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePipeSourceManagedStreamingKafkaParameters {
    #[serde(rename = "BatchSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<i32>,
    #[serde(rename = "Credentials")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<MSKAccessCredentials>,
    #[serde(rename = "MaximumBatchingWindowInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_batching_window_in_seconds: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePipeSourceRabbitMQBrokerParameters {
    #[serde(rename = "BatchSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<i32>,
    #[serde(rename = "Credentials")]
    #[serde(default)]
    pub credentials: MQBrokerAccessCredentials,
    #[serde(rename = "MaximumBatchingWindowInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_batching_window_in_seconds: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePipeSourceSelfManagedKafkaParameters {
    #[serde(rename = "BatchSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<i32>,
    #[serde(rename = "Credentials")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<SelfManagedKafkaAccessConfigurationCredentials>,
    #[serde(rename = "MaximumBatchingWindowInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_batching_window_in_seconds: Option<i32>,
    #[serde(rename = "ServerRootCaCertificate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_root_ca_certificate: Option<String>,
    #[serde(rename = "Vpc")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc: Option<SelfManagedKafkaAccessConfigurationVpc>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePipeSourceSqsQueueParameters {
    #[serde(rename = "BatchSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<i32>,
    #[serde(rename = "MaximumBatchingWindowInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_batching_window_in_seconds: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePipeResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "CurrentState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_state: Option<String>,
    #[serde(rename = "DesiredState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_state: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
