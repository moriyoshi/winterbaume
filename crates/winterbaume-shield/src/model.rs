//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-shield

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateDRTLogBucketRequest {
    #[serde(rename = "LogBucket")]
    #[serde(default)]
    pub log_bucket: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateDRTLogBucketResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateDRTRoleRequest {
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    pub role_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateDRTRoleResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateHealthCheckRequest {
    #[serde(rename = "HealthCheckArn")]
    #[serde(default)]
    pub health_check_arn: String,
    #[serde(rename = "ProtectionId")]
    #[serde(default)]
    pub protection_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateHealthCheckResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateProactiveEngagementDetailsRequest {
    #[serde(rename = "EmergencyContactList")]
    #[serde(default)]
    pub emergency_contact_list: Vec<EmergencyContact>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EmergencyContact {
    #[serde(rename = "ContactNotes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_notes: Option<String>,
    #[serde(rename = "EmailAddress")]
    #[serde(default)]
    pub email_address: String,
    #[serde(rename = "PhoneNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateProactiveEngagementDetailsResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateProtectionGroupRequest {
    #[serde(rename = "Aggregation")]
    #[serde(default)]
    pub aggregation: String,
    #[serde(rename = "Members")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<String>>,
    #[serde(rename = "Pattern")]
    #[serde(default)]
    pub pattern: String,
    #[serde(rename = "ProtectionGroupId")]
    #[serde(default)]
    pub protection_group_id: String,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
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
pub struct CreateProtectionGroupResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateProtectionRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateProtectionResponse {
    #[serde(rename = "ProtectionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protection_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSubscriptionRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSubscriptionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteProtectionGroupRequest {
    #[serde(rename = "ProtectionGroupId")]
    #[serde(default)]
    pub protection_group_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteProtectionGroupResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteProtectionRequest {
    #[serde(rename = "ProtectionId")]
    #[serde(default)]
    pub protection_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteProtectionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSubscriptionRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSubscriptionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAttackRequest {
    #[serde(rename = "AttackId")]
    #[serde(default)]
    pub attack_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAttackResponse {
    #[serde(rename = "Attack")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack: Option<AttackDetail>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttackDetail {
    #[serde(rename = "AttackCounters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack_counters: Option<Vec<SummarizedCounter>>,
    #[serde(rename = "AttackId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack_id: Option<String>,
    #[serde(rename = "AttackProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack_properties: Option<Vec<AttackProperty>>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "Mitigations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mitigations: Option<Vec<Mitigation>>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(rename = "SubResources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_resources: Option<Vec<SubResourceSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SummarizedCounter {
    #[serde(rename = "Average")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average: Option<f64>,
    #[serde(rename = "Max")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    #[serde(rename = "N")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<i32>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Sum")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sum: Option<f64>,
    #[serde(rename = "Unit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttackProperty {
    #[serde(rename = "AttackLayer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack_layer: Option<String>,
    #[serde(rename = "AttackPropertyIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack_property_identifier: Option<String>,
    #[serde(rename = "TopContributors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_contributors: Option<Vec<Contributor>>,
    #[serde(rename = "Total")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    #[serde(rename = "Unit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Contributor {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Mitigation {
    #[serde(rename = "MitigationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mitigation_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SubResourceSummary {
    #[serde(rename = "AttackVectors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack_vectors: Option<Vec<SummarizedAttackVector>>,
    #[serde(rename = "Counters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counters: Option<Vec<SummarizedCounter>>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SummarizedAttackVector {
    #[serde(rename = "VectorCounters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_counters: Option<Vec<SummarizedCounter>>,
    #[serde(rename = "VectorType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAttackStatisticsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAttackStatisticsResponse {
    #[serde(rename = "DataItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_items: Option<Vec<AttackStatisticsDataItem>>,
    #[serde(rename = "TimeRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_range: Option<TimeRange>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttackStatisticsDataItem {
    #[serde(rename = "AttackCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack_count: Option<i64>,
    #[serde(rename = "AttackVolume")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack_volume: Option<AttackVolume>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttackVolume {
    #[serde(rename = "BitsPerSecond")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bits_per_second: Option<AttackVolumeStatistics>,
    #[serde(rename = "PacketsPerSecond")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packets_per_second: Option<AttackVolumeStatistics>,
    #[serde(rename = "RequestsPerSecond")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests_per_second: Option<AttackVolumeStatistics>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttackVolumeStatistics {
    #[serde(rename = "Max")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TimeRange {
    #[serde(rename = "FromInclusive")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_inclusive: Option<f64>,
    #[serde(rename = "ToExclusive")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_exclusive: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDRTAccessRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDRTAccessResponse {
    #[serde(rename = "LogBucketList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_bucket_list: Option<Vec<String>>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeEmergencyContactSettingsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeEmergencyContactSettingsResponse {
    #[serde(rename = "EmergencyContactList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emergency_contact_list: Option<Vec<EmergencyContact>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeProtectionGroupRequest {
    #[serde(rename = "ProtectionGroupId")]
    #[serde(default)]
    pub protection_group_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeProtectionGroupResponse {
    #[serde(rename = "ProtectionGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protection_group: Option<ProtectionGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProtectionGroup {
    #[serde(rename = "Aggregation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation: Option<String>,
    #[serde(rename = "Members")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<String>>,
    #[serde(rename = "Pattern")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    #[serde(rename = "ProtectionGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protection_group_arn: Option<String>,
    #[serde(rename = "ProtectionGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protection_group_id: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeProtectionRequest {
    #[serde(rename = "ProtectionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protection_id: Option<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeProtectionResponse {
    #[serde(rename = "Protection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protection: Option<Protection>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Protection {
    #[serde(rename = "ApplicationLayerAutomaticResponseConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_layer_automatic_response_configuration:
        Option<ApplicationLayerAutomaticResponseConfiguration>,
    #[serde(rename = "HealthCheckIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_ids: Option<Vec<String>>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ProtectionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protection_arn: Option<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApplicationLayerAutomaticResponseConfiguration {
    #[serde(rename = "Action")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<ResponseAction>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResponseAction {
    #[serde(rename = "Block")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block: Option<BlockAction>,
    #[serde(rename = "Count")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<CountAction>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BlockAction {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CountAction {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSubscriptionRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSubscriptionResponse {
    #[serde(rename = "Subscription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<Subscription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Subscription {
    #[serde(rename = "AutoRenew")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew: Option<String>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "Limits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits: Option<Vec<Limit>>,
    #[serde(rename = "ProactiveEngagementStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proactive_engagement_status: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(rename = "SubscriptionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_arn: Option<String>,
    #[serde(rename = "SubscriptionLimits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_limits: Option<SubscriptionLimits>,
    #[serde(rename = "TimeCommitmentInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_commitment_in_seconds: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Limit {
    #[serde(rename = "Max")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<i64>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SubscriptionLimits {
    #[serde(rename = "ProtectionGroupLimits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protection_group_limits: Option<ProtectionGroupLimits>,
    #[serde(rename = "ProtectionLimits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protection_limits: Option<ProtectionLimits>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProtectionGroupLimits {
    #[serde(rename = "MaxProtectionGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_protection_groups: Option<i64>,
    #[serde(rename = "PatternTypeLimits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern_type_limits: Option<ProtectionGroupPatternTypeLimits>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProtectionGroupPatternTypeLimits {
    #[serde(rename = "ArbitraryPatternLimits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arbitrary_pattern_limits: Option<ProtectionGroupArbitraryPatternLimits>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProtectionGroupArbitraryPatternLimits {
    #[serde(rename = "MaxMembers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_members: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProtectionLimits {
    #[serde(rename = "ProtectedResourceTypeLimits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protected_resource_type_limits: Option<Vec<Limit>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisableApplicationLayerAutomaticResponseRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisableApplicationLayerAutomaticResponseResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisableProactiveEngagementRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisableProactiveEngagementResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateDRTLogBucketRequest {
    #[serde(rename = "LogBucket")]
    #[serde(default)]
    pub log_bucket: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateDRTLogBucketResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateDRTRoleRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateDRTRoleResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateHealthCheckRequest {
    #[serde(rename = "HealthCheckArn")]
    #[serde(default)]
    pub health_check_arn: String,
    #[serde(rename = "ProtectionId")]
    #[serde(default)]
    pub protection_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateHealthCheckResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnableApplicationLayerAutomaticResponseRequest {
    #[serde(rename = "Action")]
    #[serde(default)]
    pub action: ResponseAction,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnableApplicationLayerAutomaticResponseResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnableProactiveEngagementRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnableProactiveEngagementResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSubscriptionStateRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSubscriptionStateResponse {
    #[serde(rename = "SubscriptionState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAttacksRequest {
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<TimeRange>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arns: Option<Vec<String>>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<TimeRange>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAttacksResponse {
    #[serde(rename = "AttackSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack_summaries: Option<Vec<AttackSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttackSummary {
    #[serde(rename = "AttackId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack_id: Option<String>,
    #[serde(rename = "AttackVectors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack_vectors: Option<Vec<AttackVectorDescription>>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttackVectorDescription {
    #[serde(rename = "VectorType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListProtectionGroupsRequest {
    #[serde(rename = "InclusionFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inclusion_filters: Option<InclusionProtectionGroupFilters>,
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
pub struct InclusionProtectionGroupFilters {
    #[serde(rename = "Aggregations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregations: Option<Vec<String>>,
    #[serde(rename = "Patterns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patterns: Option<Vec<String>>,
    #[serde(rename = "ProtectionGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protection_group_ids: Option<Vec<String>>,
    #[serde(rename = "ResourceTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_types: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListProtectionGroupsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ProtectionGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protection_groups: Option<Vec<ProtectionGroup>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListProtectionsRequest {
    #[serde(rename = "InclusionFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inclusion_filters: Option<InclusionProtectionFilters>,
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
pub struct InclusionProtectionFilters {
    #[serde(rename = "ProtectionNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protection_names: Option<Vec<String>>,
    #[serde(rename = "ResourceArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arns: Option<Vec<String>>,
    #[serde(rename = "ResourceTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_types: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListProtectionsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Protections")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protections: Option<Vec<Protection>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListResourcesInProtectionGroupRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ProtectionGroupId")]
    #[serde(default)]
    pub protection_group_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListResourcesInProtectionGroupResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arns: Option<Vec<String>>,
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
pub struct UpdateApplicationLayerAutomaticResponseRequest {
    #[serde(rename = "Action")]
    #[serde(default)]
    pub action: ResponseAction,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateApplicationLayerAutomaticResponseResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateEmergencyContactSettingsRequest {
    #[serde(rename = "EmergencyContactList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emergency_contact_list: Option<Vec<EmergencyContact>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateEmergencyContactSettingsResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateProtectionGroupRequest {
    #[serde(rename = "Aggregation")]
    #[serde(default)]
    pub aggregation: String,
    #[serde(rename = "Members")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<String>>,
    #[serde(rename = "Pattern")]
    #[serde(default)]
    pub pattern: String,
    #[serde(rename = "ProtectionGroupId")]
    #[serde(default)]
    pub protection_group_id: String,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateProtectionGroupResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSubscriptionRequest {
    #[serde(rename = "AutoRenew")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSubscriptionResponse {}
