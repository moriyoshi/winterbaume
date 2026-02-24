//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-tagging

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetComplianceSummaryOutput {
    #[serde(rename = "PaginationToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_token: Option<String>,
    #[serde(rename = "SummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary_list: Option<Vec<Summary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Summary {
    #[serde(rename = "LastUpdated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
    #[serde(rename = "NonCompliantResources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_compliant_resources: Option<i64>,
    #[serde(rename = "Region")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "TargetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
    #[serde(rename = "TargetIdType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetComplianceSummaryInput {
    #[serde(rename = "GroupBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by: Option<Vec<String>>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "PaginationToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_token: Option<String>,
    #[serde(rename = "RegionFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_filters: Option<Vec<String>>,
    #[serde(rename = "ResourceTypeFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type_filters: Option<Vec<String>>,
    #[serde(rename = "TagKeyFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key_filters: Option<Vec<String>>,
    #[serde(rename = "TargetIdFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id_filters: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTagKeysOutput {
    #[serde(rename = "PaginationToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_token: Option<String>,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_keys: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRequiredTagsInput {
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
pub struct ListRequiredTagsOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RequiredTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_tags: Option<Vec<RequiredTag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RequiredTag {
    #[serde(rename = "CloudFormationResourceTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_formation_resource_types: Option<Vec<String>>,
    #[serde(rename = "ReportingTagKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reporting_tag_keys: Option<Vec<String>>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartReportCreationInput {
    #[serde(rename = "S3Bucket")]
    #[serde(default)]
    pub s3_bucket: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartReportCreationOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourcesOutput {
    #[serde(rename = "FailedResourcesMap")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_resources_map: Option<std::collections::HashMap<String, FailureInfo>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FailureInfo {
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "StatusCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeReportCreationInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeReportCreationOutput {
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "S3Location")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_location: Option<String>,
    #[serde(rename = "StartDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourcesInput {
    #[serde(rename = "ResourceARNList")]
    #[serde(default)]
    pub resource_a_r_n_list: Vec<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourcesOutput {
    #[serde(rename = "FailedResourcesMap")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_resources_map: Option<std::collections::HashMap<String, FailureInfo>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTagValuesOutput {
    #[serde(rename = "PaginationToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_token: Option<String>,
    #[serde(rename = "TagValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourcesOutput {
    #[serde(rename = "PaginationToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_token: Option<String>,
    #[serde(rename = "ResourceTagMappingList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_tag_mapping_list: Option<Vec<ResourceTagMapping>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceTagMapping {
    #[serde(rename = "ComplianceDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_details: Option<ComplianceDetails>,
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_a_r_n: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ComplianceDetails {
    #[serde(rename = "ComplianceStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_status: Option<bool>,
    #[serde(rename = "KeysWithNoncompliantValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keys_with_noncompliant_values: Option<Vec<String>>,
    #[serde(rename = "NoncompliantKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub noncompliant_keys: Option<Vec<String>>,
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
pub struct GetTagKeysInput {
    #[serde(rename = "PaginationToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourcesInput {
    #[serde(rename = "ExcludeCompliantResources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_compliant_resources: Option<bool>,
    #[serde(rename = "IncludeComplianceDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_compliance_details: Option<bool>,
    #[serde(rename = "PaginationToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_token: Option<String>,
    #[serde(rename = "ResourceARNList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_a_r_n_list: Option<Vec<String>>,
    #[serde(rename = "ResourceTypeFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type_filters: Option<Vec<String>>,
    #[serde(rename = "ResourcesPerPage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources_per_page: Option<i32>,
    #[serde(rename = "TagFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_filters: Option<Vec<TagFilter>>,
    #[serde(rename = "TagsPerPage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_per_page: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagFilter {
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
pub struct GetTagValuesInput {
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "PaginationToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourcesInput {
    #[serde(rename = "ResourceARNList")]
    #[serde(default)]
    pub resource_a_r_n_list: Vec<String>,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}
