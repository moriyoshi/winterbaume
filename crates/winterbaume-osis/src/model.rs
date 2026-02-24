//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-osis

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopPipelineResponse {
    #[serde(rename = "Pipeline")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline: Option<Pipeline>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Pipeline {
    #[serde(rename = "BufferOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffer_options: Option<BufferOptions>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "Destinations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<PipelineDestination>>,
    #[serde(rename = "EncryptionAtRestOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_at_rest_options: Option<EncryptionAtRestOptions>,
    #[serde(rename = "IngestEndpointUrls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingest_endpoint_urls: Option<Vec<String>>,
    #[serde(rename = "LastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    #[serde(rename = "LogPublishingOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_publishing_options: Option<LogPublishingOptions>,
    #[serde(rename = "MaxUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_units: Option<i32>,
    #[serde(rename = "MinUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_units: Option<i32>,
    #[serde(rename = "PipelineArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_arn: Option<String>,
    #[serde(rename = "PipelineConfigurationBody")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_configuration_body: Option<String>,
    #[serde(rename = "PipelineName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_name: Option<String>,
    #[serde(rename = "PipelineRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_role_arn: Option<String>,
    #[serde(rename = "ServiceVpcEndpoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_vpc_endpoints: Option<Vec<ServiceVpcEndpoint>>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<PipelineStatusReason>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "VpcEndpointService")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_service: Option<String>,
    #[serde(rename = "VpcEndpoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoints: Option<Vec<VpcEndpoint>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BufferOptions {
    #[serde(rename = "PersistentBufferEnabled")]
    #[serde(default)]
    pub persistent_buffer_enabled: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PipelineDestination {
    #[serde(rename = "Endpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(rename = "ServiceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EncryptionAtRestOptions {
    #[serde(rename = "KmsKeyArn")]
    #[serde(default)]
    pub kms_key_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LogPublishingOptions {
    #[serde(rename = "CloudWatchLogDestination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_log_destination: Option<CloudWatchLogDestination>,
    #[serde(rename = "IsLoggingEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_logging_enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CloudWatchLogDestination {
    #[serde(rename = "LogGroup")]
    #[serde(default)]
    pub log_group: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceVpcEndpoint {
    #[serde(rename = "ServiceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    #[serde(rename = "VpcEndpointId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PipelineStatusReason {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
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
pub struct VpcEndpoint {
    #[serde(rename = "VpcEndpointId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_id: Option<String>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    #[serde(rename = "VpcOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_options: Option<VpcOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VpcOptions {
    #[serde(rename = "SecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(rename = "SubnetIds")]
    #[serde(default)]
    pub subnet_ids: Vec<String>,
    #[serde(rename = "VpcAttachmentOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_attachment_options: Option<VpcAttachmentOptions>,
    #[serde(rename = "VpcEndpointManagement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_management: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VpcAttachmentOptions {
    #[serde(rename = "AttachToVpc")]
    #[serde(default)]
    pub attach_to_vpc: bool,
    #[serde(rename = "CidrBlock")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_block: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPipelineChangeProgressResponse {
    #[serde(rename = "ChangeProgressStatuses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_progress_statuses: Option<Vec<ChangeProgressStatus>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChangeProgressStatus {
    #[serde(rename = "ChangeProgressStages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_progress_stages: Option<Vec<ChangeProgressStage>>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TotalNumberOfStages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_number_of_stages: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChangeProgressStage {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "LastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RevokePipelineEndpointConnectionsResponse {
    #[serde(rename = "PipelineArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteResourcePolicyResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartPipelineResponse {
    #[serde(rename = "Pipeline")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline: Option<Pipeline>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePipelineResponse {
    #[serde(rename = "Pipeline")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline: Option<Pipeline>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPipelinesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Pipelines")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipelines: Option<Vec<PipelineSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PipelineSummary {
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "Destinations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<PipelineDestination>>,
    #[serde(rename = "LastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    #[serde(rename = "MaxUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_units: Option<i32>,
    #[serde(rename = "MinUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_units: Option<i32>,
    #[serde(rename = "PipelineArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_arn: Option<String>,
    #[serde(rename = "PipelineName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<PipelineStatusReason>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPipelineResponse {
    #[serde(rename = "Pipeline")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline: Option<Pipeline>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPipelineBlueprintRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteResourcePolicyRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPipelineBlueprintResponse {
    #[serde(rename = "Blueprint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blueprint: Option<PipelineBlueprint>,
    #[serde(rename = "Format")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PipelineBlueprint {
    #[serde(rename = "BlueprintName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blueprint_name: Option<String>,
    #[serde(rename = "DisplayDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_description: Option<String>,
    #[serde(rename = "DisplayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "PipelineConfigurationBody")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_configuration_body: Option<String>,
    #[serde(rename = "Service")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    #[serde(rename = "UseCase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_case: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePipelineResponse {
    #[serde(rename = "Pipeline")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline: Option<Pipeline>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePipelineEndpointResponse {
    #[serde(rename = "EndpointId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_id: Option<String>,
    #[serde(rename = "PipelineArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_arn: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPipelinesRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPipelineEndpointsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PipelineEndpoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_endpoints: Option<Vec<PipelineEndpoint>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PipelineEndpoint {
    #[serde(rename = "EndpointId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_id: Option<String>,
    #[serde(rename = "IngestEndpointUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingest_endpoint_url: Option<String>,
    #[serde(rename = "PipelineArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_arn: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    #[serde(rename = "VpcOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_options: Option<PipelineEndpointVpcOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PipelineEndpointVpcOptions {
    #[serde(rename = "SecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(rename = "SubnetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartPipelineRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePipelineEndpointRequest {
    #[serde(rename = "PipelineArn")]
    #[serde(default)]
    pub pipeline_arn: String,
    #[serde(rename = "VpcOptions")]
    #[serde(default)]
    pub vpc_options: PipelineEndpointVpcOptions,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePipelineEndpointResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePipelineRequest {
    #[serde(rename = "BufferOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffer_options: Option<BufferOptions>,
    #[serde(rename = "EncryptionAtRestOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_at_rest_options: Option<EncryptionAtRestOptions>,
    #[serde(rename = "LogPublishingOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_publishing_options: Option<LogPublishingOptions>,
    #[serde(rename = "MaxUnits")]
    #[serde(default)]
    pub max_units: i32,
    #[serde(rename = "MinUnits")]
    #[serde(default)]
    pub min_units: i32,
    #[serde(rename = "PipelineConfigurationBody")]
    #[serde(default)]
    pub pipeline_configuration_body: String,
    #[serde(rename = "PipelineName")]
    #[serde(default)]
    pub pipeline_name: String,
    #[serde(rename = "PipelineRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_role_arn: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "VpcOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_options: Option<VpcOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutResourcePolicyResponse {
    #[serde(rename = "Policy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopPipelineRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePipelineRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourcePolicyResponse {
    #[serde(rename = "Policy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourcePolicyRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPipelineBlueprintsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPipelineEndpointsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePipelineRequest {
    #[serde(rename = "BufferOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffer_options: Option<BufferOptions>,
    #[serde(rename = "EncryptionAtRestOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_at_rest_options: Option<EncryptionAtRestOptions>,
    #[serde(rename = "LogPublishingOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_publishing_options: Option<LogPublishingOptions>,
    #[serde(rename = "MaxUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_units: Option<i32>,
    #[serde(rename = "MinUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_units: Option<i32>,
    #[serde(rename = "PipelineConfigurationBody")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_configuration_body: Option<String>,
    #[serde(rename = "PipelineRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePipelineResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutResourcePolicyRequest {
    #[serde(rename = "Policy")]
    #[serde(default)]
    pub policy: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPipelineRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPipelineBlueprintsResponse {
    #[serde(rename = "Blueprints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blueprints: Option<Vec<PipelineBlueprintSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PipelineBlueprintSummary {
    #[serde(rename = "BlueprintName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blueprint_name: Option<String>,
    #[serde(rename = "DisplayDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_description: Option<String>,
    #[serde(rename = "DisplayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "Service")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    #[serde(rename = "UseCase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_case: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPipelineEndpointConnectionsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PipelineEndpointConnections")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_endpoint_connections: Option<Vec<PipelineEndpointConnection>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PipelineEndpointConnection {
    #[serde(rename = "EndpointId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_id: Option<String>,
    #[serde(rename = "PipelineArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_arn: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "VpcEndpointOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_owner: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPipelineChangeProgressRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ValidatePipelineResponse {
    #[serde(rename = "Errors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<ValidationMessage>>,
    #[serde(rename = "isValid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_valid: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ValidationMessage {
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPipelineEndpointConnectionsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePipelineEndpointRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RevokePipelineEndpointConnectionsRequest {
    #[serde(rename = "EndpointIds")]
    #[serde(default)]
    pub endpoint_ids: Vec<String>,
    #[serde(rename = "PipelineArn")]
    #[serde(default)]
    pub pipeline_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ValidatePipelineRequest {
    #[serde(rename = "PipelineConfigurationBody")]
    #[serde(default)]
    pub pipeline_configuration_body: String,
}
