//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-connect

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActivateEvaluationFormRequest {
    #[serde(rename = "EvaluationFormId")]
    #[serde(default)]
    pub evaluation_form_id: String,
    #[serde(rename = "EvaluationFormVersion")]
    #[serde(default)]
    pub evaluation_form_version: i32,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActivateEvaluationFormResponse {
    #[serde(rename = "EvaluationFormArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_form_arn: Option<String>,
    #[serde(rename = "EvaluationFormId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_form_id: Option<String>,
    #[serde(rename = "EvaluationFormVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_form_version: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateAnalyticsDataSetRequest {
    #[serde(rename = "DataSetId")]
    #[serde(default)]
    pub data_set_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "TargetAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_account_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateAnalyticsDataSetResponse {
    #[serde(rename = "DataSetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_id: Option<String>,
    #[serde(rename = "ResourceShareArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_share_arn: Option<String>,
    #[serde(rename = "ResourceShareId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_share_id: Option<String>,
    #[serde(rename = "TargetAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_account_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateApprovedOriginRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Origin")]
    #[serde(default)]
    pub origin: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateBotRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "LexBot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lex_bot: Option<LexBot>,
    #[serde(rename = "LexV2Bot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lex_v2_bot: Option<LexV2Bot>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LexBot {
    #[serde(rename = "LexRegion")]
    #[serde(default)]
    pub lex_region: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LexV2Bot {
    #[serde(rename = "AliasArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateContactWithUserRequest {
    #[serde(rename = "ContactId")]
    #[serde(default)]
    pub contact_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "UserId")]
    #[serde(default)]
    pub user_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateContactWithUserResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateDefaultVocabularyRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    pub language_code: String,
    #[serde(rename = "VocabularyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateDefaultVocabularyResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateEmailAddressAliasRequest {
    #[serde(rename = "AliasConfiguration")]
    #[serde(default)]
    pub alias_configuration: AliasConfiguration,
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "EmailAddressId")]
    #[serde(default)]
    pub email_address_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AliasConfiguration {
    #[serde(rename = "EmailAddressId")]
    #[serde(default)]
    pub email_address_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateEmailAddressAliasResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateFlowRequest {
    #[serde(rename = "FlowId")]
    #[serde(default)]
    pub flow_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    pub resource_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateFlowResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateHoursOfOperationsRequest {
    #[serde(rename = "HoursOfOperationId")]
    #[serde(default)]
    pub hours_of_operation_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "ParentHoursOfOperationConfigs")]
    #[serde(default)]
    pub parent_hours_of_operation_configs: Vec<ParentHoursOfOperationConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ParentHoursOfOperationConfig {
    #[serde(rename = "HoursOfOperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hours_of_operation_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateInstanceStorageConfigRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    pub resource_type: String,
    #[serde(rename = "StorageConfig")]
    #[serde(default)]
    pub storage_config: InstanceStorageConfig,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceStorageConfig {
    #[serde(rename = "AssociationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_id: Option<String>,
    #[serde(rename = "KinesisFirehoseConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_firehose_config: Option<KinesisFirehoseConfig>,
    #[serde(rename = "KinesisStreamConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_stream_config: Option<KinesisStreamConfig>,
    #[serde(rename = "KinesisVideoStreamConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_video_stream_config: Option<KinesisVideoStreamConfig>,
    #[serde(rename = "S3Config")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_config: Option<S3Config>,
    #[serde(rename = "StorageType")]
    #[serde(default)]
    pub storage_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KinesisFirehoseConfig {
    #[serde(rename = "FirehoseArn")]
    #[serde(default)]
    pub firehose_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KinesisStreamConfig {
    #[serde(rename = "StreamArn")]
    #[serde(default)]
    pub stream_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KinesisVideoStreamConfig {
    #[serde(rename = "EncryptionConfig")]
    #[serde(default)]
    pub encryption_config: EncryptionConfig,
    #[serde(rename = "Prefix")]
    #[serde(default)]
    pub prefix: String,
    #[serde(rename = "RetentionPeriodHours")]
    #[serde(default)]
    pub retention_period_hours: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EncryptionConfig {
    #[serde(rename = "EncryptionType")]
    #[serde(default)]
    pub encryption_type: String,
    #[serde(rename = "KeyId")]
    #[serde(default)]
    pub key_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3Config {
    #[serde(rename = "BucketName")]
    #[serde(default)]
    pub bucket_name: String,
    #[serde(rename = "BucketPrefix")]
    #[serde(default)]
    pub bucket_prefix: String,
    #[serde(rename = "EncryptionConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_config: Option<EncryptionConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateInstanceStorageConfigResponse {
    #[serde(rename = "AssociationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateLambdaFunctionRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "FunctionArn")]
    #[serde(default)]
    pub function_arn: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateLexBotRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "LexBot")]
    #[serde(default)]
    pub lex_bot: LexBot,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociatePhoneNumberContactFlowRequest {
    #[serde(rename = "ContactFlowId")]
    #[serde(default)]
    pub contact_flow_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "PhoneNumberId")]
    #[serde(default)]
    pub phone_number_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateQueueEmailAddressesRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "EmailAddressesConfig")]
    #[serde(default)]
    pub email_addresses_config: Vec<EmailAddressConfig>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "QueueId")]
    #[serde(default)]
    pub queue_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EmailAddressConfig {
    #[serde(rename = "EmailAddressId")]
    #[serde(default)]
    pub email_address_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateQueueQuickConnectsRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "QueueId")]
    #[serde(default)]
    pub queue_id: String,
    #[serde(rename = "QuickConnectIds")]
    #[serde(default)]
    pub quick_connect_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateRoutingProfileQueuesRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "ManualAssignmentQueueConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_assignment_queue_configs: Option<Vec<RoutingProfileManualAssignmentQueueConfig>>,
    #[serde(rename = "QueueConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_configs: Option<Vec<RoutingProfileQueueConfig>>,
    #[serde(rename = "RoutingProfileId")]
    #[serde(default)]
    pub routing_profile_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RoutingProfileManualAssignmentQueueConfig {
    #[serde(rename = "QueueReference")]
    #[serde(default)]
    pub queue_reference: RoutingProfileQueueReference,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RoutingProfileQueueReference {
    #[serde(rename = "Channel")]
    #[serde(default)]
    pub channel: String,
    #[serde(rename = "QueueId")]
    #[serde(default)]
    pub queue_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RoutingProfileQueueConfig {
    #[serde(rename = "Delay")]
    #[serde(default)]
    pub delay: i32,
    #[serde(rename = "Priority")]
    #[serde(default)]
    pub priority: i32,
    #[serde(rename = "QueueReference")]
    #[serde(default)]
    pub queue_reference: RoutingProfileQueueReference,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateSecurityKeyRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateSecurityKeyResponse {
    #[serde(rename = "AssociationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateSecurityProfilesRequest {
    #[serde(rename = "EntityArn")]
    #[serde(default)]
    pub entity_arn: String,
    #[serde(rename = "EntityType")]
    #[serde(default)]
    pub entity_type: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "SecurityProfiles")]
    #[serde(default)]
    pub security_profiles: Vec<SecurityProfileItem>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SecurityProfileItem {
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateTrafficDistributionGroupUserRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "TrafficDistributionGroupId")]
    #[serde(default)]
    pub traffic_distribution_group_id: String,
    #[serde(rename = "UserId")]
    #[serde(default)]
    pub user_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateTrafficDistributionGroupUserResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateUserProficienciesRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "UserId")]
    #[serde(default)]
    pub user_id: String,
    #[serde(rename = "UserProficiencies")]
    #[serde(default)]
    pub user_proficiencies: Vec<UserProficiency>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserProficiency {
    #[serde(rename = "AttributeName")]
    #[serde(default)]
    pub attribute_name: String,
    #[serde(rename = "AttributeValue")]
    #[serde(default)]
    pub attribute_value: String,
    #[serde(rename = "Level")]
    #[serde(default)]
    pub level: f32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateWorkspaceRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "ResourceArns")]
    #[serde(default)]
    pub resource_arns: Vec<String>,
    #[serde(rename = "WorkspaceId")]
    #[serde(default)]
    pub workspace_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateWorkspaceResponse {
    #[serde(rename = "FailedList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_list: Option<Vec<FailedBatchAssociationSummary>>,
    #[serde(rename = "SuccessfulList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful_list: Option<Vec<SuccessfulBatchAssociationSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FailedBatchAssociationSummary {
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SuccessfulBatchAssociationSummary {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchAssociateAnalyticsDataSetRequest {
    #[serde(rename = "DataSetIds")]
    #[serde(default)]
    pub data_set_ids: Vec<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "TargetAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_account_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchAssociateAnalyticsDataSetResponse {
    #[serde(rename = "Created")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<Vec<AnalyticsDataAssociationResult>>,
    #[serde(rename = "Errors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<ErrorResult>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnalyticsDataAssociationResult {
    #[serde(rename = "DataSetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_id: Option<String>,
    #[serde(rename = "ResourceShareArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_share_arn: Option<String>,
    #[serde(rename = "ResourceShareId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_share_id: Option<String>,
    #[serde(rename = "ResourceShareStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_share_status: Option<String>,
    #[serde(rename = "TargetAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_account_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ErrorResult {
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchCreateDataTableValueRequest {
    #[serde(rename = "DataTableId")]
    #[serde(default)]
    pub data_table_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Values")]
    #[serde(default)]
    pub values: Vec<DataTableValue>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataTableValue {
    #[serde(rename = "AttributeName")]
    #[serde(default)]
    pub attribute_name: String,
    #[serde(rename = "LastModifiedRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_region: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "LockVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_version: Option<DataTableLockVersion>,
    #[serde(rename = "PrimaryValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_values: Option<Vec<PrimaryValue>>,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataTableLockVersion {
    #[serde(rename = "Attribute")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute: Option<String>,
    #[serde(rename = "DataTable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_table: Option<String>,
    #[serde(rename = "PrimaryValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_values: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PrimaryValue {
    #[serde(rename = "AttributeName")]
    #[serde(default)]
    pub attribute_name: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchCreateDataTableValueResponse {
    #[serde(rename = "Failed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed: Option<Vec<BatchCreateDataTableValueFailureResult>>,
    #[serde(rename = "Successful")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful: Option<Vec<BatchCreateDataTableValueSuccessResult>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchCreateDataTableValueFailureResult {
    #[serde(rename = "AttributeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_name: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "PrimaryValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_values: Option<Vec<PrimaryValue>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchCreateDataTableValueSuccessResult {
    #[serde(rename = "AttributeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_name: Option<String>,
    #[serde(rename = "LockVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_version: Option<DataTableLockVersion>,
    #[serde(rename = "PrimaryValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_values: Option<Vec<PrimaryValue>>,
    #[serde(rename = "RecordId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDeleteDataTableValueRequest {
    #[serde(rename = "DataTableId")]
    #[serde(default)]
    pub data_table_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Values")]
    #[serde(default)]
    pub values: Vec<DataTableDeleteValueIdentifier>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataTableDeleteValueIdentifier {
    #[serde(rename = "AttributeName")]
    #[serde(default)]
    pub attribute_name: String,
    #[serde(rename = "LockVersion")]
    #[serde(default)]
    pub lock_version: DataTableLockVersion,
    #[serde(rename = "PrimaryValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_values: Option<Vec<PrimaryValue>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDeleteDataTableValueResponse {
    #[serde(rename = "Failed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed: Option<Vec<BatchDeleteDataTableValueFailureResult>>,
    #[serde(rename = "Successful")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful: Option<Vec<BatchDeleteDataTableValueSuccessResult>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDeleteDataTableValueFailureResult {
    #[serde(rename = "AttributeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_name: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "PrimaryValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_values: Option<Vec<PrimaryValue>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDeleteDataTableValueSuccessResult {
    #[serde(rename = "AttributeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_name: Option<String>,
    #[serde(rename = "LockVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_version: Option<DataTableLockVersion>,
    #[serde(rename = "PrimaryValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_values: Option<Vec<PrimaryValue>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDescribeDataTableValueRequest {
    #[serde(rename = "DataTableId")]
    #[serde(default)]
    pub data_table_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Values")]
    #[serde(default)]
    pub values: Vec<DataTableValueIdentifier>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataTableValueIdentifier {
    #[serde(rename = "AttributeName")]
    #[serde(default)]
    pub attribute_name: String,
    #[serde(rename = "PrimaryValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_values: Option<Vec<PrimaryValue>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDescribeDataTableValueResponse {
    #[serde(rename = "Failed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed: Option<Vec<BatchDescribeDataTableValueFailureResult>>,
    #[serde(rename = "Successful")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful: Option<Vec<BatchDescribeDataTableValueSuccessResult>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDescribeDataTableValueFailureResult {
    #[serde(rename = "AttributeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_name: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "PrimaryValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_values: Option<Vec<PrimaryValue>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDescribeDataTableValueSuccessResult {
    #[serde(rename = "AttributeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_id: Option<String>,
    #[serde(rename = "AttributeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_name: Option<String>,
    #[serde(rename = "LastModifiedRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_region: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "LockVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_version: Option<DataTableLockVersion>,
    #[serde(rename = "PrimaryValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_values: Option<Vec<PrimaryValueResponse>>,
    #[serde(rename = "RecordId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_id: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PrimaryValueResponse {
    #[serde(rename = "AttributeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_id: Option<String>,
    #[serde(rename = "AttributeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_name: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDisassociateAnalyticsDataSetRequest {
    #[serde(rename = "DataSetIds")]
    #[serde(default)]
    pub data_set_ids: Vec<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "TargetAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_account_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDisassociateAnalyticsDataSetResponse {
    #[serde(rename = "Deleted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<Vec<String>>,
    #[serde(rename = "Errors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<ErrorResult>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetAttachedFileMetadataRequest {
    #[serde(rename = "AssociatedResourceArn")]
    #[serde(default)]
    pub associated_resource_arn: String,
    #[serde(rename = "FileIds")]
    #[serde(default)]
    pub file_ids: Vec<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetAttachedFileMetadataResponse {
    #[serde(rename = "Errors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<AttachedFileError>>,
    #[serde(rename = "Files")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<AttachedFile>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttachedFileError {
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "FileId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttachedFile {
    #[serde(rename = "AssociatedResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_resource_arn: Option<String>,
    #[serde(rename = "CreatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<CreatedByInfo>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(rename = "FileArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_arn: Option<String>,
    #[serde(rename = "FileId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    #[serde(rename = "FileName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(rename = "FileSizeInBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size_in_bytes: Option<i64>,
    #[serde(rename = "FileStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_status: Option<String>,
    #[serde(rename = "FileUseCaseType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_use_case_type: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatedByInfo {
    #[serde(rename = "AWSIdentityArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_w_s_identity_arn: Option<String>,
    #[serde(rename = "ConnectUserArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_user_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetFlowAssociationRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "ResourceIds")]
    #[serde(default)]
    pub resource_ids: Vec<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetFlowAssociationResponse {
    #[serde(rename = "FlowAssociationSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_association_summary_list: Option<Vec<FlowAssociationSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FlowAssociationSummary {
    #[serde(rename = "FlowId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_id: Option<String>,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchPutContactRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "ContactDataRequestList")]
    #[serde(default)]
    pub contact_data_request_list: Vec<ContactDataRequest>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContactDataRequest {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Campaign")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign: Option<Campaign>,
    #[serde(rename = "CustomerEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_endpoint: Option<Endpoint>,
    #[serde(rename = "OutboundStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_strategy: Option<OutboundStrategy>,
    #[serde(rename = "QueueId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_id: Option<String>,
    #[serde(rename = "RequestIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_identifier: Option<String>,
    #[serde(rename = "SystemEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_endpoint: Option<Endpoint>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Campaign {
    #[serde(rename = "CampaignId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Endpoint {
    #[serde(rename = "Address")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OutboundStrategy {
    #[serde(rename = "Config")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<OutboundStrategyConfig>,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OutboundStrategyConfig {
    #[serde(rename = "AgentFirst")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_first: Option<AgentFirst>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AgentFirst {
    #[serde(rename = "Preview")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview: Option<Preview>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Preview {
    #[serde(rename = "AllowedUserActions")]
    #[serde(default)]
    pub allowed_user_actions: Vec<String>,
    #[serde(rename = "PostAcceptTimeoutConfig")]
    #[serde(default)]
    pub post_accept_timeout_config: PostAcceptTimeoutConfig,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PostAcceptTimeoutConfig {
    #[serde(rename = "DurationInSeconds")]
    #[serde(default)]
    pub duration_in_seconds: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchPutContactResponse {
    #[serde(rename = "FailedRequestList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_request_list: Option<Vec<FailedRequest>>,
    #[serde(rename = "SuccessfulRequestList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful_request_list: Option<Vec<SuccessfulRequest>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FailedRequest {
    #[serde(rename = "FailureReasonCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason_code: Option<String>,
    #[serde(rename = "FailureReasonMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason_message: Option<String>,
    #[serde(rename = "RequestIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SuccessfulRequest {
    #[serde(rename = "ContactId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<String>,
    #[serde(rename = "RequestIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchUpdateDataTableValueRequest {
    #[serde(rename = "DataTableId")]
    #[serde(default)]
    pub data_table_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Values")]
    #[serde(default)]
    pub values: Vec<DataTableValue>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchUpdateDataTableValueResponse {
    #[serde(rename = "Failed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed: Option<Vec<BatchUpdateDataTableValueFailureResult>>,
    #[serde(rename = "Successful")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful: Option<Vec<BatchUpdateDataTableValueSuccessResult>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchUpdateDataTableValueFailureResult {
    #[serde(rename = "AttributeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_name: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "PrimaryValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_values: Option<Vec<PrimaryValue>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchUpdateDataTableValueSuccessResult {
    #[serde(rename = "AttributeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_name: Option<String>,
    #[serde(rename = "LockVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_version: Option<DataTableLockVersion>,
    #[serde(rename = "PrimaryValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_values: Option<Vec<PrimaryValue>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClaimPhoneNumberRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(rename = "PhoneNumber")]
    #[serde(default)]
    pub phone_number: String,
    #[serde(rename = "PhoneNumberDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_description: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "TargetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClaimPhoneNumberResponse {
    #[serde(rename = "PhoneNumberArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_arn: Option<String>,
    #[serde(rename = "PhoneNumberId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CompleteAttachedFileUploadRequest {
    #[serde(rename = "AssociatedResourceArn")]
    #[serde(default)]
    pub associated_resource_arn: String,
    #[serde(rename = "FileId")]
    #[serde(default)]
    pub file_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CompleteAttachedFileUploadResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAgentStatusRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "DisplayOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_order: Option<i32>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "State")]
    #[serde(default)]
    pub state: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAgentStatusResponse {
    #[serde(rename = "AgentStatusARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_status_a_r_n: Option<String>,
    #[serde(rename = "AgentStatusId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_status_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateContactFlowModuleAliasRequest {
    #[serde(rename = "AliasName")]
    #[serde(default)]
    pub alias_name: String,
    #[serde(rename = "ContactFlowModuleId")]
    #[serde(default)]
    pub contact_flow_module_id: String,
    #[serde(rename = "ContactFlowModuleVersion")]
    #[serde(default)]
    pub contact_flow_module_version: i64,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateContactFlowModuleAliasResponse {
    #[serde(rename = "ContactFlowModuleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_flow_module_arn: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateContactFlowModuleRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "Content")]
    #[serde(default)]
    pub content: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ExternalInvocationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_invocation_configuration: Option<ExternalInvocationConfiguration>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Settings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExternalInvocationConfiguration {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateContactFlowModuleResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateContactFlowModuleVersionRequest {
    #[serde(rename = "ContactFlowModuleId")]
    #[serde(default)]
    pub contact_flow_module_id: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "FlowModuleContentSha256")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_module_content_sha256: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateContactFlowModuleVersionResponse {
    #[serde(rename = "ContactFlowModuleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_flow_module_arn: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateContactFlowRequest {
    #[serde(rename = "Content")]
    #[serde(default)]
    pub content: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateContactFlowResponse {
    #[serde(rename = "ContactFlowArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_flow_arn: Option<String>,
    #[serde(rename = "ContactFlowId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_flow_id: Option<String>,
    #[serde(rename = "FlowContentSha256")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_content_sha256: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateContactFlowVersionRequest {
    #[serde(rename = "ContactFlowId")]
    #[serde(default)]
    pub contact_flow_id: String,
    #[serde(rename = "ContactFlowVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_flow_version: Option<i64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "FlowContentSha256")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_content_sha256: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "LastModifiedRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_region: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateContactFlowVersionResponse {
    #[serde(rename = "ContactFlowArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_flow_arn: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateContactRequest {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Channel")]
    #[serde(default)]
    pub channel: String,
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ExpiryDurationInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_duration_in_minutes: Option<i32>,
    #[serde(rename = "InitiateAs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiate_as: Option<String>,
    #[serde(rename = "InitiationMethod")]
    #[serde(default)]
    pub initiation_method: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "PreviousContactId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_contact_id: Option<String>,
    #[serde(rename = "References")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub references: Option<std::collections::HashMap<String, Reference>>,
    #[serde(rename = "RelatedContactId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_contact_id: Option<String>,
    #[serde(rename = "SegmentAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_attributes: Option<std::collections::HashMap<String, SegmentAttributeValue>>,
    #[serde(rename = "UserInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_info: Option<UserInfo>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Reference {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SegmentAttributeValue {
    #[serde(rename = "ValueArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_arn: Option<String>,
    #[serde(rename = "ValueInteger")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_integer: Option<i32>,
    #[serde(rename = "ValueList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_list: Option<Vec<SegmentAttributeValue>>,
    #[serde(rename = "ValueMap")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_map: Option<std::collections::HashMap<String, SegmentAttributeValue>>,
    #[serde(rename = "ValueString")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_string: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserInfo {
    #[serde(rename = "UserId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateContactResponse {
    #[serde(rename = "ContactArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_arn: Option<String>,
    #[serde(rename = "ContactId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDataTableAttributeRequest {
    #[serde(rename = "DataTableId")]
    #[serde(default)]
    pub data_table_id: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Primary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
    #[serde(rename = "Validation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation: Option<Validation>,
    #[serde(rename = "ValueType")]
    #[serde(default)]
    pub value_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Validation {
    #[serde(rename = "Enum")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#enum: Option<ValidationEnum>,
    #[serde(rename = "ExclusiveMaximum")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_maximum: Option<f64>,
    #[serde(rename = "ExclusiveMinimum")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_minimum: Option<f64>,
    #[serde(rename = "IgnoreCase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_case: Option<bool>,
    #[serde(rename = "MaxLength")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<i32>,
    #[serde(rename = "MaxValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_values: Option<i32>,
    #[serde(rename = "Maximum")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f64>,
    #[serde(rename = "MinLength")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<i32>,
    #[serde(rename = "MinValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_values: Option<i32>,
    #[serde(rename = "Minimum")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<f64>,
    #[serde(rename = "MultipleOf")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_of: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ValidationEnum {
    #[serde(rename = "Strict")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDataTableAttributeResponse {
    #[serde(rename = "AttributeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_id: Option<String>,
    #[serde(rename = "LockVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_version: Option<DataTableLockVersion>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDataTableRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Status")]
    #[serde(default)]
    pub status: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "TimeZone")]
    #[serde(default)]
    pub time_zone: String,
    #[serde(rename = "ValueLockLevel")]
    #[serde(default)]
    pub value_lock_level: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDataTableResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LockVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_version: Option<DataTableLockVersion>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateEmailAddressRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "DisplayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "EmailAddress")]
    #[serde(default)]
    pub email_address: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateEmailAddressResponse {
    #[serde(rename = "EmailAddressArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address_arn: Option<String>,
    #[serde(rename = "EmailAddressId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateEvaluationFormRequest {
    #[serde(rename = "AsDraft")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as_draft: Option<bool>,
    #[serde(rename = "AutoEvaluationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_evaluation_configuration: Option<EvaluationFormAutoEvaluationConfiguration>,
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Items")]
    #[serde(default)]
    pub items: Vec<EvaluationFormItem>,
    #[serde(rename = "LanguageConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_configuration: Option<EvaluationFormLanguageConfiguration>,
    #[serde(rename = "ReviewConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_configuration: Option<EvaluationReviewConfiguration>,
    #[serde(rename = "ScoringStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scoring_strategy: Option<EvaluationFormScoringStrategy>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "TargetConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_configuration: Option<EvaluationFormTargetConfiguration>,
    #[serde(rename = "Title")]
    #[serde(default)]
    pub title: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationFormAutoEvaluationConfiguration {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    pub enabled: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationFormItem {
    #[serde(rename = "Question")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub question: Option<EvaluationFormQuestion>,
    #[serde(rename = "Section")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section: Option<Box<EvaluationFormSection>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationFormQuestion {
    #[serde(rename = "Enablement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enablement: Option<EvaluationFormItemEnablementConfiguration>,
    #[serde(rename = "Instructions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    #[serde(rename = "NotApplicableEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_applicable_enabled: Option<bool>,
    #[serde(rename = "QuestionType")]
    #[serde(default)]
    pub question_type: String,
    #[serde(rename = "QuestionTypeProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub question_type_properties: Option<EvaluationFormQuestionTypeProperties>,
    #[serde(rename = "RefId")]
    #[serde(default)]
    pub ref_id: String,
    #[serde(rename = "Title")]
    #[serde(default)]
    pub title: String,
    #[serde(rename = "Weight")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationFormItemEnablementConfiguration {
    #[serde(rename = "Action")]
    #[serde(default)]
    pub action: String,
    #[serde(rename = "Condition")]
    #[serde(default)]
    pub condition: EvaluationFormItemEnablementCondition,
    #[serde(rename = "DefaultAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_action: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationFormItemEnablementCondition {
    #[serde(rename = "Operands")]
    #[serde(default)]
    pub operands: Vec<EvaluationFormItemEnablementConditionOperand>,
    #[serde(rename = "Operator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationFormItemEnablementConditionOperand {
    #[serde(rename = "Condition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<Box<EvaluationFormItemEnablementCondition>>,
    #[serde(rename = "Expression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<EvaluationFormItemEnablementExpression>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationFormItemEnablementExpression {
    #[serde(rename = "Comparator")]
    #[serde(default)]
    pub comparator: String,
    #[serde(rename = "Source")]
    #[serde(default)]
    pub source: EvaluationFormItemEnablementSource,
    #[serde(rename = "Values")]
    #[serde(default)]
    pub values: Vec<EvaluationFormItemEnablementSourceValue>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationFormItemEnablementSource {
    #[serde(rename = "RefId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_id: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationFormItemEnablementSourceValue {
    #[serde(rename = "RefId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_id: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationFormQuestionTypeProperties {
    #[serde(rename = "MultiSelect")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_select: Option<EvaluationFormMultiSelectQuestionProperties>,
    #[serde(rename = "Numeric")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numeric: Option<EvaluationFormNumericQuestionProperties>,
    #[serde(rename = "SingleSelect")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_select: Option<EvaluationFormSingleSelectQuestionProperties>,
    #[serde(rename = "Text")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<EvaluationFormTextQuestionProperties>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationFormMultiSelectQuestionProperties {
    #[serde(rename = "Automation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automation: Option<EvaluationFormMultiSelectQuestionAutomation>,
    #[serde(rename = "DisplayAs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_as: Option<String>,
    #[serde(rename = "Options")]
    #[serde(default)]
    pub options: Vec<EvaluationFormMultiSelectQuestionOption>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationFormMultiSelectQuestionAutomation {
    #[serde(rename = "AnswerSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answer_source: Option<EvaluationFormQuestionAutomationAnswerSource>,
    #[serde(rename = "DefaultOptionRefIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_option_ref_ids: Option<Vec<String>>,
    #[serde(rename = "Options")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<EvaluationFormMultiSelectQuestionAutomationOption>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationFormQuestionAutomationAnswerSource {
    #[serde(rename = "SourceType")]
    #[serde(default)]
    pub source_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationFormMultiSelectQuestionAutomationOption {
    #[serde(rename = "RuleCategory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_category: Option<MultiSelectQuestionRuleCategoryAutomation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MultiSelectQuestionRuleCategoryAutomation {
    #[serde(rename = "Category")]
    #[serde(default)]
    pub category: String,
    #[serde(rename = "Condition")]
    #[serde(default)]
    pub condition: String,
    #[serde(rename = "OptionRefIds")]
    #[serde(default)]
    pub option_ref_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationFormMultiSelectQuestionOption {
    #[serde(rename = "RefId")]
    #[serde(default)]
    pub ref_id: String,
    #[serde(rename = "Text")]
    #[serde(default)]
    pub text: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationFormNumericQuestionProperties {
    #[serde(rename = "Automation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automation: Option<EvaluationFormNumericQuestionAutomation>,
    #[serde(rename = "MaxValue")]
    #[serde(default)]
    pub max_value: i32,
    #[serde(rename = "MinValue")]
    #[serde(default)]
    pub min_value: i32,
    #[serde(rename = "Options")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<EvaluationFormNumericQuestionOption>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationFormNumericQuestionAutomation {
    #[serde(rename = "AnswerSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answer_source: Option<EvaluationFormQuestionAutomationAnswerSource>,
    #[serde(rename = "PropertyValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_value: Option<NumericQuestionPropertyValueAutomation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NumericQuestionPropertyValueAutomation {
    #[serde(rename = "Label")]
    #[serde(default)]
    pub label: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationFormNumericQuestionOption {
    #[serde(rename = "AutomaticFail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_fail: Option<bool>,
    #[serde(rename = "AutomaticFailConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_fail_configuration: Option<AutomaticFailConfiguration>,
    #[serde(rename = "MaxValue")]
    #[serde(default)]
    pub max_value: i32,
    #[serde(rename = "MinValue")]
    #[serde(default)]
    pub min_value: i32,
    #[serde(rename = "Score")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomaticFailConfiguration {
    #[serde(rename = "TargetSection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_section: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationFormSingleSelectQuestionProperties {
    #[serde(rename = "Automation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automation: Option<EvaluationFormSingleSelectQuestionAutomation>,
    #[serde(rename = "DisplayAs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_as: Option<String>,
    #[serde(rename = "Options")]
    #[serde(default)]
    pub options: Vec<EvaluationFormSingleSelectQuestionOption>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationFormSingleSelectQuestionAutomation {
    #[serde(rename = "AnswerSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answer_source: Option<EvaluationFormQuestionAutomationAnswerSource>,
    #[serde(rename = "DefaultOptionRefId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_option_ref_id: Option<String>,
    #[serde(rename = "Options")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<EvaluationFormSingleSelectQuestionAutomationOption>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationFormSingleSelectQuestionAutomationOption {
    #[serde(rename = "RuleCategory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_category: Option<SingleSelectQuestionRuleCategoryAutomation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SingleSelectQuestionRuleCategoryAutomation {
    #[serde(rename = "Category")]
    #[serde(default)]
    pub category: String,
    #[serde(rename = "Condition")]
    #[serde(default)]
    pub condition: String,
    #[serde(rename = "OptionRefId")]
    #[serde(default)]
    pub option_ref_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationFormSingleSelectQuestionOption {
    #[serde(rename = "AutomaticFail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_fail: Option<bool>,
    #[serde(rename = "AutomaticFailConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_fail_configuration: Option<AutomaticFailConfiguration>,
    #[serde(rename = "RefId")]
    #[serde(default)]
    pub ref_id: String,
    #[serde(rename = "Score")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<i32>,
    #[serde(rename = "Text")]
    #[serde(default)]
    pub text: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationFormTextQuestionProperties {
    #[serde(rename = "Automation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automation: Option<EvaluationFormTextQuestionAutomation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationFormTextQuestionAutomation {
    #[serde(rename = "AnswerSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answer_source: Option<EvaluationFormQuestionAutomationAnswerSource>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationFormSection {
    #[serde(rename = "Instructions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    #[serde(rename = "Items")]
    #[serde(default)]
    pub items: Vec<EvaluationFormItem>,
    #[serde(rename = "RefId")]
    #[serde(default)]
    pub ref_id: String,
    #[serde(rename = "Title")]
    #[serde(default)]
    pub title: String,
    #[serde(rename = "Weight")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationFormLanguageConfiguration {
    #[serde(rename = "FormLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form_language: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationReviewConfiguration {
    #[serde(rename = "EligibilityDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eligibility_days: Option<i32>,
    #[serde(rename = "ReviewNotificationRecipients")]
    #[serde(default)]
    pub review_notification_recipients: Vec<EvaluationReviewNotificationRecipient>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationReviewNotificationRecipient {
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: EvaluationReviewNotificationRecipientValue,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationReviewNotificationRecipientValue {
    #[serde(rename = "UserId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationFormScoringStrategy {
    #[serde(rename = "Mode")]
    #[serde(default)]
    pub mode: String,
    #[serde(rename = "Status")]
    #[serde(default)]
    pub status: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationFormTargetConfiguration {
    #[serde(rename = "ContactInteractionType")]
    #[serde(default)]
    pub contact_interaction_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateEvaluationFormResponse {
    #[serde(rename = "EvaluationFormArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_form_arn: Option<String>,
    #[serde(rename = "EvaluationFormId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_form_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateHoursOfOperationOverrideRequest {
    #[serde(rename = "Config")]
    #[serde(default)]
    pub config: Vec<HoursOfOperationOverrideConfig>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EffectiveFrom")]
    #[serde(default)]
    pub effective_from: String,
    #[serde(rename = "EffectiveTill")]
    #[serde(default)]
    pub effective_till: String,
    #[serde(rename = "HoursOfOperationId")]
    #[serde(default)]
    pub hours_of_operation_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "OverrideType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_type: Option<String>,
    #[serde(rename = "RecurrenceConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrence_config: Option<RecurrenceConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HoursOfOperationOverrideConfig {
    #[serde(rename = "Day")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day: Option<String>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<OverrideTimeSlice>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<OverrideTimeSlice>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OverrideTimeSlice {
    #[serde(rename = "Hours")]
    #[serde(default)]
    pub hours: i32,
    #[serde(rename = "Minutes")]
    #[serde(default)]
    pub minutes: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecurrenceConfig {
    #[serde(rename = "RecurrencePattern")]
    #[serde(default)]
    pub recurrence_pattern: RecurrencePattern,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecurrencePattern {
    #[serde(rename = "ByMonth")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_month: Option<Vec<i32>>,
    #[serde(rename = "ByMonthDay")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_month_day: Option<Vec<i32>>,
    #[serde(rename = "ByWeekdayOccurrence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_weekday_occurrence: Option<Vec<i32>>,
    #[serde(rename = "Frequency")]
    #[serde(default)]
    pub frequency: String,
    #[serde(rename = "Interval")]
    #[serde(default)]
    pub interval: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateHoursOfOperationOverrideResponse {
    #[serde(rename = "HoursOfOperationOverrideId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hours_of_operation_override_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateHoursOfOperationRequest {
    #[serde(rename = "Config")]
    #[serde(default)]
    pub config: Vec<HoursOfOperationConfig>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "ParentHoursOfOperationConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_hours_of_operation_configs: Option<Vec<ParentHoursOfOperationConfig>>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "TimeZone")]
    #[serde(default)]
    pub time_zone: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HoursOfOperationConfig {
    #[serde(rename = "Day")]
    #[serde(default)]
    pub day: String,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    pub end_time: HoursOfOperationTimeSlice,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    pub start_time: HoursOfOperationTimeSlice,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HoursOfOperationTimeSlice {
    #[serde(rename = "Hours")]
    #[serde(default)]
    pub hours: i32,
    #[serde(rename = "Minutes")]
    #[serde(default)]
    pub minutes: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateHoursOfOperationResponse {
    #[serde(rename = "HoursOfOperationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hours_of_operation_arn: Option<String>,
    #[serde(rename = "HoursOfOperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hours_of_operation_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateInstanceRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    #[serde(rename = "IdentityManagementType")]
    #[serde(default)]
    pub identity_management_type: String,
    #[serde(rename = "InboundCallsEnabled")]
    #[serde(default)]
    pub inbound_calls_enabled: bool,
    #[serde(rename = "InstanceAlias")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_alias: Option<String>,
    #[serde(rename = "OutboundCallsEnabled")]
    #[serde(default)]
    pub outbound_calls_enabled: bool,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateInstanceResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateIntegrationAssociationRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "IntegrationArn")]
    #[serde(default)]
    pub integration_arn: String,
    #[serde(rename = "IntegrationType")]
    #[serde(default)]
    pub integration_type: String,
    #[serde(rename = "SourceApplicationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_application_name: Option<String>,
    #[serde(rename = "SourceApplicationUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_application_url: Option<String>,
    #[serde(rename = "SourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateIntegrationAssociationResponse {
    #[serde(rename = "IntegrationAssociationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_association_arn: Option<String>,
    #[serde(rename = "IntegrationAssociationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_association_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateNotificationRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "Content")]
    #[serde(default)]
    pub content: std::collections::HashMap<String, String>,
    #[serde(rename = "ExpiresAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<f64>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "PredefinedNotificationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predefined_notification_id: Option<String>,
    #[serde(rename = "Priority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    #[serde(rename = "Recipients")]
    #[serde(default)]
    pub recipients: Vec<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateNotificationResponse {
    #[serde(rename = "NotificationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_arn: Option<String>,
    #[serde(rename = "NotificationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateParticipantRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "ContactId")]
    #[serde(default)]
    pub contact_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "ParticipantDetails")]
    #[serde(default)]
    pub participant_details: ParticipantDetailsToAdd,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ParticipantDetailsToAdd {
    #[serde(rename = "DisplayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "ParticipantCapabilities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant_capabilities: Option<ParticipantCapabilities>,
    #[serde(rename = "ParticipantRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant_role: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ParticipantCapabilities {
    #[serde(rename = "ScreenShare")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screen_share: Option<String>,
    #[serde(rename = "Video")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateParticipantResponse {
    #[serde(rename = "ParticipantCredentials")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant_credentials: Option<ParticipantTokenCredentials>,
    #[serde(rename = "ParticipantId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ParticipantTokenCredentials {
    #[serde(rename = "Expiry")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry: Option<String>,
    #[serde(rename = "ParticipantToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePersistentContactAssociationRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "InitialContactId")]
    #[serde(default)]
    pub initial_contact_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "RehydrationType")]
    #[serde(default)]
    pub rehydration_type: String,
    #[serde(rename = "SourceContactId")]
    #[serde(default)]
    pub source_contact_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePersistentContactAssociationResponse {
    #[serde(rename = "ContinuedFromContactId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continued_from_contact_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePredefinedAttributeRequest {
    #[serde(rename = "AttributeConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_configuration: Option<InputPredefinedAttributeConfiguration>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Purposes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purposes: Option<Vec<String>>,
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<PredefinedAttributeValues>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InputPredefinedAttributeConfiguration {
    #[serde(rename = "EnableValueValidationOnAssociation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_value_validation_on_association: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PredefinedAttributeValues {
    #[serde(rename = "StringList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_list: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePromptRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "S3Uri")]
    #[serde(default)]
    pub s3_uri: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePromptResponse {
    #[serde(rename = "PromptARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_a_r_n: Option<String>,
    #[serde(rename = "PromptId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePushNotificationRegistrationRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "ContactConfiguration")]
    #[serde(default)]
    pub contact_configuration: ContactConfiguration,
    #[serde(rename = "DeviceToken")]
    #[serde(default)]
    pub device_token: String,
    #[serde(rename = "DeviceType")]
    #[serde(default)]
    pub device_type: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "PinpointAppArn")]
    #[serde(default)]
    pub pinpoint_app_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContactConfiguration {
    #[serde(rename = "ContactId")]
    #[serde(default)]
    pub contact_id: String,
    #[serde(rename = "IncludeRawMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_raw_message: Option<bool>,
    #[serde(rename = "ParticipantRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant_role: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePushNotificationRegistrationResponse {
    #[serde(rename = "RegistrationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateQueueRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EmailAddressesConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_addresses_config: Option<Vec<EmailAddressConfig>>,
    #[serde(rename = "HoursOfOperationId")]
    #[serde(default)]
    pub hours_of_operation_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "MaxContacts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_contacts: Option<i32>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "OutboundCallerConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_caller_config: Option<OutboundCallerConfig>,
    #[serde(rename = "OutboundEmailConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_email_config: Option<OutboundEmailConfig>,
    #[serde(rename = "QuickConnectIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quick_connect_ids: Option<Vec<String>>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OutboundCallerConfig {
    #[serde(rename = "OutboundCallerIdName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_caller_id_name: Option<String>,
    #[serde(rename = "OutboundCallerIdNumberId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_caller_id_number_id: Option<String>,
    #[serde(rename = "OutboundFlowId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_flow_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OutboundEmailConfig {
    #[serde(rename = "OutboundEmailAddressId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_email_address_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateQueueResponse {
    #[serde(rename = "QueueArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_arn: Option<String>,
    #[serde(rename = "QueueId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateQuickConnectRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "QuickConnectConfig")]
    #[serde(default)]
    pub quick_connect_config: QuickConnectConfig,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QuickConnectConfig {
    #[serde(rename = "FlowConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_config: Option<FlowQuickConnectConfig>,
    #[serde(rename = "PhoneConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_config: Option<PhoneNumberQuickConnectConfig>,
    #[serde(rename = "QueueConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_config: Option<QueueQuickConnectConfig>,
    #[serde(rename = "QuickConnectType")]
    #[serde(default)]
    pub quick_connect_type: String,
    #[serde(rename = "UserConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_config: Option<UserQuickConnectConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FlowQuickConnectConfig {
    #[serde(rename = "ContactFlowId")]
    #[serde(default)]
    pub contact_flow_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PhoneNumberQuickConnectConfig {
    #[serde(rename = "PhoneNumber")]
    #[serde(default)]
    pub phone_number: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueueQuickConnectConfig {
    #[serde(rename = "ContactFlowId")]
    #[serde(default)]
    pub contact_flow_id: String,
    #[serde(rename = "QueueId")]
    #[serde(default)]
    pub queue_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserQuickConnectConfig {
    #[serde(rename = "ContactFlowId")]
    #[serde(default)]
    pub contact_flow_id: String,
    #[serde(rename = "UserId")]
    #[serde(default)]
    pub user_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateQuickConnectResponse {
    #[serde(rename = "QuickConnectARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quick_connect_a_r_n: Option<String>,
    #[serde(rename = "QuickConnectId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quick_connect_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateRoutingProfileRequest {
    #[serde(rename = "AgentAvailabilityTimer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_availability_timer: Option<String>,
    #[serde(rename = "DefaultOutboundQueueId")]
    #[serde(default)]
    pub default_outbound_queue_id: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    pub description: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "ManualAssignmentQueueConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_assignment_queue_configs: Option<Vec<RoutingProfileManualAssignmentQueueConfig>>,
    #[serde(rename = "MediaConcurrencies")]
    #[serde(default)]
    pub media_concurrencies: Vec<MediaConcurrency>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "QueueConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_configs: Option<Vec<RoutingProfileQueueConfig>>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MediaConcurrency {
    #[serde(rename = "Channel")]
    #[serde(default)]
    pub channel: String,
    #[serde(rename = "Concurrency")]
    #[serde(default)]
    pub concurrency: i32,
    #[serde(rename = "CrossChannelBehavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_channel_behavior: Option<CrossChannelBehavior>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CrossChannelBehavior {
    #[serde(rename = "BehaviorType")]
    #[serde(default)]
    pub behavior_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateRoutingProfileResponse {
    #[serde(rename = "RoutingProfileArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_profile_arn: Option<String>,
    #[serde(rename = "RoutingProfileId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_profile_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateRuleRequest {
    #[serde(rename = "Actions")]
    #[serde(default)]
    pub actions: Vec<RuleAction>,
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "Function")]
    #[serde(default)]
    pub function: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "PublishStatus")]
    #[serde(default)]
    pub publish_status: String,
    #[serde(rename = "TriggerEventSource")]
    #[serde(default)]
    pub trigger_event_source: RuleTriggerEventSource,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuleAction {
    #[serde(rename = "ActionType")]
    #[serde(default)]
    pub action_type: String,
    #[serde(rename = "AssignContactCategoryAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assign_contact_category_action: Option<AssignContactCategoryActionDefinition>,
    #[serde(rename = "AssignSlaAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assign_sla_action: Option<AssignSlaActionDefinition>,
    #[serde(rename = "CreateCaseAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_case_action: Option<CreateCaseActionDefinition>,
    #[serde(rename = "EndAssociatedTasksAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_associated_tasks_action: Option<EndAssociatedTasksActionDefinition>,
    #[serde(rename = "EventBridgeAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bridge_action: Option<EventBridgeActionDefinition>,
    #[serde(rename = "SendNotificationAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_notification_action: Option<SendNotificationActionDefinition>,
    #[serde(rename = "SubmitAutoEvaluationAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_auto_evaluation_action: Option<SubmitAutoEvaluationActionDefinition>,
    #[serde(rename = "TaskAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_action: Option<TaskActionDefinition>,
    #[serde(rename = "UpdateCaseAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_case_action: Option<UpdateCaseActionDefinition>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssignContactCategoryActionDefinition {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssignSlaActionDefinition {
    #[serde(rename = "CaseSlaConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub case_sla_configuration: Option<CaseSlaConfiguration>,
    #[serde(rename = "SlaAssignmentType")]
    #[serde(default)]
    pub sla_assignment_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CaseSlaConfiguration {
    #[serde(rename = "FieldId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "TargetFieldValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_field_values: Option<Vec<FieldValueUnion>>,
    #[serde(rename = "TargetSlaMinutes")]
    #[serde(default)]
    pub target_sla_minutes: i64,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FieldValueUnion {
    #[serde(rename = "BooleanValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boolean_value: Option<bool>,
    #[serde(rename = "DoubleValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub double_value: Option<f64>,
    #[serde(rename = "EmptyValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty_value: Option<EmptyFieldValue>,
    #[serde(rename = "StringValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EmptyFieldValue {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCaseActionDefinition {
    #[serde(rename = "Fields")]
    #[serde(default)]
    pub fields: Vec<FieldValue>,
    #[serde(rename = "TemplateId")]
    #[serde(default)]
    pub template_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FieldValue {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: FieldValueUnion,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EndAssociatedTasksActionDefinition {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventBridgeActionDefinition {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SendNotificationActionDefinition {
    #[serde(rename = "Content")]
    #[serde(default)]
    pub content: String,
    #[serde(rename = "ContentType")]
    #[serde(default)]
    pub content_type: String,
    #[serde(rename = "DeliveryMethod")]
    #[serde(default)]
    pub delivery_method: String,
    #[serde(rename = "Exclusion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusion: Option<NotificationRecipientType>,
    #[serde(rename = "Recipient")]
    #[serde(default)]
    pub recipient: NotificationRecipientType,
    #[serde(rename = "Subject")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NotificationRecipientType {
    #[serde(rename = "UserIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
    #[serde(rename = "UserTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SubmitAutoEvaluationActionDefinition {
    #[serde(rename = "EvaluationFormId")]
    #[serde(default)]
    pub evaluation_form_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TaskActionDefinition {
    #[serde(rename = "ContactFlowId")]
    #[serde(default)]
    pub contact_flow_id: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "References")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub references: Option<std::collections::HashMap<String, Reference>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCaseActionDefinition {
    #[serde(rename = "Fields")]
    #[serde(default)]
    pub fields: Vec<FieldValue>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuleTriggerEventSource {
    #[serde(rename = "EventSourceName")]
    #[serde(default)]
    pub event_source_name: String,
    #[serde(rename = "IntegrationAssociationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_association_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateRuleResponse {
    #[serde(rename = "RuleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_arn: Option<String>,
    #[serde(rename = "RuleId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSecurityProfileRequest {
    #[serde(rename = "AllowedAccessControlHierarchyGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_access_control_hierarchy_group_id: Option<String>,
    #[serde(rename = "AllowedAccessControlTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_access_control_tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "AllowedFlowModules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_flow_modules: Option<Vec<FlowModule>>,
    #[serde(rename = "Applications")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applications: Option<Vec<Application>>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "GranularAccessControlConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granular_access_control_configuration: Option<GranularAccessControlConfiguration>,
    #[serde(rename = "HierarchyRestrictedResources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hierarchy_restricted_resources: Option<Vec<String>>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Permissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
    #[serde(rename = "SecurityProfileName")]
    #[serde(default)]
    pub security_profile_name: String,
    #[serde(rename = "TagRestrictedResources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_restricted_resources: Option<Vec<String>>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FlowModule {
    #[serde(rename = "FlowModuleId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_module_id: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Application {
    #[serde(rename = "ApplicationPermissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_permissions: Option<Vec<String>>,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GranularAccessControlConfiguration {
    #[serde(rename = "DataTableAccessControlConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_table_access_control_configuration: Option<DataTableAccessControlConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataTableAccessControlConfiguration {
    #[serde(rename = "PrimaryAttributeAccessControlConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_attribute_access_control_configuration:
        Option<PrimaryAttributeAccessControlConfigurationItem>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PrimaryAttributeAccessControlConfigurationItem {
    #[serde(rename = "PrimaryAttributeValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_attribute_values: Option<Vec<PrimaryAttributeValue>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PrimaryAttributeValue {
    #[serde(rename = "AccessType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_type: Option<String>,
    #[serde(rename = "AttributeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_name: Option<String>,
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSecurityProfileResponse {
    #[serde(rename = "SecurityProfileArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_profile_arn: Option<String>,
    #[serde(rename = "SecurityProfileId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_profile_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTaskTemplateRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "Constraints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraints: Option<TaskTemplateConstraints>,
    #[serde(rename = "ContactFlowId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_flow_id: Option<String>,
    #[serde(rename = "Defaults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defaults: Option<TaskTemplateDefaults>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Fields")]
    #[serde(default)]
    pub fields: Vec<TaskTemplateField>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "SelfAssignFlowId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_assign_flow_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TaskTemplateConstraints {
    #[serde(rename = "InvisibleFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invisible_fields: Option<Vec<InvisibleFieldInfo>>,
    #[serde(rename = "ReadOnlyFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only_fields: Option<Vec<ReadOnlyFieldInfo>>,
    #[serde(rename = "RequiredFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_fields: Option<Vec<RequiredFieldInfo>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InvisibleFieldInfo {
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<TaskTemplateFieldIdentifier>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TaskTemplateFieldIdentifier {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReadOnlyFieldInfo {
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<TaskTemplateFieldIdentifier>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RequiredFieldInfo {
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<TaskTemplateFieldIdentifier>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TaskTemplateDefaults {
    #[serde(rename = "DefaultFieldValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_field_values: Option<Vec<TaskTemplateDefaultFieldValue>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TaskTemplateDefaultFieldValue {
    #[serde(rename = "DefaultValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<TaskTemplateFieldIdentifier>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TaskTemplateField {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: TaskTemplateFieldIdentifier,
    #[serde(rename = "SingleSelectOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_select_options: Option<Vec<String>>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTaskTemplateResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTestCaseRequest {
    #[serde(rename = "Content")]
    #[serde(default)]
    pub content: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EntryPoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_point: Option<TestCaseEntryPoint>,
    #[serde(rename = "InitializationData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initialization_data: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "LastModifiedRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_region: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "TestCaseId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_case_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TestCaseEntryPoint {
    #[serde(rename = "ChatEntryPointParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_entry_point_parameters: Option<ChatEntryPointParameters>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "VoiceCallEntryPointParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_call_entry_point_parameters: Option<VoiceCallEntryPointParameters>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChatEntryPointParameters {
    #[serde(rename = "FlowId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VoiceCallEntryPointParameters {
    #[serde(rename = "DestinationPhoneNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_phone_number: Option<String>,
    #[serde(rename = "FlowId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_id: Option<String>,
    #[serde(rename = "SourcePhoneNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_phone_number: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTestCaseResponse {
    #[serde(rename = "TestCaseArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_case_arn: Option<String>,
    #[serde(rename = "TestCaseId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_case_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTrafficDistributionGroupRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTrafficDistributionGroupResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateUseCaseRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "IntegrationAssociationId")]
    #[serde(default)]
    pub integration_association_id: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "UseCaseType")]
    #[serde(default)]
    pub use_case_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateUseCaseResponse {
    #[serde(rename = "UseCaseArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_case_arn: Option<String>,
    #[serde(rename = "UseCaseId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_case_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateUserHierarchyGroupRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "ParentGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_group_id: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateUserHierarchyGroupResponse {
    #[serde(rename = "HierarchyGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hierarchy_group_arn: Option<String>,
    #[serde(rename = "HierarchyGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hierarchy_group_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateUserRequest {
    #[serde(rename = "AfterContactWorkConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_contact_work_configs: Option<Vec<AfterContactWorkConfigPerChannel>>,
    #[serde(rename = "AutoAcceptConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_accept_configs: Option<Vec<AutoAcceptConfig>>,
    #[serde(rename = "DirectoryUserId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_user_id: Option<String>,
    #[serde(rename = "HierarchyGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hierarchy_group_id: Option<String>,
    #[serde(rename = "IdentityInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_info: Option<UserIdentityInfo>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Password")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "PersistentConnectionConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent_connection_configs: Option<Vec<PersistentConnectionConfig>>,
    #[serde(rename = "PhoneConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_config: Option<UserPhoneConfig>,
    #[serde(rename = "PhoneNumberConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_configs: Option<Vec<PhoneNumberConfig>>,
    #[serde(rename = "RoutingProfileId")]
    #[serde(default)]
    pub routing_profile_id: String,
    #[serde(rename = "SecurityProfileIds")]
    #[serde(default)]
    pub security_profile_ids: Vec<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Username")]
    #[serde(default)]
    pub username: String,
    #[serde(rename = "VoiceEnhancementConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_enhancement_configs: Option<Vec<VoiceEnhancementConfig>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AfterContactWorkConfigPerChannel {
    #[serde(rename = "AfterContactWorkConfig")]
    #[serde(default)]
    pub after_contact_work_config: AfterContactWorkConfig,
    #[serde(rename = "AgentFirstCallbackAfterContactWorkConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_first_callback_after_contact_work_config: Option<AfterContactWorkConfig>,
    #[serde(rename = "Channel")]
    #[serde(default)]
    pub channel: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AfterContactWorkConfig {
    #[serde(rename = "AfterContactWorkTimeLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_contact_work_time_limit: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutoAcceptConfig {
    #[serde(rename = "AgentFirstCallbackAutoAccept")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_first_callback_auto_accept: Option<bool>,
    #[serde(rename = "AutoAccept")]
    #[serde(default)]
    pub auto_accept: bool,
    #[serde(rename = "Channel")]
    #[serde(default)]
    pub channel: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserIdentityInfo {
    #[serde(rename = "Email")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "FirstName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(rename = "LastName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(rename = "Mobile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    #[serde(rename = "SecondaryEmail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_email: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PersistentConnectionConfig {
    #[serde(rename = "Channel")]
    #[serde(default)]
    pub channel: String,
    #[serde(rename = "PersistentConnection")]
    #[serde(default)]
    pub persistent_connection: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserPhoneConfig {
    #[serde(rename = "AfterContactWorkTimeLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_contact_work_time_limit: Option<i32>,
    #[serde(rename = "AutoAccept")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_accept: Option<bool>,
    #[serde(rename = "DeskPhoneNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desk_phone_number: Option<String>,
    #[serde(rename = "PersistentConnection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent_connection: Option<bool>,
    #[serde(rename = "PhoneType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PhoneNumberConfig {
    #[serde(rename = "Channel")]
    #[serde(default)]
    pub channel: String,
    #[serde(rename = "PhoneNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(rename = "PhoneType")]
    #[serde(default)]
    pub phone_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VoiceEnhancementConfig {
    #[serde(rename = "Channel")]
    #[serde(default)]
    pub channel: String,
    #[serde(rename = "VoiceEnhancementMode")]
    #[serde(default)]
    pub voice_enhancement_mode: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateUserResponse {
    #[serde(rename = "UserArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_arn: Option<String>,
    #[serde(rename = "UserId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateViewRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "Content")]
    #[serde(default)]
    pub content: ViewInputContent,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Status")]
    #[serde(default)]
    pub status: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ViewInputContent {
    #[serde(rename = "Actions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<String>>,
    #[serde(rename = "Template")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateViewResponse {
    #[serde(rename = "View")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view: Option<View>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct View {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Content")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<ViewContent>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
    #[serde(rename = "VersionDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_description: Option<String>,
    #[serde(rename = "ViewContentSha256")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_content_sha256: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ViewContent {
    #[serde(rename = "Actions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<String>>,
    #[serde(rename = "InputSchema")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_schema: Option<String>,
    #[serde(rename = "Template")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateViewVersionRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "VersionDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_description: Option<String>,
    #[serde(rename = "ViewContentSha256")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_content_sha256: Option<String>,
    #[serde(rename = "ViewId")]
    #[serde(default)]
    pub view_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateViewVersionResponse {
    #[serde(rename = "View")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view: Option<View>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateVocabularyRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "Content")]
    #[serde(default)]
    pub content: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    pub language_code: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "VocabularyName")]
    #[serde(default)]
    pub vocabulary_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateVocabularyResponse {
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "VocabularyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_arn: Option<String>,
    #[serde(rename = "VocabularyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateWorkspacePageRequest {
    #[serde(rename = "InputData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_data: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Page")]
    #[serde(default)]
    pub page: String,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "Slug")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(rename = "WorkspaceId")]
    #[serde(default)]
    pub workspace_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateWorkspacePageResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateWorkspaceRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Theme")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme: Option<WorkspaceTheme>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkspaceTheme {
    #[serde(rename = "Dark")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dark: Option<WorkspaceThemeConfig>,
    #[serde(rename = "Light")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light: Option<WorkspaceThemeConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkspaceThemeConfig {
    #[serde(rename = "Images")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<WorkspaceThemeImages>,
    #[serde(rename = "Palette")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub palette: Option<WorkspaceThemePalette>,
    #[serde(rename = "Typography")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typography: Option<WorkspaceThemeTypography>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkspaceThemeImages {
    #[serde(rename = "Logo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<ImagesLogo>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImagesLogo {
    #[serde(rename = "Default")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
    #[serde(rename = "Favicon")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favicon: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkspaceThemePalette {
    #[serde(rename = "Canvas")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canvas: Option<PaletteCanvas>,
    #[serde(rename = "Header")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<PaletteHeader>,
    #[serde(rename = "Navigation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub navigation: Option<PaletteNavigation>,
    #[serde(rename = "Primary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<PalettePrimary>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PaletteCanvas {
    #[serde(rename = "ActiveBackground")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_background: Option<String>,
    #[serde(rename = "ContainerBackground")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_background: Option<String>,
    #[serde(rename = "PageBackground")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_background: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PaletteHeader {
    #[serde(rename = "Background")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<String>,
    #[serde(rename = "InvertActionsColors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invert_actions_colors: Option<bool>,
    #[serde(rename = "Text")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "TextHover")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_hover: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PaletteNavigation {
    #[serde(rename = "Background")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<String>,
    #[serde(rename = "InvertActionsColors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invert_actions_colors: Option<bool>,
    #[serde(rename = "Text")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "TextActive")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_active: Option<String>,
    #[serde(rename = "TextBackgroundActive")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_background_active: Option<String>,
    #[serde(rename = "TextBackgroundHover")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_background_hover: Option<String>,
    #[serde(rename = "TextHover")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_hover: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PalettePrimary {
    #[serde(rename = "Active")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<String>,
    #[serde(rename = "ContrastText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contrast_text: Option<String>,
    #[serde(rename = "Default")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkspaceThemeTypography {
    #[serde(rename = "FontFamily")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_family: Option<FontFamily>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FontFamily {
    #[serde(rename = "Default")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateWorkspaceResponse {
    #[serde(rename = "WorkspaceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_arn: Option<String>,
    #[serde(rename = "WorkspaceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeactivateEvaluationFormRequest {
    #[serde(rename = "EvaluationFormId")]
    #[serde(default)]
    pub evaluation_form_id: String,
    #[serde(rename = "EvaluationFormVersion")]
    #[serde(default)]
    pub evaluation_form_version: i32,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeactivateEvaluationFormResponse {
    #[serde(rename = "EvaluationFormArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_form_arn: Option<String>,
    #[serde(rename = "EvaluationFormId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_form_id: Option<String>,
    #[serde(rename = "EvaluationFormVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_form_version: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAttachedFileRequest {
    #[serde(rename = "AssociatedResourceArn")]
    #[serde(default)]
    pub associated_resource_arn: String,
    #[serde(rename = "FileId")]
    #[serde(default)]
    pub file_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAttachedFileResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteContactEvaluationRequest {
    #[serde(rename = "EvaluationId")]
    #[serde(default)]
    pub evaluation_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteContactFlowModuleAliasRequest {
    #[serde(rename = "AliasId")]
    #[serde(default)]
    pub alias_id: String,
    #[serde(rename = "ContactFlowModuleId")]
    #[serde(default)]
    pub contact_flow_module_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteContactFlowModuleAliasResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteContactFlowModuleRequest {
    #[serde(rename = "ContactFlowModuleId")]
    #[serde(default)]
    pub contact_flow_module_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteContactFlowModuleResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteContactFlowModuleVersionRequest {
    #[serde(rename = "ContactFlowModuleId")]
    #[serde(default)]
    pub contact_flow_module_id: String,
    #[serde(rename = "ContactFlowModuleVersion")]
    #[serde(default)]
    pub contact_flow_module_version: i64,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteContactFlowModuleVersionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteContactFlowRequest {
    #[serde(rename = "ContactFlowId")]
    #[serde(default)]
    pub contact_flow_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteContactFlowResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteContactFlowVersionRequest {
    #[serde(rename = "ContactFlowId")]
    #[serde(default)]
    pub contact_flow_id: String,
    #[serde(rename = "ContactFlowVersion")]
    #[serde(default)]
    pub contact_flow_version: i64,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteContactFlowVersionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDataTableAttributeRequest {
    #[serde(rename = "AttributeName")]
    #[serde(default)]
    pub attribute_name: String,
    #[serde(rename = "DataTableId")]
    #[serde(default)]
    pub data_table_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDataTableAttributeResponse {
    #[serde(rename = "LockVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_version: Option<DataTableLockVersion>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDataTableRequest {
    #[serde(rename = "DataTableId")]
    #[serde(default)]
    pub data_table_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDataTableResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteEmailAddressRequest {
    #[serde(rename = "EmailAddressId")]
    #[serde(default)]
    pub email_address_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteEmailAddressResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteEvaluationFormRequest {
    #[serde(rename = "EvaluationFormId")]
    #[serde(default)]
    pub evaluation_form_id: String,
    #[serde(rename = "EvaluationFormVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_form_version: Option<i32>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteHoursOfOperationOverrideRequest {
    #[serde(rename = "HoursOfOperationId")]
    #[serde(default)]
    pub hours_of_operation_id: String,
    #[serde(rename = "HoursOfOperationOverrideId")]
    #[serde(default)]
    pub hours_of_operation_override_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteHoursOfOperationRequest {
    #[serde(rename = "HoursOfOperationId")]
    #[serde(default)]
    pub hours_of_operation_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteInstanceRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteIntegrationAssociationRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "IntegrationAssociationId")]
    #[serde(default)]
    pub integration_association_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteNotificationRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "NotificationId")]
    #[serde(default)]
    pub notification_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteNotificationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePredefinedAttributeRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePromptRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "PromptId")]
    #[serde(default)]
    pub prompt_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePushNotificationRegistrationRequest {
    #[serde(rename = "ContactId")]
    #[serde(default)]
    pub contact_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "RegistrationId")]
    #[serde(default)]
    pub registration_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePushNotificationRegistrationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteQueueRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "QueueId")]
    #[serde(default)]
    pub queue_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteQuickConnectRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "QuickConnectId")]
    #[serde(default)]
    pub quick_connect_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRoutingProfileRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "RoutingProfileId")]
    #[serde(default)]
    pub routing_profile_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRuleRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "RuleId")]
    #[serde(default)]
    pub rule_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSecurityProfileRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "SecurityProfileId")]
    #[serde(default)]
    pub security_profile_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTaskTemplateRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "TaskTemplateId")]
    #[serde(default)]
    pub task_template_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTaskTemplateResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTestCaseRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "TestCaseId")]
    #[serde(default)]
    pub test_case_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTestCaseResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTrafficDistributionGroupRequest {
    #[serde(rename = "TrafficDistributionGroupId")]
    #[serde(default)]
    pub traffic_distribution_group_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTrafficDistributionGroupResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteUseCaseRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "IntegrationAssociationId")]
    #[serde(default)]
    pub integration_association_id: String,
    #[serde(rename = "UseCaseId")]
    #[serde(default)]
    pub use_case_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteUserHierarchyGroupRequest {
    #[serde(rename = "HierarchyGroupId")]
    #[serde(default)]
    pub hierarchy_group_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteUserRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "UserId")]
    #[serde(default)]
    pub user_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteViewRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "ViewId")]
    #[serde(default)]
    pub view_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteViewResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteViewVersionRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "ViewId")]
    #[serde(default)]
    pub view_id: String,
    #[serde(rename = "ViewVersion")]
    #[serde(default)]
    pub view_version: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteViewVersionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteVocabularyRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "VocabularyId")]
    #[serde(default)]
    pub vocabulary_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteVocabularyResponse {
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "VocabularyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_arn: Option<String>,
    #[serde(rename = "VocabularyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteWorkspaceMediaRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "MediaType")]
    #[serde(default)]
    pub media_type: String,
    #[serde(rename = "WorkspaceId")]
    #[serde(default)]
    pub workspace_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteWorkspaceMediaResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteWorkspacePageRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Page")]
    #[serde(default)]
    pub page: String,
    #[serde(rename = "WorkspaceId")]
    #[serde(default)]
    pub workspace_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteWorkspacePageResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteWorkspaceRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "WorkspaceId")]
    #[serde(default)]
    pub workspace_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteWorkspaceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAgentStatusRequest {
    #[serde(rename = "AgentStatusId")]
    #[serde(default)]
    pub agent_status_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAgentStatusResponse {
    #[serde(rename = "AgentStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_status: Option<AgentStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AgentStatus {
    #[serde(rename = "AgentStatusARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_status_a_r_n: Option<String>,
    #[serde(rename = "AgentStatusId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_status_id: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "DisplayOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_order: Option<i32>,
    #[serde(rename = "LastModifiedRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_region: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAttachedFilesConfigurationRequest {
    #[serde(rename = "AttachmentScope")]
    #[serde(default)]
    pub attachment_scope: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAttachedFilesConfigurationResponse {
    #[serde(rename = "AttachedFilesConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attached_files_configuration: Option<AttachedFilesConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttachedFilesConfiguration {
    #[serde(rename = "AttachmentScope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_scope: Option<String>,
    #[serde(rename = "ExtensionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_configuration: Option<ExtensionConfiguration>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "MaximumSizeLimitInBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_size_limit_in_bytes: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExtensionConfiguration {
    #[serde(rename = "AllowedExtensions")]
    #[serde(default)]
    pub allowed_extensions: Vec<AllowedExtension>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AllowedExtension {
    #[serde(rename = "Extension")]
    #[serde(default)]
    pub extension: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAuthenticationProfileRequest {
    #[serde(rename = "AuthenticationProfileId")]
    #[serde(default)]
    pub authentication_profile_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAuthenticationProfileResponse {
    #[serde(rename = "AuthenticationProfile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_profile: Option<AuthenticationProfile>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuthenticationProfile {
    #[serde(rename = "AllowedIps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_ips: Option<Vec<String>>,
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "BlockedIps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked_ips: Option<Vec<String>>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "IsDefault")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[serde(rename = "LastModifiedRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_region: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "MaxSessionDuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_session_duration: Option<i32>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "PeriodicSessionDuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub periodic_session_duration: Option<i32>,
    #[serde(rename = "SessionInactivityDuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_inactivity_duration: Option<i32>,
    #[serde(rename = "SessionInactivityHandlingEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_inactivity_handling_enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeContactEvaluationRequest {
    #[serde(rename = "EvaluationId")]
    #[serde(default)]
    pub evaluation_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeContactEvaluationResponse {
    #[serde(rename = "Evaluation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation: Option<Evaluation>,
    #[serde(rename = "EvaluationForm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_form: Option<EvaluationFormContent>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Evaluation {
    #[serde(rename = "Answers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answers: Option<std::collections::HashMap<String, EvaluationAnswerOutput>>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "EvaluationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_arn: Option<String>,
    #[serde(rename = "EvaluationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_id: Option<String>,
    #[serde(rename = "EvaluationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_type: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "Metadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<EvaluationMetadata>,
    #[serde(rename = "Notes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<std::collections::HashMap<String, EvaluationNote>>,
    #[serde(rename = "Scores")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scores: Option<std::collections::HashMap<String, EvaluationScore>>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationAnswerOutput {
    #[serde(rename = "SuggestedAnswers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_answers: Option<Vec<EvaluationSuggestedAnswer>>,
    #[serde(rename = "SystemSuggestedValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_suggested_value: Option<EvaluationAnswerData>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<EvaluationAnswerData>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationSuggestedAnswer {
    #[serde(rename = "AnalysisDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_details: Option<EvaluationQuestionAnswerAnalysisDetails>,
    #[serde(rename = "AnalysisType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_type: Option<String>,
    #[serde(rename = "Input")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<EvaluationQuestionInputDetails>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<EvaluationAnswerData>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationQuestionAnswerAnalysisDetails {
    #[serde(rename = "ContactLens")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_lens: Option<EvaluationContactLensAnswerAnalysisDetails>,
    #[serde(rename = "GenAI")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gen_a_i: Option<EvaluationGenAIAnswerAnalysisDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationContactLensAnswerAnalysisDetails {
    #[serde(rename = "MatchedRuleCategories")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matched_rule_categories: Option<Vec<EvaluationAutomationRuleCategory>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationAutomationRuleCategory {
    #[serde(rename = "Category")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "Condition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[serde(rename = "PointsOfInterest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub points_of_interest: Option<Vec<EvaluationTranscriptPointOfInterest>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationTranscriptPointOfInterest {
    #[serde(rename = "MillisecondOffsets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub millisecond_offsets: Option<EvaluationSuggestedAnswerTranscriptMillisecondOffsets>,
    #[serde(rename = "TranscriptSegment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcript_segment: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationSuggestedAnswerTranscriptMillisecondOffsets {
    #[serde(rename = "BeginOffsetMillis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin_offset_millis: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationGenAIAnswerAnalysisDetails {
    #[serde(rename = "Justification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub justification: Option<String>,
    #[serde(rename = "PointsOfInterest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub points_of_interest: Option<Vec<EvaluationTranscriptPointOfInterest>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationQuestionInputDetails {
    #[serde(rename = "TranscriptType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcript_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationAnswerData {
    #[serde(rename = "DateTimeValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_time_value: Option<String>,
    #[serde(rename = "NotApplicable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_applicable: Option<bool>,
    #[serde(rename = "NumericValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numeric_value: Option<f64>,
    #[serde(rename = "StringValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_value: Option<String>,
    #[serde(rename = "StringValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationMetadata {
    #[serde(rename = "Acknowledgement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acknowledgement: Option<EvaluationAcknowledgement>,
    #[serde(rename = "AutoEvaluation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_evaluation: Option<AutoEvaluationDetails>,
    #[serde(rename = "CalibrationSessionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calibration_session_id: Option<String>,
    #[serde(rename = "ContactAgentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_agent_id: Option<String>,
    #[serde(rename = "ContactId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<String>,
    #[serde(rename = "ContactParticipant")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_participant: Option<EvaluationContactParticipant>,
    #[serde(rename = "EvaluatorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluator_arn: Option<String>,
    #[serde(rename = "Review")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review: Option<EvaluationReviewMetadata>,
    #[serde(rename = "SamplingJobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling_job_id: Option<String>,
    #[serde(rename = "Score")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<EvaluationScore>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationAcknowledgement {
    #[serde(rename = "AcknowledgedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acknowledged_by: Option<String>,
    #[serde(rename = "AcknowledgedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acknowledged_time: Option<f64>,
    #[serde(rename = "AcknowledgerComment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acknowledger_comment: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutoEvaluationDetails {
    #[serde(rename = "AutoEvaluationEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_evaluation_enabled: Option<bool>,
    #[serde(rename = "AutoEvaluationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_evaluation_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationContactParticipant {
    #[serde(rename = "ContactParticipantId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_participant_id: Option<String>,
    #[serde(rename = "ContactParticipantRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_participant_role: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationReviewMetadata {
    #[serde(rename = "CreatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "RequestedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_by: Option<String>,
    #[serde(rename = "RequestedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_time: Option<f64>,
    #[serde(rename = "ReviewId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_id: Option<String>,
    #[serde(rename = "ReviewRequestComments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_request_comments: Option<Vec<EvaluationReviewRequestComment>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationReviewRequestComment {
    #[serde(rename = "Comment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "CreatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationScore {
    #[serde(rename = "AppliedWeight")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applied_weight: Option<f64>,
    #[serde(rename = "AutomaticFail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_fail: Option<bool>,
    #[serde(rename = "NotApplicable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_applicable: Option<bool>,
    #[serde(rename = "Percentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationNote {
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationFormContent {
    #[serde(rename = "AutoEvaluationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_evaluation_configuration: Option<EvaluationFormAutoEvaluationConfiguration>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EvaluationFormArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_form_arn: Option<String>,
    #[serde(rename = "EvaluationFormId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_form_id: Option<String>,
    #[serde(rename = "EvaluationFormVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_form_version: Option<i32>,
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<EvaluationFormItem>>,
    #[serde(rename = "LanguageConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_configuration: Option<EvaluationFormLanguageConfiguration>,
    #[serde(rename = "ReviewConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_configuration: Option<EvaluationReviewConfiguration>,
    #[serde(rename = "ScoringStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scoring_strategy: Option<EvaluationFormScoringStrategy>,
    #[serde(rename = "TargetConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_configuration: Option<EvaluationFormTargetConfiguration>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeContactFlowModuleAliasRequest {
    #[serde(rename = "AliasId")]
    #[serde(default)]
    pub alias_id: String,
    #[serde(rename = "ContactFlowModuleId")]
    #[serde(default)]
    pub contact_flow_module_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeContactFlowModuleAliasResponse {
    #[serde(rename = "ContactFlowModuleAlias")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_flow_module_alias: Option<ContactFlowModuleAliasInfo>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContactFlowModuleAliasInfo {
    #[serde(rename = "AliasId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_id: Option<String>,
    #[serde(rename = "ContactFlowModuleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_flow_module_arn: Option<String>,
    #[serde(rename = "ContactFlowModuleId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_flow_module_id: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "LastModifiedRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_region: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeContactFlowModuleRequest {
    #[serde(rename = "ContactFlowModuleId")]
    #[serde(default)]
    pub contact_flow_module_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeContactFlowModuleResponse {
    #[serde(rename = "ContactFlowModule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_flow_module: Option<ContactFlowModule>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContactFlowModule {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Content")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ExternalInvocationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_invocation_configuration: Option<ExternalInvocationConfiguration>,
    #[serde(rename = "FlowModuleContentSha256")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_module_content_sha256: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Settings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    #[serde(rename = "VersionDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_description: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeContactFlowRequest {
    #[serde(rename = "ContactFlowId")]
    #[serde(default)]
    pub contact_flow_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeContactFlowResponse {
    #[serde(rename = "ContactFlow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_flow: Option<ContactFlow>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContactFlow {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Content")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "FlowContentSha256")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_content_sha256: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LastModifiedRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_region: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    #[serde(rename = "VersionDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_description: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeContactRequest {
    #[serde(rename = "ContactId")]
    #[serde(default)]
    pub contact_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeContactResponse {
    #[serde(rename = "Contact")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<Contact>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Contact {
    #[serde(rename = "AdditionalEmailRecipients")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_email_recipients: Option<AdditionalEmailRecipients>,
    #[serde(rename = "AgentInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_info: Option<AgentInfo>,
    #[serde(rename = "AnsweringMachineDetectionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answering_machine_detection_status: Option<String>,
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Campaign")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign: Option<Campaign>,
    #[serde(rename = "Channel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(rename = "ChatMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_metrics: Option<ChatMetrics>,
    #[serde(rename = "ConnectedToSystemTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_to_system_timestamp: Option<f64>,
    #[serde(rename = "ContactAssociationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_association_id: Option<String>,
    #[serde(rename = "ContactDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_details: Option<ContactDetails>,
    #[serde(rename = "ContactEvaluations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_evaluations: Option<std::collections::HashMap<String, ContactEvaluation>>,
    #[serde(rename = "Customer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Customer>,
    #[serde(rename = "CustomerEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_endpoint: Option<EndpointInfo>,
    #[serde(rename = "CustomerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[serde(rename = "CustomerVoiceActivity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_voice_activity: Option<CustomerVoiceActivity>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "DisconnectDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnect_details: Option<DisconnectDetails>,
    #[serde(rename = "DisconnectReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnect_reason: Option<String>,
    #[serde(rename = "DisconnectTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnect_timestamp: Option<f64>,
    #[serde(rename = "GlobalResiliencyMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_resiliency_metadata: Option<GlobalResiliencyMetadata>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "InitialContactId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_contact_id: Option<String>,
    #[serde(rename = "InitiationMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiation_method: Option<String>,
    #[serde(rename = "InitiationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiation_timestamp: Option<f64>,
    #[serde(rename = "LastPausedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_paused_timestamp: Option<f64>,
    #[serde(rename = "LastResumedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_resumed_timestamp: Option<f64>,
    #[serde(rename = "LastUpdateTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_timestamp: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "NextContacts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_contacts: Option<Vec<NextContactEntry>>,
    #[serde(rename = "OutboundStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_strategy: Option<OutboundStrategy>,
    #[serde(rename = "PreviousContactId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_contact_id: Option<String>,
    #[serde(rename = "QualityMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_metrics: Option<QualityMetrics>,
    #[serde(rename = "QueueInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_info: Option<QueueInfo>,
    #[serde(rename = "QueuePriority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_priority: Option<i64>,
    #[serde(rename = "QueueTimeAdjustmentSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_time_adjustment_seconds: Option<i32>,
    #[serde(rename = "Recordings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recordings: Option<Vec<RecordingInfo>>,
    #[serde(rename = "RelatedContactId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_contact_id: Option<String>,
    #[serde(rename = "RingStartTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ring_start_timestamp: Option<f64>,
    #[serde(rename = "RoutingCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_criteria: Option<RoutingCriteria>,
    #[serde(rename = "ScheduledTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_timestamp: Option<f64>,
    #[serde(rename = "SegmentAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_attributes: Option<std::collections::HashMap<String, SegmentAttributeValue>>,
    #[serde(rename = "SystemEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_endpoint: Option<EndpointInfo>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "TaskTemplateInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_template_info: Option<TaskTemplateInfoV2>,
    #[serde(rename = "TotalPauseCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_pause_count: Option<i32>,
    #[serde(rename = "TotalPauseDurationInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_pause_duration_in_seconds: Option<i32>,
    #[serde(rename = "WisdomInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wisdom_info: Option<WisdomInfo>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdditionalEmailRecipients {
    #[serde(rename = "CcList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc_list: Option<Vec<EmailRecipient>>,
    #[serde(rename = "ToList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_list: Option<Vec<EmailRecipient>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EmailRecipient {
    #[serde(rename = "Address")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "DisplayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AgentInfo {
    #[serde(rename = "AcceptedByAgentTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepted_by_agent_timestamp: Option<f64>,
    #[serde(rename = "AfterContactWorkDuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_contact_work_duration: Option<i32>,
    #[serde(rename = "AfterContactWorkEndTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_contact_work_end_timestamp: Option<f64>,
    #[serde(rename = "AfterContactWorkStartTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_contact_work_start_timestamp: Option<f64>,
    #[serde(rename = "AgentInitiatedHoldDuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_initiated_hold_duration: Option<i32>,
    #[serde(rename = "AgentPauseDurationInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_pause_duration_in_seconds: Option<i32>,
    #[serde(rename = "Capabilities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<ParticipantCapabilities>,
    #[serde(rename = "ConnectedToAgentTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_to_agent_timestamp: Option<f64>,
    #[serde(rename = "DeviceInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_info: Option<DeviceInfo>,
    #[serde(rename = "HierarchyGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hierarchy_groups: Option<HierarchyGroups>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "PreviewEndTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_end_timestamp: Option<f64>,
    #[serde(rename = "StateTransitions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_transitions: Option<Vec<StateTransition>>,
    #[serde(rename = "VoiceEnhancementMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_enhancement_mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeviceInfo {
    #[serde(rename = "OperatingSystem")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<String>,
    #[serde(rename = "PlatformName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_name: Option<String>,
    #[serde(rename = "PlatformVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HierarchyGroups {
    #[serde(rename = "Level1")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level1: Option<AgentHierarchyGroup>,
    #[serde(rename = "Level2")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level2: Option<AgentHierarchyGroup>,
    #[serde(rename = "Level3")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level3: Option<AgentHierarchyGroup>,
    #[serde(rename = "Level4")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level4: Option<AgentHierarchyGroup>,
    #[serde(rename = "Level5")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level5: Option<AgentHierarchyGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AgentHierarchyGroup {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StateTransition {
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "StateEndTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_end_timestamp: Option<f64>,
    #[serde(rename = "StateStartTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_start_timestamp: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChatMetrics {
    #[serde(rename = "AgentMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_metrics: Option<ParticipantMetrics>,
    #[serde(rename = "ChatContactMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_contact_metrics: Option<ChatContactMetrics>,
    #[serde(rename = "CustomerMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_metrics: Option<ParticipantMetrics>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ParticipantMetrics {
    #[serde(rename = "ConversationAbandon")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_abandon: Option<bool>,
    #[serde(rename = "LastMessageTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_message_timestamp: Option<f64>,
    #[serde(rename = "MaxResponseTimeInMillis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_response_time_in_millis: Option<i64>,
    #[serde(rename = "MessageLengthInChars")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_length_in_chars: Option<i32>,
    #[serde(rename = "MessagesSent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages_sent: Option<i32>,
    #[serde(rename = "NumResponses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_responses: Option<i32>,
    #[serde(rename = "ParticipantId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant_id: Option<String>,
    #[serde(rename = "ParticipantType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant_type: Option<String>,
    #[serde(rename = "TotalResponseTimeInMillis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_response_time_in_millis: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChatContactMetrics {
    #[serde(rename = "AgentFirstResponseTimeInMillis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_first_response_time_in_millis: Option<i64>,
    #[serde(rename = "AgentFirstResponseTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_first_response_timestamp: Option<f64>,
    #[serde(rename = "ConversationCloseTimeInMillis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_close_time_in_millis: Option<i64>,
    #[serde(rename = "ConversationTurnCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_turn_count: Option<i32>,
    #[serde(rename = "MultiParty")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_party: Option<bool>,
    #[serde(rename = "TotalBotMessageLengthInChars")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_bot_message_length_in_chars: Option<i32>,
    #[serde(rename = "TotalBotMessages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_bot_messages: Option<i32>,
    #[serde(rename = "TotalMessages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_messages: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContactDetails {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContactEvaluation {
    #[serde(rename = "DeleteTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_timestamp: Option<f64>,
    #[serde(rename = "EndTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_timestamp: Option<f64>,
    #[serde(rename = "EvaluationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_arn: Option<String>,
    #[serde(rename = "ExportLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_location: Option<String>,
    #[serde(rename = "FormId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form_id: Option<String>,
    #[serde(rename = "StartTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_timestamp: Option<f64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Customer {
    #[serde(rename = "Capabilities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<ParticipantCapabilities>,
    #[serde(rename = "DeviceInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_info: Option<DeviceInfo>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EndpointInfo {
    #[serde(rename = "Address")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "DisplayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomerVoiceActivity {
    #[serde(rename = "GreetingEndTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub greeting_end_timestamp: Option<f64>,
    #[serde(rename = "GreetingStartTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub greeting_start_timestamp: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisconnectDetails {
    #[serde(rename = "PotentialDisconnectIssue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub potential_disconnect_issue: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GlobalResiliencyMetadata {
    #[serde(rename = "ActiveRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_region: Option<String>,
    #[serde(rename = "OriginRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_region: Option<String>,
    #[serde(rename = "TrafficDistributionGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_distribution_group_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NextContactEntry {
    #[serde(rename = "NextContactMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_contact_metadata: Option<NextContactMetadata>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NextContactMetadata {
    #[serde(rename = "QuickConnectContactData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quick_connect_contact_data: Option<QuickConnectContactData>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QuickConnectContactData {
    #[serde(rename = "ContactId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<String>,
    #[serde(rename = "InitiationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiation_timestamp: Option<f64>,
    #[serde(rename = "QuickConnectId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quick_connect_id: Option<String>,
    #[serde(rename = "QuickConnectName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quick_connect_name: Option<String>,
    #[serde(rename = "QuickConnectType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quick_connect_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QualityMetrics {
    #[serde(rename = "Agent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent: Option<AgentQualityMetrics>,
    #[serde(rename = "Customer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<CustomerQualityMetrics>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AgentQualityMetrics {
    #[serde(rename = "Audio")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<AudioQualityMetricsInfo>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AudioQualityMetricsInfo {
    #[serde(rename = "PotentialQualityIssues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub potential_quality_issues: Option<Vec<String>>,
    #[serde(rename = "QualityScore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_score: Option<f32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomerQualityMetrics {
    #[serde(rename = "Audio")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<AudioQualityMetricsInfo>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueueInfo {
    #[serde(rename = "EnqueueTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enqueue_timestamp: Option<f64>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecordingInfo {
    #[serde(rename = "DeletionReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_reason: Option<String>,
    #[serde(rename = "FragmentStartNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fragment_start_number: Option<String>,
    #[serde(rename = "FragmentStopNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fragment_stop_number: Option<String>,
    #[serde(rename = "Location")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(rename = "MediaStreamType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_stream_type: Option<String>,
    #[serde(rename = "ParticipantType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant_type: Option<String>,
    #[serde(rename = "StartTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_timestamp: Option<f64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StopTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_timestamp: Option<f64>,
    #[serde(rename = "StorageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    #[serde(rename = "UnprocessedTranscriptLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_transcript_location: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RoutingCriteria {
    #[serde(rename = "ActivationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activation_timestamp: Option<f64>,
    #[serde(rename = "Index")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    #[serde(rename = "Steps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<Vec<Step>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Step {
    #[serde(rename = "Expiry")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry: Option<Expiry>,
    #[serde(rename = "Expression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<Expression>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Expiry {
    #[serde(rename = "DurationInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_in_seconds: Option<i32>,
    #[serde(rename = "ExpiryTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_timestamp: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Expression {
    #[serde(rename = "AndExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and_expression: Option<Vec<Expression>>,
    #[serde(rename = "AttributeCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_condition: Option<AttributeCondition>,
    #[serde(rename = "NotAttributeCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_attribute_condition: Option<AttributeCondition>,
    #[serde(rename = "OrExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub or_expression: Option<Vec<Expression>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttributeCondition {
    #[serde(rename = "ComparisonOperator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparison_operator: Option<String>,
    #[serde(rename = "MatchCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_criteria: Option<MatchCriteria>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ProficiencyLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proficiency_level: Option<f32>,
    #[serde(rename = "Range")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<Range>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MatchCriteria {
    #[serde(rename = "AgentsCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agents_criteria: Option<AgentsCriteria>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AgentsCriteria {
    #[serde(rename = "AgentIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Range {
    #[serde(rename = "MaxProficiencyLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_proficiency_level: Option<f32>,
    #[serde(rename = "MinProficiencyLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_proficiency_level: Option<f32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TaskTemplateInfoV2 {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WisdomInfo {
    #[serde(rename = "AiAgents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ai_agents: Option<Vec<AiAgentInfo>>,
    #[serde(rename = "SessionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AiAgentInfo {
    #[serde(rename = "AiAgentEscalated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ai_agent_escalated: Option<bool>,
    #[serde(rename = "AiAgentVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ai_agent_version_id: Option<String>,
    #[serde(rename = "AiUseCase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ai_use_case: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDataTableAttributeRequest {
    #[serde(rename = "AttributeName")]
    #[serde(default)]
    pub attribute_name: String,
    #[serde(rename = "DataTableId")]
    #[serde(default)]
    pub data_table_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDataTableAttributeResponse {
    #[serde(rename = "Attribute")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute: Option<DataTableAttribute>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataTableAttribute {
    #[serde(rename = "AttributeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_id: Option<String>,
    #[serde(rename = "DataTableArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_table_arn: Option<String>,
    #[serde(rename = "DataTableId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_table_id: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "LastModifiedRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_region: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "LockVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_version: Option<DataTableLockVersion>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Primary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
    #[serde(rename = "Validation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation: Option<Validation>,
    #[serde(rename = "ValueType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_type: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDataTableRequest {
    #[serde(rename = "DataTableId")]
    #[serde(default)]
    pub data_table_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDataTableResponse {
    #[serde(rename = "DataTable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_table: Option<DataTable>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataTable {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LastModifiedRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_region: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "LockVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_version: Option<DataTableLockVersion>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "TimeZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    #[serde(rename = "ValueLockLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_lock_level: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "VersionDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_description: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeEmailAddressRequest {
    #[serde(rename = "EmailAddressId")]
    #[serde(default)]
    pub email_address_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeEmailAddressResponse {
    #[serde(rename = "AliasConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_configurations: Option<Vec<AliasConfiguration>>,
    #[serde(rename = "CreateTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_timestamp: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "DisplayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "EmailAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    #[serde(rename = "EmailAddressArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address_arn: Option<String>,
    #[serde(rename = "EmailAddressId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address_id: Option<String>,
    #[serde(rename = "ModifiedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_timestamp: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeEvaluationFormRequest {
    #[serde(rename = "EvaluationFormId")]
    #[serde(default)]
    pub evaluation_form_id: String,
    #[serde(rename = "EvaluationFormVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_form_version: Option<i32>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeEvaluationFormResponse {
    #[serde(rename = "EvaluationForm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_form: Option<EvaluationForm>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationForm {
    #[serde(rename = "AutoEvaluationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_evaluation_configuration: Option<EvaluationFormAutoEvaluationConfiguration>,
    #[serde(rename = "CreatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EvaluationFormArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_form_arn: Option<String>,
    #[serde(rename = "EvaluationFormId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_form_id: Option<String>,
    #[serde(rename = "EvaluationFormVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_form_version: Option<i32>,
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<EvaluationFormItem>>,
    #[serde(rename = "LanguageConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_configuration: Option<EvaluationFormLanguageConfiguration>,
    #[serde(rename = "LastModifiedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "Locked")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
    #[serde(rename = "ReviewConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_configuration: Option<EvaluationReviewConfiguration>,
    #[serde(rename = "ScoringStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scoring_strategy: Option<EvaluationFormScoringStrategy>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "TargetConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_configuration: Option<EvaluationFormTargetConfiguration>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeHoursOfOperationOverrideRequest {
    #[serde(rename = "HoursOfOperationId")]
    #[serde(default)]
    pub hours_of_operation_id: String,
    #[serde(rename = "HoursOfOperationOverrideId")]
    #[serde(default)]
    pub hours_of_operation_override_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeHoursOfOperationOverrideResponse {
    #[serde(rename = "HoursOfOperationOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hours_of_operation_override: Option<HoursOfOperationOverride>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HoursOfOperationOverride {
    #[serde(rename = "Config")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<Vec<HoursOfOperationOverrideConfig>>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EffectiveFrom")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_from: Option<String>,
    #[serde(rename = "EffectiveTill")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_till: Option<String>,
    #[serde(rename = "HoursOfOperationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hours_of_operation_arn: Option<String>,
    #[serde(rename = "HoursOfOperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hours_of_operation_id: Option<String>,
    #[serde(rename = "HoursOfOperationOverrideId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hours_of_operation_override_id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "OverrideType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_type: Option<String>,
    #[serde(rename = "RecurrenceConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrence_config: Option<RecurrenceConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeHoursOfOperationRequest {
    #[serde(rename = "HoursOfOperationId")]
    #[serde(default)]
    pub hours_of_operation_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeHoursOfOperationResponse {
    #[serde(rename = "HoursOfOperation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hours_of_operation: Option<HoursOfOperation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HoursOfOperation {
    #[serde(rename = "Config")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<Vec<HoursOfOperationConfig>>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "HoursOfOperationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hours_of_operation_arn: Option<String>,
    #[serde(rename = "HoursOfOperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hours_of_operation_id: Option<String>,
    #[serde(rename = "LastModifiedRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_region: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ParentHoursOfOperations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_hours_of_operations: Option<Vec<HoursOfOperationsIdentifier>>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "TimeZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HoursOfOperationsIdentifier {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeInstanceAttributeRequest {
    #[serde(rename = "AttributeType")]
    #[serde(default)]
    pub attribute_type: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeInstanceAttributeResponse {
    #[serde(rename = "Attribute")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute: Option<Attribute>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Attribute {
    #[serde(rename = "AttributeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_type: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeInstanceRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeInstanceResponse {
    #[serde(rename = "Instance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<Instance>,
    #[serde(rename = "ReplicationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_configuration: Option<ReplicationConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Instance {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "IdentityManagementType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_management_type: Option<String>,
    #[serde(rename = "InboundCallsEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_calls_enabled: Option<bool>,
    #[serde(rename = "InstanceAccessUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_access_url: Option<String>,
    #[serde(rename = "InstanceAlias")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_alias: Option<String>,
    #[serde(rename = "InstanceStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_status: Option<String>,
    #[serde(rename = "OutboundCallsEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_calls_enabled: Option<bool>,
    #[serde(rename = "ServiceRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role: Option<String>,
    #[serde(rename = "StatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<InstanceStatusReason>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceStatusReason {
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplicationConfiguration {
    #[serde(rename = "GlobalSignInEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_sign_in_endpoint: Option<String>,
    #[serde(rename = "ReplicationStatusSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_status_summary_list: Option<Vec<ReplicationStatusSummary>>,
    #[serde(rename = "SourceRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_region: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplicationStatusSummary {
    #[serde(rename = "Region")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "ReplicationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_status: Option<String>,
    #[serde(rename = "ReplicationStatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_status_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeInstanceStorageConfigRequest {
    #[serde(rename = "AssociationId")]
    #[serde(default)]
    pub association_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    pub resource_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeInstanceStorageConfigResponse {
    #[serde(rename = "StorageConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_config: Option<InstanceStorageConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeNotificationRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "NotificationId")]
    #[serde(default)]
    pub notification_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeNotificationResponse {
    #[serde(rename = "Notification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification: Option<Notification>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Notification {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Content")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "ExpiresAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<f64>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LastModifiedRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_region: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "Priority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    #[serde(rename = "Recipients")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipients: Option<Vec<String>>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribePhoneNumberRequest {
    #[serde(rename = "PhoneNumberId")]
    #[serde(default)]
    pub phone_number_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribePhoneNumberResponse {
    #[serde(rename = "ClaimedPhoneNumberSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claimed_phone_number_summary: Option<ClaimedPhoneNumberSummary>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClaimedPhoneNumberSummary {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(rename = "PhoneNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(rename = "PhoneNumberArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_arn: Option<String>,
    #[serde(rename = "PhoneNumberCountryCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_country_code: Option<String>,
    #[serde(rename = "PhoneNumberDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_description: Option<String>,
    #[serde(rename = "PhoneNumberId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_id: Option<String>,
    #[serde(rename = "PhoneNumberStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_status: Option<PhoneNumberStatus>,
    #[serde(rename = "PhoneNumberType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_type: Option<String>,
    #[serde(rename = "SourcePhoneNumberArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_phone_number_arn: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "TargetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PhoneNumberStatus {
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
pub struct DescribePredefinedAttributeRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribePredefinedAttributeResponse {
    #[serde(rename = "PredefinedAttribute")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predefined_attribute: Option<PredefinedAttribute>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PredefinedAttribute {
    #[serde(rename = "AttributeConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_configuration: Option<PredefinedAttributeConfiguration>,
    #[serde(rename = "LastModifiedRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_region: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Purposes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purposes: Option<Vec<String>>,
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<PredefinedAttributeValues>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PredefinedAttributeConfiguration {
    #[serde(rename = "EnableValueValidationOnAssociation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_value_validation_on_association: Option<bool>,
    #[serde(rename = "IsReadOnly")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_read_only: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribePromptRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "PromptId")]
    #[serde(default)]
    pub prompt_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribePromptResponse {
    #[serde(rename = "Prompt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<Prompt>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Prompt {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "LastModifiedRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_region: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "PromptARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_a_r_n: Option<String>,
    #[serde(rename = "PromptId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_id: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeQueueRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "QueueId")]
    #[serde(default)]
    pub queue_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeQueueResponse {
    #[serde(rename = "Queue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue: Option<Queue>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Queue {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "HoursOfOperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hours_of_operation_id: Option<String>,
    #[serde(rename = "LastModifiedRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_region: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "MaxContacts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_contacts: Option<i32>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "OutboundCallerConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_caller_config: Option<OutboundCallerConfig>,
    #[serde(rename = "OutboundEmailConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_email_config: Option<OutboundEmailConfig>,
    #[serde(rename = "QueueArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_arn: Option<String>,
    #[serde(rename = "QueueId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeQuickConnectRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "QuickConnectId")]
    #[serde(default)]
    pub quick_connect_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeQuickConnectResponse {
    #[serde(rename = "QuickConnect")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quick_connect: Option<QuickConnect>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QuickConnect {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "LastModifiedRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_region: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "QuickConnectARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quick_connect_a_r_n: Option<String>,
    #[serde(rename = "QuickConnectConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quick_connect_config: Option<QuickConnectConfig>,
    #[serde(rename = "QuickConnectId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quick_connect_id: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRoutingProfileRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "RoutingProfileId")]
    #[serde(default)]
    pub routing_profile_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRoutingProfileResponse {
    #[serde(rename = "RoutingProfile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_profile: Option<RoutingProfile>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RoutingProfile {
    #[serde(rename = "AgentAvailabilityTimer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_availability_timer: Option<String>,
    #[serde(rename = "AssociatedManualAssignmentQueueIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_manual_assignment_queue_ids: Option<Vec<String>>,
    #[serde(rename = "AssociatedQueueIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_queue_ids: Option<Vec<String>>,
    #[serde(rename = "DefaultOutboundQueueId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_outbound_queue_id: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(rename = "IsDefault")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[serde(rename = "LastModifiedRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_region: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "MediaConcurrencies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_concurrencies: Option<Vec<MediaConcurrency>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "NumberOfAssociatedManualAssignmentQueues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_associated_manual_assignment_queues: Option<i64>,
    #[serde(rename = "NumberOfAssociatedQueues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_associated_queues: Option<i64>,
    #[serde(rename = "NumberOfAssociatedUsers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_associated_users: Option<i64>,
    #[serde(rename = "RoutingProfileArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_profile_arn: Option<String>,
    #[serde(rename = "RoutingProfileId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_profile_id: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRuleRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "RuleId")]
    #[serde(default)]
    pub rule_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRuleResponse {
    #[serde(rename = "Rule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule: Option<Rule>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Rule {
    #[serde(rename = "Actions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<RuleAction>>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "Function")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function: Option<String>,
    #[serde(rename = "LastUpdatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_by: Option<String>,
    #[serde(rename = "LastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "PublishStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_status: Option<String>,
    #[serde(rename = "RuleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_arn: Option<String>,
    #[serde(rename = "RuleId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "TriggerEventSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_event_source: Option<RuleTriggerEventSource>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSecurityProfileRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "SecurityProfileId")]
    #[serde(default)]
    pub security_profile_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSecurityProfileResponse {
    #[serde(rename = "SecurityProfile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_profile: Option<SecurityProfile>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SecurityProfile {
    #[serde(rename = "AllowedAccessControlHierarchyGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_access_control_hierarchy_group_id: Option<String>,
    #[serde(rename = "AllowedAccessControlTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_access_control_tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "GranularAccessControlConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granular_access_control_configuration: Option<GranularAccessControlConfiguration>,
    #[serde(rename = "HierarchyRestrictedResources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hierarchy_restricted_resources: Option<Vec<String>>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LastModifiedRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_region: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "OrganizationResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_resource_id: Option<String>,
    #[serde(rename = "SecurityProfileName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_profile_name: Option<String>,
    #[serde(rename = "TagRestrictedResources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_restricted_resources: Option<Vec<String>>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTestCaseRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TestCaseId")]
    #[serde(default)]
    pub test_case_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTestCaseResponse {
    #[serde(rename = "TestCase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_case: Option<TestCase>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TestCase {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Content")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EntryPoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_point: Option<TestCaseEntryPoint>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "InitializationData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initialization_data: Option<String>,
    #[serde(rename = "LastModifiedRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_region: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "TestCaseSha256")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_case_sha256: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTrafficDistributionGroupRequest {
    #[serde(rename = "TrafficDistributionGroupId")]
    #[serde(default)]
    pub traffic_distribution_group_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTrafficDistributionGroupResponse {
    #[serde(rename = "TrafficDistributionGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_distribution_group: Option<TrafficDistributionGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TrafficDistributionGroup {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "InstanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_arn: Option<String>,
    #[serde(rename = "IsDefault")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeUserHierarchyGroupRequest {
    #[serde(rename = "HierarchyGroupId")]
    #[serde(default)]
    pub hierarchy_group_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeUserHierarchyGroupResponse {
    #[serde(rename = "HierarchyGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hierarchy_group: Option<HierarchyGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HierarchyGroup {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "HierarchyPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hierarchy_path: Option<HierarchyPath>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LastModifiedRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_region: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "LevelId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HierarchyPath {
    #[serde(rename = "LevelFive")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_five: Option<HierarchyGroupSummary>,
    #[serde(rename = "LevelFour")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_four: Option<HierarchyGroupSummary>,
    #[serde(rename = "LevelOne")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_one: Option<HierarchyGroupSummary>,
    #[serde(rename = "LevelThree")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_three: Option<HierarchyGroupSummary>,
    #[serde(rename = "LevelTwo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_two: Option<HierarchyGroupSummary>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HierarchyGroupSummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LastModifiedRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_region: Option<String>,
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
pub struct DescribeUserHierarchyStructureRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeUserHierarchyStructureResponse {
    #[serde(rename = "HierarchyStructure")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hierarchy_structure: Option<HierarchyStructure>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HierarchyStructure {
    #[serde(rename = "LevelFive")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_five: Option<HierarchyLevel>,
    #[serde(rename = "LevelFour")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_four: Option<HierarchyLevel>,
    #[serde(rename = "LevelOne")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_one: Option<HierarchyLevel>,
    #[serde(rename = "LevelThree")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_three: Option<HierarchyLevel>,
    #[serde(rename = "LevelTwo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_two: Option<HierarchyLevel>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HierarchyLevel {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LastModifiedRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_region: Option<String>,
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
pub struct DescribeUserRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "UserId")]
    #[serde(default)]
    pub user_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeUserResponse {
    #[serde(rename = "User")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct User {
    #[serde(rename = "AfterContactWorkConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_contact_work_configs: Option<Vec<AfterContactWorkConfigPerChannel>>,
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "AutoAcceptConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_accept_configs: Option<Vec<AutoAcceptConfig>>,
    #[serde(rename = "DirectoryUserId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_user_id: Option<String>,
    #[serde(rename = "HierarchyGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hierarchy_group_id: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "IdentityInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_info: Option<UserIdentityInfo>,
    #[serde(rename = "LastModifiedRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_region: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "PersistentConnectionConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent_connection_configs: Option<Vec<PersistentConnectionConfig>>,
    #[serde(rename = "PhoneConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_config: Option<UserPhoneConfig>,
    #[serde(rename = "PhoneNumberConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_configs: Option<Vec<PhoneNumberConfig>>,
    #[serde(rename = "RoutingProfileId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_profile_id: Option<String>,
    #[serde(rename = "SecurityProfileIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_profile_ids: Option<Vec<String>>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Username")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(rename = "VoiceEnhancementConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_enhancement_configs: Option<Vec<VoiceEnhancementConfig>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeViewRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "ViewId")]
    #[serde(default)]
    pub view_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeViewResponse {
    #[serde(rename = "View")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view: Option<View>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeVocabularyRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "VocabularyId")]
    #[serde(default)]
    pub vocabulary_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeVocabularyResponse {
    #[serde(rename = "Vocabulary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary: Option<Vocabulary>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Vocabulary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Content")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(rename = "FailureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeWorkspaceRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "WorkspaceId")]
    #[serde(default)]
    pub workspace_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeWorkspaceResponse {
    #[serde(rename = "Workspace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<Workspace>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Workspace {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LastModifiedRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_region: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Theme")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme: Option<WorkspaceTheme>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "Visibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateAnalyticsDataSetRequest {
    #[serde(rename = "DataSetId")]
    #[serde(default)]
    pub data_set_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "TargetAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_account_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateApprovedOriginRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Origin")]
    #[serde(default)]
    pub origin: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateBotRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "LexBot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lex_bot: Option<LexBot>,
    #[serde(rename = "LexV2Bot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lex_v2_bot: Option<LexV2Bot>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateEmailAddressAliasRequest {
    #[serde(rename = "AliasConfiguration")]
    #[serde(default)]
    pub alias_configuration: AliasConfiguration,
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "EmailAddressId")]
    #[serde(default)]
    pub email_address_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateEmailAddressAliasResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateFlowRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    pub resource_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateFlowResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateHoursOfOperationsRequest {
    #[serde(rename = "HoursOfOperationId")]
    #[serde(default)]
    pub hours_of_operation_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "ParentHoursOfOperationIds")]
    #[serde(default)]
    pub parent_hours_of_operation_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateInstanceStorageConfigRequest {
    #[serde(rename = "AssociationId")]
    #[serde(default)]
    pub association_id: String,
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    pub resource_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateLambdaFunctionRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "FunctionArn")]
    #[serde(default)]
    pub function_arn: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateLexBotRequest {
    #[serde(rename = "BotName")]
    #[serde(default)]
    pub bot_name: String,
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "LexRegion")]
    #[serde(default)]
    pub lex_region: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociatePhoneNumberContactFlowRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "PhoneNumberId")]
    #[serde(default)]
    pub phone_number_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateQueueEmailAddressesRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "EmailAddressesId")]
    #[serde(default)]
    pub email_addresses_id: Vec<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "QueueId")]
    #[serde(default)]
    pub queue_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateQueueQuickConnectsRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "QueueId")]
    #[serde(default)]
    pub queue_id: String,
    #[serde(rename = "QuickConnectIds")]
    #[serde(default)]
    pub quick_connect_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateRoutingProfileQueuesRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "ManualAssignmentQueueReferences")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_assignment_queue_references: Option<Vec<RoutingProfileQueueReference>>,
    #[serde(rename = "QueueReferences")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_references: Option<Vec<RoutingProfileQueueReference>>,
    #[serde(rename = "RoutingProfileId")]
    #[serde(default)]
    pub routing_profile_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateSecurityKeyRequest {
    #[serde(rename = "AssociationId")]
    #[serde(default)]
    pub association_id: String,
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateSecurityProfilesRequest {
    #[serde(rename = "EntityArn")]
    #[serde(default)]
    pub entity_arn: String,
    #[serde(rename = "EntityType")]
    #[serde(default)]
    pub entity_type: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "SecurityProfiles")]
    #[serde(default)]
    pub security_profiles: Vec<SecurityProfileItem>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateTrafficDistributionGroupUserRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "TrafficDistributionGroupId")]
    #[serde(default)]
    pub traffic_distribution_group_id: String,
    #[serde(rename = "UserId")]
    #[serde(default)]
    pub user_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateTrafficDistributionGroupUserResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateUserProficienciesRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "UserId")]
    #[serde(default)]
    pub user_id: String,
    #[serde(rename = "UserProficiencies")]
    #[serde(default)]
    pub user_proficiencies: Vec<UserProficiencyDisassociate>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserProficiencyDisassociate {
    #[serde(rename = "AttributeName")]
    #[serde(default)]
    pub attribute_name: String,
    #[serde(rename = "AttributeValue")]
    #[serde(default)]
    pub attribute_value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateWorkspaceRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "ResourceArns")]
    #[serde(default)]
    pub resource_arns: Vec<String>,
    #[serde(rename = "WorkspaceId")]
    #[serde(default)]
    pub workspace_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateWorkspaceResponse {
    #[serde(rename = "FailedList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_list: Option<Vec<FailedBatchAssociationSummary>>,
    #[serde(rename = "SuccessfulList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful_list: Option<Vec<SuccessfulBatchAssociationSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DismissUserContactRequest {
    #[serde(rename = "ContactId")]
    #[serde(default)]
    pub contact_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "UserId")]
    #[serde(default)]
    pub user_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DismissUserContactResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluateDataTableValuesRequest {
    #[serde(rename = "DataTableId")]
    #[serde(default)]
    pub data_table_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TimeZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    #[serde(rename = "Values")]
    #[serde(default)]
    pub values: Vec<DataTableValueEvaluationSet>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataTableValueEvaluationSet {
    #[serde(rename = "AttributeNames")]
    #[serde(default)]
    pub attribute_names: Vec<String>,
    #[serde(rename = "PrimaryValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_values: Option<Vec<PrimaryValue>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluateDataTableValuesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<DataTableEvaluatedValue>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataTableEvaluatedValue {
    #[serde(rename = "AttributeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_name: Option<String>,
    #[serde(rename = "Error")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<bool>,
    #[serde(rename = "EvaluatedValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluated_value: Option<String>,
    #[serde(rename = "Found")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub found: Option<bool>,
    #[serde(rename = "PrimaryValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_values: Option<Vec<PrimaryValue>>,
    #[serde(rename = "RecordId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_id: Option<String>,
    #[serde(rename = "ValueType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAttachedFileRequest {
    #[serde(rename = "AssociatedResourceArn")]
    #[serde(default)]
    pub associated_resource_arn: String,
    #[serde(rename = "FileId")]
    #[serde(default)]
    pub file_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "UrlExpiryInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_expiry_in_seconds: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAttachedFileResponse {
    #[serde(rename = "AssociatedResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_resource_arn: Option<String>,
    #[serde(rename = "CreatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<CreatedByInfo>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(rename = "DownloadUrlMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_url_metadata: Option<DownloadUrlMetadata>,
    #[serde(rename = "FileArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_arn: Option<String>,
    #[serde(rename = "FileId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    #[serde(rename = "FileName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(rename = "FileSizeInBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size_in_bytes: Option<i64>,
    #[serde(rename = "FileStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_status: Option<String>,
    #[serde(rename = "FileUseCaseType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_use_case_type: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DownloadUrlMetadata {
    #[serde(rename = "Url")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "UrlExpiry")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_expiry: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetContactAttributesRequest {
    #[serde(rename = "InitialContactId")]
    #[serde(default)]
    pub initial_contact_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetContactAttributesResponse {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetContactMetricsRequest {
    #[serde(rename = "ContactId")]
    #[serde(default)]
    pub contact_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Metrics")]
    #[serde(default)]
    pub metrics: Vec<ContactMetricInfo>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContactMetricInfo {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetContactMetricsResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "MetricResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_results: Option<Vec<ContactMetricResult>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContactMetricResult {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<ContactMetricValue>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContactMetricValue {
    #[serde(rename = "Number")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCurrentMetricDataRequest {
    #[serde(rename = "CurrentMetrics")]
    #[serde(default)]
    pub current_metrics: Vec<CurrentMetric>,
    #[serde(rename = "Filters")]
    #[serde(default)]
    pub filters: Filters,
    #[serde(rename = "Groupings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groupings: Option<Vec<String>>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SortCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_criteria: Option<Vec<CurrentMetricSortCriteria>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CurrentMetric {
    #[serde(rename = "MetricId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Unit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Filters {
    #[serde(rename = "AgentStatuses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_statuses: Option<Vec<String>>,
    #[serde(rename = "Channels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<String>>,
    #[serde(rename = "Queues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queues: Option<Vec<String>>,
    #[serde(rename = "RoutingProfiles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_profiles: Option<Vec<String>>,
    #[serde(rename = "RoutingStepExpressions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_step_expressions: Option<Vec<String>>,
    #[serde(rename = "Subtypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtypes: Option<Vec<String>>,
    #[serde(rename = "ValidationTestTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_test_types: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CurrentMetricSortCriteria {
    #[serde(rename = "SortByMetric")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by_metric: Option<String>,
    #[serde(rename = "SortOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCurrentMetricDataResponse {
    #[serde(rename = "ApproximateTotalCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_total_count: Option<i64>,
    #[serde(rename = "DataSnapshotTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_snapshot_time: Option<f64>,
    #[serde(rename = "MetricResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_results: Option<Vec<CurrentMetricResult>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CurrentMetricResult {
    #[serde(rename = "Collections")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collections: Option<Vec<CurrentMetricData>>,
    #[serde(rename = "Dimensions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Dimensions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CurrentMetricData {
    #[serde(rename = "Metric")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric: Option<CurrentMetric>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Dimensions {
    #[serde(rename = "AgentStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_status: Option<AgentStatusIdentifier>,
    #[serde(rename = "Channel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(rename = "Queue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue: Option<QueueReference>,
    #[serde(rename = "RoutingProfile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_profile: Option<RoutingProfileReference>,
    #[serde(rename = "RoutingStepExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_step_expression: Option<String>,
    #[serde(rename = "Subtype")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtype: Option<String>,
    #[serde(rename = "ValidationTestType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_test_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AgentStatusIdentifier {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueueReference {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RoutingProfileReference {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCurrentUserDataRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    pub filters: UserDataFilters,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
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
pub struct UserDataFilters {
    #[serde(rename = "Agents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agents: Option<Vec<String>>,
    #[serde(rename = "ContactFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_filter: Option<ContactFilter>,
    #[serde(rename = "Queues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queues: Option<Vec<String>>,
    #[serde(rename = "RoutingProfiles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_profiles: Option<Vec<String>>,
    #[serde(rename = "UserHierarchyGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_hierarchy_groups: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContactFilter {
    #[serde(rename = "ContactStates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_states: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCurrentUserDataResponse {
    #[serde(rename = "ApproximateTotalCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_total_count: Option<i64>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "UserDataList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_data_list: Option<Vec<UserData>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserData {
    #[serde(rename = "ActiveSlotsByChannel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_slots_by_channel: Option<std::collections::HashMap<String, i32>>,
    #[serde(rename = "AvailableSlotsByChannel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_slots_by_channel: Option<std::collections::HashMap<String, i32>>,
    #[serde(rename = "Contacts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contacts: Option<Vec<AgentContactReference>>,
    #[serde(rename = "HierarchyPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hierarchy_path: Option<HierarchyPathReference>,
    #[serde(rename = "MaxSlotsByChannel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_slots_by_channel: Option<std::collections::HashMap<String, i32>>,
    #[serde(rename = "NextStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_status: Option<String>,
    #[serde(rename = "RoutingProfile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_profile: Option<RoutingProfileReference>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<AgentStatusReference>,
    #[serde(rename = "User")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserReference>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AgentContactReference {
    #[serde(rename = "AgentContactState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_contact_state: Option<String>,
    #[serde(rename = "Channel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(rename = "ConnectedToAgentTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_to_agent_timestamp: Option<f64>,
    #[serde(rename = "ContactId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<String>,
    #[serde(rename = "InitiationMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiation_method: Option<String>,
    #[serde(rename = "Queue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue: Option<QueueReference>,
    #[serde(rename = "StateStartTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_start_timestamp: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HierarchyPathReference {
    #[serde(rename = "LevelFive")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_five: Option<HierarchyGroupSummaryReference>,
    #[serde(rename = "LevelFour")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_four: Option<HierarchyGroupSummaryReference>,
    #[serde(rename = "LevelOne")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_one: Option<HierarchyGroupSummaryReference>,
    #[serde(rename = "LevelThree")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_three: Option<HierarchyGroupSummaryReference>,
    #[serde(rename = "LevelTwo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_two: Option<HierarchyGroupSummaryReference>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HierarchyGroupSummaryReference {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AgentStatusReference {
    #[serde(rename = "StatusArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_arn: Option<String>,
    #[serde(rename = "StatusName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_name: Option<String>,
    #[serde(rename = "StatusStartTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_start_timestamp: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserReference {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetEffectiveHoursOfOperationsRequest {
    #[serde(rename = "FromDate")]
    #[serde(default)]
    pub from_date: String,
    #[serde(rename = "HoursOfOperationId")]
    #[serde(default)]
    pub hours_of_operation_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "ToDate")]
    #[serde(default)]
    pub to_date: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetEffectiveHoursOfOperationsResponse {
    #[serde(rename = "EffectiveHoursOfOperationList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_hours_of_operation_list: Option<Vec<EffectiveHoursOfOperations>>,
    #[serde(rename = "EffectiveOverrideHoursList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_override_hours_list: Option<Vec<EffectiveOverrideHours>>,
    #[serde(rename = "TimeZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EffectiveHoursOfOperations {
    #[serde(rename = "Date")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(rename = "OperationalHours")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operational_hours: Option<Vec<OperationalHour>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OperationalHour {
    #[serde(rename = "End")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<OverrideTimeSlice>,
    #[serde(rename = "Start")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<OverrideTimeSlice>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EffectiveOverrideHours {
    #[serde(rename = "Date")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(rename = "OverrideHours")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_hours: Option<Vec<OverrideHour>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OverrideHour {
    #[serde(rename = "End")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<OverrideTimeSlice>,
    #[serde(rename = "OperationalStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operational_status: Option<String>,
    #[serde(rename = "OverrideName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_name: Option<String>,
    #[serde(rename = "Start")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<OverrideTimeSlice>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFederationTokenRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFederationTokenResponse {
    #[serde(rename = "Credentials")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<Credentials>,
    #[serde(rename = "SignInUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sign_in_url: Option<String>,
    #[serde(rename = "UserArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_arn: Option<String>,
    #[serde(rename = "UserId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Credentials {
    #[serde(rename = "AccessToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[serde(rename = "AccessTokenExpiration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token_expiration: Option<f64>,
    #[serde(rename = "RefreshToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    #[serde(rename = "RefreshTokenExpiration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token_expiration: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFlowAssociationRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    pub resource_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFlowAssociationResponse {
    #[serde(rename = "FlowId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_id: Option<String>,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMetricDataRequest {
    #[serde(rename = "EndTime")]
    #[serde(default)]
    pub end_time: f64,
    #[serde(rename = "Filters")]
    #[serde(default)]
    pub filters: Filters,
    #[serde(rename = "Groupings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groupings: Option<Vec<String>>,
    #[serde(rename = "HistoricalMetrics")]
    #[serde(default)]
    pub historical_metrics: Vec<HistoricalMetric>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    pub start_time: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HistoricalMetric {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Statistic")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistic: Option<String>,
    #[serde(rename = "Threshold")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<Threshold>,
    #[serde(rename = "Unit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Threshold {
    #[serde(rename = "Comparison")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparison: Option<String>,
    #[serde(rename = "ThresholdValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold_value: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMetricDataResponse {
    #[serde(rename = "MetricResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_results: Option<Vec<HistoricalMetricResult>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HistoricalMetricResult {
    #[serde(rename = "Collections")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collections: Option<Vec<HistoricalMetricData>>,
    #[serde(rename = "Dimensions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Dimensions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HistoricalMetricData {
    #[serde(rename = "Metric")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric: Option<HistoricalMetric>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMetricDataV2Request {
    #[serde(rename = "EndTime")]
    #[serde(default)]
    pub end_time: f64,
    #[serde(rename = "Filters")]
    #[serde(default)]
    pub filters: Vec<FilterV2>,
    #[serde(rename = "Groupings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groupings: Option<Vec<String>>,
    #[serde(rename = "Interval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<IntervalDetails>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "Metrics")]
    #[serde(default)]
    pub metrics: Vec<MetricV2>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    pub start_time: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FilterV2 {
    #[serde(rename = "FilterKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_key: Option<String>,
    #[serde(rename = "FilterValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_values: Option<Vec<String>>,
    #[serde(rename = "StringCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_condition: Option<FilterV2StringCondition>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FilterV2StringCondition {
    #[serde(rename = "Comparison")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparison: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IntervalDetails {
    #[serde(rename = "IntervalPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_period: Option<String>,
    #[serde(rename = "TimeZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MetricV2 {
    #[serde(rename = "MetricFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_filters: Option<Vec<MetricFilterV2>>,
    #[serde(rename = "MetricId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Threshold")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<Vec<ThresholdV2>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MetricFilterV2 {
    #[serde(rename = "MetricFilterKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_filter_key: Option<String>,
    #[serde(rename = "MetricFilterValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_filter_values: Option<Vec<String>>,
    #[serde(rename = "Negate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negate: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ThresholdV2 {
    #[serde(rename = "Comparison")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparison: Option<String>,
    #[serde(rename = "ThresholdValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold_value: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMetricDataV2Response {
    #[serde(rename = "MetricResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_results: Option<Vec<MetricResultV2>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MetricResultV2 {
    #[serde(rename = "Collections")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collections: Option<Vec<MetricDataV2>>,
    #[serde(rename = "Dimensions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "MetricInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_interval: Option<MetricInterval>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MetricDataV2 {
    #[serde(rename = "Metric")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric: Option<MetricV2>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MetricInterval {
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "Interval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPromptFileRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "PromptId")]
    #[serde(default)]
    pub prompt_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPromptFileResponse {
    #[serde(rename = "LastModifiedRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_region: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "PromptPresignedUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_presigned_url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTaskTemplateRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "SnapshotVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_version: Option<String>,
    #[serde(rename = "TaskTemplateId")]
    #[serde(default)]
    pub task_template_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTaskTemplateResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Constraints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraints: Option<TaskTemplateConstraints>,
    #[serde(rename = "ContactFlowId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_flow_id: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "Defaults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defaults: Option<TaskTemplateDefaults>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Fields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<TaskTemplateField>>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "SelfAssignFlowId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_assign_flow_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTestCaseExecutionSummaryRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "TestCaseExecutionId")]
    #[serde(default)]
    pub test_case_execution_id: String,
    #[serde(rename = "TestCaseId")]
    #[serde(default)]
    pub test_case_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTestCaseExecutionSummaryResponse {
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "ObservationSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observation_summary: Option<ObservationSummary>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ObservationSummary {
    #[serde(rename = "ObservationsFailed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observations_failed: Option<i32>,
    #[serde(rename = "ObservationsPassed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observations_passed: Option<i32>,
    #[serde(rename = "TotalObservations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_observations: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTrafficDistributionRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTrafficDistributionResponse {
    #[serde(rename = "AgentConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_config: Option<AgentConfig>,
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "SignInConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sign_in_config: Option<SignInConfig>,
    #[serde(rename = "TelephonyConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telephony_config: Option<TelephonyConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AgentConfig {
    #[serde(rename = "Distributions")]
    #[serde(default)]
    pub distributions: Vec<Distribution>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Distribution {
    #[serde(rename = "Percentage")]
    #[serde(default)]
    pub percentage: i32,
    #[serde(rename = "Region")]
    #[serde(default)]
    pub region: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SignInConfig {
    #[serde(rename = "Distributions")]
    #[serde(default)]
    pub distributions: Vec<SignInDistribution>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SignInDistribution {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    pub enabled: bool,
    #[serde(rename = "Region")]
    #[serde(default)]
    pub region: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TelephonyConfig {
    #[serde(rename = "Distributions")]
    #[serde(default)]
    pub distributions: Vec<Distribution>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportPhoneNumberRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "PhoneNumberDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_description: Option<String>,
    #[serde(rename = "SourcePhoneNumberArn")]
    #[serde(default)]
    pub source_phone_number_arn: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportPhoneNumberResponse {
    #[serde(rename = "PhoneNumberArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_arn: Option<String>,
    #[serde(rename = "PhoneNumberId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportWorkspaceMediaRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "MediaSource")]
    #[serde(default)]
    pub media_source: String,
    #[serde(rename = "MediaType")]
    #[serde(default)]
    pub media_type: String,
    #[serde(rename = "WorkspaceId")]
    #[serde(default)]
    pub workspace_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportWorkspaceMediaResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAgentStatusRequest {
    #[serde(rename = "AgentStatusTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_status_types: Option<Vec<String>>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
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
pub struct ListAgentStatusResponse {
    #[serde(rename = "AgentStatusSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_status_summary_list: Option<Vec<AgentStatusSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AgentStatusSummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LastModifiedRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_region: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAnalyticsDataAssociationsRequest {
    #[serde(rename = "DataSetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_id: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
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
pub struct ListAnalyticsDataAssociationsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Results")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<AnalyticsDataAssociationResult>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAnalyticsDataLakeDataSetsRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
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
pub struct ListAnalyticsDataLakeDataSetsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Results")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<AnalyticsDataSetsResult>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnalyticsDataSetsResult {
    #[serde(rename = "DataSetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_id: Option<String>,
    #[serde(rename = "DataSetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListApprovedOriginsRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
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
pub struct ListApprovedOriginsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Origins")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origins: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAssociatedContactsRequest {
    #[serde(rename = "ContactId")]
    #[serde(default)]
    pub contact_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
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
pub struct ListAssociatedContactsResponse {
    #[serde(rename = "ContactSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_summary_list: Option<Vec<AssociatedContactSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociatedContactSummary {
    #[serde(rename = "Channel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(rename = "ContactArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_arn: Option<String>,
    #[serde(rename = "ContactId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<String>,
    #[serde(rename = "DisconnectTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnect_timestamp: Option<f64>,
    #[serde(rename = "InitialContactId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_contact_id: Option<String>,
    #[serde(rename = "InitiationMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiation_method: Option<String>,
    #[serde(rename = "InitiationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiation_timestamp: Option<f64>,
    #[serde(rename = "PreviousContactId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_contact_id: Option<String>,
    #[serde(rename = "RelatedContactId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_contact_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAttachedFilesConfigurationsRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
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
pub struct ListAttachedFilesConfigurationsResponse {
    #[serde(rename = "AttachedFilesConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attached_files_configurations: Option<Vec<AttachedFilesConfigurationSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttachedFilesConfigurationSummary {
    #[serde(rename = "AttachmentScope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_scope: Option<String>,
    #[serde(rename = "ExtensionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_configuration: Option<ExtensionConfiguration>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(rename = "MaximumSizeLimitInBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_size_limit_in_bytes: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAuthenticationProfilesRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
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
pub struct ListAuthenticationProfilesResponse {
    #[serde(rename = "AuthenticationProfileSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_profile_summary_list: Option<Vec<AuthenticationProfileSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuthenticationProfileSummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "IsDefault")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[serde(rename = "LastModifiedRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_region: Option<String>,
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
pub struct ListBotsRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "LexVersion")]
    #[serde(default)]
    pub lex_version: String,
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
pub struct ListBotsResponse {
    #[serde(rename = "LexBots")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lex_bots: Option<Vec<LexBotConfig>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LexBotConfig {
    #[serde(rename = "LexBot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lex_bot: Option<LexBot>,
    #[serde(rename = "LexV2Bot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lex_v2_bot: Option<LexV2Bot>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListChildHoursOfOperationsRequest {
    #[serde(rename = "HoursOfOperationId")]
    #[serde(default)]
    pub hours_of_operation_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
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
pub struct ListChildHoursOfOperationsResponse {
    #[serde(rename = "ChildHoursOfOperationsSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_hours_of_operations_summary_list: Option<Vec<HoursOfOperationsIdentifier>>,
    #[serde(rename = "LastModifiedRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_region: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListContactEvaluationsRequest {
    #[serde(rename = "ContactId")]
    #[serde(default)]
    pub contact_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListContactEvaluationsResponse {
    #[serde(rename = "EvaluationSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_summary_list: Option<Vec<EvaluationSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationSummary {
    #[serde(rename = "Acknowledgement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acknowledgement: Option<EvaluationAcknowledgementSummary>,
    #[serde(rename = "AutoEvaluationEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_evaluation_enabled: Option<bool>,
    #[serde(rename = "AutoEvaluationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_evaluation_status: Option<String>,
    #[serde(rename = "CalibrationSessionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calibration_session_id: Option<String>,
    #[serde(rename = "ContactParticipant")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_participant: Option<EvaluationContactParticipant>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "EvaluationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_arn: Option<String>,
    #[serde(rename = "EvaluationFormId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_form_id: Option<String>,
    #[serde(rename = "EvaluationFormTitle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_form_title: Option<String>,
    #[serde(rename = "EvaluationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_id: Option<String>,
    #[serde(rename = "EvaluationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_type: Option<String>,
    #[serde(rename = "EvaluatorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluator_arn: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "Score")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<EvaluationScore>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationAcknowledgementSummary {
    #[serde(rename = "AcknowledgedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acknowledged_by: Option<String>,
    #[serde(rename = "AcknowledgedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acknowledged_time: Option<f64>,
    #[serde(rename = "AcknowledgerComment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acknowledger_comment: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListContactFlowModuleAliasesRequest {
    #[serde(rename = "ContactFlowModuleId")]
    #[serde(default)]
    pub contact_flow_module_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
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
pub struct ListContactFlowModuleAliasesResponse {
    #[serde(rename = "ContactFlowModuleAliasSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_flow_module_alias_summary_list: Option<Vec<ContactFlowModuleAliasSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContactFlowModuleAliasSummary {
    #[serde(rename = "AliasDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_description: Option<String>,
    #[serde(rename = "AliasId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_id: Option<String>,
    #[serde(rename = "AliasName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_name: Option<String>,
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListContactFlowModuleVersionsRequest {
    #[serde(rename = "ContactFlowModuleId")]
    #[serde(default)]
    pub contact_flow_module_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
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
pub struct ListContactFlowModuleVersionsResponse {
    #[serde(rename = "ContactFlowModuleVersionSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_flow_module_version_summary_list: Option<Vec<ContactFlowModuleVersionSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContactFlowModuleVersionSummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    #[serde(rename = "VersionDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_description: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListContactFlowModulesRequest {
    #[serde(rename = "ContactFlowModuleState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_flow_module_state: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
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
pub struct ListContactFlowModulesResponse {
    #[serde(rename = "ContactFlowModulesSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_flow_modules_summary_list: Option<Vec<ContactFlowModuleSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContactFlowModuleSummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListContactFlowVersionsRequest {
    #[serde(rename = "ContactFlowId")]
    #[serde(default)]
    pub contact_flow_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
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
pub struct ListContactFlowVersionsResponse {
    #[serde(rename = "ContactFlowVersionSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_flow_version_summary_list: Option<Vec<ContactFlowVersionSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContactFlowVersionSummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    #[serde(rename = "VersionDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_description: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListContactFlowsRequest {
    #[serde(rename = "ContactFlowTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_flow_types: Option<Vec<String>>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
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
pub struct ListContactFlowsResponse {
    #[serde(rename = "ContactFlowSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_flow_summary_list: Option<Vec<ContactFlowSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContactFlowSummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "ContactFlowState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_flow_state: Option<String>,
    #[serde(rename = "ContactFlowStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_flow_status: Option<String>,
    #[serde(rename = "ContactFlowType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_flow_type: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListContactReferencesRequest {
    #[serde(rename = "ContactId")]
    #[serde(default)]
    pub contact_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ReferenceTypes")]
    #[serde(default)]
    pub reference_types: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListContactReferencesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ReferenceSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_summary_list: Option<Vec<ReferenceSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReferenceSummary {
    #[serde(rename = "Attachment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment: Option<AttachmentReference>,
    #[serde(rename = "Date")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<DateReference>,
    #[serde(rename = "Email")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<EmailReference>,
    #[serde(rename = "EmailMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_message: Option<EmailMessageReference>,
    #[serde(rename = "EmailMessagePlainText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_message_plain_text: Option<EmailMessageReference>,
    #[serde(rename = "EmailMessagePlainTextRedacted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_message_plain_text_redacted: Option<EmailMessageReference>,
    #[serde(rename = "EmailMessageRedacted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_message_redacted: Option<EmailMessageReference>,
    #[serde(rename = "Number")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<NumberReference>,
    #[serde(rename = "String")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string: Option<StringReference>,
    #[serde(rename = "Url")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<UrlReference>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttachmentReference {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DateReference {
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
pub struct EmailReference {
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
pub struct EmailMessageReference {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NumberReference {
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
pub struct StringReference {
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
pub struct UrlReference {
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
pub struct ListDataTableAttributesRequest {
    #[serde(rename = "AttributeIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_ids: Option<Vec<String>>,
    #[serde(rename = "DataTableId")]
    #[serde(default)]
    pub data_table_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
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
pub struct ListDataTableAttributesResponse {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<DataTableAttribute>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDataTablePrimaryValuesRequest {
    #[serde(rename = "DataTableId")]
    #[serde(default)]
    pub data_table_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PrimaryAttributeValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_attribute_values: Option<Vec<PrimaryAttributeValueFilter>>,
    #[serde(rename = "RecordIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PrimaryAttributeValueFilter {
    #[serde(rename = "AttributeName")]
    #[serde(default)]
    pub attribute_name: String,
    #[serde(rename = "Values")]
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDataTablePrimaryValuesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PrimaryValuesList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_values_list: Option<Vec<RecordPrimaryValue>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecordPrimaryValue {
    #[serde(rename = "LastModifiedRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_region: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "PrimaryValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_values: Option<Vec<PrimaryValueResponse>>,
    #[serde(rename = "RecordId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDataTableValuesRequest {
    #[serde(rename = "DataTableId")]
    #[serde(default)]
    pub data_table_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PrimaryAttributeValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_attribute_values: Option<Vec<PrimaryAttributeValueFilter>>,
    #[serde(rename = "RecordIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDataTableValuesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<DataTableValueSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataTableValueSummary {
    #[serde(rename = "AttributeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_id: Option<String>,
    #[serde(rename = "AttributeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_name: Option<String>,
    #[serde(rename = "LastModifiedRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_region: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "LockVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_version: Option<DataTableLockVersion>,
    #[serde(rename = "PrimaryValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_values: Option<Vec<PrimaryValueResponse>>,
    #[serde(rename = "RecordId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_id: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "ValueType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDataTablesRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
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
pub struct ListDataTablesResponse {
    #[serde(rename = "DataTableSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_table_summary_list: Option<Vec<DataTableSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataTableSummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LastModifiedRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_region: Option<String>,
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
pub struct ListDefaultVocabulariesRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
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
pub struct ListDefaultVocabulariesResponse {
    #[serde(rename = "DefaultVocabularyList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_vocabulary_list: Option<Vec<DefaultVocabulary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DefaultVocabulary {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(rename = "VocabularyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_id: Option<String>,
    #[serde(rename = "VocabularyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListEntitySecurityProfilesRequest {
    #[serde(rename = "EntityArn")]
    #[serde(default)]
    pub entity_arn: String,
    #[serde(rename = "EntityType")]
    #[serde(default)]
    pub entity_type: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
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
pub struct ListEntitySecurityProfilesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SecurityProfiles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_profiles: Option<Vec<SecurityProfileItem>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListEvaluationFormVersionsRequest {
    #[serde(rename = "EvaluationFormId")]
    #[serde(default)]
    pub evaluation_form_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
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
pub struct ListEvaluationFormVersionsResponse {
    #[serde(rename = "EvaluationFormVersionSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_form_version_summary_list: Option<Vec<EvaluationFormVersionSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationFormVersionSummary {
    #[serde(rename = "CreatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "EvaluationFormArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_form_arn: Option<String>,
    #[serde(rename = "EvaluationFormId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_form_id: Option<String>,
    #[serde(rename = "EvaluationFormVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_form_version: Option<i32>,
    #[serde(rename = "LastModifiedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "Locked")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListEvaluationFormsRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
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
pub struct ListEvaluationFormsResponse {
    #[serde(rename = "EvaluationFormSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_form_summary_list: Option<Vec<EvaluationFormSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationFormSummary {
    #[serde(rename = "ActiveVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_version: Option<i32>,
    #[serde(rename = "CreatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "EvaluationFormArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_form_arn: Option<String>,
    #[serde(rename = "EvaluationFormId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_form_id: Option<String>,
    #[serde(rename = "LastActivatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_activated_by: Option<String>,
    #[serde(rename = "LastActivatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_activated_time: Option<f64>,
    #[serde(rename = "LastModifiedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "LatestVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<i32>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFlowAssociationsRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFlowAssociationsResponse {
    #[serde(rename = "FlowAssociationSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_association_summary_list: Option<Vec<FlowAssociationSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListHoursOfOperationOverridesRequest {
    #[serde(rename = "HoursOfOperationId")]
    #[serde(default)]
    pub hours_of_operation_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
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
pub struct ListHoursOfOperationOverridesResponse {
    #[serde(rename = "HoursOfOperationOverrideList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hours_of_operation_override_list: Option<Vec<HoursOfOperationOverride>>,
    #[serde(rename = "LastModifiedRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_region: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListHoursOfOperationsRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
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
pub struct ListHoursOfOperationsResponse {
    #[serde(rename = "HoursOfOperationSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hours_of_operation_summary_list: Option<Vec<HoursOfOperationSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HoursOfOperationSummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LastModifiedRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_region: Option<String>,
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
pub struct ListInstanceAttributesRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
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
pub struct ListInstanceAttributesResponse {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<Attribute>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListInstanceStorageConfigsRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    pub resource_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListInstanceStorageConfigsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StorageConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_configs: Option<Vec<InstanceStorageConfig>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListInstancesRequest {
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
pub struct ListInstancesResponse {
    #[serde(rename = "InstanceSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_summary_list: Option<Vec<InstanceSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceSummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "IdentityManagementType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_management_type: Option<String>,
    #[serde(rename = "InboundCallsEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_calls_enabled: Option<bool>,
    #[serde(rename = "InstanceAccessUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_access_url: Option<String>,
    #[serde(rename = "InstanceAlias")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_alias: Option<String>,
    #[serde(rename = "InstanceStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_status: Option<String>,
    #[serde(rename = "OutboundCallsEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_calls_enabled: Option<bool>,
    #[serde(rename = "ServiceRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListIntegrationAssociationsRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "IntegrationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_arn: Option<String>,
    #[serde(rename = "IntegrationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_type: Option<String>,
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
pub struct ListIntegrationAssociationsResponse {
    #[serde(rename = "IntegrationAssociationSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_association_summary_list: Option<Vec<IntegrationAssociationSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IntegrationAssociationSummary {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(rename = "IntegrationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_arn: Option<String>,
    #[serde(rename = "IntegrationAssociationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_association_arn: Option<String>,
    #[serde(rename = "IntegrationAssociationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_association_id: Option<String>,
    #[serde(rename = "IntegrationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_type: Option<String>,
    #[serde(rename = "SourceApplicationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_application_name: Option<String>,
    #[serde(rename = "SourceApplicationUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_application_url: Option<String>,
    #[serde(rename = "SourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListLambdaFunctionsRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
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
pub struct ListLambdaFunctionsResponse {
    #[serde(rename = "LambdaFunctions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_functions: Option<Vec<String>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListLexBotsRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
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
pub struct ListLexBotsResponse {
    #[serde(rename = "LexBots")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lex_bots: Option<Vec<LexBot>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListNotificationsRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
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
pub struct ListNotificationsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "NotificationSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_summary_list: Option<Vec<Notification>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPhoneNumbersRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PhoneNumberCountryCodes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_country_codes: Option<Vec<String>>,
    #[serde(rename = "PhoneNumberTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_types: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPhoneNumbersResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PhoneNumberSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_summary_list: Option<Vec<PhoneNumberSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PhoneNumberSummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "PhoneNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(rename = "PhoneNumberCountryCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_country_code: Option<String>,
    #[serde(rename = "PhoneNumberType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPhoneNumbersV2Request {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PhoneNumberCountryCodes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_country_codes: Option<Vec<String>>,
    #[serde(rename = "PhoneNumberPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_prefix: Option<String>,
    #[serde(rename = "PhoneNumberTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_types: Option<Vec<String>>,
    #[serde(rename = "TargetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPhoneNumbersV2Response {
    #[serde(rename = "ListPhoneNumbersSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_phone_numbers_summary_list: Option<Vec<ListPhoneNumbersSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPhoneNumbersSummary {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(rename = "PhoneNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(rename = "PhoneNumberArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_arn: Option<String>,
    #[serde(rename = "PhoneNumberCountryCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_country_code: Option<String>,
    #[serde(rename = "PhoneNumberDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_description: Option<String>,
    #[serde(rename = "PhoneNumberId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_id: Option<String>,
    #[serde(rename = "PhoneNumberType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_type: Option<String>,
    #[serde(rename = "SourcePhoneNumberArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_phone_number_arn: Option<String>,
    #[serde(rename = "TargetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPredefinedAttributesRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
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
pub struct ListPredefinedAttributesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PredefinedAttributeSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predefined_attribute_summary_list: Option<Vec<PredefinedAttributeSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PredefinedAttributeSummary {
    #[serde(rename = "LastModifiedRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_region: Option<String>,
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
pub struct ListPromptsRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
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
pub struct ListPromptsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PromptSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_summary_list: Option<Vec<PromptSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PromptSummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LastModifiedRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_region: Option<String>,
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
pub struct ListQueueEmailAddressesRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "QueueId")]
    #[serde(default)]
    pub queue_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListQueueEmailAddressesResponse {
    #[serde(rename = "EmailAddressMetadataList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address_metadata_list: Option<Vec<EmailAddressSummary>>,
    #[serde(rename = "LastModifiedRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_region: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EmailAddressSummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "IsDefaultOutboundEmail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default_outbound_email: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListQueueQuickConnectsRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "QueueId")]
    #[serde(default)]
    pub queue_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListQueueQuickConnectsResponse {
    #[serde(rename = "LastModifiedRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_region: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "QuickConnectSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quick_connect_summary_list: Option<Vec<QuickConnectSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QuickConnectSummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LastModifiedRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_region: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "QuickConnectType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quick_connect_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListQueuesRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "QueueTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_types: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListQueuesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "QueueSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_summary_list: Option<Vec<QueueSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueueSummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LastModifiedRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_region: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "QueueType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListQuickConnectsRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "QuickConnectTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quick_connect_types: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListQuickConnectsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "QuickConnectSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quick_connect_summary_list: Option<Vec<QuickConnectSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRealtimeContactAnalysisSegmentsV2Request {
    #[serde(rename = "ContactId")]
    #[serde(default)]
    pub contact_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "OutputType")]
    #[serde(default)]
    pub output_type: String,
    #[serde(rename = "SegmentTypes")]
    #[serde(default)]
    pub segment_types: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRealtimeContactAnalysisSegmentsV2Response {
    #[serde(rename = "Channel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Segments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segments: Option<Vec<RealtimeContactAnalysisSegment>>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RealtimeContactAnalysisSegment {
    #[serde(rename = "Attachments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<RealTimeContactAnalysisSegmentAttachments>,
    #[serde(rename = "Categories")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<RealTimeContactAnalysisSegmentCategories>,
    #[serde(rename = "Event")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<RealTimeContactAnalysisSegmentEvent>,
    #[serde(rename = "Issues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issues: Option<RealTimeContactAnalysisSegmentIssues>,
    #[serde(rename = "PostContactSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_contact_summary: Option<RealTimeContactAnalysisSegmentPostContactSummary>,
    #[serde(rename = "Transcript")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcript: Option<RealTimeContactAnalysisSegmentTranscript>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RealTimeContactAnalysisSegmentAttachments {
    #[serde(rename = "Attachments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<RealTimeContactAnalysisAttachment>>,
    #[serde(rename = "DisplayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "ParticipantId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant_id: Option<String>,
    #[serde(rename = "ParticipantRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant_role: Option<String>,
    #[serde(rename = "Time")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<RealTimeContactAnalysisTimeData>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RealTimeContactAnalysisAttachment {
    #[serde(rename = "AttachmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_id: Option<String>,
    #[serde(rename = "AttachmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_name: Option<String>,
    #[serde(rename = "ContentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RealTimeContactAnalysisTimeData {
    #[serde(rename = "AbsoluteTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub absolute_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RealTimeContactAnalysisSegmentCategories {
    #[serde(rename = "MatchedDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matched_details:
        Option<std::collections::HashMap<String, RealTimeContactAnalysisCategoryDetails>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RealTimeContactAnalysisCategoryDetails {
    #[serde(rename = "PointsOfInterest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub points_of_interest: Option<Vec<RealTimeContactAnalysisPointOfInterest>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RealTimeContactAnalysisPointOfInterest {
    #[serde(rename = "TranscriptItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcript_items: Option<Vec<RealTimeContactAnalysisTranscriptItemWithCharacterOffsets>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RealTimeContactAnalysisTranscriptItemWithCharacterOffsets {
    #[serde(rename = "CharacterOffsets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_offsets: Option<RealTimeContactAnalysisCharacterInterval>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RealTimeContactAnalysisCharacterInterval {
    #[serde(rename = "BeginOffsetChar")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin_offset_char: Option<i32>,
    #[serde(rename = "EndOffsetChar")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_offset_char: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RealTimeContactAnalysisSegmentEvent {
    #[serde(rename = "DisplayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "EventType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "ParticipantId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant_id: Option<String>,
    #[serde(rename = "ParticipantRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant_role: Option<String>,
    #[serde(rename = "Time")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<RealTimeContactAnalysisTimeData>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RealTimeContactAnalysisSegmentIssues {
    #[serde(rename = "IssuesDetected")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issues_detected: Option<Vec<RealTimeContactAnalysisIssueDetected>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RealTimeContactAnalysisIssueDetected {
    #[serde(rename = "TranscriptItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcript_items: Option<Vec<RealTimeContactAnalysisTranscriptItemWithContent>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RealTimeContactAnalysisTranscriptItemWithContent {
    #[serde(rename = "CharacterOffsets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_offsets: Option<RealTimeContactAnalysisCharacterInterval>,
    #[serde(rename = "Content")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RealTimeContactAnalysisSegmentPostContactSummary {
    #[serde(rename = "Content")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(rename = "FailureCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RealTimeContactAnalysisSegmentTranscript {
    #[serde(rename = "Content")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(rename = "ContentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(rename = "DisplayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "ParticipantId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant_id: Option<String>,
    #[serde(rename = "ParticipantRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant_role: Option<String>,
    #[serde(rename = "Redaction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redaction: Option<RealTimeContactAnalysisTranscriptItemRedaction>,
    #[serde(rename = "Sentiment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sentiment: Option<String>,
    #[serde(rename = "Time")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<RealTimeContactAnalysisTimeData>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RealTimeContactAnalysisTranscriptItemRedaction {
    #[serde(rename = "CharacterOffsets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_offsets: Option<Vec<RealTimeContactAnalysisCharacterInterval>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRoutingProfileManualAssignmentQueuesRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RoutingProfileId")]
    #[serde(default)]
    pub routing_profile_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRoutingProfileManualAssignmentQueuesResponse {
    #[serde(rename = "LastModifiedRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_region: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RoutingProfileManualAssignmentQueueConfigSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_profile_manual_assignment_queue_config_summary_list:
        Option<Vec<RoutingProfileManualAssignmentQueueConfigSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RoutingProfileManualAssignmentQueueConfigSummary {
    #[serde(rename = "Channel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(rename = "QueueArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_arn: Option<String>,
    #[serde(rename = "QueueId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_id: Option<String>,
    #[serde(rename = "QueueName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRoutingProfileQueuesRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RoutingProfileId")]
    #[serde(default)]
    pub routing_profile_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRoutingProfileQueuesResponse {
    #[serde(rename = "LastModifiedRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_region: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RoutingProfileQueueConfigSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_profile_queue_config_summary_list: Option<Vec<RoutingProfileQueueConfigSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RoutingProfileQueueConfigSummary {
    #[serde(rename = "Channel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(rename = "Delay")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay: Option<i32>,
    #[serde(rename = "Priority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(rename = "QueueArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_arn: Option<String>,
    #[serde(rename = "QueueId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_id: Option<String>,
    #[serde(rename = "QueueName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRoutingProfilesRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
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
pub struct ListRoutingProfilesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RoutingProfileSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_profile_summary_list: Option<Vec<RoutingProfileSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RoutingProfileSummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LastModifiedRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_region: Option<String>,
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
pub struct ListRulesRequest {
    #[serde(rename = "EventSourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_source_name: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PublishStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRulesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RuleSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_summary_list: Option<Vec<RuleSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuleSummary {
    #[serde(rename = "ActionSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_summaries: Option<Vec<ActionSummary>>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "EventSourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_source_name: Option<String>,
    #[serde(rename = "LastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "PublishStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_status: Option<String>,
    #[serde(rename = "RuleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_arn: Option<String>,
    #[serde(rename = "RuleId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActionSummary {
    #[serde(rename = "ActionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSecurityKeysRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
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
pub struct ListSecurityKeysResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SecurityKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_keys: Option<Vec<SecurityKey>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SecurityKey {
    #[serde(rename = "AssociationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_id: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSecurityProfileApplicationsRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SecurityProfileId")]
    #[serde(default)]
    pub security_profile_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSecurityProfileApplicationsResponse {
    #[serde(rename = "Applications")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applications: Option<Vec<Application>>,
    #[serde(rename = "LastModifiedRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_region: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSecurityProfileFlowModulesRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SecurityProfileId")]
    #[serde(default)]
    pub security_profile_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSecurityProfileFlowModulesResponse {
    #[serde(rename = "AllowedFlowModules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_flow_modules: Option<Vec<FlowModule>>,
    #[serde(rename = "LastModifiedRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_region: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSecurityProfilePermissionsRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SecurityProfileId")]
    #[serde(default)]
    pub security_profile_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSecurityProfilePermissionsResponse {
    #[serde(rename = "LastModifiedRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_region: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Permissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSecurityProfilesRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
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
pub struct ListSecurityProfilesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SecurityProfileSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_profile_summary_list: Option<Vec<SecurityProfileSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SecurityProfileSummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LastModifiedRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_region: Option<String>,
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
pub struct ListTaskTemplatesRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTaskTemplatesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TaskTemplates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_templates: Option<Vec<TaskTemplateMetadata>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TaskTemplateMetadata {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
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
pub struct ListTestCaseExecutionRecordsRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TestCaseExecutionId")]
    #[serde(default)]
    pub test_case_execution_id: String,
    #[serde(rename = "TestCaseId")]
    #[serde(default)]
    pub test_case_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTestCaseExecutionRecordsResponse {
    #[serde(rename = "ExecutionRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_records: Option<Vec<ExecutionRecord>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExecutionRecord {
    #[serde(rename = "ObservationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observation_id: Option<String>,
    #[serde(rename = "Record")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Timestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTestCaseExecutionsRequest {
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TestCaseId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_case_id: Option<String>,
    #[serde(rename = "TestCaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_case_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTestCaseExecutionsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TestCaseExecutions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_case_executions: Option<Vec<TestCaseExecution>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TestCaseExecution {
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "TestCaseExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_case_execution_id: Option<String>,
    #[serde(rename = "TestCaseExecutionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_case_execution_status: Option<String>,
    #[serde(rename = "TestCaseId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_case_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTestCasesRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
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
pub struct ListTestCasesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TestCaseSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_case_summary_list: Option<Vec<TestCaseSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TestCaseSummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LastModifiedRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_region: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
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
pub struct ListTrafficDistributionGroupUsersRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TrafficDistributionGroupId")]
    #[serde(default)]
    pub traffic_distribution_group_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTrafficDistributionGroupUsersResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TrafficDistributionGroupUserSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_distribution_group_user_summary_list:
        Option<Vec<TrafficDistributionGroupUserSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TrafficDistributionGroupUserSummary {
    #[serde(rename = "UserId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTrafficDistributionGroupsRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
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
pub struct ListTrafficDistributionGroupsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TrafficDistributionGroupSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_distribution_group_summary_list: Option<Vec<TrafficDistributionGroupSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TrafficDistributionGroupSummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "InstanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_arn: Option<String>,
    #[serde(rename = "IsDefault")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
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
pub struct ListUseCasesRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "IntegrationAssociationId")]
    #[serde(default)]
    pub integration_association_id: String,
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
pub struct ListUseCasesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "UseCaseSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_case_summary_list: Option<Vec<UseCase>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UseCase {
    #[serde(rename = "UseCaseArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_case_arn: Option<String>,
    #[serde(rename = "UseCaseId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_case_id: Option<String>,
    #[serde(rename = "UseCaseType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_case_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListUserHierarchyGroupsRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
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
pub struct ListUserHierarchyGroupsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "UserHierarchyGroupSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_hierarchy_group_summary_list: Option<Vec<HierarchyGroupSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListUserNotificationsRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "UserId")]
    #[serde(default)]
    pub user_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListUserNotificationsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "UserNotifications")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_notifications: Option<Vec<UserNotificationSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserNotificationSummary {
    #[serde(rename = "Content")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "ExpiresAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<f64>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(rename = "NotificationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_id: Option<String>,
    #[serde(rename = "NotificationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_status: Option<String>,
    #[serde(rename = "Priority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    #[serde(rename = "RecipientId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_id: Option<String>,
    #[serde(rename = "Source")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListUserProficienciesRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "UserId")]
    #[serde(default)]
    pub user_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListUserProficienciesResponse {
    #[serde(rename = "LastModifiedRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_region: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "UserProficiencyList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_proficiency_list: Option<Vec<UserProficiency>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListUsersRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
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
pub struct ListUsersResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "UserSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_summary_list: Option<Vec<UserSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserSummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LastModifiedRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_region: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "Username")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListViewVersionsRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ViewId")]
    #[serde(default)]
    pub view_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListViewVersionsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ViewVersionSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_version_summary_list: Option<Vec<ViewVersionSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ViewVersionSummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
    #[serde(rename = "VersionDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_description: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListViewsRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListViewsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ViewsSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub views_summary_list: Option<Vec<ViewSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ViewSummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListWorkspaceMediaRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "WorkspaceId")]
    #[serde(default)]
    pub workspace_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListWorkspaceMediaResponse {
    #[serde(rename = "Media")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<Vec<MediaItem>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MediaItem {
    #[serde(rename = "Source")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListWorkspacePagesRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "WorkspaceId")]
    #[serde(default)]
    pub workspace_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListWorkspacePagesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "WorkspacePageList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_page_list: Option<Vec<WorkspacePage>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkspacePage {
    #[serde(rename = "InputData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_data: Option<String>,
    #[serde(rename = "Page")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "Slug")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListWorkspacesRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
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
pub struct ListWorkspacesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "WorkspaceSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_summary_list: Option<Vec<WorkspaceSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkspaceSummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LastModifiedRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_region: Option<String>,
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
pub struct MonitorContactRequest {
    #[serde(rename = "AllowedMonitorCapabilities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_monitor_capabilities: Option<Vec<String>>,
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "ContactId")]
    #[serde(default)]
    pub contact_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "UserId")]
    #[serde(default)]
    pub user_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MonitorContactResponse {
    #[serde(rename = "ContactArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_arn: Option<String>,
    #[serde(rename = "ContactId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PauseContactRequest {
    #[serde(rename = "ContactFlowId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_flow_id: Option<String>,
    #[serde(rename = "ContactId")]
    #[serde(default)]
    pub contact_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PauseContactResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutUserStatusRequest {
    #[serde(rename = "AgentStatusId")]
    #[serde(default)]
    pub agent_status_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "UserId")]
    #[serde(default)]
    pub user_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutUserStatusResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReleasePhoneNumberRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "PhoneNumberId")]
    #[serde(default)]
    pub phone_number_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplicateInstanceRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "ReplicaAlias")]
    #[serde(default)]
    pub replica_alias: String,
    #[serde(rename = "ReplicaRegion")]
    #[serde(default)]
    pub replica_region: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplicateInstanceResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResumeContactRecordingRequest {
    #[serde(rename = "ContactId")]
    #[serde(default)]
    pub contact_id: String,
    #[serde(rename = "ContactRecordingType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_recording_type: Option<String>,
    #[serde(rename = "InitialContactId")]
    #[serde(default)]
    pub initial_contact_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResumeContactRecordingResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResumeContactRequest {
    #[serde(rename = "ContactFlowId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_flow_id: Option<String>,
    #[serde(rename = "ContactId")]
    #[serde(default)]
    pub contact_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResumeContactResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchAgentStatusesRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SearchCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_criteria: Option<AgentStatusSearchCriteria>,
    #[serde(rename = "SearchFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_filter: Option<AgentStatusSearchFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AgentStatusSearchCriteria {
    #[serde(rename = "AndConditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and_conditions: Option<Vec<AgentStatusSearchCriteria>>,
    #[serde(rename = "OrConditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub or_conditions: Option<Vec<AgentStatusSearchCriteria>>,
    #[serde(rename = "StringCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_condition: Option<StringCondition>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StringCondition {
    #[serde(rename = "ComparisonType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparison_type: Option<String>,
    #[serde(rename = "FieldName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AgentStatusSearchFilter {
    #[serde(rename = "AttributeFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_filter: Option<ControlPlaneAttributeFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ControlPlaneAttributeFilter {
    #[serde(rename = "AndCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and_condition: Option<CommonAttributeAndCondition>,
    #[serde(rename = "OrConditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub or_conditions: Option<Vec<CommonAttributeAndCondition>>,
    #[serde(rename = "TagCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_condition: Option<TagCondition>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CommonAttributeAndCondition {
    #[serde(rename = "TagConditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_conditions: Option<Vec<TagCondition>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagCondition {
    #[serde(rename = "TagKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<String>,
    #[serde(rename = "TagValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchAgentStatusesResponse {
    #[serde(rename = "AgentStatuses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_statuses: Option<Vec<AgentStatus>>,
    #[serde(rename = "ApproximateTotalCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_total_count: Option<i64>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchAvailablePhoneNumbersRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PhoneNumberCountryCode")]
    #[serde(default)]
    pub phone_number_country_code: String,
    #[serde(rename = "PhoneNumberPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_prefix: Option<String>,
    #[serde(rename = "PhoneNumberType")]
    #[serde(default)]
    pub phone_number_type: String,
    #[serde(rename = "TargetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchAvailablePhoneNumbersResponse {
    #[serde(rename = "AvailableNumbersList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_numbers_list: Option<Vec<AvailableNumberSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AvailableNumberSummary {
    #[serde(rename = "PhoneNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(rename = "PhoneNumberCountryCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_country_code: Option<String>,
    #[serde(rename = "PhoneNumberType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchContactEvaluationsRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SearchCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_criteria: Option<EvaluationSearchCriteria>,
    #[serde(rename = "SearchFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_filter: Option<EvaluationSearchFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationSearchCriteria {
    #[serde(rename = "AndConditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and_conditions: Option<Vec<EvaluationSearchCriteria>>,
    #[serde(rename = "BooleanCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boolean_condition: Option<BooleanCondition>,
    #[serde(rename = "DateTimeCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_time_condition: Option<DateTimeCondition>,
    #[serde(rename = "DecimalCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decimal_condition: Option<DecimalCondition>,
    #[serde(rename = "NumberCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_condition: Option<NumberCondition>,
    #[serde(rename = "OrConditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub or_conditions: Option<Vec<EvaluationSearchCriteria>>,
    #[serde(rename = "StringCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_condition: Option<StringCondition>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BooleanCondition {
    #[serde(rename = "ComparisonType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparison_type: Option<String>,
    #[serde(rename = "FieldName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DateTimeCondition {
    #[serde(rename = "ComparisonType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparison_type: Option<String>,
    #[serde(rename = "FieldName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    #[serde(rename = "MaxValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_value: Option<String>,
    #[serde(rename = "MinValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DecimalCondition {
    #[serde(rename = "ComparisonType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparison_type: Option<String>,
    #[serde(rename = "FieldName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    #[serde(rename = "MaxValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_value: Option<f64>,
    #[serde(rename = "MinValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_value: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NumberCondition {
    #[serde(rename = "ComparisonType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparison_type: Option<String>,
    #[serde(rename = "FieldName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    #[serde(rename = "MaxValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_value: Option<i32>,
    #[serde(rename = "MinValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_value: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationSearchFilter {
    #[serde(rename = "AttributeFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_filter: Option<ControlPlaneAttributeFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchContactEvaluationsResponse {
    #[serde(rename = "ApproximateTotalCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_total_count: Option<i64>,
    #[serde(rename = "EvaluationSearchSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_search_summary_list: Option<Vec<EvaluationSearchSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationSearchSummary {
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "EvaluationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_arn: Option<String>,
    #[serde(rename = "EvaluationFormId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_form_id: Option<String>,
    #[serde(rename = "EvaluationFormTitle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_form_title: Option<String>,
    #[serde(rename = "EvaluationFormVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_form_version: Option<i32>,
    #[serde(rename = "EvaluationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_id: Option<String>,
    #[serde(rename = "EvaluationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_type: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "Metadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<EvaluationSearchMetadata>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationSearchMetadata {
    #[serde(rename = "AcknowledgedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acknowledged_by: Option<String>,
    #[serde(rename = "AcknowledgedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acknowledged_time: Option<f64>,
    #[serde(rename = "AcknowledgerComment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acknowledger_comment: Option<String>,
    #[serde(rename = "AutoEvaluationEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_evaluation_enabled: Option<bool>,
    #[serde(rename = "AutoEvaluationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_evaluation_status: Option<String>,
    #[serde(rename = "CalibrationSessionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calibration_session_id: Option<String>,
    #[serde(rename = "ContactAgentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_agent_id: Option<String>,
    #[serde(rename = "ContactId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<String>,
    #[serde(rename = "ContactParticipantId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_participant_id: Option<String>,
    #[serde(rename = "ContactParticipantRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_participant_role: Option<String>,
    #[serde(rename = "EvaluatorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluator_arn: Option<String>,
    #[serde(rename = "ReviewId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_id: Option<String>,
    #[serde(rename = "SamplingJobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling_job_id: Option<String>,
    #[serde(rename = "ScoreAutomaticFail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_automatic_fail: Option<bool>,
    #[serde(rename = "ScoreNotApplicable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_not_applicable: Option<bool>,
    #[serde(rename = "ScorePercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_percentage: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchContactFlowModulesRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SearchCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_criteria: Option<ContactFlowModuleSearchCriteria>,
    #[serde(rename = "SearchFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_filter: Option<ContactFlowModuleSearchFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContactFlowModuleSearchCriteria {
    #[serde(rename = "AndConditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and_conditions: Option<Vec<ContactFlowModuleSearchCriteria>>,
    #[serde(rename = "OrConditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub or_conditions: Option<Vec<ContactFlowModuleSearchCriteria>>,
    #[serde(rename = "StateCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_condition: Option<String>,
    #[serde(rename = "StatusCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_condition: Option<String>,
    #[serde(rename = "StringCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_condition: Option<StringCondition>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContactFlowModuleSearchFilter {
    #[serde(rename = "TagFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_filter: Option<ControlPlaneTagFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ControlPlaneTagFilter {
    #[serde(rename = "AndConditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and_conditions: Option<Vec<TagCondition>>,
    #[serde(rename = "OrConditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub or_conditions: Option<Vec<Vec<TagCondition>>>,
    #[serde(rename = "TagCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_condition: Option<TagCondition>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchContactFlowModulesResponse {
    #[serde(rename = "ApproximateTotalCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_total_count: Option<i64>,
    #[serde(rename = "ContactFlowModules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_flow_modules: Option<Vec<ContactFlowModule>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchContactFlowsRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SearchCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_criteria: Option<ContactFlowSearchCriteria>,
    #[serde(rename = "SearchFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_filter: Option<ContactFlowSearchFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContactFlowSearchCriteria {
    #[serde(rename = "AndConditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and_conditions: Option<Vec<ContactFlowSearchCriteria>>,
    #[serde(rename = "OrConditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub or_conditions: Option<Vec<ContactFlowSearchCriteria>>,
    #[serde(rename = "StateCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_condition: Option<String>,
    #[serde(rename = "StatusCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_condition: Option<String>,
    #[serde(rename = "StringCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_condition: Option<StringCondition>,
    #[serde(rename = "TypeCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_condition: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContactFlowSearchFilter {
    #[serde(rename = "FlowAttributeFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_attribute_filter: Option<ContactFlowAttributeFilter>,
    #[serde(rename = "TagFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_filter: Option<ControlPlaneTagFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContactFlowAttributeFilter {
    #[serde(rename = "AndCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and_condition: Option<ContactFlowAttributeAndCondition>,
    #[serde(rename = "ContactFlowTypeCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_flow_type_condition: Option<ContactFlowTypeCondition>,
    #[serde(rename = "OrConditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub or_conditions: Option<Vec<ContactFlowAttributeAndCondition>>,
    #[serde(rename = "TagCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_condition: Option<TagCondition>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContactFlowAttributeAndCondition {
    #[serde(rename = "ContactFlowTypeCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_flow_type_condition: Option<ContactFlowTypeCondition>,
    #[serde(rename = "TagConditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_conditions: Option<Vec<TagCondition>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContactFlowTypeCondition {
    #[serde(rename = "ContactFlowType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_flow_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchContactFlowsResponse {
    #[serde(rename = "ApproximateTotalCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_total_count: Option<i64>,
    #[serde(rename = "ContactFlows")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_flows: Option<Vec<ContactFlow>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchContactsRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SearchCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_criteria: Option<SearchCriteria>,
    #[serde(rename = "Sort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Sort>,
    #[serde(rename = "TimeRange")]
    #[serde(default)]
    pub time_range: SearchContactsTimeRange,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchCriteria {
    #[serde(rename = "ActiveRegions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_regions: Option<Vec<String>>,
    #[serde(rename = "AdditionalTimeRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_time_range: Option<SearchContactsAdditionalTimeRange>,
    #[serde(rename = "AgentHierarchyGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_hierarchy_groups: Option<AgentHierarchyGroups>,
    #[serde(rename = "AgentIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_ids: Option<Vec<String>>,
    #[serde(rename = "Channels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<String>>,
    #[serde(rename = "ContactAnalysis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_analysis: Option<ContactAnalysis>,
    #[serde(rename = "ContactTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_tags: Option<ControlPlaneTagFilter>,
    #[serde(rename = "InitiationMethods")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiation_methods: Option<Vec<String>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<NameCriteria>,
    #[serde(rename = "QueueIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_ids: Option<Vec<String>>,
    #[serde(rename = "RoutingCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_criteria: Option<SearchableRoutingCriteria>,
    #[serde(rename = "SearchableContactAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub searchable_contact_attributes: Option<SearchableContactAttributes>,
    #[serde(rename = "SearchableSegmentAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub searchable_segment_attributes: Option<SearchableSegmentAttributes>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchContactsAdditionalTimeRange {
    #[serde(rename = "Criteria")]
    #[serde(default)]
    pub criteria: Vec<SearchContactsAdditionalTimeRangeCriteria>,
    #[serde(rename = "MatchType")]
    #[serde(default)]
    pub match_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchContactsAdditionalTimeRangeCriteria {
    #[serde(rename = "TimeRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_range: Option<SearchContactsTimeRange>,
    #[serde(rename = "TimestampCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_condition: Option<SearchContactsTimestampCondition>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchContactsTimeRange {
    #[serde(rename = "EndTime")]
    #[serde(default)]
    pub end_time: f64,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    pub start_time: f64,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchContactsTimestampCondition {
    #[serde(rename = "ConditionType")]
    #[serde(default)]
    pub condition_type: String,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AgentHierarchyGroups {
    #[serde(rename = "L1Ids")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub l1_ids: Option<Vec<String>>,
    #[serde(rename = "L2Ids")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub l2_ids: Option<Vec<String>>,
    #[serde(rename = "L3Ids")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub l3_ids: Option<Vec<String>>,
    #[serde(rename = "L4Ids")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub l4_ids: Option<Vec<String>>,
    #[serde(rename = "L5Ids")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub l5_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContactAnalysis {
    #[serde(rename = "Transcript")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcript: Option<Transcript>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Transcript {
    #[serde(rename = "Criteria")]
    #[serde(default)]
    pub criteria: Vec<TranscriptCriteria>,
    #[serde(rename = "MatchType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TranscriptCriteria {
    #[serde(rename = "MatchType")]
    #[serde(default)]
    pub match_type: String,
    #[serde(rename = "ParticipantRole")]
    #[serde(default)]
    pub participant_role: String,
    #[serde(rename = "SearchText")]
    #[serde(default)]
    pub search_text: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NameCriteria {
    #[serde(rename = "MatchType")]
    #[serde(default)]
    pub match_type: String,
    #[serde(rename = "SearchText")]
    #[serde(default)]
    pub search_text: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchableRoutingCriteria {
    #[serde(rename = "Steps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<Vec<SearchableRoutingCriteriaStep>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchableRoutingCriteriaStep {
    #[serde(rename = "AgentCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_criteria: Option<SearchableAgentCriteriaStep>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchableAgentCriteriaStep {
    #[serde(rename = "AgentIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_ids: Option<Vec<String>>,
    #[serde(rename = "MatchType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchableContactAttributes {
    #[serde(rename = "Criteria")]
    #[serde(default)]
    pub criteria: Vec<SearchableContactAttributesCriteria>,
    #[serde(rename = "MatchType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchableContactAttributesCriteria {
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "Values")]
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchableSegmentAttributes {
    #[serde(rename = "Criteria")]
    #[serde(default)]
    pub criteria: Vec<SearchableSegmentAttributesCriteria>,
    #[serde(rename = "MatchType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchableSegmentAttributesCriteria {
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "Values")]
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Sort {
    #[serde(rename = "FieldName")]
    #[serde(default)]
    pub field_name: String,
    #[serde(rename = "Order")]
    #[serde(default)]
    pub order: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchContactsResponse {
    #[serde(rename = "Contacts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contacts: Option<Vec<ContactSearchSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TotalCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContactSearchSummary {
    #[serde(rename = "AgentInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_info: Option<ContactSearchSummaryAgentInfo>,
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Channel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(rename = "DisconnectTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnect_timestamp: Option<f64>,
    #[serde(rename = "GlobalResiliencyMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_resiliency_metadata: Option<GlobalResiliencyMetadata>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "InitialContactId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_contact_id: Option<String>,
    #[serde(rename = "InitiationMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiation_method: Option<String>,
    #[serde(rename = "InitiationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiation_timestamp: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "PreviousContactId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_contact_id: Option<String>,
    #[serde(rename = "QueueInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_info: Option<ContactSearchSummaryQueueInfo>,
    #[serde(rename = "RoutingCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_criteria: Option<RoutingCriteria>,
    #[serde(rename = "ScheduledTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_timestamp: Option<f64>,
    #[serde(rename = "SegmentAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_attributes:
        Option<std::collections::HashMap<String, ContactSearchSummarySegmentAttributeValue>>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContactSearchSummaryAgentInfo {
    #[serde(rename = "ConnectedToAgentTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_to_agent_timestamp: Option<f64>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContactSearchSummaryQueueInfo {
    #[serde(rename = "EnqueueTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enqueue_timestamp: Option<f64>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContactSearchSummarySegmentAttributeValue {
    #[serde(rename = "ValueMap")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_map: Option<std::collections::HashMap<String, SegmentAttributeValue>>,
    #[serde(rename = "ValueString")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_string: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchDataTablesRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SearchCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_criteria: Option<DataTableSearchCriteria>,
    #[serde(rename = "SearchFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_filter: Option<DataTableSearchFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataTableSearchCriteria {
    #[serde(rename = "AndConditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and_conditions: Option<Vec<DataTableSearchCriteria>>,
    #[serde(rename = "OrConditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub or_conditions: Option<Vec<DataTableSearchCriteria>>,
    #[serde(rename = "StringCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_condition: Option<StringCondition>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataTableSearchFilter {
    #[serde(rename = "AttributeFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_filter: Option<ControlPlaneAttributeFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchDataTablesResponse {
    #[serde(rename = "ApproximateTotalCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_total_count: Option<i64>,
    #[serde(rename = "DataTables")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_tables: Option<Vec<DataTable>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchEmailAddressesRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SearchCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_criteria: Option<EmailAddressSearchCriteria>,
    #[serde(rename = "SearchFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_filter: Option<EmailAddressSearchFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EmailAddressSearchCriteria {
    #[serde(rename = "AndConditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and_conditions: Option<Vec<EmailAddressSearchCriteria>>,
    #[serde(rename = "OrConditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub or_conditions: Option<Vec<EmailAddressSearchCriteria>>,
    #[serde(rename = "StringCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_condition: Option<StringCondition>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EmailAddressSearchFilter {
    #[serde(rename = "TagFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_filter: Option<ControlPlaneTagFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchEmailAddressesResponse {
    #[serde(rename = "ApproximateTotalCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_total_count: Option<i64>,
    #[serde(rename = "EmailAddresses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_addresses: Option<Vec<EmailAddressMetadata>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EmailAddressMetadata {
    #[serde(rename = "AliasConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_configurations: Option<Vec<AliasConfiguration>>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "DisplayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "EmailAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    #[serde(rename = "EmailAddressArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address_arn: Option<String>,
    #[serde(rename = "EmailAddressId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchEvaluationFormsRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SearchCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_criteria: Option<EvaluationFormSearchCriteria>,
    #[serde(rename = "SearchFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_filter: Option<EvaluationFormSearchFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationFormSearchCriteria {
    #[serde(rename = "AndConditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and_conditions: Option<Vec<EvaluationFormSearchCriteria>>,
    #[serde(rename = "BooleanCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boolean_condition: Option<BooleanCondition>,
    #[serde(rename = "DateTimeCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_time_condition: Option<DateTimeCondition>,
    #[serde(rename = "NumberCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_condition: Option<NumberCondition>,
    #[serde(rename = "OrConditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub or_conditions: Option<Vec<EvaluationFormSearchCriteria>>,
    #[serde(rename = "StringCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_condition: Option<StringCondition>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationFormSearchFilter {
    #[serde(rename = "AttributeFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_filter: Option<ControlPlaneAttributeFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchEvaluationFormsResponse {
    #[serde(rename = "ApproximateTotalCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_total_count: Option<i64>,
    #[serde(rename = "EvaluationFormSearchSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_form_search_summary_list: Option<Vec<EvaluationFormSearchSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationFormSearchSummary {
    #[serde(rename = "ActiveVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_version: Option<i32>,
    #[serde(rename = "AutoEvaluationEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_evaluation_enabled: Option<bool>,
    #[serde(rename = "ContactInteractionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_interaction_type: Option<String>,
    #[serde(rename = "CreatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EvaluationFormArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_form_arn: Option<String>,
    #[serde(rename = "EvaluationFormId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_form_id: Option<String>,
    #[serde(rename = "EvaluationFormLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_form_language: Option<String>,
    #[serde(rename = "LastActivatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_activated_by: Option<String>,
    #[serde(rename = "LastActivatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_activated_time: Option<f64>,
    #[serde(rename = "LastModifiedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "LatestVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<i32>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchHoursOfOperationOverridesRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SearchCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_criteria: Option<HoursOfOperationOverrideSearchCriteria>,
    #[serde(rename = "SearchFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_filter: Option<HoursOfOperationSearchFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HoursOfOperationOverrideSearchCriteria {
    #[serde(rename = "AndConditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and_conditions: Option<Vec<HoursOfOperationOverrideSearchCriteria>>,
    #[serde(rename = "DateCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_condition: Option<DateCondition>,
    #[serde(rename = "OrConditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub or_conditions: Option<Vec<HoursOfOperationOverrideSearchCriteria>>,
    #[serde(rename = "StringCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_condition: Option<StringCondition>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DateCondition {
    #[serde(rename = "ComparisonType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparison_type: Option<String>,
    #[serde(rename = "FieldName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HoursOfOperationSearchFilter {
    #[serde(rename = "TagFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_filter: Option<ControlPlaneTagFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchHoursOfOperationOverridesResponse {
    #[serde(rename = "ApproximateTotalCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_total_count: Option<i64>,
    #[serde(rename = "HoursOfOperationOverrides")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hours_of_operation_overrides: Option<Vec<HoursOfOperationOverride>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchHoursOfOperationsRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SearchCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_criteria: Option<HoursOfOperationSearchCriteria>,
    #[serde(rename = "SearchFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_filter: Option<HoursOfOperationSearchFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HoursOfOperationSearchCriteria {
    #[serde(rename = "AndConditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and_conditions: Option<Vec<HoursOfOperationSearchCriteria>>,
    #[serde(rename = "OrConditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub or_conditions: Option<Vec<HoursOfOperationSearchCriteria>>,
    #[serde(rename = "StringCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_condition: Option<StringCondition>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchHoursOfOperationsResponse {
    #[serde(rename = "ApproximateTotalCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_total_count: Option<i64>,
    #[serde(rename = "HoursOfOperations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hours_of_operations: Option<Vec<HoursOfOperation>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchNotificationsRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SearchCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_criteria: Option<NotificationSearchCriteria>,
    #[serde(rename = "SearchFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_filter: Option<NotificationSearchFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NotificationSearchCriteria {
    #[serde(rename = "AndConditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and_conditions: Option<Vec<NotificationSearchCriteria>>,
    #[serde(rename = "OrConditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub or_conditions: Option<Vec<NotificationSearchCriteria>>,
    #[serde(rename = "StringCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_condition: Option<StringCondition>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NotificationSearchFilter {
    #[serde(rename = "AttributeFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_filter: Option<ControlPlaneAttributeFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchNotificationsResponse {
    #[serde(rename = "ApproximateTotalCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_total_count: Option<i64>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Notifications")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notifications: Option<Vec<NotificationSearchSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NotificationSearchSummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Content")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "ExpiresAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<f64>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(rename = "LastModifiedRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_region: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "Priority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    #[serde(rename = "Recipients")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipients: Option<Vec<String>>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchPredefinedAttributesRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SearchCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_criteria: Option<PredefinedAttributeSearchCriteria>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PredefinedAttributeSearchCriteria {
    #[serde(rename = "AndConditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and_conditions: Option<Vec<PredefinedAttributeSearchCriteria>>,
    #[serde(rename = "OrConditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub or_conditions: Option<Vec<PredefinedAttributeSearchCriteria>>,
    #[serde(rename = "StringCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_condition: Option<StringCondition>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchPredefinedAttributesResponse {
    #[serde(rename = "ApproximateTotalCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_total_count: Option<i64>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PredefinedAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predefined_attributes: Option<Vec<PredefinedAttribute>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchPromptsRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SearchCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_criteria: Option<PromptSearchCriteria>,
    #[serde(rename = "SearchFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_filter: Option<PromptSearchFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PromptSearchCriteria {
    #[serde(rename = "AndConditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and_conditions: Option<Vec<PromptSearchCriteria>>,
    #[serde(rename = "OrConditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub or_conditions: Option<Vec<PromptSearchCriteria>>,
    #[serde(rename = "StringCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_condition: Option<StringCondition>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PromptSearchFilter {
    #[serde(rename = "TagFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_filter: Option<ControlPlaneTagFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchPromptsResponse {
    #[serde(rename = "ApproximateTotalCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_total_count: Option<i64>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Prompts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompts: Option<Vec<Prompt>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchQueuesRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SearchCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_criteria: Option<QueueSearchCriteria>,
    #[serde(rename = "SearchFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_filter: Option<QueueSearchFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueueSearchCriteria {
    #[serde(rename = "AndConditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and_conditions: Option<Vec<QueueSearchCriteria>>,
    #[serde(rename = "OrConditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub or_conditions: Option<Vec<QueueSearchCriteria>>,
    #[serde(rename = "QueueTypeCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_type_condition: Option<String>,
    #[serde(rename = "StringCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_condition: Option<StringCondition>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueueSearchFilter {
    #[serde(rename = "TagFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_filter: Option<ControlPlaneTagFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchQueuesResponse {
    #[serde(rename = "ApproximateTotalCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_total_count: Option<i64>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Queues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queues: Option<Vec<Queue>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchQuickConnectsRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SearchCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_criteria: Option<QuickConnectSearchCriteria>,
    #[serde(rename = "SearchFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_filter: Option<QuickConnectSearchFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QuickConnectSearchCriteria {
    #[serde(rename = "AndConditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and_conditions: Option<Vec<QuickConnectSearchCriteria>>,
    #[serde(rename = "OrConditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub or_conditions: Option<Vec<QuickConnectSearchCriteria>>,
    #[serde(rename = "StringCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_condition: Option<StringCondition>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QuickConnectSearchFilter {
    #[serde(rename = "TagFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_filter: Option<ControlPlaneTagFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchQuickConnectsResponse {
    #[serde(rename = "ApproximateTotalCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_total_count: Option<i64>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "QuickConnects")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quick_connects: Option<Vec<QuickConnect>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchResourceTagsRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_types: Option<Vec<String>>,
    #[serde(rename = "SearchCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_criteria: Option<ResourceTagsSearchCriteria>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceTagsSearchCriteria {
    #[serde(rename = "TagSearchCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_search_condition: Option<TagSearchCondition>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagSearchCondition {
    #[serde(rename = "tagKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<String>,
    #[serde(rename = "tagKeyComparisonType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key_comparison_type: Option<String>,
    #[serde(rename = "tagValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_value: Option<String>,
    #[serde(rename = "tagValueComparisonType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_value_comparison_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchResourceTagsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<TagSet>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagSet {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchRoutingProfilesRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SearchCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_criteria: Option<RoutingProfileSearchCriteria>,
    #[serde(rename = "SearchFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_filter: Option<RoutingProfileSearchFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RoutingProfileSearchCriteria {
    #[serde(rename = "AndConditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and_conditions: Option<Vec<RoutingProfileSearchCriteria>>,
    #[serde(rename = "OrConditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub or_conditions: Option<Vec<RoutingProfileSearchCriteria>>,
    #[serde(rename = "StringCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_condition: Option<StringCondition>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RoutingProfileSearchFilter {
    #[serde(rename = "TagFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_filter: Option<ControlPlaneTagFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchRoutingProfilesResponse {
    #[serde(rename = "ApproximateTotalCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_total_count: Option<i64>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RoutingProfiles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_profiles: Option<Vec<RoutingProfile>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchSecurityProfilesRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SearchCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_criteria: Option<SecurityProfileSearchCriteria>,
    #[serde(rename = "SearchFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_filter: Option<SecurityProfilesSearchFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SecurityProfileSearchCriteria {
    #[serde(rename = "AndConditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and_conditions: Option<Vec<SecurityProfileSearchCriteria>>,
    #[serde(rename = "OrConditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub or_conditions: Option<Vec<SecurityProfileSearchCriteria>>,
    #[serde(rename = "StringCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_condition: Option<StringCondition>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SecurityProfilesSearchFilter {
    #[serde(rename = "TagFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_filter: Option<ControlPlaneTagFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchSecurityProfilesResponse {
    #[serde(rename = "ApproximateTotalCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_total_count: Option<i64>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SecurityProfiles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_profiles: Option<Vec<SecurityProfileSearchSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SecurityProfileSearchSummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "OrganizationResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_resource_id: Option<String>,
    #[serde(rename = "SecurityProfileName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_profile_name: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchTestCasesRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SearchCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_criteria: Option<TestCaseSearchCriteria>,
    #[serde(rename = "SearchFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_filter: Option<TestCaseSearchFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TestCaseSearchCriteria {
    #[serde(rename = "AndConditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and_conditions: Option<Vec<TestCaseSearchCriteria>>,
    #[serde(rename = "OrConditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub or_conditions: Option<Vec<TestCaseSearchCriteria>>,
    #[serde(rename = "StatusCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_condition: Option<String>,
    #[serde(rename = "StringCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_condition: Option<StringCondition>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TestCaseSearchFilter {
    #[serde(rename = "TagFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_filter: Option<ControlPlaneTagFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchTestCasesResponse {
    #[serde(rename = "ApproximateTotalCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_total_count: Option<i64>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TestCases")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_cases: Option<Vec<TestCase>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchUserHierarchyGroupsRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SearchCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_criteria: Option<UserHierarchyGroupSearchCriteria>,
    #[serde(rename = "SearchFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_filter: Option<UserHierarchyGroupSearchFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserHierarchyGroupSearchCriteria {
    #[serde(rename = "AndConditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and_conditions: Option<Vec<UserHierarchyGroupSearchCriteria>>,
    #[serde(rename = "OrConditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub or_conditions: Option<Vec<UserHierarchyGroupSearchCriteria>>,
    #[serde(rename = "StringCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_condition: Option<StringCondition>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserHierarchyGroupSearchFilter {
    #[serde(rename = "AttributeFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_filter: Option<ControlPlaneAttributeFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchUserHierarchyGroupsResponse {
    #[serde(rename = "ApproximateTotalCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_total_count: Option<i64>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "UserHierarchyGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_hierarchy_groups: Option<Vec<HierarchyGroup>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchUsersRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SearchCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_criteria: Option<UserSearchCriteria>,
    #[serde(rename = "SearchFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_filter: Option<UserSearchFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserSearchCriteria {
    #[serde(rename = "AndConditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and_conditions: Option<Vec<UserSearchCriteria>>,
    #[serde(rename = "HierarchyGroupCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hierarchy_group_condition: Option<HierarchyGroupCondition>,
    #[serde(rename = "ListCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_condition: Option<ListCondition>,
    #[serde(rename = "OrConditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub or_conditions: Option<Vec<UserSearchCriteria>>,
    #[serde(rename = "StringCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_condition: Option<StringCondition>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HierarchyGroupCondition {
    #[serde(rename = "HierarchyGroupMatchType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hierarchy_group_match_type: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCondition {
    #[serde(rename = "Conditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    #[serde(rename = "TargetListType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_list_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Condition {
    #[serde(rename = "NumberCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_condition: Option<NumberCondition>,
    #[serde(rename = "StringCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_condition: Option<StringCondition>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserSearchFilter {
    #[serde(rename = "TagFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_filter: Option<ControlPlaneTagFilter>,
    #[serde(rename = "UserAttributeFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_attribute_filter: Option<ControlPlaneUserAttributeFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ControlPlaneUserAttributeFilter {
    #[serde(rename = "AndCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and_condition: Option<AttributeAndCondition>,
    #[serde(rename = "HierarchyGroupCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hierarchy_group_condition: Option<HierarchyGroupCondition>,
    #[serde(rename = "OrConditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub or_conditions: Option<Vec<AttributeAndCondition>>,
    #[serde(rename = "TagCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_condition: Option<TagCondition>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttributeAndCondition {
    #[serde(rename = "HierarchyGroupCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hierarchy_group_condition: Option<HierarchyGroupCondition>,
    #[serde(rename = "TagConditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_conditions: Option<Vec<TagCondition>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchUsersResponse {
    #[serde(rename = "ApproximateTotalCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_total_count: Option<i64>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Users")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<UserSearchSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserSearchSummary {
    #[serde(rename = "AfterContactWorkConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_contact_work_configs: Option<Vec<AfterContactWorkConfigPerChannel>>,
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "AutoAcceptConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_accept_configs: Option<Vec<AutoAcceptConfig>>,
    #[serde(rename = "DirectoryUserId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_user_id: Option<String>,
    #[serde(rename = "HierarchyGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hierarchy_group_id: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "IdentityInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_info: Option<UserIdentityInfoLite>,
    #[serde(rename = "PersistentConnectionConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent_connection_configs: Option<Vec<PersistentConnectionConfig>>,
    #[serde(rename = "PhoneConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_config: Option<UserPhoneConfig>,
    #[serde(rename = "PhoneNumberConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_configs: Option<Vec<PhoneNumberConfig>>,
    #[serde(rename = "RoutingProfileId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_profile_id: Option<String>,
    #[serde(rename = "SecurityProfileIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_profile_ids: Option<Vec<String>>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Username")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(rename = "VoiceEnhancementConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_enhancement_configs: Option<Vec<VoiceEnhancementConfig>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserIdentityInfoLite {
    #[serde(rename = "FirstName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(rename = "LastName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchViewsRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SearchCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_criteria: Option<ViewSearchCriteria>,
    #[serde(rename = "SearchFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_filter: Option<ViewSearchFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ViewSearchCriteria {
    #[serde(rename = "AndConditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and_conditions: Option<Vec<ViewSearchCriteria>>,
    #[serde(rename = "OrConditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub or_conditions: Option<Vec<ViewSearchCriteria>>,
    #[serde(rename = "StringCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_condition: Option<StringCondition>,
    #[serde(rename = "ViewStatusCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_status_condition: Option<String>,
    #[serde(rename = "ViewTypeCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_type_condition: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ViewSearchFilter {
    #[serde(rename = "AttributeFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_filter: Option<ControlPlaneAttributeFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchViewsResponse {
    #[serde(rename = "ApproximateTotalCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_total_count: Option<i64>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Views")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub views: Option<Vec<View>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchVocabulariesRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NameStartsWith")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_starts_with: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchVocabulariesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "VocabularySummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_summary_list: Option<Vec<VocabularySummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VocabularySummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "FailureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchWorkspaceAssociationsRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SearchCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_criteria: Option<WorkspaceAssociationSearchCriteria>,
    #[serde(rename = "SearchFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_filter: Option<WorkspaceAssociationSearchFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkspaceAssociationSearchCriteria {
    #[serde(rename = "AndConditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and_conditions: Option<Vec<WorkspaceAssociationSearchCriteria>>,
    #[serde(rename = "OrConditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub or_conditions: Option<Vec<WorkspaceAssociationSearchCriteria>>,
    #[serde(rename = "StringCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_condition: Option<StringCondition>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkspaceAssociationSearchFilter {
    #[serde(rename = "AttributeFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_filter: Option<ControlPlaneAttributeFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchWorkspaceAssociationsResponse {
    #[serde(rename = "ApproximateTotalCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_total_count: Option<i64>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "WorkspaceAssociations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_associations: Option<Vec<WorkspaceAssociationSearchSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkspaceAssociationSearchSummary {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "ResourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "WorkspaceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_arn: Option<String>,
    #[serde(rename = "WorkspaceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchWorkspacesRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SearchCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_criteria: Option<WorkspaceSearchCriteria>,
    #[serde(rename = "SearchFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_filter: Option<WorkspaceSearchFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkspaceSearchCriteria {
    #[serde(rename = "AndConditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and_conditions: Option<Vec<WorkspaceSearchCriteria>>,
    #[serde(rename = "OrConditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub or_conditions: Option<Vec<WorkspaceSearchCriteria>>,
    #[serde(rename = "StringCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_condition: Option<StringCondition>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkspaceSearchFilter {
    #[serde(rename = "AttributeFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_filter: Option<ControlPlaneAttributeFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchWorkspacesResponse {
    #[serde(rename = "ApproximateTotalCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_total_count: Option<i64>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Workspaces")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspaces: Option<Vec<WorkspaceSearchSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkspaceSearchSummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "Visibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SendChatIntegrationEventRequest {
    #[serde(rename = "DestinationId")]
    #[serde(default)]
    pub destination_id: String,
    #[serde(rename = "Event")]
    #[serde(default)]
    pub event: ChatEvent,
    #[serde(rename = "NewSessionDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_session_details: Option<NewSessionDetails>,
    #[serde(rename = "SourceId")]
    #[serde(default)]
    pub source_id: String,
    #[serde(rename = "Subtype")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtype: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChatEvent {
    #[serde(rename = "Content")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(rename = "ContentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NewSessionDetails {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ParticipantDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant_details: Option<ParticipantDetails>,
    #[serde(rename = "StreamingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub streaming_configuration: Option<ChatStreamingConfiguration>,
    #[serde(rename = "SupportedMessagingContentTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_messaging_content_types: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ParticipantDetails {
    #[serde(rename = "DisplayName")]
    #[serde(default)]
    pub display_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChatStreamingConfiguration {
    #[serde(rename = "StreamingEndpointArn")]
    #[serde(default)]
    pub streaming_endpoint_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SendChatIntegrationEventResponse {
    #[serde(rename = "InitialContactId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_contact_id: Option<String>,
    #[serde(rename = "NewChatCreated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_chat_created: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SendOutboundEmailRequest {
    #[serde(rename = "AdditionalRecipients")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_recipients: Option<OutboundAdditionalRecipients>,
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "DestinationEmailAddress")]
    #[serde(default)]
    pub destination_email_address: EmailAddressInfo,
    #[serde(rename = "EmailMessage")]
    #[serde(default)]
    pub email_message: OutboundEmailContent,
    #[serde(rename = "FromEmailAddress")]
    #[serde(default)]
    pub from_email_address: EmailAddressInfo,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "SourceCampaign")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_campaign: Option<SourceCampaign>,
    #[serde(rename = "TrafficType")]
    #[serde(default)]
    pub traffic_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OutboundAdditionalRecipients {
    #[serde(rename = "CcEmailAddresses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc_email_addresses: Option<Vec<EmailAddressInfo>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EmailAddressInfo {
    #[serde(rename = "DisplayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "EmailAddress")]
    #[serde(default)]
    pub email_address: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OutboundEmailContent {
    #[serde(rename = "MessageSourceType")]
    #[serde(default)]
    pub message_source_type: String,
    #[serde(rename = "RawMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_message: Option<OutboundRawMessage>,
    #[serde(rename = "TemplatedMessageConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub templated_message_config: Option<TemplatedMessageConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OutboundRawMessage {
    #[serde(rename = "Body")]
    #[serde(default)]
    pub body: String,
    #[serde(rename = "ContentType")]
    #[serde(default)]
    pub content_type: String,
    #[serde(rename = "Subject")]
    #[serde(default)]
    pub subject: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TemplatedMessageConfig {
    #[serde(rename = "KnowledgeBaseId")]
    #[serde(default)]
    pub knowledge_base_id: String,
    #[serde(rename = "MessageTemplateId")]
    #[serde(default)]
    pub message_template_id: String,
    #[serde(rename = "TemplateAttributes")]
    #[serde(default)]
    pub template_attributes: TemplateAttributes,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TemplateAttributes {
    #[serde(rename = "CustomAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_attributes: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "CustomerProfileAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_profile_attributes: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SourceCampaign {
    #[serde(rename = "CampaignId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_id: Option<String>,
    #[serde(rename = "OutboundRequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_request_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SendOutboundEmailResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartAttachedFileUploadRequest {
    #[serde(rename = "AssociatedResourceArn")]
    #[serde(default)]
    pub associated_resource_arn: String,
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "CreatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<CreatedByInfo>,
    #[serde(rename = "FileName")]
    #[serde(default)]
    pub file_name: String,
    #[serde(rename = "FileSizeInBytes")]
    #[serde(default)]
    pub file_size_in_bytes: i64,
    #[serde(rename = "FileUseCaseType")]
    #[serde(default)]
    pub file_use_case_type: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "UrlExpiryInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_expiry_in_seconds: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartAttachedFileUploadResponse {
    #[serde(rename = "CreatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<CreatedByInfo>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(rename = "FileArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_arn: Option<String>,
    #[serde(rename = "FileId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    #[serde(rename = "FileStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_status: Option<String>,
    #[serde(rename = "UploadUrlMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_url_metadata: Option<UploadUrlMetadata>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UploadUrlMetadata {
    #[serde(rename = "HeadersToInclude")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers_to_include: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Url")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "UrlExpiry")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_expiry: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartChatContactRequest {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ChatDurationInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_duration_in_minutes: Option<i32>,
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "ContactFlowId")]
    #[serde(default)]
    pub contact_flow_id: String,
    #[serde(rename = "CustomerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[serde(rename = "DisconnectOnCustomerExit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnect_on_customer_exit: Option<Vec<String>>,
    #[serde(rename = "InitialMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_message: Option<ChatMessage>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "ParticipantConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant_configuration: Option<ParticipantConfiguration>,
    #[serde(rename = "ParticipantDetails")]
    #[serde(default)]
    pub participant_details: ParticipantDetails,
    #[serde(rename = "PersistentChat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent_chat: Option<PersistentChat>,
    #[serde(rename = "RelatedContactId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_contact_id: Option<String>,
    #[serde(rename = "SegmentAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_attributes: Option<std::collections::HashMap<String, SegmentAttributeValue>>,
    #[serde(rename = "SupportedMessagingContentTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_messaging_content_types: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChatMessage {
    #[serde(rename = "Content")]
    #[serde(default)]
    pub content: String,
    #[serde(rename = "ContentType")]
    #[serde(default)]
    pub content_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ParticipantConfiguration {
    #[serde(rename = "ResponseMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PersistentChat {
    #[serde(rename = "RehydrationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rehydration_type: Option<String>,
    #[serde(rename = "SourceContactId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_contact_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartChatContactResponse {
    #[serde(rename = "ContactId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<String>,
    #[serde(rename = "ContinuedFromContactId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continued_from_contact_id: Option<String>,
    #[serde(rename = "ParticipantId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant_id: Option<String>,
    #[serde(rename = "ParticipantToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartContactEvaluationRequest {
    #[serde(rename = "AutoEvaluationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_evaluation_configuration: Option<AutoEvaluationConfiguration>,
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "ContactId")]
    #[serde(default)]
    pub contact_id: String,
    #[serde(rename = "EvaluationFormId")]
    #[serde(default)]
    pub evaluation_form_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutoEvaluationConfiguration {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    pub enabled: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartContactEvaluationResponse {
    #[serde(rename = "EvaluationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_arn: Option<String>,
    #[serde(rename = "EvaluationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartContactMediaProcessingRequest {
    #[serde(rename = "ContactId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<String>,
    #[serde(rename = "FailureMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_mode: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(rename = "ProcessorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processor_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartContactMediaProcessingResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartContactRecordingRequest {
    #[serde(rename = "ContactId")]
    #[serde(default)]
    pub contact_id: String,
    #[serde(rename = "InitialContactId")]
    #[serde(default)]
    pub initial_contact_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "VoiceRecordingConfiguration")]
    #[serde(default)]
    pub voice_recording_configuration: VoiceRecordingConfiguration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VoiceRecordingConfiguration {
    #[serde(rename = "IvrRecordingTrack")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ivr_recording_track: Option<String>,
    #[serde(rename = "VoiceRecordingTrack")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_recording_track: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartContactRecordingResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartContactStreamingRequest {
    #[serde(rename = "ChatStreamingConfiguration")]
    #[serde(default)]
    pub chat_streaming_configuration: ChatStreamingConfiguration,
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    pub client_token: String,
    #[serde(rename = "ContactId")]
    #[serde(default)]
    pub contact_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartContactStreamingResponse {
    #[serde(rename = "StreamingId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub streaming_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartEmailContactRequest {
    #[serde(rename = "AdditionalRecipients")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_recipients: Option<InboundAdditionalRecipients>,
    #[serde(rename = "Attachments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<EmailAttachment>>,
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "ContactFlowId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_flow_id: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "DestinationEmailAddress")]
    #[serde(default)]
    pub destination_email_address: String,
    #[serde(rename = "EmailMessage")]
    #[serde(default)]
    pub email_message: InboundEmailContent,
    #[serde(rename = "FromEmailAddress")]
    #[serde(default)]
    pub from_email_address: EmailAddressInfo,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "References")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub references: Option<std::collections::HashMap<String, Reference>>,
    #[serde(rename = "RelatedContactId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_contact_id: Option<String>,
    #[serde(rename = "SegmentAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_attributes: Option<std::collections::HashMap<String, SegmentAttributeValue>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InboundAdditionalRecipients {
    #[serde(rename = "CcAddresses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc_addresses: Option<Vec<EmailAddressInfo>>,
    #[serde(rename = "ToAddresses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_addresses: Option<Vec<EmailAddressInfo>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EmailAttachment {
    #[serde(rename = "FileName")]
    #[serde(default)]
    pub file_name: String,
    #[serde(rename = "S3Url")]
    #[serde(default)]
    pub s3_url: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InboundEmailContent {
    #[serde(rename = "MessageSourceType")]
    #[serde(default)]
    pub message_source_type: String,
    #[serde(rename = "RawMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_message: Option<InboundRawMessage>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InboundRawMessage {
    #[serde(rename = "Body")]
    #[serde(default)]
    pub body: String,
    #[serde(rename = "ContentType")]
    #[serde(default)]
    pub content_type: String,
    #[serde(rename = "Headers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Subject")]
    #[serde(default)]
    pub subject: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartEmailContactResponse {
    #[serde(rename = "ContactId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartOutboundChatContactRequest {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ChatDurationInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_duration_in_minutes: Option<i32>,
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "ContactFlowId")]
    #[serde(default)]
    pub contact_flow_id: String,
    #[serde(rename = "DestinationEndpoint")]
    #[serde(default)]
    pub destination_endpoint: Endpoint,
    #[serde(rename = "InitialSystemMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_system_message: Option<ChatMessage>,
    #[serde(rename = "InitialTemplatedSystemMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_templated_system_message: Option<TemplatedMessageConfig>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "ParticipantDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant_details: Option<ParticipantDetails>,
    #[serde(rename = "RelatedContactId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_contact_id: Option<String>,
    #[serde(rename = "SegmentAttributes")]
    #[serde(default)]
    pub segment_attributes: std::collections::HashMap<String, SegmentAttributeValue>,
    #[serde(rename = "SourceEndpoint")]
    #[serde(default)]
    pub source_endpoint: Endpoint,
    #[serde(rename = "SupportedMessagingContentTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_messaging_content_types: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartOutboundChatContactResponse {
    #[serde(rename = "ContactId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartOutboundEmailContactRequest {
    #[serde(rename = "AdditionalRecipients")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_recipients: Option<OutboundAdditionalRecipients>,
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "ContactId")]
    #[serde(default)]
    pub contact_id: String,
    #[serde(rename = "DestinationEmailAddress")]
    #[serde(default)]
    pub destination_email_address: EmailAddressInfo,
    #[serde(rename = "EmailMessage")]
    #[serde(default)]
    pub email_message: OutboundEmailContent,
    #[serde(rename = "FromEmailAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_email_address: Option<EmailAddressInfo>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartOutboundEmailContactResponse {
    #[serde(rename = "ContactId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartOutboundVoiceContactRequest {
    #[serde(rename = "AnswerMachineDetectionConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answer_machine_detection_config: Option<AnswerMachineDetectionConfig>,
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "CampaignId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_id: Option<String>,
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "ContactFlowId")]
    #[serde(default)]
    pub contact_flow_id: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "DestinationPhoneNumber")]
    #[serde(default)]
    pub destination_phone_number: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "OutboundStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_strategy: Option<OutboundStrategy>,
    #[serde(rename = "QueueId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_id: Option<String>,
    #[serde(rename = "References")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub references: Option<std::collections::HashMap<String, Reference>>,
    #[serde(rename = "RelatedContactId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_contact_id: Option<String>,
    #[serde(rename = "RingTimeoutInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ring_timeout_in_seconds: Option<i32>,
    #[serde(rename = "SourcePhoneNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_phone_number: Option<String>,
    #[serde(rename = "TrafficType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnswerMachineDetectionConfig {
    #[serde(rename = "AwaitAnswerMachinePrompt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub await_answer_machine_prompt: Option<bool>,
    #[serde(rename = "EnableAnswerMachineDetection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_answer_machine_detection: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartOutboundVoiceContactResponse {
    #[serde(rename = "ContactId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartScreenSharingRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "ContactId")]
    #[serde(default)]
    pub contact_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartScreenSharingResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartTaskContactRequest {
    #[serde(rename = "Attachments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<TaskAttachment>>,
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "ContactFlowId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_flow_id: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "PreviousContactId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_contact_id: Option<String>,
    #[serde(rename = "QuickConnectId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quick_connect_id: Option<String>,
    #[serde(rename = "References")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub references: Option<std::collections::HashMap<String, Reference>>,
    #[serde(rename = "RelatedContactId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_contact_id: Option<String>,
    #[serde(rename = "ScheduledTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_time: Option<f64>,
    #[serde(rename = "SegmentAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_attributes: Option<std::collections::HashMap<String, SegmentAttributeValue>>,
    #[serde(rename = "TaskTemplateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_template_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TaskAttachment {
    #[serde(rename = "FileName")]
    #[serde(default)]
    pub file_name: String,
    #[serde(rename = "S3Url")]
    #[serde(default)]
    pub s3_url: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartTaskContactResponse {
    #[serde(rename = "ContactId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartTestCaseExecutionRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "TestCaseId")]
    #[serde(default)]
    pub test_case_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartTestCaseExecutionResponse {
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TestCaseExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_case_execution_id: Option<String>,
    #[serde(rename = "TestCaseId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_case_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartWebRTCContactRequest {
    #[serde(rename = "AllowedCapabilities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_capabilities: Option<AllowedCapabilities>,
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "ContactFlowId")]
    #[serde(default)]
    pub contact_flow_id: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "ParticipantDetails")]
    #[serde(default)]
    pub participant_details: ParticipantDetails,
    #[serde(rename = "References")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub references: Option<std::collections::HashMap<String, Reference>>,
    #[serde(rename = "RelatedContactId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_contact_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AllowedCapabilities {
    #[serde(rename = "Agent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent: Option<ParticipantCapabilities>,
    #[serde(rename = "Customer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<ParticipantCapabilities>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartWebRTCContactResponse {
    #[serde(rename = "ConnectionData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_data: Option<ConnectionData>,
    #[serde(rename = "ContactId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<String>,
    #[serde(rename = "ParticipantId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant_id: Option<String>,
    #[serde(rename = "ParticipantToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectionData {
    #[serde(rename = "Attendee")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attendee: Option<Attendee>,
    #[serde(rename = "Meeting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meeting: Option<Meeting>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Attendee {
    #[serde(rename = "AttendeeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attendee_id: Option<String>,
    #[serde(rename = "JoinToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Meeting {
    #[serde(rename = "MediaPlacement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_placement: Option<MediaPlacement>,
    #[serde(rename = "MediaRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_region: Option<String>,
    #[serde(rename = "MeetingFeatures")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meeting_features: Option<MeetingFeaturesConfiguration>,
    #[serde(rename = "MeetingId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meeting_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MediaPlacement {
    #[serde(rename = "AudioFallbackUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_fallback_url: Option<String>,
    #[serde(rename = "AudioHostUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_host_url: Option<String>,
    #[serde(rename = "EventIngestionUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_ingestion_url: Option<String>,
    #[serde(rename = "SignalingUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signaling_url: Option<String>,
    #[serde(rename = "TurnControlUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turn_control_url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MeetingFeaturesConfiguration {
    #[serde(rename = "Audio")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<AudioFeatures>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AudioFeatures {
    #[serde(rename = "EchoReduction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub echo_reduction: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopContactMediaProcessingRequest {
    #[serde(rename = "ContactId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopContactMediaProcessingResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopContactRecordingRequest {
    #[serde(rename = "ContactId")]
    #[serde(default)]
    pub contact_id: String,
    #[serde(rename = "ContactRecordingType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_recording_type: Option<String>,
    #[serde(rename = "InitialContactId")]
    #[serde(default)]
    pub initial_contact_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopContactRecordingResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopContactRequest {
    #[serde(rename = "ContactId")]
    #[serde(default)]
    pub contact_id: String,
    #[serde(rename = "DisconnectReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnect_reason: Option<DisconnectReason>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisconnectReason {
    #[serde(rename = "Code")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopContactResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopContactStreamingRequest {
    #[serde(rename = "ContactId")]
    #[serde(default)]
    pub contact_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "StreamingId")]
    #[serde(default)]
    pub streaming_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopContactStreamingResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopTestCaseExecutionRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "TestCaseExecutionId")]
    #[serde(default)]
    pub test_case_execution_id: String,
    #[serde(rename = "TestCaseId")]
    #[serde(default)]
    pub test_case_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopTestCaseExecutionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SubmitContactEvaluationRequest {
    #[serde(rename = "Answers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answers: Option<std::collections::HashMap<String, EvaluationAnswerInput>>,
    #[serde(rename = "EvaluationId")]
    #[serde(default)]
    pub evaluation_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Notes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<std::collections::HashMap<String, EvaluationNote>>,
    #[serde(rename = "SubmittedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submitted_by: Option<EvaluatorUserUnion>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationAnswerInput {
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<EvaluationAnswerData>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluatorUserUnion {
    #[serde(rename = "ConnectUserArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_user_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SubmitContactEvaluationResponse {
    #[serde(rename = "EvaluationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_arn: Option<String>,
    #[serde(rename = "EvaluationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SuspendContactRecordingRequest {
    #[serde(rename = "ContactId")]
    #[serde(default)]
    pub contact_id: String,
    #[serde(rename = "ContactRecordingType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_recording_type: Option<String>,
    #[serde(rename = "InitialContactId")]
    #[serde(default)]
    pub initial_contact_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SuspendContactRecordingResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagContactRequest {
    #[serde(rename = "ContactId")]
    #[serde(default)]
    pub contact_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagContactResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(default)]
    pub tags: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TransferContactRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "ContactFlowId")]
    #[serde(default)]
    pub contact_flow_id: String,
    #[serde(rename = "ContactId")]
    #[serde(default)]
    pub contact_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "QueueId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_id: Option<String>,
    #[serde(rename = "UserId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TransferContactResponse {
    #[serde(rename = "ContactArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_arn: Option<String>,
    #[serde(rename = "ContactId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagContactRequest {
    #[serde(rename = "ContactId")]
    #[serde(default)]
    pub contact_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagContactResponse {}

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
pub struct UpdateAgentStatusRequest {
    #[serde(rename = "AgentStatusId")]
    #[serde(default)]
    pub agent_status_id: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "DisplayOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_order: Option<i32>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ResetOrderNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_order_number: Option<bool>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAttachedFilesConfigurationRequest {
    #[serde(rename = "AttachmentScope")]
    #[serde(default)]
    pub attachment_scope: String,
    #[serde(rename = "ExtensionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_configuration: Option<ExtensionConfiguration>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "MaximumSizeLimitInBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_size_limit_in_bytes: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAttachedFilesConfigurationResponse {
    #[serde(rename = "AttachmentScope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_scope: Option<String>,
    #[serde(rename = "ExtensionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_configuration: Option<ExtensionConfiguration>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "MaximumSizeLimitInBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_size_limit_in_bytes: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAuthenticationProfileRequest {
    #[serde(rename = "AllowedIps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_ips: Option<Vec<String>>,
    #[serde(rename = "AuthenticationProfileId")]
    #[serde(default)]
    pub authentication_profile_id: String,
    #[serde(rename = "BlockedIps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked_ips: Option<Vec<String>>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "PeriodicSessionDuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub periodic_session_duration: Option<i32>,
    #[serde(rename = "SessionInactivityDuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_inactivity_duration: Option<i32>,
    #[serde(rename = "SessionInactivityHandlingEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_inactivity_handling_enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateContactAttributesRequest {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    pub attributes: std::collections::HashMap<String, String>,
    #[serde(rename = "InitialContactId")]
    #[serde(default)]
    pub initial_contact_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateContactAttributesResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateContactEvaluationRequest {
    #[serde(rename = "Answers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answers: Option<std::collections::HashMap<String, EvaluationAnswerInput>>,
    #[serde(rename = "EvaluationId")]
    #[serde(default)]
    pub evaluation_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Notes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<std::collections::HashMap<String, EvaluationNote>>,
    #[serde(rename = "UpdatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<EvaluatorUserUnion>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateContactEvaluationResponse {
    #[serde(rename = "EvaluationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_arn: Option<String>,
    #[serde(rename = "EvaluationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateContactFlowContentRequest {
    #[serde(rename = "ContactFlowId")]
    #[serde(default)]
    pub contact_flow_id: String,
    #[serde(rename = "Content")]
    #[serde(default)]
    pub content: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateContactFlowContentResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateContactFlowMetadataRequest {
    #[serde(rename = "ContactFlowId")]
    #[serde(default)]
    pub contact_flow_id: String,
    #[serde(rename = "ContactFlowState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_flow_state: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateContactFlowMetadataResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateContactFlowModuleAliasRequest {
    #[serde(rename = "AliasId")]
    #[serde(default)]
    pub alias_id: String,
    #[serde(rename = "ContactFlowModuleId")]
    #[serde(default)]
    pub contact_flow_module_id: String,
    #[serde(rename = "ContactFlowModuleVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_flow_module_version: Option<i64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateContactFlowModuleAliasResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateContactFlowModuleContentRequest {
    #[serde(rename = "ContactFlowModuleId")]
    #[serde(default)]
    pub contact_flow_module_id: String,
    #[serde(rename = "Content")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Settings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateContactFlowModuleContentResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateContactFlowModuleMetadataRequest {
    #[serde(rename = "ContactFlowModuleId")]
    #[serde(default)]
    pub contact_flow_module_id: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateContactFlowModuleMetadataResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateContactFlowNameRequest {
    #[serde(rename = "ContactFlowId")]
    #[serde(default)]
    pub contact_flow_id: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateContactFlowNameResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateContactRequest {
    #[serde(rename = "ContactId")]
    #[serde(default)]
    pub contact_id: String,
    #[serde(rename = "CustomerEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_endpoint: Option<Endpoint>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "QueueInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_info: Option<QueueInfoInput>,
    #[serde(rename = "References")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub references: Option<std::collections::HashMap<String, Reference>>,
    #[serde(rename = "SegmentAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_attributes: Option<std::collections::HashMap<String, SegmentAttributeValue>>,
    #[serde(rename = "SystemEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_endpoint: Option<Endpoint>,
    #[serde(rename = "UserInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_info: Option<UserInfo>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueueInfoInput {
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateContactResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateContactRoutingDataRequest {
    #[serde(rename = "ContactId")]
    #[serde(default)]
    pub contact_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "QueuePriority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_priority: Option<i64>,
    #[serde(rename = "QueueTimeAdjustmentSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_time_adjustment_seconds: Option<i32>,
    #[serde(rename = "RoutingCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_criteria: Option<RoutingCriteriaInput>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RoutingCriteriaInput {
    #[serde(rename = "Steps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<Vec<RoutingCriteriaInputStep>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RoutingCriteriaInputStep {
    #[serde(rename = "Expiry")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry: Option<RoutingCriteriaInputStepExpiry>,
    #[serde(rename = "Expression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<Expression>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RoutingCriteriaInputStepExpiry {
    #[serde(rename = "DurationInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_in_seconds: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateContactRoutingDataResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateContactScheduleRequest {
    #[serde(rename = "ContactId")]
    #[serde(default)]
    pub contact_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "ScheduledTime")]
    #[serde(default)]
    pub scheduled_time: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateContactScheduleResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDataTableAttributeRequest {
    #[serde(rename = "AttributeName")]
    #[serde(default)]
    pub attribute_name: String,
    #[serde(rename = "DataTableId")]
    #[serde(default)]
    pub data_table_id: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Primary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
    #[serde(rename = "Validation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation: Option<Validation>,
    #[serde(rename = "ValueType")]
    #[serde(default)]
    pub value_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDataTableAttributeResponse {
    #[serde(rename = "LockVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_version: Option<DataTableLockVersion>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDataTableMetadataRequest {
    #[serde(rename = "DataTableId")]
    #[serde(default)]
    pub data_table_id: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "TimeZone")]
    #[serde(default)]
    pub time_zone: String,
    #[serde(rename = "ValueLockLevel")]
    #[serde(default)]
    pub value_lock_level: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDataTableMetadataResponse {
    #[serde(rename = "LockVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_version: Option<DataTableLockVersion>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDataTablePrimaryValuesRequest {
    #[serde(rename = "DataTableId")]
    #[serde(default)]
    pub data_table_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "LockVersion")]
    #[serde(default)]
    pub lock_version: DataTableLockVersion,
    #[serde(rename = "NewPrimaryValues")]
    #[serde(default)]
    pub new_primary_values: Vec<PrimaryValue>,
    #[serde(rename = "PrimaryValues")]
    #[serde(default)]
    pub primary_values: Vec<PrimaryValue>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDataTablePrimaryValuesResponse {
    #[serde(rename = "LockVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_version: Option<DataTableLockVersion>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateEmailAddressMetadataRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "DisplayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "EmailAddressId")]
    #[serde(default)]
    pub email_address_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateEmailAddressMetadataResponse {
    #[serde(rename = "EmailAddressArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address_arn: Option<String>,
    #[serde(rename = "EmailAddressId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateEvaluationFormRequest {
    #[serde(rename = "AsDraft")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as_draft: Option<bool>,
    #[serde(rename = "AutoEvaluationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_evaluation_configuration: Option<EvaluationFormAutoEvaluationConfiguration>,
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "CreateNewVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_new_version: Option<bool>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EvaluationFormId")]
    #[serde(default)]
    pub evaluation_form_id: String,
    #[serde(rename = "EvaluationFormVersion")]
    #[serde(default)]
    pub evaluation_form_version: i32,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Items")]
    #[serde(default)]
    pub items: Vec<EvaluationFormItem>,
    #[serde(rename = "LanguageConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_configuration: Option<EvaluationFormLanguageConfiguration>,
    #[serde(rename = "ReviewConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_configuration: Option<EvaluationReviewConfiguration>,
    #[serde(rename = "ScoringStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scoring_strategy: Option<EvaluationFormScoringStrategy>,
    #[serde(rename = "TargetConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_configuration: Option<EvaluationFormTargetConfiguration>,
    #[serde(rename = "Title")]
    #[serde(default)]
    pub title: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateEvaluationFormResponse {
    #[serde(rename = "EvaluationFormArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_form_arn: Option<String>,
    #[serde(rename = "EvaluationFormId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_form_id: Option<String>,
    #[serde(rename = "EvaluationFormVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_form_version: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateHoursOfOperationOverrideRequest {
    #[serde(rename = "Config")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<Vec<HoursOfOperationOverrideConfig>>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EffectiveFrom")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_from: Option<String>,
    #[serde(rename = "EffectiveTill")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_till: Option<String>,
    #[serde(rename = "HoursOfOperationId")]
    #[serde(default)]
    pub hours_of_operation_id: String,
    #[serde(rename = "HoursOfOperationOverrideId")]
    #[serde(default)]
    pub hours_of_operation_override_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "OverrideType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_type: Option<String>,
    #[serde(rename = "RecurrenceConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrence_config: Option<RecurrenceConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateHoursOfOperationRequest {
    #[serde(rename = "Config")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<Vec<HoursOfOperationConfig>>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "HoursOfOperationId")]
    #[serde(default)]
    pub hours_of_operation_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "TimeZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateInstanceAttributeRequest {
    #[serde(rename = "AttributeType")]
    #[serde(default)]
    pub attribute_type: String,
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateInstanceStorageConfigRequest {
    #[serde(rename = "AssociationId")]
    #[serde(default)]
    pub association_id: String,
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    pub resource_type: String,
    #[serde(rename = "StorageConfig")]
    #[serde(default)]
    pub storage_config: InstanceStorageConfig,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateNotificationContentRequest {
    #[serde(rename = "Content")]
    #[serde(default)]
    pub content: std::collections::HashMap<String, String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "NotificationId")]
    #[serde(default)]
    pub notification_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateNotificationContentResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateParticipantAuthenticationRequest {
    #[serde(rename = "Code")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "Error")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "ErrorDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_description: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "State")]
    #[serde(default)]
    pub state: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateParticipantAuthenticationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateParticipantRoleConfigRequest {
    #[serde(rename = "ChannelConfiguration")]
    #[serde(default)]
    pub channel_configuration: UpdateParticipantRoleConfigChannelInfo,
    #[serde(rename = "ContactId")]
    #[serde(default)]
    pub contact_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateParticipantRoleConfigChannelInfo {
    #[serde(rename = "Chat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat: Option<ChatParticipantRoleConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChatParticipantRoleConfig {
    #[serde(rename = "ParticipantTimerConfigList")]
    #[serde(default)]
    pub participant_timer_config_list: Vec<ParticipantTimerConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ParticipantTimerConfiguration {
    #[serde(rename = "ParticipantRole")]
    #[serde(default)]
    pub participant_role: String,
    #[serde(rename = "TimerType")]
    #[serde(default)]
    pub timer_type: String,
    #[serde(rename = "TimerValue")]
    #[serde(default)]
    pub timer_value: ParticipantTimerValue,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ParticipantTimerValue {
    #[serde(rename = "ParticipantTimerAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant_timer_action: Option<String>,
    #[serde(rename = "ParticipantTimerDurationInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant_timer_duration_in_minutes: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateParticipantRoleConfigResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePhoneNumberMetadataRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "PhoneNumberDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_description: Option<String>,
    #[serde(rename = "PhoneNumberId")]
    #[serde(default)]
    pub phone_number_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePhoneNumberRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(rename = "PhoneNumberId")]
    #[serde(default)]
    pub phone_number_id: String,
    #[serde(rename = "TargetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePhoneNumberResponse {
    #[serde(rename = "PhoneNumberArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_arn: Option<String>,
    #[serde(rename = "PhoneNumberId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePredefinedAttributeRequest {
    #[serde(rename = "AttributeConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_configuration: Option<InputPredefinedAttributeConfiguration>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Purposes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purposes: Option<Vec<String>>,
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<PredefinedAttributeValues>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePromptRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "PromptId")]
    #[serde(default)]
    pub prompt_id: String,
    #[serde(rename = "S3Uri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePromptResponse {
    #[serde(rename = "PromptARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_a_r_n: Option<String>,
    #[serde(rename = "PromptId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateQueueHoursOfOperationRequest {
    #[serde(rename = "HoursOfOperationId")]
    #[serde(default)]
    pub hours_of_operation_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "QueueId")]
    #[serde(default)]
    pub queue_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateQueueMaxContactsRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "MaxContacts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_contacts: Option<i32>,
    #[serde(rename = "QueueId")]
    #[serde(default)]
    pub queue_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateQueueNameRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "QueueId")]
    #[serde(default)]
    pub queue_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateQueueOutboundCallerConfigRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "OutboundCallerConfig")]
    #[serde(default)]
    pub outbound_caller_config: OutboundCallerConfig,
    #[serde(rename = "QueueId")]
    #[serde(default)]
    pub queue_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateQueueOutboundEmailConfigRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "OutboundEmailConfig")]
    #[serde(default)]
    pub outbound_email_config: OutboundEmailConfig,
    #[serde(rename = "QueueId")]
    #[serde(default)]
    pub queue_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateQueueStatusRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "QueueId")]
    #[serde(default)]
    pub queue_id: String,
    #[serde(rename = "Status")]
    #[serde(default)]
    pub status: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateQuickConnectConfigRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "QuickConnectConfig")]
    #[serde(default)]
    pub quick_connect_config: QuickConnectConfig,
    #[serde(rename = "QuickConnectId")]
    #[serde(default)]
    pub quick_connect_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateQuickConnectNameRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "QuickConnectId")]
    #[serde(default)]
    pub quick_connect_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRoutingProfileAgentAvailabilityTimerRequest {
    #[serde(rename = "AgentAvailabilityTimer")]
    #[serde(default)]
    pub agent_availability_timer: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "RoutingProfileId")]
    #[serde(default)]
    pub routing_profile_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRoutingProfileConcurrencyRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "MediaConcurrencies")]
    #[serde(default)]
    pub media_concurrencies: Vec<MediaConcurrency>,
    #[serde(rename = "RoutingProfileId")]
    #[serde(default)]
    pub routing_profile_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRoutingProfileDefaultOutboundQueueRequest {
    #[serde(rename = "DefaultOutboundQueueId")]
    #[serde(default)]
    pub default_outbound_queue_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "RoutingProfileId")]
    #[serde(default)]
    pub routing_profile_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRoutingProfileNameRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "RoutingProfileId")]
    #[serde(default)]
    pub routing_profile_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRoutingProfileQueuesRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "QueueConfigs")]
    #[serde(default)]
    pub queue_configs: Vec<RoutingProfileQueueConfig>,
    #[serde(rename = "RoutingProfileId")]
    #[serde(default)]
    pub routing_profile_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRuleRequest {
    #[serde(rename = "Actions")]
    #[serde(default)]
    pub actions: Vec<RuleAction>,
    #[serde(rename = "Function")]
    #[serde(default)]
    pub function: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "PublishStatus")]
    #[serde(default)]
    pub publish_status: String,
    #[serde(rename = "RuleId")]
    #[serde(default)]
    pub rule_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSecurityProfileRequest {
    #[serde(rename = "AllowedAccessControlHierarchyGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_access_control_hierarchy_group_id: Option<String>,
    #[serde(rename = "AllowedAccessControlTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_access_control_tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "AllowedFlowModules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_flow_modules: Option<Vec<FlowModule>>,
    #[serde(rename = "Applications")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applications: Option<Vec<Application>>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "GranularAccessControlConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granular_access_control_configuration: Option<GranularAccessControlConfiguration>,
    #[serde(rename = "HierarchyRestrictedResources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hierarchy_restricted_resources: Option<Vec<String>>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Permissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
    #[serde(rename = "SecurityProfileId")]
    #[serde(default)]
    pub security_profile_id: String,
    #[serde(rename = "TagRestrictedResources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_restricted_resources: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTaskTemplateRequest {
    #[serde(rename = "Constraints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraints: Option<TaskTemplateConstraints>,
    #[serde(rename = "ContactFlowId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_flow_id: Option<String>,
    #[serde(rename = "Defaults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defaults: Option<TaskTemplateDefaults>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Fields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<TaskTemplateField>>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "SelfAssignFlowId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_assign_flow_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TaskTemplateId")]
    #[serde(default)]
    pub task_template_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTaskTemplateResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Constraints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraints: Option<TaskTemplateConstraints>,
    #[serde(rename = "ContactFlowId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_flow_id: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "Defaults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defaults: Option<TaskTemplateDefaults>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Fields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<TaskTemplateField>>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "SelfAssignFlowId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_assign_flow_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTestCaseRequest {
    #[serde(rename = "Content")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EntryPoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_point: Option<TestCaseEntryPoint>,
    #[serde(rename = "InitializationData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initialization_data: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "LastModifiedRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_region: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TestCaseId")]
    #[serde(default)]
    pub test_case_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTestCaseResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTrafficDistributionRequest {
    #[serde(rename = "AgentConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_config: Option<AgentConfig>,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "SignInConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sign_in_config: Option<SignInConfig>,
    #[serde(rename = "TelephonyConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telephony_config: Option<TelephonyConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTrafficDistributionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateUserConfigRequest {
    #[serde(rename = "AfterContactWorkConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_contact_work_configs: Option<Vec<AfterContactWorkConfigPerChannel>>,
    #[serde(rename = "AutoAcceptConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_accept_configs: Option<Vec<AutoAcceptConfig>>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "PersistentConnectionConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent_connection_configs: Option<Vec<PersistentConnectionConfig>>,
    #[serde(rename = "PhoneNumberConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_configs: Option<Vec<PhoneNumberConfig>>,
    #[serde(rename = "UserId")]
    #[serde(default)]
    pub user_id: String,
    #[serde(rename = "VoiceEnhancementConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_enhancement_configs: Option<Vec<VoiceEnhancementConfig>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateUserHierarchyGroupNameRequest {
    #[serde(rename = "HierarchyGroupId")]
    #[serde(default)]
    pub hierarchy_group_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateUserHierarchyRequest {
    #[serde(rename = "HierarchyGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hierarchy_group_id: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "UserId")]
    #[serde(default)]
    pub user_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateUserHierarchyStructureRequest {
    #[serde(rename = "HierarchyStructure")]
    #[serde(default)]
    pub hierarchy_structure: HierarchyStructureUpdate,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HierarchyStructureUpdate {
    #[serde(rename = "LevelFive")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_five: Option<HierarchyLevelUpdate>,
    #[serde(rename = "LevelFour")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_four: Option<HierarchyLevelUpdate>,
    #[serde(rename = "LevelOne")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_one: Option<HierarchyLevelUpdate>,
    #[serde(rename = "LevelThree")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_three: Option<HierarchyLevelUpdate>,
    #[serde(rename = "LevelTwo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_two: Option<HierarchyLevelUpdate>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HierarchyLevelUpdate {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateUserIdentityInfoRequest {
    #[serde(rename = "IdentityInfo")]
    #[serde(default)]
    pub identity_info: UserIdentityInfo,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "UserId")]
    #[serde(default)]
    pub user_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateUserNotificationStatusRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "LastModifiedRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_region: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "NotificationId")]
    #[serde(default)]
    pub notification_id: String,
    #[serde(rename = "Status")]
    #[serde(default)]
    pub status: String,
    #[serde(rename = "UserId")]
    #[serde(default)]
    pub user_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateUserNotificationStatusResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateUserPhoneConfigRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "PhoneConfig")]
    #[serde(default)]
    pub phone_config: UserPhoneConfig,
    #[serde(rename = "UserId")]
    #[serde(default)]
    pub user_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateUserProficienciesRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "UserId")]
    #[serde(default)]
    pub user_id: String,
    #[serde(rename = "UserProficiencies")]
    #[serde(default)]
    pub user_proficiencies: Vec<UserProficiency>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateUserRoutingProfileRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "RoutingProfileId")]
    #[serde(default)]
    pub routing_profile_id: String,
    #[serde(rename = "UserId")]
    #[serde(default)]
    pub user_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateUserSecurityProfilesRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "SecurityProfileIds")]
    #[serde(default)]
    pub security_profile_ids: Vec<String>,
    #[serde(rename = "UserId")]
    #[serde(default)]
    pub user_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateViewContentRequest {
    #[serde(rename = "Content")]
    #[serde(default)]
    pub content: ViewInputContent,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Status")]
    #[serde(default)]
    pub status: String,
    #[serde(rename = "ViewId")]
    #[serde(default)]
    pub view_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateViewContentResponse {
    #[serde(rename = "View")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view: Option<View>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateViewMetadataRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ViewId")]
    #[serde(default)]
    pub view_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateViewMetadataResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateWorkspaceMetadataRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "WorkspaceId")]
    #[serde(default)]
    pub workspace_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateWorkspaceMetadataResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateWorkspacePageRequest {
    #[serde(rename = "InputData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_data: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "NewPage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_page: Option<String>,
    #[serde(rename = "Page")]
    #[serde(default)]
    pub page: String,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "Slug")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(rename = "WorkspaceId")]
    #[serde(default)]
    pub workspace_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateWorkspacePageResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateWorkspaceThemeRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Theme")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme: Option<WorkspaceTheme>,
    #[serde(rename = "WorkspaceId")]
    #[serde(default)]
    pub workspace_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateWorkspaceThemeResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateWorkspaceVisibilityRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Visibility")]
    #[serde(default)]
    pub visibility: String,
    #[serde(rename = "WorkspaceId")]
    #[serde(default)]
    pub workspace_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateWorkspaceVisibilityResponse {}
