//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-costandusagereport

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteReportDefinitionRequest {
    #[serde(rename = "ReportName")]
    #[serde(default)]
    pub report_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteReportDefinitionResponse {
    #[serde(rename = "ResponseMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeReportDefinitionsRequest {
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
pub struct DescribeReportDefinitionsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ReportDefinitions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_definitions: Option<Vec<ReportDefinition>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReportDefinition {
    #[serde(rename = "AdditionalArtifacts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_artifacts: Option<Vec<String>>,
    #[serde(rename = "AdditionalSchemaElements")]
    #[serde(default)]
    pub additional_schema_elements: Vec<String>,
    #[serde(rename = "BillingViewArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_view_arn: Option<String>,
    #[serde(rename = "Compression")]
    #[serde(default)]
    pub compression: String,
    #[serde(rename = "Format")]
    #[serde(default)]
    pub format: String,
    #[serde(rename = "RefreshClosedReports")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_closed_reports: Option<bool>,
    #[serde(rename = "ReportName")]
    #[serde(default)]
    pub report_name: String,
    #[serde(rename = "ReportStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_status: Option<ReportStatus>,
    #[serde(rename = "ReportVersioning")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_versioning: Option<String>,
    #[serde(rename = "S3Bucket")]
    #[serde(default)]
    pub s3_bucket: String,
    #[serde(rename = "S3Prefix")]
    #[serde(default)]
    pub s3_prefix: String,
    #[serde(rename = "S3Region")]
    #[serde(default)]
    pub s3_region: String,
    #[serde(rename = "TimeUnit")]
    #[serde(default)]
    pub time_unit: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReportStatus {
    #[serde(rename = "lastDelivery")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_delivery: Option<String>,
    #[serde(rename = "lastStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "ReportName")]
    #[serde(default)]
    pub report_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
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
pub struct ModifyReportDefinitionRequest {
    #[serde(rename = "ReportDefinition")]
    #[serde(default)]
    pub report_definition: ReportDefinition,
    #[serde(rename = "ReportName")]
    #[serde(default)]
    pub report_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModifyReportDefinitionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutReportDefinitionRequest {
    #[serde(rename = "ReportDefinition")]
    #[serde(default)]
    pub report_definition: ReportDefinition,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutReportDefinitionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "ReportName")]
    #[serde(default)]
    pub report_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "ReportName")]
    #[serde(default)]
    pub report_name: String,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceResponse {}
