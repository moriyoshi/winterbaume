//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-auditmanager

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateAssessmentReportEvidenceFolderRequest {
    #[serde(rename = "assessmentId")]
    #[serde(default)]
    pub assessment_id: String,
    #[serde(rename = "evidenceFolderId")]
    #[serde(default)]
    pub evidence_folder_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateAssessmentReportEvidenceFolderResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchAssociateAssessmentReportEvidenceRequest {
    #[serde(rename = "assessmentId")]
    #[serde(default)]
    pub assessment_id: String,
    #[serde(rename = "evidenceFolderId")]
    #[serde(default)]
    pub evidence_folder_id: String,
    #[serde(rename = "evidenceIds")]
    #[serde(default)]
    pub evidence_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchAssociateAssessmentReportEvidenceResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<AssessmentReportEvidenceError>>,
    #[serde(rename = "evidenceIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssessmentReportEvidenceError {
    #[serde(rename = "errorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "errorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "evidenceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchCreateDelegationByAssessmentRequest {
    #[serde(rename = "assessmentId")]
    #[serde(default)]
    pub assessment_id: String,
    #[serde(rename = "createDelegationRequests")]
    #[serde(default)]
    pub create_delegation_requests: Vec<CreateDelegationRequest>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDelegationRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "controlSetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_set_id: Option<String>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "roleType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchCreateDelegationByAssessmentResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegations: Option<Vec<Delegation>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<BatchCreateDelegationByAssessmentError>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Delegation {
    #[serde(rename = "assessmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_id: Option<String>,
    #[serde(rename = "assessmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "controlSetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_set_id: Option<String>,
    #[serde(rename = "createdBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "lastUpdated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<f64>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "roleType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchCreateDelegationByAssessmentError {
    #[serde(rename = "createDelegationRequest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_delegation_request: Option<CreateDelegationRequest>,
    #[serde(rename = "errorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "errorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDeleteDelegationByAssessmentRequest {
    #[serde(rename = "assessmentId")]
    #[serde(default)]
    pub assessment_id: String,
    #[serde(rename = "delegationIds")]
    #[serde(default)]
    pub delegation_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDeleteDelegationByAssessmentResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<BatchDeleteDelegationByAssessmentError>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDeleteDelegationByAssessmentError {
    #[serde(rename = "delegationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegation_id: Option<String>,
    #[serde(rename = "errorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "errorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDisassociateAssessmentReportEvidenceRequest {
    #[serde(rename = "assessmentId")]
    #[serde(default)]
    pub assessment_id: String,
    #[serde(rename = "evidenceFolderId")]
    #[serde(default)]
    pub evidence_folder_id: String,
    #[serde(rename = "evidenceIds")]
    #[serde(default)]
    pub evidence_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDisassociateAssessmentReportEvidenceResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<AssessmentReportEvidenceError>>,
    #[serde(rename = "evidenceIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchImportEvidenceToAssessmentControlRequest {
    #[serde(rename = "assessmentId")]
    #[serde(default)]
    pub assessment_id: String,
    #[serde(rename = "controlId")]
    #[serde(default)]
    pub control_id: String,
    #[serde(rename = "controlSetId")]
    #[serde(default)]
    pub control_set_id: String,
    #[serde(rename = "manualEvidence")]
    #[serde(default)]
    pub manual_evidence: Vec<ManualEvidence>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ManualEvidence {
    #[serde(rename = "evidenceFileName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_file_name: Option<String>,
    #[serde(rename = "s3ResourcePath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_resource_path: Option<String>,
    #[serde(rename = "textResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_response: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchImportEvidenceToAssessmentControlResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<BatchImportEvidenceToAssessmentControlError>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchImportEvidenceToAssessmentControlError {
    #[serde(rename = "errorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "errorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "manualEvidence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_evidence: Option<ManualEvidence>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAssessmentFrameworkRequest {
    #[serde(rename = "complianceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_type: Option<String>,
    #[serde(rename = "controlSets")]
    #[serde(default)]
    pub control_sets: Vec<CreateAssessmentFrameworkControlSet>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAssessmentFrameworkControlSet {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controls: Option<Vec<CreateAssessmentFrameworkControl>>,
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAssessmentFrameworkControl {
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAssessmentFrameworkResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework: Option<Framework>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Framework {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "complianceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_type: Option<String>,
    #[serde(rename = "controlSets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_sets: Option<Vec<ControlSet>>,
    #[serde(rename = "controlSources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_sources: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "createdBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    #[serde(rename = "lastUpdatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_by: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ControlSet {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controls: Option<Vec<Control>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Control {
    #[serde(rename = "actionPlanInstructions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_plan_instructions: Option<String>,
    #[serde(rename = "actionPlanTitle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_plan_title: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "controlMappingSources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_mapping_sources: Option<Vec<ControlMappingSource>>,
    #[serde(rename = "controlSources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_sources: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "createdBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    #[serde(rename = "lastUpdatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_by: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "testingInformation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub testing_information: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ControlMappingSource {
    #[serde(rename = "sourceDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_description: Option<String>,
    #[serde(rename = "sourceFrequency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_frequency: Option<String>,
    #[serde(rename = "sourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    #[serde(rename = "sourceKeyword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_keyword: Option<SourceKeyword>,
    #[serde(rename = "sourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_name: Option<String>,
    #[serde(rename = "sourceSetUpOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_set_up_option: Option<String>,
    #[serde(rename = "sourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    #[serde(rename = "troubleshootingText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub troubleshooting_text: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SourceKeyword {
    #[serde(rename = "keywordInputType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword_input_type: Option<String>,
    #[serde(rename = "keywordValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword_value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAssessmentReportRequest {
    #[serde(rename = "assessmentId")]
    #[serde(default)]
    pub assessment_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "queryStatement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_statement: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAssessmentReportResponse {
    #[serde(rename = "assessmentReport")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_report: Option<AssessmentReport>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssessmentReport {
    #[serde(rename = "assessmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_id: Option<String>,
    #[serde(rename = "assessmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(rename = "awsAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_account_id: Option<String>,
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAssessmentRequest {
    #[serde(rename = "assessmentReportsDestination")]
    #[serde(default)]
    pub assessment_reports_destination: AssessmentReportsDestination,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "frameworkId")]
    #[serde(default)]
    pub framework_id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub roles: Vec<Role>,
    #[serde(default)]
    pub scope: Scope,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssessmentReportsDestination {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    #[serde(rename = "destinationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Role {
    #[serde(rename = "roleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(rename = "roleType")]
    #[serde(default)]
    pub role_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Scope {
    #[serde(rename = "awsAccounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_accounts: Option<Vec<AWSAccount>>,
    #[serde(rename = "awsServices")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_services: Option<Vec<AWSService>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AWSAccount {
    #[serde(rename = "emailAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AWSService {
    #[serde(rename = "serviceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAssessmentResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment: Option<Assessment>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Assessment {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "awsAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_account: Option<AWSAccount>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework: Option<AssessmentFramework>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<AssessmentMetadata>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssessmentFramework {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "controlSets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_sets: Option<Vec<AssessmentControlSet>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<FrameworkMetadata>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssessmentControlSet {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controls: Option<Vec<AssessmentControl>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegations: Option<Vec<Delegation>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "manualEvidenceCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_evidence_count: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<Role>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "systemEvidenceCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_evidence_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssessmentControl {
    #[serde(rename = "assessmentReportEvidenceCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_report_evidence_count: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<Vec<ControlComment>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "evidenceCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_count: Option<i32>,
    #[serde(rename = "evidenceSources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_sources: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ControlComment {
    #[serde(rename = "authorName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_name: Option<String>,
    #[serde(rename = "commentBody")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment_body: Option<String>,
    #[serde(rename = "postedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posted_date: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FrameworkMetadata {
    #[serde(rename = "complianceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssessmentMetadata {
    #[serde(rename = "assessmentReportsDestination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_reports_destination: Option<AssessmentReportsDestination>,
    #[serde(rename = "complianceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_type: Option<String>,
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegations: Option<Vec<Delegation>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "lastUpdated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<Role>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<Scope>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateControlRequest {
    #[serde(rename = "actionPlanInstructions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_plan_instructions: Option<String>,
    #[serde(rename = "actionPlanTitle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_plan_title: Option<String>,
    #[serde(rename = "controlMappingSources")]
    #[serde(default)]
    pub control_mapping_sources: Vec<CreateControlMappingSource>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "testingInformation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub testing_information: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateControlMappingSource {
    #[serde(rename = "sourceDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_description: Option<String>,
    #[serde(rename = "sourceFrequency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_frequency: Option<String>,
    #[serde(rename = "sourceKeyword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_keyword: Option<SourceKeyword>,
    #[serde(rename = "sourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_name: Option<String>,
    #[serde(rename = "sourceSetUpOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_set_up_option: Option<String>,
    #[serde(rename = "sourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    #[serde(rename = "troubleshootingText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub troubleshooting_text: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateControlResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control: Option<Control>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAssessmentFrameworkRequest {
    #[serde(rename = "frameworkId")]
    #[serde(default)]
    pub framework_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAssessmentFrameworkResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAssessmentFrameworkShareRequest {
    #[serde(rename = "requestId")]
    #[serde(default)]
    pub request_id: String,
    #[serde(rename = "requestType")]
    #[serde(default)]
    pub request_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAssessmentFrameworkShareResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAssessmentReportRequest {
    #[serde(rename = "assessmentId")]
    #[serde(default)]
    pub assessment_id: String,
    #[serde(rename = "assessmentReportId")]
    #[serde(default)]
    pub assessment_report_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAssessmentReportResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAssessmentRequest {
    #[serde(rename = "assessmentId")]
    #[serde(default)]
    pub assessment_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAssessmentResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteControlRequest {
    #[serde(rename = "controlId")]
    #[serde(default)]
    pub control_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteControlResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeregisterAccountRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeregisterAccountResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeregisterOrganizationAdminAccountRequest {
    #[serde(rename = "adminAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_account_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeregisterOrganizationAdminAccountResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateAssessmentReportEvidenceFolderRequest {
    #[serde(rename = "assessmentId")]
    #[serde(default)]
    pub assessment_id: String,
    #[serde(rename = "evidenceFolderId")]
    #[serde(default)]
    pub evidence_folder_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateAssessmentReportEvidenceFolderResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAccountStatusRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAccountStatusResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAssessmentFrameworkRequest {
    #[serde(rename = "frameworkId")]
    #[serde(default)]
    pub framework_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAssessmentFrameworkResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework: Option<Framework>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAssessmentReportUrlRequest {
    #[serde(rename = "assessmentId")]
    #[serde(default)]
    pub assessment_id: String,
    #[serde(rename = "assessmentReportId")]
    #[serde(default)]
    pub assessment_report_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAssessmentReportUrlResponse {
    #[serde(rename = "preSignedUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_signed_url: Option<URL>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct URL {
    #[serde(rename = "hyperlinkName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hyperlink_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAssessmentRequest {
    #[serde(rename = "assessmentId")]
    #[serde(default)]
    pub assessment_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAssessmentResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment: Option<Assessment>,
    #[serde(rename = "userRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_role: Option<Role>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetChangeLogsRequest {
    #[serde(rename = "assessmentId")]
    #[serde(default)]
    pub assessment_id: String,
    #[serde(rename = "controlId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_id: Option<String>,
    #[serde(rename = "controlSetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_set_id: Option<String>,
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
pub struct GetChangeLogsResponse {
    #[serde(rename = "changeLogs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_logs: Option<Vec<ChangeLog>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChangeLog {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "createdBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "objectName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_name: Option<String>,
    #[serde(rename = "objectType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetControlRequest {
    #[serde(rename = "controlId")]
    #[serde(default)]
    pub control_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetControlResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control: Option<Control>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDelegationsRequest {
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
pub struct GetDelegationsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegations: Option<Vec<DelegationMetadata>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DelegationMetadata {
    #[serde(rename = "assessmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_id: Option<String>,
    #[serde(rename = "assessmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_name: Option<String>,
    #[serde(rename = "controlSetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_set_name: Option<String>,
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetEvidenceByEvidenceFolderRequest {
    #[serde(rename = "assessmentId")]
    #[serde(default)]
    pub assessment_id: String,
    #[serde(rename = "controlSetId")]
    #[serde(default)]
    pub control_set_id: String,
    #[serde(rename = "evidenceFolderId")]
    #[serde(default)]
    pub evidence_folder_id: String,
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
pub struct GetEvidenceByEvidenceFolderResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence: Option<Vec<Evidence>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Evidence {
    #[serde(rename = "assessmentReportSelection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_report_selection: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "awsAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_account_id: Option<String>,
    #[serde(rename = "awsOrganization")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_organization: Option<String>,
    #[serde(rename = "complianceCheck")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_check: Option<String>,
    #[serde(rename = "dataSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source: Option<String>,
    #[serde(rename = "eventName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_name: Option<String>,
    #[serde(rename = "eventSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_source: Option<String>,
    #[serde(rename = "evidenceAwsAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_aws_account_id: Option<String>,
    #[serde(rename = "evidenceByType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_by_type: Option<String>,
    #[serde(rename = "evidenceFolderId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_folder_id: Option<String>,
    #[serde(rename = "iamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "resourcesIncluded")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources_included: Option<Vec<Resource>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Resource {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "complianceCheck")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_check: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetEvidenceFileUploadUrlRequest {
    #[serde(rename = "fileName")]
    #[serde(default)]
    pub file_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetEvidenceFileUploadUrlResponse {
    #[serde(rename = "evidenceFileName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_file_name: Option<String>,
    #[serde(rename = "uploadUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetEvidenceFolderRequest {
    #[serde(rename = "assessmentId")]
    #[serde(default)]
    pub assessment_id: String,
    #[serde(rename = "controlSetId")]
    #[serde(default)]
    pub control_set_id: String,
    #[serde(rename = "evidenceFolderId")]
    #[serde(default)]
    pub evidence_folder_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetEvidenceFolderResponse {
    #[serde(rename = "evidenceFolder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_folder: Option<AssessmentEvidenceFolder>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssessmentEvidenceFolder {
    #[serde(rename = "assessmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_id: Option<String>,
    #[serde(rename = "assessmentReportSelectionCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_report_selection_count: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(rename = "controlId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_id: Option<String>,
    #[serde(rename = "controlName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_name: Option<String>,
    #[serde(rename = "controlSetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_set_id: Option<String>,
    #[serde(rename = "dataSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<f64>,
    #[serde(rename = "evidenceAwsServiceSourceCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_aws_service_source_count: Option<i32>,
    #[serde(rename = "evidenceByTypeComplianceCheckCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_by_type_compliance_check_count: Option<i32>,
    #[serde(rename = "evidenceByTypeComplianceCheckIssuesCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_by_type_compliance_check_issues_count: Option<i32>,
    #[serde(rename = "evidenceByTypeConfigurationDataCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_by_type_configuration_data_count: Option<i32>,
    #[serde(rename = "evidenceByTypeManualCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_by_type_manual_count: Option<i32>,
    #[serde(rename = "evidenceByTypeUserActivityCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_by_type_user_activity_count: Option<i32>,
    #[serde(rename = "evidenceResourcesIncludedCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_resources_included_count: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "totalEvidence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_evidence: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetEvidenceFoldersByAssessmentControlRequest {
    #[serde(rename = "assessmentId")]
    #[serde(default)]
    pub assessment_id: String,
    #[serde(rename = "controlId")]
    #[serde(default)]
    pub control_id: String,
    #[serde(rename = "controlSetId")]
    #[serde(default)]
    pub control_set_id: String,
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
pub struct GetEvidenceFoldersByAssessmentControlResponse {
    #[serde(rename = "evidenceFolders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_folders: Option<Vec<AssessmentEvidenceFolder>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetEvidenceFoldersByAssessmentRequest {
    #[serde(rename = "assessmentId")]
    #[serde(default)]
    pub assessment_id: String,
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
pub struct GetEvidenceFoldersByAssessmentResponse {
    #[serde(rename = "evidenceFolders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_folders: Option<Vec<AssessmentEvidenceFolder>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetEvidenceRequest {
    #[serde(rename = "assessmentId")]
    #[serde(default)]
    pub assessment_id: String,
    #[serde(rename = "controlSetId")]
    #[serde(default)]
    pub control_set_id: String,
    #[serde(rename = "evidenceFolderId")]
    #[serde(default)]
    pub evidence_folder_id: String,
    #[serde(rename = "evidenceId")]
    #[serde(default)]
    pub evidence_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetEvidenceResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence: Option<Evidence>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetInsightsByAssessmentRequest {
    #[serde(rename = "assessmentId")]
    #[serde(default)]
    pub assessment_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetInsightsByAssessmentResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insights: Option<InsightsByAssessment>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InsightsByAssessment {
    #[serde(rename = "assessmentControlsCountByNoncompliantEvidence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_controls_count_by_noncompliant_evidence: Option<i32>,
    #[serde(rename = "compliantEvidenceCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliant_evidence_count: Option<i32>,
    #[serde(rename = "inconclusiveEvidenceCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inconclusive_evidence_count: Option<i32>,
    #[serde(rename = "lastUpdated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<f64>,
    #[serde(rename = "noncompliantEvidenceCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub noncompliant_evidence_count: Option<i32>,
    #[serde(rename = "totalAssessmentControlsCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_assessment_controls_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetInsightsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetInsightsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insights: Option<Insights>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Insights {
    #[serde(rename = "activeAssessmentsCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_assessments_count: Option<i32>,
    #[serde(rename = "assessmentControlsCountByNoncompliantEvidence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_controls_count_by_noncompliant_evidence: Option<i32>,
    #[serde(rename = "compliantEvidenceCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliant_evidence_count: Option<i32>,
    #[serde(rename = "inconclusiveEvidenceCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inconclusive_evidence_count: Option<i32>,
    #[serde(rename = "lastUpdated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<f64>,
    #[serde(rename = "noncompliantEvidenceCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub noncompliant_evidence_count: Option<i32>,
    #[serde(rename = "totalAssessmentControlsCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_assessment_controls_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetOrganizationAdminAccountRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetOrganizationAdminAccountResponse {
    #[serde(rename = "adminAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_account_id: Option<String>,
    #[serde(rename = "organizationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetServicesInScopeRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetServicesInScopeResponse {
    #[serde(rename = "serviceMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_metadata: Option<Vec<ServiceMetadata>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceMetadata {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSettingsRequest {
    #[serde(default)]
    pub attribute: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSettingsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<Settings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Settings {
    #[serde(rename = "defaultAssessmentReportsDestination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_assessment_reports_destination: Option<AssessmentReportsDestination>,
    #[serde(rename = "defaultExportDestination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_export_destination: Option<DefaultExportDestination>,
    #[serde(rename = "defaultProcessOwners")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_process_owners: Option<Vec<Role>>,
    #[serde(rename = "deregistrationPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deregistration_policy: Option<DeregistrationPolicy>,
    #[serde(rename = "evidenceFinderEnablement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_finder_enablement: Option<EvidenceFinderEnablement>,
    #[serde(rename = "isAwsOrgEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_aws_org_enabled: Option<bool>,
    #[serde(rename = "kmsKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key: Option<String>,
    #[serde(rename = "snsTopic")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DefaultExportDestination {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    #[serde(rename = "destinationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeregistrationPolicy {
    #[serde(rename = "deleteResources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_resources: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvidenceFinderEnablement {
    #[serde(rename = "backfillStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backfill_status: Option<String>,
    #[serde(rename = "enablementStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enablement_status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "eventDataStoreArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_data_store_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAssessmentControlInsightsByControlDomainRequest {
    #[serde(rename = "assessmentId")]
    #[serde(default)]
    pub assessment_id: String,
    #[serde(rename = "controlDomainId")]
    #[serde(default)]
    pub control_domain_id: String,
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
pub struct ListAssessmentControlInsightsByControlDomainResponse {
    #[serde(rename = "controlInsightsByAssessment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_insights_by_assessment: Option<Vec<ControlInsightsMetadataByAssessmentItem>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ControlInsightsMetadataByAssessmentItem {
    #[serde(rename = "controlSetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_set_name: Option<String>,
    #[serde(rename = "evidenceInsights")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_insights: Option<EvidenceInsights>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "lastUpdated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvidenceInsights {
    #[serde(rename = "compliantEvidenceCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliant_evidence_count: Option<i32>,
    #[serde(rename = "inconclusiveEvidenceCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inconclusive_evidence_count: Option<i32>,
    #[serde(rename = "noncompliantEvidenceCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub noncompliant_evidence_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAssessmentFrameworkShareRequestsRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "requestType")]
    #[serde(default)]
    pub request_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAssessmentFrameworkShareRequestsResponse {
    #[serde(rename = "assessmentFrameworkShareRequests")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_framework_share_requests: Option<Vec<AssessmentFrameworkShareRequest>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssessmentFrameworkShareRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "complianceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_type: Option<String>,
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "customControlsCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_controls_count: Option<i32>,
    #[serde(rename = "destinationAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_account: Option<String>,
    #[serde(rename = "destinationRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_region: Option<String>,
    #[serde(rename = "expirationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<f64>,
    #[serde(rename = "frameworkDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework_description: Option<String>,
    #[serde(rename = "frameworkId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework_id: Option<String>,
    #[serde(rename = "frameworkName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "lastUpdated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<f64>,
    #[serde(rename = "sourceAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_account: Option<String>,
    #[serde(rename = "standardControlsCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard_controls_count: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAssessmentFrameworksRequest {
    #[serde(rename = "frameworkType")]
    #[serde(default)]
    pub framework_type: String,
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
pub struct ListAssessmentFrameworksResponse {
    #[serde(rename = "frameworkMetadataList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework_metadata_list: Option<Vec<AssessmentFrameworkMetadata>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssessmentFrameworkMetadata {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "complianceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_type: Option<String>,
    #[serde(rename = "controlSetsCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_sets_count: Option<i32>,
    #[serde(rename = "controlsCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controls_count: Option<i32>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAssessmentReportsRequest {
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
pub struct ListAssessmentReportsResponse {
    #[serde(rename = "assessmentReports")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_reports: Option<Vec<AssessmentReportMetadata>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssessmentReportMetadata {
    #[serde(rename = "assessmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_id: Option<String>,
    #[serde(rename = "assessmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAssessmentsRequest {
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
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAssessmentsResponse {
    #[serde(rename = "assessmentMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_metadata: Option<Vec<AssessmentMetadataItem>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssessmentMetadataItem {
    #[serde(rename = "complianceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_type: Option<String>,
    #[serde(rename = "creationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegations: Option<Vec<Delegation>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "lastUpdated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<Role>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListControlDomainInsightsByAssessmentRequest {
    #[serde(rename = "assessmentId")]
    #[serde(default)]
    pub assessment_id: String,
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
pub struct ListControlDomainInsightsByAssessmentResponse {
    #[serde(rename = "controlDomainInsights")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_domain_insights: Option<Vec<ControlDomainInsights>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ControlDomainInsights {
    #[serde(rename = "controlsCountByNoncompliantEvidence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controls_count_by_noncompliant_evidence: Option<i32>,
    #[serde(rename = "evidenceInsights")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_insights: Option<EvidenceInsights>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "lastUpdated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "totalControlsCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_controls_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListControlDomainInsightsRequest {
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
pub struct ListControlDomainInsightsResponse {
    #[serde(rename = "controlDomainInsights")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_domain_insights: Option<Vec<ControlDomainInsights>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListControlInsightsByControlDomainRequest {
    #[serde(rename = "controlDomainId")]
    #[serde(default)]
    pub control_domain_id: String,
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
pub struct ListControlInsightsByControlDomainResponse {
    #[serde(rename = "controlInsightsMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_insights_metadata: Option<Vec<ControlInsightsMetadataItem>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ControlInsightsMetadataItem {
    #[serde(rename = "evidenceInsights")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_insights: Option<EvidenceInsights>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "lastUpdated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListControlsRequest {
    #[serde(rename = "controlCatalogId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_catalog_id: Option<String>,
    #[serde(rename = "controlType")]
    #[serde(default)]
    pub control_type: String,
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
pub struct ListControlsResponse {
    #[serde(rename = "controlMetadataList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_metadata_list: Option<Vec<ControlMetadata>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ControlMetadata {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "controlSources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_sources: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListKeywordsForDataSourceRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    pub source: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListKeywordsForDataSourceResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListNotificationsRequest {
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
pub struct ListNotificationsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notifications: Option<Vec<Notification>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Notification {
    #[serde(rename = "assessmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_id: Option<String>,
    #[serde(rename = "assessmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_name: Option<String>,
    #[serde(rename = "controlSetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_set_id: Option<String>,
    #[serde(rename = "controlSetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_set_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "eventTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
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
pub struct RegisterAccountRequest {
    #[serde(rename = "delegatedAdminAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegated_admin_account: Option<String>,
    #[serde(rename = "kmsKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterAccountResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterOrganizationAdminAccountRequest {
    #[serde(rename = "adminAccountId")]
    #[serde(default)]
    pub admin_account_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterOrganizationAdminAccountResponse {
    #[serde(rename = "adminAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_account_id: Option<String>,
    #[serde(rename = "organizationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartAssessmentFrameworkShareRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "destinationAccount")]
    #[serde(default)]
    pub destination_account: String,
    #[serde(rename = "destinationRegion")]
    #[serde(default)]
    pub destination_region: String,
    #[serde(rename = "frameworkId")]
    #[serde(default)]
    pub framework_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartAssessmentFrameworkShareResponse {
    #[serde(rename = "assessmentFrameworkShareRequest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_framework_share_request: Option<AssessmentFrameworkShareRequest>,
}

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
pub struct UpdateAssessmentControlRequest {
    #[serde(rename = "assessmentId")]
    #[serde(default)]
    pub assessment_id: String,
    #[serde(rename = "commentBody")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment_body: Option<String>,
    #[serde(rename = "controlId")]
    #[serde(default)]
    pub control_id: String,
    #[serde(rename = "controlSetId")]
    #[serde(default)]
    pub control_set_id: String,
    #[serde(rename = "controlStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAssessmentControlResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control: Option<AssessmentControl>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAssessmentControlSetStatusRequest {
    #[serde(rename = "assessmentId")]
    #[serde(default)]
    pub assessment_id: String,
    #[serde(default)]
    pub comment: String,
    #[serde(rename = "controlSetId")]
    #[serde(default)]
    pub control_set_id: String,
    #[serde(default)]
    pub status: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAssessmentControlSetStatusResponse {
    #[serde(rename = "controlSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_set: Option<AssessmentControlSet>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAssessmentFrameworkRequest {
    #[serde(rename = "complianceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_type: Option<String>,
    #[serde(rename = "controlSets")]
    #[serde(default)]
    pub control_sets: Vec<UpdateAssessmentFrameworkControlSet>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "frameworkId")]
    #[serde(default)]
    pub framework_id: String,
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAssessmentFrameworkControlSet {
    #[serde(default)]
    pub controls: Vec<CreateAssessmentFrameworkControl>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAssessmentFrameworkResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework: Option<Framework>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAssessmentFrameworkShareRequest {
    #[serde(default)]
    pub action: String,
    #[serde(rename = "requestId")]
    #[serde(default)]
    pub request_id: String,
    #[serde(rename = "requestType")]
    #[serde(default)]
    pub request_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAssessmentFrameworkShareResponse {
    #[serde(rename = "assessmentFrameworkShareRequest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_framework_share_request: Option<AssessmentFrameworkShareRequest>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAssessmentRequest {
    #[serde(rename = "assessmentDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_description: Option<String>,
    #[serde(rename = "assessmentId")]
    #[serde(default)]
    pub assessment_id: String,
    #[serde(rename = "assessmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_name: Option<String>,
    #[serde(rename = "assessmentReportsDestination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_reports_destination: Option<AssessmentReportsDestination>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<Role>>,
    #[serde(default)]
    pub scope: Scope,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAssessmentResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment: Option<Assessment>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAssessmentStatusRequest {
    #[serde(rename = "assessmentId")]
    #[serde(default)]
    pub assessment_id: String,
    #[serde(default)]
    pub status: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAssessmentStatusResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment: Option<Assessment>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateControlRequest {
    #[serde(rename = "actionPlanInstructions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_plan_instructions: Option<String>,
    #[serde(rename = "actionPlanTitle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_plan_title: Option<String>,
    #[serde(rename = "controlId")]
    #[serde(default)]
    pub control_id: String,
    #[serde(rename = "controlMappingSources")]
    #[serde(default)]
    pub control_mapping_sources: Vec<ControlMappingSource>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "testingInformation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub testing_information: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateControlResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control: Option<Control>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSettingsRequest {
    #[serde(rename = "defaultAssessmentReportsDestination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_assessment_reports_destination: Option<AssessmentReportsDestination>,
    #[serde(rename = "defaultExportDestination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_export_destination: Option<DefaultExportDestination>,
    #[serde(rename = "defaultProcessOwners")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_process_owners: Option<Vec<Role>>,
    #[serde(rename = "deregistrationPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deregistration_policy: Option<DeregistrationPolicy>,
    #[serde(rename = "evidenceFinderEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_finder_enabled: Option<bool>,
    #[serde(rename = "kmsKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key: Option<String>,
    #[serde(rename = "snsTopic")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSettingsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<Settings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ValidateAssessmentReportIntegrityRequest {
    #[serde(rename = "s3RelativePath")]
    #[serde(default)]
    pub s3_relative_path: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ValidateAssessmentReportIntegrityResponse {
    #[serde(rename = "signatureAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_algorithm: Option<String>,
    #[serde(rename = "signatureDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_date_time: Option<String>,
    #[serde(rename = "signatureKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_key_id: Option<String>,
    #[serde(rename = "signatureValid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_valid: Option<bool>,
    #[serde(rename = "validationErrors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_errors: Option<Vec<String>>,
}
