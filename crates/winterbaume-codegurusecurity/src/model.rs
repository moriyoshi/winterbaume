//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-codegurusecurity

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetFindingsRequest {
    #[serde(rename = "findingIdentifiers")]
    #[serde(default)]
    pub finding_identifiers: Vec<FindingIdentifier>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FindingIdentifier {
    #[serde(rename = "findingId")]
    #[serde(default)]
    pub finding_id: String,
    #[serde(rename = "scanName")]
    #[serde(default)]
    pub scan_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetFindingsResponse {
    #[serde(rename = "failedFindings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_findings: Option<Vec<BatchGetFindingsError>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub findings: Option<Vec<Finding>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetFindingsError {
    #[serde(rename = "errorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "findingId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "scanName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Finding {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "detectorId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector_id: Option<String>,
    #[serde(rename = "detectorName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector_name: Option<String>,
    #[serde(rename = "detectorTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detector_tags: Option<Vec<String>>,
    #[serde(rename = "generatorId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generator_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remediation: Option<Remediation>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<Resource>,
    #[serde(rename = "ruleId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vulnerability: Option<Vulnerability>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Remediation {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation: Option<Recommendation>,
    #[serde(rename = "suggestedFixes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_fixes: Option<Vec<SuggestedFix>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Recommendation {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SuggestedFix {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Resource {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "subResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_resource_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Vulnerability {
    #[serde(rename = "filePath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<FilePath>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "itemCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_count: Option<i32>,
    #[serde(rename = "referenceUrls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_urls: Option<Vec<String>>,
    #[serde(rename = "relatedVulnerabilities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_vulnerabilities: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FilePath {
    #[serde(rename = "codeSnippet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_snippet: Option<Vec<CodeLine>>,
    #[serde(rename = "endLine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_line: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "startLine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_line: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CodeLine {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateScanRequest {
    #[serde(rename = "analysisType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_type: Option<String>,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "resourceId")]
    #[serde(default)]
    pub resource_id: ResourceId,
    #[serde(rename = "scanName")]
    #[serde(default)]
    pub scan_name: String,
    #[serde(rename = "scanType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceId {
    #[serde(rename = "codeArtifactId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_artifact_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateScanResponse {
    #[serde(rename = "resourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<ResourceId>,
    #[serde(rename = "runId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[serde(rename = "scanName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_name: Option<String>,
    #[serde(rename = "scanNameArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_name_arn: Option<String>,
    #[serde(rename = "scanState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateUploadUrlRequest {
    #[serde(rename = "scanName")]
    #[serde(default)]
    pub scan_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateUploadUrlResponse {
    #[serde(rename = "codeArtifactId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_artifact_id: Option<String>,
    #[serde(rename = "requestHeaders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_headers: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "s3Url")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAccountConfigurationRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAccountConfigurationResponse {
    #[serde(rename = "encryptionConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_config: Option<EncryptionConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EncryptionConfig {
    #[serde(rename = "kmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFindingsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFindingsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub findings: Option<Vec<Finding>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMetricsSummaryRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMetricsSummaryResponse {
    #[serde(rename = "metricsSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics_summary: Option<MetricsSummary>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MetricsSummary {
    #[serde(rename = "categoriesWithMostFindings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories_with_most_findings: Option<Vec<CategoryWithFindingNum>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<f64>,
    #[serde(rename = "openFindings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_findings: Option<FindingMetricsValuePerSeverity>,
    #[serde(rename = "scansWithMostOpenCriticalFindings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scans_with_most_open_critical_findings: Option<Vec<ScanNameWithFindingNum>>,
    #[serde(rename = "scansWithMostOpenFindings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scans_with_most_open_findings: Option<Vec<ScanNameWithFindingNum>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CategoryWithFindingNum {
    #[serde(rename = "categoryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_name: Option<String>,
    #[serde(rename = "findingNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_number: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FindingMetricsValuePerSeverity {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub critical: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub low: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medium: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScanNameWithFindingNum {
    #[serde(rename = "findingNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_number: Option<i32>,
    #[serde(rename = "scanName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetScanRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetScanResponse {
    #[serde(rename = "analysisType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_type: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "errorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "numberOfRevisions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_revisions: Option<i64>,
    #[serde(rename = "runId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[serde(rename = "scanName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_name: Option<String>,
    #[serde(rename = "scanNameArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_name_arn: Option<String>,
    #[serde(rename = "scanState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_state: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFindingsMetricsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFindingsMetricsResponse {
    #[serde(rename = "findingsMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub findings_metrics: Option<Vec<AccountFindingsMetric>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccountFindingsMetric {
    #[serde(rename = "closedFindings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed_findings: Option<FindingMetricsValuePerSeverity>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<f64>,
    #[serde(rename = "meanTimeToClose")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mean_time_to_close: Option<FindingMetricsValuePerSeverity>,
    #[serde(rename = "newFindings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_findings: Option<FindingMetricsValuePerSeverity>,
    #[serde(rename = "openFindings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_findings: Option<FindingMetricsValuePerSeverity>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListScansRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListScansResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summaries: Option<Vec<ScanSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScanSummary {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "runId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[serde(rename = "scanName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_name: Option<String>,
    #[serde(rename = "scanNameArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_name_arn: Option<String>,
    #[serde(rename = "scanState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_state: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
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
pub struct UpdateAccountConfigurationRequest {
    #[serde(rename = "encryptionConfig")]
    #[serde(default)]
    pub encryption_config: EncryptionConfig,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAccountConfigurationResponse {
    #[serde(rename = "encryptionConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_config: Option<EncryptionConfig>,
}
