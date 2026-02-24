//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-ssm

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddTagsToResourceRequest {
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    pub resource_type: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Vec<Tag>,
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
pub struct AddTagsToResourceResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateOpsItemRelatedItemRequest {
    #[serde(rename = "AssociationType")]
    #[serde(default)]
    pub association_type: String,
    #[serde(rename = "OpsItemId")]
    #[serde(default)]
    pub ops_item_id: String,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    pub resource_type: String,
    #[serde(rename = "ResourceUri")]
    #[serde(default)]
    pub resource_uri: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateOpsItemRelatedItemResponse {
    #[serde(rename = "AssociationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelCommandRequest {
    #[serde(rename = "CommandId")]
    #[serde(default)]
    pub command_id: String,
    #[serde(rename = "InstanceIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelCommandResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelMaintenanceWindowExecutionRequest {
    #[serde(rename = "WindowExecutionId")]
    #[serde(default)]
    pub window_execution_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelMaintenanceWindowExecutionResult {
    #[serde(rename = "WindowExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_execution_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateActivationRequest {
    #[serde(rename = "DefaultInstanceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_instance_name: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ExpirationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<f64>,
    #[serde(rename = "IamRole")]
    #[serde(default)]
    pub iam_role: String,
    #[serde(rename = "RegistrationLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_limit: Option<i32>,
    #[serde(rename = "RegistrationMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_metadata: Option<Vec<RegistrationMetadataItem>>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegistrationMetadataItem {
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateActivationResult {
    #[serde(rename = "ActivationCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activation_code: Option<String>,
    #[serde(rename = "ActivationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activation_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAssociationBatchRequest {
    #[serde(rename = "AssociationDispatchAssumeRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_dispatch_assume_role: Option<String>,
    #[serde(rename = "Entries")]
    #[serde(default)]
    pub entries: Vec<CreateAssociationBatchRequestEntry>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAssociationBatchRequestEntry {
    #[serde(rename = "AlarmConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_configuration: Option<AlarmConfiguration>,
    #[serde(rename = "ApplyOnlyAtCronInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_only_at_cron_interval: Option<bool>,
    #[serde(rename = "AssociationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_name: Option<String>,
    #[serde(rename = "AutomationTargetParameterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automation_target_parameter_name: Option<String>,
    #[serde(rename = "CalendarNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calendar_names: Option<Vec<String>>,
    #[serde(rename = "ComplianceSeverity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_severity: Option<String>,
    #[serde(rename = "DocumentVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    #[serde(rename = "Duration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(rename = "MaxConcurrency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrency: Option<String>,
    #[serde(rename = "MaxErrors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_errors: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "OutputLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_location: Option<InstanceAssociationOutputLocation>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "ScheduleExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_expression: Option<String>,
    #[serde(rename = "ScheduleOffset")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_offset: Option<i32>,
    #[serde(rename = "SyncCompliance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_compliance: Option<String>,
    #[serde(rename = "TargetLocations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_locations: Option<Vec<TargetLocation>>,
    #[serde(rename = "TargetMaps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_maps: Option<Vec<std::collections::HashMap<String, Vec<String>>>>,
    #[serde(rename = "Targets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AlarmConfiguration {
    #[serde(rename = "Alarms")]
    #[serde(default)]
    pub alarms: Vec<Alarm>,
    #[serde(rename = "IgnorePollAlarmFailure")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_poll_alarm_failure: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Alarm {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceAssociationOutputLocation {
    #[serde(rename = "S3Location")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_location: Option<S3OutputLocation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3OutputLocation {
    #[serde(rename = "OutputS3BucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_s3_bucket_name: Option<String>,
    #[serde(rename = "OutputS3KeyPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_s3_key_prefix: Option<String>,
    #[serde(rename = "OutputS3Region")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_s3_region: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TargetLocation {
    #[serde(rename = "Accounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<String>>,
    #[serde(rename = "ExcludeAccounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_accounts: Option<Vec<String>>,
    #[serde(rename = "ExecutionRoleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role_name: Option<String>,
    #[serde(rename = "IncludeChildOrganizationUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_child_organization_units: Option<bool>,
    #[serde(rename = "Regions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<String>>,
    #[serde(rename = "TargetLocationAlarmConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_location_alarm_configuration: Option<AlarmConfiguration>,
    #[serde(rename = "TargetLocationMaxConcurrency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_location_max_concurrency: Option<String>,
    #[serde(rename = "TargetLocationMaxErrors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_location_max_errors: Option<String>,
    #[serde(rename = "Targets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
    #[serde(rename = "TargetsMaxConcurrency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets_max_concurrency: Option<String>,
    #[serde(rename = "TargetsMaxErrors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets_max_errors: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Target {
    #[serde(rename = "Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAssociationBatchResult {
    #[serde(rename = "Failed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed: Option<Vec<FailedCreateAssociation>>,
    #[serde(rename = "Successful")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful: Option<Vec<AssociationDescription>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FailedCreateAssociation {
    #[serde(rename = "Entry")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry: Option<CreateAssociationBatchRequestEntry>,
    #[serde(rename = "Fault")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fault: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociationDescription {
    #[serde(rename = "AlarmConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_configuration: Option<AlarmConfiguration>,
    #[serde(rename = "ApplyOnlyAtCronInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_only_at_cron_interval: Option<bool>,
    #[serde(rename = "AssociationDispatchAssumeRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_dispatch_assume_role: Option<String>,
    #[serde(rename = "AssociationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_id: Option<String>,
    #[serde(rename = "AssociationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_name: Option<String>,
    #[serde(rename = "AssociationVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_version: Option<String>,
    #[serde(rename = "AutomationTargetParameterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automation_target_parameter_name: Option<String>,
    #[serde(rename = "CalendarNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calendar_names: Option<Vec<String>>,
    #[serde(rename = "ComplianceSeverity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_severity: Option<String>,
    #[serde(rename = "Date")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<f64>,
    #[serde(rename = "DocumentVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    #[serde(rename = "Duration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(rename = "LastExecutionDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_execution_date: Option<f64>,
    #[serde(rename = "LastSuccessfulExecutionDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_successful_execution_date: Option<f64>,
    #[serde(rename = "LastUpdateAssociationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_association_date: Option<f64>,
    #[serde(rename = "MaxConcurrency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrency: Option<String>,
    #[serde(rename = "MaxErrors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_errors: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "OutputLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_location: Option<InstanceAssociationOutputLocation>,
    #[serde(rename = "Overview")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overview: Option<AssociationOverview>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "ScheduleExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_expression: Option<String>,
    #[serde(rename = "ScheduleOffset")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_offset: Option<i32>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<AssociationStatus>,
    #[serde(rename = "SyncCompliance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_compliance: Option<String>,
    #[serde(rename = "TargetLocations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_locations: Option<Vec<TargetLocation>>,
    #[serde(rename = "TargetMaps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_maps: Option<Vec<std::collections::HashMap<String, Vec<String>>>>,
    #[serde(rename = "Targets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
    #[serde(rename = "TriggeredAlarms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggered_alarms: Option<Vec<AlarmStateInformation>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociationOverview {
    #[serde(rename = "AssociationStatusAggregatedCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_status_aggregated_count: Option<std::collections::HashMap<String, i32>>,
    #[serde(rename = "DetailedStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detailed_status: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociationStatus {
    #[serde(rename = "AdditionalInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<String>,
    #[serde(rename = "Date")]
    #[serde(default)]
    pub date: f64,
    #[serde(rename = "Message")]
    #[serde(default)]
    pub message: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AlarmStateInformation {
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
pub struct CreateAssociationRequest {
    #[serde(rename = "AlarmConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_configuration: Option<AlarmConfiguration>,
    #[serde(rename = "ApplyOnlyAtCronInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_only_at_cron_interval: Option<bool>,
    #[serde(rename = "AssociationDispatchAssumeRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_dispatch_assume_role: Option<String>,
    #[serde(rename = "AssociationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_name: Option<String>,
    #[serde(rename = "AutomationTargetParameterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automation_target_parameter_name: Option<String>,
    #[serde(rename = "CalendarNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calendar_names: Option<Vec<String>>,
    #[serde(rename = "ComplianceSeverity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_severity: Option<String>,
    #[serde(rename = "DocumentVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    #[serde(rename = "Duration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(rename = "MaxConcurrency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrency: Option<String>,
    #[serde(rename = "MaxErrors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_errors: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "OutputLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_location: Option<InstanceAssociationOutputLocation>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "ScheduleExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_expression: Option<String>,
    #[serde(rename = "ScheduleOffset")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_offset: Option<i32>,
    #[serde(rename = "SyncCompliance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_compliance: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "TargetLocations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_locations: Option<Vec<TargetLocation>>,
    #[serde(rename = "TargetMaps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_maps: Option<Vec<std::collections::HashMap<String, Vec<String>>>>,
    #[serde(rename = "Targets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAssociationResult {
    #[serde(rename = "AssociationDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_description: Option<AssociationDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDocumentRequest {
    #[serde(rename = "Attachments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<AttachmentsSource>>,
    #[serde(rename = "Content")]
    #[serde(default)]
    pub content: String,
    #[serde(rename = "DisplayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "DocumentFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_format: Option<String>,
    #[serde(rename = "DocumentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_type: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Requires")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requires: Option<Vec<DocumentRequires>>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "TargetType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,
    #[serde(rename = "VersionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttachmentsSource {
    #[serde(rename = "Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DocumentRequires {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "RequireType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_type: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "VersionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDocumentResult {
    #[serde(rename = "DocumentDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_description: Option<DocumentDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DocumentDescription {
    #[serde(rename = "ApprovedVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_version: Option<String>,
    #[serde(rename = "AttachmentsInformation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments_information: Option<Vec<AttachmentInformation>>,
    #[serde(rename = "Author")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(rename = "Category")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<Vec<String>>,
    #[serde(rename = "CategoryEnum")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_enum: Option<Vec<String>>,
    #[serde(rename = "CreatedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    #[serde(rename = "DefaultVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_version: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "DisplayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "DocumentFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_format: Option<String>,
    #[serde(rename = "DocumentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_type: Option<String>,
    #[serde(rename = "DocumentVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    #[serde(rename = "Hash")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[serde(rename = "HashType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash_type: Option<String>,
    #[serde(rename = "LatestVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Owner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<DocumentParameter>>,
    #[serde(rename = "PendingReviewVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_review_version: Option<String>,
    #[serde(rename = "PlatformTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_types: Option<Vec<String>>,
    #[serde(rename = "Requires")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requires: Option<Vec<DocumentRequires>>,
    #[serde(rename = "ReviewInformation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_information: Option<Vec<ReviewInformation>>,
    #[serde(rename = "ReviewStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_status: Option<String>,
    #[serde(rename = "SchemaVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<String>,
    #[serde(rename = "Sha1")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha1: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusInformation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_information: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "TargetType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,
    #[serde(rename = "VersionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttachmentInformation {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DocumentParameter {
    #[serde(rename = "DefaultValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
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
pub struct ReviewInformation {
    #[serde(rename = "ReviewedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reviewed_time: Option<f64>,
    #[serde(rename = "Reviewer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reviewer: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateMaintenanceWindowRequest {
    #[serde(rename = "AllowUnassociatedTargets")]
    #[serde(default)]
    pub allow_unassociated_targets: bool,
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "Cutoff")]
    #[serde(default)]
    pub cutoff: i32,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Duration")]
    #[serde(default)]
    pub duration: i32,
    #[serde(rename = "EndDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Schedule")]
    #[serde(default)]
    pub schedule: String,
    #[serde(rename = "ScheduleOffset")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_offset: Option<i32>,
    #[serde(rename = "ScheduleTimezone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_timezone: Option<String>,
    #[serde(rename = "StartDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateMaintenanceWindowResult {
    #[serde(rename = "WindowId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateOpsItemRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "ActualEndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_end_time: Option<f64>,
    #[serde(rename = "ActualStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_start_time: Option<f64>,
    #[serde(rename = "Category")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    pub description: String,
    #[serde(rename = "Notifications")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notifications: Option<Vec<OpsItemNotification>>,
    #[serde(rename = "OperationalData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operational_data: Option<std::collections::HashMap<String, OpsItemDataValue>>,
    #[serde(rename = "OpsItemType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ops_item_type: Option<String>,
    #[serde(rename = "PlannedEndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub planned_end_time: Option<f64>,
    #[serde(rename = "PlannedStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub planned_start_time: Option<f64>,
    #[serde(rename = "Priority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(rename = "RelatedOpsItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_ops_items: Option<Vec<RelatedOpsItem>>,
    #[serde(rename = "Severity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    #[serde(rename = "Source")]
    #[serde(default)]
    pub source: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "Title")]
    #[serde(default)]
    pub title: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OpsItemNotification {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OpsItemDataValue {
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RelatedOpsItem {
    #[serde(rename = "OpsItemId")]
    #[serde(default)]
    pub ops_item_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateOpsItemResponse {
    #[serde(rename = "OpsItemArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ops_item_arn: Option<String>,
    #[serde(rename = "OpsItemId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ops_item_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateOpsMetadataRequest {
    #[serde(rename = "Metadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, MetadataValue>>,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MetadataValue {
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateOpsMetadataResult {
    #[serde(rename = "OpsMetadataArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ops_metadata_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePatchBaselineRequest {
    #[serde(rename = "ApprovalRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_rules: Option<PatchRuleGroup>,
    #[serde(rename = "ApprovedPatches")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_patches: Option<Vec<String>>,
    #[serde(rename = "ApprovedPatchesComplianceLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_patches_compliance_level: Option<String>,
    #[serde(rename = "ApprovedPatchesEnableNonSecurity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_patches_enable_non_security: Option<bool>,
    #[serde(rename = "AvailableSecurityUpdatesComplianceStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_security_updates_compliance_status: Option<String>,
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "GlobalFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_filters: Option<PatchFilterGroup>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "OperatingSystem")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<String>,
    #[serde(rename = "RejectedPatches")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejected_patches: Option<Vec<String>>,
    #[serde(rename = "RejectedPatchesAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejected_patches_action: Option<String>,
    #[serde(rename = "Sources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<PatchSource>>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PatchRuleGroup {
    #[serde(rename = "PatchRules")]
    #[serde(default)]
    pub patch_rules: Vec<PatchRule>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PatchRule {
    #[serde(rename = "ApproveAfterDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approve_after_days: Option<i32>,
    #[serde(rename = "ApproveUntilDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approve_until_date: Option<String>,
    #[serde(rename = "ComplianceLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_level: Option<String>,
    #[serde(rename = "EnableNonSecurity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_non_security: Option<bool>,
    #[serde(rename = "PatchFilterGroup")]
    #[serde(default)]
    pub patch_filter_group: PatchFilterGroup,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PatchFilterGroup {
    #[serde(rename = "PatchFilters")]
    #[serde(default)]
    pub patch_filters: Vec<PatchFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PatchFilter {
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "Values")]
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PatchSource {
    #[serde(rename = "Configuration")]
    #[serde(default)]
    pub configuration: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Products")]
    #[serde(default)]
    pub products: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePatchBaselineResult {
    #[serde(rename = "BaselineId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateResourceDataSyncRequest {
    #[serde(rename = "S3Destination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_destination: Option<ResourceDataSyncS3Destination>,
    #[serde(rename = "SyncName")]
    #[serde(default)]
    pub sync_name: String,
    #[serde(rename = "SyncSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_source: Option<ResourceDataSyncSource>,
    #[serde(rename = "SyncType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceDataSyncS3Destination {
    #[serde(rename = "AWSKMSKeyARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_w_s_k_m_s_key_a_r_n: Option<String>,
    #[serde(rename = "BucketName")]
    #[serde(default)]
    pub bucket_name: String,
    #[serde(rename = "DestinationDataSharing")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_data_sharing: Option<ResourceDataSyncDestinationDataSharing>,
    #[serde(rename = "Prefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "Region")]
    #[serde(default)]
    pub region: String,
    #[serde(rename = "SyncFormat")]
    #[serde(default)]
    pub sync_format: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceDataSyncDestinationDataSharing {
    #[serde(rename = "DestinationDataSharingType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_data_sharing_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceDataSyncSource {
    #[serde(rename = "AwsOrganizationsSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_organizations_source: Option<ResourceDataSyncAwsOrganizationsSource>,
    #[serde(rename = "EnableAllOpsDataSources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_all_ops_data_sources: Option<bool>,
    #[serde(rename = "IncludeFutureRegions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_future_regions: Option<bool>,
    #[serde(rename = "SourceRegions")]
    #[serde(default)]
    pub source_regions: Vec<String>,
    #[serde(rename = "SourceType")]
    #[serde(default)]
    pub source_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceDataSyncAwsOrganizationsSource {
    #[serde(rename = "OrganizationSourceType")]
    #[serde(default)]
    pub organization_source_type: String,
    #[serde(rename = "OrganizationalUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizational_units: Option<Vec<ResourceDataSyncOrganizationalUnit>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceDataSyncOrganizationalUnit {
    #[serde(rename = "OrganizationalUnitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizational_unit_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateResourceDataSyncResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteActivationRequest {
    #[serde(rename = "ActivationId")]
    #[serde(default)]
    pub activation_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteActivationResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAssociationRequest {
    #[serde(rename = "AssociationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_id: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAssociationResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDocumentRequest {
    #[serde(rename = "DocumentVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    #[serde(rename = "Force")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "VersionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDocumentResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteInventoryRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "DryRun")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    #[serde(rename = "SchemaDeleteOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_delete_option: Option<String>,
    #[serde(rename = "TypeName")]
    #[serde(default)]
    pub type_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteInventoryResult {
    #[serde(rename = "DeletionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_id: Option<String>,
    #[serde(rename = "DeletionSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_summary: Option<InventoryDeletionSummary>,
    #[serde(rename = "TypeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InventoryDeletionSummary {
    #[serde(rename = "RemainingCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining_count: Option<i32>,
    #[serde(rename = "SummaryItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary_items: Option<Vec<InventoryDeletionSummaryItem>>,
    #[serde(rename = "TotalCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InventoryDeletionSummaryItem {
    #[serde(rename = "Count")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(rename = "RemainingCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining_count: Option<i32>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteMaintenanceWindowRequest {
    #[serde(rename = "WindowId")]
    #[serde(default)]
    pub window_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteMaintenanceWindowResult {
    #[serde(rename = "WindowId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteOpsItemRequest {
    #[serde(rename = "OpsItemId")]
    #[serde(default)]
    pub ops_item_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteOpsItemResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteOpsMetadataRequest {
    #[serde(rename = "OpsMetadataArn")]
    #[serde(default)]
    pub ops_metadata_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteOpsMetadataResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteParameterRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteParameterResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteParametersRequest {
    #[serde(rename = "Names")]
    #[serde(default)]
    pub names: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteParametersResult {
    #[serde(rename = "DeletedParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_parameters: Option<Vec<String>>,
    #[serde(rename = "InvalidParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalid_parameters: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePatchBaselineRequest {
    #[serde(rename = "BaselineId")]
    #[serde(default)]
    pub baseline_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePatchBaselineResult {
    #[serde(rename = "BaselineId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteResourceDataSyncRequest {
    #[serde(rename = "SyncName")]
    #[serde(default)]
    pub sync_name: String,
    #[serde(rename = "SyncType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteResourceDataSyncResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteResourcePolicyRequest {
    #[serde(rename = "PolicyHash")]
    #[serde(default)]
    pub policy_hash: String,
    #[serde(rename = "PolicyId")]
    #[serde(default)]
    pub policy_id: String,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteResourcePolicyResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeregisterManagedInstanceRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeregisterManagedInstanceResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeregisterPatchBaselineForPatchGroupRequest {
    #[serde(rename = "BaselineId")]
    #[serde(default)]
    pub baseline_id: String,
    #[serde(rename = "PatchGroup")]
    #[serde(default)]
    pub patch_group: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeregisterPatchBaselineForPatchGroupResult {
    #[serde(rename = "BaselineId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_id: Option<String>,
    #[serde(rename = "PatchGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_group: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeregisterTargetFromMaintenanceWindowRequest {
    #[serde(rename = "Safe")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safe: Option<bool>,
    #[serde(rename = "WindowId")]
    #[serde(default)]
    pub window_id: String,
    #[serde(rename = "WindowTargetId")]
    #[serde(default)]
    pub window_target_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeregisterTargetFromMaintenanceWindowResult {
    #[serde(rename = "WindowId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_id: Option<String>,
    #[serde(rename = "WindowTargetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_target_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeregisterTaskFromMaintenanceWindowRequest {
    #[serde(rename = "WindowId")]
    #[serde(default)]
    pub window_id: String,
    #[serde(rename = "WindowTaskId")]
    #[serde(default)]
    pub window_task_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeregisterTaskFromMaintenanceWindowResult {
    #[serde(rename = "WindowId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_id: Option<String>,
    #[serde(rename = "WindowTaskId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_task_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeActivationsRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<DescribeActivationsFilter>>,
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
pub struct DescribeActivationsFilter {
    #[serde(rename = "FilterKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_key: Option<String>,
    #[serde(rename = "FilterValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeActivationsResult {
    #[serde(rename = "ActivationList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activation_list: Option<Vec<Activation>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Activation {
    #[serde(rename = "ActivationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activation_id: Option<String>,
    #[serde(rename = "CreatedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    #[serde(rename = "DefaultInstanceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_instance_name: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ExpirationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<f64>,
    #[serde(rename = "Expired")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired: Option<bool>,
    #[serde(rename = "IamRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role: Option<String>,
    #[serde(rename = "RegistrationLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_limit: Option<i32>,
    #[serde(rename = "RegistrationsCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registrations_count: Option<i32>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAssociationExecutionTargetsRequest {
    #[serde(rename = "AssociationId")]
    #[serde(default)]
    pub association_id: String,
    #[serde(rename = "ExecutionId")]
    #[serde(default)]
    pub execution_id: String,
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<AssociationExecutionTargetsFilter>>,
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
pub struct AssociationExecutionTargetsFilter {
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAssociationExecutionTargetsResult {
    #[serde(rename = "AssociationExecutionTargets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_execution_targets: Option<Vec<AssociationExecutionTarget>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociationExecutionTarget {
    #[serde(rename = "AssociationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_id: Option<String>,
    #[serde(rename = "AssociationVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_version: Option<String>,
    #[serde(rename = "DetailedStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detailed_status: Option<String>,
    #[serde(rename = "ExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_id: Option<String>,
    #[serde(rename = "LastExecutionDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_execution_date: Option<f64>,
    #[serde(rename = "OutputSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_source: Option<OutputSource>,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OutputSource {
    #[serde(rename = "OutputSourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_source_id: Option<String>,
    #[serde(rename = "OutputSourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_source_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAssociationExecutionsRequest {
    #[serde(rename = "AssociationId")]
    #[serde(default)]
    pub association_id: String,
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<AssociationExecutionFilter>>,
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
pub struct AssociationExecutionFilter {
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAssociationExecutionsResult {
    #[serde(rename = "AssociationExecutions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_executions: Option<Vec<AssociationExecution>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociationExecution {
    #[serde(rename = "AlarmConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_configuration: Option<AlarmConfiguration>,
    #[serde(rename = "AssociationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_id: Option<String>,
    #[serde(rename = "AssociationVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_version: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "DetailedStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detailed_status: Option<String>,
    #[serde(rename = "ExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_id: Option<String>,
    #[serde(rename = "LastExecutionDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_execution_date: Option<f64>,
    #[serde(rename = "ResourceCountByStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_count_by_status: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TriggeredAlarms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggered_alarms: Option<Vec<AlarmStateInformation>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAssociationRequest {
    #[serde(rename = "AssociationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_id: Option<String>,
    #[serde(rename = "AssociationVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_version: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAssociationResult {
    #[serde(rename = "AssociationDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_description: Option<AssociationDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAutomationExecutionsRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<AutomationExecutionFilter>>,
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
pub struct AutomationExecutionFilter {
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "Values")]
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAutomationExecutionsResult {
    #[serde(rename = "AutomationExecutionMetadataList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automation_execution_metadata_list: Option<Vec<AutomationExecutionMetadata>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomationExecutionMetadata {
    #[serde(rename = "AlarmConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_configuration: Option<AlarmConfiguration>,
    #[serde(rename = "AssociationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_id: Option<String>,
    #[serde(rename = "AutomationExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automation_execution_id: Option<String>,
    #[serde(rename = "AutomationExecutionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automation_execution_status: Option<String>,
    #[serde(rename = "AutomationSubtype")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automation_subtype: Option<String>,
    #[serde(rename = "AutomationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automation_type: Option<String>,
    #[serde(rename = "ChangeRequestName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_request_name: Option<String>,
    #[serde(rename = "CurrentAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_action: Option<String>,
    #[serde(rename = "CurrentStepName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_step_name: Option<String>,
    #[serde(rename = "DocumentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_name: Option<String>,
    #[serde(rename = "DocumentVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    #[serde(rename = "ExecutedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executed_by: Option<String>,
    #[serde(rename = "ExecutionEndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_end_time: Option<f64>,
    #[serde(rename = "ExecutionStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_start_time: Option<f64>,
    #[serde(rename = "FailureMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
    #[serde(rename = "LogFile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_file: Option<String>,
    #[serde(rename = "MaxConcurrency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrency: Option<String>,
    #[serde(rename = "MaxErrors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_errors: Option<String>,
    #[serde(rename = "Mode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(rename = "OpsItemId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ops_item_id: Option<String>,
    #[serde(rename = "Outputs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "ParentAutomationExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_automation_execution_id: Option<String>,
    #[serde(rename = "ResolvedTargets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_targets: Option<ResolvedTargets>,
    #[serde(rename = "Runbooks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runbooks: Option<Vec<Runbook>>,
    #[serde(rename = "ScheduledTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_time: Option<f64>,
    #[serde(rename = "Target")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(rename = "TargetLocationsURL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_locations_u_r_l: Option<String>,
    #[serde(rename = "TargetMaps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_maps: Option<Vec<std::collections::HashMap<String, Vec<String>>>>,
    #[serde(rename = "TargetParameterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_parameter_name: Option<String>,
    #[serde(rename = "Targets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
    #[serde(rename = "TriggeredAlarms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggered_alarms: Option<Vec<AlarmStateInformation>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResolvedTargets {
    #[serde(rename = "ParameterValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_values: Option<Vec<String>>,
    #[serde(rename = "Truncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncated: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Runbook {
    #[serde(rename = "DocumentName")]
    #[serde(default)]
    pub document_name: String,
    #[serde(rename = "DocumentVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    #[serde(rename = "MaxConcurrency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrency: Option<String>,
    #[serde(rename = "MaxErrors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_errors: Option<String>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "TargetLocations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_locations: Option<Vec<TargetLocation>>,
    #[serde(rename = "TargetMaps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_maps: Option<Vec<std::collections::HashMap<String, Vec<String>>>>,
    #[serde(rename = "TargetParameterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_parameter_name: Option<String>,
    #[serde(rename = "Targets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAutomationStepExecutionsRequest {
    #[serde(rename = "AutomationExecutionId")]
    #[serde(default)]
    pub automation_execution_id: String,
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<StepExecutionFilter>>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ReverseOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_order: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StepExecutionFilter {
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "Values")]
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAutomationStepExecutionsResult {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StepExecutions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_executions: Option<Vec<StepExecution>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StepExecution {
    #[serde(rename = "Action")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "ExecutionEndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_end_time: Option<f64>,
    #[serde(rename = "ExecutionStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_start_time: Option<f64>,
    #[serde(rename = "FailureDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_details: Option<FailureDetails>,
    #[serde(rename = "FailureMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
    #[serde(rename = "Inputs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "IsCritical")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_critical: Option<bool>,
    #[serde(rename = "IsEnd")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_end: Option<bool>,
    #[serde(rename = "MaxAttempts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_attempts: Option<i32>,
    #[serde(rename = "NextStep")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_step: Option<String>,
    #[serde(rename = "OnFailure")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_failure: Option<String>,
    #[serde(rename = "Outputs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "OverriddenParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overridden_parameters: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "ParentStepDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_step_details: Option<ParentStepDetails>,
    #[serde(rename = "Response")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<String>,
    #[serde(rename = "ResponseCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_code: Option<String>,
    #[serde(rename = "StepExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_execution_id: Option<String>,
    #[serde(rename = "StepName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_name: Option<String>,
    #[serde(rename = "StepStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_status: Option<String>,
    #[serde(rename = "TargetLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_location: Option<TargetLocation>,
    #[serde(rename = "Targets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
    #[serde(rename = "TimeoutSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<i64>,
    #[serde(rename = "TriggeredAlarms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggered_alarms: Option<Vec<AlarmStateInformation>>,
    #[serde(rename = "ValidNextSteps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_next_steps: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FailureDetails {
    #[serde(rename = "Details")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "FailureStage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_stage: Option<String>,
    #[serde(rename = "FailureType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ParentStepDetails {
    #[serde(rename = "Action")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "Iteration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iteration: Option<i32>,
    #[serde(rename = "IteratorValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iterator_value: Option<String>,
    #[serde(rename = "StepExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_execution_id: Option<String>,
    #[serde(rename = "StepName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAvailablePatchesRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<PatchOrchestratorFilter>>,
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
pub struct PatchOrchestratorFilter {
    #[serde(rename = "Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAvailablePatchesResult {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Patches")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patches: Option<Vec<Patch>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Patch {
    #[serde(rename = "AdvisoryIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advisory_ids: Option<Vec<String>>,
    #[serde(rename = "Arch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arch: Option<String>,
    #[serde(rename = "BugzillaIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bugzilla_ids: Option<Vec<String>>,
    #[serde(rename = "CVEIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_v_e_ids: Option<Vec<String>>,
    #[serde(rename = "Classification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification: Option<String>,
    #[serde(rename = "ContentUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_url: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Epoch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub epoch: Option<i32>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "KbNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kb_number: Option<String>,
    #[serde(rename = "Language")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(rename = "MsrcNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msrc_number: Option<String>,
    #[serde(rename = "MsrcSeverity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msrc_severity: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Product")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    #[serde(rename = "ProductFamily")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_family: Option<String>,
    #[serde(rename = "Release")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release: Option<String>,
    #[serde(rename = "ReleaseDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_date: Option<f64>,
    #[serde(rename = "Repository")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
    #[serde(rename = "Severity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "Vendor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDocumentPermissionRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PermissionType")]
    #[serde(default)]
    pub permission_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDocumentPermissionResponse {
    #[serde(rename = "AccountIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    #[serde(rename = "AccountSharingInfoList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_sharing_info_list: Option<Vec<AccountSharingInfo>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccountSharingInfo {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "SharedDocumentVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_document_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDocumentRequest {
    #[serde(rename = "DocumentVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "VersionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDocumentResult {
    #[serde(rename = "Document")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<DocumentDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeEffectiveInstanceAssociationsRequest {
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
pub struct DescribeEffectiveInstanceAssociationsResult {
    #[serde(rename = "Associations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associations: Option<Vec<InstanceAssociation>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceAssociation {
    #[serde(rename = "AssociationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_id: Option<String>,
    #[serde(rename = "AssociationVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_version: Option<String>,
    #[serde(rename = "Content")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeEffectivePatchesForPatchBaselineRequest {
    #[serde(rename = "BaselineId")]
    #[serde(default)]
    pub baseline_id: String,
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
pub struct DescribeEffectivePatchesForPatchBaselineResult {
    #[serde(rename = "EffectivePatches")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_patches: Option<Vec<EffectivePatch>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EffectivePatch {
    #[serde(rename = "Patch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch: Option<Patch>,
    #[serde(rename = "PatchStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_status: Option<PatchStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PatchStatus {
    #[serde(rename = "ApprovalDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_date: Option<f64>,
    #[serde(rename = "ComplianceLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_level: Option<String>,
    #[serde(rename = "DeploymentStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeInstanceAssociationsStatusRequest {
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
pub struct DescribeInstanceAssociationsStatusResult {
    #[serde(rename = "InstanceAssociationStatusInfos")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_association_status_infos: Option<Vec<InstanceAssociationStatusInfo>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceAssociationStatusInfo {
    #[serde(rename = "AssociationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_id: Option<String>,
    #[serde(rename = "AssociationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_name: Option<String>,
    #[serde(rename = "AssociationVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_version: Option<String>,
    #[serde(rename = "DetailedStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detailed_status: Option<String>,
    #[serde(rename = "DocumentVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "ExecutionDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_date: Option<f64>,
    #[serde(rename = "ExecutionSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_summary: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "OutputUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_url: Option<InstanceAssociationOutputUrl>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceAssociationOutputUrl {
    #[serde(rename = "S3OutputUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_output_url: Option<S3OutputUrl>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3OutputUrl {
    #[serde(rename = "OutputUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeInstanceInformationRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<InstanceInformationStringFilter>>,
    #[serde(rename = "InstanceInformationFilterList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_information_filter_list: Option<Vec<InstanceInformationFilter>>,
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
pub struct InstanceInformationStringFilter {
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "Values")]
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceInformationFilter {
    #[serde(default)]
    pub key: String,
    #[serde(rename = "valueSet")]
    #[serde(default)]
    pub value_set: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeInstanceInformationResult {
    #[serde(rename = "InstanceInformationList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_information_list: Option<Vec<InstanceInformation>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceInformation {
    #[serde(rename = "ActivationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activation_id: Option<String>,
    #[serde(rename = "AgentVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    #[serde(rename = "AssociationOverview")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_overview: Option<InstanceAggregatedAssociationOverview>,
    #[serde(rename = "AssociationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_status: Option<String>,
    #[serde(rename = "ComputerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub computer_name: Option<String>,
    #[serde(rename = "IPAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_p_address: Option<String>,
    #[serde(rename = "IamRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(rename = "IsLatestVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_latest_version: Option<bool>,
    #[serde(rename = "LastAssociationExecutionDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_association_execution_date: Option<f64>,
    #[serde(rename = "LastPingDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_ping_date_time: Option<f64>,
    #[serde(rename = "LastSuccessfulAssociationExecutionDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_successful_association_execution_date: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "PingStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ping_status: Option<String>,
    #[serde(rename = "PlatformName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_name: Option<String>,
    #[serde(rename = "PlatformType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_type: Option<String>,
    #[serde(rename = "PlatformVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_version: Option<String>,
    #[serde(rename = "RegistrationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_date: Option<f64>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "SourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    #[serde(rename = "SourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceAggregatedAssociationOverview {
    #[serde(rename = "DetailedStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detailed_status: Option<String>,
    #[serde(rename = "InstanceAssociationStatusAggregatedCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_association_status_aggregated_count:
        Option<std::collections::HashMap<String, i32>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeInstancePatchStatesForPatchGroupRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<InstancePatchStateFilter>>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PatchGroup")]
    #[serde(default)]
    pub patch_group: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstancePatchStateFilter {
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
    #[serde(rename = "Values")]
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeInstancePatchStatesForPatchGroupResult {
    #[serde(rename = "InstancePatchStates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_patch_states: Option<Vec<InstancePatchState>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstancePatchState {
    #[serde(rename = "AvailableSecurityUpdateCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_security_update_count: Option<i32>,
    #[serde(rename = "BaselineId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_id: Option<String>,
    #[serde(rename = "CriticalNonCompliantCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub critical_non_compliant_count: Option<i32>,
    #[serde(rename = "FailedCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_count: Option<i32>,
    #[serde(rename = "InstallOverrideList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub install_override_list: Option<String>,
    #[serde(rename = "InstalledCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installed_count: Option<i32>,
    #[serde(rename = "InstalledOtherCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installed_other_count: Option<i32>,
    #[serde(rename = "InstalledPendingRebootCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installed_pending_reboot_count: Option<i32>,
    #[serde(rename = "InstalledRejectedCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installed_rejected_count: Option<i32>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(rename = "LastNoRebootInstallOperationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_no_reboot_install_operation_time: Option<f64>,
    #[serde(rename = "MissingCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_count: Option<i32>,
    #[serde(rename = "NotApplicableCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_applicable_count: Option<i32>,
    #[serde(rename = "Operation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(rename = "OperationEndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_end_time: Option<f64>,
    #[serde(rename = "OperationStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_start_time: Option<f64>,
    #[serde(rename = "OtherNonCompliantCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_non_compliant_count: Option<i32>,
    #[serde(rename = "OwnerInformation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_information: Option<String>,
    #[serde(rename = "PatchGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_group: Option<String>,
    #[serde(rename = "RebootOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reboot_option: Option<String>,
    #[serde(rename = "SecurityNonCompliantCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_non_compliant_count: Option<i32>,
    #[serde(rename = "SnapshotId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
    #[serde(rename = "UnreportedNotApplicableCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unreported_not_applicable_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeInstancePatchStatesRequest {
    #[serde(rename = "InstanceIds")]
    #[serde(default)]
    pub instance_ids: Vec<String>,
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
pub struct DescribeInstancePatchStatesResult {
    #[serde(rename = "InstancePatchStates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_patch_states: Option<Vec<InstancePatchState>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeInstancePatchesRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<PatchOrchestratorFilter>>,
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
pub struct DescribeInstancePatchesResult {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Patches")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patches: Option<Vec<PatchComplianceData>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PatchComplianceData {
    #[serde(rename = "CVEIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_v_e_ids: Option<String>,
    #[serde(rename = "Classification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification: Option<String>,
    #[serde(rename = "InstalledTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installed_time: Option<f64>,
    #[serde(rename = "KBId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k_b_id: Option<String>,
    #[serde(rename = "Severity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeInstancePropertiesRequest {
    #[serde(rename = "FiltersWithOperator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters_with_operator: Option<Vec<InstancePropertyStringFilter>>,
    #[serde(rename = "InstancePropertyFilterList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_property_filter_list: Option<Vec<InstancePropertyFilter>>,
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
pub struct InstancePropertyStringFilter {
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "Operator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    #[serde(rename = "Values")]
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstancePropertyFilter {
    #[serde(default)]
    pub key: String,
    #[serde(rename = "valueSet")]
    #[serde(default)]
    pub value_set: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeInstancePropertiesResult {
    #[serde(rename = "InstanceProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_properties: Option<Vec<InstanceProperty>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceProperty {
    #[serde(rename = "ActivationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activation_id: Option<String>,
    #[serde(rename = "AgentVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    #[serde(rename = "Architecture")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    #[serde(rename = "AssociationOverview")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_overview: Option<InstanceAggregatedAssociationOverview>,
    #[serde(rename = "AssociationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_status: Option<String>,
    #[serde(rename = "ComputerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub computer_name: Option<String>,
    #[serde(rename = "IPAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_p_address: Option<String>,
    #[serde(rename = "IamRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(rename = "InstanceRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_role: Option<String>,
    #[serde(rename = "InstanceState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_state: Option<String>,
    #[serde(rename = "InstanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    #[serde(rename = "KeyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_name: Option<String>,
    #[serde(rename = "LastAssociationExecutionDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_association_execution_date: Option<f64>,
    #[serde(rename = "LastPingDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_ping_date_time: Option<f64>,
    #[serde(rename = "LastSuccessfulAssociationExecutionDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_successful_association_execution_date: Option<f64>,
    #[serde(rename = "LaunchTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "PingStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ping_status: Option<String>,
    #[serde(rename = "PlatformName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_name: Option<String>,
    #[serde(rename = "PlatformType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_type: Option<String>,
    #[serde(rename = "PlatformVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_version: Option<String>,
    #[serde(rename = "RegistrationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_date: Option<f64>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "SourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    #[serde(rename = "SourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeInventoryDeletionsRequest {
    #[serde(rename = "DeletionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_id: Option<String>,
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
pub struct DescribeInventoryDeletionsResult {
    #[serde(rename = "InventoryDeletions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory_deletions: Option<Vec<InventoryDeletionStatusItem>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InventoryDeletionStatusItem {
    #[serde(rename = "DeletionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_id: Option<String>,
    #[serde(rename = "DeletionStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_start_time: Option<f64>,
    #[serde(rename = "DeletionSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_summary: Option<InventoryDeletionSummary>,
    #[serde(rename = "LastStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_status: Option<String>,
    #[serde(rename = "LastStatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_status_message: Option<String>,
    #[serde(rename = "LastStatusUpdateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_status_update_time: Option<f64>,
    #[serde(rename = "TypeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeMaintenanceWindowExecutionTaskInvocationsRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<MaintenanceWindowFilter>>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TaskId")]
    #[serde(default)]
    pub task_id: String,
    #[serde(rename = "WindowExecutionId")]
    #[serde(default)]
    pub window_execution_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MaintenanceWindowFilter {
    #[serde(rename = "Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeMaintenanceWindowExecutionTaskInvocationsResult {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "WindowExecutionTaskInvocationIdentities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_execution_task_invocation_identities:
        Option<Vec<MaintenanceWindowExecutionTaskInvocationIdentity>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MaintenanceWindowExecutionTaskInvocationIdentity {
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "ExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_id: Option<String>,
    #[serde(rename = "InvocationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invocation_id: Option<String>,
    #[serde(rename = "OwnerInformation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_information: Option<String>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
    #[serde(rename = "TaskExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_execution_id: Option<String>,
    #[serde(rename = "TaskType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_type: Option<String>,
    #[serde(rename = "WindowExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_execution_id: Option<String>,
    #[serde(rename = "WindowTargetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_target_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeMaintenanceWindowExecutionTasksRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<MaintenanceWindowFilter>>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "WindowExecutionId")]
    #[serde(default)]
    pub window_execution_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeMaintenanceWindowExecutionTasksResult {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "WindowExecutionTaskIdentities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_execution_task_identities: Option<Vec<MaintenanceWindowExecutionTaskIdentity>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MaintenanceWindowExecutionTaskIdentity {
    #[serde(rename = "AlarmConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_configuration: Option<AlarmConfiguration>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
    #[serde(rename = "TaskArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_arn: Option<String>,
    #[serde(rename = "TaskExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_execution_id: Option<String>,
    #[serde(rename = "TaskType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_type: Option<String>,
    #[serde(rename = "TriggeredAlarms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggered_alarms: Option<Vec<AlarmStateInformation>>,
    #[serde(rename = "WindowExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_execution_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeMaintenanceWindowExecutionsRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<MaintenanceWindowFilter>>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "WindowId")]
    #[serde(default)]
    pub window_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeMaintenanceWindowExecutionsResult {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "WindowExecutions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_executions: Option<Vec<MaintenanceWindowExecution>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MaintenanceWindowExecution {
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
    #[serde(rename = "WindowExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_execution_id: Option<String>,
    #[serde(rename = "WindowId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeMaintenanceWindowScheduleRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<PatchOrchestratorFilter>>,
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
    #[serde(rename = "Targets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
    #[serde(rename = "WindowId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeMaintenanceWindowScheduleResult {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ScheduledWindowExecutions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_window_executions: Option<Vec<ScheduledWindowExecution>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScheduledWindowExecution {
    #[serde(rename = "ExecutionTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_time: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "WindowId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeMaintenanceWindowTargetsRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<MaintenanceWindowFilter>>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "WindowId")]
    #[serde(default)]
    pub window_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeMaintenanceWindowTargetsResult {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Targets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<MaintenanceWindowTarget>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MaintenanceWindowTarget {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "OwnerInformation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_information: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "Targets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
    #[serde(rename = "WindowId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_id: Option<String>,
    #[serde(rename = "WindowTargetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_target_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeMaintenanceWindowTasksRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<MaintenanceWindowFilter>>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "WindowId")]
    #[serde(default)]
    pub window_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeMaintenanceWindowTasksResult {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Tasks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tasks: Option<Vec<MaintenanceWindowTask>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MaintenanceWindowTask {
    #[serde(rename = "AlarmConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_configuration: Option<AlarmConfiguration>,
    #[serde(rename = "CutoffBehavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cutoff_behavior: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "LoggingInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_info: Option<LoggingInfo>,
    #[serde(rename = "MaxConcurrency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrency: Option<String>,
    #[serde(rename = "MaxErrors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_errors: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Priority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(rename = "ServiceRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role_arn: Option<String>,
    #[serde(rename = "Targets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
    #[serde(rename = "TaskArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_arn: Option<String>,
    #[serde(rename = "TaskParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_parameters:
        Option<std::collections::HashMap<String, MaintenanceWindowTaskParameterValueExpression>>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "WindowId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_id: Option<String>,
    #[serde(rename = "WindowTaskId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_task_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LoggingInfo {
    #[serde(rename = "S3BucketName")]
    #[serde(default)]
    pub s3_bucket_name: String,
    #[serde(rename = "S3KeyPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key_prefix: Option<String>,
    #[serde(rename = "S3Region")]
    #[serde(default)]
    pub s3_region: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MaintenanceWindowTaskParameterValueExpression {
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeMaintenanceWindowsForTargetRequest {
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
    #[serde(rename = "Targets")]
    #[serde(default)]
    pub targets: Vec<Target>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeMaintenanceWindowsForTargetResult {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "WindowIdentities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_identities: Option<Vec<MaintenanceWindowIdentityForTarget>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MaintenanceWindowIdentityForTarget {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "WindowId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeMaintenanceWindowsRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<MaintenanceWindowFilter>>,
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
pub struct DescribeMaintenanceWindowsResult {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "WindowIdentities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_identities: Option<Vec<MaintenanceWindowIdentity>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MaintenanceWindowIdentity {
    #[serde(rename = "Cutoff")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cutoff: Option<i32>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Duration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "EndDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "NextExecutionTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_execution_time: Option<String>,
    #[serde(rename = "Schedule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,
    #[serde(rename = "ScheduleOffset")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_offset: Option<i32>,
    #[serde(rename = "ScheduleTimezone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_timezone: Option<String>,
    #[serde(rename = "StartDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(rename = "WindowId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeOpsItemsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "OpsItemFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ops_item_filters: Option<Vec<OpsItemFilter>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OpsItemFilter {
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "Operator")]
    #[serde(default)]
    pub operator: String,
    #[serde(rename = "Values")]
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeOpsItemsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "OpsItemSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ops_item_summaries: Option<Vec<OpsItemSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OpsItemSummary {
    #[serde(rename = "ActualEndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_end_time: Option<f64>,
    #[serde(rename = "ActualStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_start_time: Option<f64>,
    #[serde(rename = "Category")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "CreatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "LastModifiedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "OperationalData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operational_data: Option<std::collections::HashMap<String, OpsItemDataValue>>,
    #[serde(rename = "OpsItemId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ops_item_id: Option<String>,
    #[serde(rename = "OpsItemType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ops_item_type: Option<String>,
    #[serde(rename = "PlannedEndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub planned_end_time: Option<f64>,
    #[serde(rename = "PlannedStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub planned_start_time: Option<f64>,
    #[serde(rename = "Priority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(rename = "Severity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    #[serde(rename = "Source")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeParametersRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<ParametersFilter>>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ParameterFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_filters: Option<Vec<ParameterStringFilter>>,
    #[serde(rename = "Shared")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ParametersFilter {
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "Values")]
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ParameterStringFilter {
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "Option")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option: Option<String>,
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeParametersResult {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<ParameterMetadata>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ParameterMetadata {
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
    #[serde(rename = "AllowedPattern")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_pattern: Option<String>,
    #[serde(rename = "DataType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "KeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    #[serde(rename = "LastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[serde(rename = "LastModifiedUser")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_user: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Policies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<ParameterInlinePolicy>>,
    #[serde(rename = "Tier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ParameterInlinePolicy {
    #[serde(rename = "PolicyStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_status: Option<String>,
    #[serde(rename = "PolicyText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_text: Option<String>,
    #[serde(rename = "PolicyType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribePatchBaselinesRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<PatchOrchestratorFilter>>,
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
pub struct DescribePatchBaselinesResult {
    #[serde(rename = "BaselineIdentities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_identities: Option<Vec<PatchBaselineIdentity>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PatchBaselineIdentity {
    #[serde(rename = "BaselineDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_description: Option<String>,
    #[serde(rename = "BaselineId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_id: Option<String>,
    #[serde(rename = "BaselineName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_name: Option<String>,
    #[serde(rename = "DefaultBaseline")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_baseline: Option<bool>,
    #[serde(rename = "OperatingSystem")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribePatchGroupStateRequest {
    #[serde(rename = "PatchGroup")]
    #[serde(default)]
    pub patch_group: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribePatchGroupStateResult {
    #[serde(rename = "Instances")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances: Option<i32>,
    #[serde(rename = "InstancesWithAvailableSecurityUpdates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances_with_available_security_updates: Option<i32>,
    #[serde(rename = "InstancesWithCriticalNonCompliantPatches")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances_with_critical_non_compliant_patches: Option<i32>,
    #[serde(rename = "InstancesWithFailedPatches")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances_with_failed_patches: Option<i32>,
    #[serde(rename = "InstancesWithInstalledOtherPatches")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances_with_installed_other_patches: Option<i32>,
    #[serde(rename = "InstancesWithInstalledPatches")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances_with_installed_patches: Option<i32>,
    #[serde(rename = "InstancesWithInstalledPendingRebootPatches")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances_with_installed_pending_reboot_patches: Option<i32>,
    #[serde(rename = "InstancesWithInstalledRejectedPatches")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances_with_installed_rejected_patches: Option<i32>,
    #[serde(rename = "InstancesWithMissingPatches")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances_with_missing_patches: Option<i32>,
    #[serde(rename = "InstancesWithNotApplicablePatches")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances_with_not_applicable_patches: Option<i32>,
    #[serde(rename = "InstancesWithOtherNonCompliantPatches")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances_with_other_non_compliant_patches: Option<i32>,
    #[serde(rename = "InstancesWithSecurityNonCompliantPatches")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances_with_security_non_compliant_patches: Option<i32>,
    #[serde(rename = "InstancesWithUnreportedNotApplicablePatches")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances_with_unreported_not_applicable_patches: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribePatchGroupsRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<PatchOrchestratorFilter>>,
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
pub struct DescribePatchGroupsResult {
    #[serde(rename = "Mappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mappings: Option<Vec<PatchGroupPatchBaselineMapping>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PatchGroupPatchBaselineMapping {
    #[serde(rename = "BaselineIdentity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_identity: Option<PatchBaselineIdentity>,
    #[serde(rename = "PatchGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_group: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribePatchPropertiesRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "OperatingSystem")]
    #[serde(default)]
    pub operating_system: String,
    #[serde(rename = "PatchSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_set: Option<String>,
    #[serde(rename = "Property")]
    #[serde(default)]
    pub property: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribePatchPropertiesResult {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Properties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<std::collections::HashMap<String, String>>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSessionsRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<SessionFilter>>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    pub state: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SessionFilter {
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSessionsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Sessions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sessions: Option<Vec<Session>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Session {
    #[serde(rename = "AccessType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_type: Option<String>,
    #[serde(rename = "Details")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[serde(rename = "DocumentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_name: Option<String>,
    #[serde(rename = "EndDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<f64>,
    #[serde(rename = "MaxSessionDuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_session_duration: Option<String>,
    #[serde(rename = "OutputUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_url: Option<SessionManagerOutputUrl>,
    #[serde(rename = "Owner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(rename = "Reason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "SessionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(rename = "StartDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<f64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Target")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SessionManagerOutputUrl {
    #[serde(rename = "CloudWatchOutputUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_output_url: Option<String>,
    #[serde(rename = "S3OutputUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_output_url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateOpsItemRelatedItemRequest {
    #[serde(rename = "AssociationId")]
    #[serde(default)]
    pub association_id: String,
    #[serde(rename = "OpsItemId")]
    #[serde(default)]
    pub ops_item_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateOpsItemRelatedItemResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAccessTokenRequest {
    #[serde(rename = "AccessRequestId")]
    #[serde(default)]
    pub access_request_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAccessTokenResponse {
    #[serde(rename = "AccessRequestStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_request_status: Option<String>,
    #[serde(rename = "Credentials")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<Credentials>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Credentials {
    #[serde(rename = "AccessKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_id: Option<String>,
    #[serde(rename = "ExpirationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<f64>,
    #[serde(rename = "SecretAccessKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_access_key: Option<String>,
    #[serde(rename = "SessionToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAutomationExecutionRequest {
    #[serde(rename = "AutomationExecutionId")]
    #[serde(default)]
    pub automation_execution_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAutomationExecutionResult {
    #[serde(rename = "AutomationExecution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automation_execution: Option<AutomationExecution>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomationExecution {
    #[serde(rename = "AlarmConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_configuration: Option<AlarmConfiguration>,
    #[serde(rename = "AssociationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_id: Option<String>,
    #[serde(rename = "AutomationExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automation_execution_id: Option<String>,
    #[serde(rename = "AutomationExecutionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automation_execution_status: Option<String>,
    #[serde(rename = "AutomationSubtype")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automation_subtype: Option<String>,
    #[serde(rename = "ChangeRequestName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_request_name: Option<String>,
    #[serde(rename = "CurrentAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_action: Option<String>,
    #[serde(rename = "CurrentStepName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_step_name: Option<String>,
    #[serde(rename = "DocumentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_name: Option<String>,
    #[serde(rename = "DocumentVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    #[serde(rename = "ExecutedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executed_by: Option<String>,
    #[serde(rename = "ExecutionEndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_end_time: Option<f64>,
    #[serde(rename = "ExecutionStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_start_time: Option<f64>,
    #[serde(rename = "FailureMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
    #[serde(rename = "MaxConcurrency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrency: Option<String>,
    #[serde(rename = "MaxErrors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_errors: Option<String>,
    #[serde(rename = "Mode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(rename = "OpsItemId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ops_item_id: Option<String>,
    #[serde(rename = "Outputs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "ParentAutomationExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_automation_execution_id: Option<String>,
    #[serde(rename = "ProgressCounters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_counters: Option<ProgressCounters>,
    #[serde(rename = "ResolvedTargets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_targets: Option<ResolvedTargets>,
    #[serde(rename = "Runbooks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runbooks: Option<Vec<Runbook>>,
    #[serde(rename = "ScheduledTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_time: Option<f64>,
    #[serde(rename = "StepExecutions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_executions: Option<Vec<StepExecution>>,
    #[serde(rename = "StepExecutionsTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_executions_truncated: Option<bool>,
    #[serde(rename = "Target")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(rename = "TargetLocations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_locations: Option<Vec<TargetLocation>>,
    #[serde(rename = "TargetLocationsURL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_locations_u_r_l: Option<String>,
    #[serde(rename = "TargetMaps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_maps: Option<Vec<std::collections::HashMap<String, Vec<String>>>>,
    #[serde(rename = "TargetParameterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_parameter_name: Option<String>,
    #[serde(rename = "Targets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
    #[serde(rename = "TriggeredAlarms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggered_alarms: Option<Vec<AlarmStateInformation>>,
    #[serde(rename = "Variables")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<std::collections::HashMap<String, Vec<String>>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProgressCounters {
    #[serde(rename = "CancelledSteps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancelled_steps: Option<i32>,
    #[serde(rename = "FailedSteps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_steps: Option<i32>,
    #[serde(rename = "SuccessSteps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_steps: Option<i32>,
    #[serde(rename = "TimedOutSteps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timed_out_steps: Option<i32>,
    #[serde(rename = "TotalSteps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_steps: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCalendarStateRequest {
    #[serde(rename = "AtTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at_time: Option<String>,
    #[serde(rename = "CalendarNames")]
    #[serde(default)]
    pub calendar_names: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCalendarStateResponse {
    #[serde(rename = "AtTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at_time: Option<String>,
    #[serde(rename = "NextTransitionTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_transition_time: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCommandInvocationRequest {
    #[serde(rename = "CommandId")]
    #[serde(default)]
    pub command_id: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "PluginName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugin_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCommandInvocationResult {
    #[serde(rename = "CloudWatchOutputConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_output_config: Option<CloudWatchOutputConfig>,
    #[serde(rename = "CommandId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_id: Option<String>,
    #[serde(rename = "Comment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "DocumentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_name: Option<String>,
    #[serde(rename = "DocumentVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    #[serde(rename = "ExecutionElapsedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_elapsed_time: Option<String>,
    #[serde(rename = "ExecutionEndDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_end_date_time: Option<String>,
    #[serde(rename = "ExecutionStartDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_start_date_time: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(rename = "PluginName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugin_name: Option<String>,
    #[serde(rename = "ResponseCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_code: Option<i32>,
    #[serde(rename = "StandardErrorContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard_error_content: Option<String>,
    #[serde(rename = "StandardErrorUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard_error_url: Option<String>,
    #[serde(rename = "StandardOutputContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard_output_content: Option<String>,
    #[serde(rename = "StandardOutputUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard_output_url: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CloudWatchOutputConfig {
    #[serde(rename = "CloudWatchLogGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_log_group_name: Option<String>,
    #[serde(rename = "CloudWatchOutputEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_output_enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetConnectionStatusRequest {
    #[serde(rename = "Target")]
    #[serde(default)]
    pub target: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetConnectionStatusResponse {
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Target")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDefaultPatchBaselineRequest {
    #[serde(rename = "OperatingSystem")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDefaultPatchBaselineResult {
    #[serde(rename = "BaselineId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_id: Option<String>,
    #[serde(rename = "OperatingSystem")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDeployablePatchSnapshotForInstanceRequest {
    #[serde(rename = "BaselineOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_override: Option<BaselineOverride>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "SnapshotId")]
    #[serde(default)]
    pub snapshot_id: String,
    #[serde(rename = "UseS3DualStackEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_s3_dual_stack_endpoint: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BaselineOverride {
    #[serde(rename = "ApprovalRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_rules: Option<PatchRuleGroup>,
    #[serde(rename = "ApprovedPatches")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_patches: Option<Vec<String>>,
    #[serde(rename = "ApprovedPatchesComplianceLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_patches_compliance_level: Option<String>,
    #[serde(rename = "ApprovedPatchesEnableNonSecurity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_patches_enable_non_security: Option<bool>,
    #[serde(rename = "AvailableSecurityUpdatesComplianceStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_security_updates_compliance_status: Option<String>,
    #[serde(rename = "GlobalFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_filters: Option<PatchFilterGroup>,
    #[serde(rename = "OperatingSystem")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<String>,
    #[serde(rename = "RejectedPatches")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejected_patches: Option<Vec<String>>,
    #[serde(rename = "RejectedPatchesAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejected_patches_action: Option<String>,
    #[serde(rename = "Sources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<PatchSource>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDeployablePatchSnapshotForInstanceResult {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(rename = "Product")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    #[serde(rename = "SnapshotDownloadUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_download_url: Option<String>,
    #[serde(rename = "SnapshotId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDocumentRequest {
    #[serde(rename = "DocumentFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_format: Option<String>,
    #[serde(rename = "DocumentVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "VersionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDocumentResult {
    #[serde(rename = "AttachmentsContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments_content: Option<Vec<AttachmentContent>>,
    #[serde(rename = "Content")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(rename = "CreatedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    #[serde(rename = "DisplayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "DocumentFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_format: Option<String>,
    #[serde(rename = "DocumentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_type: Option<String>,
    #[serde(rename = "DocumentVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Requires")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requires: Option<Vec<DocumentRequires>>,
    #[serde(rename = "ReviewStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_status: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusInformation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_information: Option<String>,
    #[serde(rename = "VersionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttachmentContent {
    #[serde(rename = "Hash")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[serde(rename = "HashType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash_type: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Size")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(rename = "Url")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetExecutionPreviewRequest {
    #[serde(rename = "ExecutionPreviewId")]
    #[serde(default)]
    pub execution_preview_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetExecutionPreviewResponse {
    #[serde(rename = "EndedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ended_at: Option<f64>,
    #[serde(rename = "ExecutionPreview")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_preview: Option<ExecutionPreview>,
    #[serde(rename = "ExecutionPreviewId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_preview_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExecutionPreview {
    #[serde(rename = "Automation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automation: Option<AutomationExecutionPreview>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomationExecutionPreview {
    #[serde(rename = "Regions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<String>>,
    #[serde(rename = "StepPreviews")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_previews: Option<std::collections::HashMap<String, i32>>,
    #[serde(rename = "TargetPreviews")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_previews: Option<Vec<TargetPreview>>,
    #[serde(rename = "TotalAccounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_accounts: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TargetPreview {
    #[serde(rename = "Count")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(rename = "TargetType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetInventoryRequest {
    #[serde(rename = "Aggregators")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregators: Option<Vec<InventoryAggregator>>,
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<InventoryFilter>>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResultAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_attributes: Option<Vec<ResultAttribute>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InventoryAggregator {
    #[serde(rename = "Aggregators")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregators: Option<Vec<InventoryAggregator>>,
    #[serde(rename = "Expression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(rename = "Groups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<InventoryGroup>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InventoryGroup {
    #[serde(rename = "Filters")]
    #[serde(default)]
    pub filters: Vec<InventoryFilter>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InventoryFilter {
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Values")]
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResultAttribute {
    #[serde(rename = "TypeName")]
    #[serde(default)]
    pub type_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetInventoryResult {
    #[serde(rename = "Entities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<InventoryResultEntity>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InventoryResultEntity {
    #[serde(rename = "Data")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<std::collections::HashMap<String, InventoryResultItem>>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InventoryResultItem {
    #[serde(rename = "CaptureTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_time: Option<String>,
    #[serde(rename = "Content")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<std::collections::HashMap<String, String>>>,
    #[serde(rename = "ContentHash")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_hash: Option<String>,
    #[serde(rename = "SchemaVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<String>,
    #[serde(rename = "TypeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetInventorySchemaRequest {
    #[serde(rename = "Aggregator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregator: Option<bool>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SubType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_type: Option<bool>,
    #[serde(rename = "TypeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetInventorySchemaResult {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Schemas")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schemas: Option<Vec<InventoryItemSchema>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InventoryItemSchema {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<InventoryItemAttribute>>,
    #[serde(rename = "DisplayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "TypeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InventoryItemAttribute {
    #[serde(rename = "DataType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMaintenanceWindowExecutionRequest {
    #[serde(rename = "WindowExecutionId")]
    #[serde(default)]
    pub window_execution_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMaintenanceWindowExecutionResult {
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
    #[serde(rename = "TaskIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_ids: Option<Vec<String>>,
    #[serde(rename = "WindowExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_execution_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMaintenanceWindowExecutionTaskInvocationRequest {
    #[serde(rename = "InvocationId")]
    #[serde(default)]
    pub invocation_id: String,
    #[serde(rename = "TaskId")]
    #[serde(default)]
    pub task_id: String,
    #[serde(rename = "WindowExecutionId")]
    #[serde(default)]
    pub window_execution_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMaintenanceWindowExecutionTaskInvocationResult {
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "ExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_id: Option<String>,
    #[serde(rename = "InvocationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invocation_id: Option<String>,
    #[serde(rename = "OwnerInformation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_information: Option<String>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
    #[serde(rename = "TaskExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_execution_id: Option<String>,
    #[serde(rename = "TaskType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_type: Option<String>,
    #[serde(rename = "WindowExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_execution_id: Option<String>,
    #[serde(rename = "WindowTargetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_target_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMaintenanceWindowExecutionTaskRequest {
    #[serde(rename = "TaskId")]
    #[serde(default)]
    pub task_id: String,
    #[serde(rename = "WindowExecutionId")]
    #[serde(default)]
    pub window_execution_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMaintenanceWindowExecutionTaskResult {
    #[serde(rename = "AlarmConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_configuration: Option<AlarmConfiguration>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "MaxConcurrency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrency: Option<String>,
    #[serde(rename = "MaxErrors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_errors: Option<String>,
    #[serde(rename = "Priority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(rename = "ServiceRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
    #[serde(rename = "TaskArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_arn: Option<String>,
    #[serde(rename = "TaskExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_execution_id: Option<String>,
    #[serde(rename = "TaskParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_parameters: Option<
        Vec<std::collections::HashMap<String, MaintenanceWindowTaskParameterValueExpression>>,
    >,
    #[serde(rename = "TriggeredAlarms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggered_alarms: Option<Vec<AlarmStateInformation>>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "WindowExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_execution_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMaintenanceWindowRequest {
    #[serde(rename = "WindowId")]
    #[serde(default)]
    pub window_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMaintenanceWindowResult {
    #[serde(rename = "AllowUnassociatedTargets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_unassociated_targets: Option<bool>,
    #[serde(rename = "CreatedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    #[serde(rename = "Cutoff")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cutoff: Option<i32>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Duration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "EndDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(rename = "ModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_date: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "NextExecutionTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_execution_time: Option<String>,
    #[serde(rename = "Schedule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,
    #[serde(rename = "ScheduleOffset")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_offset: Option<i32>,
    #[serde(rename = "ScheduleTimezone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_timezone: Option<String>,
    #[serde(rename = "StartDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(rename = "WindowId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMaintenanceWindowTaskRequest {
    #[serde(rename = "WindowId")]
    #[serde(default)]
    pub window_id: String,
    #[serde(rename = "WindowTaskId")]
    #[serde(default)]
    pub window_task_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMaintenanceWindowTaskResult {
    #[serde(rename = "AlarmConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_configuration: Option<AlarmConfiguration>,
    #[serde(rename = "CutoffBehavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cutoff_behavior: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "LoggingInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_info: Option<LoggingInfo>,
    #[serde(rename = "MaxConcurrency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrency: Option<String>,
    #[serde(rename = "MaxErrors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_errors: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Priority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(rename = "ServiceRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role_arn: Option<String>,
    #[serde(rename = "Targets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
    #[serde(rename = "TaskArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_arn: Option<String>,
    #[serde(rename = "TaskInvocationParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_invocation_parameters: Option<MaintenanceWindowTaskInvocationParameters>,
    #[serde(rename = "TaskParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_parameters:
        Option<std::collections::HashMap<String, MaintenanceWindowTaskParameterValueExpression>>,
    #[serde(rename = "TaskType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_type: Option<String>,
    #[serde(rename = "WindowId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_id: Option<String>,
    #[serde(rename = "WindowTaskId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_task_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MaintenanceWindowTaskInvocationParameters {
    #[serde(rename = "Automation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automation: Option<MaintenanceWindowAutomationParameters>,
    #[serde(rename = "Lambda")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda: Option<MaintenanceWindowLambdaParameters>,
    #[serde(rename = "RunCommand")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_command: Option<MaintenanceWindowRunCommandParameters>,
    #[serde(rename = "StepFunctions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_functions: Option<MaintenanceWindowStepFunctionsParameters>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MaintenanceWindowAutomationParameters {
    #[serde(rename = "DocumentVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, Vec<String>>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MaintenanceWindowLambdaParameters {
    #[serde(rename = "ClientContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_context: Option<String>,
    #[serde(rename = "Payload")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,
    #[serde(rename = "Qualifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MaintenanceWindowRunCommandParameters {
    #[serde(rename = "CloudWatchOutputConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_output_config: Option<CloudWatchOutputConfig>,
    #[serde(rename = "Comment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "DocumentHash")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_hash: Option<String>,
    #[serde(rename = "DocumentHashType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_hash_type: Option<String>,
    #[serde(rename = "DocumentVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    #[serde(rename = "NotificationConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_config: Option<NotificationConfig>,
    #[serde(rename = "OutputS3BucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_s3_bucket_name: Option<String>,
    #[serde(rename = "OutputS3KeyPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_s3_key_prefix: Option<String>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "ServiceRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role_arn: Option<String>,
    #[serde(rename = "TimeoutSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NotificationConfig {
    #[serde(rename = "NotificationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_arn: Option<String>,
    #[serde(rename = "NotificationEvents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_events: Option<Vec<String>>,
    #[serde(rename = "NotificationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MaintenanceWindowStepFunctionsParameters {
    #[serde(rename = "Input")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetOpsItemRequest {
    #[serde(rename = "OpsItemArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ops_item_arn: Option<String>,
    #[serde(rename = "OpsItemId")]
    #[serde(default)]
    pub ops_item_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetOpsItemResponse {
    #[serde(rename = "OpsItem")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ops_item: Option<OpsItem>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OpsItem {
    #[serde(rename = "ActualEndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_end_time: Option<f64>,
    #[serde(rename = "ActualStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_start_time: Option<f64>,
    #[serde(rename = "Category")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
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
    #[serde(rename = "LastModifiedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "Notifications")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notifications: Option<Vec<OpsItemNotification>>,
    #[serde(rename = "OperationalData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operational_data: Option<std::collections::HashMap<String, OpsItemDataValue>>,
    #[serde(rename = "OpsItemArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ops_item_arn: Option<String>,
    #[serde(rename = "OpsItemId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ops_item_id: Option<String>,
    #[serde(rename = "OpsItemType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ops_item_type: Option<String>,
    #[serde(rename = "PlannedEndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub planned_end_time: Option<f64>,
    #[serde(rename = "PlannedStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub planned_start_time: Option<f64>,
    #[serde(rename = "Priority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(rename = "RelatedOpsItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_ops_items: Option<Vec<RelatedOpsItem>>,
    #[serde(rename = "Severity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    #[serde(rename = "Source")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetOpsMetadataRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "OpsMetadataArn")]
    #[serde(default)]
    pub ops_metadata_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetOpsMetadataResult {
    #[serde(rename = "Metadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, MetadataValue>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetOpsSummaryRequest {
    #[serde(rename = "Aggregators")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregators: Option<Vec<OpsAggregator>>,
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<OpsFilter>>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResultAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_attributes: Option<Vec<OpsResultAttribute>>,
    #[serde(rename = "SyncName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OpsAggregator {
    #[serde(rename = "AggregatorType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregator_type: Option<String>,
    #[serde(rename = "Aggregators")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregators: Option<Vec<OpsAggregator>>,
    #[serde(rename = "AttributeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_name: Option<String>,
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<OpsFilter>>,
    #[serde(rename = "TypeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OpsFilter {
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Values")]
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OpsResultAttribute {
    #[serde(rename = "TypeName")]
    #[serde(default)]
    pub type_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetOpsSummaryResult {
    #[serde(rename = "Entities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<OpsEntity>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OpsEntity {
    #[serde(rename = "Data")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<std::collections::HashMap<String, OpsEntityItem>>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OpsEntityItem {
    #[serde(rename = "CaptureTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_time: Option<String>,
    #[serde(rename = "Content")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<std::collections::HashMap<String, String>>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetParameterHistoryRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "WithDecryption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_decryption: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetParameterHistoryResult {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<ParameterHistory>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ParameterHistory {
    #[serde(rename = "AllowedPattern")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_pattern: Option<String>,
    #[serde(rename = "DataType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "KeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    #[serde(rename = "Labels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    #[serde(rename = "LastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[serde(rename = "LastModifiedUser")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_user: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Policies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<ParameterInlinePolicy>>,
    #[serde(rename = "Tier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetParameterRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "WithDecryption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_decryption: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetParameterResult {
    #[serde(rename = "Parameter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter: Option<Parameter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Parameter {
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
    #[serde(rename = "DataType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
    #[serde(rename = "LastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Selector")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,
    #[serde(rename = "SourceResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_result: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetParametersByPathRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ParameterFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_filters: Option<Vec<ParameterStringFilter>>,
    #[serde(rename = "Path")]
    #[serde(default)]
    pub path: String,
    #[serde(rename = "Recursive")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recursive: Option<bool>,
    #[serde(rename = "WithDecryption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_decryption: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetParametersByPathResult {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<Parameter>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetParametersRequest {
    #[serde(rename = "Names")]
    #[serde(default)]
    pub names: Vec<String>,
    #[serde(rename = "WithDecryption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_decryption: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetParametersResult {
    #[serde(rename = "InvalidParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalid_parameters: Option<Vec<String>>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<Parameter>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPatchBaselineForPatchGroupRequest {
    #[serde(rename = "OperatingSystem")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<String>,
    #[serde(rename = "PatchGroup")]
    #[serde(default)]
    pub patch_group: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPatchBaselineForPatchGroupResult {
    #[serde(rename = "BaselineId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_id: Option<String>,
    #[serde(rename = "OperatingSystem")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<String>,
    #[serde(rename = "PatchGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_group: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPatchBaselineRequest {
    #[serde(rename = "BaselineId")]
    #[serde(default)]
    pub baseline_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPatchBaselineResult {
    #[serde(rename = "ApprovalRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_rules: Option<PatchRuleGroup>,
    #[serde(rename = "ApprovedPatches")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_patches: Option<Vec<String>>,
    #[serde(rename = "ApprovedPatchesComplianceLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_patches_compliance_level: Option<String>,
    #[serde(rename = "ApprovedPatchesEnableNonSecurity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_patches_enable_non_security: Option<bool>,
    #[serde(rename = "AvailableSecurityUpdatesComplianceStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_security_updates_compliance_status: Option<String>,
    #[serde(rename = "BaselineId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_id: Option<String>,
    #[serde(rename = "CreatedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "GlobalFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_filters: Option<PatchFilterGroup>,
    #[serde(rename = "ModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_date: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "OperatingSystem")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<String>,
    #[serde(rename = "PatchGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_groups: Option<Vec<String>>,
    #[serde(rename = "RejectedPatches")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejected_patches: Option<Vec<String>>,
    #[serde(rename = "RejectedPatchesAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejected_patches_action: Option<String>,
    #[serde(rename = "Sources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<PatchSource>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourcePoliciesRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourcePoliciesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Policies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<GetResourcePoliciesResponseEntry>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourcePoliciesResponseEntry {
    #[serde(rename = "Policy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    #[serde(rename = "PolicyHash")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_hash: Option<String>,
    #[serde(rename = "PolicyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetServiceSettingRequest {
    #[serde(rename = "SettingId")]
    #[serde(default)]
    pub setting_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetServiceSettingResult {
    #[serde(rename = "ServiceSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_setting: Option<ServiceSetting>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceSetting {
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
    #[serde(rename = "LastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[serde(rename = "LastModifiedUser")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_user: Option<String>,
    #[serde(rename = "SettingId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setting_id: Option<String>,
    #[serde(rename = "SettingValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setting_value: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LabelParameterVersionRequest {
    #[serde(rename = "Labels")]
    #[serde(default)]
    pub labels: Vec<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "ParameterVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_version: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LabelParameterVersionResult {
    #[serde(rename = "InvalidLabels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalid_labels: Option<Vec<String>>,
    #[serde(rename = "ParameterVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_version: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAssociationVersionsRequest {
    #[serde(rename = "AssociationId")]
    #[serde(default)]
    pub association_id: String,
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
pub struct ListAssociationVersionsResult {
    #[serde(rename = "AssociationVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_versions: Option<Vec<AssociationVersionInfo>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociationVersionInfo {
    #[serde(rename = "ApplyOnlyAtCronInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_only_at_cron_interval: Option<bool>,
    #[serde(rename = "AssociationDispatchAssumeRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_dispatch_assume_role: Option<String>,
    #[serde(rename = "AssociationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_id: Option<String>,
    #[serde(rename = "AssociationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_name: Option<String>,
    #[serde(rename = "AssociationVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_version: Option<String>,
    #[serde(rename = "CalendarNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calendar_names: Option<Vec<String>>,
    #[serde(rename = "ComplianceSeverity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_severity: Option<String>,
    #[serde(rename = "CreatedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    #[serde(rename = "DocumentVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    #[serde(rename = "Duration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(rename = "MaxConcurrency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrency: Option<String>,
    #[serde(rename = "MaxErrors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_errors: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "OutputLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_location: Option<InstanceAssociationOutputLocation>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "ScheduleExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_expression: Option<String>,
    #[serde(rename = "ScheduleOffset")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_offset: Option<i32>,
    #[serde(rename = "SyncCompliance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_compliance: Option<String>,
    #[serde(rename = "TargetLocations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_locations: Option<Vec<TargetLocation>>,
    #[serde(rename = "TargetMaps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_maps: Option<Vec<std::collections::HashMap<String, Vec<String>>>>,
    #[serde(rename = "Targets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAssociationsRequest {
    #[serde(rename = "AssociationFilterList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_filter_list: Option<Vec<AssociationFilter>>,
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
pub struct AssociationFilter {
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAssociationsResult {
    #[serde(rename = "Associations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associations: Option<Vec<Association>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Association {
    #[serde(rename = "AssociationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_id: Option<String>,
    #[serde(rename = "AssociationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_name: Option<String>,
    #[serde(rename = "AssociationVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_version: Option<String>,
    #[serde(rename = "DocumentVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    #[serde(rename = "Duration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(rename = "LastExecutionDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_execution_date: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Overview")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overview: Option<AssociationOverview>,
    #[serde(rename = "ScheduleExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_expression: Option<String>,
    #[serde(rename = "ScheduleOffset")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_offset: Option<i32>,
    #[serde(rename = "TargetMaps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_maps: Option<Vec<std::collections::HashMap<String, Vec<String>>>>,
    #[serde(rename = "Targets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCommandInvocationsRequest {
    #[serde(rename = "CommandId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_id: Option<String>,
    #[serde(rename = "Details")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<bool>,
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<CommandFilter>>,
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
pub struct CommandFilter {
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCommandInvocationsResult {
    #[serde(rename = "CommandInvocations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_invocations: Option<Vec<CommandInvocation>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CommandInvocation {
    #[serde(rename = "CloudWatchOutputConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_output_config: Option<CloudWatchOutputConfig>,
    #[serde(rename = "CommandId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_id: Option<String>,
    #[serde(rename = "CommandPlugins")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_plugins: Option<Vec<CommandPlugin>>,
    #[serde(rename = "Comment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "DocumentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_name: Option<String>,
    #[serde(rename = "DocumentVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(rename = "InstanceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    #[serde(rename = "NotificationConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_config: Option<NotificationConfig>,
    #[serde(rename = "RequestedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_date_time: Option<f64>,
    #[serde(rename = "ServiceRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role: Option<String>,
    #[serde(rename = "StandardErrorUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard_error_url: Option<String>,
    #[serde(rename = "StandardOutputUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard_output_url: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
    #[serde(rename = "TraceOutput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_output: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CommandPlugin {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Output")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    #[serde(rename = "OutputS3BucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_s3_bucket_name: Option<String>,
    #[serde(rename = "OutputS3KeyPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_s3_key_prefix: Option<String>,
    #[serde(rename = "OutputS3Region")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_s3_region: Option<String>,
    #[serde(rename = "ResponseCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_code: Option<i32>,
    #[serde(rename = "ResponseFinishDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_finish_date_time: Option<f64>,
    #[serde(rename = "ResponseStartDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_start_date_time: Option<f64>,
    #[serde(rename = "StandardErrorUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard_error_url: Option<String>,
    #[serde(rename = "StandardOutputUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard_output_url: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCommandsRequest {
    #[serde(rename = "CommandId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_id: Option<String>,
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<CommandFilter>>,
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
pub struct ListCommandsResult {
    #[serde(rename = "Commands")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commands: Option<Vec<Command>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Command {
    #[serde(rename = "AlarmConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_configuration: Option<AlarmConfiguration>,
    #[serde(rename = "CloudWatchOutputConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_output_config: Option<CloudWatchOutputConfig>,
    #[serde(rename = "CommandId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_id: Option<String>,
    #[serde(rename = "Comment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "CompletedCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_count: Option<i32>,
    #[serde(rename = "DeliveryTimedOutCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_timed_out_count: Option<i32>,
    #[serde(rename = "DocumentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_name: Option<String>,
    #[serde(rename = "DocumentVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    #[serde(rename = "ErrorCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_count: Option<i32>,
    #[serde(rename = "ExpiresAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after: Option<f64>,
    #[serde(rename = "InstanceIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_ids: Option<Vec<String>>,
    #[serde(rename = "MaxConcurrency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrency: Option<String>,
    #[serde(rename = "MaxErrors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_errors: Option<String>,
    #[serde(rename = "NotificationConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_config: Option<NotificationConfig>,
    #[serde(rename = "OutputS3BucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_s3_bucket_name: Option<String>,
    #[serde(rename = "OutputS3KeyPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_s3_key_prefix: Option<String>,
    #[serde(rename = "OutputS3Region")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_s3_region: Option<String>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "RequestedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_date_time: Option<f64>,
    #[serde(rename = "ServiceRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
    #[serde(rename = "TargetCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_count: Option<i32>,
    #[serde(rename = "Targets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
    #[serde(rename = "TimeoutSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<i32>,
    #[serde(rename = "TriggeredAlarms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggered_alarms: Option<Vec<AlarmStateInformation>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListComplianceItemsRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<ComplianceStringFilter>>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_ids: Option<Vec<String>>,
    #[serde(rename = "ResourceTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_types: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ComplianceStringFilter {
    #[serde(rename = "Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListComplianceItemsResult {
    #[serde(rename = "ComplianceItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_items: Option<Vec<ComplianceItem>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ComplianceItem {
    #[serde(rename = "ComplianceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_type: Option<String>,
    #[serde(rename = "Details")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ExecutionSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_summary: Option<ComplianceExecutionSummary>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "Severity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ComplianceExecutionSummary {
    #[serde(rename = "ExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_id: Option<String>,
    #[serde(rename = "ExecutionTime")]
    #[serde(default)]
    pub execution_time: f64,
    #[serde(rename = "ExecutionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListComplianceSummariesRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<ComplianceStringFilter>>,
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
pub struct ListComplianceSummariesResult {
    #[serde(rename = "ComplianceSummaryItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_summary_items: Option<Vec<ComplianceSummaryItem>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ComplianceSummaryItem {
    #[serde(rename = "ComplianceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_type: Option<String>,
    #[serde(rename = "CompliantSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliant_summary: Option<CompliantSummary>,
    #[serde(rename = "NonCompliantSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_compliant_summary: Option<NonCompliantSummary>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CompliantSummary {
    #[serde(rename = "CompliantCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliant_count: Option<i32>,
    #[serde(rename = "SeveritySummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity_summary: Option<SeveritySummary>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SeveritySummary {
    #[serde(rename = "CriticalCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub critical_count: Option<i32>,
    #[serde(rename = "HighCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high_count: Option<i32>,
    #[serde(rename = "InformationalCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub informational_count: Option<i32>,
    #[serde(rename = "LowCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub low_count: Option<i32>,
    #[serde(rename = "MediumCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medium_count: Option<i32>,
    #[serde(rename = "UnspecifiedCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unspecified_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NonCompliantSummary {
    #[serde(rename = "NonCompliantCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_compliant_count: Option<i32>,
    #[serde(rename = "SeveritySummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity_summary: Option<SeveritySummary>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDocumentMetadataHistoryRequest {
    #[serde(rename = "DocumentVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "Metadata")]
    #[serde(default)]
    pub metadata: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDocumentMetadataHistoryResponse {
    #[serde(rename = "Author")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(rename = "DocumentVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    #[serde(rename = "Metadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<DocumentMetadataResponseInfo>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DocumentMetadataResponseInfo {
    #[serde(rename = "ReviewerResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reviewer_response: Option<Vec<DocumentReviewerResponseSource>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DocumentReviewerResponseSource {
    #[serde(rename = "Comment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<Vec<DocumentReviewCommentSource>>,
    #[serde(rename = "CreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<f64>,
    #[serde(rename = "ReviewStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_status: Option<String>,
    #[serde(rename = "Reviewer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reviewer: Option<String>,
    #[serde(rename = "UpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DocumentReviewCommentSource {
    #[serde(rename = "Content")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDocumentVersionsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDocumentVersionsResult {
    #[serde(rename = "DocumentVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_versions: Option<Vec<DocumentVersionInfo>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DocumentVersionInfo {
    #[serde(rename = "CreatedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    #[serde(rename = "DisplayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "DocumentFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_format: Option<String>,
    #[serde(rename = "DocumentVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    #[serde(rename = "IsDefaultVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default_version: Option<bool>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ReviewStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_status: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusInformation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_information: Option<String>,
    #[serde(rename = "VersionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDocumentsRequest {
    #[serde(rename = "DocumentFilterList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_filter_list: Option<Vec<DocumentFilter>>,
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<DocumentKeyValuesFilter>>,
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
pub struct DocumentFilter {
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DocumentKeyValuesFilter {
    #[serde(rename = "Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDocumentsResult {
    #[serde(rename = "DocumentIdentifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_identifiers: Option<Vec<DocumentIdentifier>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DocumentIdentifier {
    #[serde(rename = "Author")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(rename = "CreatedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    #[serde(rename = "DisplayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "DocumentFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_format: Option<String>,
    #[serde(rename = "DocumentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_type: Option<String>,
    #[serde(rename = "DocumentVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Owner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(rename = "PlatformTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_types: Option<Vec<String>>,
    #[serde(rename = "Requires")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requires: Option<Vec<DocumentRequires>>,
    #[serde(rename = "ReviewStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_status: Option<String>,
    #[serde(rename = "SchemaVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "TargetType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,
    #[serde(rename = "VersionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListInventoryEntriesRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<InventoryFilter>>,
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
    #[serde(rename = "TypeName")]
    #[serde(default)]
    pub type_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListInventoryEntriesResult {
    #[serde(rename = "CaptureTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_time: Option<String>,
    #[serde(rename = "Entries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<std::collections::HashMap<String, String>>>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SchemaVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<String>,
    #[serde(rename = "TypeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListNodesRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<NodeFilter>>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SyncName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NodeFilter {
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Values")]
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListNodesResult {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Nodes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<Node>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Node {
    #[serde(rename = "CaptureTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_time: Option<f64>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "NodeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<NodeType>,
    #[serde(rename = "Owner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<NodeOwnerInfo>,
    #[serde(rename = "Region")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NodeType {
    #[serde(rename = "Instance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<InstanceInfo>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceInfo {
    #[serde(rename = "AgentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_type: Option<String>,
    #[serde(rename = "AgentVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    #[serde(rename = "ComputerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub computer_name: Option<String>,
    #[serde(rename = "InstanceStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_status: Option<String>,
    #[serde(rename = "IpAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(rename = "ManagedStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_status: Option<String>,
    #[serde(rename = "PlatformName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_name: Option<String>,
    #[serde(rename = "PlatformType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_type: Option<String>,
    #[serde(rename = "PlatformVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_version: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NodeOwnerInfo {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "OrganizationalUnitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizational_unit_id: Option<String>,
    #[serde(rename = "OrganizationalUnitPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizational_unit_path: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListNodesSummaryRequest {
    #[serde(rename = "Aggregators")]
    #[serde(default)]
    pub aggregators: Vec<NodeAggregator>,
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<NodeFilter>>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SyncName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NodeAggregator {
    #[serde(rename = "AggregatorType")]
    #[serde(default)]
    pub aggregator_type: String,
    #[serde(rename = "Aggregators")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregators: Option<Vec<NodeAggregator>>,
    #[serde(rename = "AttributeName")]
    #[serde(default)]
    pub attribute_name: String,
    #[serde(rename = "TypeName")]
    #[serde(default)]
    pub type_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListNodesSummaryResult {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Summary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Vec<std::collections::HashMap<String, String>>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListOpsItemEventsRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<OpsItemEventFilter>>,
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
pub struct OpsItemEventFilter {
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "Operator")]
    #[serde(default)]
    pub operator: String,
    #[serde(rename = "Values")]
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListOpsItemEventsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Summaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summaries: Option<Vec<OpsItemEventSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OpsItemEventSummary {
    #[serde(rename = "CreatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<OpsItemIdentity>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "Detail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(rename = "DetailType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail_type: Option<String>,
    #[serde(rename = "EventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    #[serde(rename = "OpsItemId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ops_item_id: Option<String>,
    #[serde(rename = "Source")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OpsItemIdentity {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListOpsItemRelatedItemsRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<OpsItemRelatedItemsFilter>>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "OpsItemId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ops_item_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OpsItemRelatedItemsFilter {
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "Operator")]
    #[serde(default)]
    pub operator: String,
    #[serde(rename = "Values")]
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListOpsItemRelatedItemsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Summaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summaries: Option<Vec<OpsItemRelatedItemSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OpsItemRelatedItemSummary {
    #[serde(rename = "AssociationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_id: Option<String>,
    #[serde(rename = "AssociationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_type: Option<String>,
    #[serde(rename = "CreatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<OpsItemIdentity>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "LastModifiedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<OpsItemIdentity>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "OpsItemId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ops_item_id: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "ResourceUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListOpsMetadataRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<OpsMetadataFilter>>,
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
pub struct OpsMetadataFilter {
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "Values")]
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListOpsMetadataResult {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "OpsMetadataList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ops_metadata_list: Option<Vec<OpsMetadata>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OpsMetadata {
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "LastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[serde(rename = "LastModifiedUser")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_user: Option<String>,
    #[serde(rename = "OpsMetadataArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ops_metadata_arn: Option<String>,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListResourceComplianceSummariesRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<ComplianceStringFilter>>,
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
pub struct ListResourceComplianceSummariesResult {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceComplianceSummaryItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_compliance_summary_items: Option<Vec<ResourceComplianceSummaryItem>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceComplianceSummaryItem {
    #[serde(rename = "ComplianceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_type: Option<String>,
    #[serde(rename = "CompliantSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliant_summary: Option<CompliantSummary>,
    #[serde(rename = "ExecutionSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_summary: Option<ComplianceExecutionSummary>,
    #[serde(rename = "NonCompliantSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_compliant_summary: Option<NonCompliantSummary>,
    #[serde(rename = "OverallSeverity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall_severity: Option<String>,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListResourceDataSyncRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SyncType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListResourceDataSyncResult {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceDataSyncItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_data_sync_items: Option<Vec<ResourceDataSyncItem>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceDataSyncItem {
    #[serde(rename = "LastStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_status: Option<String>,
    #[serde(rename = "LastSuccessfulSyncTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_successful_sync_time: Option<f64>,
    #[serde(rename = "LastSyncStatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_sync_status_message: Option<String>,
    #[serde(rename = "LastSyncTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_sync_time: Option<f64>,
    #[serde(rename = "S3Destination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_destination: Option<ResourceDataSyncS3Destination>,
    #[serde(rename = "SyncCreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_created_time: Option<f64>,
    #[serde(rename = "SyncLastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_last_modified_time: Option<f64>,
    #[serde(rename = "SyncName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_name: Option<String>,
    #[serde(rename = "SyncSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_source: Option<ResourceDataSyncSourceWithState>,
    #[serde(rename = "SyncType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceDataSyncSourceWithState {
    #[serde(rename = "AwsOrganizationsSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_organizations_source: Option<ResourceDataSyncAwsOrganizationsSource>,
    #[serde(rename = "EnableAllOpsDataSources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_all_ops_data_sources: Option<bool>,
    #[serde(rename = "IncludeFutureRegions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_future_regions: Option<bool>,
    #[serde(rename = "SourceRegions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_regions: Option<Vec<String>>,
    #[serde(rename = "SourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    pub resource_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResult {
    #[serde(rename = "TagList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModifyDocumentPermissionRequest {
    #[serde(rename = "AccountIdsToAdd")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids_to_add: Option<Vec<String>>,
    #[serde(rename = "AccountIdsToRemove")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids_to_remove: Option<Vec<String>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "PermissionType")]
    #[serde(default)]
    pub permission_type: String,
    #[serde(rename = "SharedDocumentVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_document_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModifyDocumentPermissionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutComplianceItemsRequest {
    #[serde(rename = "ComplianceType")]
    #[serde(default)]
    pub compliance_type: String,
    #[serde(rename = "ExecutionSummary")]
    #[serde(default)]
    pub execution_summary: ComplianceExecutionSummary,
    #[serde(rename = "ItemContentHash")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_content_hash: Option<String>,
    #[serde(rename = "Items")]
    #[serde(default)]
    pub items: Vec<ComplianceItemEntry>,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    pub resource_type: String,
    #[serde(rename = "UploadType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ComplianceItemEntry {
    #[serde(rename = "Details")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Severity")]
    #[serde(default)]
    pub severity: String,
    #[serde(rename = "Status")]
    #[serde(default)]
    pub status: String,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutComplianceItemsResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutInventoryRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Items")]
    #[serde(default)]
    pub items: Vec<InventoryItem>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InventoryItem {
    #[serde(rename = "CaptureTime")]
    #[serde(default)]
    pub capture_time: String,
    #[serde(rename = "Content")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<std::collections::HashMap<String, String>>>,
    #[serde(rename = "ContentHash")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_hash: Option<String>,
    #[serde(rename = "Context")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "SchemaVersion")]
    #[serde(default)]
    pub schema_version: String,
    #[serde(rename = "TypeName")]
    #[serde(default)]
    pub type_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutInventoryResult {
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutParameterRequest {
    #[serde(rename = "AllowedPattern")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_pattern: Option<String>,
    #[serde(rename = "DataType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "KeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Overwrite")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overwrite: Option<bool>,
    #[serde(rename = "Policies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "Tier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutParameterResult {
    #[serde(rename = "Tier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutResourcePolicyRequest {
    #[serde(rename = "Policy")]
    #[serde(default)]
    pub policy: String,
    #[serde(rename = "PolicyHash")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_hash: Option<String>,
    #[serde(rename = "PolicyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutResourcePolicyResponse {
    #[serde(rename = "PolicyHash")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_hash: Option<String>,
    #[serde(rename = "PolicyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterDefaultPatchBaselineRequest {
    #[serde(rename = "BaselineId")]
    #[serde(default)]
    pub baseline_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterDefaultPatchBaselineResult {
    #[serde(rename = "BaselineId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterPatchBaselineForPatchGroupRequest {
    #[serde(rename = "BaselineId")]
    #[serde(default)]
    pub baseline_id: String,
    #[serde(rename = "PatchGroup")]
    #[serde(default)]
    pub patch_group: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterPatchBaselineForPatchGroupResult {
    #[serde(rename = "BaselineId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_id: Option<String>,
    #[serde(rename = "PatchGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_group: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterTargetWithMaintenanceWindowRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "OwnerInformation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_information: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    pub resource_type: String,
    #[serde(rename = "Targets")]
    #[serde(default)]
    pub targets: Vec<Target>,
    #[serde(rename = "WindowId")]
    #[serde(default)]
    pub window_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterTargetWithMaintenanceWindowResult {
    #[serde(rename = "WindowTargetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_target_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterTaskWithMaintenanceWindowRequest {
    #[serde(rename = "AlarmConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_configuration: Option<AlarmConfiguration>,
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "CutoffBehavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cutoff_behavior: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "LoggingInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_info: Option<LoggingInfo>,
    #[serde(rename = "MaxConcurrency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrency: Option<String>,
    #[serde(rename = "MaxErrors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_errors: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Priority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(rename = "ServiceRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role_arn: Option<String>,
    #[serde(rename = "Targets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
    #[serde(rename = "TaskArn")]
    #[serde(default)]
    pub task_arn: String,
    #[serde(rename = "TaskInvocationParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_invocation_parameters: Option<MaintenanceWindowTaskInvocationParameters>,
    #[serde(rename = "TaskParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_parameters:
        Option<std::collections::HashMap<String, MaintenanceWindowTaskParameterValueExpression>>,
    #[serde(rename = "TaskType")]
    #[serde(default)]
    pub task_type: String,
    #[serde(rename = "WindowId")]
    #[serde(default)]
    pub window_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterTaskWithMaintenanceWindowResult {
    #[serde(rename = "WindowTaskId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_task_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveTagsFromResourceRequest {
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    pub resource_type: String,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveTagsFromResourceResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResetServiceSettingRequest {
    #[serde(rename = "SettingId")]
    #[serde(default)]
    pub setting_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResetServiceSettingResult {
    #[serde(rename = "ServiceSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_setting: Option<ServiceSetting>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResumeSessionRequest {
    #[serde(rename = "SessionId")]
    #[serde(default)]
    pub session_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResumeSessionResponse {
    #[serde(rename = "SessionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(rename = "StreamUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_url: Option<String>,
    #[serde(rename = "TokenValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SendAutomationSignalRequest {
    #[serde(rename = "AutomationExecutionId")]
    #[serde(default)]
    pub automation_execution_id: String,
    #[serde(rename = "Payload")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "SignalType")]
    #[serde(default)]
    pub signal_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SendAutomationSignalResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SendCommandRequest {
    #[serde(rename = "AlarmConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_configuration: Option<AlarmConfiguration>,
    #[serde(rename = "CloudWatchOutputConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_output_config: Option<CloudWatchOutputConfig>,
    #[serde(rename = "Comment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "DocumentHash")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_hash: Option<String>,
    #[serde(rename = "DocumentHashType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_hash_type: Option<String>,
    #[serde(rename = "DocumentName")]
    #[serde(default)]
    pub document_name: String,
    #[serde(rename = "DocumentVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    #[serde(rename = "InstanceIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_ids: Option<Vec<String>>,
    #[serde(rename = "MaxConcurrency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrency: Option<String>,
    #[serde(rename = "MaxErrors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_errors: Option<String>,
    #[serde(rename = "NotificationConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_config: Option<NotificationConfig>,
    #[serde(rename = "OutputS3BucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_s3_bucket_name: Option<String>,
    #[serde(rename = "OutputS3KeyPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_s3_key_prefix: Option<String>,
    #[serde(rename = "OutputS3Region")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_s3_region: Option<String>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "ServiceRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role_arn: Option<String>,
    #[serde(rename = "Targets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
    #[serde(rename = "TimeoutSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SendCommandResult {
    #[serde(rename = "Command")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Command>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartAccessRequestRequest {
    #[serde(rename = "Reason")]
    #[serde(default)]
    pub reason: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "Targets")]
    #[serde(default)]
    pub targets: Vec<Target>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartAccessRequestResponse {
    #[serde(rename = "AccessRequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_request_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartAssociationsOnceRequest {
    #[serde(rename = "AssociationIds")]
    #[serde(default)]
    pub association_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartAssociationsOnceResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartAutomationExecutionRequest {
    #[serde(rename = "AlarmConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_configuration: Option<AlarmConfiguration>,
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "DocumentName")]
    #[serde(default)]
    pub document_name: String,
    #[serde(rename = "DocumentVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    #[serde(rename = "MaxConcurrency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrency: Option<String>,
    #[serde(rename = "MaxErrors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_errors: Option<String>,
    #[serde(rename = "Mode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "TargetLocations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_locations: Option<Vec<TargetLocation>>,
    #[serde(rename = "TargetLocationsURL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_locations_u_r_l: Option<String>,
    #[serde(rename = "TargetMaps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_maps: Option<Vec<std::collections::HashMap<String, Vec<String>>>>,
    #[serde(rename = "TargetParameterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_parameter_name: Option<String>,
    #[serde(rename = "Targets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartAutomationExecutionResult {
    #[serde(rename = "AutomationExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automation_execution_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartChangeRequestExecutionRequest {
    #[serde(rename = "AutoApprove")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_approve: Option<bool>,
    #[serde(rename = "ChangeDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_details: Option<String>,
    #[serde(rename = "ChangeRequestName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_request_name: Option<String>,
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "DocumentName")]
    #[serde(default)]
    pub document_name: String,
    #[serde(rename = "DocumentVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "Runbooks")]
    #[serde(default)]
    pub runbooks: Vec<Runbook>,
    #[serde(rename = "ScheduledEndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_end_time: Option<f64>,
    #[serde(rename = "ScheduledTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_time: Option<f64>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartChangeRequestExecutionResult {
    #[serde(rename = "AutomationExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automation_execution_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartExecutionPreviewRequest {
    #[serde(rename = "DocumentName")]
    #[serde(default)]
    pub document_name: String,
    #[serde(rename = "DocumentVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    #[serde(rename = "ExecutionInputs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_inputs: Option<ExecutionInputs>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExecutionInputs {
    #[serde(rename = "Automation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automation: Option<AutomationExecutionInputs>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomationExecutionInputs {
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "TargetLocations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_locations: Option<Vec<TargetLocation>>,
    #[serde(rename = "TargetLocationsURL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_locations_u_r_l: Option<String>,
    #[serde(rename = "TargetMaps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_maps: Option<Vec<std::collections::HashMap<String, Vec<String>>>>,
    #[serde(rename = "TargetParameterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_parameter_name: Option<String>,
    #[serde(rename = "Targets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartExecutionPreviewResponse {
    #[serde(rename = "ExecutionPreviewId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_preview_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartSessionRequest {
    #[serde(rename = "DocumentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_name: Option<String>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "Reason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "Target")]
    #[serde(default)]
    pub target: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartSessionResponse {
    #[serde(rename = "SessionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(rename = "StreamUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_url: Option<String>,
    #[serde(rename = "TokenValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopAutomationExecutionRequest {
    #[serde(rename = "AutomationExecutionId")]
    #[serde(default)]
    pub automation_execution_id: String,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopAutomationExecutionResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TerminateSessionRequest {
    #[serde(rename = "SessionId")]
    #[serde(default)]
    pub session_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TerminateSessionResponse {
    #[serde(rename = "SessionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UnlabelParameterVersionRequest {
    #[serde(rename = "Labels")]
    #[serde(default)]
    pub labels: Vec<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "ParameterVersion")]
    #[serde(default)]
    pub parameter_version: i64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UnlabelParameterVersionResult {
    #[serde(rename = "InvalidLabels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalid_labels: Option<Vec<String>>,
    #[serde(rename = "RemovedLabels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub removed_labels: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAssociationRequest {
    #[serde(rename = "AlarmConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_configuration: Option<AlarmConfiguration>,
    #[serde(rename = "ApplyOnlyAtCronInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_only_at_cron_interval: Option<bool>,
    #[serde(rename = "AssociationDispatchAssumeRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_dispatch_assume_role: Option<String>,
    #[serde(rename = "AssociationId")]
    #[serde(default)]
    pub association_id: String,
    #[serde(rename = "AssociationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_name: Option<String>,
    #[serde(rename = "AssociationVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_version: Option<String>,
    #[serde(rename = "AutomationTargetParameterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automation_target_parameter_name: Option<String>,
    #[serde(rename = "CalendarNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calendar_names: Option<Vec<String>>,
    #[serde(rename = "ComplianceSeverity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_severity: Option<String>,
    #[serde(rename = "DocumentVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    #[serde(rename = "Duration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(rename = "MaxConcurrency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrency: Option<String>,
    #[serde(rename = "MaxErrors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_errors: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "OutputLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_location: Option<InstanceAssociationOutputLocation>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "ScheduleExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_expression: Option<String>,
    #[serde(rename = "ScheduleOffset")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_offset: Option<i32>,
    #[serde(rename = "SyncCompliance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_compliance: Option<String>,
    #[serde(rename = "TargetLocations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_locations: Option<Vec<TargetLocation>>,
    #[serde(rename = "TargetMaps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_maps: Option<Vec<std::collections::HashMap<String, Vec<String>>>>,
    #[serde(rename = "Targets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAssociationResult {
    #[serde(rename = "AssociationDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_description: Option<AssociationDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAssociationStatusRequest {
    #[serde(rename = "AssociationStatus")]
    #[serde(default)]
    pub association_status: AssociationStatus,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAssociationStatusResult {
    #[serde(rename = "AssociationDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_description: Option<AssociationDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDocumentDefaultVersionRequest {
    #[serde(rename = "DocumentVersion")]
    #[serde(default)]
    pub document_version: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDocumentDefaultVersionResult {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<DocumentDefaultVersionDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DocumentDefaultVersionDescription {
    #[serde(rename = "DefaultVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_version: Option<String>,
    #[serde(rename = "DefaultVersionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_version_name: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDocumentMetadataRequest {
    #[serde(rename = "DocumentReviews")]
    #[serde(default)]
    pub document_reviews: DocumentReviews,
    #[serde(rename = "DocumentVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DocumentReviews {
    #[serde(rename = "Action")]
    #[serde(default)]
    pub action: String,
    #[serde(rename = "Comment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<Vec<DocumentReviewCommentSource>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDocumentMetadataResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDocumentRequest {
    #[serde(rename = "Attachments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<AttachmentsSource>>,
    #[serde(rename = "Content")]
    #[serde(default)]
    pub content: String,
    #[serde(rename = "DisplayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "DocumentFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_format: Option<String>,
    #[serde(rename = "DocumentVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "TargetType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,
    #[serde(rename = "VersionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDocumentResult {
    #[serde(rename = "DocumentDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_description: Option<DocumentDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateMaintenanceWindowRequest {
    #[serde(rename = "AllowUnassociatedTargets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_unassociated_targets: Option<bool>,
    #[serde(rename = "Cutoff")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cutoff: Option<i32>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Duration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "EndDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Replace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace: Option<bool>,
    #[serde(rename = "Schedule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,
    #[serde(rename = "ScheduleOffset")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_offset: Option<i32>,
    #[serde(rename = "ScheduleTimezone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_timezone: Option<String>,
    #[serde(rename = "StartDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(rename = "WindowId")]
    #[serde(default)]
    pub window_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateMaintenanceWindowResult {
    #[serde(rename = "AllowUnassociatedTargets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_unassociated_targets: Option<bool>,
    #[serde(rename = "Cutoff")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cutoff: Option<i32>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Duration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "EndDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Schedule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,
    #[serde(rename = "ScheduleOffset")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_offset: Option<i32>,
    #[serde(rename = "ScheduleTimezone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_timezone: Option<String>,
    #[serde(rename = "StartDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(rename = "WindowId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateMaintenanceWindowTargetRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "OwnerInformation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_information: Option<String>,
    #[serde(rename = "Replace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace: Option<bool>,
    #[serde(rename = "Targets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
    #[serde(rename = "WindowId")]
    #[serde(default)]
    pub window_id: String,
    #[serde(rename = "WindowTargetId")]
    #[serde(default)]
    pub window_target_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateMaintenanceWindowTargetResult {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "OwnerInformation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_information: Option<String>,
    #[serde(rename = "Targets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
    #[serde(rename = "WindowId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_id: Option<String>,
    #[serde(rename = "WindowTargetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_target_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateMaintenanceWindowTaskRequest {
    #[serde(rename = "AlarmConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_configuration: Option<AlarmConfiguration>,
    #[serde(rename = "CutoffBehavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cutoff_behavior: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "LoggingInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_info: Option<LoggingInfo>,
    #[serde(rename = "MaxConcurrency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrency: Option<String>,
    #[serde(rename = "MaxErrors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_errors: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Priority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(rename = "Replace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace: Option<bool>,
    #[serde(rename = "ServiceRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role_arn: Option<String>,
    #[serde(rename = "Targets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
    #[serde(rename = "TaskArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_arn: Option<String>,
    #[serde(rename = "TaskInvocationParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_invocation_parameters: Option<MaintenanceWindowTaskInvocationParameters>,
    #[serde(rename = "TaskParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_parameters:
        Option<std::collections::HashMap<String, MaintenanceWindowTaskParameterValueExpression>>,
    #[serde(rename = "WindowId")]
    #[serde(default)]
    pub window_id: String,
    #[serde(rename = "WindowTaskId")]
    #[serde(default)]
    pub window_task_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateMaintenanceWindowTaskResult {
    #[serde(rename = "AlarmConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_configuration: Option<AlarmConfiguration>,
    #[serde(rename = "CutoffBehavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cutoff_behavior: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "LoggingInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_info: Option<LoggingInfo>,
    #[serde(rename = "MaxConcurrency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrency: Option<String>,
    #[serde(rename = "MaxErrors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_errors: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Priority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(rename = "ServiceRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role_arn: Option<String>,
    #[serde(rename = "Targets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
    #[serde(rename = "TaskArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_arn: Option<String>,
    #[serde(rename = "TaskInvocationParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_invocation_parameters: Option<MaintenanceWindowTaskInvocationParameters>,
    #[serde(rename = "TaskParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_parameters:
        Option<std::collections::HashMap<String, MaintenanceWindowTaskParameterValueExpression>>,
    #[serde(rename = "WindowId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_id: Option<String>,
    #[serde(rename = "WindowTaskId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_task_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateManagedInstanceRoleRequest {
    #[serde(rename = "IamRole")]
    #[serde(default)]
    pub iam_role: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateManagedInstanceRoleResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateOpsItemRequest {
    #[serde(rename = "ActualEndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_end_time: Option<f64>,
    #[serde(rename = "ActualStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_start_time: Option<f64>,
    #[serde(rename = "Category")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Notifications")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notifications: Option<Vec<OpsItemNotification>>,
    #[serde(rename = "OperationalData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operational_data: Option<std::collections::HashMap<String, OpsItemDataValue>>,
    #[serde(rename = "OperationalDataToDelete")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operational_data_to_delete: Option<Vec<String>>,
    #[serde(rename = "OpsItemArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ops_item_arn: Option<String>,
    #[serde(rename = "OpsItemId")]
    #[serde(default)]
    pub ops_item_id: String,
    #[serde(rename = "PlannedEndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub planned_end_time: Option<f64>,
    #[serde(rename = "PlannedStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub planned_start_time: Option<f64>,
    #[serde(rename = "Priority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(rename = "RelatedOpsItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_ops_items: Option<Vec<RelatedOpsItem>>,
    #[serde(rename = "Severity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateOpsItemResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateOpsMetadataRequest {
    #[serde(rename = "KeysToDelete")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keys_to_delete: Option<Vec<String>>,
    #[serde(rename = "MetadataToUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_to_update: Option<std::collections::HashMap<String, MetadataValue>>,
    #[serde(rename = "OpsMetadataArn")]
    #[serde(default)]
    pub ops_metadata_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateOpsMetadataResult {
    #[serde(rename = "OpsMetadataArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ops_metadata_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePatchBaselineRequest {
    #[serde(rename = "ApprovalRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_rules: Option<PatchRuleGroup>,
    #[serde(rename = "ApprovedPatches")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_patches: Option<Vec<String>>,
    #[serde(rename = "ApprovedPatchesComplianceLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_patches_compliance_level: Option<String>,
    #[serde(rename = "ApprovedPatchesEnableNonSecurity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_patches_enable_non_security: Option<bool>,
    #[serde(rename = "AvailableSecurityUpdatesComplianceStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_security_updates_compliance_status: Option<String>,
    #[serde(rename = "BaselineId")]
    #[serde(default)]
    pub baseline_id: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "GlobalFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_filters: Option<PatchFilterGroup>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "RejectedPatches")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejected_patches: Option<Vec<String>>,
    #[serde(rename = "RejectedPatchesAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejected_patches_action: Option<String>,
    #[serde(rename = "Replace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace: Option<bool>,
    #[serde(rename = "Sources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<PatchSource>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePatchBaselineResult {
    #[serde(rename = "ApprovalRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_rules: Option<PatchRuleGroup>,
    #[serde(rename = "ApprovedPatches")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_patches: Option<Vec<String>>,
    #[serde(rename = "ApprovedPatchesComplianceLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_patches_compliance_level: Option<String>,
    #[serde(rename = "ApprovedPatchesEnableNonSecurity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_patches_enable_non_security: Option<bool>,
    #[serde(rename = "AvailableSecurityUpdatesComplianceStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_security_updates_compliance_status: Option<String>,
    #[serde(rename = "BaselineId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_id: Option<String>,
    #[serde(rename = "CreatedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "GlobalFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_filters: Option<PatchFilterGroup>,
    #[serde(rename = "ModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_date: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "OperatingSystem")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<String>,
    #[serde(rename = "RejectedPatches")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejected_patches: Option<Vec<String>>,
    #[serde(rename = "RejectedPatchesAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejected_patches_action: Option<String>,
    #[serde(rename = "Sources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<PatchSource>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateResourceDataSyncRequest {
    #[serde(rename = "SyncName")]
    #[serde(default)]
    pub sync_name: String,
    #[serde(rename = "SyncSource")]
    #[serde(default)]
    pub sync_source: ResourceDataSyncSource,
    #[serde(rename = "SyncType")]
    #[serde(default)]
    pub sync_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateResourceDataSyncResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateServiceSettingRequest {
    #[serde(rename = "SettingId")]
    #[serde(default)]
    pub setting_id: String,
    #[serde(rename = "SettingValue")]
    #[serde(default)]
    pub setting_value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateServiceSettingResult {}
