//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-simpledbv2

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetExportRequest {
    #[serde(rename = "exportArn")]
    #[serde(default)]
    pub export_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetExportResponse {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "domainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(rename = "exportArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_arn: Option<String>,
    #[serde(rename = "exportDataCutoffTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_data_cutoff_time: Option<f64>,
    #[serde(rename = "exportManifest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_manifest: Option<String>,
    #[serde(rename = "exportStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_status: Option<String>,
    #[serde(rename = "failureCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<String>,
    #[serde(rename = "failureMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
    #[serde(rename = "itemsCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items_count: Option<i64>,
    #[serde(rename = "requestedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_at: Option<f64>,
    #[serde(rename = "s3Bucket")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket: Option<String>,
    #[serde(rename = "s3BucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_owner: Option<String>,
    #[serde(rename = "s3KeyPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key_prefix: Option<String>,
    #[serde(rename = "s3SseAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_sse_algorithm: Option<String>,
    #[serde(rename = "s3SseKmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_sse_kms_key_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListExportsRequest {
    #[serde(rename = "domainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
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
pub struct ListExportsResponse {
    #[serde(rename = "exportSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_summaries: Option<Vec<ExportSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExportSummary {
    #[serde(rename = "domainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(rename = "exportArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_arn: Option<String>,
    #[serde(rename = "exportStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_status: Option<String>,
    #[serde(rename = "requestedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartDomainExportRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "domainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "s3Bucket")]
    #[serde(default)]
    pub s3_bucket: String,
    #[serde(rename = "s3BucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_owner: Option<String>,
    #[serde(rename = "s3KeyPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key_prefix: Option<String>,
    #[serde(rename = "s3SseAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_sse_algorithm: Option<String>,
    #[serde(rename = "s3SseKmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_sse_kms_key_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartDomainExportResponse {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "exportArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_arn: Option<String>,
    #[serde(rename = "requestedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_at: Option<f64>,
}
