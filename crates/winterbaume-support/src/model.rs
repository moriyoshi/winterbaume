//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-support

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddAttachmentsToSetRequest {
    #[serde(rename = "attachmentSetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_set_id: Option<String>,
    #[serde(default)]
    pub attachments: Vec<Attachment>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Attachment {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(rename = "fileName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddAttachmentsToSetResponse {
    #[serde(rename = "attachmentSetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_set_id: Option<String>,
    #[serde(rename = "expiryTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddCommunicationToCaseRequest {
    #[serde(rename = "attachmentSetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_set_id: Option<String>,
    #[serde(rename = "caseId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub case_id: Option<String>,
    #[serde(rename = "ccEmailAddresses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc_email_addresses: Option<Vec<String>>,
    #[serde(rename = "communicationBody")]
    #[serde(default)]
    pub communication_body: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddCommunicationToCaseResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCaseRequest {
    #[serde(rename = "attachmentSetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_set_id: Option<String>,
    #[serde(rename = "categoryCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_code: Option<String>,
    #[serde(rename = "ccEmailAddresses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc_email_addresses: Option<Vec<String>>,
    #[serde(rename = "communicationBody")]
    #[serde(default)]
    pub communication_body: String,
    #[serde(rename = "issueType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(rename = "serviceCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_code: Option<String>,
    #[serde(rename = "severityCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity_code: Option<String>,
    #[serde(default)]
    pub subject: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCaseResponse {
    #[serde(rename = "caseId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub case_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAttachmentRequest {
    #[serde(rename = "attachmentId")]
    #[serde(default)]
    pub attachment_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAttachmentResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment: Option<Attachment>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeCasesRequest {
    #[serde(rename = "afterTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_time: Option<String>,
    #[serde(rename = "beforeTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_time: Option<String>,
    #[serde(rename = "caseIdList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub case_id_list: Option<Vec<String>>,
    #[serde(rename = "displayId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_id: Option<String>,
    #[serde(rename = "includeCommunications")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_communications: Option<bool>,
    #[serde(rename = "includeResolvedCases")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_resolved_cases: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
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
pub struct DescribeCasesResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cases: Option<Vec<CaseDetails>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CaseDetails {
    #[serde(rename = "caseId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub case_id: Option<String>,
    #[serde(rename = "categoryCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_code: Option<String>,
    #[serde(rename = "ccEmailAddresses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc_email_addresses: Option<Vec<String>>,
    #[serde(rename = "displayId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(rename = "recentCommunications")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recent_communications: Option<RecentCaseCommunications>,
    #[serde(rename = "serviceCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_code: Option<String>,
    #[serde(rename = "severityCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity_code: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(rename = "submittedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submitted_by: Option<String>,
    #[serde(rename = "timeCreated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_created: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecentCaseCommunications {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub communications: Option<Vec<Communication>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Communication {
    #[serde(rename = "attachmentSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_set: Option<Vec<AttachmentDetails>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(rename = "caseId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub case_id: Option<String>,
    #[serde(rename = "submittedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submitted_by: Option<String>,
    #[serde(rename = "timeCreated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_created: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttachmentDetails {
    #[serde(rename = "attachmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_id: Option<String>,
    #[serde(rename = "fileName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeCommunicationsRequest {
    #[serde(rename = "afterTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_time: Option<String>,
    #[serde(rename = "beforeTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_time: Option<String>,
    #[serde(rename = "caseId")]
    #[serde(default)]
    pub case_id: String,
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
pub struct DescribeCommunicationsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub communications: Option<Vec<Communication>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeCreateCaseOptionsRequest {
    #[serde(rename = "categoryCode")]
    #[serde(default)]
    pub category_code: String,
    #[serde(rename = "issueType")]
    #[serde(default)]
    pub issue_type: String,
    #[serde(default)]
    pub language: String,
    #[serde(rename = "serviceCode")]
    #[serde(default)]
    pub service_code: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeCreateCaseOptionsResponse {
    #[serde(rename = "communicationTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub communication_types: Option<Vec<CommunicationTypeOptions>>,
    #[serde(rename = "languageAvailability")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_availability: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CommunicationTypeOptions {
    #[serde(rename = "datesWithoutSupport")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dates_without_support: Option<Vec<DateInterval>>,
    #[serde(rename = "supportedHours")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_hours: Option<Vec<SupportedHour>>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DateInterval {
    #[serde(rename = "endDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date_time: Option<String>,
    #[serde(rename = "startDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SupportedHour {
    #[serde(rename = "endTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(rename = "startTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeServicesRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(rename = "serviceCodeList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_code_list: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeServicesResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<Service>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Service {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<Category>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Category {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSeverityLevelsRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSeverityLevelsResponse {
    #[serde(rename = "severityLevels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity_levels: Option<Vec<SeverityLevel>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SeverityLevel {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSupportedLanguagesRequest {
    #[serde(rename = "categoryCode")]
    #[serde(default)]
    pub category_code: String,
    #[serde(rename = "issueType")]
    #[serde(default)]
    pub issue_type: String,
    #[serde(rename = "serviceCode")]
    #[serde(default)]
    pub service_code: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSupportedLanguagesResponse {
    #[serde(rename = "supportedLanguages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_languages: Option<Vec<SupportedLanguage>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SupportedLanguage {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTrustedAdvisorCheckRefreshStatusesRequest {
    #[serde(rename = "checkIds")]
    #[serde(default)]
    pub check_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTrustedAdvisorCheckRefreshStatusesResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Vec<TrustedAdvisorCheckRefreshStatus>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TrustedAdvisorCheckRefreshStatus {
    #[serde(rename = "checkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_id: Option<String>,
    #[serde(rename = "millisUntilNextRefreshable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub millis_until_next_refreshable: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTrustedAdvisorCheckResultRequest {
    #[serde(rename = "checkId")]
    #[serde(default)]
    pub check_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTrustedAdvisorCheckResultResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<TrustedAdvisorCheckResult>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TrustedAdvisorCheckResult {
    #[serde(rename = "categorySpecificSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_specific_summary: Option<TrustedAdvisorCategorySpecificSummary>,
    #[serde(rename = "checkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_id: Option<String>,
    #[serde(rename = "flaggedResources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flagged_resources: Option<Vec<TrustedAdvisorResourceDetail>>,
    #[serde(rename = "resourcesSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources_summary: Option<TrustedAdvisorResourcesSummary>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TrustedAdvisorCategorySpecificSummary {
    #[serde(rename = "costOptimizing")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_optimizing: Option<TrustedAdvisorCostOptimizingSummary>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TrustedAdvisorCostOptimizingSummary {
    #[serde(rename = "estimatedMonthlySavings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_monthly_savings: Option<f64>,
    #[serde(rename = "estimatedPercentMonthlySavings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_percent_monthly_savings: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TrustedAdvisorResourceDetail {
    #[serde(rename = "isSuppressed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_suppressed: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "resourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TrustedAdvisorResourcesSummary {
    #[serde(rename = "resourcesFlagged")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources_flagged: Option<i64>,
    #[serde(rename = "resourcesIgnored")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources_ignored: Option<i64>,
    #[serde(rename = "resourcesProcessed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources_processed: Option<i64>,
    #[serde(rename = "resourcesSuppressed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources_suppressed: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTrustedAdvisorCheckSummariesRequest {
    #[serde(rename = "checkIds")]
    #[serde(default)]
    pub check_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTrustedAdvisorCheckSummariesResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summaries: Option<Vec<TrustedAdvisorCheckSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TrustedAdvisorCheckSummary {
    #[serde(rename = "categorySpecificSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_specific_summary: Option<TrustedAdvisorCategorySpecificSummary>,
    #[serde(rename = "checkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_id: Option<String>,
    #[serde(rename = "hasFlaggedResources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_flagged_resources: Option<bool>,
    #[serde(rename = "resourcesSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources_summary: Option<TrustedAdvisorResourcesSummary>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTrustedAdvisorChecksRequest {
    #[serde(default)]
    pub language: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTrustedAdvisorChecksResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checks: Option<Vec<TrustedAdvisorCheckDescription>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TrustedAdvisorCheckDescription {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RefreshTrustedAdvisorCheckRequest {
    #[serde(rename = "checkId")]
    #[serde(default)]
    pub check_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RefreshTrustedAdvisorCheckResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TrustedAdvisorCheckRefreshStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResolveCaseRequest {
    #[serde(rename = "caseId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub case_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResolveCaseResponse {
    #[serde(rename = "finalCaseStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_case_status: Option<String>,
    #[serde(rename = "initialCaseStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_case_status: Option<String>,
}
