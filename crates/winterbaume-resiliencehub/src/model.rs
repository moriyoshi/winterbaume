//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-resiliencehub

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AcceptResourceGroupingRecommendationsRequest {
    #[serde(rename = "appArn")]
    #[serde(default)]
    pub app_arn: String,
    #[serde(default)]
    pub entries: Vec<AcceptGroupingRecommendationEntry>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AcceptGroupingRecommendationEntry {
    #[serde(rename = "groupingRecommendationId")]
    #[serde(default)]
    pub grouping_recommendation_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AcceptResourceGroupingRecommendationsResponse {
    #[serde(rename = "appArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_arn: Option<String>,
    #[serde(rename = "failedEntries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_entries: Option<Vec<FailedGroupingRecommendationEntry>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FailedGroupingRecommendationEntry {
    #[serde(rename = "errorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "groupingRecommendationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grouping_recommendation_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddDraftAppVersionResourceMappingsRequest {
    #[serde(rename = "appArn")]
    #[serde(default)]
    pub app_arn: String,
    #[serde(rename = "resourceMappings")]
    #[serde(default)]
    pub resource_mappings: Vec<ResourceMapping>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceMapping {
    #[serde(rename = "appRegistryAppName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_registry_app_name: Option<String>,
    #[serde(rename = "eksSourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eks_source_name: Option<String>,
    #[serde(rename = "logicalStackName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logical_stack_name: Option<String>,
    #[serde(rename = "mappingType")]
    #[serde(default)]
    pub mapping_type: String,
    #[serde(rename = "physicalResourceId")]
    #[serde(default)]
    pub physical_resource_id: PhysicalResourceId,
    #[serde(rename = "resourceGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_name: Option<String>,
    #[serde(rename = "resourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    #[serde(rename = "terraformSourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terraform_source_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PhysicalResourceId {
    #[serde(rename = "awsAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_account_id: Option<String>,
    #[serde(rename = "awsRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_region: Option<String>,
    #[serde(default)]
    pub identifier: String,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddDraftAppVersionResourceMappingsResponse {
    #[serde(rename = "appArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_arn: Option<String>,
    #[serde(rename = "appVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_version: Option<String>,
    #[serde(rename = "resourceMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_mappings: Option<Vec<ResourceMapping>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchUpdateRecommendationStatusRequest {
    #[serde(rename = "appArn")]
    #[serde(default)]
    pub app_arn: String,
    #[serde(rename = "requestEntries")]
    #[serde(default)]
    pub request_entries: Vec<UpdateRecommendationStatusRequestEntry>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRecommendationStatusRequestEntry {
    #[serde(rename = "appComponentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_component_id: Option<String>,
    #[serde(rename = "entryId")]
    #[serde(default)]
    pub entry_id: String,
    #[serde(rename = "excludeReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_reason: Option<String>,
    #[serde(default)]
    pub excluded: bool,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<UpdateRecommendationStatusItem>,
    #[serde(rename = "referenceId")]
    #[serde(default)]
    pub reference_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRecommendationStatusItem {
    #[serde(rename = "resourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "targetAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_account_id: Option<String>,
    #[serde(rename = "targetRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_region: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchUpdateRecommendationStatusResponse {
    #[serde(rename = "appArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_arn: Option<String>,
    #[serde(rename = "failedEntries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_entries: Option<Vec<BatchUpdateRecommendationStatusFailedEntry>>,
    #[serde(rename = "successfulEntries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful_entries: Option<Vec<BatchUpdateRecommendationStatusSuccessfulEntry>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchUpdateRecommendationStatusFailedEntry {
    #[serde(rename = "entryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_id: Option<String>,
    #[serde(rename = "errorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchUpdateRecommendationStatusSuccessfulEntry {
    #[serde(rename = "appComponentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_component_id: Option<String>,
    #[serde(rename = "entryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_id: Option<String>,
    #[serde(rename = "excludeReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_reason: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<UpdateRecommendationStatusItem>,
    #[serde(rename = "referenceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAppRequest {
    #[serde(rename = "assessmentSchedule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_schedule: Option<String>,
    #[serde(rename = "awsApplicationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_application_arn: Option<String>,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "eventSubscriptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_subscriptions: Option<Vec<EventSubscription>>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "permissionModel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_model: Option<PermissionModel>,
    #[serde(rename = "policyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventSubscription {
    #[serde(rename = "eventType")]
    #[serde(default)]
    pub event_type: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "snsTopicArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PermissionModel {
    #[serde(rename = "crossAccountRoleArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_account_role_arns: Option<Vec<String>>,
    #[serde(rename = "invokerRoleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoker_role_name: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAppResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app: Option<App>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct App {
    #[serde(rename = "appArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_arn: Option<String>,
    #[serde(rename = "assessmentSchedule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_schedule: Option<String>,
    #[serde(rename = "awsApplicationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_application_arn: Option<String>,
    #[serde(rename = "complianceStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_status: Option<String>,
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "driftStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drift_status: Option<String>,
    #[serde(rename = "eventSubscriptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_subscriptions: Option<Vec<EventSubscription>>,
    #[serde(rename = "lastAppComplianceEvaluationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_app_compliance_evaluation_time: Option<f64>,
    #[serde(rename = "lastDriftEvaluationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_drift_evaluation_time: Option<f64>,
    #[serde(rename = "lastResiliencyScoreEvaluationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_resiliency_score_evaluation_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "permissionModel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_model: Option<PermissionModel>,
    #[serde(rename = "policyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_arn: Option<String>,
    #[serde(rename = "resiliencyScore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resiliency_score: Option<f64>,
    #[serde(rename = "rpoInSecs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rpo_in_secs: Option<i32>,
    #[serde(rename = "rtoInSecs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rto_in_secs: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAppVersionAppComponentRequest {
    #[serde(rename = "additionalInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "appArn")]
    #[serde(default)]
    pub app_arn: String,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAppVersionAppComponentResponse {
    #[serde(rename = "appArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_arn: Option<String>,
    #[serde(rename = "appComponent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_component: Option<AppComponent>,
    #[serde(rename = "appVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AppComponent {
    #[serde(rename = "additionalInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAppVersionResourceRequest {
    #[serde(rename = "additionalInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "appArn")]
    #[serde(default)]
    pub app_arn: String,
    #[serde(rename = "appComponents")]
    #[serde(default)]
    pub app_components: Vec<String>,
    #[serde(rename = "awsAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_account_id: Option<String>,
    #[serde(rename = "awsRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_region: Option<String>,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "logicalResourceId")]
    #[serde(default)]
    pub logical_resource_id: LogicalResourceId,
    #[serde(rename = "physicalResourceId")]
    #[serde(default)]
    pub physical_resource_id: String,
    #[serde(rename = "resourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    pub resource_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LogicalResourceId {
    #[serde(rename = "eksSourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eks_source_name: Option<String>,
    #[serde(default)]
    pub identifier: String,
    #[serde(rename = "logicalStackName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logical_stack_name: Option<String>,
    #[serde(rename = "resourceGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_name: Option<String>,
    #[serde(rename = "terraformSourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terraform_source_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAppVersionResourceResponse {
    #[serde(rename = "appArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_arn: Option<String>,
    #[serde(rename = "appVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_version: Option<String>,
    #[serde(rename = "physicalResource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_resource: Option<PhysicalResource>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PhysicalResource {
    #[serde(rename = "additionalInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "appComponents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_components: Option<Vec<AppComponent>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded: Option<bool>,
    #[serde(rename = "logicalResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logical_resource_id: Option<LogicalResourceId>,
    #[serde(rename = "parentResourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_resource_name: Option<String>,
    #[serde(rename = "physicalResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_resource_id: Option<PhysicalResourceId>,
    #[serde(rename = "resourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "sourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateRecommendationTemplateRequest {
    #[serde(rename = "assessmentArn")]
    #[serde(default)]
    pub assessment_arn: String,
    #[serde(rename = "bucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<String>,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "recommendationIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_ids: Option<Vec<String>>,
    #[serde(rename = "recommendationTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_types: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateRecommendationTemplateResponse {
    #[serde(rename = "recommendationTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_template: Option<RecommendationTemplate>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecommendationTemplate {
    #[serde(rename = "appArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_arn: Option<String>,
    #[serde(rename = "assessmentArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_arn: Option<String>,
    #[serde(rename = "endTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "needsReplacements")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needs_replacements: Option<bool>,
    #[serde(rename = "recommendationIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_ids: Option<Vec<String>>,
    #[serde(rename = "recommendationTemplateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_template_arn: Option<String>,
    #[serde(rename = "recommendationTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_types: Option<Vec<String>>,
    #[serde(rename = "startTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "templatesLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub templates_location: Option<S3Location>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3Location {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateResiliencyPolicyRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "dataLocationConstraint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_location_constraint: Option<String>,
    #[serde(default)]
    pub policy: std::collections::HashMap<String, FailurePolicy>,
    #[serde(rename = "policyDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_description: Option<String>,
    #[serde(rename = "policyName")]
    #[serde(default)]
    pub policy_name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    pub tier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FailurePolicy {
    #[serde(rename = "rpoInSecs")]
    #[serde(default)]
    pub rpo_in_secs: i32,
    #[serde(rename = "rtoInSecs")]
    #[serde(default)]
    pub rto_in_secs: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateResiliencyPolicyResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<ResiliencyPolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResiliencyPolicy {
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "dataLocationConstraint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_location_constraint: Option<String>,
    #[serde(rename = "estimatedCostTier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_cost_tier: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<std::collections::HashMap<String, FailurePolicy>>,
    #[serde(rename = "policyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_arn: Option<String>,
    #[serde(rename = "policyDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_description: Option<String>,
    #[serde(rename = "policyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAppAssessmentRequest {
    #[serde(rename = "assessmentArn")]
    #[serde(default)]
    pub assessment_arn: String,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAppAssessmentResponse {
    #[serde(rename = "assessmentArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_arn: Option<String>,
    #[serde(rename = "assessmentStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAppInputSourceRequest {
    #[serde(rename = "appArn")]
    #[serde(default)]
    pub app_arn: String,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "eksSourceClusterNamespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eks_source_cluster_namespace: Option<EksSourceClusterNamespace>,
    #[serde(rename = "sourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_arn: Option<String>,
    #[serde(rename = "terraformSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terraform_source: Option<TerraformSource>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EksSourceClusterNamespace {
    #[serde(rename = "eksClusterArn")]
    #[serde(default)]
    pub eks_cluster_arn: String,
    #[serde(default)]
    pub namespace: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TerraformSource {
    #[serde(rename = "s3StateFileUrl")]
    #[serde(default)]
    pub s3_state_file_url: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAppInputSourceResponse {
    #[serde(rename = "appArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_arn: Option<String>,
    #[serde(rename = "appInputSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_input_source: Option<AppInputSource>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AppInputSource {
    #[serde(rename = "eksSourceClusterNamespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eks_source_cluster_namespace: Option<EksSourceClusterNamespace>,
    #[serde(rename = "importType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_type: Option<String>,
    #[serde(rename = "resourceCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_count: Option<i32>,
    #[serde(rename = "sourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_arn: Option<String>,
    #[serde(rename = "sourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_name: Option<String>,
    #[serde(rename = "terraformSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terraform_source: Option<TerraformSource>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAppRequest {
    #[serde(rename = "appArn")]
    #[serde(default)]
    pub app_arn: String,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "forceDelete")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_delete: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAppResponse {
    #[serde(rename = "appArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAppVersionAppComponentRequest {
    #[serde(rename = "appArn")]
    #[serde(default)]
    pub app_arn: String,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAppVersionAppComponentResponse {
    #[serde(rename = "appArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_arn: Option<String>,
    #[serde(rename = "appComponent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_component: Option<AppComponent>,
    #[serde(rename = "appVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAppVersionResourceRequest {
    #[serde(rename = "appArn")]
    #[serde(default)]
    pub app_arn: String,
    #[serde(rename = "awsAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_account_id: Option<String>,
    #[serde(rename = "awsRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_region: Option<String>,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "logicalResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logical_resource_id: Option<LogicalResourceId>,
    #[serde(rename = "physicalResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_resource_id: Option<String>,
    #[serde(rename = "resourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAppVersionResourceResponse {
    #[serde(rename = "appArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_arn: Option<String>,
    #[serde(rename = "appVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_version: Option<String>,
    #[serde(rename = "physicalResource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_resource: Option<PhysicalResource>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRecommendationTemplateRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "recommendationTemplateArn")]
    #[serde(default)]
    pub recommendation_template_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRecommendationTemplateResponse {
    #[serde(rename = "recommendationTemplateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_template_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteResiliencyPolicyRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "policyArn")]
    #[serde(default)]
    pub policy_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteResiliencyPolicyResponse {
    #[serde(rename = "policyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAppAssessmentRequest {
    #[serde(rename = "assessmentArn")]
    #[serde(default)]
    pub assessment_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAppAssessmentResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment: Option<AppAssessment>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AppAssessment {
    #[serde(rename = "appArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_arn: Option<String>,
    #[serde(rename = "appVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_version: Option<String>,
    #[serde(rename = "assessmentArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_arn: Option<String>,
    #[serde(rename = "assessmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_name: Option<String>,
    #[serde(rename = "assessmentStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance: Option<std::collections::HashMap<String, DisruptionCompliance>>,
    #[serde(rename = "complianceStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<Cost>,
    #[serde(rename = "driftStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drift_status: Option<String>,
    #[serde(rename = "endTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoker: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<ResiliencyPolicy>,
    #[serde(rename = "resiliencyScore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resiliency_score: Option<ResiliencyScore>,
    #[serde(rename = "resourceErrorsDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_errors_details: Option<ResourceErrorsDetails>,
    #[serde(rename = "startTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<AssessmentSummary>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "versionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisruptionCompliance {
    #[serde(rename = "achievableRpoInSecs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub achievable_rpo_in_secs: Option<i32>,
    #[serde(rename = "achievableRtoInSecs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub achievable_rto_in_secs: Option<i32>,
    #[serde(rename = "complianceStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_status: Option<String>,
    #[serde(rename = "currentRpoInSecs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_rpo_in_secs: Option<i32>,
    #[serde(rename = "currentRtoInSecs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_rto_in_secs: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "rpoDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rpo_description: Option<String>,
    #[serde(rename = "rpoReferenceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rpo_reference_id: Option<String>,
    #[serde(rename = "rtoDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rto_description: Option<String>,
    #[serde(rename = "rtoReferenceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rto_reference_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Cost {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResiliencyScore {
    #[serde(rename = "componentScore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_score: Option<std::collections::HashMap<String, ScoringComponentResiliencyScore>>,
    #[serde(rename = "disruptionScore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disruption_score: Option<std::collections::HashMap<String, f64>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScoringComponentResiliencyScore {
    #[serde(rename = "excludedCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_count: Option<i64>,
    #[serde(rename = "outstandingCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outstanding_count: Option<i64>,
    #[serde(rename = "possibleScore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub possible_score: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceErrorsDetails {
    #[serde(rename = "hasMoreErrors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more_errors: Option<bool>,
    #[serde(rename = "resourceErrors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_errors: Option<Vec<ResourceError>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceError {
    #[serde(rename = "logicalResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logical_resource_id: Option<String>,
    #[serde(rename = "physicalResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_resource_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssessmentSummary {
    #[serde(rename = "riskRecommendations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_recommendations: Option<Vec<AssessmentRiskRecommendation>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssessmentRiskRecommendation {
    #[serde(rename = "appComponents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_components: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAppRequest {
    #[serde(rename = "appArn")]
    #[serde(default)]
    pub app_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAppResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app: Option<App>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAppVersionAppComponentRequest {
    #[serde(rename = "appArn")]
    #[serde(default)]
    pub app_arn: String,
    #[serde(rename = "appVersion")]
    #[serde(default)]
    pub app_version: String,
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAppVersionAppComponentResponse {
    #[serde(rename = "appArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_arn: Option<String>,
    #[serde(rename = "appComponent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_component: Option<AppComponent>,
    #[serde(rename = "appVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAppVersionRequest {
    #[serde(rename = "appArn")]
    #[serde(default)]
    pub app_arn: String,
    #[serde(rename = "appVersion")]
    #[serde(default)]
    pub app_version: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAppVersionResourceRequest {
    #[serde(rename = "appArn")]
    #[serde(default)]
    pub app_arn: String,
    #[serde(rename = "appVersion")]
    #[serde(default)]
    pub app_version: String,
    #[serde(rename = "awsAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_account_id: Option<String>,
    #[serde(rename = "awsRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_region: Option<String>,
    #[serde(rename = "logicalResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logical_resource_id: Option<LogicalResourceId>,
    #[serde(rename = "physicalResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_resource_id: Option<String>,
    #[serde(rename = "resourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAppVersionResourceResponse {
    #[serde(rename = "appArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_arn: Option<String>,
    #[serde(rename = "appVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_version: Option<String>,
    #[serde(rename = "physicalResource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_resource: Option<PhysicalResource>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAppVersionResourcesResolutionStatusRequest {
    #[serde(rename = "appArn")]
    #[serde(default)]
    pub app_arn: String,
    #[serde(rename = "appVersion")]
    #[serde(default)]
    pub app_version: String,
    #[serde(rename = "resolutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAppVersionResourcesResolutionStatusResponse {
    #[serde(rename = "appArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_arn: Option<String>,
    #[serde(rename = "appVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_version: Option<String>,
    #[serde(rename = "errorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "resolutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAppVersionResponse {
    #[serde(rename = "additionalInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "appArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_arn: Option<String>,
    #[serde(rename = "appVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAppVersionTemplateRequest {
    #[serde(rename = "appArn")]
    #[serde(default)]
    pub app_arn: String,
    #[serde(rename = "appVersion")]
    #[serde(default)]
    pub app_version: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAppVersionTemplateResponse {
    #[serde(rename = "appArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_arn: Option<String>,
    #[serde(rename = "appTemplateBody")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_template_body: Option<String>,
    #[serde(rename = "appVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDraftAppVersionResourcesImportStatusRequest {
    #[serde(rename = "appArn")]
    #[serde(default)]
    pub app_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDraftAppVersionResourcesImportStatusResponse {
    #[serde(rename = "appArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_arn: Option<String>,
    #[serde(rename = "appVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_version: Option<String>,
    #[serde(rename = "errorDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_details: Option<Vec<ErrorDetail>>,
    #[serde(rename = "errorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusChangeTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_change_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ErrorDetail {
    #[serde(rename = "errorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeMetricsExportRequest {
    #[serde(rename = "metricsExportId")]
    #[serde(default)]
    pub metrics_export_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeMetricsExportResponse {
    #[serde(rename = "errorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "exportLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_location: Option<S3Location>,
    #[serde(rename = "metricsExportId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics_export_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeResiliencyPolicyRequest {
    #[serde(rename = "policyArn")]
    #[serde(default)]
    pub policy_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeResiliencyPolicyResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<ResiliencyPolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeResourceGroupingRecommendationTaskRequest {
    #[serde(rename = "appArn")]
    #[serde(default)]
    pub app_arn: String,
    #[serde(rename = "groupingId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grouping_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeResourceGroupingRecommendationTaskResponse {
    #[serde(rename = "errorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "groupingId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grouping_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportResourcesToDraftAppVersionRequest {
    #[serde(rename = "appArn")]
    #[serde(default)]
    pub app_arn: String,
    #[serde(rename = "eksSources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eks_sources: Option<Vec<EksSource>>,
    #[serde(rename = "importStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_strategy: Option<String>,
    #[serde(rename = "sourceArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_arns: Option<Vec<String>>,
    #[serde(rename = "terraformSources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terraform_sources: Option<Vec<TerraformSource>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EksSource {
    #[serde(rename = "eksClusterArn")]
    #[serde(default)]
    pub eks_cluster_arn: String,
    #[serde(default)]
    pub namespaces: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportResourcesToDraftAppVersionResponse {
    #[serde(rename = "appArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_arn: Option<String>,
    #[serde(rename = "appVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_version: Option<String>,
    #[serde(rename = "eksSources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eks_sources: Option<Vec<EksSource>>,
    #[serde(rename = "sourceArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_arns: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "terraformSources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terraform_sources: Option<Vec<TerraformSource>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAlarmRecommendationsRequest {
    #[serde(rename = "assessmentArn")]
    #[serde(default)]
    pub assessment_arn: String,
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
pub struct ListAlarmRecommendationsResponse {
    #[serde(rename = "alarmRecommendations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_recommendations: Option<Vec<AlarmRecommendation>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AlarmRecommendation {
    #[serde(rename = "appComponentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_component_name: Option<String>,
    #[serde(rename = "appComponentNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_component_names: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<RecommendationItem>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prerequisite: Option<String>,
    #[serde(rename = "recommendationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_id: Option<String>,
    #[serde(rename = "recommendationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_status: Option<String>,
    #[serde(rename = "referenceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecommendationItem {
    #[serde(rename = "alreadyImplemented")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub already_implemented: Option<bool>,
    #[serde(rename = "discoveredAlarm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discovered_alarm: Option<Alarm>,
    #[serde(rename = "excludeReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_reason: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded: Option<bool>,
    #[serde(rename = "latestDiscoveredExperiment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_discovered_experiment: Option<Experiment>,
    #[serde(rename = "resourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "targetAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_account_id: Option<String>,
    #[serde(rename = "targetRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_region: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Alarm {
    #[serde(rename = "alarmArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Experiment {
    #[serde(rename = "experimentArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experiment_arn: Option<String>,
    #[serde(rename = "experimentTemplateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experiment_template_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAppAssessmentComplianceDriftsRequest {
    #[serde(rename = "assessmentArn")]
    #[serde(default)]
    pub assessment_arn: String,
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
pub struct ListAppAssessmentComplianceDriftsResponse {
    #[serde(rename = "complianceDrifts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_drifts: Option<Vec<ComplianceDrift>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ComplianceDrift {
    #[serde(rename = "actualReferenceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_reference_id: Option<String>,
    #[serde(rename = "actualValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_value: Option<std::collections::HashMap<String, DisruptionCompliance>>,
    #[serde(rename = "appId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(rename = "appVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_version: Option<String>,
    #[serde(rename = "diffType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diff_type: Option<String>,
    #[serde(rename = "driftType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drift_type: Option<String>,
    #[serde(rename = "entityId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    #[serde(rename = "entityType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<String>,
    #[serde(rename = "expectedReferenceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_reference_id: Option<String>,
    #[serde(rename = "expectedValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_value: Option<std::collections::HashMap<String, DisruptionCompliance>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAppAssessmentResourceDriftsRequest {
    #[serde(rename = "assessmentArn")]
    #[serde(default)]
    pub assessment_arn: String,
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
pub struct ListAppAssessmentResourceDriftsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "resourceDrifts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_drifts: Option<Vec<ResourceDrift>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceDrift {
    #[serde(rename = "appArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_arn: Option<String>,
    #[serde(rename = "appVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_version: Option<String>,
    #[serde(rename = "diffType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diff_type: Option<String>,
    #[serde(rename = "referenceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
    #[serde(rename = "resourceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_identifier: Option<ResourceIdentifier>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceIdentifier {
    #[serde(rename = "logicalResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logical_resource_id: Option<LogicalResourceId>,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAppAssessmentsRequest {
    #[serde(rename = "appArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_arn: Option<String>,
    #[serde(rename = "assessmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_name: Option<String>,
    #[serde(rename = "assessmentStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_status: Option<Vec<String>>,
    #[serde(rename = "complianceStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoker: Option<String>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "reverseOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_order: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAppAssessmentsResponse {
    #[serde(rename = "assessmentSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_summaries: Option<Vec<AppAssessmentSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AppAssessmentSummary {
    #[serde(rename = "appArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_arn: Option<String>,
    #[serde(rename = "appVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_version: Option<String>,
    #[serde(rename = "assessmentArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_arn: Option<String>,
    #[serde(rename = "assessmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_name: Option<String>,
    #[serde(rename = "assessmentStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_status: Option<String>,
    #[serde(rename = "complianceStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<Cost>,
    #[serde(rename = "driftStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drift_status: Option<String>,
    #[serde(rename = "endTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoker: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "resiliencyScore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resiliency_score: Option<f64>,
    #[serde(rename = "startTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(rename = "versionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAppComponentCompliancesRequest {
    #[serde(rename = "assessmentArn")]
    #[serde(default)]
    pub assessment_arn: String,
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
pub struct ListAppComponentCompliancesResponse {
    #[serde(rename = "componentCompliances")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_compliances: Option<Vec<AppComponentCompliance>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AppComponentCompliance {
    #[serde(rename = "appComponentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_component_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance: Option<std::collections::HashMap<String, DisruptionCompliance>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<Cost>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "resiliencyScore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resiliency_score: Option<ResiliencyScore>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAppComponentRecommendationsRequest {
    #[serde(rename = "assessmentArn")]
    #[serde(default)]
    pub assessment_arn: String,
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
pub struct ListAppComponentRecommendationsResponse {
    #[serde(rename = "componentRecommendations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_recommendations: Option<Vec<ComponentRecommendation>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ComponentRecommendation {
    #[serde(rename = "appComponentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_component_name: Option<String>,
    #[serde(rename = "configRecommendations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_recommendations: Option<Vec<ConfigRecommendation>>,
    #[serde(rename = "recommendationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConfigRecommendation {
    #[serde(rename = "appComponentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_component_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance: Option<std::collections::HashMap<String, DisruptionCompliance>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<Cost>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "haArchitecture")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ha_architecture: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "optimizationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optimization_type: Option<String>,
    #[serde(rename = "recommendationCompliance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_compliance:
        Option<std::collections::HashMap<String, RecommendationDisruptionCompliance>>,
    #[serde(rename = "referenceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
    #[serde(rename = "suggestedChanges")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_changes: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecommendationDisruptionCompliance {
    #[serde(rename = "expectedComplianceStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_compliance_status: Option<String>,
    #[serde(rename = "expectedRpoDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_rpo_description: Option<String>,
    #[serde(rename = "expectedRpoInSecs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_rpo_in_secs: Option<i32>,
    #[serde(rename = "expectedRtoDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_rto_description: Option<String>,
    #[serde(rename = "expectedRtoInSecs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_rto_in_secs: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAppInputSourcesRequest {
    #[serde(rename = "appArn")]
    #[serde(default)]
    pub app_arn: String,
    #[serde(rename = "appVersion")]
    #[serde(default)]
    pub app_version: String,
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
pub struct ListAppInputSourcesResponse {
    #[serde(rename = "appInputSources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_input_sources: Option<Vec<AppInputSource>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAppVersionAppComponentsRequest {
    #[serde(rename = "appArn")]
    #[serde(default)]
    pub app_arn: String,
    #[serde(rename = "appVersion")]
    #[serde(default)]
    pub app_version: String,
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
pub struct ListAppVersionAppComponentsResponse {
    #[serde(rename = "appArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_arn: Option<String>,
    #[serde(rename = "appComponents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_components: Option<Vec<AppComponent>>,
    #[serde(rename = "appVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_version: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAppVersionResourceMappingsRequest {
    #[serde(rename = "appArn")]
    #[serde(default)]
    pub app_arn: String,
    #[serde(rename = "appVersion")]
    #[serde(default)]
    pub app_version: String,
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
pub struct ListAppVersionResourceMappingsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "resourceMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_mappings: Option<Vec<ResourceMapping>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAppVersionResourcesRequest {
    #[serde(rename = "appArn")]
    #[serde(default)]
    pub app_arn: String,
    #[serde(rename = "appVersion")]
    #[serde(default)]
    pub app_version: String,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "resolutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAppVersionResourcesResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "physicalResources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_resources: Option<Vec<PhysicalResource>>,
    #[serde(rename = "resolutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAppVersionsRequest {
    #[serde(rename = "appArn")]
    #[serde(default)]
    pub app_arn: String,
    #[serde(rename = "endTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "startTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAppVersionsResponse {
    #[serde(rename = "appVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_versions: Option<Vec<AppVersionSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AppVersionSummary {
    #[serde(rename = "appVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_version: Option<String>,
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<i64>,
    #[serde(rename = "versionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAppsRequest {
    #[serde(rename = "appArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_arn: Option<String>,
    #[serde(rename = "awsApplicationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_application_arn: Option<String>,
    #[serde(rename = "fromLastAssessmentTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_last_assessment_time: Option<f64>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "reverseOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_order: Option<bool>,
    #[serde(rename = "toLastAssessmentTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_last_assessment_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAppsResponse {
    #[serde(rename = "appSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_summaries: Option<Vec<AppSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AppSummary {
    #[serde(rename = "appArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_arn: Option<String>,
    #[serde(rename = "assessmentSchedule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_schedule: Option<String>,
    #[serde(rename = "awsApplicationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_application_arn: Option<String>,
    #[serde(rename = "complianceStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_status: Option<String>,
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "driftStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drift_status: Option<String>,
    #[serde(rename = "lastAppComplianceEvaluationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_app_compliance_evaluation_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "resiliencyScore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resiliency_score: Option<f64>,
    #[serde(rename = "rpoInSecs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rpo_in_secs: Option<i32>,
    #[serde(rename = "rtoInSecs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rto_in_secs: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListMetricsRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    #[serde(rename = "dataSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<Field>>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sorts: Option<Vec<Sort>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Condition {
    #[serde(default)]
    pub field: String,
    #[serde(default)]
    pub operator: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Field {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation: Option<String>,
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Sort {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ascending: Option<bool>,
    #[serde(default)]
    pub field: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListMetricsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rows: Option<Vec<Vec<String>>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRecommendationTemplatesRequest {
    #[serde(rename = "assessmentArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_arn: Option<String>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "recommendationTemplateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_template_arn: Option<String>,
    #[serde(rename = "reverseOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_order: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRecommendationTemplatesResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "recommendationTemplates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_templates: Option<Vec<RecommendationTemplate>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListResiliencyPoliciesRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "policyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListResiliencyPoliciesResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "resiliencyPolicies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resiliency_policies: Option<Vec<ResiliencyPolicy>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListResourceGroupingRecommendationsRequest {
    #[serde(rename = "appArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_arn: Option<String>,
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
pub struct ListResourceGroupingRecommendationsResponse {
    #[serde(rename = "groupingRecommendations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grouping_recommendations: Option<Vec<GroupingRecommendation>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GroupingRecommendation {
    #[serde(rename = "confidenceLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence_level: Option<String>,
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "groupingAppComponent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grouping_app_component: Option<GroupingAppComponent>,
    #[serde(rename = "groupingRecommendationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grouping_recommendation_id: Option<String>,
    #[serde(rename = "recommendationReasons")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_reasons: Option<Vec<String>>,
    #[serde(rename = "rejectionReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejection_reason: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<GroupingResource>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GroupingAppComponent {
    #[serde(rename = "appComponentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_component_id: Option<String>,
    #[serde(rename = "appComponentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_component_name: Option<String>,
    #[serde(rename = "appComponentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_component_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GroupingResource {
    #[serde(rename = "logicalResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logical_resource_id: Option<LogicalResourceId>,
    #[serde(rename = "physicalResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_resource_id: Option<PhysicalResourceId>,
    #[serde(rename = "resourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "sourceAppComponentIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_app_component_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSopRecommendationsRequest {
    #[serde(rename = "assessmentArn")]
    #[serde(default)]
    pub assessment_arn: String,
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
pub struct ListSopRecommendationsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "sopRecommendations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sop_recommendations: Option<Vec<SopRecommendation>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SopRecommendation {
    #[serde(rename = "appComponentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_component_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<RecommendationItem>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prerequisite: Option<String>,
    #[serde(rename = "recommendationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_id: Option<String>,
    #[serde(rename = "recommendationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_status: Option<String>,
    #[serde(rename = "referenceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
    #[serde(rename = "serviceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSuggestedResiliencyPoliciesRequest {
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
pub struct ListSuggestedResiliencyPoliciesResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "resiliencyPolicies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resiliency_policies: Option<Vec<ResiliencyPolicy>>,
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
pub struct ListTestRecommendationsRequest {
    #[serde(rename = "assessmentArn")]
    #[serde(default)]
    pub assessment_arn: String,
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
pub struct ListTestRecommendationsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "testRecommendations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_recommendations: Option<Vec<TestRecommendation>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TestRecommendation {
    #[serde(rename = "appComponentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_component_id: Option<String>,
    #[serde(rename = "appComponentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_component_name: Option<String>,
    #[serde(rename = "dependsOnAlarms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depends_on_alarms: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<RecommendationItem>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prerequisite: Option<String>,
    #[serde(rename = "recommendationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_id: Option<String>,
    #[serde(rename = "recommendationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_status: Option<String>,
    #[serde(rename = "referenceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListUnsupportedAppVersionResourcesRequest {
    #[serde(rename = "appArn")]
    #[serde(default)]
    pub app_arn: String,
    #[serde(rename = "appVersion")]
    #[serde(default)]
    pub app_version: String,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "resolutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListUnsupportedAppVersionResourcesResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "resolutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution_id: Option<String>,
    #[serde(rename = "unsupportedResources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsupported_resources: Option<Vec<UnsupportedResource>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UnsupportedResource {
    #[serde(rename = "logicalResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logical_resource_id: Option<LogicalResourceId>,
    #[serde(rename = "physicalResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_resource_id: Option<PhysicalResourceId>,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "unsupportedResourceStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsupported_resource_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PublishAppVersionRequest {
    #[serde(rename = "appArn")]
    #[serde(default)]
    pub app_arn: String,
    #[serde(rename = "versionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PublishAppVersionResponse {
    #[serde(rename = "appArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_arn: Option<String>,
    #[serde(rename = "appVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_version: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<i64>,
    #[serde(rename = "versionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutDraftAppVersionTemplateRequest {
    #[serde(rename = "appArn")]
    #[serde(default)]
    pub app_arn: String,
    #[serde(rename = "appTemplateBody")]
    #[serde(default)]
    pub app_template_body: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutDraftAppVersionTemplateResponse {
    #[serde(rename = "appArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_arn: Option<String>,
    #[serde(rename = "appVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RejectResourceGroupingRecommendationsRequest {
    #[serde(rename = "appArn")]
    #[serde(default)]
    pub app_arn: String,
    #[serde(default)]
    pub entries: Vec<RejectGroupingRecommendationEntry>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RejectGroupingRecommendationEntry {
    #[serde(rename = "groupingRecommendationId")]
    #[serde(default)]
    pub grouping_recommendation_id: String,
    #[serde(rename = "rejectionReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejection_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RejectResourceGroupingRecommendationsResponse {
    #[serde(rename = "appArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_arn: Option<String>,
    #[serde(rename = "failedEntries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_entries: Option<Vec<FailedGroupingRecommendationEntry>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveDraftAppVersionResourceMappingsRequest {
    #[serde(rename = "appArn")]
    #[serde(default)]
    pub app_arn: String,
    #[serde(rename = "appRegistryAppNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_registry_app_names: Option<Vec<String>>,
    #[serde(rename = "eksSourceNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eks_source_names: Option<Vec<String>>,
    #[serde(rename = "logicalStackNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logical_stack_names: Option<Vec<String>>,
    #[serde(rename = "resourceGroupNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_names: Option<Vec<String>>,
    #[serde(rename = "resourceNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_names: Option<Vec<String>>,
    #[serde(rename = "terraformSourceNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terraform_source_names: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveDraftAppVersionResourceMappingsResponse {
    #[serde(rename = "appArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_arn: Option<String>,
    #[serde(rename = "appVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResolveAppVersionResourcesRequest {
    #[serde(rename = "appArn")]
    #[serde(default)]
    pub app_arn: String,
    #[serde(rename = "appVersion")]
    #[serde(default)]
    pub app_version: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResolveAppVersionResourcesResponse {
    #[serde(rename = "appArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_arn: Option<String>,
    #[serde(rename = "appVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_version: Option<String>,
    #[serde(rename = "resolutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartAppAssessmentRequest {
    #[serde(rename = "appArn")]
    #[serde(default)]
    pub app_arn: String,
    #[serde(rename = "appVersion")]
    #[serde(default)]
    pub app_version: String,
    #[serde(rename = "assessmentName")]
    #[serde(default)]
    pub assessment_name: String,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartAppAssessmentResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment: Option<AppAssessment>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartMetricsExportRequest {
    #[serde(rename = "bucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<String>,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartMetricsExportResponse {
    #[serde(rename = "metricsExportId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics_export_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartResourceGroupingRecommendationTaskRequest {
    #[serde(rename = "appArn")]
    #[serde(default)]
    pub app_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartResourceGroupingRecommendationTaskResponse {
    #[serde(rename = "appArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_arn: Option<String>,
    #[serde(rename = "errorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "groupingId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grouping_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
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
pub struct UpdateAppRequest {
    #[serde(rename = "appArn")]
    #[serde(default)]
    pub app_arn: String,
    #[serde(rename = "assessmentSchedule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_schedule: Option<String>,
    #[serde(rename = "clearResiliencyPolicyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clear_resiliency_policy_arn: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "eventSubscriptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_subscriptions: Option<Vec<EventSubscription>>,
    #[serde(rename = "permissionModel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_model: Option<PermissionModel>,
    #[serde(rename = "policyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAppResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app: Option<App>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAppVersionAppComponentRequest {
    #[serde(rename = "additionalInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "appArn")]
    #[serde(default)]
    pub app_arn: String,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAppVersionAppComponentResponse {
    #[serde(rename = "appArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_arn: Option<String>,
    #[serde(rename = "appComponent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_component: Option<AppComponent>,
    #[serde(rename = "appVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAppVersionRequest {
    #[serde(rename = "additionalInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "appArn")]
    #[serde(default)]
    pub app_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAppVersionResourceRequest {
    #[serde(rename = "additionalInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "appArn")]
    #[serde(default)]
    pub app_arn: String,
    #[serde(rename = "appComponents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_components: Option<Vec<String>>,
    #[serde(rename = "awsAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_account_id: Option<String>,
    #[serde(rename = "awsRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_region: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded: Option<bool>,
    #[serde(rename = "logicalResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logical_resource_id: Option<LogicalResourceId>,
    #[serde(rename = "physicalResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_resource_id: Option<String>,
    #[serde(rename = "resourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAppVersionResourceResponse {
    #[serde(rename = "appArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_arn: Option<String>,
    #[serde(rename = "appVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_version: Option<String>,
    #[serde(rename = "physicalResource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_resource: Option<PhysicalResource>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAppVersionResponse {
    #[serde(rename = "additionalInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "appArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_arn: Option<String>,
    #[serde(rename = "appVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateResiliencyPolicyRequest {
    #[serde(rename = "dataLocationConstraint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_location_constraint: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<std::collections::HashMap<String, FailurePolicy>>,
    #[serde(rename = "policyArn")]
    #[serde(default)]
    pub policy_arn: String,
    #[serde(rename = "policyDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_description: Option<String>,
    #[serde(rename = "policyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateResiliencyPolicyResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<ResiliencyPolicy>,
}
