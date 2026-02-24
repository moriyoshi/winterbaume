//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-config

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateResourceTypesRequest {
    #[serde(rename = "ConfigurationRecorderArn")]
    #[serde(default)]
    pub configuration_recorder_arn: String,
    #[serde(rename = "ResourceTypes")]
    #[serde(default)]
    pub resource_types: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateResourceTypesResponse {
    #[serde(rename = "ConfigurationRecorder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_recorder: Option<ConfigurationRecorder>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConfigurationRecorder {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "recordingGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording_group: Option<RecordingGroup>,
    #[serde(rename = "recordingMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording_mode: Option<RecordingMode>,
    #[serde(rename = "recordingScope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording_scope: Option<String>,
    #[serde(rename = "roleARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_a_r_n: Option<String>,
    #[serde(rename = "servicePrincipal")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_principal: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecordingGroup {
    #[serde(rename = "allSupported")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_supported: Option<bool>,
    #[serde(rename = "exclusionByResourceTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusion_by_resource_types: Option<ExclusionByResourceTypes>,
    #[serde(rename = "includeGlobalResourceTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_global_resource_types: Option<bool>,
    #[serde(rename = "recordingStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording_strategy: Option<RecordingStrategy>,
    #[serde(rename = "resourceTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_types: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExclusionByResourceTypes {
    #[serde(rename = "resourceTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_types: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecordingStrategy {
    #[serde(rename = "useOnly")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_only: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecordingMode {
    #[serde(rename = "recordingFrequency")]
    #[serde(default)]
    pub recording_frequency: String,
    #[serde(rename = "recordingModeOverrides")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording_mode_overrides: Option<Vec<RecordingModeOverride>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecordingModeOverride {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "recordingFrequency")]
    #[serde(default)]
    pub recording_frequency: String,
    #[serde(rename = "resourceTypes")]
    #[serde(default)]
    pub resource_types: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetAggregateResourceConfigRequest {
    #[serde(rename = "ConfigurationAggregatorName")]
    #[serde(default)]
    pub configuration_aggregator_name: String,
    #[serde(rename = "ResourceIdentifiers")]
    #[serde(default)]
    pub resource_identifiers: Vec<AggregateResourceIdentifier>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AggregateResourceIdentifier {
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "ResourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    pub resource_type: String,
    #[serde(rename = "SourceAccountId")]
    #[serde(default)]
    pub source_account_id: String,
    #[serde(rename = "SourceRegion")]
    #[serde(default)]
    pub source_region: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetAggregateResourceConfigResponse {
    #[serde(rename = "BaseConfigurationItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_configuration_items: Option<Vec<BaseConfigurationItem>>,
    #[serde(rename = "UnprocessedResourceIdentifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_resource_identifiers: Option<Vec<AggregateResourceIdentifier>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BaseConfigurationItem {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "availabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "awsRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_region: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<String>,
    #[serde(rename = "configurationItemCaptureTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_item_capture_time: Option<f64>,
    #[serde(rename = "configurationItemDeliveryTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_item_delivery_time: Option<f64>,
    #[serde(rename = "configurationItemStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_item_status: Option<String>,
    #[serde(rename = "configurationStateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_state_id: Option<String>,
    #[serde(rename = "recordingFrequency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording_frequency: Option<String>,
    #[serde(rename = "resourceCreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_creation_time: Option<f64>,
    #[serde(rename = "resourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "resourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "supplementaryConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplementary_configuration: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetResourceConfigRequest {
    #[serde(rename = "resourceKeys")]
    #[serde(default)]
    pub resource_keys: Vec<ResourceKey>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceKey {
    #[serde(rename = "resourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    pub resource_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetResourceConfigResponse {
    #[serde(rename = "baseConfigurationItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_configuration_items: Option<Vec<BaseConfigurationItem>>,
    #[serde(rename = "unprocessedResourceKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_resource_keys: Option<Vec<ResourceKey>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAggregationAuthorizationRequest {
    #[serde(rename = "AuthorizedAccountId")]
    #[serde(default)]
    pub authorized_account_id: String,
    #[serde(rename = "AuthorizedAwsRegion")]
    #[serde(default)]
    pub authorized_aws_region: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteConfigRuleRequest {
    #[serde(rename = "ConfigRuleName")]
    #[serde(default)]
    pub config_rule_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteConfigurationAggregatorRequest {
    #[serde(rename = "ConfigurationAggregatorName")]
    #[serde(default)]
    pub configuration_aggregator_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteConfigurationRecorderRequest {
    #[serde(rename = "ConfigurationRecorderName")]
    #[serde(default)]
    pub configuration_recorder_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteConformancePackRequest {
    #[serde(rename = "ConformancePackName")]
    #[serde(default)]
    pub conformance_pack_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDeliveryChannelRequest {
    #[serde(rename = "DeliveryChannelName")]
    #[serde(default)]
    pub delivery_channel_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteEvaluationResultsRequest {
    #[serde(rename = "ConfigRuleName")]
    #[serde(default)]
    pub config_rule_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteEvaluationResultsResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteOrganizationConfigRuleRequest {
    #[serde(rename = "OrganizationConfigRuleName")]
    #[serde(default)]
    pub organization_config_rule_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteOrganizationConformancePackRequest {
    #[serde(rename = "OrganizationConformancePackName")]
    #[serde(default)]
    pub organization_conformance_pack_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePendingAggregationRequestRequest {
    #[serde(rename = "RequesterAccountId")]
    #[serde(default)]
    pub requester_account_id: String,
    #[serde(rename = "RequesterAwsRegion")]
    #[serde(default)]
    pub requester_aws_region: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRemediationConfigurationRequest {
    #[serde(rename = "ConfigRuleName")]
    #[serde(default)]
    pub config_rule_name: String,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRemediationConfigurationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRemediationExceptionsRequest {
    #[serde(rename = "ConfigRuleName")]
    #[serde(default)]
    pub config_rule_name: String,
    #[serde(rename = "ResourceKeys")]
    #[serde(default)]
    pub resource_keys: Vec<RemediationExceptionResourceKey>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemediationExceptionResourceKey {
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
pub struct DeleteRemediationExceptionsResponse {
    #[serde(rename = "FailedBatches")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_batches: Option<Vec<FailedDeleteRemediationExceptionsBatch>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FailedDeleteRemediationExceptionsBatch {
    #[serde(rename = "FailedItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_items: Option<Vec<RemediationExceptionResourceKey>>,
    #[serde(rename = "FailureMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteResourceConfigRequest {
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    pub resource_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRetentionConfigurationRequest {
    #[serde(rename = "RetentionConfigurationName")]
    #[serde(default)]
    pub retention_configuration_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteServiceLinkedConfigurationRecorderRequest {
    #[serde(rename = "ServicePrincipal")]
    #[serde(default)]
    pub service_principal: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteServiceLinkedConfigurationRecorderResponse {
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
pub struct DeleteStoredQueryRequest {
    #[serde(rename = "QueryName")]
    #[serde(default)]
    pub query_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteStoredQueryResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeliverConfigSnapshotRequest {
    #[serde(rename = "deliveryChannelName")]
    #[serde(default)]
    pub delivery_channel_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeliverConfigSnapshotResponse {
    #[serde(rename = "configSnapshotId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_snapshot_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAggregateComplianceByConfigRulesRequest {
    #[serde(rename = "ConfigurationAggregatorName")]
    #[serde(default)]
    pub configuration_aggregator_name: String,
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<ConfigRuleComplianceFilters>,
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
pub struct ConfigRuleComplianceFilters {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "AwsRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_region: Option<String>,
    #[serde(rename = "ComplianceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_type: Option<String>,
    #[serde(rename = "ConfigRuleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rule_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAggregateComplianceByConfigRulesResponse {
    #[serde(rename = "AggregateComplianceByConfigRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_compliance_by_config_rules: Option<Vec<AggregateComplianceByConfigRule>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AggregateComplianceByConfigRule {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "AwsRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_region: Option<String>,
    #[serde(rename = "Compliance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance: Option<Compliance>,
    #[serde(rename = "ConfigRuleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rule_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Compliance {
    #[serde(rename = "ComplianceContributorCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_contributor_count: Option<ComplianceContributorCount>,
    #[serde(rename = "ComplianceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ComplianceContributorCount {
    #[serde(rename = "CapExceeded")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cap_exceeded: Option<bool>,
    #[serde(rename = "CappedCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capped_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAggregateComplianceByConformancePacksRequest {
    #[serde(rename = "ConfigurationAggregatorName")]
    #[serde(default)]
    pub configuration_aggregator_name: String,
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<AggregateConformancePackComplianceFilters>,
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
pub struct AggregateConformancePackComplianceFilters {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "AwsRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_region: Option<String>,
    #[serde(rename = "ComplianceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_type: Option<String>,
    #[serde(rename = "ConformancePackName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conformance_pack_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAggregateComplianceByConformancePacksResponse {
    #[serde(rename = "AggregateComplianceByConformancePacks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_compliance_by_conformance_packs:
        Option<Vec<AggregateComplianceByConformancePack>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AggregateComplianceByConformancePack {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "AwsRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_region: Option<String>,
    #[serde(rename = "Compliance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance: Option<AggregateConformancePackCompliance>,
    #[serde(rename = "ConformancePackName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conformance_pack_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AggregateConformancePackCompliance {
    #[serde(rename = "ComplianceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_type: Option<String>,
    #[serde(rename = "CompliantRuleCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliant_rule_count: Option<i32>,
    #[serde(rename = "NonCompliantRuleCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_compliant_rule_count: Option<i32>,
    #[serde(rename = "TotalRuleCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_rule_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAggregationAuthorizationsRequest {
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
pub struct DescribeAggregationAuthorizationsResponse {
    #[serde(rename = "AggregationAuthorizations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_authorizations: Option<Vec<AggregationAuthorization>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AggregationAuthorization {
    #[serde(rename = "AggregationAuthorizationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_authorization_arn: Option<String>,
    #[serde(rename = "AuthorizedAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_account_id: Option<String>,
    #[serde(rename = "AuthorizedAwsRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_aws_region: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeComplianceByConfigRuleRequest {
    #[serde(rename = "ComplianceTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_types: Option<Vec<String>>,
    #[serde(rename = "ConfigRuleNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rule_names: Option<Vec<String>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeComplianceByConfigRuleResponse {
    #[serde(rename = "ComplianceByConfigRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_by_config_rules: Option<Vec<ComplianceByConfigRule>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ComplianceByConfigRule {
    #[serde(rename = "Compliance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance: Option<Compliance>,
    #[serde(rename = "ConfigRuleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rule_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeComplianceByResourceRequest {
    #[serde(rename = "ComplianceTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_types: Option<Vec<String>>,
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
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
pub struct DescribeComplianceByResourceResponse {
    #[serde(rename = "ComplianceByResources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_by_resources: Option<Vec<ComplianceByResource>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ComplianceByResource {
    #[serde(rename = "Compliance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance: Option<Compliance>,
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
pub struct DescribeConfigRuleEvaluationStatusRequest {
    #[serde(rename = "ConfigRuleNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rule_names: Option<Vec<String>>,
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
pub struct DescribeConfigRuleEvaluationStatusResponse {
    #[serde(rename = "ConfigRulesEvaluationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rules_evaluation_status: Option<Vec<ConfigRuleEvaluationStatus>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConfigRuleEvaluationStatus {
    #[serde(rename = "ConfigRuleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rule_arn: Option<String>,
    #[serde(rename = "ConfigRuleId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rule_id: Option<String>,
    #[serde(rename = "ConfigRuleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rule_name: Option<String>,
    #[serde(rename = "FirstActivatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_activated_time: Option<f64>,
    #[serde(rename = "FirstEvaluationStarted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_evaluation_started: Option<bool>,
    #[serde(rename = "LastDeactivatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_deactivated_time: Option<f64>,
    #[serde(rename = "LastDebugLogDeliveryStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_debug_log_delivery_status: Option<String>,
    #[serde(rename = "LastDebugLogDeliveryStatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_debug_log_delivery_status_reason: Option<String>,
    #[serde(rename = "LastDebugLogDeliveryTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_debug_log_delivery_time: Option<f64>,
    #[serde(rename = "LastErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_code: Option<String>,
    #[serde(rename = "LastErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_message: Option<String>,
    #[serde(rename = "LastFailedEvaluationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_failed_evaluation_time: Option<f64>,
    #[serde(rename = "LastFailedInvocationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_failed_invocation_time: Option<f64>,
    #[serde(rename = "LastSuccessfulEvaluationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_successful_evaluation_time: Option<f64>,
    #[serde(rename = "LastSuccessfulInvocationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_successful_invocation_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeConfigRulesRequest {
    #[serde(rename = "ConfigRuleNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rule_names: Option<Vec<String>>,
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<DescribeConfigRulesFilters>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeConfigRulesFilters {
    #[serde(rename = "EvaluationMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeConfigRulesResponse {
    #[serde(rename = "ConfigRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rules: Option<Vec<ConfigRule>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConfigRule {
    #[serde(rename = "ConfigRuleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rule_arn: Option<String>,
    #[serde(rename = "ConfigRuleId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rule_id: Option<String>,
    #[serde(rename = "ConfigRuleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rule_name: Option<String>,
    #[serde(rename = "ConfigRuleState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rule_state: Option<String>,
    #[serde(rename = "CreatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EvaluationModes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_modes: Option<Vec<EvaluationModeConfiguration>>,
    #[serde(rename = "InputParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_parameters: Option<String>,
    #[serde(rename = "MaximumExecutionFrequency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_execution_frequency: Option<String>,
    #[serde(rename = "Scope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<Scope>,
    #[serde(rename = "Source")]
    #[serde(default)]
    pub source: Source,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationModeConfiguration {
    #[serde(rename = "Mode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Scope {
    #[serde(rename = "ComplianceResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_resource_id: Option<String>,
    #[serde(rename = "ComplianceResourceTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_resource_types: Option<Vec<String>>,
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
pub struct Source {
    #[serde(rename = "CustomPolicyDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_policy_details: Option<CustomPolicyDetails>,
    #[serde(rename = "Owner")]
    #[serde(default)]
    pub owner: String,
    #[serde(rename = "SourceDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_details: Option<Vec<SourceDetail>>,
    #[serde(rename = "SourceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomPolicyDetails {
    #[serde(rename = "EnableDebugLogDelivery")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_debug_log_delivery: Option<bool>,
    #[serde(rename = "PolicyRuntime")]
    #[serde(default)]
    pub policy_runtime: String,
    #[serde(rename = "PolicyText")]
    #[serde(default)]
    pub policy_text: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SourceDetail {
    #[serde(rename = "EventSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_source: Option<String>,
    #[serde(rename = "MaximumExecutionFrequency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_execution_frequency: Option<String>,
    #[serde(rename = "MessageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeConfigurationAggregatorSourcesStatusRequest {
    #[serde(rename = "ConfigurationAggregatorName")]
    #[serde(default)]
    pub configuration_aggregator_name: String,
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "UpdateStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_status: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeConfigurationAggregatorSourcesStatusResponse {
    #[serde(rename = "AggregatedSourceStatusList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregated_source_status_list: Option<Vec<AggregatedSourceStatus>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AggregatedSourceStatus {
    #[serde(rename = "AwsRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_region: Option<String>,
    #[serde(rename = "LastErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_code: Option<String>,
    #[serde(rename = "LastErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_message: Option<String>,
    #[serde(rename = "LastUpdateStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_status: Option<String>,
    #[serde(rename = "LastUpdateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<f64>,
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
pub struct DescribeConfigurationAggregatorsRequest {
    #[serde(rename = "ConfigurationAggregatorNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_aggregator_names: Option<Vec<String>>,
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
pub struct DescribeConfigurationAggregatorsResponse {
    #[serde(rename = "ConfigurationAggregators")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_aggregators: Option<Vec<ConfigurationAggregator>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConfigurationAggregator {
    #[serde(rename = "AccountAggregationSources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_aggregation_sources: Option<Vec<AccountAggregationSource>>,
    #[serde(rename = "AggregatorFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregator_filters: Option<AggregatorFilters>,
    #[serde(rename = "ConfigurationAggregatorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_aggregator_arn: Option<String>,
    #[serde(rename = "ConfigurationAggregatorName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_aggregator_name: Option<String>,
    #[serde(rename = "CreatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "LastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(rename = "OrganizationAggregationSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_aggregation_source: Option<OrganizationAggregationSource>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccountAggregationSource {
    #[serde(rename = "AccountIds")]
    #[serde(default)]
    pub account_ids: Vec<String>,
    #[serde(rename = "AllAwsRegions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_aws_regions: Option<bool>,
    #[serde(rename = "AwsRegions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_regions: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AggregatorFilters {
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<AggregatorFilterResourceType>,
    #[serde(rename = "ServicePrincipal")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_principal: Option<AggregatorFilterServicePrincipal>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AggregatorFilterResourceType {
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AggregatorFilterServicePrincipal {
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OrganizationAggregationSource {
    #[serde(rename = "AllAwsRegions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_aws_regions: Option<bool>,
    #[serde(rename = "AwsRegions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_regions: Option<Vec<String>>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    pub role_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeConfigurationRecorderStatusRequest {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "ConfigurationRecorderNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_recorder_names: Option<Vec<String>>,
    #[serde(rename = "ServicePrincipal")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_principal: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeConfigurationRecorderStatusResponse {
    #[serde(rename = "ConfigurationRecordersStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_recorders_status: Option<Vec<ConfigurationRecorderStatus>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConfigurationRecorderStatus {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "lastErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_code: Option<String>,
    #[serde(rename = "lastErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_message: Option<String>,
    #[serde(rename = "lastStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_start_time: Option<f64>,
    #[serde(rename = "lastStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_status: Option<String>,
    #[serde(rename = "lastStatusChangeTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_status_change_time: Option<f64>,
    #[serde(rename = "lastStopTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_stop_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording: Option<bool>,
    #[serde(rename = "servicePrincipal")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_principal: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeConfigurationRecordersRequest {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "ConfigurationRecorderNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_recorder_names: Option<Vec<String>>,
    #[serde(rename = "ServicePrincipal")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_principal: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeConfigurationRecordersResponse {
    #[serde(rename = "ConfigurationRecorders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_recorders: Option<Vec<ConfigurationRecorder>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeConformancePackComplianceRequest {
    #[serde(rename = "ConformancePackName")]
    #[serde(default)]
    pub conformance_pack_name: String,
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<ConformancePackComplianceFilters>,
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
pub struct ConformancePackComplianceFilters {
    #[serde(rename = "ComplianceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_type: Option<String>,
    #[serde(rename = "ConfigRuleNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rule_names: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeConformancePackComplianceResponse {
    #[serde(rename = "ConformancePackName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conformance_pack_name: Option<String>,
    #[serde(rename = "ConformancePackRuleComplianceList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conformance_pack_rule_compliance_list: Option<Vec<ConformancePackRuleCompliance>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConformancePackRuleCompliance {
    #[serde(rename = "ComplianceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_type: Option<String>,
    #[serde(rename = "ConfigRuleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rule_name: Option<String>,
    #[serde(rename = "Controls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controls: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeConformancePackStatusRequest {
    #[serde(rename = "ConformancePackNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conformance_pack_names: Option<Vec<String>>,
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
pub struct DescribeConformancePackStatusResponse {
    #[serde(rename = "ConformancePackStatusDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conformance_pack_status_details: Option<Vec<ConformancePackStatusDetail>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConformancePackStatusDetail {
    #[serde(rename = "ConformancePackArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conformance_pack_arn: Option<String>,
    #[serde(rename = "ConformancePackId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conformance_pack_id: Option<String>,
    #[serde(rename = "ConformancePackName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conformance_pack_name: Option<String>,
    #[serde(rename = "ConformancePackState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conformance_pack_state: Option<String>,
    #[serde(rename = "ConformancePackStatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conformance_pack_status_reason: Option<String>,
    #[serde(rename = "LastUpdateCompletedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_completed_time: Option<f64>,
    #[serde(rename = "LastUpdateRequestedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_requested_time: Option<f64>,
    #[serde(rename = "StackArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeConformancePacksRequest {
    #[serde(rename = "ConformancePackNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conformance_pack_names: Option<Vec<String>>,
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
pub struct DescribeConformancePacksResponse {
    #[serde(rename = "ConformancePackDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conformance_pack_details: Option<Vec<ConformancePackDetail>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConformancePackDetail {
    #[serde(rename = "ConformancePackArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conformance_pack_arn: Option<String>,
    #[serde(rename = "ConformancePackId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conformance_pack_id: Option<String>,
    #[serde(rename = "ConformancePackInputParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conformance_pack_input_parameters: Option<Vec<ConformancePackInputParameter>>,
    #[serde(rename = "ConformancePackName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conformance_pack_name: Option<String>,
    #[serde(rename = "CreatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "DeliveryS3Bucket")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_s3_bucket: Option<String>,
    #[serde(rename = "DeliveryS3KeyPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_s3_key_prefix: Option<String>,
    #[serde(rename = "LastUpdateRequestedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_requested_time: Option<f64>,
    #[serde(rename = "TemplateSSMDocumentDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_s_s_m_document_details: Option<TemplateSSMDocumentDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConformancePackInputParameter {
    #[serde(rename = "ParameterName")]
    #[serde(default)]
    pub parameter_name: String,
    #[serde(rename = "ParameterValue")]
    #[serde(default)]
    pub parameter_value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TemplateSSMDocumentDetails {
    #[serde(rename = "DocumentName")]
    #[serde(default)]
    pub document_name: String,
    #[serde(rename = "DocumentVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDeliveryChannelStatusRequest {
    #[serde(rename = "DeliveryChannelNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_channel_names: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDeliveryChannelStatusResponse {
    #[serde(rename = "DeliveryChannelsStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_channels_status: Option<Vec<DeliveryChannelStatus>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeliveryChannelStatus {
    #[serde(rename = "configHistoryDeliveryInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_history_delivery_info: Option<ConfigExportDeliveryInfo>,
    #[serde(rename = "configSnapshotDeliveryInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_snapshot_delivery_info: Option<ConfigExportDeliveryInfo>,
    #[serde(rename = "configStreamDeliveryInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_stream_delivery_info: Option<ConfigStreamDeliveryInfo>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConfigExportDeliveryInfo {
    #[serde(rename = "lastAttemptTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_attempt_time: Option<f64>,
    #[serde(rename = "lastErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_code: Option<String>,
    #[serde(rename = "lastErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_message: Option<String>,
    #[serde(rename = "lastStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_status: Option<String>,
    #[serde(rename = "lastSuccessfulTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_successful_time: Option<f64>,
    #[serde(rename = "nextDeliveryTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_delivery_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConfigStreamDeliveryInfo {
    #[serde(rename = "lastErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_code: Option<String>,
    #[serde(rename = "lastErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_message: Option<String>,
    #[serde(rename = "lastStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_status: Option<String>,
    #[serde(rename = "lastStatusChangeTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_status_change_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDeliveryChannelsRequest {
    #[serde(rename = "DeliveryChannelNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_channel_names: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDeliveryChannelsResponse {
    #[serde(rename = "DeliveryChannels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_channels: Option<Vec<DeliveryChannel>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeliveryChannel {
    #[serde(rename = "configSnapshotDeliveryProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_snapshot_delivery_properties: Option<ConfigSnapshotDeliveryProperties>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "s3BucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_name: Option<String>,
    #[serde(rename = "s3KeyPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key_prefix: Option<String>,
    #[serde(rename = "s3KmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_kms_key_arn: Option<String>,
    #[serde(rename = "snsTopicARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_a_r_n: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConfigSnapshotDeliveryProperties {
    #[serde(rename = "deliveryFrequency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_frequency: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeOrganizationConfigRuleStatusesRequest {
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "OrganizationConfigRuleNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_config_rule_names: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeOrganizationConfigRuleStatusesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "OrganizationConfigRuleStatuses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_config_rule_statuses: Option<Vec<OrganizationConfigRuleStatus>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OrganizationConfigRuleStatus {
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "LastUpdateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<f64>,
    #[serde(rename = "OrganizationConfigRuleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_config_rule_name: Option<String>,
    #[serde(rename = "OrganizationRuleStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_rule_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeOrganizationConfigRulesRequest {
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "OrganizationConfigRuleNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_config_rule_names: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeOrganizationConfigRulesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "OrganizationConfigRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_config_rules: Option<Vec<OrganizationConfigRule>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OrganizationConfigRule {
    #[serde(rename = "ExcludedAccounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_accounts: Option<Vec<String>>,
    #[serde(rename = "LastUpdateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<f64>,
    #[serde(rename = "OrganizationConfigRuleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_config_rule_arn: Option<String>,
    #[serde(rename = "OrganizationConfigRuleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_config_rule_name: Option<String>,
    #[serde(rename = "OrganizationCustomPolicyRuleMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_custom_policy_rule_metadata:
        Option<OrganizationCustomPolicyRuleMetadataNoPolicy>,
    #[serde(rename = "OrganizationCustomRuleMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_custom_rule_metadata: Option<OrganizationCustomRuleMetadata>,
    #[serde(rename = "OrganizationManagedRuleMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_managed_rule_metadata: Option<OrganizationManagedRuleMetadata>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OrganizationCustomPolicyRuleMetadataNoPolicy {
    #[serde(rename = "DebugLogDeliveryAccounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug_log_delivery_accounts: Option<Vec<String>>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "InputParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_parameters: Option<String>,
    #[serde(rename = "MaximumExecutionFrequency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_execution_frequency: Option<String>,
    #[serde(rename = "OrganizationConfigRuleTriggerTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_config_rule_trigger_types: Option<Vec<String>>,
    #[serde(rename = "PolicyRuntime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_runtime: Option<String>,
    #[serde(rename = "ResourceIdScope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id_scope: Option<String>,
    #[serde(rename = "ResourceTypesScope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_types_scope: Option<Vec<String>>,
    #[serde(rename = "TagKeyScope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key_scope: Option<String>,
    #[serde(rename = "TagValueScope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_value_scope: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OrganizationCustomRuleMetadata {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "InputParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_parameters: Option<String>,
    #[serde(rename = "LambdaFunctionArn")]
    #[serde(default)]
    pub lambda_function_arn: String,
    #[serde(rename = "MaximumExecutionFrequency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_execution_frequency: Option<String>,
    #[serde(rename = "OrganizationConfigRuleTriggerTypes")]
    #[serde(default)]
    pub organization_config_rule_trigger_types: Vec<String>,
    #[serde(rename = "ResourceIdScope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id_scope: Option<String>,
    #[serde(rename = "ResourceTypesScope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_types_scope: Option<Vec<String>>,
    #[serde(rename = "TagKeyScope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key_scope: Option<String>,
    #[serde(rename = "TagValueScope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_value_scope: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OrganizationManagedRuleMetadata {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "InputParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_parameters: Option<String>,
    #[serde(rename = "MaximumExecutionFrequency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_execution_frequency: Option<String>,
    #[serde(rename = "ResourceIdScope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id_scope: Option<String>,
    #[serde(rename = "ResourceTypesScope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_types_scope: Option<Vec<String>>,
    #[serde(rename = "RuleIdentifier")]
    #[serde(default)]
    pub rule_identifier: String,
    #[serde(rename = "TagKeyScope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key_scope: Option<String>,
    #[serde(rename = "TagValueScope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_value_scope: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeOrganizationConformancePackStatusesRequest {
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "OrganizationConformancePackNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_conformance_pack_names: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeOrganizationConformancePackStatusesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "OrganizationConformancePackStatuses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_conformance_pack_statuses: Option<Vec<OrganizationConformancePackStatus>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OrganizationConformancePackStatus {
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "LastUpdateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<f64>,
    #[serde(rename = "OrganizationConformancePackName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_conformance_pack_name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeOrganizationConformancePacksRequest {
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "OrganizationConformancePackNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_conformance_pack_names: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeOrganizationConformancePacksResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "OrganizationConformancePacks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_conformance_packs: Option<Vec<OrganizationConformancePack>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OrganizationConformancePack {
    #[serde(rename = "ConformancePackInputParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conformance_pack_input_parameters: Option<Vec<ConformancePackInputParameter>>,
    #[serde(rename = "DeliveryS3Bucket")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_s3_bucket: Option<String>,
    #[serde(rename = "DeliveryS3KeyPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_s3_key_prefix: Option<String>,
    #[serde(rename = "ExcludedAccounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_accounts: Option<Vec<String>>,
    #[serde(rename = "LastUpdateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<f64>,
    #[serde(rename = "OrganizationConformancePackArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_conformance_pack_arn: Option<String>,
    #[serde(rename = "OrganizationConformancePackName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_conformance_pack_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribePendingAggregationRequestsRequest {
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
pub struct DescribePendingAggregationRequestsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PendingAggregationRequests")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_aggregation_requests: Option<Vec<PendingAggregationRequest>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PendingAggregationRequest {
    #[serde(rename = "RequesterAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requester_account_id: Option<String>,
    #[serde(rename = "RequesterAwsRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requester_aws_region: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRemediationConfigurationsRequest {
    #[serde(rename = "ConfigRuleNames")]
    #[serde(default)]
    pub config_rule_names: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRemediationConfigurationsResponse {
    #[serde(rename = "RemediationConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remediation_configurations: Option<Vec<RemediationConfiguration>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemediationConfiguration {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Automatic")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic: Option<bool>,
    #[serde(rename = "ConfigRuleName")]
    #[serde(default)]
    pub config_rule_name: String,
    #[serde(rename = "CreatedByService")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_service: Option<String>,
    #[serde(rename = "ExecutionControls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_controls: Option<ExecutionControls>,
    #[serde(rename = "MaximumAutomaticAttempts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_automatic_attempts: Option<i32>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, RemediationParameterValue>>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "RetryAttemptSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_attempt_seconds: Option<i64>,
    #[serde(rename = "TargetId")]
    #[serde(default)]
    pub target_id: String,
    #[serde(rename = "TargetType")]
    #[serde(default)]
    pub target_type: String,
    #[serde(rename = "TargetVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExecutionControls {
    #[serde(rename = "SsmControls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssm_controls: Option<SsmControls>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SsmControls {
    #[serde(rename = "ConcurrentExecutionRatePercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrent_execution_rate_percentage: Option<i32>,
    #[serde(rename = "ErrorPercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_percentage: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemediationParameterValue {
    #[serde(rename = "ResourceValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_value: Option<ResourceValue>,
    #[serde(rename = "StaticValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_value: Option<StaticValue>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceValue {
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StaticValue {
    #[serde(rename = "Values")]
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRemediationExceptionsRequest {
    #[serde(rename = "ConfigRuleName")]
    #[serde(default)]
    pub config_rule_name: String,
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_keys: Option<Vec<RemediationExceptionResourceKey>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRemediationExceptionsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RemediationExceptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remediation_exceptions: Option<Vec<RemediationException>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemediationException {
    #[serde(rename = "ConfigRuleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rule_name: Option<String>,
    #[serde(rename = "ExpirationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<f64>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
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
pub struct DescribeRemediationExecutionStatusRequest {
    #[serde(rename = "ConfigRuleName")]
    #[serde(default)]
    pub config_rule_name: String,
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_keys: Option<Vec<ResourceKey>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRemediationExecutionStatusResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RemediationExecutionStatuses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remediation_execution_statuses: Option<Vec<RemediationExecutionStatus>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemediationExecutionStatus {
    #[serde(rename = "InvocationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invocation_time: Option<f64>,
    #[serde(rename = "LastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(rename = "ResourceKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_key: Option<ResourceKey>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "StepDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_details: Option<Vec<RemediationExecutionStep>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemediationExecutionStep {
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "StopTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRetentionConfigurationsRequest {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RetentionConfigurationNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_configuration_names: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRetentionConfigurationsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RetentionConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_configurations: Option<Vec<RetentionConfiguration>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RetentionConfiguration {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "RetentionPeriodInDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_period_in_days: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateResourceTypesRequest {
    #[serde(rename = "ConfigurationRecorderArn")]
    #[serde(default)]
    pub configuration_recorder_arn: String,
    #[serde(rename = "ResourceTypes")]
    #[serde(default)]
    pub resource_types: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateResourceTypesResponse {
    #[serde(rename = "ConfigurationRecorder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_recorder: Option<ConfigurationRecorder>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAggregateComplianceDetailsByConfigRuleRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "AwsRegion")]
    #[serde(default)]
    pub aws_region: String,
    #[serde(rename = "ComplianceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_type: Option<String>,
    #[serde(rename = "ConfigRuleName")]
    #[serde(default)]
    pub config_rule_name: String,
    #[serde(rename = "ConfigurationAggregatorName")]
    #[serde(default)]
    pub configuration_aggregator_name: String,
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
pub struct GetAggregateComplianceDetailsByConfigRuleResponse {
    #[serde(rename = "AggregateEvaluationResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_evaluation_results: Option<Vec<AggregateEvaluationResult>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AggregateEvaluationResult {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "Annotation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation: Option<String>,
    #[serde(rename = "AwsRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_region: Option<String>,
    #[serde(rename = "ComplianceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_type: Option<String>,
    #[serde(rename = "ConfigRuleInvokedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rule_invoked_time: Option<f64>,
    #[serde(rename = "EvaluationResultIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_result_identifier: Option<EvaluationResultIdentifier>,
    #[serde(rename = "ResultRecordedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_recorded_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationResultIdentifier {
    #[serde(rename = "EvaluationResultQualifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_result_qualifier: Option<EvaluationResultQualifier>,
    #[serde(rename = "OrderingTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordering_timestamp: Option<f64>,
    #[serde(rename = "ResourceEvaluationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_evaluation_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationResultQualifier {
    #[serde(rename = "ConfigRuleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rule_name: Option<String>,
    #[serde(rename = "EvaluationMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_mode: Option<String>,
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
pub struct GetAggregateConfigRuleComplianceSummaryRequest {
    #[serde(rename = "ConfigurationAggregatorName")]
    #[serde(default)]
    pub configuration_aggregator_name: String,
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<ConfigRuleComplianceSummaryFilters>,
    #[serde(rename = "GroupByKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by_key: Option<String>,
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
pub struct ConfigRuleComplianceSummaryFilters {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "AwsRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_region: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAggregateConfigRuleComplianceSummaryResponse {
    #[serde(rename = "AggregateComplianceCounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_compliance_counts: Option<Vec<AggregateComplianceCount>>,
    #[serde(rename = "GroupByKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by_key: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AggregateComplianceCount {
    #[serde(rename = "ComplianceSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_summary: Option<ComplianceSummary>,
    #[serde(rename = "GroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ComplianceSummary {
    #[serde(rename = "ComplianceSummaryTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_summary_timestamp: Option<f64>,
    #[serde(rename = "CompliantResourceCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliant_resource_count: Option<ComplianceContributorCount>,
    #[serde(rename = "NonCompliantResourceCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_compliant_resource_count: Option<ComplianceContributorCount>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAggregateConformancePackComplianceSummaryRequest {
    #[serde(rename = "ConfigurationAggregatorName")]
    #[serde(default)]
    pub configuration_aggregator_name: String,
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<AggregateConformancePackComplianceSummaryFilters>,
    #[serde(rename = "GroupByKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by_key: Option<String>,
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
pub struct AggregateConformancePackComplianceSummaryFilters {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "AwsRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_region: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAggregateConformancePackComplianceSummaryResponse {
    #[serde(rename = "AggregateConformancePackComplianceSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_conformance_pack_compliance_summaries:
        Option<Vec<AggregateConformancePackComplianceSummary>>,
    #[serde(rename = "GroupByKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by_key: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AggregateConformancePackComplianceSummary {
    #[serde(rename = "ComplianceSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_summary: Option<AggregateConformancePackComplianceCount>,
    #[serde(rename = "GroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AggregateConformancePackComplianceCount {
    #[serde(rename = "CompliantConformancePackCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliant_conformance_pack_count: Option<i32>,
    #[serde(rename = "NonCompliantConformancePackCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_compliant_conformance_pack_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAggregateDiscoveredResourceCountsRequest {
    #[serde(rename = "ConfigurationAggregatorName")]
    #[serde(default)]
    pub configuration_aggregator_name: String,
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<ResourceCountFilters>,
    #[serde(rename = "GroupByKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by_key: Option<String>,
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
pub struct ResourceCountFilters {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "Region")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAggregateDiscoveredResourceCountsResponse {
    #[serde(rename = "GroupByKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by_key: Option<String>,
    #[serde(rename = "GroupedResourceCounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grouped_resource_counts: Option<Vec<GroupedResourceCount>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TotalDiscoveredResources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_discovered_resources: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GroupedResourceCount {
    #[serde(rename = "GroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(rename = "ResourceCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_count: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAggregateResourceConfigRequest {
    #[serde(rename = "ConfigurationAggregatorName")]
    #[serde(default)]
    pub configuration_aggregator_name: String,
    #[serde(rename = "ResourceIdentifier")]
    #[serde(default)]
    pub resource_identifier: AggregateResourceIdentifier,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAggregateResourceConfigResponse {
    #[serde(rename = "ConfigurationItem")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_item: Option<ConfigurationItem>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConfigurationItem {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "availabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "awsRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_region: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<String>,
    #[serde(rename = "configurationItemCaptureTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_item_capture_time: Option<f64>,
    #[serde(rename = "configurationItemDeliveryTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_item_delivery_time: Option<f64>,
    #[serde(rename = "configurationItemMD5Hash")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_item_m_d5_hash: Option<String>,
    #[serde(rename = "configurationItemStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_item_status: Option<String>,
    #[serde(rename = "configurationStateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_state_id: Option<String>,
    #[serde(rename = "recordingFrequency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording_frequency: Option<String>,
    #[serde(rename = "relatedEvents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_events: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationships: Option<Vec<Relationship>>,
    #[serde(rename = "resourceCreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_creation_time: Option<f64>,
    #[serde(rename = "resourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "resourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "supplementaryConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplementary_configuration: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Relationship {
    #[serde(rename = "relationshipName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship_name: Option<String>,
    #[serde(rename = "resourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
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
pub struct GetComplianceDetailsByConfigRuleRequest {
    #[serde(rename = "ComplianceTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_types: Option<Vec<String>>,
    #[serde(rename = "ConfigRuleName")]
    #[serde(default)]
    pub config_rule_name: String,
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
pub struct GetComplianceDetailsByConfigRuleResponse {
    #[serde(rename = "EvaluationResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_results: Option<Vec<EvaluationResult>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationResult {
    #[serde(rename = "Annotation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation: Option<String>,
    #[serde(rename = "ComplianceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_type: Option<String>,
    #[serde(rename = "ConfigRuleInvokedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rule_invoked_time: Option<f64>,
    #[serde(rename = "EvaluationResultIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_result_identifier: Option<EvaluationResultIdentifier>,
    #[serde(rename = "ResultRecordedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_recorded_time: Option<f64>,
    #[serde(rename = "ResultToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetComplianceDetailsByResourceRequest {
    #[serde(rename = "ComplianceTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_types: Option<Vec<String>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceEvaluationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_evaluation_id: Option<String>,
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
pub struct GetComplianceDetailsByResourceResponse {
    #[serde(rename = "EvaluationResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_results: Option<Vec<EvaluationResult>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetComplianceSummaryByConfigRuleResponse {
    #[serde(rename = "ComplianceSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_summary: Option<ComplianceSummary>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetComplianceSummaryByResourceTypeRequest {
    #[serde(rename = "ResourceTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_types: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetComplianceSummaryByResourceTypeResponse {
    #[serde(rename = "ComplianceSummariesByResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_summaries_by_resource_type: Option<Vec<ComplianceSummaryByResourceType>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ComplianceSummaryByResourceType {
    #[serde(rename = "ComplianceSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_summary: Option<ComplianceSummary>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetConformancePackComplianceDetailsRequest {
    #[serde(rename = "ConformancePackName")]
    #[serde(default)]
    pub conformance_pack_name: String,
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<ConformancePackEvaluationFilters>,
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
pub struct ConformancePackEvaluationFilters {
    #[serde(rename = "ComplianceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_type: Option<String>,
    #[serde(rename = "ConfigRuleNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rule_names: Option<Vec<String>>,
    #[serde(rename = "ResourceIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_ids: Option<Vec<String>>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetConformancePackComplianceDetailsResponse {
    #[serde(rename = "ConformancePackName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conformance_pack_name: Option<String>,
    #[serde(rename = "ConformancePackRuleEvaluationResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conformance_pack_rule_evaluation_results: Option<Vec<ConformancePackEvaluationResult>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConformancePackEvaluationResult {
    #[serde(rename = "Annotation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation: Option<String>,
    #[serde(rename = "ComplianceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_type: Option<String>,
    #[serde(rename = "ConfigRuleInvokedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rule_invoked_time: Option<f64>,
    #[serde(rename = "EvaluationResultIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_result_identifier: Option<EvaluationResultIdentifier>,
    #[serde(rename = "ResultRecordedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_recorded_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetConformancePackComplianceSummaryRequest {
    #[serde(rename = "ConformancePackNames")]
    #[serde(default)]
    pub conformance_pack_names: Vec<String>,
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
pub struct GetConformancePackComplianceSummaryResponse {
    #[serde(rename = "ConformancePackComplianceSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conformance_pack_compliance_summary_list: Option<Vec<ConformancePackComplianceSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConformancePackComplianceSummary {
    #[serde(rename = "ConformancePackComplianceStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conformance_pack_compliance_status: Option<String>,
    #[serde(rename = "ConformancePackName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conformance_pack_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCustomRulePolicyRequest {
    #[serde(rename = "ConfigRuleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rule_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCustomRulePolicyResponse {
    #[serde(rename = "PolicyText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_text: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDiscoveredResourceCountsRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "resourceTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_types: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDiscoveredResourceCountsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "resourceCounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_counts: Option<Vec<ResourceCount>>,
    #[serde(rename = "totalDiscoveredResources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_discovered_resources: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceCount {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetOrganizationConfigRuleDetailedStatusRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<StatusDetailFilters>,
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "OrganizationConfigRuleName")]
    #[serde(default)]
    pub organization_config_rule_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StatusDetailFilters {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "MemberAccountRuleStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_account_rule_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetOrganizationConfigRuleDetailedStatusResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "OrganizationConfigRuleDetailedStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_config_rule_detailed_status: Option<Vec<MemberAccountStatus>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MemberAccountStatus {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "ConfigRuleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rule_name: Option<String>,
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "LastUpdateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<f64>,
    #[serde(rename = "MemberAccountRuleStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_account_rule_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetOrganizationConformancePackDetailedStatusRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<OrganizationResourceDetailedStatusFilters>,
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "OrganizationConformancePackName")]
    #[serde(default)]
    pub organization_conformance_pack_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OrganizationResourceDetailedStatusFilters {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetOrganizationConformancePackDetailedStatusResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "OrganizationConformancePackDetailedStatuses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_conformance_pack_detailed_statuses:
        Option<Vec<OrganizationConformancePackDetailedStatus>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OrganizationConformancePackDetailedStatus {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "ConformancePackName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conformance_pack_name: Option<String>,
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "LastUpdateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<f64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetOrganizationCustomRulePolicyRequest {
    #[serde(rename = "OrganizationConfigRuleName")]
    #[serde(default)]
    pub organization_config_rule_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetOrganizationCustomRulePolicyResponse {
    #[serde(rename = "PolicyText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_text: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourceConfigHistoryRequest {
    #[serde(rename = "chronologicalOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chronological_order: Option<String>,
    #[serde(rename = "earlierTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub earlier_time: Option<f64>,
    #[serde(rename = "laterTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub later_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "resourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    pub resource_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourceConfigHistoryResponse {
    #[serde(rename = "configurationItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_items: Option<Vec<ConfigurationItem>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourceEvaluationSummaryRequest {
    #[serde(rename = "ResourceEvaluationId")]
    #[serde(default)]
    pub resource_evaluation_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourceEvaluationSummaryResponse {
    #[serde(rename = "Compliance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance: Option<String>,
    #[serde(rename = "EvaluationContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_context: Option<EvaluationContext>,
    #[serde(rename = "EvaluationMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_mode: Option<String>,
    #[serde(rename = "EvaluationStartTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_start_timestamp: Option<f64>,
    #[serde(rename = "EvaluationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_status: Option<EvaluationStatus>,
    #[serde(rename = "ResourceDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_details: Option<ResourceDetails>,
    #[serde(rename = "ResourceEvaluationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_evaluation_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationContext {
    #[serde(rename = "EvaluationContextIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_context_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationStatus {
    #[serde(rename = "FailureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceDetails {
    #[serde(rename = "ResourceConfiguration")]
    #[serde(default)]
    pub resource_configuration: String,
    #[serde(rename = "ResourceConfigurationSchemaType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_configuration_schema_type: Option<String>,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    pub resource_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetStoredQueryRequest {
    #[serde(rename = "QueryName")]
    #[serde(default)]
    pub query_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetStoredQueryResponse {
    #[serde(rename = "StoredQuery")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stored_query: Option<StoredQuery>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StoredQuery {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Expression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(rename = "QueryArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_arn: Option<String>,
    #[serde(rename = "QueryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_id: Option<String>,
    #[serde(rename = "QueryName")]
    #[serde(default)]
    pub query_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAggregateDiscoveredResourcesRequest {
    #[serde(rename = "ConfigurationAggregatorName")]
    #[serde(default)]
    pub configuration_aggregator_name: String,
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<ResourceFilters>,
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    pub resource_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceFilters {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "Region")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "ResourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAggregateDiscoveredResourcesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceIdentifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_identifiers: Option<Vec<AggregateResourceIdentifier>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListConfigurationRecordersRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<ConfigurationRecorderFilter>>,
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
pub struct ConfigurationRecorderFilter {
    #[serde(rename = "filterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_name: Option<String>,
    #[serde(rename = "filterValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_value: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListConfigurationRecordersResponse {
    #[serde(rename = "ConfigurationRecorderSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_recorder_summaries: Option<Vec<ConfigurationRecorderSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConfigurationRecorderSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "recordingScope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording_scope: Option<String>,
    #[serde(rename = "servicePrincipal")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_principal: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListConformancePackComplianceScoresRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<ConformancePackComplianceScoresFilters>,
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(rename = "SortOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConformancePackComplianceScoresFilters {
    #[serde(rename = "ConformancePackNames")]
    #[serde(default)]
    pub conformance_pack_names: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListConformancePackComplianceScoresResponse {
    #[serde(rename = "ConformancePackComplianceScores")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conformance_pack_compliance_scores: Option<Vec<ConformancePackComplianceScore>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConformancePackComplianceScore {
    #[serde(rename = "ConformancePackName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conformance_pack_name: Option<String>,
    #[serde(rename = "LastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(rename = "Score")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDiscoveredResourcesRequest {
    #[serde(rename = "includeDeletedResources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_deleted_resources: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "resourceIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_ids: Option<Vec<String>>,
    #[serde(rename = "resourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    pub resource_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDiscoveredResourcesResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "resourceIdentifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_identifiers: Option<Vec<ResourceIdentifier>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceIdentifier {
    #[serde(rename = "resourceDeletionTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_deletion_time: Option<f64>,
    #[serde(rename = "resourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
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
pub struct ListResourceEvaluationsRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<ResourceEvaluationFilters>,
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
pub struct ResourceEvaluationFilters {
    #[serde(rename = "EvaluationContextIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_context_identifier: Option<String>,
    #[serde(rename = "EvaluationMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_mode: Option<String>,
    #[serde(rename = "TimeWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_window: Option<TimeWindow>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TimeWindow {
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListResourceEvaluationsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceEvaluations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_evaluations: Option<Vec<ResourceEvaluation>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceEvaluation {
    #[serde(rename = "EvaluationMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_mode: Option<String>,
    #[serde(rename = "EvaluationStartTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_start_timestamp: Option<f64>,
    #[serde(rename = "ResourceEvaluationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_evaluation_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListStoredQueriesRequest {
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
pub struct ListStoredQueriesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StoredQueryMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stored_query_metadata: Option<Vec<StoredQueryMetadata>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StoredQueryMetadata {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "QueryArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_arn: Option<String>,
    #[serde(rename = "QueryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_id: Option<String>,
    #[serde(rename = "QueryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Tag {
    #[serde(rename = "Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutAggregationAuthorizationRequest {
    #[serde(rename = "AuthorizedAccountId")]
    #[serde(default)]
    pub authorized_account_id: String,
    #[serde(rename = "AuthorizedAwsRegion")]
    #[serde(default)]
    pub authorized_aws_region: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutAggregationAuthorizationResponse {
    #[serde(rename = "AggregationAuthorization")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_authorization: Option<AggregationAuthorization>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutConfigRuleRequest {
    #[serde(rename = "ConfigRule")]
    #[serde(default)]
    pub config_rule: ConfigRule,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutConfigurationAggregatorRequest {
    #[serde(rename = "AccountAggregationSources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_aggregation_sources: Option<Vec<AccountAggregationSource>>,
    #[serde(rename = "AggregatorFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregator_filters: Option<AggregatorFilters>,
    #[serde(rename = "ConfigurationAggregatorName")]
    #[serde(default)]
    pub configuration_aggregator_name: String,
    #[serde(rename = "OrganizationAggregationSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_aggregation_source: Option<OrganizationAggregationSource>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutConfigurationAggregatorResponse {
    #[serde(rename = "ConfigurationAggregator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_aggregator: Option<ConfigurationAggregator>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutConfigurationRecorderRequest {
    #[serde(rename = "ConfigurationRecorder")]
    #[serde(default)]
    pub configuration_recorder: ConfigurationRecorder,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutConformancePackRequest {
    #[serde(rename = "ConformancePackInputParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conformance_pack_input_parameters: Option<Vec<ConformancePackInputParameter>>,
    #[serde(rename = "ConformancePackName")]
    #[serde(default)]
    pub conformance_pack_name: String,
    #[serde(rename = "DeliveryS3Bucket")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_s3_bucket: Option<String>,
    #[serde(rename = "DeliveryS3KeyPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_s3_key_prefix: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "TemplateBody")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_body: Option<String>,
    #[serde(rename = "TemplateS3Uri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_s3_uri: Option<String>,
    #[serde(rename = "TemplateSSMDocumentDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_s_s_m_document_details: Option<TemplateSSMDocumentDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutConformancePackResponse {
    #[serde(rename = "ConformancePackArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conformance_pack_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutDeliveryChannelRequest {
    #[serde(rename = "DeliveryChannel")]
    #[serde(default)]
    pub delivery_channel: DeliveryChannel,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutEvaluationsRequest {
    #[serde(rename = "Evaluations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluations: Option<Vec<Evaluation>>,
    #[serde(rename = "ResultToken")]
    #[serde(default)]
    pub result_token: String,
    #[serde(rename = "TestMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_mode: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Evaluation {
    #[serde(rename = "Annotation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation: Option<String>,
    #[serde(rename = "ComplianceResourceId")]
    #[serde(default)]
    pub compliance_resource_id: String,
    #[serde(rename = "ComplianceResourceType")]
    #[serde(default)]
    pub compliance_resource_type: String,
    #[serde(rename = "ComplianceType")]
    #[serde(default)]
    pub compliance_type: String,
    #[serde(rename = "OrderingTimestamp")]
    #[serde(default)]
    pub ordering_timestamp: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutEvaluationsResponse {
    #[serde(rename = "FailedEvaluations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_evaluations: Option<Vec<Evaluation>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutExternalEvaluationRequest {
    #[serde(rename = "ConfigRuleName")]
    #[serde(default)]
    pub config_rule_name: String,
    #[serde(rename = "ExternalEvaluation")]
    #[serde(default)]
    pub external_evaluation: ExternalEvaluation,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExternalEvaluation {
    #[serde(rename = "Annotation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation: Option<String>,
    #[serde(rename = "ComplianceResourceId")]
    #[serde(default)]
    pub compliance_resource_id: String,
    #[serde(rename = "ComplianceResourceType")]
    #[serde(default)]
    pub compliance_resource_type: String,
    #[serde(rename = "ComplianceType")]
    #[serde(default)]
    pub compliance_type: String,
    #[serde(rename = "OrderingTimestamp")]
    #[serde(default)]
    pub ordering_timestamp: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutExternalEvaluationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutOrganizationConfigRuleRequest {
    #[serde(rename = "ExcludedAccounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_accounts: Option<Vec<String>>,
    #[serde(rename = "OrganizationConfigRuleName")]
    #[serde(default)]
    pub organization_config_rule_name: String,
    #[serde(rename = "OrganizationCustomPolicyRuleMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_custom_policy_rule_metadata: Option<OrganizationCustomPolicyRuleMetadata>,
    #[serde(rename = "OrganizationCustomRuleMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_custom_rule_metadata: Option<OrganizationCustomRuleMetadata>,
    #[serde(rename = "OrganizationManagedRuleMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_managed_rule_metadata: Option<OrganizationManagedRuleMetadata>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OrganizationCustomPolicyRuleMetadata {
    #[serde(rename = "DebugLogDeliveryAccounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug_log_delivery_accounts: Option<Vec<String>>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "InputParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_parameters: Option<String>,
    #[serde(rename = "MaximumExecutionFrequency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_execution_frequency: Option<String>,
    #[serde(rename = "OrganizationConfigRuleTriggerTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_config_rule_trigger_types: Option<Vec<String>>,
    #[serde(rename = "PolicyRuntime")]
    #[serde(default)]
    pub policy_runtime: String,
    #[serde(rename = "PolicyText")]
    #[serde(default)]
    pub policy_text: String,
    #[serde(rename = "ResourceIdScope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id_scope: Option<String>,
    #[serde(rename = "ResourceTypesScope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_types_scope: Option<Vec<String>>,
    #[serde(rename = "TagKeyScope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key_scope: Option<String>,
    #[serde(rename = "TagValueScope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_value_scope: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutOrganizationConfigRuleResponse {
    #[serde(rename = "OrganizationConfigRuleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_config_rule_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutOrganizationConformancePackRequest {
    #[serde(rename = "ConformancePackInputParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conformance_pack_input_parameters: Option<Vec<ConformancePackInputParameter>>,
    #[serde(rename = "DeliveryS3Bucket")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_s3_bucket: Option<String>,
    #[serde(rename = "DeliveryS3KeyPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_s3_key_prefix: Option<String>,
    #[serde(rename = "ExcludedAccounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_accounts: Option<Vec<String>>,
    #[serde(rename = "OrganizationConformancePackName")]
    #[serde(default)]
    pub organization_conformance_pack_name: String,
    #[serde(rename = "TemplateBody")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_body: Option<String>,
    #[serde(rename = "TemplateS3Uri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_s3_uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutOrganizationConformancePackResponse {
    #[serde(rename = "OrganizationConformancePackArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_conformance_pack_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutRemediationConfigurationsRequest {
    #[serde(rename = "RemediationConfigurations")]
    #[serde(default)]
    pub remediation_configurations: Vec<RemediationConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutRemediationConfigurationsResponse {
    #[serde(rename = "FailedBatches")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_batches: Option<Vec<FailedRemediationBatch>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FailedRemediationBatch {
    #[serde(rename = "FailedItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_items: Option<Vec<RemediationConfiguration>>,
    #[serde(rename = "FailureMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutRemediationExceptionsRequest {
    #[serde(rename = "ConfigRuleName")]
    #[serde(default)]
    pub config_rule_name: String,
    #[serde(rename = "ExpirationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<f64>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "ResourceKeys")]
    #[serde(default)]
    pub resource_keys: Vec<RemediationExceptionResourceKey>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutRemediationExceptionsResponse {
    #[serde(rename = "FailedBatches")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_batches: Option<Vec<FailedRemediationExceptionBatch>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FailedRemediationExceptionBatch {
    #[serde(rename = "FailedItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_items: Option<Vec<RemediationException>>,
    #[serde(rename = "FailureMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutResourceConfigRequest {
    #[serde(rename = "Configuration")]
    #[serde(default)]
    pub configuration: String,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "ResourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    pub resource_type: String,
    #[serde(rename = "SchemaVersionId")]
    #[serde(default)]
    pub schema_version_id: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutRetentionConfigurationRequest {
    #[serde(rename = "RetentionPeriodInDays")]
    #[serde(default)]
    pub retention_period_in_days: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutRetentionConfigurationResponse {
    #[serde(rename = "RetentionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_configuration: Option<RetentionConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutServiceLinkedConfigurationRecorderRequest {
    #[serde(rename = "ServicePrincipal")]
    #[serde(default)]
    pub service_principal: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutServiceLinkedConfigurationRecorderResponse {
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
pub struct PutStoredQueryRequest {
    #[serde(rename = "StoredQuery")]
    #[serde(default)]
    pub stored_query: StoredQuery,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutStoredQueryResponse {
    #[serde(rename = "QueryArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SelectAggregateResourceConfigRequest {
    #[serde(rename = "ConfigurationAggregatorName")]
    #[serde(default)]
    pub configuration_aggregator_name: String,
    #[serde(rename = "Expression")]
    #[serde(default)]
    pub expression: String,
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
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
pub struct SelectAggregateResourceConfigResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "QueryInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_info: Option<QueryInfo>,
    #[serde(rename = "Results")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueryInfo {
    #[serde(rename = "SelectFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub select_fields: Option<Vec<FieldInfo>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FieldInfo {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SelectResourceConfigRequest {
    #[serde(rename = "Expression")]
    #[serde(default)]
    pub expression: String,
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
pub struct SelectResourceConfigResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "QueryInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_info: Option<QueryInfo>,
    #[serde(rename = "Results")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartConfigRulesEvaluationRequest {
    #[serde(rename = "ConfigRuleNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rule_names: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartConfigRulesEvaluationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartConfigurationRecorderRequest {
    #[serde(rename = "ConfigurationRecorderName")]
    #[serde(default)]
    pub configuration_recorder_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartRemediationExecutionRequest {
    #[serde(rename = "ConfigRuleName")]
    #[serde(default)]
    pub config_rule_name: String,
    #[serde(rename = "ResourceKeys")]
    #[serde(default)]
    pub resource_keys: Vec<ResourceKey>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartRemediationExecutionResponse {
    #[serde(rename = "FailedItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_items: Option<Vec<ResourceKey>>,
    #[serde(rename = "FailureMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartResourceEvaluationRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "EvaluationContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_context: Option<EvaluationContext>,
    #[serde(rename = "EvaluationMode")]
    #[serde(default)]
    pub evaluation_mode: String,
    #[serde(rename = "EvaluationTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_timeout: Option<i32>,
    #[serde(rename = "ResourceDetails")]
    #[serde(default)]
    pub resource_details: ResourceDetails,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartResourceEvaluationResponse {
    #[serde(rename = "ResourceEvaluationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_evaluation_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopConfigurationRecorderRequest {
    #[serde(rename = "ConfigurationRecorderName")]
    #[serde(default)]
    pub configuration_recorder_name: String,
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
pub struct UntagResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}
