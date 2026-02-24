//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-inspector2

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateMemberRequest {
    #[serde(rename = "accountId")]
    #[serde(default)]
    pub account_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateMemberResponse {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchAssociateCodeSecurityScanConfigurationRequest {
    #[serde(rename = "associateConfigurationRequests")]
    #[serde(default)]
    pub associate_configuration_requests: Vec<AssociateConfigurationRequest>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateConfigurationRequest {
    #[serde(default)]
    pub resource: CodeSecurityResource,
    #[serde(rename = "scanConfigurationArn")]
    #[serde(default)]
    pub scan_configuration_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CodeSecurityResource {
    #[serde(rename = "projectId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchAssociateCodeSecurityScanConfigurationResponse {
    #[serde(rename = "failedAssociations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_associations: Option<Vec<FailedAssociationResult>>,
    #[serde(rename = "successfulAssociations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful_associations: Option<Vec<SuccessfulAssociationResult>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FailedAssociationResult {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<CodeSecurityResource>,
    #[serde(rename = "scanConfigurationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_configuration_arn: Option<String>,
    #[serde(rename = "statusCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
    #[serde(rename = "statusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SuccessfulAssociationResult {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<CodeSecurityResource>,
    #[serde(rename = "scanConfigurationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_configuration_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDisassociateCodeSecurityScanConfigurationRequest {
    #[serde(rename = "disassociateConfigurationRequests")]
    #[serde(default)]
    pub disassociate_configuration_requests: Vec<DisassociateConfigurationRequest>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateConfigurationRequest {
    #[serde(default)]
    pub resource: CodeSecurityResource,
    #[serde(rename = "scanConfigurationArn")]
    #[serde(default)]
    pub scan_configuration_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDisassociateCodeSecurityScanConfigurationResponse {
    #[serde(rename = "failedAssociations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_associations: Option<Vec<FailedAssociationResult>>,
    #[serde(rename = "successfulAssociations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful_associations: Option<Vec<SuccessfulAssociationResult>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetAccountStatusRequest {
    #[serde(rename = "accountIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetAccountStatusResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<AccountState>>,
    #[serde(rename = "failedAccounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_accounts: Option<Vec<FailedAccount>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccountState {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "resourceState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_state: Option<ResourceState>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceState {
    #[serde(rename = "codeRepository")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_repository: Option<State>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2: Option<State>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecr: Option<State>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda: Option<State>,
    #[serde(rename = "lambdaCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_code: Option<State>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct State {
    #[serde(rename = "errorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "errorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FailedAccount {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "errorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "errorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "resourceStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_status: Option<ResourceStatus>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceStatus {
    #[serde(rename = "codeRepository")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_repository: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecr: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda: Option<String>,
    #[serde(rename = "lambdaCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_code: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetCodeSnippetRequest {
    #[serde(rename = "findingArns")]
    #[serde(default)]
    pub finding_arns: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetCodeSnippetResponse {
    #[serde(rename = "codeSnippetResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_snippet_results: Option<Vec<CodeSnippetResult>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<CodeSnippetError>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CodeSnippetResult {
    #[serde(rename = "codeSnippet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_snippet: Option<Vec<CodeLine>>,
    #[serde(rename = "endLine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_line: Option<i32>,
    #[serde(rename = "findingArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_arn: Option<String>,
    #[serde(rename = "startLine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_line: Option<i32>,
    #[serde(rename = "suggestedFixes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_fixes: Option<Vec<SuggestedFix>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CodeLine {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(rename = "lineNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_number: Option<i32>,
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
pub struct CodeSnippetError {
    #[serde(rename = "errorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "errorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "findingArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetFindingDetailsRequest {
    #[serde(rename = "findingArns")]
    #[serde(default)]
    pub finding_arns: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetFindingDetailsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<FindingDetailsError>>,
    #[serde(rename = "findingDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_details: Option<Vec<FindingDetail>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FindingDetailsError {
    #[serde(rename = "errorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "errorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "findingArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FindingDetail {
    #[serde(rename = "cisaData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cisa_data: Option<CisaData>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cwes: Option<Vec<String>>,
    #[serde(rename = "epssScore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub epss_score: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidences: Option<Vec<Evidence>>,
    #[serde(rename = "exploitObserved")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exploit_observed: Option<ExploitObserved>,
    #[serde(rename = "findingArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_arn: Option<String>,
    #[serde(rename = "referenceUrls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_urls: Option<Vec<String>>,
    #[serde(rename = "riskScore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_score: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttps: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CisaData {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "dateAdded")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_added: Option<f64>,
    #[serde(rename = "dateDue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_due: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Evidence {
    #[serde(rename = "evidenceDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_detail: Option<String>,
    #[serde(rename = "evidenceRule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_rule: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExploitObserved {
    #[serde(rename = "firstSeen")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_seen: Option<f64>,
    #[serde(rename = "lastSeen")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_seen: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetFreeTrialInfoRequest {
    #[serde(rename = "accountIds")]
    #[serde(default)]
    pub account_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetFreeTrialInfoResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<FreeTrialAccountInfo>>,
    #[serde(rename = "failedAccounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_accounts: Option<Vec<FreeTrialInfoError>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FreeTrialAccountInfo {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "freeTrialInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free_trial_info: Option<Vec<FreeTrialInfo>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FreeTrialInfo {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FreeTrialInfoError {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetMemberEc2DeepInspectionStatusRequest {
    #[serde(rename = "accountIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetMemberEc2DeepInspectionStatusResponse {
    #[serde(rename = "accountIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<MemberAccountEc2DeepInspectionStatusState>>,
    #[serde(rename = "failedAccountIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_account_ids: Option<Vec<FailedMemberAccountEc2DeepInspectionStatusState>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MemberAccountEc2DeepInspectionStatusState {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "errorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FailedMemberAccountEc2DeepInspectionStatusState {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "ec2ScanStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_scan_status: Option<String>,
    #[serde(rename = "errorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchUpdateMemberEc2DeepInspectionStatusRequest {
    #[serde(rename = "accountIds")]
    #[serde(default)]
    pub account_ids: Vec<MemberAccountEc2DeepInspectionStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MemberAccountEc2DeepInspectionStatus {
    #[serde(rename = "accountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "activateDeepInspection")]
    #[serde(default)]
    pub activate_deep_inspection: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchUpdateMemberEc2DeepInspectionStatusResponse {
    #[serde(rename = "accountIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<MemberAccountEc2DeepInspectionStatusState>>,
    #[serde(rename = "failedAccountIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_account_ids: Option<Vec<FailedMemberAccountEc2DeepInspectionStatusState>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelFindingsReportRequest {
    #[serde(rename = "reportId")]
    #[serde(default)]
    pub report_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelFindingsReportResponse {
    #[serde(rename = "reportId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelSbomExportRequest {
    #[serde(rename = "reportId")]
    #[serde(default)]
    pub report_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelSbomExportResponse {
    #[serde(rename = "reportId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCisScanConfigurationRequest {
    #[serde(rename = "scanName")]
    #[serde(default)]
    pub scan_name: String,
    #[serde(default)]
    pub schedule: Schedule,
    #[serde(rename = "securityLevel")]
    #[serde(default)]
    pub security_level: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    pub targets: CreateCisTargets,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Schedule {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily: Option<DailySchedule>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monthly: Option<MonthlySchedule>,
    #[serde(rename = "oneTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_time: Option<OneTimeSchedule>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekly: Option<WeeklySchedule>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DailySchedule {
    #[serde(rename = "startTime")]
    #[serde(default)]
    pub start_time: Time,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Time {
    #[serde(rename = "timeOfDay")]
    #[serde(default)]
    pub time_of_day: String,
    #[serde(default)]
    pub timezone: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MonthlySchedule {
    #[serde(default)]
    pub day: String,
    #[serde(rename = "startTime")]
    #[serde(default)]
    pub start_time: Time,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OneTimeSchedule {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WeeklySchedule {
    #[serde(default)]
    pub days: Vec<String>,
    #[serde(rename = "startTime")]
    #[serde(default)]
    pub start_time: Time,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCisTargets {
    #[serde(rename = "accountIds")]
    #[serde(default)]
    pub account_ids: Vec<String>,
    #[serde(rename = "targetResourceTags")]
    #[serde(default)]
    pub target_resource_tags: std::collections::HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCisScanConfigurationResponse {
    #[serde(rename = "scanConfigurationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_configuration_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCodeSecurityIntegrationRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<CreateIntegrationDetail>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateIntegrationDetail {
    #[serde(rename = "gitlabSelfManaged")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gitlab_self_managed: Option<CreateGitLabSelfManagedIntegrationDetail>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateGitLabSelfManagedIntegrationDetail {
    #[serde(rename = "accessToken")]
    #[serde(default)]
    pub access_token: String,
    #[serde(rename = "instanceUrl")]
    #[serde(default)]
    pub instance_url: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCodeSecurityIntegrationResponse {
    #[serde(rename = "authorizationUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_url: Option<String>,
    #[serde(rename = "integrationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCodeSecurityScanConfigurationRequest {
    #[serde(default)]
    pub configuration: CodeSecurityScanConfiguration,
    #[serde(default)]
    pub level: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "scopeSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_settings: Option<ScopeSettings>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CodeSecurityScanConfiguration {
    #[serde(rename = "continuousIntegrationScanConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuous_integration_scan_configuration: Option<ContinuousIntegrationScanConfiguration>,
    #[serde(rename = "periodicScanConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub periodic_scan_configuration: Option<PeriodicScanConfiguration>,
    #[serde(rename = "ruleSetCategories")]
    #[serde(default)]
    pub rule_set_categories: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContinuousIntegrationScanConfiguration {
    #[serde(rename = "supportedEvents")]
    #[serde(default)]
    pub supported_events: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PeriodicScanConfiguration {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<String>,
    #[serde(rename = "frequencyExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_expression: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScopeSettings {
    #[serde(rename = "projectSelectionScope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_selection_scope: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCodeSecurityScanConfigurationResponse {
    #[serde(rename = "scanConfigurationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_configuration_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFilterRequest {
    #[serde(default)]
    pub action: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "filterCriteria")]
    #[serde(default)]
    pub filter_criteria: FilterCriteria,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FilterCriteria {
    #[serde(rename = "awsAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_account_id: Option<Vec<StringFilter>>,
    #[serde(rename = "codeRepositoryProjectName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_repository_project_name: Option<Vec<StringFilter>>,
    #[serde(rename = "codeRepositoryProviderType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_repository_provider_type: Option<Vec<StringFilter>>,
    #[serde(rename = "codeVulnerabilityDetectorName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_vulnerability_detector_name: Option<Vec<StringFilter>>,
    #[serde(rename = "codeVulnerabilityDetectorTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_vulnerability_detector_tags: Option<Vec<StringFilter>>,
    #[serde(rename = "codeVulnerabilityFilePath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_vulnerability_file_path: Option<Vec<StringFilter>>,
    #[serde(rename = "componentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_id: Option<Vec<StringFilter>>,
    #[serde(rename = "componentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_type: Option<Vec<StringFilter>>,
    #[serde(rename = "ec2InstanceImageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_instance_image_id: Option<Vec<StringFilter>>,
    #[serde(rename = "ec2InstanceSubnetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_instance_subnet_id: Option<Vec<StringFilter>>,
    #[serde(rename = "ec2InstanceVpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_instance_vpc_id: Option<Vec<StringFilter>>,
    #[serde(rename = "ecrImageArchitecture")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecr_image_architecture: Option<Vec<StringFilter>>,
    #[serde(rename = "ecrImageHash")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecr_image_hash: Option<Vec<StringFilter>>,
    #[serde(rename = "ecrImageInUseCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecr_image_in_use_count: Option<Vec<NumberFilter>>,
    #[serde(rename = "ecrImageLastInUseAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecr_image_last_in_use_at: Option<Vec<DateFilter>>,
    #[serde(rename = "ecrImagePushedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecr_image_pushed_at: Option<Vec<DateFilter>>,
    #[serde(rename = "ecrImageRegistry")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecr_image_registry: Option<Vec<StringFilter>>,
    #[serde(rename = "ecrImageRepositoryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecr_image_repository_name: Option<Vec<StringFilter>>,
    #[serde(rename = "ecrImageTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecr_image_tags: Option<Vec<StringFilter>>,
    #[serde(rename = "epssScore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub epss_score: Option<Vec<NumberFilter>>,
    #[serde(rename = "exploitAvailable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exploit_available: Option<Vec<StringFilter>>,
    #[serde(rename = "findingArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_arn: Option<Vec<StringFilter>>,
    #[serde(rename = "findingStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_status: Option<Vec<StringFilter>>,
    #[serde(rename = "findingType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_type: Option<Vec<StringFilter>>,
    #[serde(rename = "firstObservedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_observed_at: Option<Vec<DateFilter>>,
    #[serde(rename = "fixAvailable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fix_available: Option<Vec<StringFilter>>,
    #[serde(rename = "inspectorScore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inspector_score: Option<Vec<NumberFilter>>,
    #[serde(rename = "lambdaFunctionExecutionRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_function_execution_role_arn: Option<Vec<StringFilter>>,
    #[serde(rename = "lambdaFunctionLastModifiedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_function_last_modified_at: Option<Vec<DateFilter>>,
    #[serde(rename = "lambdaFunctionLayers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_function_layers: Option<Vec<StringFilter>>,
    #[serde(rename = "lambdaFunctionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_function_name: Option<Vec<StringFilter>>,
    #[serde(rename = "lambdaFunctionRuntime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_function_runtime: Option<Vec<StringFilter>>,
    #[serde(rename = "lastObservedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_observed_at: Option<Vec<DateFilter>>,
    #[serde(rename = "networkProtocol")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_protocol: Option<Vec<StringFilter>>,
    #[serde(rename = "portRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_range: Option<Vec<PortRangeFilter>>,
    #[serde(rename = "relatedVulnerabilities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_vulnerabilities: Option<Vec<StringFilter>>,
    #[serde(rename = "resourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<Vec<StringFilter>>,
    #[serde(rename = "resourceTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_tags: Option<Vec<MapFilter>>,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<Vec<StringFilter>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<Vec<StringFilter>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Vec<StringFilter>>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<Vec<DateFilter>>,
    #[serde(rename = "vendorSeverity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_severity: Option<Vec<StringFilter>>,
    #[serde(rename = "vulnerabilityId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vulnerability_id: Option<Vec<StringFilter>>,
    #[serde(rename = "vulnerabilitySource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vulnerability_source: Option<Vec<StringFilter>>,
    #[serde(rename = "vulnerablePackages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vulnerable_packages: Option<Vec<PackageFilter>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StringFilter {
    #[serde(default)]
    pub comparison: String,
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NumberFilter {
    #[serde(rename = "lowerInclusive")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lower_inclusive: Option<f64>,
    #[serde(rename = "upperInclusive")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upper_inclusive: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DateFilter {
    #[serde(rename = "endInclusive")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_inclusive: Option<f64>,
    #[serde(rename = "startInclusive")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_inclusive: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PortRangeFilter {
    #[serde(rename = "beginInclusive")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin_inclusive: Option<i32>,
    #[serde(rename = "endInclusive")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_inclusive: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MapFilter {
    #[serde(default)]
    pub comparison: String,
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PackageFilter {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<StringFilter>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub epoch: Option<NumberFilter>,
    #[serde(rename = "filePath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<StringFilter>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<StringFilter>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release: Option<StringFilter>,
    #[serde(rename = "sourceLambdaLayerArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_lambda_layer_arn: Option<StringFilter>,
    #[serde(rename = "sourceLayerHash")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_layer_hash: Option<StringFilter>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<StringFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFilterResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFindingsReportRequest {
    #[serde(rename = "filterCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_criteria: Option<FilterCriteria>,
    #[serde(rename = "reportFormat")]
    #[serde(default)]
    pub report_format: String,
    #[serde(rename = "s3Destination")]
    #[serde(default)]
    pub s3_destination: Destination,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Destination {
    #[serde(rename = "bucketName")]
    #[serde(default)]
    pub bucket_name: String,
    #[serde(rename = "keyPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_prefix: Option<String>,
    #[serde(rename = "kmsKeyArn")]
    #[serde(default)]
    pub kms_key_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFindingsReportResponse {
    #[serde(rename = "reportId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSbomExportRequest {
    #[serde(rename = "reportFormat")]
    #[serde(default)]
    pub report_format: String,
    #[serde(rename = "resourceFilterCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_filter_criteria: Option<ResourceFilterCriteria>,
    #[serde(rename = "s3Destination")]
    #[serde(default)]
    pub s3_destination: Destination,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceFilterCriteria {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<Vec<ResourceStringFilter>>,
    #[serde(rename = "ec2InstanceTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_instance_tags: Option<Vec<ResourceMapFilter>>,
    #[serde(rename = "ecrImageTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecr_image_tags: Option<Vec<ResourceStringFilter>>,
    #[serde(rename = "ecrRepositoryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecr_repository_name: Option<Vec<ResourceStringFilter>>,
    #[serde(rename = "lambdaFunctionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_function_name: Option<Vec<ResourceStringFilter>>,
    #[serde(rename = "lambdaFunctionTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_function_tags: Option<Vec<ResourceMapFilter>>,
    #[serde(rename = "resourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<Vec<ResourceStringFilter>>,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<Vec<ResourceStringFilter>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceStringFilter {
    #[serde(default)]
    pub comparison: String,
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceMapFilter {
    #[serde(default)]
    pub comparison: String,
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSbomExportResponse {
    #[serde(rename = "reportId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCisScanConfigurationRequest {
    #[serde(rename = "scanConfigurationArn")]
    #[serde(default)]
    pub scan_configuration_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCisScanConfigurationResponse {
    #[serde(rename = "scanConfigurationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_configuration_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCodeSecurityIntegrationRequest {
    #[serde(rename = "integrationArn")]
    #[serde(default)]
    pub integration_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCodeSecurityIntegrationResponse {
    #[serde(rename = "integrationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCodeSecurityScanConfigurationRequest {
    #[serde(rename = "scanConfigurationArn")]
    #[serde(default)]
    pub scan_configuration_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCodeSecurityScanConfigurationResponse {
    #[serde(rename = "scanConfigurationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_configuration_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFilterRequest {
    #[serde(default)]
    pub arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFilterResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeOrganizationConfigurationRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeOrganizationConfigurationResponse {
    #[serde(rename = "autoEnable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_enable: Option<AutoEnable>,
    #[serde(rename = "maxAccountLimitReached")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_account_limit_reached: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutoEnable {
    #[serde(rename = "codeRepository")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_repository: Option<bool>,
    #[serde(default)]
    pub ec2: bool,
    #[serde(default)]
    pub ecr: bool,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda: Option<bool>,
    #[serde(rename = "lambdaCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_code: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisableDelegatedAdminAccountRequest {
    #[serde(rename = "delegatedAdminAccountId")]
    #[serde(default)]
    pub delegated_admin_account_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisableDelegatedAdminAccountResponse {
    #[serde(rename = "delegatedAdminAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegated_admin_account_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisableRequest {
    #[serde(rename = "accountIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    #[serde(rename = "resourceTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_types: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisableResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<Account>>,
    #[serde(rename = "failedAccounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_accounts: Option<Vec<FailedAccount>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Account {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "resourceStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_status: Option<ResourceStatus>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateMemberRequest {
    #[serde(rename = "accountId")]
    #[serde(default)]
    pub account_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateMemberResponse {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnableDelegatedAdminAccountRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "delegatedAdminAccountId")]
    #[serde(default)]
    pub delegated_admin_account_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnableDelegatedAdminAccountResponse {
    #[serde(rename = "delegatedAdminAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegated_admin_account_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnableRequest {
    #[serde(rename = "accountIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "resourceTypes")]
    #[serde(default)]
    pub resource_types: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnableResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<Account>>,
    #[serde(rename = "failedAccounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_accounts: Option<Vec<FailedAccount>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCisScanReportRequest {
    #[serde(rename = "reportFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_format: Option<String>,
    #[serde(rename = "scanArn")]
    #[serde(default)]
    pub scan_arn: String,
    #[serde(rename = "targetAccounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_accounts: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCisScanReportResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCisScanResultDetailsRequest {
    #[serde(rename = "accountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "filterCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_criteria: Option<CisScanResultDetailsFilterCriteria>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "scanArn")]
    #[serde(default)]
    pub scan_arn: String,
    #[serde(rename = "sortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(rename = "sortOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    #[serde(rename = "targetResourceId")]
    #[serde(default)]
    pub target_resource_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CisScanResultDetailsFilterCriteria {
    #[serde(rename = "checkIdFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_id_filters: Option<Vec<CisStringFilter>>,
    #[serde(rename = "findingArnFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_arn_filters: Option<Vec<CisStringFilter>>,
    #[serde(rename = "findingStatusFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_status_filters: Option<Vec<CisFindingStatusFilter>>,
    #[serde(rename = "securityLevelFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_level_filters: Option<Vec<CisSecurityLevelFilter>>,
    #[serde(rename = "titleFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_filters: Option<Vec<CisStringFilter>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CisStringFilter {
    #[serde(default)]
    pub comparison: String,
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CisFindingStatusFilter {
    #[serde(default)]
    pub comparison: String,
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CisSecurityLevelFilter {
    #[serde(default)]
    pub comparison: String,
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCisScanResultDetailsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "scanResultDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_result_details: Option<Vec<CisScanResultDetails>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CisScanResultDetails {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "checkDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_description: Option<String>,
    #[serde(rename = "checkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_id: Option<String>,
    #[serde(rename = "findingArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remediation: Option<String>,
    #[serde(rename = "scanArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    #[serde(rename = "targetResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_resource_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetClustersForImageRequest {
    #[serde(default)]
    pub filter: ClusterForImageFilterCriteria,
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
pub struct ClusterForImageFilterCriteria {
    #[serde(rename = "resourceId")]
    #[serde(default)]
    pub resource_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetClustersForImageResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Vec<ClusterInformation>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClusterInformation {
    #[serde(rename = "clusterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    #[serde(rename = "clusterDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_details: Option<Vec<ClusterDetails>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClusterDetails {
    #[serde(rename = "clusterMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_metadata: Option<ClusterMetadata>,
    #[serde(rename = "lastInUse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_in_use: Option<f64>,
    #[serde(rename = "runningUnitCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running_unit_count: Option<i64>,
    #[serde(rename = "stoppedUnitCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopped_unit_count: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClusterMetadata {
    #[serde(rename = "awsEcsMetadataDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_ecs_metadata_details: Option<AwsEcsMetadataDetails>,
    #[serde(rename = "awsEksMetadataDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_eks_metadata_details: Option<AwsEksMetadataDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEcsMetadataDetails {
    #[serde(rename = "detailsGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details_group: Option<String>,
    #[serde(rename = "taskDefinitionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_definition_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEksMetadataDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "workloadInfoList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workload_info_list: Option<Vec<AwsEksWorkloadInfo>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEksWorkloadInfo {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCodeSecurityIntegrationRequest {
    #[serde(rename = "integrationArn")]
    #[serde(default)]
    pub integration_arn: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCodeSecurityIntegrationResponse {
    #[serde(rename = "authorizationUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_url: Option<String>,
    #[serde(rename = "createdOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_on: Option<f64>,
    #[serde(rename = "integrationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_arn: Option<String>,
    #[serde(rename = "lastUpdateOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_on: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCodeSecurityScanConfigurationRequest {
    #[serde(rename = "scanConfigurationArn")]
    #[serde(default)]
    pub scan_configuration_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCodeSecurityScanConfigurationResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<CodeSecurityScanConfiguration>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "scanConfigurationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_configuration_arn: Option<String>,
    #[serde(rename = "scopeSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_settings: Option<ScopeSettings>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCodeSecurityScanRequest {
    #[serde(default)]
    pub resource: CodeSecurityResource,
    #[serde(rename = "scanId")]
    #[serde(default)]
    pub scan_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCodeSecurityScanResponse {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "lastCommitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_commit_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<CodeSecurityResource>,
    #[serde(rename = "scanId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetConfigurationRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetConfigurationResponse {
    #[serde(rename = "ec2Configuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_configuration: Option<Ec2ConfigurationState>,
    #[serde(rename = "ecrConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecr_configuration: Option<EcrConfigurationState>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Ec2ConfigurationState {
    #[serde(rename = "scanModeState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_mode_state: Option<Ec2ScanModeState>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Ec2ScanModeState {
    #[serde(rename = "scanMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_mode: Option<String>,
    #[serde(rename = "scanModeStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_mode_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EcrConfigurationState {
    #[serde(rename = "rescanDurationState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rescan_duration_state: Option<EcrRescanDurationState>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EcrRescanDurationState {
    #[serde(rename = "pullDateRescanDuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_date_rescan_duration: Option<String>,
    #[serde(rename = "pullDateRescanMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_date_rescan_mode: Option<String>,
    #[serde(rename = "rescanDuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rescan_duration: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDelegatedAdminAccountRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDelegatedAdminAccountResponse {
    #[serde(rename = "delegatedAdmin")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegated_admin: Option<DelegatedAdmin>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DelegatedAdmin {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "relationshipStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetEc2DeepInspectionConfigurationRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetEc2DeepInspectionConfigurationResponse {
    #[serde(rename = "errorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "orgPackagePaths")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_package_paths: Option<Vec<String>>,
    #[serde(rename = "packagePaths")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_paths: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetEncryptionKeyRequest {
    #[serde(rename = "resourceType")]
    #[serde(default)]
    pub resource_type: String,
    #[serde(rename = "scanType")]
    #[serde(default)]
    pub scan_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetEncryptionKeyResponse {
    #[serde(rename = "kmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFindingsReportStatusRequest {
    #[serde(rename = "reportId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFindingsReportStatusResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<Destination>,
    #[serde(rename = "errorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "errorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "filterCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_criteria: Option<FilterCriteria>,
    #[serde(rename = "reportId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMemberRequest {
    #[serde(rename = "accountId")]
    #[serde(default)]
    pub account_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMemberResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<Member>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Member {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "delegatedAdminAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegated_admin_account_id: Option<String>,
    #[serde(rename = "relationshipStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship_status: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSbomExportRequest {
    #[serde(rename = "reportId")]
    #[serde(default)]
    pub report_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSbomExportResponse {
    #[serde(rename = "errorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "errorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "filterCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_criteria: Option<ResourceFilterCriteria>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(rename = "reportId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_id: Option<String>,
    #[serde(rename = "s3Destination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_destination: Option<Destination>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAccountPermissionsRequest {
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
    pub service: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAccountPermissionsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<Permission>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Permission {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCisScanConfigurationsRequest {
    #[serde(rename = "filterCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_criteria: Option<ListCisScanConfigurationsFilterCriteria>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "sortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(rename = "sortOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCisScanConfigurationsFilterCriteria {
    #[serde(rename = "scanConfigurationArnFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_configuration_arn_filters: Option<Vec<CisStringFilter>>,
    #[serde(rename = "scanNameFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_name_filters: Option<Vec<CisStringFilter>>,
    #[serde(rename = "targetResourceTagFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_resource_tag_filters: Option<Vec<TagFilter>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagFilter {
    #[serde(default)]
    pub comparison: String,
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCisScanConfigurationsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "scanConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_configurations: Option<Vec<CisScanConfiguration>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CisScanConfiguration {
    #[serde(rename = "ownerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(rename = "scanConfigurationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_configuration_arn: Option<String>,
    #[serde(rename = "scanName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Schedule>,
    #[serde(rename = "securityLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_level: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<CisTargets>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CisTargets {
    #[serde(rename = "accountIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    #[serde(rename = "targetResourceTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_resource_tags: Option<std::collections::HashMap<String, Vec<String>>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCisScanResultsAggregatedByChecksRequest {
    #[serde(rename = "filterCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_criteria: Option<CisScanResultsAggregatedByChecksFilterCriteria>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "scanArn")]
    #[serde(default)]
    pub scan_arn: String,
    #[serde(rename = "sortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(rename = "sortOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CisScanResultsAggregatedByChecksFilterCriteria {
    #[serde(rename = "accountIdFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id_filters: Option<Vec<CisStringFilter>>,
    #[serde(rename = "checkIdFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_id_filters: Option<Vec<CisStringFilter>>,
    #[serde(rename = "failedResourcesFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_resources_filters: Option<Vec<CisNumberFilter>>,
    #[serde(rename = "platformFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_filters: Option<Vec<CisStringFilter>>,
    #[serde(rename = "securityLevelFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_level_filters: Option<Vec<CisSecurityLevelFilter>>,
    #[serde(rename = "titleFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_filters: Option<Vec<CisStringFilter>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CisNumberFilter {
    #[serde(rename = "lowerInclusive")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lower_inclusive: Option<i32>,
    #[serde(rename = "upperInclusive")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upper_inclusive: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCisScanResultsAggregatedByChecksResponse {
    #[serde(rename = "checkAggregations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_aggregations: Option<Vec<CisCheckAggregation>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CisCheckAggregation {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "checkDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_description: Option<String>,
    #[serde(rename = "checkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(rename = "scanArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_arn: Option<String>,
    #[serde(rename = "statusCounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_counts: Option<StatusCounts>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StatusCounts {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passed: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skipped: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCisScanResultsAggregatedByTargetResourceRequest {
    #[serde(rename = "filterCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_criteria: Option<CisScanResultsAggregatedByTargetResourceFilterCriteria>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "scanArn")]
    #[serde(default)]
    pub scan_arn: String,
    #[serde(rename = "sortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(rename = "sortOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CisScanResultsAggregatedByTargetResourceFilterCriteria {
    #[serde(rename = "accountIdFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id_filters: Option<Vec<CisStringFilter>>,
    #[serde(rename = "checkIdFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_id_filters: Option<Vec<CisStringFilter>>,
    #[serde(rename = "failedChecksFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_checks_filters: Option<Vec<CisNumberFilter>>,
    #[serde(rename = "platformFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_filters: Option<Vec<CisStringFilter>>,
    #[serde(rename = "statusFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_filters: Option<Vec<CisResultStatusFilter>>,
    #[serde(rename = "targetResourceIdFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_resource_id_filters: Option<Vec<CisStringFilter>>,
    #[serde(rename = "targetResourceTagFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_resource_tag_filters: Option<Vec<TagFilter>>,
    #[serde(rename = "targetStatusFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_status_filters: Option<Vec<CisTargetStatusFilter>>,
    #[serde(rename = "targetStatusReasonFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_status_reason_filters: Option<Vec<CisTargetStatusReasonFilter>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CisResultStatusFilter {
    #[serde(default)]
    pub comparison: String,
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CisTargetStatusFilter {
    #[serde(default)]
    pub comparison: String,
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CisTargetStatusReasonFilter {
    #[serde(default)]
    pub comparison: String,
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCisScanResultsAggregatedByTargetResourceResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "targetResourceAggregations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_resource_aggregations: Option<Vec<CisTargetResourceAggregation>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CisTargetResourceAggregation {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(rename = "scanArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_arn: Option<String>,
    #[serde(rename = "statusCounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_counts: Option<StatusCounts>,
    #[serde(rename = "targetResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_resource_id: Option<String>,
    #[serde(rename = "targetResourceTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_resource_tags: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "targetStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_status: Option<String>,
    #[serde(rename = "targetStatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_status_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCisScansRequest {
    #[serde(rename = "detailLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail_level: Option<String>,
    #[serde(rename = "filterCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_criteria: Option<ListCisScansFilterCriteria>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "sortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(rename = "sortOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCisScansFilterCriteria {
    #[serde(rename = "failedChecksFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_checks_filters: Option<Vec<CisNumberFilter>>,
    #[serde(rename = "scanArnFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_arn_filters: Option<Vec<CisStringFilter>>,
    #[serde(rename = "scanAtFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_at_filters: Option<Vec<CisDateFilter>>,
    #[serde(rename = "scanConfigurationArnFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_configuration_arn_filters: Option<Vec<CisStringFilter>>,
    #[serde(rename = "scanNameFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_name_filters: Option<Vec<CisStringFilter>>,
    #[serde(rename = "scanStatusFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_status_filters: Option<Vec<CisScanStatusFilter>>,
    #[serde(rename = "scheduledByFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_by_filters: Option<Vec<CisStringFilter>>,
    #[serde(rename = "targetAccountIdFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_account_id_filters: Option<Vec<CisStringFilter>>,
    #[serde(rename = "targetResourceIdFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_resource_id_filters: Option<Vec<CisStringFilter>>,
    #[serde(rename = "targetResourceTagFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_resource_tag_filters: Option<Vec<TagFilter>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CisDateFilter {
    #[serde(rename = "earliestScanStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub earliest_scan_start_time: Option<f64>,
    #[serde(rename = "latestScanStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_scan_start_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CisScanStatusFilter {
    #[serde(default)]
    pub comparison: String,
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCisScansResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scans: Option<Vec<CisScan>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CisScan {
    #[serde(rename = "failedChecks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_checks: Option<i32>,
    #[serde(rename = "scanArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_arn: Option<String>,
    #[serde(rename = "scanConfigurationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_configuration_arn: Option<String>,
    #[serde(rename = "scanDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_date: Option<f64>,
    #[serde(rename = "scanName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_name: Option<String>,
    #[serde(rename = "scheduledBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_by: Option<String>,
    #[serde(rename = "securityLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_level: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<CisTargets>,
    #[serde(rename = "totalChecks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_checks: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCodeSecurityIntegrationsRequest {
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
pub struct ListCodeSecurityIntegrationsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integrations: Option<Vec<CodeSecurityIntegrationSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CodeSecurityIntegrationSummary {
    #[serde(rename = "createdOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_on: Option<f64>,
    #[serde(rename = "integrationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_arn: Option<String>,
    #[serde(rename = "lastUpdateOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_on: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCodeSecurityScanConfigurationAssociationsRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "scanConfigurationArn")]
    #[serde(default)]
    pub scan_configuration_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCodeSecurityScanConfigurationAssociationsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associations: Option<Vec<CodeSecurityScanConfigurationAssociationSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CodeSecurityScanConfigurationAssociationSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<CodeSecurityResource>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCodeSecurityScanConfigurationsRequest {
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
pub struct ListCodeSecurityScanConfigurationsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configurations: Option<Vec<CodeSecurityScanConfigurationSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CodeSecurityScanConfigurationSummary {
    #[serde(rename = "continuousIntegrationScanSupportedEvents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuous_integration_scan_supported_events: Option<Vec<String>>,
    #[serde(rename = "frequencyExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_expression: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ownerAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_account_id: Option<String>,
    #[serde(rename = "periodicScanFrequency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub periodic_scan_frequency: Option<String>,
    #[serde(rename = "ruleSetCategories")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_set_categories: Option<Vec<String>>,
    #[serde(rename = "scanConfigurationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_configuration_arn: Option<String>,
    #[serde(rename = "scopeSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_settings: Option<ScopeSettings>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCoverageRequest {
    #[serde(rename = "filterCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_criteria: Option<CoverageFilterCriteria>,
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
pub struct CoverageFilterCriteria {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<Vec<CoverageStringFilter>>,
    #[serde(rename = "codeRepositoryProjectName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_repository_project_name: Option<Vec<CoverageStringFilter>>,
    #[serde(rename = "codeRepositoryProviderType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_repository_provider_type: Option<Vec<CoverageStringFilter>>,
    #[serde(rename = "codeRepositoryProviderTypeVisibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_repository_provider_type_visibility: Option<Vec<CoverageStringFilter>>,
    #[serde(rename = "ec2InstanceTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_instance_tags: Option<Vec<CoverageMapFilter>>,
    #[serde(rename = "ecrImageInUseCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecr_image_in_use_count: Option<Vec<CoverageNumberFilter>>,
    #[serde(rename = "ecrImageLastInUseAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecr_image_last_in_use_at: Option<Vec<CoverageDateFilter>>,
    #[serde(rename = "ecrImageTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecr_image_tags: Option<Vec<CoverageStringFilter>>,
    #[serde(rename = "ecrRepositoryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecr_repository_name: Option<Vec<CoverageStringFilter>>,
    #[serde(rename = "imagePulledAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_pulled_at: Option<Vec<CoverageDateFilter>>,
    #[serde(rename = "lambdaFunctionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_function_name: Option<Vec<CoverageStringFilter>>,
    #[serde(rename = "lambdaFunctionRuntime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_function_runtime: Option<Vec<CoverageStringFilter>>,
    #[serde(rename = "lambdaFunctionTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_function_tags: Option<Vec<CoverageMapFilter>>,
    #[serde(rename = "lastScannedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_scanned_at: Option<Vec<CoverageDateFilter>>,
    #[serde(rename = "lastScannedCommitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_scanned_commit_id: Option<Vec<CoverageStringFilter>>,
    #[serde(rename = "resourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<Vec<CoverageStringFilter>>,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<Vec<CoverageStringFilter>>,
    #[serde(rename = "scanMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_mode: Option<Vec<CoverageStringFilter>>,
    #[serde(rename = "scanStatusCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_status_code: Option<Vec<CoverageStringFilter>>,
    #[serde(rename = "scanStatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_status_reason: Option<Vec<CoverageStringFilter>>,
    #[serde(rename = "scanType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_type: Option<Vec<CoverageStringFilter>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CoverageStringFilter {
    #[serde(default)]
    pub comparison: String,
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CoverageMapFilter {
    #[serde(default)]
    pub comparison: String,
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CoverageNumberFilter {
    #[serde(rename = "lowerInclusive")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lower_inclusive: Option<i64>,
    #[serde(rename = "upperInclusive")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upper_inclusive: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CoverageDateFilter {
    #[serde(rename = "endInclusive")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_inclusive: Option<f64>,
    #[serde(rename = "startInclusive")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_inclusive: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCoverageResponse {
    #[serde(rename = "coveredResources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub covered_resources: Option<Vec<CoveredResource>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CoveredResource {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "lastScannedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_scanned_at: Option<f64>,
    #[serde(rename = "resourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "resourceMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_metadata: Option<ResourceScanMetadata>,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "scanMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_mode: Option<String>,
    #[serde(rename = "scanStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_status: Option<ScanStatus>,
    #[serde(rename = "scanType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceScanMetadata {
    #[serde(rename = "codeRepository")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_repository: Option<CodeRepositoryMetadata>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2: Option<Ec2Metadata>,
    #[serde(rename = "ecrImage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecr_image: Option<EcrContainerImageMetadata>,
    #[serde(rename = "ecrRepository")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecr_repository: Option<EcrRepositoryMetadata>,
    #[serde(rename = "lambdaFunction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_function: Option<LambdaFunctionMetadata>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CodeRepositoryMetadata {
    #[serde(rename = "integrationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_arn: Option<String>,
    #[serde(rename = "lastScannedCommitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_scanned_commit_id: Option<String>,
    #[serde(rename = "onDemandScan")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_demand_scan: Option<CodeRepositoryOnDemandScan>,
    #[serde(rename = "projectName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_name: Option<String>,
    #[serde(rename = "providerType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_type: Option<String>,
    #[serde(rename = "providerTypeVisibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_type_visibility: Option<String>,
    #[serde(rename = "scanConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_configuration: Option<ProjectCodeSecurityScanConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CodeRepositoryOnDemandScan {
    #[serde(rename = "lastScanAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_scan_at: Option<f64>,
    #[serde(rename = "lastScannedCommitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_scanned_commit_id: Option<String>,
    #[serde(rename = "scanStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_status: Option<ScanStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScanStatus {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "statusCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProjectCodeSecurityScanConfiguration {
    #[serde(rename = "continuousIntegrationScanConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuous_integration_scan_configurations:
        Option<Vec<ProjectContinuousIntegrationScanConfiguration>>,
    #[serde(rename = "periodicScanConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub periodic_scan_configurations: Option<Vec<ProjectPeriodicScanConfiguration>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProjectContinuousIntegrationScanConfiguration {
    #[serde(rename = "ruleSetCategories")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_set_categories: Option<Vec<String>>,
    #[serde(rename = "supportedEvent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_event: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProjectPeriodicScanConfiguration {
    #[serde(rename = "frequencyExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_expression: Option<String>,
    #[serde(rename = "ruleSetCategories")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_set_categories: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Ec2Metadata {
    #[serde(rename = "amiId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ami_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EcrContainerImageMetadata {
    #[serde(rename = "imagePulledAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_pulled_at: Option<f64>,
    #[serde(rename = "inUseCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_use_count: Option<i64>,
    #[serde(rename = "lastInUseAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_in_use_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EcrRepositoryMetadata {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "scanFrequency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_frequency: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LambdaFunctionMetadata {
    #[serde(rename = "functionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_name: Option<String>,
    #[serde(rename = "functionTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_tags: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layers: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCoverageStatisticsRequest {
    #[serde(rename = "filterCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_criteria: Option<CoverageFilterCriteria>,
    #[serde(rename = "groupBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCoverageStatisticsResponse {
    #[serde(rename = "countsByGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counts_by_group: Option<Vec<Counts>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "totalCounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_counts: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Counts {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(rename = "groupKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_key: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDelegatedAdminAccountsRequest {
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
pub struct ListDelegatedAdminAccountsResponse {
    #[serde(rename = "delegatedAdminAccounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegated_admin_accounts: Option<Vec<DelegatedAdminAccount>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DelegatedAdminAccount {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFiltersRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arns: Option<Vec<String>>,
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
pub struct ListFiltersResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Filter {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub criteria: Option<FilterCriteria>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ownerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFindingAggregationsRequest {
    #[serde(rename = "accountIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<StringFilter>>,
    #[serde(rename = "aggregationRequest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_request: Option<AggregationRequest>,
    #[serde(rename = "aggregationType")]
    #[serde(default)]
    pub aggregation_type: String,
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
pub struct AggregationRequest {
    #[serde(rename = "accountAggregation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_aggregation: Option<AccountAggregation>,
    #[serde(rename = "amiAggregation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ami_aggregation: Option<AmiAggregation>,
    #[serde(rename = "awsEcrContainerAggregation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_ecr_container_aggregation: Option<AwsEcrContainerAggregation>,
    #[serde(rename = "codeRepositoryAggregation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_repository_aggregation: Option<CodeRepositoryAggregation>,
    #[serde(rename = "ec2InstanceAggregation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_instance_aggregation: Option<Ec2InstanceAggregation>,
    #[serde(rename = "findingTypeAggregation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_type_aggregation: Option<FindingTypeAggregation>,
    #[serde(rename = "imageLayerAggregation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_layer_aggregation: Option<ImageLayerAggregation>,
    #[serde(rename = "lambdaFunctionAggregation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_function_aggregation: Option<LambdaFunctionAggregation>,
    #[serde(rename = "lambdaLayerAggregation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_layer_aggregation: Option<LambdaLayerAggregation>,
    #[serde(rename = "packageAggregation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_aggregation: Option<PackageAggregation>,
    #[serde(rename = "repositoryAggregation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_aggregation: Option<RepositoryAggregation>,
    #[serde(rename = "titleAggregation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_aggregation: Option<TitleAggregation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccountAggregation {
    #[serde(rename = "findingType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_type: Option<String>,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "sortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(rename = "sortOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AmiAggregation {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amis: Option<Vec<StringFilter>>,
    #[serde(rename = "sortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(rename = "sortOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEcrContainerAggregation {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architectures: Option<Vec<StringFilter>>,
    #[serde(rename = "imageShas")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_shas: Option<Vec<StringFilter>>,
    #[serde(rename = "imageTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_tags: Option<Vec<StringFilter>>,
    #[serde(rename = "inUseCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_use_count: Option<Vec<NumberFilter>>,
    #[serde(rename = "lastInUseAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_in_use_at: Option<Vec<DateFilter>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repositories: Option<Vec<StringFilter>>,
    #[serde(rename = "resourceIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_ids: Option<Vec<StringFilter>>,
    #[serde(rename = "sortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(rename = "sortOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CodeRepositoryAggregation {
    #[serde(rename = "projectNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_names: Option<Vec<StringFilter>>,
    #[serde(rename = "providerTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_types: Option<Vec<StringFilter>>,
    #[serde(rename = "resourceIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_ids: Option<Vec<StringFilter>>,
    #[serde(rename = "sortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(rename = "sortOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Ec2InstanceAggregation {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amis: Option<Vec<StringFilter>>,
    #[serde(rename = "instanceIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_ids: Option<Vec<StringFilter>>,
    #[serde(rename = "instanceTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_tags: Option<Vec<MapFilter>>,
    #[serde(rename = "operatingSystems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_systems: Option<Vec<StringFilter>>,
    #[serde(rename = "sortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(rename = "sortOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FindingTypeAggregation {
    #[serde(rename = "findingType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_type: Option<String>,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "sortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(rename = "sortOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImageLayerAggregation {
    #[serde(rename = "layerHashes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer_hashes: Option<Vec<StringFilter>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repositories: Option<Vec<StringFilter>>,
    #[serde(rename = "resourceIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_ids: Option<Vec<StringFilter>>,
    #[serde(rename = "sortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(rename = "sortOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LambdaFunctionAggregation {
    #[serde(rename = "functionNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_names: Option<Vec<StringFilter>>,
    #[serde(rename = "functionTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_tags: Option<Vec<MapFilter>>,
    #[serde(rename = "resourceIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_ids: Option<Vec<StringFilter>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtimes: Option<Vec<StringFilter>>,
    #[serde(rename = "sortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(rename = "sortOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LambdaLayerAggregation {
    #[serde(rename = "functionNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_names: Option<Vec<StringFilter>>,
    #[serde(rename = "layerArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer_arns: Option<Vec<StringFilter>>,
    #[serde(rename = "resourceIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_ids: Option<Vec<StringFilter>>,
    #[serde(rename = "sortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(rename = "sortOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PackageAggregation {
    #[serde(rename = "packageNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_names: Option<Vec<StringFilter>>,
    #[serde(rename = "sortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(rename = "sortOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RepositoryAggregation {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repositories: Option<Vec<StringFilter>>,
    #[serde(rename = "sortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(rename = "sortOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TitleAggregation {
    #[serde(rename = "findingType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_type: Option<String>,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "sortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(rename = "sortOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub titles: Option<Vec<StringFilter>>,
    #[serde(rename = "vulnerabilityIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vulnerability_ids: Option<Vec<StringFilter>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFindingAggregationsResponse {
    #[serde(rename = "aggregationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_type: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responses: Option<Vec<AggregationResponse>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AggregationResponse {
    #[serde(rename = "accountAggregation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_aggregation: Option<AccountAggregationResponse>,
    #[serde(rename = "amiAggregation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ami_aggregation: Option<AmiAggregationResponse>,
    #[serde(rename = "awsEcrContainerAggregation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_ecr_container_aggregation: Option<AwsEcrContainerAggregationResponse>,
    #[serde(rename = "codeRepositoryAggregation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_repository_aggregation: Option<CodeRepositoryAggregationResponse>,
    #[serde(rename = "ec2InstanceAggregation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_instance_aggregation: Option<Ec2InstanceAggregationResponse>,
    #[serde(rename = "findingTypeAggregation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_type_aggregation: Option<FindingTypeAggregationResponse>,
    #[serde(rename = "imageLayerAggregation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_layer_aggregation: Option<ImageLayerAggregationResponse>,
    #[serde(rename = "lambdaFunctionAggregation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_function_aggregation: Option<LambdaFunctionAggregationResponse>,
    #[serde(rename = "lambdaLayerAggregation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_layer_aggregation: Option<LambdaLayerAggregationResponse>,
    #[serde(rename = "packageAggregation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_aggregation: Option<PackageAggregationResponse>,
    #[serde(rename = "repositoryAggregation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_aggregation: Option<RepositoryAggregationResponse>,
    #[serde(rename = "titleAggregation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_aggregation: Option<TitleAggregationResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccountAggregationResponse {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "exploitAvailableCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exploit_available_count: Option<i64>,
    #[serde(rename = "fixAvailableCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fix_available_count: Option<i64>,
    #[serde(rename = "severityCounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity_counts: Option<SeverityCounts>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SeverityCounts {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub critical: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medium: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AmiAggregationResponse {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "affectedInstances")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affected_instances: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ami: Option<String>,
    #[serde(rename = "severityCounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity_counts: Option<SeverityCounts>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEcrContainerAggregationResponse {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    #[serde(rename = "imageSha")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_sha: Option<String>,
    #[serde(rename = "imageTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_tags: Option<Vec<String>>,
    #[serde(rename = "inUseCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_use_count: Option<i64>,
    #[serde(rename = "lastInUseAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_in_use_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
    #[serde(rename = "resourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "severityCounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity_counts: Option<SeverityCounts>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CodeRepositoryAggregationResponse {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "exploitAvailableActiveFindingsCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exploit_available_active_findings_count: Option<i64>,
    #[serde(rename = "fixAvailableActiveFindingsCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fix_available_active_findings_count: Option<i64>,
    #[serde(rename = "projectNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_names: Option<String>,
    #[serde(rename = "providerType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_type: Option<String>,
    #[serde(rename = "resourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "severityCounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity_counts: Option<SeverityCounts>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Ec2InstanceAggregationResponse {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ami: Option<String>,
    #[serde(rename = "instanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(rename = "instanceTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "networkFindings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_findings: Option<i64>,
    #[serde(rename = "operatingSystem")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<String>,
    #[serde(rename = "severityCounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity_counts: Option<SeverityCounts>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FindingTypeAggregationResponse {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "exploitAvailableCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exploit_available_count: Option<i64>,
    #[serde(rename = "fixAvailableCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fix_available_count: Option<i64>,
    #[serde(rename = "severityCounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity_counts: Option<SeverityCounts>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImageLayerAggregationResponse {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "layerHash")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer_hash: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
    #[serde(rename = "resourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "severityCounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity_counts: Option<SeverityCounts>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LambdaFunctionAggregationResponse {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "functionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_name: Option<String>,
    #[serde(rename = "lambdaTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "lastModifiedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_at: Option<f64>,
    #[serde(rename = "resourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime: Option<String>,
    #[serde(rename = "severityCounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity_counts: Option<SeverityCounts>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LambdaLayerAggregationResponse {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "functionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_name: Option<String>,
    #[serde(rename = "layerArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer_arn: Option<String>,
    #[serde(rename = "resourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "severityCounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity_counts: Option<SeverityCounts>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PackageAggregationResponse {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "packageName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_name: Option<String>,
    #[serde(rename = "severityCounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity_counts: Option<SeverityCounts>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RepositoryAggregationResponse {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "affectedImages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affected_images: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
    #[serde(rename = "severityCounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity_counts: Option<SeverityCounts>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TitleAggregationResponse {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "severityCounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity_counts: Option<SeverityCounts>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "vulnerabilityId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vulnerability_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFindingsRequest {
    #[serde(rename = "filterCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_criteria: Option<FilterCriteria>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "sortCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_criteria: Option<SortCriteria>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SortCriteria {
    #[serde(default)]
    pub field: String,
    #[serde(rename = "sortOrder")]
    #[serde(default)]
    pub sort_order: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFindingsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub findings: Option<Vec<Finding>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Finding {
    #[serde(rename = "awsAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_account_id: Option<String>,
    #[serde(rename = "codeVulnerabilityDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_vulnerability_details: Option<CodeVulnerabilityDetails>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub epss: Option<EpssDetails>,
    #[serde(rename = "exploitAvailable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exploit_available: Option<String>,
    #[serde(rename = "exploitabilityDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exploitability_details: Option<ExploitabilityDetails>,
    #[serde(rename = "findingArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_arn: Option<String>,
    #[serde(rename = "firstObservedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_observed_at: Option<f64>,
    #[serde(rename = "fixAvailable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fix_available: Option<String>,
    #[serde(rename = "inspectorScore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inspector_score: Option<f64>,
    #[serde(rename = "inspectorScoreDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inspector_score_details: Option<InspectorScoreDetails>,
    #[serde(rename = "lastObservedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_observed_at: Option<f64>,
    #[serde(rename = "networkReachabilityDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_reachability_details: Option<NetworkReachabilityDetails>,
    #[serde(rename = "packageVulnerabilityDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_vulnerability_details: Option<PackageVulnerabilityDetails>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remediation: Option<Remediation>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<Resource>>,
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
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CodeVulnerabilityDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cwes: Option<Vec<String>>,
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
    #[serde(rename = "filePath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<CodeFilePath>,
    #[serde(rename = "referenceUrls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_urls: Option<Vec<String>>,
    #[serde(rename = "ruleId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
    #[serde(rename = "sourceLambdaLayerArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_lambda_layer_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CodeFilePath {
    #[serde(rename = "endLine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_line: Option<i32>,
    #[serde(rename = "fileName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(rename = "filePath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    #[serde(rename = "startLine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_line: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EpssDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExploitabilityDetails {
    #[serde(rename = "lastKnownExploitAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_known_exploit_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InspectorScoreDetails {
    #[serde(rename = "adjustedCvss")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjusted_cvss: Option<CvssScoreDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CvssScoreDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjustments: Option<Vec<CvssScoreAdjustment>>,
    #[serde(rename = "cvssSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvss_source: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
    #[serde(rename = "scoreSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_source: Option<String>,
    #[serde(rename = "scoringVector")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scoring_vector: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CvssScoreAdjustment {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NetworkReachabilityDetails {
    #[serde(rename = "networkPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_path: Option<NetworkPath>,
    #[serde(rename = "openPortRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_port_range: Option<PortRange>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NetworkPath {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<Vec<Step>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Step {
    #[serde(rename = "componentArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_arn: Option<String>,
    #[serde(rename = "componentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_id: Option<String>,
    #[serde(rename = "componentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PortRange {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PackageVulnerabilityDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvss: Option<Vec<CvssScore>>,
    #[serde(rename = "referenceUrls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_urls: Option<Vec<String>>,
    #[serde(rename = "relatedVulnerabilities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_vulnerabilities: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "sourceUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<String>,
    #[serde(rename = "vendorCreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_created_at: Option<f64>,
    #[serde(rename = "vendorSeverity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_severity: Option<String>,
    #[serde(rename = "vendorUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_updated_at: Option<f64>,
    #[serde(rename = "vulnerabilityId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vulnerability_id: Option<String>,
    #[serde(rename = "vulnerablePackages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vulnerable_packages: Option<Vec<VulnerablePackage>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CvssScore {
    #[serde(rename = "baseScore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_score: Option<f64>,
    #[serde(rename = "scoringVector")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scoring_vector: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VulnerablePackage {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arch: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub epoch: Option<i32>,
    #[serde(rename = "filePath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    #[serde(rename = "fixedInVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_in_version: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "packageManager")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_manager: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remediation: Option<String>,
    #[serde(rename = "sourceLambdaLayerArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_lambda_layer_arn: Option<String>,
    #[serde(rename = "sourceLayerHash")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_layer_hash: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Remediation {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation: Option<Recommendation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Recommendation {
    #[serde(rename = "Url")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Resource {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<ResourceDetails>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceDetails {
    #[serde(rename = "awsEc2Instance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_ec2_instance: Option<AwsEc2InstanceDetails>,
    #[serde(rename = "awsEcrContainerImage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_ecr_container_image: Option<AwsEcrContainerImageDetails>,
    #[serde(rename = "awsLambdaFunction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_lambda_function: Option<AwsLambdaFunctionDetails>,
    #[serde(rename = "codeRepository")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_repository: Option<CodeRepositoryDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2InstanceDetails {
    #[serde(rename = "iamInstanceProfileArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_instance_profile_arn: Option<String>,
    #[serde(rename = "imageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    #[serde(rename = "ipV4Addresses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_v4_addresses: Option<Vec<String>>,
    #[serde(rename = "ipV6Addresses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_v6_addresses: Option<Vec<String>>,
    #[serde(rename = "keyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_name: Option<String>,
    #[serde(rename = "launchedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launched_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(rename = "subnetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "vpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEcrContainerImageDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(rename = "imageHash")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_hash: Option<String>,
    #[serde(rename = "imageTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_tags: Option<Vec<String>>,
    #[serde(rename = "inUseCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_use_count: Option<i64>,
    #[serde(rename = "lastInUseAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_in_use_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(rename = "pushedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pushed_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsLambdaFunctionDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architectures: Option<Vec<String>>,
    #[serde(rename = "codeSha256")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_sha256: Option<String>,
    #[serde(rename = "executionRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role_arn: Option<String>,
    #[serde(rename = "functionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_name: Option<String>,
    #[serde(rename = "lastModifiedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layers: Option<Vec<String>>,
    #[serde(rename = "packageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "vpcConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<LambdaVpcConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LambdaVpcConfig {
    #[serde(rename = "securityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(rename = "subnetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    #[serde(rename = "vpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CodeRepositoryDetails {
    #[serde(rename = "integrationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_arn: Option<String>,
    #[serde(rename = "projectName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_name: Option<String>,
    #[serde(rename = "providerType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListMembersRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "onlyAssociated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_associated: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListMembersResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<Member>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
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
pub struct ListUsageTotalsRequest {
    #[serde(rename = "accountIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
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
pub struct ListUsageTotalsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub totals: Option<Vec<UsageTotal>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UsageTotal {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<Vec<Usage>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Usage {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "estimatedMonthlyCost")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_monthly_cost: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<f64>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResetEncryptionKeyRequest {
    #[serde(rename = "resourceType")]
    #[serde(default)]
    pub resource_type: String,
    #[serde(rename = "scanType")]
    #[serde(default)]
    pub scan_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResetEncryptionKeyResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchVulnerabilitiesRequest {
    #[serde(rename = "filterCriteria")]
    #[serde(default)]
    pub filter_criteria: SearchVulnerabilitiesFilterCriteria,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchVulnerabilitiesFilterCriteria {
    #[serde(rename = "vulnerabilityIds")]
    #[serde(default)]
    pub vulnerability_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchVulnerabilitiesResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vulnerabilities: Option<Vec<Vulnerability>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Vulnerability {
    #[serde(rename = "atigData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub atig_data: Option<AtigData>,
    #[serde(rename = "cisaData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cisa_data: Option<CisaData>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvss2: Option<Cvss2>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvss3: Option<Cvss3>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvss4: Option<Cvss4>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cwes: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "detectionPlatforms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detection_platforms: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub epss: Option<Epss>,
    #[serde(rename = "exploitObserved")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exploit_observed: Option<ExploitObserved>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "referenceUrls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_urls: Option<Vec<String>>,
    #[serde(rename = "relatedVulnerabilities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_vulnerabilities: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "sourceUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<String>,
    #[serde(rename = "vendorCreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_created_at: Option<f64>,
    #[serde(rename = "vendorSeverity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_severity: Option<String>,
    #[serde(rename = "vendorUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_updated_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AtigData {
    #[serde(rename = "firstSeen")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_seen: Option<f64>,
    #[serde(rename = "lastSeen")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_seen: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttps: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Cvss2 {
    #[serde(rename = "baseScore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_score: Option<f64>,
    #[serde(rename = "scoringVector")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scoring_vector: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Cvss3 {
    #[serde(rename = "baseScore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_score: Option<f64>,
    #[serde(rename = "scoringVector")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scoring_vector: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Cvss4 {
    #[serde(rename = "baseScore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_score: Option<f64>,
    #[serde(rename = "scoringVector")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scoring_vector: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Epss {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SendCisSessionHealthRequest {
    #[serde(rename = "scanJobId")]
    #[serde(default)]
    pub scan_job_id: String,
    #[serde(rename = "sessionToken")]
    #[serde(default)]
    pub session_token: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SendCisSessionHealthResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SendCisSessionTelemetryRequest {
    #[serde(default)]
    pub messages: Vec<CisSessionMessage>,
    #[serde(rename = "scanJobId")]
    #[serde(default)]
    pub scan_job_id: String,
    #[serde(rename = "sessionToken")]
    #[serde(default)]
    pub session_token: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CisSessionMessage {
    #[serde(rename = "cisRuleDetails")]
    #[serde(default)]
    pub cis_rule_details: String,
    #[serde(rename = "ruleId")]
    #[serde(default)]
    pub rule_id: String,
    #[serde(default)]
    pub status: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SendCisSessionTelemetryResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartCisSessionRequest {
    #[serde(default)]
    pub message: StartCisSessionMessage,
    #[serde(rename = "scanJobId")]
    #[serde(default)]
    pub scan_job_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartCisSessionMessage {
    #[serde(rename = "sessionToken")]
    #[serde(default)]
    pub session_token: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartCisSessionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartCodeSecurityScanRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    pub resource: CodeSecurityResource,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartCodeSecurityScanResponse {
    #[serde(rename = "scanId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopCisSessionRequest {
    #[serde(default)]
    pub message: StopCisSessionMessage,
    #[serde(rename = "scanJobId")]
    #[serde(default)]
    pub scan_job_id: String,
    #[serde(rename = "sessionToken")]
    #[serde(default)]
    pub session_token: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopCisSessionMessage {
    #[serde(rename = "benchmarkProfile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub benchmark_profile: Option<String>,
    #[serde(rename = "benchmarkVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub benchmark_version: Option<String>,
    #[serde(rename = "computePlatform")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_platform: Option<ComputePlatform>,
    #[serde(default)]
    pub progress: StopCisMessageProgress,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default)]
    pub status: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ComputePlatform {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopCisMessageProgress {
    #[serde(rename = "errorChecks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_checks: Option<i32>,
    #[serde(rename = "failedChecks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_checks: Option<i32>,
    #[serde(rename = "informationalChecks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub informational_checks: Option<i32>,
    #[serde(rename = "notApplicableChecks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_applicable_checks: Option<i32>,
    #[serde(rename = "notEvaluatedChecks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_evaluated_checks: Option<i32>,
    #[serde(rename = "successfulChecks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful_checks: Option<i32>,
    #[serde(rename = "totalChecks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_checks: Option<i32>,
    #[serde(rename = "unknownChecks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown_checks: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopCisSessionResponse {}

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
pub struct UpdateCisScanConfigurationRequest {
    #[serde(rename = "scanConfigurationArn")]
    #[serde(default)]
    pub scan_configuration_arn: String,
    #[serde(rename = "scanName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Schedule>,
    #[serde(rename = "securityLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_level: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<UpdateCisTargets>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCisTargets {
    #[serde(rename = "accountIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    #[serde(rename = "targetResourceTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_resource_tags: Option<std::collections::HashMap<String, Vec<String>>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCisScanConfigurationResponse {
    #[serde(rename = "scanConfigurationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_configuration_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCodeSecurityIntegrationRequest {
    #[serde(default)]
    pub details: UpdateIntegrationDetails,
    #[serde(rename = "integrationArn")]
    #[serde(default)]
    pub integration_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateIntegrationDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub github: Option<UpdateGitHubIntegrationDetail>,
    #[serde(rename = "gitlabSelfManaged")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gitlab_self_managed: Option<UpdateGitLabSelfManagedIntegrationDetail>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateGitHubIntegrationDetail {
    #[serde(default)]
    pub code: String,
    #[serde(rename = "installationId")]
    #[serde(default)]
    pub installation_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateGitLabSelfManagedIntegrationDetail {
    #[serde(rename = "authCode")]
    #[serde(default)]
    pub auth_code: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCodeSecurityIntegrationResponse {
    #[serde(rename = "integrationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCodeSecurityScanConfigurationRequest {
    #[serde(default)]
    pub configuration: CodeSecurityScanConfiguration,
    #[serde(rename = "scanConfigurationArn")]
    #[serde(default)]
    pub scan_configuration_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCodeSecurityScanConfigurationResponse {
    #[serde(rename = "scanConfigurationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_configuration_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateConfigurationRequest {
    #[serde(rename = "ec2Configuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_configuration: Option<Ec2Configuration>,
    #[serde(rename = "ecrConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecr_configuration: Option<EcrConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Ec2Configuration {
    #[serde(rename = "scanMode")]
    #[serde(default)]
    pub scan_mode: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EcrConfiguration {
    #[serde(rename = "pullDateRescanDuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_date_rescan_duration: Option<String>,
    #[serde(rename = "pullDateRescanMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_date_rescan_mode: Option<String>,
    #[serde(rename = "rescanDuration")]
    #[serde(default)]
    pub rescan_duration: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateConfigurationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateEc2DeepInspectionConfigurationRequest {
    #[serde(rename = "activateDeepInspection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activate_deep_inspection: Option<bool>,
    #[serde(rename = "packagePaths")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_paths: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateEc2DeepInspectionConfigurationResponse {
    #[serde(rename = "errorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "orgPackagePaths")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_package_paths: Option<Vec<String>>,
    #[serde(rename = "packagePaths")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_paths: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateEncryptionKeyRequest {
    #[serde(rename = "kmsKeyId")]
    #[serde(default)]
    pub kms_key_id: String,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    pub resource_type: String,
    #[serde(rename = "scanType")]
    #[serde(default)]
    pub scan_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateEncryptionKeyResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFilterRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "filterArn")]
    #[serde(default)]
    pub filter_arn: String,
    #[serde(rename = "filterCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_criteria: Option<FilterCriteria>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFilterResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateOrgEc2DeepInspectionConfigurationRequest {
    #[serde(rename = "orgPackagePaths")]
    #[serde(default)]
    pub org_package_paths: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateOrgEc2DeepInspectionConfigurationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateOrganizationConfigurationRequest {
    #[serde(rename = "autoEnable")]
    #[serde(default)]
    pub auto_enable: AutoEnable,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateOrganizationConfigurationResponse {
    #[serde(rename = "autoEnable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_enable: Option<AutoEnable>,
}
