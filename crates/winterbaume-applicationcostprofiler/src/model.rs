//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-applicationcostprofiler

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteReportDefinitionRequest {
    #[serde(rename = "reportId")]
    #[serde(default)]
    pub report_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteReportDefinitionResult {
    #[serde(rename = "reportId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetReportDefinitionRequest {
    #[serde(rename = "reportId")]
    #[serde(default)]
    pub report_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetReportDefinitionResult {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "destinationS3Location")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_s3_location: Option<S3Location>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(rename = "lastUpdated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<f64>,
    #[serde(rename = "reportDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_description: Option<String>,
    #[serde(rename = "reportFrequency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_frequency: Option<String>,
    #[serde(rename = "reportId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3Location {
    #[serde(default)]
    pub bucket: String,
    #[serde(default)]
    pub prefix: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportApplicationUsageRequest {
    #[serde(rename = "sourceS3Location")]
    #[serde(default)]
    pub source_s3_location: SourceS3Location,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SourceS3Location {
    #[serde(default)]
    pub bucket: String,
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportApplicationUsageResult {
    #[serde(rename = "importId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListReportDefinitionsRequest {
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
pub struct ListReportDefinitionsResult {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "reportDefinitions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_definitions: Option<Vec<ReportDefinition>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReportDefinition {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "destinationS3Location")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_s3_location: Option<S3Location>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    #[serde(rename = "reportDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_description: Option<String>,
    #[serde(rename = "reportFrequency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_frequency: Option<String>,
    #[serde(rename = "reportId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutReportDefinitionRequest {
    #[serde(rename = "destinationS3Location")]
    #[serde(default)]
    pub destination_s3_location: S3Location,
    #[serde(default)]
    pub format: String,
    #[serde(rename = "reportDescription")]
    #[serde(default)]
    pub report_description: String,
    #[serde(rename = "reportFrequency")]
    #[serde(default)]
    pub report_frequency: String,
    #[serde(rename = "reportId")]
    #[serde(default)]
    pub report_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutReportDefinitionResult {
    #[serde(rename = "reportId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateReportDefinitionRequest {
    #[serde(rename = "destinationS3Location")]
    #[serde(default)]
    pub destination_s3_location: S3Location,
    #[serde(default)]
    pub format: String,
    #[serde(rename = "reportDescription")]
    #[serde(default)]
    pub report_description: String,
    #[serde(rename = "reportFrequency")]
    #[serde(default)]
    pub report_frequency: String,
    #[serde(rename = "reportId")]
    #[serde(default)]
    pub report_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateReportDefinitionResult {
    #[serde(rename = "reportId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_id: Option<String>,
}
