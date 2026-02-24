//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-dlm

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateLifecyclePolicyRequest {
    #[serde(rename = "CopyTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_tags: Option<bool>,
    #[serde(rename = "CreateInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_interval: Option<i32>,
    #[serde(rename = "CrossRegionCopyTargets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_region_copy_targets: Option<Vec<CrossRegionCopyTarget>>,
    #[serde(rename = "DefaultPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_policy: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    pub description: String,
    #[serde(rename = "Exclusions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusions: Option<Exclusions>,
    #[serde(rename = "ExecutionRoleArn")]
    #[serde(default)]
    pub execution_role_arn: String,
    #[serde(rename = "ExtendDeletion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extend_deletion: Option<bool>,
    #[serde(rename = "PolicyDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_details: Option<PolicyDetails>,
    #[serde(rename = "RetainInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retain_interval: Option<i32>,
    #[serde(rename = "State")]
    #[serde(default)]
    pub state: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CrossRegionCopyTarget {
    #[serde(rename = "TargetRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_region: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Exclusions {
    #[serde(rename = "ExcludeBootVolumes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_boot_volumes: Option<bool>,
    #[serde(rename = "ExcludeTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_tags: Option<Vec<Tag>>,
    #[serde(rename = "ExcludeVolumeTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_volume_types: Option<Vec<String>>,
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
pub struct PolicyDetails {
    #[serde(rename = "Actions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<Action>>,
    #[serde(rename = "CopyTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_tags: Option<bool>,
    #[serde(rename = "CreateInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_interval: Option<i32>,
    #[serde(rename = "CrossRegionCopyTargets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_region_copy_targets: Option<Vec<CrossRegionCopyTarget>>,
    #[serde(rename = "EventSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_source: Option<EventSource>,
    #[serde(rename = "Exclusions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusions: Option<Exclusions>,
    #[serde(rename = "ExtendDeletion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extend_deletion: Option<bool>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Parameters>,
    #[serde(rename = "PolicyLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_language: Option<String>,
    #[serde(rename = "PolicyType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<String>,
    #[serde(rename = "ResourceLocations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_locations: Option<Vec<String>>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "ResourceTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_types: Option<Vec<String>>,
    #[serde(rename = "RetainInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retain_interval: Option<i32>,
    #[serde(rename = "Schedules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedules: Option<Vec<Schedule>>,
    #[serde(rename = "TargetTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Action {
    #[serde(rename = "CrossRegionCopy")]
    #[serde(default)]
    pub cross_region_copy: Vec<CrossRegionCopyAction>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CrossRegionCopyAction {
    #[serde(rename = "EncryptionConfiguration")]
    #[serde(default)]
    pub encryption_configuration: EncryptionConfiguration,
    #[serde(rename = "RetainRule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retain_rule: Option<CrossRegionCopyRetainRule>,
    #[serde(rename = "Target")]
    #[serde(default)]
    pub target: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EncryptionConfiguration {
    #[serde(rename = "CmkArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmk_arn: Option<String>,
    #[serde(rename = "Encrypted")]
    #[serde(default)]
    pub encrypted: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CrossRegionCopyRetainRule {
    #[serde(rename = "Interval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<i32>,
    #[serde(rename = "IntervalUnit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_unit: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventSource {
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<EventParameters>,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventParameters {
    #[serde(rename = "DescriptionRegex")]
    #[serde(default)]
    pub description_regex: String,
    #[serde(rename = "EventType")]
    #[serde(default)]
    pub event_type: String,
    #[serde(rename = "SnapshotOwner")]
    #[serde(default)]
    pub snapshot_owner: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Parameters {
    #[serde(rename = "ExcludeBootVolume")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_boot_volume: Option<bool>,
    #[serde(rename = "ExcludeDataVolumeTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_data_volume_tags: Option<Vec<Tag>>,
    #[serde(rename = "NoReboot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_reboot: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Schedule {
    #[serde(rename = "ArchiveRule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_rule: Option<ArchiveRule>,
    #[serde(rename = "CopyTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_tags: Option<bool>,
    #[serde(rename = "CreateRule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_rule: Option<CreateRule>,
    #[serde(rename = "CrossRegionCopyRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_region_copy_rules: Option<Vec<CrossRegionCopyRule>>,
    #[serde(rename = "DeprecateRule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecate_rule: Option<DeprecateRule>,
    #[serde(rename = "FastRestoreRule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fast_restore_rule: Option<FastRestoreRule>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "RetainRule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retain_rule: Option<RetainRule>,
    #[serde(rename = "ShareRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_rules: Option<Vec<ShareRule>>,
    #[serde(rename = "TagsToAdd")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_to_add: Option<Vec<Tag>>,
    #[serde(rename = "VariableTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variable_tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ArchiveRule {
    #[serde(rename = "RetainRule")]
    #[serde(default)]
    pub retain_rule: ArchiveRetainRule,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ArchiveRetainRule {
    #[serde(rename = "RetentionArchiveTier")]
    #[serde(default)]
    pub retention_archive_tier: RetentionArchiveTier,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RetentionArchiveTier {
    #[serde(rename = "Count")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(rename = "Interval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<i32>,
    #[serde(rename = "IntervalUnit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_unit: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateRule {
    #[serde(rename = "CronExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cron_expression: Option<String>,
    #[serde(rename = "Interval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<i32>,
    #[serde(rename = "IntervalUnit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_unit: Option<String>,
    #[serde(rename = "Location")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(rename = "Scripts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scripts: Option<Vec<Script>>,
    #[serde(rename = "Times")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub times: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Script {
    #[serde(rename = "ExecuteOperationOnScriptFailure")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execute_operation_on_script_failure: Option<bool>,
    #[serde(rename = "ExecutionHandler")]
    #[serde(default)]
    pub execution_handler: String,
    #[serde(rename = "ExecutionHandlerService")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_handler_service: Option<String>,
    #[serde(rename = "ExecutionTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_timeout: Option<i32>,
    #[serde(rename = "MaximumRetryCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_retry_count: Option<i32>,
    #[serde(rename = "Stages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stages: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CrossRegionCopyRule {
    #[serde(rename = "CmkArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmk_arn: Option<String>,
    #[serde(rename = "CopyTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_tags: Option<bool>,
    #[serde(rename = "DeprecateRule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecate_rule: Option<CrossRegionCopyDeprecateRule>,
    #[serde(rename = "Encrypted")]
    #[serde(default)]
    pub encrypted: bool,
    #[serde(rename = "RetainRule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retain_rule: Option<CrossRegionCopyRetainRule>,
    #[serde(rename = "Target")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(rename = "TargetRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_region: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CrossRegionCopyDeprecateRule {
    #[serde(rename = "Interval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<i32>,
    #[serde(rename = "IntervalUnit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_unit: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeprecateRule {
    #[serde(rename = "Count")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(rename = "Interval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<i32>,
    #[serde(rename = "IntervalUnit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_unit: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FastRestoreRule {
    #[serde(rename = "AvailabilityZones")]
    #[serde(default)]
    pub availability_zones: Vec<String>,
    #[serde(rename = "Count")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(rename = "Interval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<i32>,
    #[serde(rename = "IntervalUnit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_unit: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RetainRule {
    #[serde(rename = "Count")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(rename = "Interval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<i32>,
    #[serde(rename = "IntervalUnit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_unit: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ShareRule {
    #[serde(rename = "TargetAccounts")]
    #[serde(default)]
    pub target_accounts: Vec<String>,
    #[serde(rename = "UnshareInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unshare_interval: Option<i32>,
    #[serde(rename = "UnshareIntervalUnit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unshare_interval_unit: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateLifecyclePolicyResponse {
    #[serde(rename = "PolicyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteLifecyclePolicyRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteLifecyclePolicyResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetLifecyclePoliciesRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetLifecyclePoliciesResponse {
    #[serde(rename = "Policies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<LifecyclePolicySummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LifecyclePolicySummary {
    #[serde(rename = "DefaultPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_policy: Option<bool>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "PolicyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    #[serde(rename = "PolicyType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<String>,
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
pub struct GetLifecyclePolicyRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetLifecyclePolicyResponse {
    #[serde(rename = "Policy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<LifecyclePolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LifecyclePolicy {
    #[serde(rename = "DateCreated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    #[serde(rename = "DateModified")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<String>,
    #[serde(rename = "DefaultPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_policy: Option<bool>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ExecutionRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role_arn: Option<String>,
    #[serde(rename = "PolicyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_arn: Option<String>,
    #[serde(rename = "PolicyDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_details: Option<PolicyDetails>,
    #[serde(rename = "PolicyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateLifecyclePolicyRequest {
    #[serde(rename = "CopyTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_tags: Option<bool>,
    #[serde(rename = "CreateInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_interval: Option<i32>,
    #[serde(rename = "CrossRegionCopyTargets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_region_copy_targets: Option<Vec<CrossRegionCopyTarget>>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Exclusions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusions: Option<Exclusions>,
    #[serde(rename = "ExecutionRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role_arn: Option<String>,
    #[serde(rename = "ExtendDeletion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extend_deletion: Option<bool>,
    #[serde(rename = "PolicyDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_details: Option<PolicyDetails>,
    #[serde(rename = "RetainInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retain_interval: Option<i32>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateLifecyclePolicyResponse {}
