//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-securityhub

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AcceptAdministratorInvitationRequest {
    #[serde(rename = "AdministratorId")]
    #[serde(default)]
    pub administrator_id: String,
    #[serde(rename = "InvitationId")]
    #[serde(default)]
    pub invitation_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AcceptAdministratorInvitationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AcceptInvitationRequest {
    #[serde(rename = "InvitationId")]
    #[serde(default)]
    pub invitation_id: String,
    #[serde(rename = "MasterId")]
    #[serde(default)]
    pub master_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AcceptInvitationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDeleteAutomationRulesRequest {
    #[serde(rename = "AutomationRulesArns")]
    #[serde(default)]
    pub automation_rules_arns: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDeleteAutomationRulesResponse {
    #[serde(rename = "ProcessedAutomationRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processed_automation_rules: Option<Vec<String>>,
    #[serde(rename = "UnprocessedAutomationRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_automation_rules: Option<Vec<UnprocessedAutomationRule>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UnprocessedAutomationRule {
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i32>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "RuleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDisableStandardsRequest {
    #[serde(rename = "StandardsSubscriptionArns")]
    #[serde(default)]
    pub standards_subscription_arns: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDisableStandardsResponse {
    #[serde(rename = "StandardsSubscriptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standards_subscriptions: Option<Vec<StandardsSubscription>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StandardsSubscription {
    #[serde(rename = "StandardsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standards_arn: Option<String>,
    #[serde(rename = "StandardsControlsUpdatable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standards_controls_updatable: Option<String>,
    #[serde(rename = "StandardsInput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standards_input: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "StandardsStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standards_status: Option<String>,
    #[serde(rename = "StandardsStatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standards_status_reason: Option<StandardsStatusReason>,
    #[serde(rename = "StandardsSubscriptionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standards_subscription_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StandardsStatusReason {
    #[serde(rename = "StatusReasonCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason_code: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchEnableStandardsRequest {
    #[serde(rename = "StandardsSubscriptionRequests")]
    #[serde(default)]
    pub standards_subscription_requests: Vec<StandardsSubscriptionRequest>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StandardsSubscriptionRequest {
    #[serde(rename = "StandardsArn")]
    #[serde(default)]
    pub standards_arn: String,
    #[serde(rename = "StandardsInput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standards_input: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchEnableStandardsResponse {
    #[serde(rename = "StandardsSubscriptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standards_subscriptions: Option<Vec<StandardsSubscription>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetAutomationRulesRequest {
    #[serde(rename = "AutomationRulesArns")]
    #[serde(default)]
    pub automation_rules_arns: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetAutomationRulesResponse {
    #[serde(rename = "Rules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<AutomationRulesConfig>>,
    #[serde(rename = "UnprocessedAutomationRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_automation_rules: Option<Vec<UnprocessedAutomationRule>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomationRulesConfig {
    #[serde(rename = "Actions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<AutomationRulesAction>>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "CreatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "Criteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub criteria: Option<AutomationRulesFindingFilters>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "IsTerminal")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_terminal: Option<bool>,
    #[serde(rename = "RuleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_arn: Option<String>,
    #[serde(rename = "RuleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
    #[serde(rename = "RuleOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_order: Option<i32>,
    #[serde(rename = "RuleStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_status: Option<String>,
    #[serde(rename = "UpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomationRulesAction {
    #[serde(rename = "FindingFieldsUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_fields_update: Option<AutomationRulesFindingFieldsUpdate>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomationRulesFindingFieldsUpdate {
    #[serde(rename = "Confidence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<i32>,
    #[serde(rename = "Criticality")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub criticality: Option<i32>,
    #[serde(rename = "Note")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<NoteUpdate>,
    #[serde(rename = "RelatedFindings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_findings: Option<Vec<RelatedFinding>>,
    #[serde(rename = "Severity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<SeverityUpdate>,
    #[serde(rename = "Types")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<String>>,
    #[serde(rename = "UserDefinedFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_defined_fields: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "VerificationState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_state: Option<String>,
    #[serde(rename = "Workflow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow: Option<WorkflowUpdate>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NoteUpdate {
    #[serde(rename = "Text")]
    #[serde(default)]
    pub text: String,
    #[serde(rename = "UpdatedBy")]
    #[serde(default)]
    pub updated_by: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RelatedFinding {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "ProductArn")]
    #[serde(default)]
    pub product_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SeverityUpdate {
    #[serde(rename = "Label")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "Normalized")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normalized: Option<i32>,
    #[serde(rename = "Product")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkflowUpdate {
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomationRulesFindingFilters {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_account_id: Option<Vec<StringFilter>>,
    #[serde(rename = "AwsAccountName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_account_name: Option<Vec<StringFilter>>,
    #[serde(rename = "CompanyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name: Option<Vec<StringFilter>>,
    #[serde(rename = "ComplianceAssociatedStandardsId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_associated_standards_id: Option<Vec<StringFilter>>,
    #[serde(rename = "ComplianceSecurityControlId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_security_control_id: Option<Vec<StringFilter>>,
    #[serde(rename = "ComplianceStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_status: Option<Vec<StringFilter>>,
    #[serde(rename = "Confidence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<Vec<NumberFilter>>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Vec<DateFilter>>,
    #[serde(rename = "Criticality")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub criticality: Option<Vec<NumberFilter>>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<StringFilter>>,
    #[serde(rename = "FirstObservedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_observed_at: Option<Vec<DateFilter>>,
    #[serde(rename = "GeneratorId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generator_id: Option<Vec<StringFilter>>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Vec<StringFilter>>,
    #[serde(rename = "LastObservedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_observed_at: Option<Vec<DateFilter>>,
    #[serde(rename = "NoteText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note_text: Option<Vec<StringFilter>>,
    #[serde(rename = "NoteUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note_updated_at: Option<Vec<DateFilter>>,
    #[serde(rename = "NoteUpdatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note_updated_by: Option<Vec<StringFilter>>,
    #[serde(rename = "ProductArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_arn: Option<Vec<StringFilter>>,
    #[serde(rename = "ProductName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_name: Option<Vec<StringFilter>>,
    #[serde(rename = "RecordState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_state: Option<Vec<StringFilter>>,
    #[serde(rename = "RelatedFindingsId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_findings_id: Option<Vec<StringFilter>>,
    #[serde(rename = "RelatedFindingsProductArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_findings_product_arn: Option<Vec<StringFilter>>,
    #[serde(rename = "ResourceApplicationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_application_arn: Option<Vec<StringFilter>>,
    #[serde(rename = "ResourceApplicationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_application_name: Option<Vec<StringFilter>>,
    #[serde(rename = "ResourceDetailsOther")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_details_other: Option<Vec<MapFilter>>,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<Vec<StringFilter>>,
    #[serde(rename = "ResourcePartition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_partition: Option<Vec<StringFilter>>,
    #[serde(rename = "ResourceRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_region: Option<Vec<StringFilter>>,
    #[serde(rename = "ResourceTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_tags: Option<Vec<MapFilter>>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<Vec<StringFilter>>,
    #[serde(rename = "SeverityLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity_label: Option<Vec<StringFilter>>,
    #[serde(rename = "SourceUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<Vec<StringFilter>>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Vec<StringFilter>>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Vec<StringFilter>>,
    #[serde(rename = "UpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<Vec<DateFilter>>,
    #[serde(rename = "UserDefinedFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_defined_fields: Option<Vec<MapFilter>>,
    #[serde(rename = "VerificationState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_state: Option<Vec<StringFilter>>,
    #[serde(rename = "WorkflowStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_status: Option<Vec<StringFilter>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StringFilter {
    #[serde(rename = "Comparison")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparison: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NumberFilter {
    #[serde(rename = "Eq")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eq: Option<f64>,
    #[serde(rename = "Gt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gt: Option<f64>,
    #[serde(rename = "Gte")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gte: Option<f64>,
    #[serde(rename = "Lt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lt: Option<f64>,
    #[serde(rename = "Lte")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lte: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DateFilter {
    #[serde(rename = "DateRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_range: Option<DateRange>,
    #[serde(rename = "End")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    #[serde(rename = "Start")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DateRange {
    #[serde(rename = "Unit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MapFilter {
    #[serde(rename = "Comparison")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparison: Option<String>,
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
pub struct BatchGetConfigurationPolicyAssociationsRequest {
    #[serde(rename = "ConfigurationPolicyAssociationIdentifiers")]
    #[serde(default)]
    pub configuration_policy_association_identifiers: Vec<ConfigurationPolicyAssociation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConfigurationPolicyAssociation {
    #[serde(rename = "Target")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<Target>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Target {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "OrganizationalUnitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizational_unit_id: Option<String>,
    #[serde(rename = "RootId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetConfigurationPolicyAssociationsResponse {
    #[serde(rename = "ConfigurationPolicyAssociations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_policy_associations: Option<Vec<ConfigurationPolicyAssociationSummary>>,
    #[serde(rename = "UnprocessedConfigurationPolicyAssociations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_configuration_policy_associations:
        Option<Vec<UnprocessedConfigurationPolicyAssociation>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConfigurationPolicyAssociationSummary {
    #[serde(rename = "AssociationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_status: Option<String>,
    #[serde(rename = "AssociationStatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_status_message: Option<String>,
    #[serde(rename = "AssociationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_type: Option<String>,
    #[serde(rename = "ConfigurationPolicyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_policy_id: Option<String>,
    #[serde(rename = "TargetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
    #[serde(rename = "TargetType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,
    #[serde(rename = "UpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UnprocessedConfigurationPolicyAssociation {
    #[serde(rename = "ConfigurationPolicyAssociationIdentifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_policy_association_identifiers: Option<ConfigurationPolicyAssociation>,
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "ErrorReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetSecurityControlsRequest {
    #[serde(rename = "SecurityControlIds")]
    #[serde(default)]
    pub security_control_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetSecurityControlsResponse {
    #[serde(rename = "SecurityControls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_controls: Option<Vec<SecurityControl>>,
    #[serde(rename = "UnprocessedIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_ids: Option<Vec<UnprocessedSecurityControl>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SecurityControl {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "LastUpdateReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_reason: Option<String>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, ParameterConfiguration>>,
    #[serde(rename = "RemediationUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remediation_url: Option<String>,
    #[serde(rename = "SecurityControlArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_control_arn: Option<String>,
    #[serde(rename = "SecurityControlId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_control_id: Option<String>,
    #[serde(rename = "SecurityControlStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_control_status: Option<String>,
    #[serde(rename = "SeverityRating")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity_rating: Option<String>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "UpdateStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ParameterConfiguration {
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<ParameterValue>,
    #[serde(rename = "ValueType")]
    #[serde(default)]
    pub value_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ParameterValue {
    #[serde(rename = "Boolean")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boolean: Option<bool>,
    #[serde(rename = "Double")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub double: Option<f64>,
    #[serde(rename = "Enum")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#enum: Option<String>,
    #[serde(rename = "EnumList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enum_list: Option<Vec<String>>,
    #[serde(rename = "Integer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integer: Option<i32>,
    #[serde(rename = "IntegerList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integer_list: Option<Vec<i32>>,
    #[serde(rename = "String")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string: Option<String>,
    #[serde(rename = "StringList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_list: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UnprocessedSecurityControl {
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "ErrorReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_reason: Option<String>,
    #[serde(rename = "SecurityControlId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_control_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetStandardsControlAssociationsRequest {
    #[serde(rename = "StandardsControlAssociationIds")]
    #[serde(default)]
    pub standards_control_association_ids: Vec<StandardsControlAssociationId>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StandardsControlAssociationId {
    #[serde(rename = "SecurityControlId")]
    #[serde(default)]
    pub security_control_id: String,
    #[serde(rename = "StandardsArn")]
    #[serde(default)]
    pub standards_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetStandardsControlAssociationsResponse {
    #[serde(rename = "StandardsControlAssociationDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standards_control_association_details: Option<Vec<StandardsControlAssociationDetail>>,
    #[serde(rename = "UnprocessedAssociations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_associations: Option<Vec<UnprocessedStandardsControlAssociation>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StandardsControlAssociationDetail {
    #[serde(rename = "AssociationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_status: Option<String>,
    #[serde(rename = "RelatedRequirements")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_requirements: Option<Vec<String>>,
    #[serde(rename = "SecurityControlArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_control_arn: Option<String>,
    #[serde(rename = "SecurityControlId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_control_id: Option<String>,
    #[serde(rename = "StandardsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standards_arn: Option<String>,
    #[serde(rename = "StandardsControlArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standards_control_arns: Option<Vec<String>>,
    #[serde(rename = "StandardsControlDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standards_control_description: Option<String>,
    #[serde(rename = "StandardsControlTitle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standards_control_title: Option<String>,
    #[serde(rename = "UpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "UpdatedReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UnprocessedStandardsControlAssociation {
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "ErrorReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_reason: Option<String>,
    #[serde(rename = "StandardsControlAssociationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standards_control_association_id: Option<StandardsControlAssociationId>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchImportFindingsRequest {
    #[serde(rename = "Findings")]
    #[serde(default)]
    pub findings: Vec<AwsSecurityFinding>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsSecurityFinding {
    #[serde(rename = "Action")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<Action>,
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "AwsAccountName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_account_name: Option<String>,
    #[serde(rename = "CompanyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    #[serde(rename = "Compliance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance: Option<Compliance>,
    #[serde(rename = "Confidence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<i32>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    pub created_at: String,
    #[serde(rename = "Criticality")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub criticality: Option<i32>,
    #[serde(rename = "Description")]
    #[serde(default)]
    pub description: String,
    #[serde(rename = "Detection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detection: Option<Detection>,
    #[serde(rename = "FindingProviderFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_provider_fields: Option<FindingProviderFields>,
    #[serde(rename = "FirstObservedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_observed_at: Option<String>,
    #[serde(rename = "GeneratorDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generator_details: Option<GeneratorDetails>,
    #[serde(rename = "GeneratorId")]
    #[serde(default)]
    pub generator_id: String,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "LastObservedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_observed_at: Option<String>,
    #[serde(rename = "Malware")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub malware: Option<Vec<Malware>>,
    #[serde(rename = "Network")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<Network>,
    #[serde(rename = "NetworkPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_path: Option<Vec<NetworkPathComponent>>,
    #[serde(rename = "Note")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<Note>,
    #[serde(rename = "PatchSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_summary: Option<PatchSummary>,
    #[serde(rename = "Process")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process: Option<ProcessDetails>,
    #[serde(rename = "ProcessedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processed_at: Option<String>,
    #[serde(rename = "ProductArn")]
    #[serde(default)]
    pub product_arn: String,
    #[serde(rename = "ProductFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_fields: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ProductName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
    #[serde(rename = "RecordState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_state: Option<String>,
    #[serde(rename = "Region")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "RelatedFindings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_findings: Option<Vec<RelatedFinding>>,
    #[serde(rename = "Remediation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remediation: Option<Remediation>,
    #[serde(rename = "Resources")]
    #[serde(default)]
    pub resources: Vec<Resource>,
    #[serde(rename = "Sample")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample: Option<bool>,
    #[serde(rename = "SchemaVersion")]
    #[serde(default)]
    pub schema_version: String,
    #[serde(rename = "Severity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<Severity>,
    #[serde(rename = "SourceUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<String>,
    #[serde(rename = "ThreatIntelIndicators")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_intel_indicators: Option<Vec<ThreatIntelIndicator>>,
    #[serde(rename = "Threats")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threats: Option<Vec<Threat>>,
    #[serde(rename = "Title")]
    #[serde(default)]
    pub title: String,
    #[serde(rename = "Types")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<String>>,
    #[serde(rename = "UpdatedAt")]
    #[serde(default)]
    pub updated_at: String,
    #[serde(rename = "UserDefinedFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_defined_fields: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "VerificationState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_state: Option<String>,
    #[serde(rename = "Vulnerabilities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vulnerabilities: Option<Vec<Vulnerability>>,
    #[serde(rename = "Workflow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow: Option<Workflow>,
    #[serde(rename = "WorkflowState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Action {
    #[serde(rename = "ActionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_type: Option<String>,
    #[serde(rename = "AwsApiCallAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_api_call_action: Option<AwsApiCallAction>,
    #[serde(rename = "DnsRequestAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_request_action: Option<DnsRequestAction>,
    #[serde(rename = "NetworkConnectionAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_connection_action: Option<NetworkConnectionAction>,
    #[serde(rename = "PortProbeAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_probe_action: Option<PortProbeAction>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsApiCallAction {
    #[serde(rename = "AffectedResources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affected_resources: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Api")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api: Option<String>,
    #[serde(rename = "CallerType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caller_type: Option<String>,
    #[serde(rename = "DomainDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_details: Option<AwsApiCallActionDomainDetails>,
    #[serde(rename = "FirstSeen")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_seen: Option<String>,
    #[serde(rename = "LastSeen")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_seen: Option<String>,
    #[serde(rename = "RemoteIpDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_ip_details: Option<ActionRemoteIpDetails>,
    #[serde(rename = "ServiceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsApiCallActionDomainDetails {
    #[serde(rename = "Domain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActionRemoteIpDetails {
    #[serde(rename = "City")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<City>,
    #[serde(rename = "Country")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Country>,
    #[serde(rename = "GeoLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo_location: Option<GeoLocation>,
    #[serde(rename = "IpAddressV4")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_v4: Option<String>,
    #[serde(rename = "Organization")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<IpOrganizationDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct City {
    #[serde(rename = "CityName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Country {
    #[serde(rename = "CountryCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    #[serde(rename = "CountryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GeoLocation {
    #[serde(rename = "Lat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lat: Option<f64>,
    #[serde(rename = "Lon")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lon: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IpOrganizationDetails {
    #[serde(rename = "Asn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn: Option<i32>,
    #[serde(rename = "AsnOrg")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn_org: Option<String>,
    #[serde(rename = "Isp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isp: Option<String>,
    #[serde(rename = "Org")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DnsRequestAction {
    #[serde(rename = "Blocked")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked: Option<bool>,
    #[serde(rename = "Domain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "Protocol")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NetworkConnectionAction {
    #[serde(rename = "Blocked")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked: Option<bool>,
    #[serde(rename = "ConnectionDirection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_direction: Option<String>,
    #[serde(rename = "LocalPortDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_port_details: Option<ActionLocalPortDetails>,
    #[serde(rename = "Protocol")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(rename = "RemoteIpDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_ip_details: Option<ActionRemoteIpDetails>,
    #[serde(rename = "RemotePortDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_port_details: Option<ActionRemotePortDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActionLocalPortDetails {
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "PortName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActionRemotePortDetails {
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "PortName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PortProbeAction {
    #[serde(rename = "Blocked")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked: Option<bool>,
    #[serde(rename = "PortProbeDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_probe_details: Option<Vec<PortProbeDetail>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PortProbeDetail {
    #[serde(rename = "LocalIpDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_ip_details: Option<ActionLocalIpDetails>,
    #[serde(rename = "LocalPortDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_port_details: Option<ActionLocalPortDetails>,
    #[serde(rename = "RemoteIpDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_ip_details: Option<ActionRemoteIpDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActionLocalIpDetails {
    #[serde(rename = "IpAddressV4")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_v4: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Compliance {
    #[serde(rename = "AssociatedStandards")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_standards: Option<Vec<AssociatedStandard>>,
    #[serde(rename = "RelatedRequirements")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_requirements: Option<Vec<String>>,
    #[serde(rename = "SecurityControlId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_control_id: Option<String>,
    #[serde(rename = "SecurityControlParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_control_parameters: Option<Vec<SecurityControlParameter>>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusReasons")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reasons: Option<Vec<StatusReason>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociatedStandard {
    #[serde(rename = "StandardsId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standards_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SecurityControlParameter {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StatusReason {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ReasonCode")]
    #[serde(default)]
    pub reason_code: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Detection {
    #[serde(rename = "Sequence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence: Option<Sequence>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Sequence {
    #[serde(rename = "Actors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actors: Option<Vec<Actor>>,
    #[serde(rename = "Endpoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<Vec<NetworkEndpoint>>,
    #[serde(rename = "SequenceIndicators")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_indicators: Option<Vec<Indicator>>,
    #[serde(rename = "Signals")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signals: Option<Vec<Signal>>,
    #[serde(rename = "Uid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Actor {
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Session")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<ActorSession>,
    #[serde(rename = "User")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<ActorUser>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActorSession {
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<i64>,
    #[serde(rename = "Issuer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    #[serde(rename = "MfaStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_status: Option<String>,
    #[serde(rename = "Uid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActorUser {
    #[serde(rename = "Account")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<UserAccount>,
    #[serde(rename = "CredentialUid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_uid: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Uid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserAccount {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Uid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NetworkEndpoint {
    #[serde(rename = "AutonomousSystem")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autonomous_system: Option<NetworkAutonomousSystem>,
    #[serde(rename = "Connection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection: Option<NetworkConnection>,
    #[serde(rename = "Domain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Ip")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(rename = "Location")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<NetworkGeoLocation>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NetworkAutonomousSystem {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Number")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NetworkConnection {
    #[serde(rename = "Direction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NetworkGeoLocation {
    #[serde(rename = "City")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(rename = "Country")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(rename = "Lat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lat: Option<f64>,
    #[serde(rename = "Lon")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lon: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Indicator {
    #[serde(rename = "Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Signal {
    #[serde(rename = "ActorIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_ids: Option<Vec<String>>,
    #[serde(rename = "Count")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    #[serde(rename = "EndpointIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_ids: Option<Vec<String>>,
    #[serde(rename = "FirstSeenAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_seen_at: Option<i64>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LastSeenAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_seen_at: Option<i64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ProductArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_arn: Option<String>,
    #[serde(rename = "ResourceIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_ids: Option<Vec<String>>,
    #[serde(rename = "Severity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<f64>,
    #[serde(rename = "SignalIndicators")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signal_indicators: Option<Vec<Indicator>>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "UpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FindingProviderFields {
    #[serde(rename = "Confidence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<i32>,
    #[serde(rename = "Criticality")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub criticality: Option<i32>,
    #[serde(rename = "RelatedFindings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_findings: Option<Vec<RelatedFinding>>,
    #[serde(rename = "Severity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<FindingProviderSeverity>,
    #[serde(rename = "Types")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FindingProviderSeverity {
    #[serde(rename = "Label")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "Original")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GeneratorDetails {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Labels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Malware {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Path")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Network {
    #[serde(rename = "DestinationDomain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_domain: Option<String>,
    #[serde(rename = "DestinationIpV4")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_ip_v4: Option<String>,
    #[serde(rename = "DestinationIpV6")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_ip_v6: Option<String>,
    #[serde(rename = "DestinationPort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_port: Option<i32>,
    #[serde(rename = "Direction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    #[serde(rename = "OpenPortRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_port_range: Option<PortRange>,
    #[serde(rename = "Protocol")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(rename = "SourceDomain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_domain: Option<String>,
    #[serde(rename = "SourceIpV4")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ip_v4: Option<String>,
    #[serde(rename = "SourceIpV6")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ip_v6: Option<String>,
    #[serde(rename = "SourceMac")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_mac: Option<String>,
    #[serde(rename = "SourcePort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_port: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PortRange {
    #[serde(rename = "Begin")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin: Option<i32>,
    #[serde(rename = "End")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NetworkPathComponent {
    #[serde(rename = "ComponentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_id: Option<String>,
    #[serde(rename = "ComponentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_type: Option<String>,
    #[serde(rename = "Egress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress: Option<NetworkHeader>,
    #[serde(rename = "Ingress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingress: Option<NetworkHeader>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NetworkHeader {
    #[serde(rename = "Destination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<NetworkPathComponentDetails>,
    #[serde(rename = "Protocol")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(rename = "Source")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<NetworkPathComponentDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NetworkPathComponentDetails {
    #[serde(rename = "Address")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Vec<String>>,
    #[serde(rename = "PortRanges")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_ranges: Option<Vec<PortRange>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Note {
    #[serde(rename = "Text")]
    #[serde(default)]
    pub text: String,
    #[serde(rename = "UpdatedAt")]
    #[serde(default)]
    pub updated_at: String,
    #[serde(rename = "UpdatedBy")]
    #[serde(default)]
    pub updated_by: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PatchSummary {
    #[serde(rename = "FailedCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_count: Option<i32>,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "InstalledCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installed_count: Option<i32>,
    #[serde(rename = "InstalledOtherCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installed_other_count: Option<i32>,
    #[serde(rename = "InstalledPendingReboot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installed_pending_reboot: Option<i32>,
    #[serde(rename = "InstalledRejectedCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installed_rejected_count: Option<i32>,
    #[serde(rename = "MissingCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_count: Option<i32>,
    #[serde(rename = "Operation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(rename = "OperationEndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_end_time: Option<String>,
    #[serde(rename = "OperationStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_start_time: Option<String>,
    #[serde(rename = "RebootOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reboot_option: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProcessDetails {
    #[serde(rename = "LaunchedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launched_at: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ParentPid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_pid: Option<i32>,
    #[serde(rename = "Path")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "Pid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<i32>,
    #[serde(rename = "TerminatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Remediation {
    #[serde(rename = "Recommendation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation: Option<Recommendation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Recommendation {
    #[serde(rename = "Text")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "Url")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Resource {
    #[serde(rename = "ApplicationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_arn: Option<String>,
    #[serde(rename = "ApplicationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    #[serde(rename = "DataClassification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_classification: Option<DataClassificationDetails>,
    #[serde(rename = "Details")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<ResourceDetails>,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "Partition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition: Option<String>,
    #[serde(rename = "Region")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "ResourceRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_role: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataClassificationDetails {
    #[serde(rename = "DetailedResultsLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detailed_results_location: Option<String>,
    #[serde(rename = "Result")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<ClassificationResult>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClassificationResult {
    #[serde(rename = "AdditionalOccurrences")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_occurrences: Option<bool>,
    #[serde(rename = "CustomDataIdentifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data_identifiers: Option<CustomDataIdentifiersResult>,
    #[serde(rename = "MimeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(rename = "SensitiveData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensitive_data: Option<Vec<SensitiveDataResult>>,
    #[serde(rename = "SizeClassified")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_classified: Option<i64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ClassificationStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomDataIdentifiersResult {
    #[serde(rename = "Detections")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detections: Option<Vec<CustomDataIdentifiersDetections>>,
    #[serde(rename = "TotalCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomDataIdentifiersDetections {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Count")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Occurrences")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub occurrences: Option<Occurrences>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Occurrences {
    #[serde(rename = "Cells")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cells: Option<Vec<Cell>>,
    #[serde(rename = "LineRanges")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_ranges: Option<Vec<Range>>,
    #[serde(rename = "OffsetRanges")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_ranges: Option<Vec<Range>>,
    #[serde(rename = "Pages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pages: Option<Vec<Page>>,
    #[serde(rename = "Records")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<Record>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Cell {
    #[serde(rename = "CellReference")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cell_reference: Option<String>,
    #[serde(rename = "Column")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column: Option<i64>,
    #[serde(rename = "ColumnName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_name: Option<String>,
    #[serde(rename = "Row")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Range {
    #[serde(rename = "End")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<i64>,
    #[serde(rename = "Start")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<i64>,
    #[serde(rename = "StartColumn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_column: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Page {
    #[serde(rename = "LineRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_range: Option<Range>,
    #[serde(rename = "OffsetRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_range: Option<Range>,
    #[serde(rename = "PageNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Record {
    #[serde(rename = "JsonPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_path: Option<String>,
    #[serde(rename = "RecordIndex")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_index: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SensitiveDataResult {
    #[serde(rename = "Category")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "Detections")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detections: Option<Vec<SensitiveDataDetections>>,
    #[serde(rename = "TotalCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SensitiveDataDetections {
    #[serde(rename = "Count")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(rename = "Occurrences")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub occurrences: Option<Occurrences>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClassificationStatus {
    #[serde(rename = "Code")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "Reason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceDetails {
    #[serde(rename = "AwsAmazonMqBroker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_amazon_mq_broker: Option<AwsAmazonMqBrokerDetails>,
    #[serde(rename = "AwsApiGatewayRestApi")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_api_gateway_rest_api: Option<AwsApiGatewayRestApiDetails>,
    #[serde(rename = "AwsApiGatewayStage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_api_gateway_stage: Option<AwsApiGatewayStageDetails>,
    #[serde(rename = "AwsApiGatewayV2Api")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_api_gateway_v2_api: Option<AwsApiGatewayV2ApiDetails>,
    #[serde(rename = "AwsApiGatewayV2Stage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_api_gateway_v2_stage: Option<AwsApiGatewayV2StageDetails>,
    #[serde(rename = "AwsAppSyncGraphQlApi")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_app_sync_graph_ql_api: Option<AwsAppSyncGraphQlApiDetails>,
    #[serde(rename = "AwsAthenaWorkGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_athena_work_group: Option<AwsAthenaWorkGroupDetails>,
    #[serde(rename = "AwsAutoScalingAutoScalingGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_auto_scaling_auto_scaling_group: Option<AwsAutoScalingAutoScalingGroupDetails>,
    #[serde(rename = "AwsAutoScalingLaunchConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_auto_scaling_launch_configuration: Option<AwsAutoScalingLaunchConfigurationDetails>,
    #[serde(rename = "AwsBackupBackupPlan")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_backup_backup_plan: Option<AwsBackupBackupPlanDetails>,
    #[serde(rename = "AwsBackupBackupVault")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_backup_backup_vault: Option<AwsBackupBackupVaultDetails>,
    #[serde(rename = "AwsBackupRecoveryPoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_backup_recovery_point: Option<AwsBackupRecoveryPointDetails>,
    #[serde(rename = "AwsCertificateManagerCertificate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_certificate_manager_certificate: Option<AwsCertificateManagerCertificateDetails>,
    #[serde(rename = "AwsCloudFormationStack")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_cloud_formation_stack: Option<AwsCloudFormationStackDetails>,
    #[serde(rename = "AwsCloudFrontDistribution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_cloud_front_distribution: Option<AwsCloudFrontDistributionDetails>,
    #[serde(rename = "AwsCloudTrailTrail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_cloud_trail_trail: Option<AwsCloudTrailTrailDetails>,
    #[serde(rename = "AwsCloudWatchAlarm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_cloud_watch_alarm: Option<AwsCloudWatchAlarmDetails>,
    #[serde(rename = "AwsCodeBuildProject")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_code_build_project: Option<AwsCodeBuildProjectDetails>,
    #[serde(rename = "AwsDmsEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_dms_endpoint: Option<AwsDmsEndpointDetails>,
    #[serde(rename = "AwsDmsReplicationInstance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_dms_replication_instance: Option<AwsDmsReplicationInstanceDetails>,
    #[serde(rename = "AwsDmsReplicationTask")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_dms_replication_task: Option<AwsDmsReplicationTaskDetails>,
    #[serde(rename = "AwsDynamoDbTable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_dynamo_db_table: Option<AwsDynamoDbTableDetails>,
    #[serde(rename = "AwsEc2ClientVpnEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_ec2_client_vpn_endpoint: Option<AwsEc2ClientVpnEndpointDetails>,
    #[serde(rename = "AwsEc2Eip")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_ec2_eip: Option<AwsEc2EipDetails>,
    #[serde(rename = "AwsEc2Instance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_ec2_instance: Option<AwsEc2InstanceDetails>,
    #[serde(rename = "AwsEc2LaunchTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_ec2_launch_template: Option<AwsEc2LaunchTemplateDetails>,
    #[serde(rename = "AwsEc2NetworkAcl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_ec2_network_acl: Option<AwsEc2NetworkAclDetails>,
    #[serde(rename = "AwsEc2NetworkInterface")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_ec2_network_interface: Option<AwsEc2NetworkInterfaceDetails>,
    #[serde(rename = "AwsEc2RouteTable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_ec2_route_table: Option<AwsEc2RouteTableDetails>,
    #[serde(rename = "AwsEc2SecurityGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_ec2_security_group: Option<AwsEc2SecurityGroupDetails>,
    #[serde(rename = "AwsEc2Subnet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_ec2_subnet: Option<AwsEc2SubnetDetails>,
    #[serde(rename = "AwsEc2TransitGateway")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_ec2_transit_gateway: Option<AwsEc2TransitGatewayDetails>,
    #[serde(rename = "AwsEc2Volume")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_ec2_volume: Option<AwsEc2VolumeDetails>,
    #[serde(rename = "AwsEc2Vpc")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_ec2_vpc: Option<AwsEc2VpcDetails>,
    #[serde(rename = "AwsEc2VpcEndpointService")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_ec2_vpc_endpoint_service: Option<AwsEc2VpcEndpointServiceDetails>,
    #[serde(rename = "AwsEc2VpcPeeringConnection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_ec2_vpc_peering_connection: Option<AwsEc2VpcPeeringConnectionDetails>,
    #[serde(rename = "AwsEc2VpnConnection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_ec2_vpn_connection: Option<AwsEc2VpnConnectionDetails>,
    #[serde(rename = "AwsEcrContainerImage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_ecr_container_image: Option<AwsEcrContainerImageDetails>,
    #[serde(rename = "AwsEcrRepository")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_ecr_repository: Option<AwsEcrRepositoryDetails>,
    #[serde(rename = "AwsEcsCluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_ecs_cluster: Option<AwsEcsClusterDetails>,
    #[serde(rename = "AwsEcsContainer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_ecs_container: Option<AwsEcsContainerDetails>,
    #[serde(rename = "AwsEcsService")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_ecs_service: Option<AwsEcsServiceDetails>,
    #[serde(rename = "AwsEcsTask")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_ecs_task: Option<AwsEcsTaskDetails>,
    #[serde(rename = "AwsEcsTaskDefinition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_ecs_task_definition: Option<AwsEcsTaskDefinitionDetails>,
    #[serde(rename = "AwsEfsAccessPoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_efs_access_point: Option<AwsEfsAccessPointDetails>,
    #[serde(rename = "AwsEksCluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_eks_cluster: Option<AwsEksClusterDetails>,
    #[serde(rename = "AwsElasticBeanstalkEnvironment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_elastic_beanstalk_environment: Option<AwsElasticBeanstalkEnvironmentDetails>,
    #[serde(rename = "AwsElasticsearchDomain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_elasticsearch_domain: Option<AwsElasticsearchDomainDetails>,
    #[serde(rename = "AwsElbLoadBalancer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_elb_load_balancer: Option<AwsElbLoadBalancerDetails>,
    #[serde(rename = "AwsElbv2LoadBalancer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_elbv2_load_balancer: Option<AwsElbv2LoadBalancerDetails>,
    #[serde(rename = "AwsEventSchemasRegistry")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_event_schemas_registry: Option<AwsEventSchemasRegistryDetails>,
    #[serde(rename = "AwsEventsEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_events_endpoint: Option<AwsEventsEndpointDetails>,
    #[serde(rename = "AwsEventsEventbus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_events_eventbus: Option<AwsEventsEventbusDetails>,
    #[serde(rename = "AwsGuardDutyDetector")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_guard_duty_detector: Option<AwsGuardDutyDetectorDetails>,
    #[serde(rename = "AwsIamAccessKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_iam_access_key: Option<AwsIamAccessKeyDetails>,
    #[serde(rename = "AwsIamGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_iam_group: Option<AwsIamGroupDetails>,
    #[serde(rename = "AwsIamPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_iam_policy: Option<AwsIamPolicyDetails>,
    #[serde(rename = "AwsIamRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_iam_role: Option<AwsIamRoleDetails>,
    #[serde(rename = "AwsIamUser")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_iam_user: Option<AwsIamUserDetails>,
    #[serde(rename = "AwsKinesisStream")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_kinesis_stream: Option<AwsKinesisStreamDetails>,
    #[serde(rename = "AwsKmsKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_kms_key: Option<AwsKmsKeyDetails>,
    #[serde(rename = "AwsLambdaFunction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_lambda_function: Option<AwsLambdaFunctionDetails>,
    #[serde(rename = "AwsLambdaLayerVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_lambda_layer_version: Option<AwsLambdaLayerVersionDetails>,
    #[serde(rename = "AwsMskCluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_msk_cluster: Option<AwsMskClusterDetails>,
    #[serde(rename = "AwsNetworkFirewallFirewall")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_network_firewall_firewall: Option<AwsNetworkFirewallFirewallDetails>,
    #[serde(rename = "AwsNetworkFirewallFirewallPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_network_firewall_firewall_policy: Option<AwsNetworkFirewallFirewallPolicyDetails>,
    #[serde(rename = "AwsNetworkFirewallRuleGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_network_firewall_rule_group: Option<AwsNetworkFirewallRuleGroupDetails>,
    #[serde(rename = "AwsOpenSearchServiceDomain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_open_search_service_domain: Option<AwsOpenSearchServiceDomainDetails>,
    #[serde(rename = "AwsRdsDbCluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_rds_db_cluster: Option<AwsRdsDbClusterDetails>,
    #[serde(rename = "AwsRdsDbClusterSnapshot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_rds_db_cluster_snapshot: Option<AwsRdsDbClusterSnapshotDetails>,
    #[serde(rename = "AwsRdsDbInstance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_rds_db_instance: Option<AwsRdsDbInstanceDetails>,
    #[serde(rename = "AwsRdsDbSecurityGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_rds_db_security_group: Option<AwsRdsDbSecurityGroupDetails>,
    #[serde(rename = "AwsRdsDbSnapshot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_rds_db_snapshot: Option<AwsRdsDbSnapshotDetails>,
    #[serde(rename = "AwsRdsEventSubscription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_rds_event_subscription: Option<AwsRdsEventSubscriptionDetails>,
    #[serde(rename = "AwsRedshiftCluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_redshift_cluster: Option<AwsRedshiftClusterDetails>,
    #[serde(rename = "AwsRoute53HostedZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_route53_hosted_zone: Option<AwsRoute53HostedZoneDetails>,
    #[serde(rename = "AwsS3AccessPoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_s3_access_point: Option<AwsS3AccessPointDetails>,
    #[serde(rename = "AwsS3AccountPublicAccessBlock")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_s3_account_public_access_block: Option<AwsS3AccountPublicAccessBlockDetails>,
    #[serde(rename = "AwsS3Bucket")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_s3_bucket: Option<AwsS3BucketDetails>,
    #[serde(rename = "AwsS3Object")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_s3_object: Option<AwsS3ObjectDetails>,
    #[serde(rename = "AwsSageMakerNotebookInstance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_sage_maker_notebook_instance: Option<AwsSageMakerNotebookInstanceDetails>,
    #[serde(rename = "AwsSecretsManagerSecret")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_secrets_manager_secret: Option<AwsSecretsManagerSecretDetails>,
    #[serde(rename = "AwsSnsTopic")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_sns_topic: Option<AwsSnsTopicDetails>,
    #[serde(rename = "AwsSqsQueue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_sqs_queue: Option<AwsSqsQueueDetails>,
    #[serde(rename = "AwsSsmPatchCompliance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_ssm_patch_compliance: Option<AwsSsmPatchComplianceDetails>,
    #[serde(rename = "AwsStepFunctionStateMachine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_step_function_state_machine: Option<AwsStepFunctionStateMachineDetails>,
    #[serde(rename = "AwsWafRateBasedRule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_waf_rate_based_rule: Option<AwsWafRateBasedRuleDetails>,
    #[serde(rename = "AwsWafRegionalRateBasedRule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_waf_regional_rate_based_rule: Option<AwsWafRegionalRateBasedRuleDetails>,
    #[serde(rename = "AwsWafRegionalRule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_waf_regional_rule: Option<AwsWafRegionalRuleDetails>,
    #[serde(rename = "AwsWafRegionalRuleGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_waf_regional_rule_group: Option<AwsWafRegionalRuleGroupDetails>,
    #[serde(rename = "AwsWafRegionalWebAcl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_waf_regional_web_acl: Option<AwsWafRegionalWebAclDetails>,
    #[serde(rename = "AwsWafRule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_waf_rule: Option<AwsWafRuleDetails>,
    #[serde(rename = "AwsWafRuleGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_waf_rule_group: Option<AwsWafRuleGroupDetails>,
    #[serde(rename = "AwsWafWebAcl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_waf_web_acl: Option<AwsWafWebAclDetails>,
    #[serde(rename = "AwsWafv2RuleGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_wafv2_rule_group: Option<AwsWafv2RuleGroupDetails>,
    #[serde(rename = "AwsWafv2WebAcl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_wafv2_web_acl: Option<AwsWafv2WebAclDetails>,
    #[serde(rename = "AwsXrayEncryptionConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_xray_encryption_config: Option<AwsXrayEncryptionConfigDetails>,
    #[serde(rename = "CodeRepository")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_repository: Option<CodeRepositoryDetails>,
    #[serde(rename = "Container")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<ContainerDetails>,
    #[serde(rename = "Other")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsAmazonMqBrokerDetails {
    #[serde(rename = "AuthenticationStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_strategy: Option<String>,
    #[serde(rename = "AutoMinorVersionUpgrade")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_minor_version_upgrade: Option<bool>,
    #[serde(rename = "BrokerArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broker_arn: Option<String>,
    #[serde(rename = "BrokerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broker_id: Option<String>,
    #[serde(rename = "BrokerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broker_name: Option<String>,
    #[serde(rename = "DeploymentMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_mode: Option<String>,
    #[serde(rename = "EncryptionOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_options: Option<AwsAmazonMqBrokerEncryptionOptionsDetails>,
    #[serde(rename = "EngineType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_type: Option<String>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "HostInstanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_instance_type: Option<String>,
    #[serde(rename = "LdapServerMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ldap_server_metadata: Option<AwsAmazonMqBrokerLdapServerMetadataDetails>,
    #[serde(rename = "Logs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logs: Option<AwsAmazonMqBrokerLogsDetails>,
    #[serde(rename = "MaintenanceWindowStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_window_start_time: Option<AwsAmazonMqBrokerMaintenanceWindowStartTimeDetails>,
    #[serde(rename = "PubliclyAccessible")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<bool>,
    #[serde(rename = "SecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,
    #[serde(rename = "StorageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    #[serde(rename = "SubnetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    #[serde(rename = "Users")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<AwsAmazonMqBrokerUsersDetails>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsAmazonMqBrokerEncryptionOptionsDetails {
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "UseAwsOwnedKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_aws_owned_key: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsAmazonMqBrokerLdapServerMetadataDetails {
    #[serde(rename = "Hosts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosts: Option<Vec<String>>,
    #[serde(rename = "RoleBase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_base: Option<String>,
    #[serde(rename = "RoleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    #[serde(rename = "RoleSearchMatching")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_search_matching: Option<String>,
    #[serde(rename = "RoleSearchSubtree")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_search_subtree: Option<bool>,
    #[serde(rename = "ServiceAccountUsername")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_account_username: Option<String>,
    #[serde(rename = "UserBase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_base: Option<String>,
    #[serde(rename = "UserRoleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_role_name: Option<String>,
    #[serde(rename = "UserSearchMatching")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_search_matching: Option<String>,
    #[serde(rename = "UserSearchSubtree")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_search_subtree: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsAmazonMqBrokerLogsDetails {
    #[serde(rename = "Audit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit: Option<bool>,
    #[serde(rename = "AuditLogGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_log_group: Option<String>,
    #[serde(rename = "General")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub general: Option<bool>,
    #[serde(rename = "GeneralLogGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub general_log_group: Option<String>,
    #[serde(rename = "Pending")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending: Option<AwsAmazonMqBrokerLogsPendingDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsAmazonMqBrokerLogsPendingDetails {
    #[serde(rename = "Audit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit: Option<bool>,
    #[serde(rename = "General")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub general: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsAmazonMqBrokerMaintenanceWindowStartTimeDetails {
    #[serde(rename = "DayOfWeek")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_of_week: Option<String>,
    #[serde(rename = "TimeOfDay")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_of_day: Option<String>,
    #[serde(rename = "TimeZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsAmazonMqBrokerUsersDetails {
    #[serde(rename = "PendingChange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_change: Option<String>,
    #[serde(rename = "Username")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsApiGatewayRestApiDetails {
    #[serde(rename = "ApiKeySource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_source: Option<String>,
    #[serde(rename = "BinaryMediaTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary_media_types: Option<Vec<String>>,
    #[serde(rename = "CreatedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EndpointConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_configuration: Option<AwsApiGatewayEndpointConfiguration>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "MinimumCompressionSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_compression_size: Option<i32>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsApiGatewayEndpointConfiguration {
    #[serde(rename = "Types")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsApiGatewayStageDetails {
    #[serde(rename = "AccessLogSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_log_settings: Option<AwsApiGatewayAccessLogSettings>,
    #[serde(rename = "CacheClusterEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_cluster_enabled: Option<bool>,
    #[serde(rename = "CacheClusterSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_cluster_size: Option<String>,
    #[serde(rename = "CacheClusterStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_cluster_status: Option<String>,
    #[serde(rename = "CanarySettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canary_settings: Option<AwsApiGatewayCanarySettings>,
    #[serde(rename = "ClientCertificateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_certificate_id: Option<String>,
    #[serde(rename = "CreatedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    #[serde(rename = "DeploymentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "DocumentationVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_version: Option<String>,
    #[serde(rename = "LastUpdatedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date: Option<String>,
    #[serde(rename = "MethodSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method_settings: Option<Vec<AwsApiGatewayMethodSettings>>,
    #[serde(rename = "StageName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_name: Option<String>,
    #[serde(rename = "TracingEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracing_enabled: Option<bool>,
    #[serde(rename = "Variables")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "WebAclArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_acl_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsApiGatewayAccessLogSettings {
    #[serde(rename = "DestinationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_arn: Option<String>,
    #[serde(rename = "Format")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsApiGatewayCanarySettings {
    #[serde(rename = "DeploymentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    #[serde(rename = "PercentTraffic")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_traffic: Option<f64>,
    #[serde(rename = "StageVariableOverrides")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_variable_overrides: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "UseStageCache")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_stage_cache: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsApiGatewayMethodSettings {
    #[serde(rename = "CacheDataEncrypted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_data_encrypted: Option<bool>,
    #[serde(rename = "CacheTtlInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_ttl_in_seconds: Option<i32>,
    #[serde(rename = "CachingEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caching_enabled: Option<bool>,
    #[serde(rename = "DataTraceEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_trace_enabled: Option<bool>,
    #[serde(rename = "HttpMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_method: Option<String>,
    #[serde(rename = "LoggingLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_level: Option<String>,
    #[serde(rename = "MetricsEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics_enabled: Option<bool>,
    #[serde(rename = "RequireAuthorizationForCacheControl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_authorization_for_cache_control: Option<bool>,
    #[serde(rename = "ResourcePath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_path: Option<String>,
    #[serde(rename = "ThrottlingBurstLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttling_burst_limit: Option<i32>,
    #[serde(rename = "ThrottlingRateLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttling_rate_limit: Option<f64>,
    #[serde(rename = "UnauthorizedCacheControlHeaderStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unauthorized_cache_control_header_strategy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsApiGatewayV2ApiDetails {
    #[serde(rename = "ApiEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_endpoint: Option<String>,
    #[serde(rename = "ApiId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_id: Option<String>,
    #[serde(rename = "ApiKeySelectionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_selection_expression: Option<String>,
    #[serde(rename = "CorsConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cors_configuration: Option<AwsCorsConfiguration>,
    #[serde(rename = "CreatedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ProtocolType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol_type: Option<String>,
    #[serde(rename = "RouteSelectionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_selection_expression: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsCorsConfiguration {
    #[serde(rename = "AllowCredentials")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_credentials: Option<bool>,
    #[serde(rename = "AllowHeaders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_headers: Option<Vec<String>>,
    #[serde(rename = "AllowMethods")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_methods: Option<Vec<String>>,
    #[serde(rename = "AllowOrigins")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_origins: Option<Vec<String>>,
    #[serde(rename = "ExposeHeaders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expose_headers: Option<Vec<String>>,
    #[serde(rename = "MaxAge")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_age: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsApiGatewayV2StageDetails {
    #[serde(rename = "AccessLogSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_log_settings: Option<AwsApiGatewayAccessLogSettings>,
    #[serde(rename = "ApiGatewayManaged")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_gateway_managed: Option<bool>,
    #[serde(rename = "AutoDeploy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_deploy: Option<bool>,
    #[serde(rename = "ClientCertificateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_certificate_id: Option<String>,
    #[serde(rename = "CreatedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    #[serde(rename = "DefaultRouteSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_route_settings: Option<AwsApiGatewayV2RouteSettings>,
    #[serde(rename = "DeploymentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "LastDeploymentStatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_deployment_status_message: Option<String>,
    #[serde(rename = "LastUpdatedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date: Option<String>,
    #[serde(rename = "RouteSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_settings: Option<AwsApiGatewayV2RouteSettings>,
    #[serde(rename = "StageName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_name: Option<String>,
    #[serde(rename = "StageVariables")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_variables: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsApiGatewayV2RouteSettings {
    #[serde(rename = "DataTraceEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_trace_enabled: Option<bool>,
    #[serde(rename = "DetailedMetricsEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detailed_metrics_enabled: Option<bool>,
    #[serde(rename = "LoggingLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_level: Option<String>,
    #[serde(rename = "ThrottlingBurstLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttling_burst_limit: Option<i32>,
    #[serde(rename = "ThrottlingRateLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttling_rate_limit: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsAppSyncGraphQlApiDetails {
    #[serde(rename = "AdditionalAuthenticationProviders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_authentication_providers:
        Option<Vec<AwsAppSyncGraphQlApiAdditionalAuthenticationProvidersDetails>>,
    #[serde(rename = "ApiId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_id: Option<String>,
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "AuthenticationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_type: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LambdaAuthorizerConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_authorizer_config: Option<AwsAppSyncGraphQlApiLambdaAuthorizerConfigDetails>,
    #[serde(rename = "LogConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_config: Option<AwsAppSyncGraphQlApiLogConfigDetails>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "OpenIdConnectConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_id_connect_config: Option<AwsAppSyncGraphQlApiOpenIdConnectConfigDetails>,
    #[serde(rename = "UserPoolConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool_config: Option<AwsAppSyncGraphQlApiUserPoolConfigDetails>,
    #[serde(rename = "WafWebAclArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub waf_web_acl_arn: Option<String>,
    #[serde(rename = "XrayEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xray_enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsAppSyncGraphQlApiAdditionalAuthenticationProvidersDetails {
    #[serde(rename = "AuthenticationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_type: Option<String>,
    #[serde(rename = "LambdaAuthorizerConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_authorizer_config: Option<AwsAppSyncGraphQlApiLambdaAuthorizerConfigDetails>,
    #[serde(rename = "OpenIdConnectConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_id_connect_config: Option<AwsAppSyncGraphQlApiOpenIdConnectConfigDetails>,
    #[serde(rename = "UserPoolConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool_config: Option<AwsAppSyncGraphQlApiUserPoolConfigDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsAppSyncGraphQlApiLambdaAuthorizerConfigDetails {
    #[serde(rename = "AuthorizerResultTtlInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_result_ttl_in_seconds: Option<i32>,
    #[serde(rename = "AuthorizerUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_uri: Option<String>,
    #[serde(rename = "IdentityValidationExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_validation_expression: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsAppSyncGraphQlApiOpenIdConnectConfigDetails {
    #[serde(rename = "AuthTtL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_tt_l: Option<i64>,
    #[serde(rename = "ClientId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "IatTtL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iat_tt_l: Option<i64>,
    #[serde(rename = "Issuer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsAppSyncGraphQlApiUserPoolConfigDetails {
    #[serde(rename = "AppIdClientRegex")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id_client_regex: Option<String>,
    #[serde(rename = "AwsRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_region: Option<String>,
    #[serde(rename = "DefaultAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_action: Option<String>,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsAppSyncGraphQlApiLogConfigDetails {
    #[serde(rename = "CloudWatchLogsRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs_role_arn: Option<String>,
    #[serde(rename = "ExcludeVerboseContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_verbose_content: Option<bool>,
    #[serde(rename = "FieldLogLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_log_level: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsAthenaWorkGroupDetails {
    #[serde(rename = "Configuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<AwsAthenaWorkGroupConfigurationDetails>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsAthenaWorkGroupConfigurationDetails {
    #[serde(rename = "ResultConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_configuration: Option<AwsAthenaWorkGroupConfigurationResultConfigurationDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsAthenaWorkGroupConfigurationResultConfigurationDetails {
    #[serde(rename = "EncryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration:
        Option<AwsAthenaWorkGroupConfigurationResultConfigurationEncryptionConfigurationDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsAthenaWorkGroupConfigurationResultConfigurationEncryptionConfigurationDetails {
    #[serde(rename = "EncryptionOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_option: Option<String>,
    #[serde(rename = "KmsKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsAutoScalingAutoScalingGroupDetails {
    #[serde(rename = "AvailabilityZones")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<Vec<AwsAutoScalingAutoScalingGroupAvailabilityZonesListDetails>>,
    #[serde(rename = "CapacityRebalance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_rebalance: Option<bool>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(rename = "HealthCheckGracePeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_grace_period: Option<i32>,
    #[serde(rename = "HealthCheckType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_type: Option<String>,
    #[serde(rename = "LaunchConfigurationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_configuration_name: Option<String>,
    #[serde(rename = "LaunchTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_template:
        Option<AwsAutoScalingAutoScalingGroupLaunchTemplateLaunchTemplateSpecification>,
    #[serde(rename = "LoadBalancerNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_names: Option<Vec<String>>,
    #[serde(rename = "MixedInstancesPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mixed_instances_policy: Option<AwsAutoScalingAutoScalingGroupMixedInstancesPolicyDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsAutoScalingAutoScalingGroupAvailabilityZonesListDetails {
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsAutoScalingAutoScalingGroupLaunchTemplateLaunchTemplateSpecification {
    #[serde(rename = "LaunchTemplateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_template_id: Option<String>,
    #[serde(rename = "LaunchTemplateName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_template_name: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsAutoScalingAutoScalingGroupMixedInstancesPolicyDetails {
    #[serde(rename = "InstancesDistribution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances_distribution:
        Option<AwsAutoScalingAutoScalingGroupMixedInstancesPolicyInstancesDistributionDetails>,
    #[serde(rename = "LaunchTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_template:
        Option<AwsAutoScalingAutoScalingGroupMixedInstancesPolicyLaunchTemplateDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsAutoScalingAutoScalingGroupMixedInstancesPolicyInstancesDistributionDetails {
    #[serde(rename = "OnDemandAllocationStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_demand_allocation_strategy: Option<String>,
    #[serde(rename = "OnDemandBaseCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_demand_base_capacity: Option<i32>,
    #[serde(rename = "OnDemandPercentageAboveBaseCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_demand_percentage_above_base_capacity: Option<i32>,
    #[serde(rename = "SpotAllocationStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_allocation_strategy: Option<String>,
    #[serde(rename = "SpotInstancePools")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_instance_pools: Option<i32>,
    #[serde(rename = "SpotMaxPrice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_max_price: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsAutoScalingAutoScalingGroupMixedInstancesPolicyLaunchTemplateDetails {
    #[serde(rename = "LaunchTemplateSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_template_specification: Option<
        AwsAutoScalingAutoScalingGroupMixedInstancesPolicyLaunchTemplateLaunchTemplateSpecification,
    >,
    #[serde(rename = "Overrides")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overrides: Option<
        Vec<AwsAutoScalingAutoScalingGroupMixedInstancesPolicyLaunchTemplateOverridesListDetails>,
    >,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsAutoScalingAutoScalingGroupMixedInstancesPolicyLaunchTemplateLaunchTemplateSpecification
{
    #[serde(rename = "LaunchTemplateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_template_id: Option<String>,
    #[serde(rename = "LaunchTemplateName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_template_name: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsAutoScalingAutoScalingGroupMixedInstancesPolicyLaunchTemplateOverridesListDetails {
    #[serde(rename = "InstanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    #[serde(rename = "WeightedCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weighted_capacity: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsAutoScalingLaunchConfigurationDetails {
    #[serde(rename = "AssociatePublicIpAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associate_public_ip_address: Option<bool>,
    #[serde(rename = "BlockDeviceMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_device_mappings:
        Option<Vec<AwsAutoScalingLaunchConfigurationBlockDeviceMappingsDetails>>,
    #[serde(rename = "ClassicLinkVpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classic_link_vpc_id: Option<String>,
    #[serde(rename = "ClassicLinkVpcSecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classic_link_vpc_security_groups: Option<Vec<String>>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(rename = "EbsOptimized")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_optimized: Option<bool>,
    #[serde(rename = "IamInstanceProfile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_instance_profile: Option<String>,
    #[serde(rename = "ImageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    #[serde(rename = "InstanceMonitoring")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_monitoring: Option<AwsAutoScalingLaunchConfigurationInstanceMonitoringDetails>,
    #[serde(rename = "InstanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    #[serde(rename = "KernelId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kernel_id: Option<String>,
    #[serde(rename = "KeyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_name: Option<String>,
    #[serde(rename = "LaunchConfigurationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_configuration_name: Option<String>,
    #[serde(rename = "MetadataOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_options: Option<AwsAutoScalingLaunchConfigurationMetadataOptions>,
    #[serde(rename = "PlacementTenancy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_tenancy: Option<String>,
    #[serde(rename = "RamdiskId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ramdisk_id: Option<String>,
    #[serde(rename = "SecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,
    #[serde(rename = "SpotPrice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_price: Option<String>,
    #[serde(rename = "UserData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsAutoScalingLaunchConfigurationBlockDeviceMappingsDetails {
    #[serde(rename = "DeviceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    #[serde(rename = "Ebs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs: Option<AwsAutoScalingLaunchConfigurationBlockDeviceMappingsEbsDetails>,
    #[serde(rename = "NoDevice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_device: Option<bool>,
    #[serde(rename = "VirtualName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsAutoScalingLaunchConfigurationBlockDeviceMappingsEbsDetails {
    #[serde(rename = "DeleteOnTermination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_on_termination: Option<bool>,
    #[serde(rename = "Encrypted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    #[serde(rename = "Iops")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iops: Option<i32>,
    #[serde(rename = "SnapshotId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
    #[serde(rename = "VolumeSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_size: Option<i32>,
    #[serde(rename = "VolumeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsAutoScalingLaunchConfigurationInstanceMonitoringDetails {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsAutoScalingLaunchConfigurationMetadataOptions {
    #[serde(rename = "HttpEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_endpoint: Option<String>,
    #[serde(rename = "HttpPutResponseHopLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_put_response_hop_limit: Option<i32>,
    #[serde(rename = "HttpTokens")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_tokens: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsBackupBackupPlanDetails {
    #[serde(rename = "BackupPlan")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_plan: Option<AwsBackupBackupPlanBackupPlanDetails>,
    #[serde(rename = "BackupPlanArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_plan_arn: Option<String>,
    #[serde(rename = "BackupPlanId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_plan_id: Option<String>,
    #[serde(rename = "VersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsBackupBackupPlanBackupPlanDetails {
    #[serde(rename = "AdvancedBackupSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_backup_settings: Option<Vec<AwsBackupBackupPlanAdvancedBackupSettingsDetails>>,
    #[serde(rename = "BackupPlanName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_plan_name: Option<String>,
    #[serde(rename = "BackupPlanRule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_plan_rule: Option<Vec<AwsBackupBackupPlanRuleDetails>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsBackupBackupPlanAdvancedBackupSettingsDetails {
    #[serde(rename = "BackupOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_options: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsBackupBackupPlanRuleDetails {
    #[serde(rename = "CompletionWindowMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_window_minutes: Option<i64>,
    #[serde(rename = "CopyActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_actions: Option<Vec<AwsBackupBackupPlanRuleCopyActionsDetails>>,
    #[serde(rename = "EnableContinuousBackup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_continuous_backup: Option<bool>,
    #[serde(rename = "Lifecycle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<AwsBackupBackupPlanLifecycleDetails>,
    #[serde(rename = "RuleId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
    #[serde(rename = "RuleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
    #[serde(rename = "ScheduleExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_expression: Option<String>,
    #[serde(rename = "StartWindowMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_window_minutes: Option<i64>,
    #[serde(rename = "TargetBackupVault")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_backup_vault: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsBackupBackupPlanRuleCopyActionsDetails {
    #[serde(rename = "DestinationBackupVaultArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_backup_vault_arn: Option<String>,
    #[serde(rename = "Lifecycle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<AwsBackupBackupPlanLifecycleDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsBackupBackupPlanLifecycleDetails {
    #[serde(rename = "DeleteAfterDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_after_days: Option<i64>,
    #[serde(rename = "MoveToColdStorageAfterDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub move_to_cold_storage_after_days: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsBackupBackupVaultDetails {
    #[serde(rename = "AccessPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_policy: Option<String>,
    #[serde(rename = "BackupVaultArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_arn: Option<String>,
    #[serde(rename = "BackupVaultName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_name: Option<String>,
    #[serde(rename = "EncryptionKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key_arn: Option<String>,
    #[serde(rename = "Notifications")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notifications: Option<AwsBackupBackupVaultNotificationsDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsBackupBackupVaultNotificationsDetails {
    #[serde(rename = "BackupVaultEvents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_events: Option<Vec<String>>,
    #[serde(rename = "SnsTopicArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsBackupRecoveryPointDetails {
    #[serde(rename = "BackupSizeInBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_size_in_bytes: Option<i64>,
    #[serde(rename = "BackupVaultArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_arn: Option<String>,
    #[serde(rename = "BackupVaultName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_vault_name: Option<String>,
    #[serde(rename = "CalculatedLifecycle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calculated_lifecycle: Option<AwsBackupRecoveryPointCalculatedLifecycleDetails>,
    #[serde(rename = "CompletionDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_date: Option<String>,
    #[serde(rename = "CreatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<AwsBackupRecoveryPointCreatedByDetails>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "EncryptionKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key_arn: Option<String>,
    #[serde(rename = "IamRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role_arn: Option<String>,
    #[serde(rename = "IsEncrypted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_encrypted: Option<bool>,
    #[serde(rename = "LastRestoreTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_restore_time: Option<String>,
    #[serde(rename = "Lifecycle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<AwsBackupRecoveryPointLifecycleDetails>,
    #[serde(rename = "RecoveryPointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_point_arn: Option<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "SourceBackupVaultArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_backup_vault_arn: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(rename = "StorageClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_class: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsBackupRecoveryPointCalculatedLifecycleDetails {
    #[serde(rename = "DeleteAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_at: Option<String>,
    #[serde(rename = "MoveToColdStorageAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub move_to_cold_storage_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsBackupRecoveryPointCreatedByDetails {
    #[serde(rename = "BackupPlanArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_plan_arn: Option<String>,
    #[serde(rename = "BackupPlanId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_plan_id: Option<String>,
    #[serde(rename = "BackupPlanVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_plan_version: Option<String>,
    #[serde(rename = "BackupRuleId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_rule_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsBackupRecoveryPointLifecycleDetails {
    #[serde(rename = "DeleteAfterDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_after_days: Option<i64>,
    #[serde(rename = "MoveToColdStorageAfterDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub move_to_cold_storage_after_days: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsCertificateManagerCertificateDetails {
    #[serde(rename = "CertificateAuthorityArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_authority_arn: Option<String>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "DomainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(rename = "DomainValidationOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_validation_options:
        Option<Vec<AwsCertificateManagerCertificateDomainValidationOption>>,
    #[serde(rename = "ExtendedKeyUsages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_key_usages: Option<Vec<AwsCertificateManagerCertificateExtendedKeyUsage>>,
    #[serde(rename = "FailureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "ImportedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imported_at: Option<String>,
    #[serde(rename = "InUseBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_use_by: Option<Vec<String>>,
    #[serde(rename = "IssuedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued_at: Option<String>,
    #[serde(rename = "Issuer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    #[serde(rename = "KeyAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_algorithm: Option<String>,
    #[serde(rename = "KeyUsages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_usages: Option<Vec<AwsCertificateManagerCertificateKeyUsage>>,
    #[serde(rename = "NotAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_after: Option<String>,
    #[serde(rename = "NotBefore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_before: Option<String>,
    #[serde(rename = "Options")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<AwsCertificateManagerCertificateOptions>,
    #[serde(rename = "RenewalEligibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal_eligibility: Option<String>,
    #[serde(rename = "RenewalSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal_summary: Option<AwsCertificateManagerCertificateRenewalSummary>,
    #[serde(rename = "Serial")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial: Option<String>,
    #[serde(rename = "SignatureAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_algorithm: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Subject")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(rename = "SubjectAlternativeNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_alternative_names: Option<Vec<String>>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsCertificateManagerCertificateDomainValidationOption {
    #[serde(rename = "DomainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(rename = "ResourceRecord")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_record: Option<AwsCertificateManagerCertificateResourceRecord>,
    #[serde(rename = "ValidationDomain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_domain: Option<String>,
    #[serde(rename = "ValidationEmails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_emails: Option<Vec<String>>,
    #[serde(rename = "ValidationMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_method: Option<String>,
    #[serde(rename = "ValidationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsCertificateManagerCertificateResourceRecord {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsCertificateManagerCertificateExtendedKeyUsage {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "OId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsCertificateManagerCertificateKeyUsage {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsCertificateManagerCertificateOptions {
    #[serde(rename = "CertificateTransparencyLoggingPreference")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_transparency_logging_preference: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsCertificateManagerCertificateRenewalSummary {
    #[serde(rename = "DomainValidationOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_validation_options:
        Option<Vec<AwsCertificateManagerCertificateDomainValidationOption>>,
    #[serde(rename = "RenewalStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal_status: Option<String>,
    #[serde(rename = "RenewalStatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal_status_reason: Option<String>,
    #[serde(rename = "UpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsCloudFormationStackDetails {
    #[serde(rename = "Capabilities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Vec<String>>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "DisableRollback")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_rollback: Option<bool>,
    #[serde(rename = "DriftInformation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drift_information: Option<AwsCloudFormationStackDriftInformationDetails>,
    #[serde(rename = "EnableTerminationProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_termination_protection: Option<bool>,
    #[serde(rename = "LastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<String>,
    #[serde(rename = "NotificationArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_arns: Option<Vec<String>>,
    #[serde(rename = "Outputs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<AwsCloudFormationStackOutputsDetails>>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "StackId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_id: Option<String>,
    #[serde(rename = "StackName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_name: Option<String>,
    #[serde(rename = "StackStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_status: Option<String>,
    #[serde(rename = "StackStatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_status_reason: Option<String>,
    #[serde(rename = "TimeoutInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_in_minutes: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsCloudFormationStackDriftInformationDetails {
    #[serde(rename = "StackDriftStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_drift_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsCloudFormationStackOutputsDetails {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "OutputKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_key: Option<String>,
    #[serde(rename = "OutputValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsCloudFrontDistributionDetails {
    #[serde(rename = "CacheBehaviors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_behaviors: Option<AwsCloudFrontDistributionCacheBehaviors>,
    #[serde(rename = "DefaultCacheBehavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_cache_behavior: Option<AwsCloudFrontDistributionDefaultCacheBehavior>,
    #[serde(rename = "DefaultRootObject")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_root_object: Option<String>,
    #[serde(rename = "DomainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(rename = "ETag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<String>,
    #[serde(rename = "Logging")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<AwsCloudFrontDistributionLogging>,
    #[serde(rename = "OriginGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_groups: Option<AwsCloudFrontDistributionOriginGroups>,
    #[serde(rename = "Origins")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origins: Option<AwsCloudFrontDistributionOrigins>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "ViewerCertificate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewer_certificate: Option<AwsCloudFrontDistributionViewerCertificate>,
    #[serde(rename = "WebAclId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_acl_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsCloudFrontDistributionCacheBehaviors {
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<AwsCloudFrontDistributionCacheBehavior>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsCloudFrontDistributionCacheBehavior {
    #[serde(rename = "ViewerProtocolPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewer_protocol_policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsCloudFrontDistributionDefaultCacheBehavior {
    #[serde(rename = "ViewerProtocolPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewer_protocol_policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsCloudFrontDistributionLogging {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "IncludeCookies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_cookies: Option<bool>,
    #[serde(rename = "Prefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsCloudFrontDistributionOriginGroups {
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<AwsCloudFrontDistributionOriginGroup>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsCloudFrontDistributionOriginGroup {
    #[serde(rename = "FailoverCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failover_criteria: Option<AwsCloudFrontDistributionOriginGroupFailover>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsCloudFrontDistributionOriginGroupFailover {
    #[serde(rename = "StatusCodes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_codes: Option<AwsCloudFrontDistributionOriginGroupFailoverStatusCodes>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsCloudFrontDistributionOriginGroupFailoverStatusCodes {
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<i32>>,
    #[serde(rename = "Quantity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsCloudFrontDistributionOrigins {
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<AwsCloudFrontDistributionOriginItem>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsCloudFrontDistributionOriginItem {
    #[serde(rename = "CustomOriginConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_origin_config: Option<AwsCloudFrontDistributionOriginCustomOriginConfig>,
    #[serde(rename = "DomainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "OriginPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_path: Option<String>,
    #[serde(rename = "S3OriginConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_origin_config: Option<AwsCloudFrontDistributionOriginS3OriginConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsCloudFrontDistributionOriginCustomOriginConfig {
    #[serde(rename = "HttpPort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_port: Option<i32>,
    #[serde(rename = "HttpsPort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub https_port: Option<i32>,
    #[serde(rename = "OriginKeepaliveTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_keepalive_timeout: Option<i32>,
    #[serde(rename = "OriginProtocolPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_protocol_policy: Option<String>,
    #[serde(rename = "OriginReadTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_read_timeout: Option<i32>,
    #[serde(rename = "OriginSslProtocols")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_ssl_protocols: Option<AwsCloudFrontDistributionOriginSslProtocols>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsCloudFrontDistributionOriginSslProtocols {
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<String>>,
    #[serde(rename = "Quantity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsCloudFrontDistributionOriginS3OriginConfig {
    #[serde(rename = "OriginAccessIdentity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_access_identity: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsCloudFrontDistributionViewerCertificate {
    #[serde(rename = "AcmCertificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acm_certificate_arn: Option<String>,
    #[serde(rename = "Certificate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    #[serde(rename = "CertificateSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_source: Option<String>,
    #[serde(rename = "CloudFrontDefaultCertificate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_front_default_certificate: Option<bool>,
    #[serde(rename = "IamCertificateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_certificate_id: Option<String>,
    #[serde(rename = "MinimumProtocolVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_protocol_version: Option<String>,
    #[serde(rename = "SslSupportMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_support_method: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsCloudTrailTrailDetails {
    #[serde(rename = "CloudWatchLogsLogGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs_log_group_arn: Option<String>,
    #[serde(rename = "CloudWatchLogsRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs_role_arn: Option<String>,
    #[serde(rename = "HasCustomEventSelectors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_custom_event_selectors: Option<bool>,
    #[serde(rename = "HomeRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_region: Option<String>,
    #[serde(rename = "IncludeGlobalServiceEvents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_global_service_events: Option<bool>,
    #[serde(rename = "IsMultiRegionTrail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_multi_region_trail: Option<bool>,
    #[serde(rename = "IsOrganizationTrail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_organization_trail: Option<bool>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "LogFileValidationEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_file_validation_enabled: Option<bool>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "S3BucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_name: Option<String>,
    #[serde(rename = "S3KeyPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key_prefix: Option<String>,
    #[serde(rename = "SnsTopicArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_arn: Option<String>,
    #[serde(rename = "SnsTopicName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_name: Option<String>,
    #[serde(rename = "TrailArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsCloudWatchAlarmDetails {
    #[serde(rename = "ActionsEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions_enabled: Option<bool>,
    #[serde(rename = "AlarmActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_actions: Option<Vec<String>>,
    #[serde(rename = "AlarmArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_arn: Option<String>,
    #[serde(rename = "AlarmConfigurationUpdatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_configuration_updated_timestamp: Option<String>,
    #[serde(rename = "AlarmDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_description: Option<String>,
    #[serde(rename = "AlarmName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_name: Option<String>,
    #[serde(rename = "ComparisonOperator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparison_operator: Option<String>,
    #[serde(rename = "DatapointsToAlarm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datapoints_to_alarm: Option<i32>,
    #[serde(rename = "Dimensions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Vec<AwsCloudWatchAlarmDimensionsDetails>>,
    #[serde(rename = "EvaluateLowSampleCountPercentile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluate_low_sample_count_percentile: Option<String>,
    #[serde(rename = "EvaluationPeriods")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_periods: Option<i32>,
    #[serde(rename = "ExtendedStatistic")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_statistic: Option<String>,
    #[serde(rename = "InsufficientDataActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insufficient_data_actions: Option<Vec<String>>,
    #[serde(rename = "MetricName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "OkActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ok_actions: Option<Vec<String>>,
    #[serde(rename = "Period")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<i32>,
    #[serde(rename = "Statistic")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistic: Option<String>,
    #[serde(rename = "Threshold")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<f64>,
    #[serde(rename = "ThresholdMetricId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold_metric_id: Option<String>,
    #[serde(rename = "TreatMissingData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treat_missing_data: Option<String>,
    #[serde(rename = "Unit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsCloudWatchAlarmDimensionsDetails {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsCodeBuildProjectDetails {
    #[serde(rename = "Artifacts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifacts: Option<Vec<AwsCodeBuildProjectArtifactsDetails>>,
    #[serde(rename = "EncryptionKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<String>,
    #[serde(rename = "Environment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<AwsCodeBuildProjectEnvironment>,
    #[serde(rename = "LogsConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logs_config: Option<AwsCodeBuildProjectLogsConfigDetails>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "SecondaryArtifacts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_artifacts: Option<Vec<AwsCodeBuildProjectArtifactsDetails>>,
    #[serde(rename = "ServiceRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role: Option<String>,
    #[serde(rename = "Source")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<AwsCodeBuildProjectSource>,
    #[serde(rename = "VpcConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<AwsCodeBuildProjectVpcConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsCodeBuildProjectArtifactsDetails {
    #[serde(rename = "ArtifactIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_identifier: Option<String>,
    #[serde(rename = "EncryptionDisabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_disabled: Option<bool>,
    #[serde(rename = "Location")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "NamespaceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_type: Option<String>,
    #[serde(rename = "OverrideArtifactName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_artifact_name: Option<bool>,
    #[serde(rename = "Packaging")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packaging: Option<String>,
    #[serde(rename = "Path")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsCodeBuildProjectEnvironment {
    #[serde(rename = "Certificate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    #[serde(rename = "EnvironmentVariables")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_variables:
        Option<Vec<AwsCodeBuildProjectEnvironmentEnvironmentVariablesDetails>>,
    #[serde(rename = "ImagePullCredentialsType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_pull_credentials_type: Option<String>,
    #[serde(rename = "PrivilegedMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged_mode: Option<bool>,
    #[serde(rename = "RegistryCredential")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_credential: Option<AwsCodeBuildProjectEnvironmentRegistryCredential>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsCodeBuildProjectEnvironmentEnvironmentVariablesDetails {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsCodeBuildProjectEnvironmentRegistryCredential {
    #[serde(rename = "Credential")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential: Option<String>,
    #[serde(rename = "CredentialProvider")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_provider: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsCodeBuildProjectLogsConfigDetails {
    #[serde(rename = "CloudWatchLogs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs: Option<AwsCodeBuildProjectLogsConfigCloudWatchLogsDetails>,
    #[serde(rename = "S3Logs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_logs: Option<AwsCodeBuildProjectLogsConfigS3LogsDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsCodeBuildProjectLogsConfigCloudWatchLogsDetails {
    #[serde(rename = "GroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsCodeBuildProjectLogsConfigS3LogsDetails {
    #[serde(rename = "EncryptionDisabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_disabled: Option<bool>,
    #[serde(rename = "Location")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsCodeBuildProjectSource {
    #[serde(rename = "GitCloneDepth")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_clone_depth: Option<i32>,
    #[serde(rename = "InsecureSsl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insecure_ssl: Option<bool>,
    #[serde(rename = "Location")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsCodeBuildProjectVpcConfig {
    #[serde(rename = "SecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(rename = "Subnets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnets: Option<Vec<String>>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsDmsEndpointDetails {
    #[serde(rename = "CertificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(rename = "EndpointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_arn: Option<String>,
    #[serde(rename = "EndpointIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_identifier: Option<String>,
    #[serde(rename = "EndpointType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_type: Option<String>,
    #[serde(rename = "EngineName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_name: Option<String>,
    #[serde(rename = "ExternalId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(rename = "ExtraConnectionAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_connection_attributes: Option<String>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "ServerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    #[serde(rename = "SslMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_mode: Option<String>,
    #[serde(rename = "Username")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsDmsReplicationInstanceDetails {
    #[serde(rename = "AllocatedStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_storage: Option<i32>,
    #[serde(rename = "AutoMinorVersionUpgrade")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_minor_version_upgrade: Option<bool>,
    #[serde(rename = "AvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "MultiAZ")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_a_z: Option<bool>,
    #[serde(rename = "PreferredMaintenanceWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<String>,
    #[serde(rename = "PubliclyAccessible")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<bool>,
    #[serde(rename = "ReplicationInstanceClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance_class: Option<String>,
    #[serde(rename = "ReplicationInstanceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance_identifier: Option<String>,
    #[serde(rename = "ReplicationSubnetGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_subnet_group: Option<AwsDmsReplicationInstanceReplicationSubnetGroupDetails>,
    #[serde(rename = "VpcSecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_groups: Option<Vec<AwsDmsReplicationInstanceVpcSecurityGroupsDetails>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsDmsReplicationInstanceReplicationSubnetGroupDetails {
    #[serde(rename = "ReplicationSubnetGroupIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_subnet_group_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsDmsReplicationInstanceVpcSecurityGroupsDetails {
    #[serde(rename = "VpcSecurityGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_group_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsDmsReplicationTaskDetails {
    #[serde(rename = "CdcStartPosition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdc_start_position: Option<String>,
    #[serde(rename = "CdcStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdc_start_time: Option<String>,
    #[serde(rename = "CdcStopPosition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdc_stop_position: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "MigrationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migration_type: Option<String>,
    #[serde(rename = "ReplicationInstanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance_arn: Option<String>,
    #[serde(rename = "ReplicationTaskIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_identifier: Option<String>,
    #[serde(rename = "ReplicationTaskSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_settings: Option<String>,
    #[serde(rename = "ResourceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_identifier: Option<String>,
    #[serde(rename = "SourceEndpointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_endpoint_arn: Option<String>,
    #[serde(rename = "TableMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_mappings: Option<String>,
    #[serde(rename = "TargetEndpointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_endpoint_arn: Option<String>,
    #[serde(rename = "TaskData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_data: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsDynamoDbTableDetails {
    #[serde(rename = "AttributeDefinitions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_definitions: Option<Vec<AwsDynamoDbTableAttributeDefinition>>,
    #[serde(rename = "BillingModeSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_mode_summary: Option<AwsDynamoDbTableBillingModeSummary>,
    #[serde(rename = "CreationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<String>,
    #[serde(rename = "DeletionProtectionEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection_enabled: Option<bool>,
    #[serde(rename = "GlobalSecondaryIndexes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_secondary_indexes: Option<Vec<AwsDynamoDbTableGlobalSecondaryIndex>>,
    #[serde(rename = "GlobalTableVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_table_version: Option<String>,
    #[serde(rename = "ItemCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_count: Option<i32>,
    #[serde(rename = "KeySchema")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_schema: Option<Vec<AwsDynamoDbTableKeySchema>>,
    #[serde(rename = "LatestStreamArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_stream_arn: Option<String>,
    #[serde(rename = "LatestStreamLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_stream_label: Option<String>,
    #[serde(rename = "LocalSecondaryIndexes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_secondary_indexes: Option<Vec<AwsDynamoDbTableLocalSecondaryIndex>>,
    #[serde(rename = "ProvisionedThroughput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_throughput: Option<AwsDynamoDbTableProvisionedThroughput>,
    #[serde(rename = "Replicas")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicas: Option<Vec<AwsDynamoDbTableReplica>>,
    #[serde(rename = "RestoreSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_summary: Option<AwsDynamoDbTableRestoreSummary>,
    #[serde(rename = "SseDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sse_description: Option<AwsDynamoDbTableSseDescription>,
    #[serde(rename = "StreamSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_specification: Option<AwsDynamoDbTableStreamSpecification>,
    #[serde(rename = "TableId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_id: Option<String>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    #[serde(rename = "TableSizeBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_size_bytes: Option<i64>,
    #[serde(rename = "TableStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsDynamoDbTableAttributeDefinition {
    #[serde(rename = "AttributeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_name: Option<String>,
    #[serde(rename = "AttributeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsDynamoDbTableBillingModeSummary {
    #[serde(rename = "BillingMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_mode: Option<String>,
    #[serde(rename = "LastUpdateToPayPerRequestDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_to_pay_per_request_date_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsDynamoDbTableGlobalSecondaryIndex {
    #[serde(rename = "Backfilling")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backfilling: Option<bool>,
    #[serde(rename = "IndexArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_arn: Option<String>,
    #[serde(rename = "IndexName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,
    #[serde(rename = "IndexSizeBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_size_bytes: Option<i64>,
    #[serde(rename = "IndexStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_status: Option<String>,
    #[serde(rename = "ItemCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_count: Option<i32>,
    #[serde(rename = "KeySchema")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_schema: Option<Vec<AwsDynamoDbTableKeySchema>>,
    #[serde(rename = "Projection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projection: Option<AwsDynamoDbTableProjection>,
    #[serde(rename = "ProvisionedThroughput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_throughput: Option<AwsDynamoDbTableProvisionedThroughput>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsDynamoDbTableKeySchema {
    #[serde(rename = "AttributeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_name: Option<String>,
    #[serde(rename = "KeyType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsDynamoDbTableProjection {
    #[serde(rename = "NonKeyAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_key_attributes: Option<Vec<String>>,
    #[serde(rename = "ProjectionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projection_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsDynamoDbTableProvisionedThroughput {
    #[serde(rename = "LastDecreaseDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_decrease_date_time: Option<String>,
    #[serde(rename = "LastIncreaseDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_increase_date_time: Option<String>,
    #[serde(rename = "NumberOfDecreasesToday")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_decreases_today: Option<i32>,
    #[serde(rename = "ReadCapacityUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_capacity_units: Option<i32>,
    #[serde(rename = "WriteCapacityUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_capacity_units: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsDynamoDbTableLocalSecondaryIndex {
    #[serde(rename = "IndexArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_arn: Option<String>,
    #[serde(rename = "IndexName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,
    #[serde(rename = "KeySchema")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_schema: Option<Vec<AwsDynamoDbTableKeySchema>>,
    #[serde(rename = "Projection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projection: Option<AwsDynamoDbTableProjection>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsDynamoDbTableReplica {
    #[serde(rename = "GlobalSecondaryIndexes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_secondary_indexes: Option<Vec<AwsDynamoDbTableReplicaGlobalSecondaryIndex>>,
    #[serde(rename = "KmsMasterKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_master_key_id: Option<String>,
    #[serde(rename = "ProvisionedThroughputOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_throughput_override: Option<AwsDynamoDbTableProvisionedThroughputOverride>,
    #[serde(rename = "RegionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_name: Option<String>,
    #[serde(rename = "ReplicaStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_status: Option<String>,
    #[serde(rename = "ReplicaStatusDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_status_description: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsDynamoDbTableReplicaGlobalSecondaryIndex {
    #[serde(rename = "IndexName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,
    #[serde(rename = "ProvisionedThroughputOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_throughput_override: Option<AwsDynamoDbTableProvisionedThroughputOverride>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsDynamoDbTableProvisionedThroughputOverride {
    #[serde(rename = "ReadCapacityUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_capacity_units: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsDynamoDbTableRestoreSummary {
    #[serde(rename = "RestoreDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_date_time: Option<String>,
    #[serde(rename = "RestoreInProgress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_in_progress: Option<bool>,
    #[serde(rename = "SourceBackupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_backup_arn: Option<String>,
    #[serde(rename = "SourceTableArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_table_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsDynamoDbTableSseDescription {
    #[serde(rename = "InaccessibleEncryptionDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inaccessible_encryption_date_time: Option<String>,
    #[serde(rename = "KmsMasterKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_master_key_arn: Option<String>,
    #[serde(rename = "SseType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sse_type: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsDynamoDbTableStreamSpecification {
    #[serde(rename = "StreamEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_enabled: Option<bool>,
    #[serde(rename = "StreamViewType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_view_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2ClientVpnEndpointDetails {
    #[serde(rename = "AuthenticationOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_options: Option<Vec<AwsEc2ClientVpnEndpointAuthenticationOptionsDetails>>,
    #[serde(rename = "ClientCidrBlock")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_cidr_block: Option<String>,
    #[serde(rename = "ClientConnectOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_connect_options: Option<AwsEc2ClientVpnEndpointClientConnectOptionsDetails>,
    #[serde(rename = "ClientLoginBannerOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_login_banner_options: Option<AwsEc2ClientVpnEndpointClientLoginBannerOptionsDetails>,
    #[serde(rename = "ClientVpnEndpointId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_vpn_endpoint_id: Option<String>,
    #[serde(rename = "ConnectionLogOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_log_options: Option<AwsEc2ClientVpnEndpointConnectionLogOptionsDetails>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "DnsServer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_server: Option<Vec<String>>,
    #[serde(rename = "SecurityGroupIdSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_id_set: Option<Vec<String>>,
    #[serde(rename = "SelfServicePortalUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_service_portal_url: Option<String>,
    #[serde(rename = "ServerCertificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_certificate_arn: Option<String>,
    #[serde(rename = "SessionTimeoutHours")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_timeout_hours: Option<i32>,
    #[serde(rename = "SplitTunnel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split_tunnel: Option<bool>,
    #[serde(rename = "TransportProtocol")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_protocol: Option<String>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    #[serde(rename = "VpnPort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpn_port: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2ClientVpnEndpointAuthenticationOptionsDetails {
    #[serde(rename = "ActiveDirectory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_directory:
        Option<AwsEc2ClientVpnEndpointAuthenticationOptionsActiveDirectoryDetails>,
    #[serde(rename = "FederatedAuthentication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub federated_authentication:
        Option<AwsEc2ClientVpnEndpointAuthenticationOptionsFederatedAuthenticationDetails>,
    #[serde(rename = "MutualAuthentication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mutual_authentication:
        Option<AwsEc2ClientVpnEndpointAuthenticationOptionsMutualAuthenticationDetails>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2ClientVpnEndpointAuthenticationOptionsActiveDirectoryDetails {
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2ClientVpnEndpointAuthenticationOptionsFederatedAuthenticationDetails {
    #[serde(rename = "SamlProviderArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saml_provider_arn: Option<String>,
    #[serde(rename = "SelfServiceSamlProviderArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_service_saml_provider_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2ClientVpnEndpointAuthenticationOptionsMutualAuthenticationDetails {
    #[serde(rename = "ClientRootCertificateChain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_root_certificate_chain: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2ClientVpnEndpointClientConnectOptionsDetails {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "LambdaFunctionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_function_arn: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<AwsEc2ClientVpnEndpointClientConnectOptionsStatusDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2ClientVpnEndpointClientConnectOptionsStatusDetails {
    #[serde(rename = "Code")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2ClientVpnEndpointClientLoginBannerOptionsDetails {
    #[serde(rename = "BannerText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner_text: Option<String>,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2ClientVpnEndpointConnectionLogOptionsDetails {
    #[serde(rename = "CloudwatchLogGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloudwatch_log_group: Option<String>,
    #[serde(rename = "CloudwatchLogStream")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloudwatch_log_stream: Option<String>,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2EipDetails {
    #[serde(rename = "AllocationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocation_id: Option<String>,
    #[serde(rename = "AssociationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_id: Option<String>,
    #[serde(rename = "Domain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(rename = "NetworkBorderGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_border_group: Option<String>,
    #[serde(rename = "NetworkInterfaceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_id: Option<String>,
    #[serde(rename = "NetworkInterfaceOwnerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_owner_id: Option<String>,
    #[serde(rename = "PrivateIpAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<String>,
    #[serde(rename = "PublicIp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_ip: Option<String>,
    #[serde(rename = "PublicIpv4Pool")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_ipv4_pool: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2InstanceDetails {
    #[serde(rename = "IamInstanceProfileArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_instance_profile_arn: Option<String>,
    #[serde(rename = "ImageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    #[serde(rename = "IpV4Addresses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_v4_addresses: Option<Vec<String>>,
    #[serde(rename = "IpV6Addresses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_v6_addresses: Option<Vec<String>>,
    #[serde(rename = "KeyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_name: Option<String>,
    #[serde(rename = "LaunchedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launched_at: Option<String>,
    #[serde(rename = "MetadataOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_options: Option<AwsEc2InstanceMetadataOptions>,
    #[serde(rename = "Monitoring")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring: Option<AwsEc2InstanceMonitoringDetails>,
    #[serde(rename = "NetworkInterfaces")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interfaces: Option<Vec<AwsEc2InstanceNetworkInterfacesDetails>>,
    #[serde(rename = "SubnetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "VirtualizationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtualization_type: Option<String>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2InstanceMetadataOptions {
    #[serde(rename = "HttpEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_endpoint: Option<String>,
    #[serde(rename = "HttpProtocolIpv6")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_protocol_ipv6: Option<String>,
    #[serde(rename = "HttpPutResponseHopLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_put_response_hop_limit: Option<i32>,
    #[serde(rename = "HttpTokens")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_tokens: Option<String>,
    #[serde(rename = "InstanceMetadataTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_metadata_tags: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2InstanceMonitoringDetails {
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2InstanceNetworkInterfacesDetails {
    #[serde(rename = "NetworkInterfaceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2LaunchTemplateDetails {
    #[serde(rename = "DefaultVersionNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_version_number: Option<i64>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LatestVersionNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_number: Option<i64>,
    #[serde(rename = "LaunchTemplateData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_template_data: Option<AwsEc2LaunchTemplateDataDetails>,
    #[serde(rename = "LaunchTemplateName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_template_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2LaunchTemplateDataDetails {
    #[serde(rename = "BlockDeviceMappingSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_device_mapping_set: Option<Vec<AwsEc2LaunchTemplateDataBlockDeviceMappingSetDetails>>,
    #[serde(rename = "CapacityReservationSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_reservation_specification:
        Option<AwsEc2LaunchTemplateDataCapacityReservationSpecificationDetails>,
    #[serde(rename = "CpuOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_options: Option<AwsEc2LaunchTemplateDataCpuOptionsDetails>,
    #[serde(rename = "CreditSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_specification: Option<AwsEc2LaunchTemplateDataCreditSpecificationDetails>,
    #[serde(rename = "DisableApiStop")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_api_stop: Option<bool>,
    #[serde(rename = "DisableApiTermination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_api_termination: Option<bool>,
    #[serde(rename = "EbsOptimized")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_optimized: Option<bool>,
    #[serde(rename = "ElasticGpuSpecificationSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elastic_gpu_specification_set:
        Option<Vec<AwsEc2LaunchTemplateDataElasticGpuSpecificationSetDetails>>,
    #[serde(rename = "ElasticInferenceAcceleratorSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elastic_inference_accelerator_set:
        Option<Vec<AwsEc2LaunchTemplateDataElasticInferenceAcceleratorSetDetails>>,
    #[serde(rename = "EnclaveOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enclave_options: Option<AwsEc2LaunchTemplateDataEnclaveOptionsDetails>,
    #[serde(rename = "HibernationOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hibernation_options: Option<AwsEc2LaunchTemplateDataHibernationOptionsDetails>,
    #[serde(rename = "IamInstanceProfile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_instance_profile: Option<AwsEc2LaunchTemplateDataIamInstanceProfileDetails>,
    #[serde(rename = "ImageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    #[serde(rename = "InstanceInitiatedShutdownBehavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_initiated_shutdown_behavior: Option<String>,
    #[serde(rename = "InstanceMarketOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_market_options: Option<AwsEc2LaunchTemplateDataInstanceMarketOptionsDetails>,
    #[serde(rename = "InstanceRequirements")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_requirements: Option<AwsEc2LaunchTemplateDataInstanceRequirementsDetails>,
    #[serde(rename = "InstanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    #[serde(rename = "KernelId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kernel_id: Option<String>,
    #[serde(rename = "KeyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_name: Option<String>,
    #[serde(rename = "LicenseSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_set: Option<Vec<AwsEc2LaunchTemplateDataLicenseSetDetails>>,
    #[serde(rename = "MaintenanceOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_options: Option<AwsEc2LaunchTemplateDataMaintenanceOptionsDetails>,
    #[serde(rename = "MetadataOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_options: Option<AwsEc2LaunchTemplateDataMetadataOptionsDetails>,
    #[serde(rename = "Monitoring")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring: Option<AwsEc2LaunchTemplateDataMonitoringDetails>,
    #[serde(rename = "NetworkInterfaceSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_set: Option<Vec<AwsEc2LaunchTemplateDataNetworkInterfaceSetDetails>>,
    #[serde(rename = "Placement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement: Option<AwsEc2LaunchTemplateDataPlacementDetails>,
    #[serde(rename = "PrivateDnsNameOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_dns_name_options: Option<AwsEc2LaunchTemplateDataPrivateDnsNameOptionsDetails>,
    #[serde(rename = "RamDiskId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ram_disk_id: Option<String>,
    #[serde(rename = "SecurityGroupIdSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_id_set: Option<Vec<String>>,
    #[serde(rename = "SecurityGroupSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_set: Option<Vec<String>>,
    #[serde(rename = "UserData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2LaunchTemplateDataBlockDeviceMappingSetDetails {
    #[serde(rename = "DeviceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    #[serde(rename = "Ebs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs: Option<AwsEc2LaunchTemplateDataBlockDeviceMappingSetEbsDetails>,
    #[serde(rename = "NoDevice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_device: Option<String>,
    #[serde(rename = "VirtualName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2LaunchTemplateDataBlockDeviceMappingSetEbsDetails {
    #[serde(rename = "DeleteOnTermination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_on_termination: Option<bool>,
    #[serde(rename = "Encrypted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    #[serde(rename = "Iops")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iops: Option<i32>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "SnapshotId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
    #[serde(rename = "Throughput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throughput: Option<i32>,
    #[serde(rename = "VolumeSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_size: Option<i32>,
    #[serde(rename = "VolumeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2LaunchTemplateDataCapacityReservationSpecificationDetails {
    #[serde(rename = "CapacityReservationPreference")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_reservation_preference: Option<String>,
    #[serde(rename = "CapacityReservationTarget")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_reservation_target: Option<
        AwsEc2LaunchTemplateDataCapacityReservationSpecificationCapacityReservationTargetDetails,
    >,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2LaunchTemplateDataCapacityReservationSpecificationCapacityReservationTargetDetails
{
    #[serde(rename = "CapacityReservationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_reservation_id: Option<String>,
    #[serde(rename = "CapacityReservationResourceGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_reservation_resource_group_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2LaunchTemplateDataCpuOptionsDetails {
    #[serde(rename = "CoreCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_count: Option<i32>,
    #[serde(rename = "ThreadsPerCore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threads_per_core: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2LaunchTemplateDataCreditSpecificationDetails {
    #[serde(rename = "CpuCredits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_credits: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2LaunchTemplateDataElasticGpuSpecificationSetDetails {
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2LaunchTemplateDataElasticInferenceAcceleratorSetDetails {
    #[serde(rename = "Count")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2LaunchTemplateDataEnclaveOptionsDetails {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2LaunchTemplateDataHibernationOptionsDetails {
    #[serde(rename = "Configured")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configured: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2LaunchTemplateDataIamInstanceProfileDetails {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2LaunchTemplateDataInstanceMarketOptionsDetails {
    #[serde(rename = "MarketType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_type: Option<String>,
    #[serde(rename = "SpotOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_options: Option<AwsEc2LaunchTemplateDataInstanceMarketOptionsSpotOptionsDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2LaunchTemplateDataInstanceMarketOptionsSpotOptionsDetails {
    #[serde(rename = "BlockDurationMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_duration_minutes: Option<i32>,
    #[serde(rename = "InstanceInterruptionBehavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_interruption_behavior: Option<String>,
    #[serde(rename = "MaxPrice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_price: Option<String>,
    #[serde(rename = "SpotInstanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_instance_type: Option<String>,
    #[serde(rename = "ValidUntil")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_until: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2LaunchTemplateDataInstanceRequirementsDetails {
    #[serde(rename = "AcceleratorCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accelerator_count:
        Option<AwsEc2LaunchTemplateDataInstanceRequirementsAcceleratorCountDetails>,
    #[serde(rename = "AcceleratorManufacturers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accelerator_manufacturers: Option<Vec<String>>,
    #[serde(rename = "AcceleratorNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accelerator_names: Option<Vec<String>>,
    #[serde(rename = "AcceleratorTotalMemoryMiB")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accelerator_total_memory_mi_b:
        Option<AwsEc2LaunchTemplateDataInstanceRequirementsAcceleratorTotalMemoryMiBDetails>,
    #[serde(rename = "AcceleratorTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accelerator_types: Option<Vec<String>>,
    #[serde(rename = "BareMetal")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bare_metal: Option<String>,
    #[serde(rename = "BaselineEbsBandwidthMbps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_ebs_bandwidth_mbps:
        Option<AwsEc2LaunchTemplateDataInstanceRequirementsBaselineEbsBandwidthMbpsDetails>,
    #[serde(rename = "BurstablePerformance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub burstable_performance: Option<String>,
    #[serde(rename = "CpuManufacturers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_manufacturers: Option<Vec<String>>,
    #[serde(rename = "ExcludedInstanceTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_instance_types: Option<Vec<String>>,
    #[serde(rename = "InstanceGenerations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_generations: Option<Vec<String>>,
    #[serde(rename = "LocalStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_storage: Option<String>,
    #[serde(rename = "LocalStorageTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_storage_types: Option<Vec<String>>,
    #[serde(rename = "MemoryGiBPerVCpu")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_gi_b_per_v_cpu:
        Option<AwsEc2LaunchTemplateDataInstanceRequirementsMemoryGiBPerVCpuDetails>,
    #[serde(rename = "MemoryMiB")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_mi_b: Option<AwsEc2LaunchTemplateDataInstanceRequirementsMemoryMiBDetails>,
    #[serde(rename = "NetworkInterfaceCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_count:
        Option<AwsEc2LaunchTemplateDataInstanceRequirementsNetworkInterfaceCountDetails>,
    #[serde(rename = "OnDemandMaxPricePercentageOverLowestPrice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_demand_max_price_percentage_over_lowest_price: Option<i32>,
    #[serde(rename = "RequireHibernateSupport")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_hibernate_support: Option<bool>,
    #[serde(rename = "SpotMaxPricePercentageOverLowestPrice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_max_price_percentage_over_lowest_price: Option<i32>,
    #[serde(rename = "TotalLocalStorageGB")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_local_storage_g_b:
        Option<AwsEc2LaunchTemplateDataInstanceRequirementsTotalLocalStorageGBDetails>,
    #[serde(rename = "VCpuCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_cpu_count: Option<AwsEc2LaunchTemplateDataInstanceRequirementsVCpuCountDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2LaunchTemplateDataInstanceRequirementsAcceleratorCountDetails {
    #[serde(rename = "Max")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<i32>,
    #[serde(rename = "Min")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2LaunchTemplateDataInstanceRequirementsAcceleratorTotalMemoryMiBDetails {
    #[serde(rename = "Max")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<i32>,
    #[serde(rename = "Min")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2LaunchTemplateDataInstanceRequirementsBaselineEbsBandwidthMbpsDetails {
    #[serde(rename = "Max")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<i32>,
    #[serde(rename = "Min")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2LaunchTemplateDataInstanceRequirementsMemoryGiBPerVCpuDetails {
    #[serde(rename = "Max")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    #[serde(rename = "Min")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2LaunchTemplateDataInstanceRequirementsMemoryMiBDetails {
    #[serde(rename = "Max")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<i32>,
    #[serde(rename = "Min")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2LaunchTemplateDataInstanceRequirementsNetworkInterfaceCountDetails {
    #[serde(rename = "Max")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<i32>,
    #[serde(rename = "Min")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2LaunchTemplateDataInstanceRequirementsTotalLocalStorageGBDetails {
    #[serde(rename = "Max")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    #[serde(rename = "Min")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2LaunchTemplateDataInstanceRequirementsVCpuCountDetails {
    #[serde(rename = "Max")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<i32>,
    #[serde(rename = "Min")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2LaunchTemplateDataLicenseSetDetails {
    #[serde(rename = "LicenseConfigurationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_configuration_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2LaunchTemplateDataMaintenanceOptionsDetails {
    #[serde(rename = "AutoRecovery")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_recovery: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2LaunchTemplateDataMetadataOptionsDetails {
    #[serde(rename = "HttpEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_endpoint: Option<String>,
    #[serde(rename = "HttpProtocolIpv6")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_protocol_ipv6: Option<String>,
    #[serde(rename = "HttpPutResponseHopLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_put_response_hop_limit: Option<i32>,
    #[serde(rename = "HttpTokens")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_tokens: Option<String>,
    #[serde(rename = "InstanceMetadataTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_metadata_tags: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2LaunchTemplateDataMonitoringDetails {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2LaunchTemplateDataNetworkInterfaceSetDetails {
    #[serde(rename = "AssociateCarrierIpAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associate_carrier_ip_address: Option<bool>,
    #[serde(rename = "AssociatePublicIpAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associate_public_ip_address: Option<bool>,
    #[serde(rename = "DeleteOnTermination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_on_termination: Option<bool>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "DeviceIndex")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_index: Option<i32>,
    #[serde(rename = "Groups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
    #[serde(rename = "InterfaceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interface_type: Option<String>,
    #[serde(rename = "Ipv4PrefixCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv4_prefix_count: Option<i32>,
    #[serde(rename = "Ipv4Prefixes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv4_prefixes: Option<Vec<AwsEc2LaunchTemplateDataNetworkInterfaceSetIpv4PrefixesDetails>>,
    #[serde(rename = "Ipv6AddressCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_address_count: Option<i32>,
    #[serde(rename = "Ipv6Addresses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_addresses:
        Option<Vec<AwsEc2LaunchTemplateDataNetworkInterfaceSetIpv6AddressesDetails>>,
    #[serde(rename = "Ipv6PrefixCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_prefix_count: Option<i32>,
    #[serde(rename = "Ipv6Prefixes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_prefixes: Option<Vec<AwsEc2LaunchTemplateDataNetworkInterfaceSetIpv6PrefixesDetails>>,
    #[serde(rename = "NetworkCardIndex")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_card_index: Option<i32>,
    #[serde(rename = "NetworkInterfaceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_id: Option<String>,
    #[serde(rename = "PrivateIpAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<String>,
    #[serde(rename = "PrivateIpAddresses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_ip_addresses:
        Option<Vec<AwsEc2LaunchTemplateDataNetworkInterfaceSetPrivateIpAddressesDetails>>,
    #[serde(rename = "SecondaryPrivateIpAddressCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_private_ip_address_count: Option<i32>,
    #[serde(rename = "SubnetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2LaunchTemplateDataNetworkInterfaceSetIpv4PrefixesDetails {
    #[serde(rename = "Ipv4Prefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv4_prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2LaunchTemplateDataNetworkInterfaceSetIpv6AddressesDetails {
    #[serde(rename = "Ipv6Address")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_address: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2LaunchTemplateDataNetworkInterfaceSetIpv6PrefixesDetails {
    #[serde(rename = "Ipv6Prefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2LaunchTemplateDataNetworkInterfaceSetPrivateIpAddressesDetails {
    #[serde(rename = "Primary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
    #[serde(rename = "PrivateIpAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2LaunchTemplateDataPlacementDetails {
    #[serde(rename = "Affinity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affinity: Option<String>,
    #[serde(rename = "AvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "GroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(rename = "HostId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_id: Option<String>,
    #[serde(rename = "HostResourceGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_resource_group_arn: Option<String>,
    #[serde(rename = "PartitionNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_number: Option<i32>,
    #[serde(rename = "SpreadDomain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spread_domain: Option<String>,
    #[serde(rename = "Tenancy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenancy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2LaunchTemplateDataPrivateDnsNameOptionsDetails {
    #[serde(rename = "EnableResourceNameDnsAAAARecord")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_resource_name_dns_a_a_a_a_record: Option<bool>,
    #[serde(rename = "EnableResourceNameDnsARecord")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_resource_name_dns_a_record: Option<bool>,
    #[serde(rename = "HostnameType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2NetworkAclDetails {
    #[serde(rename = "Associations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associations: Option<Vec<AwsEc2NetworkAclAssociation>>,
    #[serde(rename = "Entries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<AwsEc2NetworkAclEntry>>,
    #[serde(rename = "IsDefault")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[serde(rename = "NetworkAclId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_acl_id: Option<String>,
    #[serde(rename = "OwnerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2NetworkAclAssociation {
    #[serde(rename = "NetworkAclAssociationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_acl_association_id: Option<String>,
    #[serde(rename = "NetworkAclId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_acl_id: Option<String>,
    #[serde(rename = "SubnetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2NetworkAclEntry {
    #[serde(rename = "CidrBlock")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_block: Option<String>,
    #[serde(rename = "Egress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress: Option<bool>,
    #[serde(rename = "IcmpTypeCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icmp_type_code: Option<IcmpTypeCode>,
    #[serde(rename = "Ipv6CidrBlock")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_cidr_block: Option<String>,
    #[serde(rename = "PortRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_range: Option<PortRangeFromTo>,
    #[serde(rename = "Protocol")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(rename = "RuleAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_action: Option<String>,
    #[serde(rename = "RuleNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_number: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IcmpTypeCode {
    #[serde(rename = "Code")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PortRangeFromTo {
    #[serde(rename = "From")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i32>,
    #[serde(rename = "To")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2NetworkInterfaceDetails {
    #[serde(rename = "Attachment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment: Option<AwsEc2NetworkInterfaceAttachment>,
    #[serde(rename = "IpV6Addresses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_v6_addresses: Option<Vec<AwsEc2NetworkInterfaceIpV6AddressDetail>>,
    #[serde(rename = "NetworkInterfaceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_id: Option<String>,
    #[serde(rename = "PrivateIpAddresses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_ip_addresses: Option<Vec<AwsEc2NetworkInterfacePrivateIpAddressDetail>>,
    #[serde(rename = "PublicDnsName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_dns_name: Option<String>,
    #[serde(rename = "PublicIp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_ip: Option<String>,
    #[serde(rename = "SecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<AwsEc2NetworkInterfaceSecurityGroup>>,
    #[serde(rename = "SourceDestCheck")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_dest_check: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2NetworkInterfaceAttachment {
    #[serde(rename = "AttachTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_time: Option<String>,
    #[serde(rename = "AttachmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_id: Option<String>,
    #[serde(rename = "DeleteOnTermination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_on_termination: Option<bool>,
    #[serde(rename = "DeviceIndex")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_index: Option<i32>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(rename = "InstanceOwnerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_owner_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2NetworkInterfaceIpV6AddressDetail {
    #[serde(rename = "IpV6Address")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_v6_address: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2NetworkInterfacePrivateIpAddressDetail {
    #[serde(rename = "PrivateDnsName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_dns_name: Option<String>,
    #[serde(rename = "PrivateIpAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2NetworkInterfaceSecurityGroup {
    #[serde(rename = "GroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(rename = "GroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2RouteTableDetails {
    #[serde(rename = "AssociationSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_set: Option<Vec<AssociationSetDetails>>,
    #[serde(rename = "OwnerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(rename = "PropagatingVgwSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub propagating_vgw_set: Option<Vec<PropagatingVgwSetDetails>>,
    #[serde(rename = "RouteSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_set: Option<Vec<RouteSetDetails>>,
    #[serde(rename = "RouteTableId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_table_id: Option<String>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociationSetDetails {
    #[serde(rename = "AssociationState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_state: Option<AssociationStateDetails>,
    #[serde(rename = "GatewayId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_id: Option<String>,
    #[serde(rename = "Main")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main: Option<bool>,
    #[serde(rename = "RouteTableAssociationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_table_association_id: Option<String>,
    #[serde(rename = "RouteTableId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_table_id: Option<String>,
    #[serde(rename = "SubnetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociationStateDetails {
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PropagatingVgwSetDetails {
    #[serde(rename = "GatewayId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RouteSetDetails {
    #[serde(rename = "CarrierGatewayId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier_gateway_id: Option<String>,
    #[serde(rename = "CoreNetworkArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_network_arn: Option<String>,
    #[serde(rename = "DestinationCidrBlock")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_cidr_block: Option<String>,
    #[serde(rename = "DestinationIpv6CidrBlock")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_ipv6_cidr_block: Option<String>,
    #[serde(rename = "DestinationPrefixListId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_prefix_list_id: Option<String>,
    #[serde(rename = "EgressOnlyInternetGatewayId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress_only_internet_gateway_id: Option<String>,
    #[serde(rename = "GatewayId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_id: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(rename = "InstanceOwnerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_owner_id: Option<String>,
    #[serde(rename = "LocalGatewayId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_gateway_id: Option<String>,
    #[serde(rename = "NatGatewayId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nat_gateway_id: Option<String>,
    #[serde(rename = "NetworkInterfaceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_id: Option<String>,
    #[serde(rename = "Origin")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "TransitGatewayId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_gateway_id: Option<String>,
    #[serde(rename = "VpcPeeringConnectionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_peering_connection_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2SecurityGroupDetails {
    #[serde(rename = "GroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(rename = "GroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(rename = "IpPermissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_permissions: Option<Vec<AwsEc2SecurityGroupIpPermission>>,
    #[serde(rename = "IpPermissionsEgress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_permissions_egress: Option<Vec<AwsEc2SecurityGroupIpPermission>>,
    #[serde(rename = "OwnerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2SecurityGroupIpPermission {
    #[serde(rename = "FromPort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_port: Option<i32>,
    #[serde(rename = "IpProtocol")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_protocol: Option<String>,
    #[serde(rename = "IpRanges")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_ranges: Option<Vec<AwsEc2SecurityGroupIpRange>>,
    #[serde(rename = "Ipv6Ranges")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_ranges: Option<Vec<AwsEc2SecurityGroupIpv6Range>>,
    #[serde(rename = "PrefixListIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix_list_ids: Option<Vec<AwsEc2SecurityGroupPrefixListId>>,
    #[serde(rename = "ToPort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_port: Option<i32>,
    #[serde(rename = "UserIdGroupPairs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_group_pairs: Option<Vec<AwsEc2SecurityGroupUserIdGroupPair>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2SecurityGroupIpRange {
    #[serde(rename = "CidrIp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_ip: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2SecurityGroupIpv6Range {
    #[serde(rename = "CidrIpv6")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_ipv6: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2SecurityGroupPrefixListId {
    #[serde(rename = "PrefixListId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix_list_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2SecurityGroupUserIdGroupPair {
    #[serde(rename = "GroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(rename = "GroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(rename = "PeeringStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peering_status: Option<String>,
    #[serde(rename = "UserId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    #[serde(rename = "VpcPeeringConnectionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_peering_connection_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2SubnetDetails {
    #[serde(rename = "AssignIpv6AddressOnCreation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assign_ipv6_address_on_creation: Option<bool>,
    #[serde(rename = "AvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "AvailabilityZoneId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone_id: Option<String>,
    #[serde(rename = "AvailableIpAddressCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_ip_address_count: Option<i32>,
    #[serde(rename = "CidrBlock")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_block: Option<String>,
    #[serde(rename = "DefaultForAz")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_for_az: Option<bool>,
    #[serde(rename = "Ipv6CidrBlockAssociationSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_cidr_block_association_set: Option<Vec<Ipv6CidrBlockAssociation>>,
    #[serde(rename = "MapPublicIpOnLaunch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map_public_ip_on_launch: Option<bool>,
    #[serde(rename = "OwnerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "SubnetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_arn: Option<String>,
    #[serde(rename = "SubnetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Ipv6CidrBlockAssociation {
    #[serde(rename = "AssociationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_id: Option<String>,
    #[serde(rename = "CidrBlockState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_block_state: Option<String>,
    #[serde(rename = "Ipv6CidrBlock")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_cidr_block: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2TransitGatewayDetails {
    #[serde(rename = "AmazonSideAsn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_side_asn: Option<i32>,
    #[serde(rename = "AssociationDefaultRouteTableId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_default_route_table_id: Option<String>,
    #[serde(rename = "AutoAcceptSharedAttachments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_accept_shared_attachments: Option<String>,
    #[serde(rename = "DefaultRouteTableAssociation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_route_table_association: Option<String>,
    #[serde(rename = "DefaultRouteTablePropagation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_route_table_propagation: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "DnsSupport")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_support: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "MulticastSupport")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multicast_support: Option<String>,
    #[serde(rename = "PropagationDefaultRouteTableId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub propagation_default_route_table_id: Option<String>,
    #[serde(rename = "TransitGatewayCidrBlocks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_gateway_cidr_blocks: Option<Vec<String>>,
    #[serde(rename = "VpnEcmpSupport")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpn_ecmp_support: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2VolumeDetails {
    #[serde(rename = "Attachments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<AwsEc2VolumeAttachment>>,
    #[serde(rename = "CreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(rename = "DeviceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    #[serde(rename = "Encrypted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "Size")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(rename = "SnapshotId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "VolumeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_id: Option<String>,
    #[serde(rename = "VolumeScanStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_scan_status: Option<String>,
    #[serde(rename = "VolumeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2VolumeAttachment {
    #[serde(rename = "AttachTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_time: Option<String>,
    #[serde(rename = "DeleteOnTermination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_on_termination: Option<bool>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2VpcDetails {
    #[serde(rename = "CidrBlockAssociationSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_block_association_set: Option<Vec<CidrBlockAssociation>>,
    #[serde(rename = "DhcpOptionsId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dhcp_options_id: Option<String>,
    #[serde(rename = "Ipv6CidrBlockAssociationSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_cidr_block_association_set: Option<Vec<Ipv6CidrBlockAssociation>>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CidrBlockAssociation {
    #[serde(rename = "AssociationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_id: Option<String>,
    #[serde(rename = "CidrBlock")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_block: Option<String>,
    #[serde(rename = "CidrBlockState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_block_state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2VpcEndpointServiceDetails {
    #[serde(rename = "AcceptanceRequired")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acceptance_required: Option<bool>,
    #[serde(rename = "AvailabilityZones")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<Vec<String>>,
    #[serde(rename = "BaseEndpointDnsNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_endpoint_dns_names: Option<Vec<String>>,
    #[serde(rename = "GatewayLoadBalancerArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_load_balancer_arns: Option<Vec<String>>,
    #[serde(rename = "ManagesVpcEndpoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manages_vpc_endpoints: Option<bool>,
    #[serde(rename = "NetworkLoadBalancerArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_load_balancer_arns: Option<Vec<String>>,
    #[serde(rename = "PrivateDnsName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_dns_name: Option<String>,
    #[serde(rename = "ServiceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_id: Option<String>,
    #[serde(rename = "ServiceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    #[serde(rename = "ServiceState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_state: Option<String>,
    #[serde(rename = "ServiceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_type: Option<Vec<AwsEc2VpcEndpointServiceServiceTypeDetails>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2VpcEndpointServiceServiceTypeDetails {
    #[serde(rename = "ServiceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2VpcPeeringConnectionDetails {
    #[serde(rename = "AccepterVpcInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepter_vpc_info: Option<AwsEc2VpcPeeringConnectionVpcInfoDetails>,
    #[serde(rename = "ExpirationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    #[serde(rename = "RequesterVpcInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requester_vpc_info: Option<AwsEc2VpcPeeringConnectionVpcInfoDetails>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<AwsEc2VpcPeeringConnectionStatusDetails>,
    #[serde(rename = "VpcPeeringConnectionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_peering_connection_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2VpcPeeringConnectionVpcInfoDetails {
    #[serde(rename = "CidrBlock")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_block: Option<String>,
    #[serde(rename = "CidrBlockSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_block_set: Option<Vec<VpcInfoCidrBlockSetDetails>>,
    #[serde(rename = "Ipv6CidrBlockSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_cidr_block_set: Option<Vec<VpcInfoIpv6CidrBlockSetDetails>>,
    #[serde(rename = "OwnerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(rename = "PeeringOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peering_options: Option<VpcInfoPeeringOptionsDetails>,
    #[serde(rename = "Region")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VpcInfoCidrBlockSetDetails {
    #[serde(rename = "CidrBlock")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_block: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VpcInfoIpv6CidrBlockSetDetails {
    #[serde(rename = "Ipv6CidrBlock")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_cidr_block: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VpcInfoPeeringOptionsDetails {
    #[serde(rename = "AllowDnsResolutionFromRemoteVpc")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_dns_resolution_from_remote_vpc: Option<bool>,
    #[serde(rename = "AllowEgressFromLocalClassicLinkToRemoteVpc")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_egress_from_local_classic_link_to_remote_vpc: Option<bool>,
    #[serde(rename = "AllowEgressFromLocalVpcToRemoteClassicLink")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_egress_from_local_vpc_to_remote_classic_link: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2VpcPeeringConnectionStatusDetails {
    #[serde(rename = "Code")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2VpnConnectionDetails {
    #[serde(rename = "Category")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "CustomerGatewayConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_gateway_configuration: Option<String>,
    #[serde(rename = "CustomerGatewayId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_gateway_id: Option<String>,
    #[serde(rename = "Options")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<AwsEc2VpnConnectionOptionsDetails>,
    #[serde(rename = "Routes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routes: Option<Vec<AwsEc2VpnConnectionRoutesDetails>>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "TransitGatewayId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_gateway_id: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "VgwTelemetry")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vgw_telemetry: Option<Vec<AwsEc2VpnConnectionVgwTelemetryDetails>>,
    #[serde(rename = "VpnConnectionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpn_connection_id: Option<String>,
    #[serde(rename = "VpnGatewayId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpn_gateway_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2VpnConnectionOptionsDetails {
    #[serde(rename = "StaticRoutesOnly")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_routes_only: Option<bool>,
    #[serde(rename = "TunnelOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tunnel_options: Option<Vec<AwsEc2VpnConnectionOptionsTunnelOptionsDetails>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2VpnConnectionOptionsTunnelOptionsDetails {
    #[serde(rename = "DpdTimeoutSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dpd_timeout_seconds: Option<i32>,
    #[serde(rename = "IkeVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ike_versions: Option<Vec<String>>,
    #[serde(rename = "OutsideIpAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outside_ip_address: Option<String>,
    #[serde(rename = "Phase1DhGroupNumbers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase1_dh_group_numbers: Option<Vec<i32>>,
    #[serde(rename = "Phase1EncryptionAlgorithms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase1_encryption_algorithms: Option<Vec<String>>,
    #[serde(rename = "Phase1IntegrityAlgorithms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase1_integrity_algorithms: Option<Vec<String>>,
    #[serde(rename = "Phase1LifetimeSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase1_lifetime_seconds: Option<i32>,
    #[serde(rename = "Phase2DhGroupNumbers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase2_dh_group_numbers: Option<Vec<i32>>,
    #[serde(rename = "Phase2EncryptionAlgorithms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase2_encryption_algorithms: Option<Vec<String>>,
    #[serde(rename = "Phase2IntegrityAlgorithms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase2_integrity_algorithms: Option<Vec<String>>,
    #[serde(rename = "Phase2LifetimeSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase2_lifetime_seconds: Option<i32>,
    #[serde(rename = "PreSharedKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_shared_key: Option<String>,
    #[serde(rename = "RekeyFuzzPercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rekey_fuzz_percentage: Option<i32>,
    #[serde(rename = "RekeyMarginTimeSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rekey_margin_time_seconds: Option<i32>,
    #[serde(rename = "ReplayWindowSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replay_window_size: Option<i32>,
    #[serde(rename = "TunnelInsideCidr")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tunnel_inside_cidr: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2VpnConnectionRoutesDetails {
    #[serde(rename = "DestinationCidrBlock")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_cidr_block: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEc2VpnConnectionVgwTelemetryDetails {
    #[serde(rename = "AcceptedRouteCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepted_route_count: Option<i32>,
    #[serde(rename = "CertificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    #[serde(rename = "LastStatusChange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_status_change: Option<String>,
    #[serde(rename = "OutsideIpAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outside_ip_address: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEcrContainerImageDetails {
    #[serde(rename = "Architecture")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    #[serde(rename = "ImageDigest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_digest: Option<String>,
    #[serde(rename = "ImagePublishedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_published_at: Option<String>,
    #[serde(rename = "ImageTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_tags: Option<Vec<String>>,
    #[serde(rename = "RegistryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    #[serde(rename = "RepositoryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEcrRepositoryDetails {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "ImageScanningConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_scanning_configuration: Option<AwsEcrRepositoryImageScanningConfigurationDetails>,
    #[serde(rename = "ImageTagMutability")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_tag_mutability: Option<String>,
    #[serde(rename = "LifecyclePolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_policy: Option<AwsEcrRepositoryLifecyclePolicyDetails>,
    #[serde(rename = "RepositoryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
    #[serde(rename = "RepositoryPolicyText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_policy_text: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEcrRepositoryImageScanningConfigurationDetails {
    #[serde(rename = "ScanOnPush")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_on_push: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEcrRepositoryLifecyclePolicyDetails {
    #[serde(rename = "LifecyclePolicyText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_policy_text: Option<String>,
    #[serde(rename = "RegistryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEcsClusterDetails {
    #[serde(rename = "ActiveServicesCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_services_count: Option<i32>,
    #[serde(rename = "CapacityProviders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_providers: Option<Vec<String>>,
    #[serde(rename = "ClusterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    #[serde(rename = "ClusterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    #[serde(rename = "ClusterSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_settings: Option<Vec<AwsEcsClusterClusterSettingsDetails>>,
    #[serde(rename = "Configuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<AwsEcsClusterConfigurationDetails>,
    #[serde(rename = "DefaultCapacityProviderStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_capacity_provider_strategy:
        Option<Vec<AwsEcsClusterDefaultCapacityProviderStrategyDetails>>,
    #[serde(rename = "RegisteredContainerInstancesCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_container_instances_count: Option<i32>,
    #[serde(rename = "RunningTasksCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running_tasks_count: Option<i32>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEcsClusterClusterSettingsDetails {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEcsClusterConfigurationDetails {
    #[serde(rename = "ExecuteCommandConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execute_command_configuration:
        Option<AwsEcsClusterConfigurationExecuteCommandConfigurationDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEcsClusterConfigurationExecuteCommandConfigurationDetails {
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "LogConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_configuration:
        Option<AwsEcsClusterConfigurationExecuteCommandConfigurationLogConfigurationDetails>,
    #[serde(rename = "Logging")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEcsClusterConfigurationExecuteCommandConfigurationLogConfigurationDetails {
    #[serde(rename = "CloudWatchEncryptionEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_encryption_enabled: Option<bool>,
    #[serde(rename = "CloudWatchLogGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_log_group_name: Option<String>,
    #[serde(rename = "S3BucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_name: Option<String>,
    #[serde(rename = "S3EncryptionEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_encryption_enabled: Option<bool>,
    #[serde(rename = "S3KeyPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key_prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEcsClusterDefaultCapacityProviderStrategyDetails {
    #[serde(rename = "Base")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base: Option<i32>,
    #[serde(rename = "CapacityProvider")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_provider: Option<String>,
    #[serde(rename = "Weight")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEcsContainerDetails {
    #[serde(rename = "Image")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(rename = "MountPoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_points: Option<Vec<AwsMountPoint>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Privileged")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsMountPoint {
    #[serde(rename = "ContainerPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_path: Option<String>,
    #[serde(rename = "SourceVolume")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_volume: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEcsServiceDetails {
    #[serde(rename = "CapacityProviderStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_provider_strategy: Option<Vec<AwsEcsServiceCapacityProviderStrategyDetails>>,
    #[serde(rename = "Cluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    #[serde(rename = "DeploymentConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_configuration: Option<AwsEcsServiceDeploymentConfigurationDetails>,
    #[serde(rename = "DeploymentController")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_controller: Option<AwsEcsServiceDeploymentControllerDetails>,
    #[serde(rename = "DesiredCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_count: Option<i32>,
    #[serde(rename = "EnableEcsManagedTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_ecs_managed_tags: Option<bool>,
    #[serde(rename = "EnableExecuteCommand")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_execute_command: Option<bool>,
    #[serde(rename = "HealthCheckGracePeriodSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_grace_period_seconds: Option<i32>,
    #[serde(rename = "LaunchType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_type: Option<String>,
    #[serde(rename = "LoadBalancers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancers: Option<Vec<AwsEcsServiceLoadBalancersDetails>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "NetworkConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_configuration: Option<AwsEcsServiceNetworkConfigurationDetails>,
    #[serde(rename = "PlacementConstraints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_constraints: Option<Vec<AwsEcsServicePlacementConstraintsDetails>>,
    #[serde(rename = "PlacementStrategies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_strategies: Option<Vec<AwsEcsServicePlacementStrategiesDetails>>,
    #[serde(rename = "PlatformVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_version: Option<String>,
    #[serde(rename = "PropagateTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub propagate_tags: Option<String>,
    #[serde(rename = "Role")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(rename = "SchedulingStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduling_strategy: Option<String>,
    #[serde(rename = "ServiceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_arn: Option<String>,
    #[serde(rename = "ServiceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    #[serde(rename = "ServiceRegistries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_registries: Option<Vec<AwsEcsServiceServiceRegistriesDetails>>,
    #[serde(rename = "TaskDefinition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_definition: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEcsServiceCapacityProviderStrategyDetails {
    #[serde(rename = "Base")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base: Option<i32>,
    #[serde(rename = "CapacityProvider")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_provider: Option<String>,
    #[serde(rename = "Weight")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEcsServiceDeploymentConfigurationDetails {
    #[serde(rename = "DeploymentCircuitBreaker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_circuit_breaker:
        Option<AwsEcsServiceDeploymentConfigurationDeploymentCircuitBreakerDetails>,
    #[serde(rename = "MaximumPercent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_percent: Option<i32>,
    #[serde(rename = "MinimumHealthyPercent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_healthy_percent: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEcsServiceDeploymentConfigurationDeploymentCircuitBreakerDetails {
    #[serde(rename = "Enable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    #[serde(rename = "Rollback")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollback: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEcsServiceDeploymentControllerDetails {
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEcsServiceLoadBalancersDetails {
    #[serde(rename = "ContainerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
    #[serde(rename = "ContainerPort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_port: Option<i32>,
    #[serde(rename = "LoadBalancerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_name: Option<String>,
    #[serde(rename = "TargetGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_group_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEcsServiceNetworkConfigurationDetails {
    #[serde(rename = "AwsVpcConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_vpc_configuration: Option<AwsEcsServiceNetworkConfigurationAwsVpcConfigurationDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEcsServiceNetworkConfigurationAwsVpcConfigurationDetails {
    #[serde(rename = "AssignPublicIp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assign_public_ip: Option<String>,
    #[serde(rename = "SecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,
    #[serde(rename = "Subnets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnets: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEcsServicePlacementConstraintsDetails {
    #[serde(rename = "Expression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEcsServicePlacementStrategiesDetails {
    #[serde(rename = "Field")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEcsServiceServiceRegistriesDetails {
    #[serde(rename = "ContainerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
    #[serde(rename = "ContainerPort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_port: Option<i32>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "RegistryArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEcsTaskDetails {
    #[serde(rename = "ClusterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    #[serde(rename = "Containers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub containers: Option<Vec<AwsEcsContainerDetails>>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "Group")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(rename = "StartedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,
    #[serde(rename = "StartedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_by: Option<String>,
    #[serde(rename = "TaskDefinitionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_definition_arn: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "Volumes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<AwsEcsTaskVolumeDetails>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEcsTaskVolumeDetails {
    #[serde(rename = "Host")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<AwsEcsTaskVolumeHostDetails>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEcsTaskVolumeHostDetails {
    #[serde(rename = "SourcePath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_path: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEcsTaskDefinitionDetails {
    #[serde(rename = "ContainerDefinitions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_definitions: Option<Vec<AwsEcsTaskDefinitionContainerDefinitionsDetails>>,
    #[serde(rename = "Cpu")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<String>,
    #[serde(rename = "ExecutionRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role_arn: Option<String>,
    #[serde(rename = "Family")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[serde(rename = "InferenceAccelerators")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_accelerators: Option<Vec<AwsEcsTaskDefinitionInferenceAcceleratorsDetails>>,
    #[serde(rename = "IpcMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipc_mode: Option<String>,
    #[serde(rename = "Memory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<String>,
    #[serde(rename = "NetworkMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_mode: Option<String>,
    #[serde(rename = "PidMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid_mode: Option<String>,
    #[serde(rename = "PlacementConstraints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_constraints: Option<Vec<AwsEcsTaskDefinitionPlacementConstraintsDetails>>,
    #[serde(rename = "ProxyConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_configuration: Option<AwsEcsTaskDefinitionProxyConfigurationDetails>,
    #[serde(rename = "RequiresCompatibilities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requires_compatibilities: Option<Vec<String>>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TaskRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_role_arn: Option<String>,
    #[serde(rename = "Volumes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<AwsEcsTaskDefinitionVolumesDetails>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEcsTaskDefinitionContainerDefinitionsDetails {
    #[serde(rename = "Command")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    #[serde(rename = "Cpu")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<i32>,
    #[serde(rename = "DependsOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depends_on: Option<Vec<AwsEcsTaskDefinitionContainerDefinitionsDependsOnDetails>>,
    #[serde(rename = "DisableNetworking")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_networking: Option<bool>,
    #[serde(rename = "DnsSearchDomains")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_search_domains: Option<Vec<String>>,
    #[serde(rename = "DnsServers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_servers: Option<Vec<String>>,
    #[serde(rename = "DockerLabels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docker_labels: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "DockerSecurityOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docker_security_options: Option<Vec<String>>,
    #[serde(rename = "EntryPoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_point: Option<Vec<String>>,
    #[serde(rename = "Environment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<Vec<AwsEcsTaskDefinitionContainerDefinitionsEnvironmentDetails>>,
    #[serde(rename = "EnvironmentFiles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_files:
        Option<Vec<AwsEcsTaskDefinitionContainerDefinitionsEnvironmentFilesDetails>>,
    #[serde(rename = "Essential")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub essential: Option<bool>,
    #[serde(rename = "ExtraHosts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_hosts: Option<Vec<AwsEcsTaskDefinitionContainerDefinitionsExtraHostsDetails>>,
    #[serde(rename = "FirelensConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firelens_configuration:
        Option<AwsEcsTaskDefinitionContainerDefinitionsFirelensConfigurationDetails>,
    #[serde(rename = "HealthCheck")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check: Option<AwsEcsTaskDefinitionContainerDefinitionsHealthCheckDetails>,
    #[serde(rename = "Hostname")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(rename = "Image")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(rename = "Interactive")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactive: Option<bool>,
    #[serde(rename = "Links")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<String>>,
    #[serde(rename = "LinuxParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linux_parameters: Option<AwsEcsTaskDefinitionContainerDefinitionsLinuxParametersDetails>,
    #[serde(rename = "LogConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_configuration: Option<AwsEcsTaskDefinitionContainerDefinitionsLogConfigurationDetails>,
    #[serde(rename = "Memory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<i32>,
    #[serde(rename = "MemoryReservation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_reservation: Option<i32>,
    #[serde(rename = "MountPoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_points: Option<Vec<AwsEcsTaskDefinitionContainerDefinitionsMountPointsDetails>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "PortMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_mappings: Option<Vec<AwsEcsTaskDefinitionContainerDefinitionsPortMappingsDetails>>,
    #[serde(rename = "Privileged")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,
    #[serde(rename = "PseudoTerminal")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pseudo_terminal: Option<bool>,
    #[serde(rename = "ReadonlyRootFilesystem")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readonly_root_filesystem: Option<bool>,
    #[serde(rename = "RepositoryCredentials")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_credentials:
        Option<AwsEcsTaskDefinitionContainerDefinitionsRepositoryCredentialsDetails>,
    #[serde(rename = "ResourceRequirements")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_requirements:
        Option<Vec<AwsEcsTaskDefinitionContainerDefinitionsResourceRequirementsDetails>>,
    #[serde(rename = "Secrets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets: Option<Vec<AwsEcsTaskDefinitionContainerDefinitionsSecretsDetails>>,
    #[serde(rename = "StartTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_timeout: Option<i32>,
    #[serde(rename = "StopTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_timeout: Option<i32>,
    #[serde(rename = "SystemControls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_controls: Option<Vec<AwsEcsTaskDefinitionContainerDefinitionsSystemControlsDetails>>,
    #[serde(rename = "Ulimits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ulimits: Option<Vec<AwsEcsTaskDefinitionContainerDefinitionsUlimitsDetails>>,
    #[serde(rename = "User")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(rename = "VolumesFrom")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes_from: Option<Vec<AwsEcsTaskDefinitionContainerDefinitionsVolumesFromDetails>>,
    #[serde(rename = "WorkingDirectory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_directory: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEcsTaskDefinitionContainerDefinitionsDependsOnDetails {
    #[serde(rename = "Condition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[serde(rename = "ContainerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEcsTaskDefinitionContainerDefinitionsEnvironmentDetails {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEcsTaskDefinitionContainerDefinitionsEnvironmentFilesDetails {
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEcsTaskDefinitionContainerDefinitionsExtraHostsDetails {
    #[serde(rename = "Hostname")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(rename = "IpAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEcsTaskDefinitionContainerDefinitionsFirelensConfigurationDetails {
    #[serde(rename = "Options")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEcsTaskDefinitionContainerDefinitionsHealthCheckDetails {
    #[serde(rename = "Command")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    #[serde(rename = "Interval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<i32>,
    #[serde(rename = "Retries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retries: Option<i32>,
    #[serde(rename = "StartPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_period: Option<i32>,
    #[serde(rename = "Timeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEcsTaskDefinitionContainerDefinitionsLinuxParametersDetails {
    #[serde(rename = "Capabilities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities:
        Option<AwsEcsTaskDefinitionContainerDefinitionsLinuxParametersCapabilitiesDetails>,
    #[serde(rename = "Devices")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<AwsEcsTaskDefinitionContainerDefinitionsLinuxParametersDevicesDetails>>,
    #[serde(rename = "InitProcessEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init_process_enabled: Option<bool>,
    #[serde(rename = "MaxSwap")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_swap: Option<i32>,
    #[serde(rename = "SharedMemorySize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_memory_size: Option<i32>,
    #[serde(rename = "Swappiness")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swappiness: Option<i32>,
    #[serde(rename = "Tmpfs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tmpfs: Option<Vec<AwsEcsTaskDefinitionContainerDefinitionsLinuxParametersTmpfsDetails>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEcsTaskDefinitionContainerDefinitionsLinuxParametersCapabilitiesDetails {
    #[serde(rename = "Add")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add: Option<Vec<String>>,
    #[serde(rename = "Drop")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEcsTaskDefinitionContainerDefinitionsLinuxParametersDevicesDetails {
    #[serde(rename = "ContainerPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_path: Option<String>,
    #[serde(rename = "HostPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_path: Option<String>,
    #[serde(rename = "Permissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEcsTaskDefinitionContainerDefinitionsLinuxParametersTmpfsDetails {
    #[serde(rename = "ContainerPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_path: Option<String>,
    #[serde(rename = "MountOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_options: Option<Vec<String>>,
    #[serde(rename = "Size")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEcsTaskDefinitionContainerDefinitionsLogConfigurationDetails {
    #[serde(rename = "LogDriver")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_driver: Option<String>,
    #[serde(rename = "Options")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "SecretOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_options:
        Option<Vec<AwsEcsTaskDefinitionContainerDefinitionsLogConfigurationSecretOptionsDetails>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEcsTaskDefinitionContainerDefinitionsLogConfigurationSecretOptionsDetails {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ValueFrom")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_from: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEcsTaskDefinitionContainerDefinitionsMountPointsDetails {
    #[serde(rename = "ContainerPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_path: Option<String>,
    #[serde(rename = "ReadOnly")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[serde(rename = "SourceVolume")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_volume: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEcsTaskDefinitionContainerDefinitionsPortMappingsDetails {
    #[serde(rename = "ContainerPort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_port: Option<i32>,
    #[serde(rename = "HostPort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_port: Option<i32>,
    #[serde(rename = "Protocol")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEcsTaskDefinitionContainerDefinitionsRepositoryCredentialsDetails {
    #[serde(rename = "CredentialsParameter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials_parameter: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEcsTaskDefinitionContainerDefinitionsResourceRequirementsDetails {
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEcsTaskDefinitionContainerDefinitionsSecretsDetails {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ValueFrom")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_from: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEcsTaskDefinitionContainerDefinitionsSystemControlsDetails {
    #[serde(rename = "Namespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEcsTaskDefinitionContainerDefinitionsUlimitsDetails {
    #[serde(rename = "HardLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hard_limit: Option<i32>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "SoftLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub soft_limit: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEcsTaskDefinitionContainerDefinitionsVolumesFromDetails {
    #[serde(rename = "ReadOnly")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[serde(rename = "SourceContainer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_container: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEcsTaskDefinitionInferenceAcceleratorsDetails {
    #[serde(rename = "DeviceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    #[serde(rename = "DeviceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEcsTaskDefinitionPlacementConstraintsDetails {
    #[serde(rename = "Expression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEcsTaskDefinitionProxyConfigurationDetails {
    #[serde(rename = "ContainerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
    #[serde(rename = "ProxyConfigurationProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_configuration_properties:
        Option<Vec<AwsEcsTaskDefinitionProxyConfigurationProxyConfigurationPropertiesDetails>>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEcsTaskDefinitionProxyConfigurationProxyConfigurationPropertiesDetails {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEcsTaskDefinitionVolumesDetails {
    #[serde(rename = "DockerVolumeConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docker_volume_configuration:
        Option<AwsEcsTaskDefinitionVolumesDockerVolumeConfigurationDetails>,
    #[serde(rename = "EfsVolumeConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub efs_volume_configuration: Option<AwsEcsTaskDefinitionVolumesEfsVolumeConfigurationDetails>,
    #[serde(rename = "Host")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<AwsEcsTaskDefinitionVolumesHostDetails>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEcsTaskDefinitionVolumesDockerVolumeConfigurationDetails {
    #[serde(rename = "Autoprovision")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autoprovision: Option<bool>,
    #[serde(rename = "Driver")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    #[serde(rename = "DriverOpts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver_opts: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Labels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Scope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEcsTaskDefinitionVolumesEfsVolumeConfigurationDetails {
    #[serde(rename = "AuthorizationConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_config:
        Option<AwsEcsTaskDefinitionVolumesEfsVolumeConfigurationAuthorizationConfigDetails>,
    #[serde(rename = "FilesystemId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filesystem_id: Option<String>,
    #[serde(rename = "RootDirectory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_directory: Option<String>,
    #[serde(rename = "TransitEncryption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_encryption: Option<String>,
    #[serde(rename = "TransitEncryptionPort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_encryption_port: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEcsTaskDefinitionVolumesEfsVolumeConfigurationAuthorizationConfigDetails {
    #[serde(rename = "AccessPointId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_point_id: Option<String>,
    #[serde(rename = "Iam")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEcsTaskDefinitionVolumesHostDetails {
    #[serde(rename = "SourcePath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_path: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEfsAccessPointDetails {
    #[serde(rename = "AccessPointId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_point_id: Option<String>,
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "FileSystemId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
    #[serde(rename = "PosixUser")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posix_user: Option<AwsEfsAccessPointPosixUserDetails>,
    #[serde(rename = "RootDirectory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_directory: Option<AwsEfsAccessPointRootDirectoryDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEfsAccessPointPosixUserDetails {
    #[serde(rename = "Gid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gid: Option<String>,
    #[serde(rename = "SecondaryGids")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_gids: Option<Vec<String>>,
    #[serde(rename = "Uid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEfsAccessPointRootDirectoryDetails {
    #[serde(rename = "CreationInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_info: Option<AwsEfsAccessPointRootDirectoryCreationInfoDetails>,
    #[serde(rename = "Path")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEfsAccessPointRootDirectoryCreationInfoDetails {
    #[serde(rename = "OwnerGid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_gid: Option<String>,
    #[serde(rename = "OwnerUid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_uid: Option<String>,
    #[serde(rename = "Permissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEksClusterDetails {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CertificateAuthorityData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_authority_data: Option<String>,
    #[serde(rename = "ClusterStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_status: Option<String>,
    #[serde(rename = "Endpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(rename = "Logging")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<AwsEksClusterLoggingDetails>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ResourcesVpcConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources_vpc_config: Option<AwsEksClusterResourcesVpcConfigDetails>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEksClusterLoggingDetails {
    #[serde(rename = "ClusterLogging")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_logging: Option<Vec<AwsEksClusterLoggingClusterLoggingDetails>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEksClusterLoggingClusterLoggingDetails {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "Types")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEksClusterResourcesVpcConfigDetails {
    #[serde(rename = "EndpointPublicAccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_public_access: Option<bool>,
    #[serde(rename = "SecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(rename = "SubnetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsElasticBeanstalkEnvironmentDetails {
    #[serde(rename = "ApplicationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    #[serde(rename = "Cname")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cname: Option<String>,
    #[serde(rename = "DateCreated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    #[serde(rename = "DateUpdated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EndpointUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_url: Option<String>,
    #[serde(rename = "EnvironmentArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_arn: Option<String>,
    #[serde(rename = "EnvironmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_id: Option<String>,
    #[serde(rename = "EnvironmentLinks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_links: Option<Vec<AwsElasticBeanstalkEnvironmentEnvironmentLink>>,
    #[serde(rename = "EnvironmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_name: Option<String>,
    #[serde(rename = "OptionSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_settings: Option<Vec<AwsElasticBeanstalkEnvironmentOptionSetting>>,
    #[serde(rename = "PlatformArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_arn: Option<String>,
    #[serde(rename = "SolutionStackName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution_stack_name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Tier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<AwsElasticBeanstalkEnvironmentTier>,
    #[serde(rename = "VersionLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_label: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsElasticBeanstalkEnvironmentEnvironmentLink {
    #[serde(rename = "EnvironmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_name: Option<String>,
    #[serde(rename = "LinkName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsElasticBeanstalkEnvironmentOptionSetting {
    #[serde(rename = "Namespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "OptionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_name: Option<String>,
    #[serde(rename = "ResourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsElasticBeanstalkEnvironmentTier {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsElasticsearchDomainDetails {
    #[serde(rename = "AccessPolicies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_policies: Option<String>,
    #[serde(rename = "DomainEndpointOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_endpoint_options: Option<AwsElasticsearchDomainDomainEndpointOptions>,
    #[serde(rename = "DomainId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_id: Option<String>,
    #[serde(rename = "DomainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(rename = "ElasticsearchClusterConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elasticsearch_cluster_config:
        Option<AwsElasticsearchDomainElasticsearchClusterConfigDetails>,
    #[serde(rename = "ElasticsearchVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elasticsearch_version: Option<String>,
    #[serde(rename = "EncryptionAtRestOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_at_rest_options: Option<AwsElasticsearchDomainEncryptionAtRestOptions>,
    #[serde(rename = "Endpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(rename = "Endpoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "LogPublishingOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_publishing_options: Option<AwsElasticsearchDomainLogPublishingOptions>,
    #[serde(rename = "NodeToNodeEncryptionOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_to_node_encryption_options: Option<AwsElasticsearchDomainNodeToNodeEncryptionOptions>,
    #[serde(rename = "ServiceSoftwareOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_software_options: Option<AwsElasticsearchDomainServiceSoftwareOptions>,
    #[serde(rename = "VPCOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_p_c_options: Option<AwsElasticsearchDomainVPCOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsElasticsearchDomainDomainEndpointOptions {
    #[serde(rename = "EnforceHTTPS")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enforce_h_t_t_p_s: Option<bool>,
    #[serde(rename = "TLSSecurityPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_l_s_security_policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsElasticsearchDomainElasticsearchClusterConfigDetails {
    #[serde(rename = "DedicatedMasterCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated_master_count: Option<i32>,
    #[serde(rename = "DedicatedMasterEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated_master_enabled: Option<bool>,
    #[serde(rename = "DedicatedMasterType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated_master_type: Option<String>,
    #[serde(rename = "InstanceCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_count: Option<i32>,
    #[serde(rename = "InstanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    #[serde(rename = "ZoneAwarenessConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_awareness_config:
        Option<AwsElasticsearchDomainElasticsearchClusterConfigZoneAwarenessConfigDetails>,
    #[serde(rename = "ZoneAwarenessEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_awareness_enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsElasticsearchDomainElasticsearchClusterConfigZoneAwarenessConfigDetails {
    #[serde(rename = "AvailabilityZoneCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsElasticsearchDomainEncryptionAtRestOptions {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsElasticsearchDomainLogPublishingOptions {
    #[serde(rename = "AuditLogs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_logs: Option<AwsElasticsearchDomainLogPublishingOptionsLogConfig>,
    #[serde(rename = "IndexSlowLogs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_slow_logs: Option<AwsElasticsearchDomainLogPublishingOptionsLogConfig>,
    #[serde(rename = "SearchSlowLogs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_slow_logs: Option<AwsElasticsearchDomainLogPublishingOptionsLogConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsElasticsearchDomainLogPublishingOptionsLogConfig {
    #[serde(rename = "CloudWatchLogsLogGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs_log_group_arn: Option<String>,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsElasticsearchDomainNodeToNodeEncryptionOptions {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsElasticsearchDomainServiceSoftwareOptions {
    #[serde(rename = "AutomatedUpdateDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automated_update_date: Option<String>,
    #[serde(rename = "Cancellable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellable: Option<bool>,
    #[serde(rename = "CurrentVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_version: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "NewVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_version: Option<String>,
    #[serde(rename = "UpdateAvailable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_available: Option<bool>,
    #[serde(rename = "UpdateStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsElasticsearchDomainVPCOptions {
    #[serde(rename = "AvailabilityZones")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<Vec<String>>,
    #[serde(rename = "SecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(rename = "SubnetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    #[serde(rename = "VPCId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_p_c_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsElbLoadBalancerDetails {
    #[serde(rename = "AvailabilityZones")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<Vec<String>>,
    #[serde(rename = "BackendServerDescriptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_server_descriptions: Option<Vec<AwsElbLoadBalancerBackendServerDescription>>,
    #[serde(rename = "CanonicalHostedZoneName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canonical_hosted_zone_name: Option<String>,
    #[serde(rename = "CanonicalHostedZoneNameID")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canonical_hosted_zone_name_i_d: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(rename = "DnsName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_name: Option<String>,
    #[serde(rename = "HealthCheck")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check: Option<AwsElbLoadBalancerHealthCheck>,
    #[serde(rename = "Instances")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances: Option<Vec<AwsElbLoadBalancerInstance>>,
    #[serde(rename = "ListenerDescriptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener_descriptions: Option<Vec<AwsElbLoadBalancerListenerDescription>>,
    #[serde(rename = "LoadBalancerAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_attributes: Option<AwsElbLoadBalancerAttributes>,
    #[serde(rename = "LoadBalancerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_name: Option<String>,
    #[serde(rename = "Policies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<AwsElbLoadBalancerPolicies>,
    #[serde(rename = "Scheme")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
    #[serde(rename = "SecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,
    #[serde(rename = "SourceSecurityGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_security_group: Option<AwsElbLoadBalancerSourceSecurityGroup>,
    #[serde(rename = "Subnets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnets: Option<Vec<String>>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsElbLoadBalancerBackendServerDescription {
    #[serde(rename = "InstancePort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_port: Option<i32>,
    #[serde(rename = "PolicyNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_names: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsElbLoadBalancerHealthCheck {
    #[serde(rename = "HealthyThreshold")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub healthy_threshold: Option<i32>,
    #[serde(rename = "Interval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<i32>,
    #[serde(rename = "Target")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(rename = "Timeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
    #[serde(rename = "UnhealthyThreshold")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unhealthy_threshold: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsElbLoadBalancerInstance {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsElbLoadBalancerListenerDescription {
    #[serde(rename = "Listener")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener: Option<AwsElbLoadBalancerListener>,
    #[serde(rename = "PolicyNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_names: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsElbLoadBalancerListener {
    #[serde(rename = "InstancePort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_port: Option<i32>,
    #[serde(rename = "InstanceProtocol")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_protocol: Option<String>,
    #[serde(rename = "LoadBalancerPort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_port: Option<i32>,
    #[serde(rename = "Protocol")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(rename = "SslCertificateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_certificate_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsElbLoadBalancerAttributes {
    #[serde(rename = "AccessLog")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_log: Option<AwsElbLoadBalancerAccessLog>,
    #[serde(rename = "AdditionalAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_attributes: Option<Vec<AwsElbLoadBalancerAdditionalAttribute>>,
    #[serde(rename = "ConnectionDraining")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_draining: Option<AwsElbLoadBalancerConnectionDraining>,
    #[serde(rename = "ConnectionSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_settings: Option<AwsElbLoadBalancerConnectionSettings>,
    #[serde(rename = "CrossZoneLoadBalancing")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_zone_load_balancing: Option<AwsElbLoadBalancerCrossZoneLoadBalancing>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsElbLoadBalancerAccessLog {
    #[serde(rename = "EmitInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emit_interval: Option<i32>,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "S3BucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_name: Option<String>,
    #[serde(rename = "S3BucketPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsElbLoadBalancerAdditionalAttribute {
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
pub struct AwsElbLoadBalancerConnectionDraining {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "Timeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsElbLoadBalancerConnectionSettings {
    #[serde(rename = "IdleTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_timeout: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsElbLoadBalancerCrossZoneLoadBalancing {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsElbLoadBalancerPolicies {
    #[serde(rename = "AppCookieStickinessPolicies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_cookie_stickiness_policies: Option<Vec<AwsElbAppCookieStickinessPolicy>>,
    #[serde(rename = "LbCookieStickinessPolicies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lb_cookie_stickiness_policies: Option<Vec<AwsElbLbCookieStickinessPolicy>>,
    #[serde(rename = "OtherPolicies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_policies: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsElbAppCookieStickinessPolicy {
    #[serde(rename = "CookieName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookie_name: Option<String>,
    #[serde(rename = "PolicyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsElbLbCookieStickinessPolicy {
    #[serde(rename = "CookieExpirationPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookie_expiration_period: Option<i64>,
    #[serde(rename = "PolicyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsElbLoadBalancerSourceSecurityGroup {
    #[serde(rename = "GroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(rename = "OwnerAlias")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_alias: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsElbv2LoadBalancerDetails {
    #[serde(rename = "AvailabilityZones")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<Vec<AvailabilityZone>>,
    #[serde(rename = "CanonicalHostedZoneId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canonical_hosted_zone_id: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(rename = "DNSName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_n_s_name: Option<String>,
    #[serde(rename = "IpAddressType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_type: Option<String>,
    #[serde(rename = "LoadBalancerAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_attributes: Option<Vec<AwsElbv2LoadBalancerAttribute>>,
    #[serde(rename = "Scheme")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
    #[serde(rename = "SecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<LoadBalancerState>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AvailabilityZone {
    #[serde(rename = "SubnetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    #[serde(rename = "ZoneName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsElbv2LoadBalancerAttribute {
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
pub struct LoadBalancerState {
    #[serde(rename = "Code")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "Reason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEventSchemasRegistryDetails {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "RegistryArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_arn: Option<String>,
    #[serde(rename = "RegistryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEventsEndpointDetails {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EndpointId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_id: Option<String>,
    #[serde(rename = "EndpointUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_url: Option<String>,
    #[serde(rename = "EventBuses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_buses: Option<Vec<AwsEventsEndpointEventBusesDetails>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ReplicationConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_config: Option<AwsEventsEndpointReplicationConfigDetails>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "RoutingConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_config: Option<AwsEventsEndpointRoutingConfigDetails>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "StateReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEventsEndpointEventBusesDetails {
    #[serde(rename = "EventBusArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bus_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEventsEndpointReplicationConfigDetails {
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEventsEndpointRoutingConfigDetails {
    #[serde(rename = "FailoverConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failover_config: Option<AwsEventsEndpointRoutingConfigFailoverConfigDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEventsEndpointRoutingConfigFailoverConfigDetails {
    #[serde(rename = "Primary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<AwsEventsEndpointRoutingConfigFailoverConfigPrimaryDetails>,
    #[serde(rename = "Secondary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary: Option<AwsEventsEndpointRoutingConfigFailoverConfigSecondaryDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEventsEndpointRoutingConfigFailoverConfigPrimaryDetails {
    #[serde(rename = "HealthCheck")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEventsEndpointRoutingConfigFailoverConfigSecondaryDetails {
    #[serde(rename = "Route")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEventsEventbusDetails {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Policy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsGuardDutyDetectorDetails {
    #[serde(rename = "DataSources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_sources: Option<AwsGuardDutyDetectorDataSourcesDetails>,
    #[serde(rename = "Features")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<Vec<AwsGuardDutyDetectorFeaturesDetails>>,
    #[serde(rename = "FindingPublishingFrequency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_publishing_frequency: Option<String>,
    #[serde(rename = "ServiceRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsGuardDutyDetectorDataSourcesDetails {
    #[serde(rename = "CloudTrail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_trail: Option<AwsGuardDutyDetectorDataSourcesCloudTrailDetails>,
    #[serde(rename = "DnsLogs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_logs: Option<AwsGuardDutyDetectorDataSourcesDnsLogsDetails>,
    #[serde(rename = "FlowLogs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_logs: Option<AwsGuardDutyDetectorDataSourcesFlowLogsDetails>,
    #[serde(rename = "Kubernetes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kubernetes: Option<AwsGuardDutyDetectorDataSourcesKubernetesDetails>,
    #[serde(rename = "MalwareProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub malware_protection: Option<AwsGuardDutyDetectorDataSourcesMalwareProtectionDetails>,
    #[serde(rename = "S3Logs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_logs: Option<AwsGuardDutyDetectorDataSourcesS3LogsDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsGuardDutyDetectorDataSourcesCloudTrailDetails {
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsGuardDutyDetectorDataSourcesDnsLogsDetails {
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsGuardDutyDetectorDataSourcesFlowLogsDetails {
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsGuardDutyDetectorDataSourcesKubernetesDetails {
    #[serde(rename = "AuditLogs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_logs: Option<AwsGuardDutyDetectorDataSourcesKubernetesAuditLogsDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsGuardDutyDetectorDataSourcesKubernetesAuditLogsDetails {
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsGuardDutyDetectorDataSourcesMalwareProtectionDetails {
    #[serde(rename = "ScanEc2InstanceWithFindings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_ec2_instance_with_findings:
        Option<AwsGuardDutyDetectorDataSourcesMalwareProtectionScanEc2InstanceWithFindingsDetails>,
    #[serde(rename = "ServiceRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsGuardDutyDetectorDataSourcesMalwareProtectionScanEc2InstanceWithFindingsDetails {
    #[serde(rename = "EbsVolumes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_volumes: Option<AwsGuardDutyDetectorDataSourcesMalwareProtectionScanEc2InstanceWithFindingsEbsVolumesDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsGuardDutyDetectorDataSourcesMalwareProtectionScanEc2InstanceWithFindingsEbsVolumesDetails
{
    #[serde(rename = "Reason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsGuardDutyDetectorDataSourcesS3LogsDetails {
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsGuardDutyDetectorFeaturesDetails {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsIamAccessKeyDetails {
    #[serde(rename = "AccessKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_id: Option<String>,
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "PrincipalId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[serde(rename = "PrincipalName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_name: Option<String>,
    #[serde(rename = "PrincipalType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_type: Option<String>,
    #[serde(rename = "SessionContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_context: Option<AwsIamAccessKeySessionContext>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "UserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsIamAccessKeySessionContext {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<AwsIamAccessKeySessionContextAttributes>,
    #[serde(rename = "SessionIssuer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_issuer: Option<AwsIamAccessKeySessionContextSessionIssuer>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsIamAccessKeySessionContextAttributes {
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "MfaAuthenticated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_authenticated: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsIamAccessKeySessionContextSessionIssuer {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "PrincipalId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "UserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsIamGroupDetails {
    #[serde(rename = "AttachedManagedPolicies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attached_managed_policies: Option<Vec<AwsIamAttachedManagedPolicy>>,
    #[serde(rename = "CreateDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
    #[serde(rename = "GroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(rename = "GroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(rename = "GroupPolicyList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_policy_list: Option<Vec<AwsIamGroupPolicy>>,
    #[serde(rename = "Path")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsIamAttachedManagedPolicy {
    #[serde(rename = "PolicyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_arn: Option<String>,
    #[serde(rename = "PolicyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsIamGroupPolicy {
    #[serde(rename = "PolicyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsIamPolicyDetails {
    #[serde(rename = "AttachmentCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_count: Option<i32>,
    #[serde(rename = "CreateDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
    #[serde(rename = "DefaultVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_version_id: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "IsAttachable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_attachable: Option<bool>,
    #[serde(rename = "Path")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "PermissionsBoundaryUsageCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions_boundary_usage_count: Option<i32>,
    #[serde(rename = "PolicyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    #[serde(rename = "PolicyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    #[serde(rename = "PolicyVersionList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_version_list: Option<Vec<AwsIamPolicyVersion>>,
    #[serde(rename = "UpdateDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_date: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsIamPolicyVersion {
    #[serde(rename = "CreateDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
    #[serde(rename = "IsDefaultVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default_version: Option<bool>,
    #[serde(rename = "VersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsIamRoleDetails {
    #[serde(rename = "AssumeRolePolicyDocument")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assume_role_policy_document: Option<String>,
    #[serde(rename = "AttachedManagedPolicies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attached_managed_policies: Option<Vec<AwsIamAttachedManagedPolicy>>,
    #[serde(rename = "CreateDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
    #[serde(rename = "InstanceProfileList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_profile_list: Option<Vec<AwsIamInstanceProfile>>,
    #[serde(rename = "MaxSessionDuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_session_duration: Option<i32>,
    #[serde(rename = "Path")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "PermissionsBoundary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions_boundary: Option<AwsIamPermissionsBoundary>,
    #[serde(rename = "RoleId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_id: Option<String>,
    #[serde(rename = "RoleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    #[serde(rename = "RolePolicyList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_policy_list: Option<Vec<AwsIamRolePolicy>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsIamInstanceProfile {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreateDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
    #[serde(rename = "InstanceProfileId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_profile_id: Option<String>,
    #[serde(rename = "InstanceProfileName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_profile_name: Option<String>,
    #[serde(rename = "Path")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "Roles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<AwsIamInstanceProfileRole>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsIamInstanceProfileRole {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "AssumeRolePolicyDocument")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assume_role_policy_document: Option<String>,
    #[serde(rename = "CreateDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
    #[serde(rename = "Path")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "RoleId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_id: Option<String>,
    #[serde(rename = "RoleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsIamPermissionsBoundary {
    #[serde(rename = "PermissionsBoundaryArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions_boundary_arn: Option<String>,
    #[serde(rename = "PermissionsBoundaryType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions_boundary_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsIamRolePolicy {
    #[serde(rename = "PolicyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsIamUserDetails {
    #[serde(rename = "AttachedManagedPolicies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attached_managed_policies: Option<Vec<AwsIamAttachedManagedPolicy>>,
    #[serde(rename = "CreateDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<String>,
    #[serde(rename = "GroupList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_list: Option<Vec<String>>,
    #[serde(rename = "Path")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "PermissionsBoundary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions_boundary: Option<AwsIamPermissionsBoundary>,
    #[serde(rename = "UserId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(rename = "UserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(rename = "UserPolicyList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_policy_list: Option<Vec<AwsIamUserPolicy>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsIamUserPolicy {
    #[serde(rename = "PolicyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsKinesisStreamDetails {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "RetentionPeriodHours")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_period_hours: Option<i32>,
    #[serde(rename = "ShardCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard_count: Option<i32>,
    #[serde(rename = "StreamEncryption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_encryption: Option<AwsKinesisStreamStreamEncryptionDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsKinesisStreamStreamEncryptionDetails {
    #[serde(rename = "EncryptionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_type: Option<String>,
    #[serde(rename = "KeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsKmsKeyDetails {
    #[serde(rename = "AWSAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_w_s_account_id: Option<String>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "KeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    #[serde(rename = "KeyManager")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_manager: Option<String>,
    #[serde(rename = "KeyRotationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_rotation_status: Option<bool>,
    #[serde(rename = "KeyState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_state: Option<String>,
    #[serde(rename = "Origin")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsLambdaFunctionDetails {
    #[serde(rename = "Architectures")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architectures: Option<Vec<String>>,
    #[serde(rename = "Code")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<AwsLambdaFunctionCode>,
    #[serde(rename = "CodeSha256")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_sha256: Option<String>,
    #[serde(rename = "DeadLetterConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dead_letter_config: Option<AwsLambdaFunctionDeadLetterConfig>,
    #[serde(rename = "Environment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<AwsLambdaFunctionEnvironment>,
    #[serde(rename = "FunctionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_name: Option<String>,
    #[serde(rename = "Handler")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handler: Option<String>,
    #[serde(rename = "KmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    #[serde(rename = "LastModified")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
    #[serde(rename = "Layers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layers: Option<Vec<AwsLambdaFunctionLayer>>,
    #[serde(rename = "MasterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_arn: Option<String>,
    #[serde(rename = "MemorySize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_size: Option<i32>,
    #[serde(rename = "PackageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_type: Option<String>,
    #[serde(rename = "RevisionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
    #[serde(rename = "Role")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(rename = "Runtime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime: Option<String>,
    #[serde(rename = "Timeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
    #[serde(rename = "TracingConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracing_config: Option<AwsLambdaFunctionTracingConfig>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "VpcConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<AwsLambdaFunctionVpcConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsLambdaFunctionCode {
    #[serde(rename = "S3Bucket")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket: Option<String>,
    #[serde(rename = "S3Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key: Option<String>,
    #[serde(rename = "S3ObjectVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_object_version: Option<String>,
    #[serde(rename = "ZipFile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip_file: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsLambdaFunctionDeadLetterConfig {
    #[serde(rename = "TargetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsLambdaFunctionEnvironment {
    #[serde(rename = "Error")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<AwsLambdaFunctionEnvironmentError>,
    #[serde(rename = "Variables")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsLambdaFunctionEnvironmentError {
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsLambdaFunctionLayer {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CodeSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_size: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsLambdaFunctionTracingConfig {
    #[serde(rename = "Mode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsLambdaFunctionVpcConfig {
    #[serde(rename = "SecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(rename = "SubnetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsLambdaLayerVersionDetails {
    #[serde(rename = "CompatibleRuntimes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatible_runtimes: Option<Vec<String>>,
    #[serde(rename = "CreatedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsMskClusterDetails {
    #[serde(rename = "ClusterInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_info: Option<AwsMskClusterClusterInfoDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsMskClusterClusterInfoDetails {
    #[serde(rename = "ClientAuthentication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_authentication: Option<AwsMskClusterClusterInfoClientAuthenticationDetails>,
    #[serde(rename = "ClusterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    #[serde(rename = "CurrentVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_version: Option<String>,
    #[serde(rename = "EncryptionInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_info: Option<AwsMskClusterClusterInfoEncryptionInfoDetails>,
    #[serde(rename = "EnhancedMonitoring")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enhanced_monitoring: Option<String>,
    #[serde(rename = "NumberOfBrokerNodes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_broker_nodes: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsMskClusterClusterInfoClientAuthenticationDetails {
    #[serde(rename = "Sasl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sasl: Option<AwsMskClusterClusterInfoClientAuthenticationSaslDetails>,
    #[serde(rename = "Tls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls: Option<AwsMskClusterClusterInfoClientAuthenticationTlsDetails>,
    #[serde(rename = "Unauthenticated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unauthenticated: Option<AwsMskClusterClusterInfoClientAuthenticationUnauthenticatedDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsMskClusterClusterInfoClientAuthenticationSaslDetails {
    #[serde(rename = "Iam")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam: Option<AwsMskClusterClusterInfoClientAuthenticationSaslIamDetails>,
    #[serde(rename = "Scram")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scram: Option<AwsMskClusterClusterInfoClientAuthenticationSaslScramDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsMskClusterClusterInfoClientAuthenticationSaslIamDetails {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsMskClusterClusterInfoClientAuthenticationSaslScramDetails {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsMskClusterClusterInfoClientAuthenticationTlsDetails {
    #[serde(rename = "CertificateAuthorityArnList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_authority_arn_list: Option<Vec<String>>,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsMskClusterClusterInfoClientAuthenticationUnauthenticatedDetails {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsMskClusterClusterInfoEncryptionInfoDetails {
    #[serde(rename = "EncryptionAtRest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_at_rest: Option<AwsMskClusterClusterInfoEncryptionInfoEncryptionAtRestDetails>,
    #[serde(rename = "EncryptionInTransit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_in_transit:
        Option<AwsMskClusterClusterInfoEncryptionInfoEncryptionInTransitDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsMskClusterClusterInfoEncryptionInfoEncryptionAtRestDetails {
    #[serde(rename = "DataVolumeKMSKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_volume_k_m_s_key_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsMskClusterClusterInfoEncryptionInfoEncryptionInTransitDetails {
    #[serde(rename = "ClientBroker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_broker: Option<String>,
    #[serde(rename = "InCluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_cluster: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsNetworkFirewallFirewallDetails {
    #[serde(rename = "DeleteProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_protection: Option<bool>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "FirewallArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_arn: Option<String>,
    #[serde(rename = "FirewallId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_id: Option<String>,
    #[serde(rename = "FirewallName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_name: Option<String>,
    #[serde(rename = "FirewallPolicyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_policy_arn: Option<String>,
    #[serde(rename = "FirewallPolicyChangeProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_policy_change_protection: Option<bool>,
    #[serde(rename = "SubnetChangeProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_change_protection: Option<bool>,
    #[serde(rename = "SubnetMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_mappings: Option<Vec<AwsNetworkFirewallFirewallSubnetMappingsDetails>>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsNetworkFirewallFirewallSubnetMappingsDetails {
    #[serde(rename = "SubnetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsNetworkFirewallFirewallPolicyDetails {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "FirewallPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_policy: Option<FirewallPolicyDetails>,
    #[serde(rename = "FirewallPolicyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_policy_arn: Option<String>,
    #[serde(rename = "FirewallPolicyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_policy_id: Option<String>,
    #[serde(rename = "FirewallPolicyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firewall_policy_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FirewallPolicyDetails {
    #[serde(rename = "StatefulRuleGroupReferences")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stateful_rule_group_references:
        Option<Vec<FirewallPolicyStatefulRuleGroupReferencesDetails>>,
    #[serde(rename = "StatelessCustomActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stateless_custom_actions: Option<Vec<FirewallPolicyStatelessCustomActionsDetails>>,
    #[serde(rename = "StatelessDefaultActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stateless_default_actions: Option<Vec<String>>,
    #[serde(rename = "StatelessFragmentDefaultActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stateless_fragment_default_actions: Option<Vec<String>>,
    #[serde(rename = "StatelessRuleGroupReferences")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stateless_rule_group_references:
        Option<Vec<FirewallPolicyStatelessRuleGroupReferencesDetails>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FirewallPolicyStatefulRuleGroupReferencesDetails {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FirewallPolicyStatelessCustomActionsDetails {
    #[serde(rename = "ActionDefinition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_definition: Option<StatelessCustomActionDefinition>,
    #[serde(rename = "ActionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StatelessCustomActionDefinition {
    #[serde(rename = "PublishMetricAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_metric_action: Option<StatelessCustomPublishMetricAction>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StatelessCustomPublishMetricAction {
    #[serde(rename = "Dimensions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Vec<StatelessCustomPublishMetricActionDimension>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StatelessCustomPublishMetricActionDimension {
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FirewallPolicyStatelessRuleGroupReferencesDetails {
    #[serde(rename = "Priority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsNetworkFirewallRuleGroupDetails {
    #[serde(rename = "Capacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "RuleGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_group: Option<RuleGroupDetails>,
    #[serde(rename = "RuleGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_group_arn: Option<String>,
    #[serde(rename = "RuleGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_group_id: Option<String>,
    #[serde(rename = "RuleGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_group_name: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuleGroupDetails {
    #[serde(rename = "RuleVariables")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_variables: Option<RuleGroupVariables>,
    #[serde(rename = "RulesSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules_source: Option<RuleGroupSource>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuleGroupVariables {
    #[serde(rename = "IpSets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_sets: Option<RuleGroupVariablesIpSetsDetails>,
    #[serde(rename = "PortSets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_sets: Option<RuleGroupVariablesPortSetsDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuleGroupVariablesIpSetsDetails {
    #[serde(rename = "Definition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuleGroupVariablesPortSetsDetails {
    #[serde(rename = "Definition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuleGroupSource {
    #[serde(rename = "RulesSourceList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules_source_list: Option<RuleGroupSourceListDetails>,
    #[serde(rename = "RulesString")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules_string: Option<String>,
    #[serde(rename = "StatefulRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stateful_rules: Option<Vec<RuleGroupSourceStatefulRulesDetails>>,
    #[serde(rename = "StatelessRulesAndCustomActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stateless_rules_and_custom_actions:
        Option<RuleGroupSourceStatelessRulesAndCustomActionsDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuleGroupSourceListDetails {
    #[serde(rename = "GeneratedRulesType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_rules_type: Option<String>,
    #[serde(rename = "TargetTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_types: Option<Vec<String>>,
    #[serde(rename = "Targets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuleGroupSourceStatefulRulesDetails {
    #[serde(rename = "Action")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "Header")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<RuleGroupSourceStatefulRulesHeaderDetails>,
    #[serde(rename = "RuleOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_options: Option<Vec<RuleGroupSourceStatefulRulesOptionsDetails>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuleGroupSourceStatefulRulesHeaderDetails {
    #[serde(rename = "Destination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    #[serde(rename = "DestinationPort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_port: Option<String>,
    #[serde(rename = "Direction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    #[serde(rename = "Protocol")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(rename = "Source")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "SourcePort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_port: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuleGroupSourceStatefulRulesOptionsDetails {
    #[serde(rename = "Keyword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword: Option<String>,
    #[serde(rename = "Settings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuleGroupSourceStatelessRulesAndCustomActionsDetails {
    #[serde(rename = "CustomActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_actions: Option<Vec<RuleGroupSourceCustomActionsDetails>>,
    #[serde(rename = "StatelessRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stateless_rules: Option<Vec<RuleGroupSourceStatelessRulesDetails>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuleGroupSourceCustomActionsDetails {
    #[serde(rename = "ActionDefinition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_definition: Option<StatelessCustomActionDefinition>,
    #[serde(rename = "ActionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuleGroupSourceStatelessRulesDetails {
    #[serde(rename = "Priority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(rename = "RuleDefinition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_definition: Option<RuleGroupSourceStatelessRuleDefinition>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuleGroupSourceStatelessRuleDefinition {
    #[serde(rename = "Actions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<String>>,
    #[serde(rename = "MatchAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_attributes: Option<RuleGroupSourceStatelessRuleMatchAttributes>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuleGroupSourceStatelessRuleMatchAttributes {
    #[serde(rename = "DestinationPorts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_ports: Option<Vec<RuleGroupSourceStatelessRuleMatchAttributesDestinationPorts>>,
    #[serde(rename = "Destinations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<RuleGroupSourceStatelessRuleMatchAttributesDestinations>>,
    #[serde(rename = "Protocols")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocols: Option<Vec<i32>>,
    #[serde(rename = "SourcePorts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ports: Option<Vec<RuleGroupSourceStatelessRuleMatchAttributesSourcePorts>>,
    #[serde(rename = "Sources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<RuleGroupSourceStatelessRuleMatchAttributesSources>>,
    #[serde(rename = "TcpFlags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp_flags: Option<Vec<RuleGroupSourceStatelessRuleMatchAttributesTcpFlags>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuleGroupSourceStatelessRuleMatchAttributesDestinationPorts {
    #[serde(rename = "FromPort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_port: Option<i32>,
    #[serde(rename = "ToPort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_port: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuleGroupSourceStatelessRuleMatchAttributesDestinations {
    #[serde(rename = "AddressDefinition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_definition: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuleGroupSourceStatelessRuleMatchAttributesSourcePorts {
    #[serde(rename = "FromPort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_port: Option<i32>,
    #[serde(rename = "ToPort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_port: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuleGroupSourceStatelessRuleMatchAttributesSources {
    #[serde(rename = "AddressDefinition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_definition: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuleGroupSourceStatelessRuleMatchAttributesTcpFlags {
    #[serde(rename = "Flags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<Vec<String>>,
    #[serde(rename = "Masks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub masks: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsOpenSearchServiceDomainDetails {
    #[serde(rename = "AccessPolicies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_policies: Option<String>,
    #[serde(rename = "AdvancedSecurityOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_security_options: Option<AwsOpenSearchServiceDomainAdvancedSecurityOptionsDetails>,
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "ClusterConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_config: Option<AwsOpenSearchServiceDomainClusterConfigDetails>,
    #[serde(rename = "DomainEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_endpoint: Option<String>,
    #[serde(rename = "DomainEndpointOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_endpoint_options: Option<AwsOpenSearchServiceDomainDomainEndpointOptionsDetails>,
    #[serde(rename = "DomainEndpoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_endpoints: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "DomainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(rename = "EncryptionAtRestOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_at_rest_options:
        Option<AwsOpenSearchServiceDomainEncryptionAtRestOptionsDetails>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LogPublishingOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_publishing_options: Option<AwsOpenSearchServiceDomainLogPublishingOptionsDetails>,
    #[serde(rename = "NodeToNodeEncryptionOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_to_node_encryption_options:
        Option<AwsOpenSearchServiceDomainNodeToNodeEncryptionOptionsDetails>,
    #[serde(rename = "ServiceSoftwareOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_software_options: Option<AwsOpenSearchServiceDomainServiceSoftwareOptionsDetails>,
    #[serde(rename = "VpcOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_options: Option<AwsOpenSearchServiceDomainVpcOptionsDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsOpenSearchServiceDomainAdvancedSecurityOptionsDetails {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "InternalUserDatabaseEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_user_database_enabled: Option<bool>,
    #[serde(rename = "MasterUserOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_user_options: Option<AwsOpenSearchServiceDomainMasterUserOptionsDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsOpenSearchServiceDomainMasterUserOptionsDetails {
    #[serde(rename = "MasterUserArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_user_arn: Option<String>,
    #[serde(rename = "MasterUserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_user_name: Option<String>,
    #[serde(rename = "MasterUserPassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_user_password: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsOpenSearchServiceDomainClusterConfigDetails {
    #[serde(rename = "DedicatedMasterCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated_master_count: Option<i32>,
    #[serde(rename = "DedicatedMasterEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated_master_enabled: Option<bool>,
    #[serde(rename = "DedicatedMasterType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated_master_type: Option<String>,
    #[serde(rename = "InstanceCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_count: Option<i32>,
    #[serde(rename = "InstanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    #[serde(rename = "WarmCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warm_count: Option<i32>,
    #[serde(rename = "WarmEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warm_enabled: Option<bool>,
    #[serde(rename = "WarmType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warm_type: Option<String>,
    #[serde(rename = "ZoneAwarenessConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_awareness_config:
        Option<AwsOpenSearchServiceDomainClusterConfigZoneAwarenessConfigDetails>,
    #[serde(rename = "ZoneAwarenessEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_awareness_enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsOpenSearchServiceDomainClusterConfigZoneAwarenessConfigDetails {
    #[serde(rename = "AvailabilityZoneCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsOpenSearchServiceDomainDomainEndpointOptionsDetails {
    #[serde(rename = "CustomEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_endpoint: Option<String>,
    #[serde(rename = "CustomEndpointCertificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_endpoint_certificate_arn: Option<String>,
    #[serde(rename = "CustomEndpointEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_endpoint_enabled: Option<bool>,
    #[serde(rename = "EnforceHTTPS")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enforce_h_t_t_p_s: Option<bool>,
    #[serde(rename = "TLSSecurityPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_l_s_security_policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsOpenSearchServiceDomainEncryptionAtRestOptionsDetails {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsOpenSearchServiceDomainLogPublishingOptionsDetails {
    #[serde(rename = "AuditLogs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_logs: Option<AwsOpenSearchServiceDomainLogPublishingOption>,
    #[serde(rename = "IndexSlowLogs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_slow_logs: Option<AwsOpenSearchServiceDomainLogPublishingOption>,
    #[serde(rename = "SearchSlowLogs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_slow_logs: Option<AwsOpenSearchServiceDomainLogPublishingOption>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsOpenSearchServiceDomainLogPublishingOption {
    #[serde(rename = "CloudWatchLogsLogGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs_log_group_arn: Option<String>,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsOpenSearchServiceDomainNodeToNodeEncryptionOptionsDetails {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsOpenSearchServiceDomainServiceSoftwareOptionsDetails {
    #[serde(rename = "AutomatedUpdateDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automated_update_date: Option<String>,
    #[serde(rename = "Cancellable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellable: Option<bool>,
    #[serde(rename = "CurrentVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_version: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "NewVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_version: Option<String>,
    #[serde(rename = "OptionalDeployment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional_deployment: Option<bool>,
    #[serde(rename = "UpdateAvailable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_available: Option<bool>,
    #[serde(rename = "UpdateStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsOpenSearchServiceDomainVpcOptionsDetails {
    #[serde(rename = "SecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(rename = "SubnetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsRdsDbClusterDetails {
    #[serde(rename = "ActivityStreamStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_stream_status: Option<String>,
    #[serde(rename = "AllocatedStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_storage: Option<i32>,
    #[serde(rename = "AssociatedRoles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_roles: Option<Vec<AwsRdsDbClusterAssociatedRole>>,
    #[serde(rename = "AutoMinorVersionUpgrade")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_minor_version_upgrade: Option<bool>,
    #[serde(rename = "AvailabilityZones")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<Vec<String>>,
    #[serde(rename = "BackupRetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_retention_period: Option<i32>,
    #[serde(rename = "ClusterCreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_create_time: Option<String>,
    #[serde(rename = "CopyTagsToSnapshot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_tags_to_snapshot: Option<bool>,
    #[serde(rename = "CrossAccountClone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_account_clone: Option<bool>,
    #[serde(rename = "CustomEndpoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_endpoints: Option<Vec<String>>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(rename = "DbClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_cluster_identifier: Option<String>,
    #[serde(rename = "DbClusterMembers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_cluster_members: Option<Vec<AwsRdsDbClusterMember>>,
    #[serde(rename = "DbClusterOptionGroupMemberships")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_cluster_option_group_memberships: Option<Vec<AwsRdsDbClusterOptionGroupMembership>>,
    #[serde(rename = "DbClusterParameterGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_cluster_parameter_group: Option<String>,
    #[serde(rename = "DbClusterResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_cluster_resource_id: Option<String>,
    #[serde(rename = "DbSubnetGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_subnet_group: Option<String>,
    #[serde(rename = "DeletionProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection: Option<bool>,
    #[serde(rename = "DomainMemberships")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_memberships: Option<Vec<AwsRdsDbDomainMembership>>,
    #[serde(rename = "EnabledCloudWatchLogsExports")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_cloud_watch_logs_exports: Option<Vec<String>>,
    #[serde(rename = "Endpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "EngineMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_mode: Option<String>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "HostedZoneId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_zone_id: Option<String>,
    #[serde(rename = "HttpEndpointEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_endpoint_enabled: Option<bool>,
    #[serde(rename = "IamDatabaseAuthenticationEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_database_authentication_enabled: Option<bool>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "MasterUsername")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_username: Option<String>,
    #[serde(rename = "MultiAz")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_az: Option<bool>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "PreferredBackupWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_backup_window: Option<String>,
    #[serde(rename = "PreferredMaintenanceWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<String>,
    #[serde(rename = "ReadReplicaIdentifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_replica_identifiers: Option<Vec<String>>,
    #[serde(rename = "ReaderEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reader_endpoint: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StorageEncrypted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_encrypted: Option<bool>,
    #[serde(rename = "VpcSecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_groups: Option<Vec<AwsRdsDbInstanceVpcSecurityGroup>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsRdsDbClusterAssociatedRole {
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsRdsDbClusterMember {
    #[serde(rename = "DbClusterParameterGroupStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_cluster_parameter_group_status: Option<String>,
    #[serde(rename = "DbInstanceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_identifier: Option<String>,
    #[serde(rename = "IsClusterWriter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_cluster_writer: Option<bool>,
    #[serde(rename = "PromotionTier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_tier: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsRdsDbClusterOptionGroupMembership {
    #[serde(rename = "DbClusterOptionGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_cluster_option_group_name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsRdsDbDomainMembership {
    #[serde(rename = "Domain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "Fqdn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
    #[serde(rename = "IamRoleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role_name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsRdsDbInstanceVpcSecurityGroup {
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "VpcSecurityGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_group_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsRdsDbClusterSnapshotDetails {
    #[serde(rename = "AllocatedStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_storage: Option<i32>,
    #[serde(rename = "AvailabilityZones")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<Vec<String>>,
    #[serde(rename = "ClusterCreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_create_time: Option<String>,
    #[serde(rename = "DbClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_cluster_identifier: Option<String>,
    #[serde(rename = "DbClusterSnapshotAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_cluster_snapshot_attributes:
        Option<Vec<AwsRdsDbClusterSnapshotDbClusterSnapshotAttribute>>,
    #[serde(rename = "DbClusterSnapshotIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_cluster_snapshot_identifier: Option<String>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "IamDatabaseAuthenticationEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_database_authentication_enabled: Option<bool>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "LicenseModel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_model: Option<String>,
    #[serde(rename = "MasterUsername")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_username: Option<String>,
    #[serde(rename = "PercentProgress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_progress: Option<i32>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "SnapshotCreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_create_time: Option<String>,
    #[serde(rename = "SnapshotType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_type: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StorageEncrypted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_encrypted: Option<bool>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsRdsDbClusterSnapshotDbClusterSnapshotAttribute {
    #[serde(rename = "AttributeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_name: Option<String>,
    #[serde(rename = "AttributeValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsRdsDbInstanceDetails {
    #[serde(rename = "AllocatedStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_storage: Option<i32>,
    #[serde(rename = "AssociatedRoles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_roles: Option<Vec<AwsRdsDbInstanceAssociatedRole>>,
    #[serde(rename = "AutoMinorVersionUpgrade")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_minor_version_upgrade: Option<bool>,
    #[serde(rename = "AvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "BackupRetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_retention_period: Option<i32>,
    #[serde(rename = "CACertificateIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_a_certificate_identifier: Option<String>,
    #[serde(rename = "CharacterSetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_set_name: Option<String>,
    #[serde(rename = "CopyTagsToSnapshot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_tags_to_snapshot: Option<bool>,
    #[serde(rename = "DBClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_identifier: Option<String>,
    #[serde(rename = "DBInstanceClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance_class: Option<String>,
    #[serde(rename = "DBInstanceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance_identifier: Option<String>,
    #[serde(rename = "DBName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_name: Option<String>,
    #[serde(rename = "DbInstancePort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_port: Option<i32>,
    #[serde(rename = "DbInstanceStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_status: Option<String>,
    #[serde(rename = "DbParameterGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_parameter_groups: Option<Vec<AwsRdsDbParameterGroup>>,
    #[serde(rename = "DbSecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_security_groups: Option<Vec<String>>,
    #[serde(rename = "DbSubnetGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_subnet_group: Option<AwsRdsDbSubnetGroup>,
    #[serde(rename = "DbiResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbi_resource_id: Option<String>,
    #[serde(rename = "DeletionProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection: Option<bool>,
    #[serde(rename = "DomainMemberships")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_memberships: Option<Vec<AwsRdsDbDomainMembership>>,
    #[serde(rename = "EnabledCloudWatchLogsExports")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_cloud_watch_logs_exports: Option<Vec<String>>,
    #[serde(rename = "Endpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<AwsRdsDbInstanceEndpoint>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "EnhancedMonitoringResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enhanced_monitoring_resource_arn: Option<String>,
    #[serde(rename = "IAMDatabaseAuthenticationEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_a_m_database_authentication_enabled: Option<bool>,
    #[serde(rename = "InstanceCreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_create_time: Option<String>,
    #[serde(rename = "Iops")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iops: Option<i32>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "LatestRestorableTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_restorable_time: Option<String>,
    #[serde(rename = "LicenseModel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_model: Option<String>,
    #[serde(rename = "ListenerEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener_endpoint: Option<AwsRdsDbInstanceEndpoint>,
    #[serde(rename = "MasterUsername")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_username: Option<String>,
    #[serde(rename = "MaxAllocatedStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_allocated_storage: Option<i32>,
    #[serde(rename = "MonitoringInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_interval: Option<i32>,
    #[serde(rename = "MonitoringRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_role_arn: Option<String>,
    #[serde(rename = "MultiAz")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_az: Option<bool>,
    #[serde(rename = "OptionGroupMemberships")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_group_memberships: Option<Vec<AwsRdsDbOptionGroupMembership>>,
    #[serde(rename = "PendingModifiedValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_modified_values: Option<AwsRdsDbPendingModifiedValues>,
    #[serde(rename = "PerformanceInsightsEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_insights_enabled: Option<bool>,
    #[serde(rename = "PerformanceInsightsKmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_insights_kms_key_id: Option<String>,
    #[serde(rename = "PerformanceInsightsRetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_insights_retention_period: Option<i32>,
    #[serde(rename = "PreferredBackupWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_backup_window: Option<String>,
    #[serde(rename = "PreferredMaintenanceWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<String>,
    #[serde(rename = "ProcessorFeatures")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processor_features: Option<Vec<AwsRdsDbProcessorFeature>>,
    #[serde(rename = "PromotionTier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_tier: Option<i32>,
    #[serde(rename = "PubliclyAccessible")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<bool>,
    #[serde(rename = "ReadReplicaDBClusterIdentifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_replica_d_b_cluster_identifiers: Option<Vec<String>>,
    #[serde(rename = "ReadReplicaDBInstanceIdentifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_replica_d_b_instance_identifiers: Option<Vec<String>>,
    #[serde(rename = "ReadReplicaSourceDBInstanceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_replica_source_d_b_instance_identifier: Option<String>,
    #[serde(rename = "SecondaryAvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_availability_zone: Option<String>,
    #[serde(rename = "StatusInfos")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_infos: Option<Vec<AwsRdsDbStatusInfo>>,
    #[serde(rename = "StorageEncrypted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_encrypted: Option<bool>,
    #[serde(rename = "StorageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    #[serde(rename = "TdeCredentialArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tde_credential_arn: Option<String>,
    #[serde(rename = "Timezone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde(rename = "VpcSecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_groups: Option<Vec<AwsRdsDbInstanceVpcSecurityGroup>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsRdsDbInstanceAssociatedRole {
    #[serde(rename = "FeatureName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_name: Option<String>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsRdsDbParameterGroup {
    #[serde(rename = "DbParameterGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_parameter_group_name: Option<String>,
    #[serde(rename = "ParameterApplyStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_apply_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsRdsDbSubnetGroup {
    #[serde(rename = "DbSubnetGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_subnet_group_arn: Option<String>,
    #[serde(rename = "DbSubnetGroupDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_subnet_group_description: Option<String>,
    #[serde(rename = "DbSubnetGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_subnet_group_name: Option<String>,
    #[serde(rename = "SubnetGroupStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_group_status: Option<String>,
    #[serde(rename = "Subnets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnets: Option<Vec<AwsRdsDbSubnetGroupSubnet>>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsRdsDbSubnetGroupSubnet {
    #[serde(rename = "SubnetAvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_availability_zone: Option<AwsRdsDbSubnetGroupSubnetAvailabilityZone>,
    #[serde(rename = "SubnetIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_identifier: Option<String>,
    #[serde(rename = "SubnetStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsRdsDbSubnetGroupSubnetAvailabilityZone {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsRdsDbInstanceEndpoint {
    #[serde(rename = "Address")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "HostedZoneId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_zone_id: Option<String>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsRdsDbOptionGroupMembership {
    #[serde(rename = "OptionGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_group_name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsRdsDbPendingModifiedValues {
    #[serde(rename = "AllocatedStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_storage: Option<i32>,
    #[serde(rename = "BackupRetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_retention_period: Option<i32>,
    #[serde(rename = "CaCertificateIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ca_certificate_identifier: Option<String>,
    #[serde(rename = "DbInstanceClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_class: Option<String>,
    #[serde(rename = "DbInstanceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_identifier: Option<String>,
    #[serde(rename = "DbSubnetGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_subnet_group_name: Option<String>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "Iops")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iops: Option<i32>,
    #[serde(rename = "LicenseModel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_model: Option<String>,
    #[serde(rename = "MasterUserPassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_user_password: Option<String>,
    #[serde(rename = "MultiAZ")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_a_z: Option<bool>,
    #[serde(rename = "PendingCloudWatchLogsExports")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_cloud_watch_logs_exports: Option<AwsRdsPendingCloudWatchLogsExports>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "ProcessorFeatures")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processor_features: Option<Vec<AwsRdsDbProcessorFeature>>,
    #[serde(rename = "StorageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsRdsPendingCloudWatchLogsExports {
    #[serde(rename = "LogTypesToDisable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_types_to_disable: Option<Vec<String>>,
    #[serde(rename = "LogTypesToEnable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_types_to_enable: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsRdsDbProcessorFeature {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsRdsDbStatusInfo {
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "Normal")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normal: Option<bool>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsRdsDbSecurityGroupDetails {
    #[serde(rename = "DbSecurityGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_security_group_arn: Option<String>,
    #[serde(rename = "DbSecurityGroupDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_security_group_description: Option<String>,
    #[serde(rename = "DbSecurityGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_security_group_name: Option<String>,
    #[serde(rename = "Ec2SecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_security_groups: Option<Vec<AwsRdsDbSecurityGroupEc2SecurityGroup>>,
    #[serde(rename = "IpRanges")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_ranges: Option<Vec<AwsRdsDbSecurityGroupIpRange>>,
    #[serde(rename = "OwnerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsRdsDbSecurityGroupEc2SecurityGroup {
    #[serde(rename = "Ec2SecurityGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_security_group_id: Option<String>,
    #[serde(rename = "Ec2SecurityGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_security_group_name: Option<String>,
    #[serde(rename = "Ec2SecurityGroupOwnerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_security_group_owner_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsRdsDbSecurityGroupIpRange {
    #[serde(rename = "CidrIp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_ip: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsRdsDbSnapshotDetails {
    #[serde(rename = "AllocatedStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_storage: Option<i32>,
    #[serde(rename = "AvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "DbInstanceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_identifier: Option<String>,
    #[serde(rename = "DbSnapshotIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_snapshot_identifier: Option<String>,
    #[serde(rename = "DbiResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbi_resource_id: Option<String>,
    #[serde(rename = "Encrypted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "IamDatabaseAuthenticationEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_database_authentication_enabled: Option<bool>,
    #[serde(rename = "InstanceCreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_create_time: Option<String>,
    #[serde(rename = "Iops")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iops: Option<i32>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "LicenseModel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_model: Option<String>,
    #[serde(rename = "MasterUsername")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_username: Option<String>,
    #[serde(rename = "OptionGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_group_name: Option<String>,
    #[serde(rename = "PercentProgress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_progress: Option<i32>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "ProcessorFeatures")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processor_features: Option<Vec<AwsRdsDbProcessorFeature>>,
    #[serde(rename = "SnapshotCreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_create_time: Option<String>,
    #[serde(rename = "SnapshotType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_type: Option<String>,
    #[serde(rename = "SourceDbSnapshotIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_db_snapshot_identifier: Option<String>,
    #[serde(rename = "SourceRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_region: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StorageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    #[serde(rename = "TdeCredentialArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tde_credential_arn: Option<String>,
    #[serde(rename = "Timezone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsRdsEventSubscriptionDetails {
    #[serde(rename = "CustSubscriptionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cust_subscription_id: Option<String>,
    #[serde(rename = "CustomerAwsId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_aws_id: Option<String>,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "EventCategoriesList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_categories_list: Option<Vec<String>>,
    #[serde(rename = "EventSubscriptionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_subscription_arn: Option<String>,
    #[serde(rename = "SnsTopicArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_arn: Option<String>,
    #[serde(rename = "SourceIdsList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ids_list: Option<Vec<String>>,
    #[serde(rename = "SourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "SubscriptionCreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_creation_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsRedshiftClusterDetails {
    #[serde(rename = "AllowVersionUpgrade")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_version_upgrade: Option<bool>,
    #[serde(rename = "AutomatedSnapshotRetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automated_snapshot_retention_period: Option<i32>,
    #[serde(rename = "AvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "ClusterAvailabilityStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_availability_status: Option<String>,
    #[serde(rename = "ClusterCreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_create_time: Option<String>,
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_identifier: Option<String>,
    #[serde(rename = "ClusterNodes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_nodes: Option<Vec<AwsRedshiftClusterClusterNode>>,
    #[serde(rename = "ClusterParameterGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_parameter_groups: Option<Vec<AwsRedshiftClusterClusterParameterGroup>>,
    #[serde(rename = "ClusterPublicKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_public_key: Option<String>,
    #[serde(rename = "ClusterRevisionNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_revision_number: Option<String>,
    #[serde(rename = "ClusterSecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_security_groups: Option<Vec<AwsRedshiftClusterClusterSecurityGroup>>,
    #[serde(rename = "ClusterSnapshotCopyStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_snapshot_copy_status: Option<AwsRedshiftClusterClusterSnapshotCopyStatus>,
    #[serde(rename = "ClusterStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_status: Option<String>,
    #[serde(rename = "ClusterSubnetGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_subnet_group_name: Option<String>,
    #[serde(rename = "ClusterVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_version: Option<String>,
    #[serde(rename = "DBName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_name: Option<String>,
    #[serde(rename = "DeferredMaintenanceWindows")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deferred_maintenance_windows: Option<Vec<AwsRedshiftClusterDeferredMaintenanceWindow>>,
    #[serde(rename = "ElasticIpStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elastic_ip_status: Option<AwsRedshiftClusterElasticIpStatus>,
    #[serde(rename = "ElasticResizeNumberOfNodeOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elastic_resize_number_of_node_options: Option<String>,
    #[serde(rename = "Encrypted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    #[serde(rename = "Endpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<AwsRedshiftClusterEndpoint>,
    #[serde(rename = "EnhancedVpcRouting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enhanced_vpc_routing: Option<bool>,
    #[serde(rename = "ExpectedNextSnapshotScheduleTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_next_snapshot_schedule_time: Option<String>,
    #[serde(rename = "ExpectedNextSnapshotScheduleTimeStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_next_snapshot_schedule_time_status: Option<String>,
    #[serde(rename = "HsmStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsm_status: Option<AwsRedshiftClusterHsmStatus>,
    #[serde(rename = "IamRoles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_roles: Option<Vec<AwsRedshiftClusterIamRole>>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "LoggingStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_status: Option<AwsRedshiftClusterLoggingStatus>,
    #[serde(rename = "MaintenanceTrackName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_track_name: Option<String>,
    #[serde(rename = "ManualSnapshotRetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_snapshot_retention_period: Option<i32>,
    #[serde(rename = "MasterUsername")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_username: Option<String>,
    #[serde(rename = "NextMaintenanceWindowStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_maintenance_window_start_time: Option<String>,
    #[serde(rename = "NodeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
    #[serde(rename = "NumberOfNodes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_nodes: Option<i32>,
    #[serde(rename = "PendingActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_actions: Option<Vec<String>>,
    #[serde(rename = "PendingModifiedValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_modified_values: Option<AwsRedshiftClusterPendingModifiedValues>,
    #[serde(rename = "PreferredMaintenanceWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<String>,
    #[serde(rename = "PubliclyAccessible")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<bool>,
    #[serde(rename = "ResizeInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resize_info: Option<AwsRedshiftClusterResizeInfo>,
    #[serde(rename = "RestoreStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_status: Option<AwsRedshiftClusterRestoreStatus>,
    #[serde(rename = "SnapshotScheduleIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_schedule_identifier: Option<String>,
    #[serde(rename = "SnapshotScheduleState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_schedule_state: Option<String>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    #[serde(rename = "VpcSecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_groups: Option<Vec<AwsRedshiftClusterVpcSecurityGroup>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsRedshiftClusterClusterNode {
    #[serde(rename = "NodeRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_role: Option<String>,
    #[serde(rename = "PrivateIpAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<String>,
    #[serde(rename = "PublicIpAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_ip_address: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsRedshiftClusterClusterParameterGroup {
    #[serde(rename = "ClusterParameterStatusList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_parameter_status_list: Option<Vec<AwsRedshiftClusterClusterParameterStatus>>,
    #[serde(rename = "ParameterApplyStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_apply_status: Option<String>,
    #[serde(rename = "ParameterGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_group_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsRedshiftClusterClusterParameterStatus {
    #[serde(rename = "ParameterApplyErrorDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_apply_error_description: Option<String>,
    #[serde(rename = "ParameterApplyStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_apply_status: Option<String>,
    #[serde(rename = "ParameterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsRedshiftClusterClusterSecurityGroup {
    #[serde(rename = "ClusterSecurityGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_security_group_name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsRedshiftClusterClusterSnapshotCopyStatus {
    #[serde(rename = "DestinationRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_region: Option<String>,
    #[serde(rename = "ManualSnapshotRetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_snapshot_retention_period: Option<i32>,
    #[serde(rename = "RetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_period: Option<i32>,
    #[serde(rename = "SnapshotCopyGrantName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_copy_grant_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsRedshiftClusterDeferredMaintenanceWindow {
    #[serde(rename = "DeferMaintenanceEndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defer_maintenance_end_time: Option<String>,
    #[serde(rename = "DeferMaintenanceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defer_maintenance_identifier: Option<String>,
    #[serde(rename = "DeferMaintenanceStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defer_maintenance_start_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsRedshiftClusterElasticIpStatus {
    #[serde(rename = "ElasticIp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elastic_ip: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsRedshiftClusterEndpoint {
    #[serde(rename = "Address")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsRedshiftClusterHsmStatus {
    #[serde(rename = "HsmClientCertificateIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsm_client_certificate_identifier: Option<String>,
    #[serde(rename = "HsmConfigurationIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsm_configuration_identifier: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsRedshiftClusterIamRole {
    #[serde(rename = "ApplyStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_status: Option<String>,
    #[serde(rename = "IamRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsRedshiftClusterLoggingStatus {
    #[serde(rename = "BucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<String>,
    #[serde(rename = "LastFailureMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_failure_message: Option<String>,
    #[serde(rename = "LastFailureTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_failure_time: Option<String>,
    #[serde(rename = "LastSuccessfulDeliveryTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_successful_delivery_time: Option<String>,
    #[serde(rename = "LoggingEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_enabled: Option<bool>,
    #[serde(rename = "S3KeyPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key_prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsRedshiftClusterPendingModifiedValues {
    #[serde(rename = "AutomatedSnapshotRetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automated_snapshot_retention_period: Option<i32>,
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_identifier: Option<String>,
    #[serde(rename = "ClusterType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_type: Option<String>,
    #[serde(rename = "ClusterVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_version: Option<String>,
    #[serde(rename = "EncryptionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_type: Option<String>,
    #[serde(rename = "EnhancedVpcRouting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enhanced_vpc_routing: Option<bool>,
    #[serde(rename = "MaintenanceTrackName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_track_name: Option<String>,
    #[serde(rename = "MasterUserPassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_user_password: Option<String>,
    #[serde(rename = "NodeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
    #[serde(rename = "NumberOfNodes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_nodes: Option<i32>,
    #[serde(rename = "PubliclyAccessible")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsRedshiftClusterResizeInfo {
    #[serde(rename = "AllowCancelResize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_cancel_resize: Option<bool>,
    #[serde(rename = "ResizeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resize_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsRedshiftClusterRestoreStatus {
    #[serde(rename = "CurrentRestoreRateInMegaBytesPerSecond")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_restore_rate_in_mega_bytes_per_second: Option<f64>,
    #[serde(rename = "ElapsedTimeInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elapsed_time_in_seconds: Option<i64>,
    #[serde(rename = "EstimatedTimeToCompletionInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_time_to_completion_in_seconds: Option<i64>,
    #[serde(rename = "ProgressInMegaBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_in_mega_bytes: Option<i64>,
    #[serde(rename = "SnapshotSizeInMegaBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_size_in_mega_bytes: Option<i64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsRedshiftClusterVpcSecurityGroup {
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "VpcSecurityGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_group_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsRoute53HostedZoneDetails {
    #[serde(rename = "HostedZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_zone: Option<AwsRoute53HostedZoneObjectDetails>,
    #[serde(rename = "NameServers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_servers: Option<Vec<String>>,
    #[serde(rename = "QueryLoggingConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_logging_config: Option<AwsRoute53QueryLoggingConfigDetails>,
    #[serde(rename = "Vpcs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpcs: Option<Vec<AwsRoute53HostedZoneVpcDetails>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsRoute53HostedZoneObjectDetails {
    #[serde(rename = "Config")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<AwsRoute53HostedZoneConfigDetails>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsRoute53HostedZoneConfigDetails {
    #[serde(rename = "Comment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsRoute53QueryLoggingConfigDetails {
    #[serde(rename = "CloudWatchLogsLogGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs_log_group_arn: Option<CloudWatchLogsLogGroupArnConfigDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CloudWatchLogsLogGroupArnConfigDetails {
    #[serde(rename = "CloudWatchLogsLogGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs_log_group_arn: Option<String>,
    #[serde(rename = "HostedZoneId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_zone_id: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsRoute53HostedZoneVpcDetails {
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Region")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsS3AccessPointDetails {
    #[serde(rename = "AccessPointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_point_arn: Option<String>,
    #[serde(rename = "Alias")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(rename = "Bucket")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    #[serde(rename = "BucketAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_account_id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "NetworkOrigin")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_origin: Option<String>,
    #[serde(rename = "PublicAccessBlockConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_access_block_configuration: Option<AwsS3AccountPublicAccessBlockDetails>,
    #[serde(rename = "VpcConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_configuration: Option<AwsS3AccessPointVpcConfigurationDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsS3AccountPublicAccessBlockDetails {
    #[serde(rename = "BlockPublicAcls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_public_acls: Option<bool>,
    #[serde(rename = "BlockPublicPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_public_policy: Option<bool>,
    #[serde(rename = "IgnorePublicAcls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_public_acls: Option<bool>,
    #[serde(rename = "RestrictPublicBuckets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrict_public_buckets: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsS3AccessPointVpcConfigurationDetails {
    #[serde(rename = "VpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsS3BucketDetails {
    #[serde(rename = "AccessControlList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_control_list: Option<String>,
    #[serde(rename = "BucketLifecycleConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_lifecycle_configuration: Option<AwsS3BucketBucketLifecycleConfigurationDetails>,
    #[serde(rename = "BucketLoggingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_logging_configuration: Option<AwsS3BucketLoggingConfiguration>,
    #[serde(rename = "BucketNotificationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_notification_configuration: Option<AwsS3BucketNotificationConfiguration>,
    #[serde(rename = "BucketVersioningConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_versioning_configuration: Option<AwsS3BucketBucketVersioningConfiguration>,
    #[serde(rename = "BucketWebsiteConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_website_configuration: Option<AwsS3BucketWebsiteConfiguration>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ObjectLockConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_lock_configuration: Option<AwsS3BucketObjectLockConfiguration>,
    #[serde(rename = "OwnerAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_account_id: Option<String>,
    #[serde(rename = "OwnerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(rename = "OwnerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_name: Option<String>,
    #[serde(rename = "PublicAccessBlockConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_access_block_configuration: Option<AwsS3AccountPublicAccessBlockDetails>,
    #[serde(rename = "ServerSideEncryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_side_encryption_configuration: Option<AwsS3BucketServerSideEncryptionConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsS3BucketBucketLifecycleConfigurationDetails {
    #[serde(rename = "Rules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<AwsS3BucketBucketLifecycleConfigurationRulesDetails>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsS3BucketBucketLifecycleConfigurationRulesDetails {
    #[serde(rename = "AbortIncompleteMultipartUpload")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abort_incomplete_multipart_upload:
        Option<AwsS3BucketBucketLifecycleConfigurationRulesAbortIncompleteMultipartUploadDetails>,
    #[serde(rename = "ExpirationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    #[serde(rename = "ExpirationInDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_in_days: Option<i32>,
    #[serde(rename = "ExpiredObjectDeleteMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired_object_delete_marker: Option<bool>,
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<AwsS3BucketBucketLifecycleConfigurationRulesFilterDetails>,
    #[serde(rename = "ID")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_d: Option<String>,
    #[serde(rename = "NoncurrentVersionExpirationInDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub noncurrent_version_expiration_in_days: Option<i32>,
    #[serde(rename = "NoncurrentVersionTransitions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub noncurrent_version_transitions: Option<
        Vec<AwsS3BucketBucketLifecycleConfigurationRulesNoncurrentVersionTransitionsDetails>,
    >,
    #[serde(rename = "Prefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Transitions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transitions: Option<Vec<AwsS3BucketBucketLifecycleConfigurationRulesTransitionsDetails>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsS3BucketBucketLifecycleConfigurationRulesAbortIncompleteMultipartUploadDetails {
    #[serde(rename = "DaysAfterInitiation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_after_initiation: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsS3BucketBucketLifecycleConfigurationRulesFilterDetails {
    #[serde(rename = "Predicate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predicate: Option<AwsS3BucketBucketLifecycleConfigurationRulesFilterPredicateDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsS3BucketBucketLifecycleConfigurationRulesFilterPredicateDetails {
    #[serde(rename = "Operands")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operands:
        Option<Vec<AwsS3BucketBucketLifecycleConfigurationRulesFilterPredicateOperandsDetails>>,
    #[serde(rename = "Prefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "Tag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<AwsS3BucketBucketLifecycleConfigurationRulesFilterPredicateTagDetails>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsS3BucketBucketLifecycleConfigurationRulesFilterPredicateOperandsDetails {
    #[serde(rename = "Prefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "Tag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<AwsS3BucketBucketLifecycleConfigurationRulesFilterPredicateOperandsTagDetails>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsS3BucketBucketLifecycleConfigurationRulesFilterPredicateOperandsTagDetails {
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
pub struct AwsS3BucketBucketLifecycleConfigurationRulesFilterPredicateTagDetails {
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
pub struct AwsS3BucketBucketLifecycleConfigurationRulesNoncurrentVersionTransitionsDetails {
    #[serde(rename = "Days")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days: Option<i32>,
    #[serde(rename = "StorageClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_class: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsS3BucketBucketLifecycleConfigurationRulesTransitionsDetails {
    #[serde(rename = "Date")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(rename = "Days")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days: Option<i32>,
    #[serde(rename = "StorageClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_class: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsS3BucketLoggingConfiguration {
    #[serde(rename = "DestinationBucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_bucket_name: Option<String>,
    #[serde(rename = "LogFilePrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_file_prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsS3BucketNotificationConfiguration {
    #[serde(rename = "Configurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configurations: Option<Vec<AwsS3BucketNotificationConfigurationDetail>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsS3BucketNotificationConfigurationDetail {
    #[serde(rename = "Destination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    #[serde(rename = "Events")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<String>>,
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<AwsS3BucketNotificationConfigurationFilter>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsS3BucketNotificationConfigurationFilter {
    #[serde(rename = "S3KeyFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key_filter: Option<AwsS3BucketNotificationConfigurationS3KeyFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsS3BucketNotificationConfigurationS3KeyFilter {
    #[serde(rename = "FilterRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_rules: Option<Vec<AwsS3BucketNotificationConfigurationS3KeyFilterRule>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsS3BucketNotificationConfigurationS3KeyFilterRule {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsS3BucketBucketVersioningConfiguration {
    #[serde(rename = "IsMfaDeleteEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_mfa_delete_enabled: Option<bool>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsS3BucketWebsiteConfiguration {
    #[serde(rename = "ErrorDocument")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_document: Option<String>,
    #[serde(rename = "IndexDocumentSuffix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_document_suffix: Option<String>,
    #[serde(rename = "RedirectAllRequestsTo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_all_requests_to: Option<AwsS3BucketWebsiteConfigurationRedirectTo>,
    #[serde(rename = "RoutingRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_rules: Option<Vec<AwsS3BucketWebsiteConfigurationRoutingRule>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsS3BucketWebsiteConfigurationRedirectTo {
    #[serde(rename = "Hostname")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(rename = "Protocol")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsS3BucketWebsiteConfigurationRoutingRule {
    #[serde(rename = "Condition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<AwsS3BucketWebsiteConfigurationRoutingRuleCondition>,
    #[serde(rename = "Redirect")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect: Option<AwsS3BucketWebsiteConfigurationRoutingRuleRedirect>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsS3BucketWebsiteConfigurationRoutingRuleCondition {
    #[serde(rename = "HttpErrorCodeReturnedEquals")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_error_code_returned_equals: Option<String>,
    #[serde(rename = "KeyPrefixEquals")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_prefix_equals: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsS3BucketWebsiteConfigurationRoutingRuleRedirect {
    #[serde(rename = "Hostname")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(rename = "HttpRedirectCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_redirect_code: Option<String>,
    #[serde(rename = "Protocol")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(rename = "ReplaceKeyPrefixWith")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace_key_prefix_with: Option<String>,
    #[serde(rename = "ReplaceKeyWith")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace_key_with: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsS3BucketObjectLockConfiguration {
    #[serde(rename = "ObjectLockEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_lock_enabled: Option<String>,
    #[serde(rename = "Rule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule: Option<AwsS3BucketObjectLockConfigurationRuleDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsS3BucketObjectLockConfigurationRuleDetails {
    #[serde(rename = "DefaultRetention")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_retention: Option<AwsS3BucketObjectLockConfigurationRuleDefaultRetentionDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsS3BucketObjectLockConfigurationRuleDefaultRetentionDetails {
    #[serde(rename = "Days")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days: Option<i32>,
    #[serde(rename = "Mode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(rename = "Years")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub years: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsS3BucketServerSideEncryptionConfiguration {
    #[serde(rename = "Rules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<AwsS3BucketServerSideEncryptionRule>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsS3BucketServerSideEncryptionRule {
    #[serde(rename = "ApplyServerSideEncryptionByDefault")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_server_side_encryption_by_default: Option<AwsS3BucketServerSideEncryptionByDefault>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsS3BucketServerSideEncryptionByDefault {
    #[serde(rename = "KMSMasterKeyID")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k_m_s_master_key_i_d: Option<String>,
    #[serde(rename = "SSEAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_e_algorithm: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsS3ObjectDetails {
    #[serde(rename = "ContentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(rename = "ETag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[serde(rename = "LastModified")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
    #[serde(rename = "SSEKMSKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_e_k_m_s_key_id: Option<String>,
    #[serde(rename = "ServerSideEncryption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_side_encryption: Option<String>,
    #[serde(rename = "VersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsSageMakerNotebookInstanceDetails {
    #[serde(rename = "AcceleratorTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accelerator_types: Option<Vec<String>>,
    #[serde(rename = "AdditionalCodeRepositories")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_code_repositories: Option<Vec<String>>,
    #[serde(rename = "DefaultCodeRepository")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_code_repository: Option<String>,
    #[serde(rename = "DirectInternetAccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_internet_access: Option<String>,
    #[serde(rename = "FailureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "InstanceMetadataServiceConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_metadata_service_configuration:
        Option<AwsSageMakerNotebookInstanceMetadataServiceConfigurationDetails>,
    #[serde(rename = "InstanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "NetworkInterfaceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_id: Option<String>,
    #[serde(rename = "NotebookInstanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notebook_instance_arn: Option<String>,
    #[serde(rename = "NotebookInstanceLifecycleConfigName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notebook_instance_lifecycle_config_name: Option<String>,
    #[serde(rename = "NotebookInstanceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notebook_instance_name: Option<String>,
    #[serde(rename = "NotebookInstanceStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notebook_instance_status: Option<String>,
    #[serde(rename = "PlatformIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_identifier: Option<String>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "RootAccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_access: Option<String>,
    #[serde(rename = "SecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,
    #[serde(rename = "SubnetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    #[serde(rename = "Url")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "VolumeSizeInGB")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_size_in_g_b: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsSageMakerNotebookInstanceMetadataServiceConfigurationDetails {
    #[serde(rename = "MinimumInstanceMetadataServiceVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_instance_metadata_service_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsSecretsManagerSecretDetails {
    #[serde(rename = "Deleted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "RotationEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_enabled: Option<bool>,
    #[serde(rename = "RotationLambdaArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_lambda_arn: Option<String>,
    #[serde(rename = "RotationOccurredWithinFrequency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_occurred_within_frequency: Option<bool>,
    #[serde(rename = "RotationRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_rules: Option<AwsSecretsManagerSecretRotationRules>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsSecretsManagerSecretRotationRules {
    #[serde(rename = "AutomaticallyAfterDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatically_after_days: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsSnsTopicDetails {
    #[serde(rename = "ApplicationSuccessFeedbackRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_success_feedback_role_arn: Option<String>,
    #[serde(rename = "FirehoseFailureFeedbackRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firehose_failure_feedback_role_arn: Option<String>,
    #[serde(rename = "FirehoseSuccessFeedbackRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firehose_success_feedback_role_arn: Option<String>,
    #[serde(rename = "HttpFailureFeedbackRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_failure_feedback_role_arn: Option<String>,
    #[serde(rename = "HttpSuccessFeedbackRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_success_feedback_role_arn: Option<String>,
    #[serde(rename = "KmsMasterKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_master_key_id: Option<String>,
    #[serde(rename = "Owner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(rename = "SqsFailureFeedbackRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sqs_failure_feedback_role_arn: Option<String>,
    #[serde(rename = "SqsSuccessFeedbackRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sqs_success_feedback_role_arn: Option<String>,
    #[serde(rename = "Subscription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<Vec<AwsSnsTopicSubscription>>,
    #[serde(rename = "TopicName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsSnsTopicSubscription {
    #[serde(rename = "Endpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(rename = "Protocol")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsSqsQueueDetails {
    #[serde(rename = "DeadLetterTargetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dead_letter_target_arn: Option<String>,
    #[serde(rename = "KmsDataKeyReusePeriodSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_data_key_reuse_period_seconds: Option<i32>,
    #[serde(rename = "KmsMasterKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_master_key_id: Option<String>,
    #[serde(rename = "QueueName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsSsmPatchComplianceDetails {
    #[serde(rename = "Patch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch: Option<AwsSsmPatch>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsSsmPatch {
    #[serde(rename = "ComplianceSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_summary: Option<AwsSsmComplianceSummary>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsSsmComplianceSummary {
    #[serde(rename = "ComplianceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_type: Option<String>,
    #[serde(rename = "CompliantCriticalCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliant_critical_count: Option<i32>,
    #[serde(rename = "CompliantHighCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliant_high_count: Option<i32>,
    #[serde(rename = "CompliantInformationalCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliant_informational_count: Option<i32>,
    #[serde(rename = "CompliantLowCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliant_low_count: Option<i32>,
    #[serde(rename = "CompliantMediumCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliant_medium_count: Option<i32>,
    #[serde(rename = "CompliantUnspecifiedCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliant_unspecified_count: Option<i32>,
    #[serde(rename = "ExecutionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_type: Option<String>,
    #[serde(rename = "NonCompliantCriticalCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_compliant_critical_count: Option<i32>,
    #[serde(rename = "NonCompliantHighCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_compliant_high_count: Option<i32>,
    #[serde(rename = "NonCompliantInformationalCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_compliant_informational_count: Option<i32>,
    #[serde(rename = "NonCompliantLowCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_compliant_low_count: Option<i32>,
    #[serde(rename = "NonCompliantMediumCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_compliant_medium_count: Option<i32>,
    #[serde(rename = "NonCompliantUnspecifiedCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_compliant_unspecified_count: Option<i32>,
    #[serde(rename = "OverallSeverity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall_severity: Option<String>,
    #[serde(rename = "PatchBaselineId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_baseline_id: Option<String>,
    #[serde(rename = "PatchGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_group: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsStepFunctionStateMachineDetails {
    #[serde(rename = "Label")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "LoggingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_configuration: Option<AwsStepFunctionStateMachineLoggingConfigurationDetails>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "StateMachineArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_machine_arn: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TracingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracing_configuration: Option<AwsStepFunctionStateMachineTracingConfigurationDetails>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsStepFunctionStateMachineLoggingConfigurationDetails {
    #[serde(rename = "Destinations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations:
        Option<Vec<AwsStepFunctionStateMachineLoggingConfigurationDestinationsDetails>>,
    #[serde(rename = "IncludeExecutionData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_execution_data: Option<bool>,
    #[serde(rename = "Level")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsStepFunctionStateMachineLoggingConfigurationDestinationsDetails {
    #[serde(rename = "CloudWatchLogsLogGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs_log_group: Option<
        AwsStepFunctionStateMachineLoggingConfigurationDestinationsCloudWatchLogsLogGroupDetails,
    >,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsStepFunctionStateMachineLoggingConfigurationDestinationsCloudWatchLogsLogGroupDetails
{
    #[serde(rename = "LogGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsStepFunctionStateMachineTracingConfigurationDetails {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsWafRateBasedRuleDetails {
    #[serde(rename = "MatchPredicates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_predicates: Option<Vec<AwsWafRateBasedRuleMatchPredicate>>,
    #[serde(rename = "MetricName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "RateKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_key: Option<String>,
    #[serde(rename = "RateLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit: Option<i64>,
    #[serde(rename = "RuleId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsWafRateBasedRuleMatchPredicate {
    #[serde(rename = "DataId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_id: Option<String>,
    #[serde(rename = "Negated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negated: Option<bool>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsWafRegionalRateBasedRuleDetails {
    #[serde(rename = "MatchPredicates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_predicates: Option<Vec<AwsWafRegionalRateBasedRuleMatchPredicate>>,
    #[serde(rename = "MetricName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "RateKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_key: Option<String>,
    #[serde(rename = "RateLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit: Option<i64>,
    #[serde(rename = "RuleId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsWafRegionalRateBasedRuleMatchPredicate {
    #[serde(rename = "DataId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_id: Option<String>,
    #[serde(rename = "Negated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negated: Option<bool>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsWafRegionalRuleDetails {
    #[serde(rename = "MetricName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "PredicateList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predicate_list: Option<Vec<AwsWafRegionalRulePredicateListDetails>>,
    #[serde(rename = "RuleId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsWafRegionalRulePredicateListDetails {
    #[serde(rename = "DataId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_id: Option<String>,
    #[serde(rename = "Negated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negated: Option<bool>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsWafRegionalRuleGroupDetails {
    #[serde(rename = "MetricName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "RuleGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_group_id: Option<String>,
    #[serde(rename = "Rules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<AwsWafRegionalRuleGroupRulesDetails>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsWafRegionalRuleGroupRulesDetails {
    #[serde(rename = "Action")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<AwsWafRegionalRuleGroupRulesActionDetails>,
    #[serde(rename = "Priority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(rename = "RuleId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsWafRegionalRuleGroupRulesActionDetails {
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsWafRegionalWebAclDetails {
    #[serde(rename = "DefaultAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_action: Option<String>,
    #[serde(rename = "MetricName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "RulesList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules_list: Option<Vec<AwsWafRegionalWebAclRulesListDetails>>,
    #[serde(rename = "WebAclId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_acl_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsWafRegionalWebAclRulesListDetails {
    #[serde(rename = "Action")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<AwsWafRegionalWebAclRulesListActionDetails>,
    #[serde(rename = "OverrideAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_action: Option<AwsWafRegionalWebAclRulesListOverrideActionDetails>,
    #[serde(rename = "Priority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(rename = "RuleId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsWafRegionalWebAclRulesListActionDetails {
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsWafRegionalWebAclRulesListOverrideActionDetails {
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsWafRuleDetails {
    #[serde(rename = "MetricName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "PredicateList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predicate_list: Option<Vec<AwsWafRulePredicateListDetails>>,
    #[serde(rename = "RuleId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsWafRulePredicateListDetails {
    #[serde(rename = "DataId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_id: Option<String>,
    #[serde(rename = "Negated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negated: Option<bool>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsWafRuleGroupDetails {
    #[serde(rename = "MetricName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "RuleGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_group_id: Option<String>,
    #[serde(rename = "Rules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<AwsWafRuleGroupRulesDetails>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsWafRuleGroupRulesDetails {
    #[serde(rename = "Action")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<AwsWafRuleGroupRulesActionDetails>,
    #[serde(rename = "Priority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(rename = "RuleId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsWafRuleGroupRulesActionDetails {
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsWafWebAclDetails {
    #[serde(rename = "DefaultAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_action: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Rules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<AwsWafWebAclRule>>,
    #[serde(rename = "WebAclId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_acl_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsWafWebAclRule {
    #[serde(rename = "Action")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<WafAction>,
    #[serde(rename = "ExcludedRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_rules: Option<Vec<WafExcludedRule>>,
    #[serde(rename = "OverrideAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_action: Option<WafOverrideAction>,
    #[serde(rename = "Priority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(rename = "RuleId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WafAction {
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WafExcludedRule {
    #[serde(rename = "RuleId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WafOverrideAction {
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsWafv2RuleGroupDetails {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Capacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Rules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<AwsWafv2RulesDetails>>,
    #[serde(rename = "Scope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(rename = "VisibilityConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility_config: Option<AwsWafv2VisibilityConfigDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsWafv2RulesDetails {
    #[serde(rename = "Action")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<AwsWafv2RulesActionDetails>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "OverrideAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_action: Option<String>,
    #[serde(rename = "Priority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(rename = "VisibilityConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility_config: Option<AwsWafv2VisibilityConfigDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsWafv2RulesActionDetails {
    #[serde(rename = "Allow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow: Option<AwsWafv2ActionAllowDetails>,
    #[serde(rename = "Block")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block: Option<AwsWafv2ActionBlockDetails>,
    #[serde(rename = "Captcha")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captcha: Option<AwsWafv2RulesActionCaptchaDetails>,
    #[serde(rename = "Count")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<AwsWafv2RulesActionCountDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsWafv2ActionAllowDetails {
    #[serde(rename = "CustomRequestHandling")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_request_handling: Option<AwsWafv2CustomRequestHandlingDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsWafv2CustomRequestHandlingDetails {
    #[serde(rename = "InsertHeaders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insert_headers: Option<Vec<AwsWafv2CustomHttpHeader>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsWafv2CustomHttpHeader {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsWafv2ActionBlockDetails {
    #[serde(rename = "CustomResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_response: Option<AwsWafv2CustomResponseDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsWafv2CustomResponseDetails {
    #[serde(rename = "CustomResponseBodyKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_response_body_key: Option<String>,
    #[serde(rename = "ResponseCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_code: Option<i32>,
    #[serde(rename = "ResponseHeaders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_headers: Option<Vec<AwsWafv2CustomHttpHeader>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsWafv2RulesActionCaptchaDetails {
    #[serde(rename = "CustomRequestHandling")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_request_handling: Option<AwsWafv2CustomRequestHandlingDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsWafv2RulesActionCountDetails {
    #[serde(rename = "CustomRequestHandling")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_request_handling: Option<AwsWafv2CustomRequestHandlingDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsWafv2VisibilityConfigDetails {
    #[serde(rename = "CloudWatchMetricsEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_metrics_enabled: Option<bool>,
    #[serde(rename = "MetricName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    #[serde(rename = "SampledRequestsEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampled_requests_enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsWafv2WebAclDetails {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Capacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i64>,
    #[serde(rename = "CaptchaConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captcha_config: Option<AwsWafv2WebAclCaptchaConfigDetails>,
    #[serde(rename = "DefaultAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_action: Option<AwsWafv2WebAclActionDetails>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "ManagedbyFirewallManager")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managedby_firewall_manager: Option<bool>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Rules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<AwsWafv2RulesDetails>>,
    #[serde(rename = "VisibilityConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility_config: Option<AwsWafv2VisibilityConfigDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsWafv2WebAclCaptchaConfigDetails {
    #[serde(rename = "ImmunityTimeProperty")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub immunity_time_property: Option<AwsWafv2WebAclCaptchaConfigImmunityTimePropertyDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsWafv2WebAclCaptchaConfigImmunityTimePropertyDetails {
    #[serde(rename = "ImmunityTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub immunity_time: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsWafv2WebAclActionDetails {
    #[serde(rename = "Allow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow: Option<AwsWafv2ActionAllowDetails>,
    #[serde(rename = "Block")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block: Option<AwsWafv2ActionBlockDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsXrayEncryptionConfigDetails {
    #[serde(rename = "KeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CodeRepositoryDetails {
    #[serde(rename = "CodeSecurityIntegrationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_security_integration_arn: Option<String>,
    #[serde(rename = "ProjectName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_name: Option<String>,
    #[serde(rename = "ProviderType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContainerDetails {
    #[serde(rename = "ContainerRuntime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_runtime: Option<String>,
    #[serde(rename = "ImageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    #[serde(rename = "ImageName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_name: Option<String>,
    #[serde(rename = "LaunchedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launched_at: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Privileged")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,
    #[serde(rename = "VolumeMounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_mounts: Option<Vec<VolumeMount>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VolumeMount {
    #[serde(rename = "MountPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_path: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Severity {
    #[serde(rename = "Label")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "Normalized")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normalized: Option<i32>,
    #[serde(rename = "Original")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original: Option<String>,
    #[serde(rename = "Product")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ThreatIntelIndicator {
    #[serde(rename = "Category")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "LastObservedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_observed_at: Option<String>,
    #[serde(rename = "Source")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "SourceUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Threat {
    #[serde(rename = "FilePaths")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_paths: Option<Vec<FilePaths>>,
    #[serde(rename = "ItemCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_count: Option<i32>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Severity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FilePaths {
    #[serde(rename = "FileName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(rename = "FilePath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    #[serde(rename = "Hash")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Vulnerability {
    #[serde(rename = "CodeVulnerabilities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_vulnerabilities: Option<Vec<VulnerabilityCodeVulnerabilities>>,
    #[serde(rename = "Cvss")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvss: Option<Vec<Cvss>>,
    #[serde(rename = "EpssScore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub epss_score: Option<f64>,
    #[serde(rename = "ExploitAvailable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exploit_available: Option<String>,
    #[serde(rename = "FixAvailable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fix_available: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "LastKnownExploitAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_known_exploit_at: Option<String>,
    #[serde(rename = "ReferenceUrls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_urls: Option<Vec<String>>,
    #[serde(rename = "RelatedVulnerabilities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_vulnerabilities: Option<Vec<String>>,
    #[serde(rename = "Vendor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<VulnerabilityVendor>,
    #[serde(rename = "VulnerablePackages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vulnerable_packages: Option<Vec<SoftwarePackage>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VulnerabilityCodeVulnerabilities {
    #[serde(rename = "Cwes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cwes: Option<Vec<String>>,
    #[serde(rename = "FilePath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<CodeVulnerabilitiesFilePath>,
    #[serde(rename = "SourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CodeVulnerabilitiesFilePath {
    #[serde(rename = "EndLine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_line: Option<i32>,
    #[serde(rename = "FileName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(rename = "FilePath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    #[serde(rename = "StartLine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_line: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Cvss {
    #[serde(rename = "Adjustments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjustments: Option<Vec<Adjustment>>,
    #[serde(rename = "BaseScore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_score: Option<f64>,
    #[serde(rename = "BaseVector")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_vector: Option<String>,
    #[serde(rename = "Source")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Adjustment {
    #[serde(rename = "Metric")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric: Option<String>,
    #[serde(rename = "Reason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VulnerabilityVendor {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Url")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "VendorCreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_created_at: Option<String>,
    #[serde(rename = "VendorSeverity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_severity: Option<String>,
    #[serde(rename = "VendorUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SoftwarePackage {
    #[serde(rename = "Architecture")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    #[serde(rename = "Epoch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub epoch: Option<String>,
    #[serde(rename = "FilePath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    #[serde(rename = "FixedInVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_in_version: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "PackageManager")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_manager: Option<String>,
    #[serde(rename = "Release")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release: Option<String>,
    #[serde(rename = "Remediation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remediation: Option<String>,
    #[serde(rename = "SourceLayerArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_layer_arn: Option<String>,
    #[serde(rename = "SourceLayerHash")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_layer_hash: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Workflow {
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchImportFindingsResponse {
    #[serde(rename = "FailedCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_count: Option<i32>,
    #[serde(rename = "FailedFindings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_findings: Option<Vec<ImportFindingsError>>,
    #[serde(rename = "SuccessCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportFindingsError {
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchUpdateAutomationRulesRequest {
    #[serde(rename = "UpdateAutomationRulesRequestItems")]
    #[serde(default)]
    pub update_automation_rules_request_items: Vec<UpdateAutomationRulesRequestItem>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAutomationRulesRequestItem {
    #[serde(rename = "Actions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<AutomationRulesAction>>,
    #[serde(rename = "Criteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub criteria: Option<AutomationRulesFindingFilters>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "IsTerminal")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_terminal: Option<bool>,
    #[serde(rename = "RuleArn")]
    #[serde(default)]
    pub rule_arn: String,
    #[serde(rename = "RuleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
    #[serde(rename = "RuleOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_order: Option<i32>,
    #[serde(rename = "RuleStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchUpdateAutomationRulesResponse {
    #[serde(rename = "ProcessedAutomationRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processed_automation_rules: Option<Vec<String>>,
    #[serde(rename = "UnprocessedAutomationRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_automation_rules: Option<Vec<UnprocessedAutomationRule>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchUpdateFindingsRequest {
    #[serde(rename = "Confidence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<i32>,
    #[serde(rename = "Criticality")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub criticality: Option<i32>,
    #[serde(rename = "FindingIdentifiers")]
    #[serde(default)]
    pub finding_identifiers: Vec<AwsSecurityFindingIdentifier>,
    #[serde(rename = "Note")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<NoteUpdate>,
    #[serde(rename = "RelatedFindings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_findings: Option<Vec<RelatedFinding>>,
    #[serde(rename = "Severity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<SeverityUpdate>,
    #[serde(rename = "Types")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<String>>,
    #[serde(rename = "UserDefinedFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_defined_fields: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "VerificationState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_state: Option<String>,
    #[serde(rename = "Workflow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow: Option<WorkflowUpdate>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsSecurityFindingIdentifier {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "ProductArn")]
    #[serde(default)]
    pub product_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchUpdateFindingsResponse {
    #[serde(rename = "ProcessedFindings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processed_findings: Option<Vec<AwsSecurityFindingIdentifier>>,
    #[serde(rename = "UnprocessedFindings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_findings: Option<Vec<BatchUpdateFindingsUnprocessedFinding>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchUpdateFindingsUnprocessedFinding {
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "FindingIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_identifier: Option<AwsSecurityFindingIdentifier>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchUpdateFindingsV2Request {
    #[serde(rename = "Comment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "FindingIdentifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_identifiers: Option<Vec<OcsfFindingIdentifier>>,
    #[serde(rename = "MetadataUids")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_uids: Option<Vec<String>>,
    #[serde(rename = "SeverityId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity_id: Option<i32>,
    #[serde(rename = "StatusId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_id: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OcsfFindingIdentifier {
    #[serde(rename = "CloudAccountUid")]
    #[serde(default)]
    pub cloud_account_uid: String,
    #[serde(rename = "FindingInfoUid")]
    #[serde(default)]
    pub finding_info_uid: String,
    #[serde(rename = "MetadataProductUid")]
    #[serde(default)]
    pub metadata_product_uid: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchUpdateFindingsV2Response {
    #[serde(rename = "ProcessedFindings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processed_findings: Option<Vec<BatchUpdateFindingsV2ProcessedFinding>>,
    #[serde(rename = "UnprocessedFindings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_findings: Option<Vec<BatchUpdateFindingsV2UnprocessedFinding>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchUpdateFindingsV2ProcessedFinding {
    #[serde(rename = "FindingIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_identifier: Option<OcsfFindingIdentifier>,
    #[serde(rename = "MetadataUid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_uid: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchUpdateFindingsV2UnprocessedFinding {
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "FindingIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_identifier: Option<OcsfFindingIdentifier>,
    #[serde(rename = "MetadataUid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_uid: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchUpdateStandardsControlAssociationsRequest {
    #[serde(rename = "StandardsControlAssociationUpdates")]
    #[serde(default)]
    pub standards_control_association_updates: Vec<StandardsControlAssociationUpdate>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StandardsControlAssociationUpdate {
    #[serde(rename = "AssociationStatus")]
    #[serde(default)]
    pub association_status: String,
    #[serde(rename = "SecurityControlId")]
    #[serde(default)]
    pub security_control_id: String,
    #[serde(rename = "StandardsArn")]
    #[serde(default)]
    pub standards_arn: String,
    #[serde(rename = "UpdatedReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchUpdateStandardsControlAssociationsResponse {
    #[serde(rename = "UnprocessedAssociationUpdates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_association_updates: Option<Vec<UnprocessedStandardsControlAssociationUpdate>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UnprocessedStandardsControlAssociationUpdate {
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "ErrorReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_reason: Option<String>,
    #[serde(rename = "StandardsControlAssociationUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standards_control_association_update: Option<StandardsControlAssociationUpdate>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateActionTargetRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    pub description: String,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateActionTargetResponse {
    #[serde(rename = "ActionTargetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_target_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAggregatorV2Request {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "LinkedRegions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_regions: Option<Vec<String>>,
    #[serde(rename = "RegionLinkingMode")]
    #[serde(default)]
    pub region_linking_mode: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAggregatorV2Response {
    #[serde(rename = "AggregationRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_region: Option<String>,
    #[serde(rename = "AggregatorV2Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregator_v2_arn: Option<String>,
    #[serde(rename = "LinkedRegions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_regions: Option<Vec<String>>,
    #[serde(rename = "RegionLinkingMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_linking_mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAutomationRuleRequest {
    #[serde(rename = "Actions")]
    #[serde(default)]
    pub actions: Vec<AutomationRulesAction>,
    #[serde(rename = "Criteria")]
    #[serde(default)]
    pub criteria: AutomationRulesFindingFilters,
    #[serde(rename = "Description")]
    #[serde(default)]
    pub description: String,
    #[serde(rename = "IsTerminal")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_terminal: Option<bool>,
    #[serde(rename = "RuleName")]
    #[serde(default)]
    pub rule_name: String,
    #[serde(rename = "RuleOrder")]
    #[serde(default)]
    pub rule_order: i32,
    #[serde(rename = "RuleStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_status: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAutomationRuleResponse {
    #[serde(rename = "RuleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAutomationRuleV2Request {
    #[serde(rename = "Actions")]
    #[serde(default)]
    pub actions: Vec<AutomationRulesActionV2>,
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "Criteria")]
    #[serde(default)]
    pub criteria: Criteria,
    #[serde(rename = "Description")]
    #[serde(default)]
    pub description: String,
    #[serde(rename = "RuleName")]
    #[serde(default)]
    pub rule_name: String,
    #[serde(rename = "RuleOrder")]
    #[serde(default)]
    pub rule_order: f32,
    #[serde(rename = "RuleStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_status: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomationRulesActionV2 {
    #[serde(rename = "ExternalIntegrationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_integration_configuration: Option<ExternalIntegrationConfiguration>,
    #[serde(rename = "FindingFieldsUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_fields_update: Option<AutomationRulesFindingFieldsUpdateV2>,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExternalIntegrationConfiguration {
    #[serde(rename = "ConnectorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomationRulesFindingFieldsUpdateV2 {
    #[serde(rename = "Comment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "SeverityId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity_id: Option<i32>,
    #[serde(rename = "StatusId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_id: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Criteria {
    #[serde(rename = "OcsfFindingCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ocsf_finding_criteria: Option<OcsfFindingFilters>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OcsfFindingFilters {
    #[serde(rename = "CompositeFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub composite_filters: Option<Vec<CompositeFilter>>,
    #[serde(rename = "CompositeOperator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub composite_operator: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CompositeFilter {
    #[serde(rename = "BooleanFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boolean_filters: Option<Vec<OcsfBooleanFilter>>,
    #[serde(rename = "DateFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_filters: Option<Vec<OcsfDateFilter>>,
    #[serde(rename = "IpFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_filters: Option<Vec<OcsfIpFilter>>,
    #[serde(rename = "MapFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map_filters: Option<Vec<OcsfMapFilter>>,
    #[serde(rename = "NestedCompositeFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nested_composite_filters: Option<Vec<CompositeFilter>>,
    #[serde(rename = "NumberFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_filters: Option<Vec<OcsfNumberFilter>>,
    #[serde(rename = "Operator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    #[serde(rename = "StringFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_filters: Option<Vec<OcsfStringFilter>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OcsfBooleanFilter {
    #[serde(rename = "FieldName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<BooleanFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BooleanFilter {
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OcsfDateFilter {
    #[serde(rename = "FieldName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<DateFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OcsfIpFilter {
    #[serde(rename = "FieldName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<IpFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IpFilter {
    #[serde(rename = "Cidr")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OcsfMapFilter {
    #[serde(rename = "FieldName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<MapFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OcsfNumberFilter {
    #[serde(rename = "FieldName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<NumberFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OcsfStringFilter {
    #[serde(rename = "FieldName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<StringFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAutomationRuleV2Response {
    #[serde(rename = "RuleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_arn: Option<String>,
    #[serde(rename = "RuleId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateConfigurationPolicyRequest {
    #[serde(rename = "ConfigurationPolicy")]
    #[serde(default)]
    pub configuration_policy: Policy,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Policy {
    #[serde(rename = "SecurityHub")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_hub: Option<SecurityHubPolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SecurityHubPolicy {
    #[serde(rename = "EnabledStandardIdentifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_standard_identifiers: Option<Vec<String>>,
    #[serde(rename = "SecurityControlsConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_controls_configuration: Option<SecurityControlsConfiguration>,
    #[serde(rename = "ServiceEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SecurityControlsConfiguration {
    #[serde(rename = "DisabledSecurityControlIdentifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_security_control_identifiers: Option<Vec<String>>,
    #[serde(rename = "EnabledSecurityControlIdentifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_security_control_identifiers: Option<Vec<String>>,
    #[serde(rename = "SecurityControlCustomParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_control_custom_parameters: Option<Vec<SecurityControlCustomParameter>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SecurityControlCustomParameter {
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, ParameterConfiguration>>,
    #[serde(rename = "SecurityControlId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_control_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateConfigurationPolicyResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "ConfigurationPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_policy: Option<Policy>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "UpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateConnectorV2Request {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "KmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Provider")]
    #[serde(default)]
    pub provider: ProviderConfiguration,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProviderConfiguration {
    #[serde(rename = "JiraCloud")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jira_cloud: Option<JiraCloudProviderConfiguration>,
    #[serde(rename = "ServiceNow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_now: Option<ServiceNowProviderConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JiraCloudProviderConfiguration {
    #[serde(rename = "ProjectKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_key: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceNowProviderConfiguration {
    #[serde(rename = "InstanceName")]
    #[serde(default)]
    pub instance_name: String,
    #[serde(rename = "SecretArn")]
    #[serde(default)]
    pub secret_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateConnectorV2Response {
    #[serde(rename = "AuthUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_url: Option<String>,
    #[serde(rename = "ConnectorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_arn: Option<String>,
    #[serde(rename = "ConnectorId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<String>,
    #[serde(rename = "ConnectorStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFindingAggregatorRequest {
    #[serde(rename = "RegionLinkingMode")]
    #[serde(default)]
    pub region_linking_mode: String,
    #[serde(rename = "Regions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFindingAggregatorResponse {
    #[serde(rename = "FindingAggregationRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_aggregation_region: Option<String>,
    #[serde(rename = "FindingAggregatorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_aggregator_arn: Option<String>,
    #[serde(rename = "RegionLinkingMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_linking_mode: Option<String>,
    #[serde(rename = "Regions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateInsightRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    pub filters: AwsSecurityFindingFilters,
    #[serde(rename = "GroupByAttribute")]
    #[serde(default)]
    pub group_by_attribute: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsSecurityFindingFilters {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_account_id: Option<Vec<StringFilter>>,
    #[serde(rename = "AwsAccountName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_account_name: Option<Vec<StringFilter>>,
    #[serde(rename = "CompanyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name: Option<Vec<StringFilter>>,
    #[serde(rename = "ComplianceAssociatedStandardsId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_associated_standards_id: Option<Vec<StringFilter>>,
    #[serde(rename = "ComplianceSecurityControlId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_security_control_id: Option<Vec<StringFilter>>,
    #[serde(rename = "ComplianceSecurityControlParametersName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_security_control_parameters_name: Option<Vec<StringFilter>>,
    #[serde(rename = "ComplianceSecurityControlParametersValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_security_control_parameters_value: Option<Vec<StringFilter>>,
    #[serde(rename = "ComplianceStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_status: Option<Vec<StringFilter>>,
    #[serde(rename = "Confidence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<Vec<NumberFilter>>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<Vec<DateFilter>>,
    #[serde(rename = "Criticality")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub criticality: Option<Vec<NumberFilter>>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<StringFilter>>,
    #[serde(rename = "FindingProviderFieldsConfidence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_provider_fields_confidence: Option<Vec<NumberFilter>>,
    #[serde(rename = "FindingProviderFieldsCriticality")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_provider_fields_criticality: Option<Vec<NumberFilter>>,
    #[serde(rename = "FindingProviderFieldsRelatedFindingsId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_provider_fields_related_findings_id: Option<Vec<StringFilter>>,
    #[serde(rename = "FindingProviderFieldsRelatedFindingsProductArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_provider_fields_related_findings_product_arn: Option<Vec<StringFilter>>,
    #[serde(rename = "FindingProviderFieldsSeverityLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_provider_fields_severity_label: Option<Vec<StringFilter>>,
    #[serde(rename = "FindingProviderFieldsSeverityOriginal")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_provider_fields_severity_original: Option<Vec<StringFilter>>,
    #[serde(rename = "FindingProviderFieldsTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_provider_fields_types: Option<Vec<StringFilter>>,
    #[serde(rename = "FirstObservedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_observed_at: Option<Vec<DateFilter>>,
    #[serde(rename = "GeneratorId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generator_id: Option<Vec<StringFilter>>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Vec<StringFilter>>,
    #[serde(rename = "Keyword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword: Option<Vec<KeywordFilter>>,
    #[serde(rename = "LastObservedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_observed_at: Option<Vec<DateFilter>>,
    #[serde(rename = "MalwareName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub malware_name: Option<Vec<StringFilter>>,
    #[serde(rename = "MalwarePath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub malware_path: Option<Vec<StringFilter>>,
    #[serde(rename = "MalwareState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub malware_state: Option<Vec<StringFilter>>,
    #[serde(rename = "MalwareType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub malware_type: Option<Vec<StringFilter>>,
    #[serde(rename = "NetworkDestinationDomain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_destination_domain: Option<Vec<StringFilter>>,
    #[serde(rename = "NetworkDestinationIpV4")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_destination_ip_v4: Option<Vec<IpFilter>>,
    #[serde(rename = "NetworkDestinationIpV6")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_destination_ip_v6: Option<Vec<IpFilter>>,
    #[serde(rename = "NetworkDestinationPort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_destination_port: Option<Vec<NumberFilter>>,
    #[serde(rename = "NetworkDirection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_direction: Option<Vec<StringFilter>>,
    #[serde(rename = "NetworkProtocol")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_protocol: Option<Vec<StringFilter>>,
    #[serde(rename = "NetworkSourceDomain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_source_domain: Option<Vec<StringFilter>>,
    #[serde(rename = "NetworkSourceIpV4")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_source_ip_v4: Option<Vec<IpFilter>>,
    #[serde(rename = "NetworkSourceIpV6")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_source_ip_v6: Option<Vec<IpFilter>>,
    #[serde(rename = "NetworkSourceMac")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_source_mac: Option<Vec<StringFilter>>,
    #[serde(rename = "NetworkSourcePort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_source_port: Option<Vec<NumberFilter>>,
    #[serde(rename = "NoteText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note_text: Option<Vec<StringFilter>>,
    #[serde(rename = "NoteUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note_updated_at: Option<Vec<DateFilter>>,
    #[serde(rename = "NoteUpdatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note_updated_by: Option<Vec<StringFilter>>,
    #[serde(rename = "ProcessLaunchedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_launched_at: Option<Vec<DateFilter>>,
    #[serde(rename = "ProcessName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_name: Option<Vec<StringFilter>>,
    #[serde(rename = "ProcessParentPid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_parent_pid: Option<Vec<NumberFilter>>,
    #[serde(rename = "ProcessPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_path: Option<Vec<StringFilter>>,
    #[serde(rename = "ProcessPid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_pid: Option<Vec<NumberFilter>>,
    #[serde(rename = "ProcessTerminatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_terminated_at: Option<Vec<DateFilter>>,
    #[serde(rename = "ProductArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_arn: Option<Vec<StringFilter>>,
    #[serde(rename = "ProductFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_fields: Option<Vec<MapFilter>>,
    #[serde(rename = "ProductName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_name: Option<Vec<StringFilter>>,
    #[serde(rename = "RecommendationText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_text: Option<Vec<StringFilter>>,
    #[serde(rename = "RecordState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_state: Option<Vec<StringFilter>>,
    #[serde(rename = "Region")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<Vec<StringFilter>>,
    #[serde(rename = "RelatedFindingsId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_findings_id: Option<Vec<StringFilter>>,
    #[serde(rename = "RelatedFindingsProductArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_findings_product_arn: Option<Vec<StringFilter>>,
    #[serde(rename = "ResourceApplicationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_application_arn: Option<Vec<StringFilter>>,
    #[serde(rename = "ResourceApplicationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_application_name: Option<Vec<StringFilter>>,
    #[serde(rename = "ResourceAwsEc2InstanceIamInstanceProfileArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_aws_ec2_instance_iam_instance_profile_arn: Option<Vec<StringFilter>>,
    #[serde(rename = "ResourceAwsEc2InstanceImageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_aws_ec2_instance_image_id: Option<Vec<StringFilter>>,
    #[serde(rename = "ResourceAwsEc2InstanceIpV4Addresses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_aws_ec2_instance_ip_v4_addresses: Option<Vec<IpFilter>>,
    #[serde(rename = "ResourceAwsEc2InstanceIpV6Addresses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_aws_ec2_instance_ip_v6_addresses: Option<Vec<IpFilter>>,
    #[serde(rename = "ResourceAwsEc2InstanceKeyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_aws_ec2_instance_key_name: Option<Vec<StringFilter>>,
    #[serde(rename = "ResourceAwsEc2InstanceLaunchedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_aws_ec2_instance_launched_at: Option<Vec<DateFilter>>,
    #[serde(rename = "ResourceAwsEc2InstanceSubnetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_aws_ec2_instance_subnet_id: Option<Vec<StringFilter>>,
    #[serde(rename = "ResourceAwsEc2InstanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_aws_ec2_instance_type: Option<Vec<StringFilter>>,
    #[serde(rename = "ResourceAwsEc2InstanceVpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_aws_ec2_instance_vpc_id: Option<Vec<StringFilter>>,
    #[serde(rename = "ResourceAwsIamAccessKeyCreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_aws_iam_access_key_created_at: Option<Vec<DateFilter>>,
    #[serde(rename = "ResourceAwsIamAccessKeyPrincipalName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_aws_iam_access_key_principal_name: Option<Vec<StringFilter>>,
    #[serde(rename = "ResourceAwsIamAccessKeyStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_aws_iam_access_key_status: Option<Vec<StringFilter>>,
    #[serde(rename = "ResourceAwsIamAccessKeyUserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_aws_iam_access_key_user_name: Option<Vec<StringFilter>>,
    #[serde(rename = "ResourceAwsIamUserUserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_aws_iam_user_user_name: Option<Vec<StringFilter>>,
    #[serde(rename = "ResourceAwsS3BucketOwnerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_aws_s3_bucket_owner_id: Option<Vec<StringFilter>>,
    #[serde(rename = "ResourceAwsS3BucketOwnerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_aws_s3_bucket_owner_name: Option<Vec<StringFilter>>,
    #[serde(rename = "ResourceContainerImageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_container_image_id: Option<Vec<StringFilter>>,
    #[serde(rename = "ResourceContainerImageName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_container_image_name: Option<Vec<StringFilter>>,
    #[serde(rename = "ResourceContainerLaunchedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_container_launched_at: Option<Vec<DateFilter>>,
    #[serde(rename = "ResourceContainerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_container_name: Option<Vec<StringFilter>>,
    #[serde(rename = "ResourceDetailsOther")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_details_other: Option<Vec<MapFilter>>,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<Vec<StringFilter>>,
    #[serde(rename = "ResourcePartition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_partition: Option<Vec<StringFilter>>,
    #[serde(rename = "ResourceRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_region: Option<Vec<StringFilter>>,
    #[serde(rename = "ResourceTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_tags: Option<Vec<MapFilter>>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<Vec<StringFilter>>,
    #[serde(rename = "Sample")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample: Option<Vec<BooleanFilter>>,
    #[serde(rename = "SeverityLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity_label: Option<Vec<StringFilter>>,
    #[serde(rename = "SeverityNormalized")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity_normalized: Option<Vec<NumberFilter>>,
    #[serde(rename = "SeverityProduct")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity_product: Option<Vec<NumberFilter>>,
    #[serde(rename = "SourceUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<Vec<StringFilter>>,
    #[serde(rename = "ThreatIntelIndicatorCategory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_intel_indicator_category: Option<Vec<StringFilter>>,
    #[serde(rename = "ThreatIntelIndicatorLastObservedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_intel_indicator_last_observed_at: Option<Vec<DateFilter>>,
    #[serde(rename = "ThreatIntelIndicatorSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_intel_indicator_source: Option<Vec<StringFilter>>,
    #[serde(rename = "ThreatIntelIndicatorSourceUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_intel_indicator_source_url: Option<Vec<StringFilter>>,
    #[serde(rename = "ThreatIntelIndicatorType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_intel_indicator_type: Option<Vec<StringFilter>>,
    #[serde(rename = "ThreatIntelIndicatorValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_intel_indicator_value: Option<Vec<StringFilter>>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Vec<StringFilter>>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Vec<StringFilter>>,
    #[serde(rename = "UpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<Vec<DateFilter>>,
    #[serde(rename = "UserDefinedFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_defined_fields: Option<Vec<MapFilter>>,
    #[serde(rename = "VerificationState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_state: Option<Vec<StringFilter>>,
    #[serde(rename = "VulnerabilitiesExploitAvailable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vulnerabilities_exploit_available: Option<Vec<StringFilter>>,
    #[serde(rename = "VulnerabilitiesFixAvailable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vulnerabilities_fix_available: Option<Vec<StringFilter>>,
    #[serde(rename = "WorkflowState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_state: Option<Vec<StringFilter>>,
    #[serde(rename = "WorkflowStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_status: Option<Vec<StringFilter>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KeywordFilter {
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateInsightResponse {
    #[serde(rename = "InsightArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insight_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateMembersRequest {
    #[serde(rename = "AccountDetails")]
    #[serde(default)]
    pub account_details: Vec<AccountDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccountDetails {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "Email")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateMembersResponse {
    #[serde(rename = "UnprocessedAccounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_accounts: Option<Vec<Result_>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Result_ {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "ProcessingResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_result: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTicketV2Request {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "ConnectorId")]
    #[serde(default)]
    pub connector_id: String,
    #[serde(rename = "FindingMetadataUid")]
    #[serde(default)]
    pub finding_metadata_uid: String,
    #[serde(rename = "Mode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTicketV2Response {
    #[serde(rename = "TicketId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticket_id: Option<String>,
    #[serde(rename = "TicketSrcUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticket_src_url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeclineInvitationsRequest {
    #[serde(rename = "AccountIds")]
    #[serde(default)]
    pub account_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeclineInvitationsResponse {
    #[serde(rename = "UnprocessedAccounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_accounts: Option<Vec<Result_>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteActionTargetRequest {
    #[serde(rename = "ActionTargetArn")]
    #[serde(default)]
    pub action_target_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteActionTargetResponse {
    #[serde(rename = "ActionTargetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_target_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAggregatorV2Request {
    #[serde(rename = "AggregatorV2Arn")]
    #[serde(default)]
    pub aggregator_v2_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAggregatorV2Response {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAutomationRuleV2Request {
    #[serde(rename = "Identifier")]
    #[serde(default)]
    pub identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAutomationRuleV2Response {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteConfigurationPolicyRequest {
    #[serde(rename = "Identifier")]
    #[serde(default)]
    pub identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteConfigurationPolicyResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteConnectorV2Request {
    #[serde(rename = "ConnectorId")]
    #[serde(default)]
    pub connector_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteConnectorV2Response {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFindingAggregatorRequest {
    #[serde(rename = "FindingAggregatorArn")]
    #[serde(default)]
    pub finding_aggregator_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFindingAggregatorResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteInsightRequest {
    #[serde(rename = "InsightArn")]
    #[serde(default)]
    pub insight_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteInsightResponse {
    #[serde(rename = "InsightArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insight_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteInvitationsRequest {
    #[serde(rename = "AccountIds")]
    #[serde(default)]
    pub account_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteInvitationsResponse {
    #[serde(rename = "UnprocessedAccounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_accounts: Option<Vec<Result_>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteMembersRequest {
    #[serde(rename = "AccountIds")]
    #[serde(default)]
    pub account_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteMembersResponse {
    #[serde(rename = "UnprocessedAccounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_accounts: Option<Vec<Result_>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeActionTargetsRequest {
    #[serde(rename = "ActionTargetArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_target_arns: Option<Vec<String>>,
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
pub struct DescribeActionTargetsResponse {
    #[serde(rename = "ActionTargets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_targets: Option<Vec<ActionTarget>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActionTarget {
    #[serde(rename = "ActionTargetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_target_arn: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeHubRequest {
    #[serde(rename = "HubArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hub_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeHubResponse {
    #[serde(rename = "AutoEnableControls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_enable_controls: Option<bool>,
    #[serde(rename = "ControlFindingGenerator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_finding_generator: Option<String>,
    #[serde(rename = "HubArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hub_arn: Option<String>,
    #[serde(rename = "SubscribedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribed_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeOrganizationConfigurationRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeOrganizationConfigurationResponse {
    #[serde(rename = "AutoEnable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_enable: Option<bool>,
    #[serde(rename = "AutoEnableStandards")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_enable_standards: Option<String>,
    #[serde(rename = "MemberAccountLimitReached")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_account_limit_reached: Option<bool>,
    #[serde(rename = "OrganizationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_configuration: Option<OrganizationConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OrganizationConfiguration {
    #[serde(rename = "ConfigurationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_type: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeProductsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ProductArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeProductsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Products")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub products: Option<Vec<Product>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Product {
    #[serde(rename = "ActivationUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activation_url: Option<String>,
    #[serde(rename = "Categories")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<String>>,
    #[serde(rename = "CompanyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "IntegrationTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_types: Option<Vec<String>>,
    #[serde(rename = "MarketplaceUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketplace_url: Option<String>,
    #[serde(rename = "ProductArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_arn: Option<String>,
    #[serde(rename = "ProductName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
    #[serde(rename = "ProductSubscriptionResourcePolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_subscription_resource_policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeProductsV2Request {
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
pub struct DescribeProductsV2Response {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ProductsV2")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub products_v2: Option<Vec<ProductV2>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProductV2 {
    #[serde(rename = "ActivationUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activation_url: Option<String>,
    #[serde(rename = "Categories")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<String>>,
    #[serde(rename = "CompanyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "IntegrationV2Types")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_v2_types: Option<Vec<String>>,
    #[serde(rename = "MarketplaceProductId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketplace_product_id: Option<String>,
    #[serde(rename = "MarketplaceUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketplace_url: Option<String>,
    #[serde(rename = "ProductV2Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_v2_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSecurityHubV2Request {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSecurityHubV2Response {
    #[serde(rename = "HubV2Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hub_v2_arn: Option<String>,
    #[serde(rename = "SubscribedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribed_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeStandardsControlsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StandardsSubscriptionArn")]
    #[serde(default)]
    pub standards_subscription_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeStandardsControlsResponse {
    #[serde(rename = "Controls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controls: Option<Vec<StandardsControl>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StandardsControl {
    #[serde(rename = "ControlId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_id: Option<String>,
    #[serde(rename = "ControlStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_status: Option<String>,
    #[serde(rename = "ControlStatusUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_status_updated_at: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "DisabledReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_reason: Option<String>,
    #[serde(rename = "RelatedRequirements")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_requirements: Option<Vec<String>>,
    #[serde(rename = "RemediationUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remediation_url: Option<String>,
    #[serde(rename = "SeverityRating")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity_rating: Option<String>,
    #[serde(rename = "StandardsControlArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standards_control_arn: Option<String>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeStandardsRequest {
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
pub struct DescribeStandardsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Standards")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standards: Option<Vec<Standard>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Standard {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EnabledByDefault")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_by_default: Option<bool>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "StandardsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standards_arn: Option<String>,
    #[serde(rename = "StandardsManagedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standards_managed_by: Option<StandardsManagedBy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StandardsManagedBy {
    #[serde(rename = "Company")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    #[serde(rename = "Product")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisableImportFindingsForProductRequest {
    #[serde(rename = "ProductSubscriptionArn")]
    #[serde(default)]
    pub product_subscription_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisableImportFindingsForProductResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisableOrganizationAdminAccountRequest {
    #[serde(rename = "AdminAccountId")]
    #[serde(default)]
    pub admin_account_id: String,
    #[serde(rename = "Feature")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisableOrganizationAdminAccountResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisableSecurityHubRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisableSecurityHubResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisableSecurityHubV2Request {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisableSecurityHubV2Response {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateFromAdministratorAccountRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateFromAdministratorAccountResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateFromMasterAccountRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateFromMasterAccountResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateMembersRequest {
    #[serde(rename = "AccountIds")]
    #[serde(default)]
    pub account_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateMembersResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnableImportFindingsForProductRequest {
    #[serde(rename = "ProductArn")]
    #[serde(default)]
    pub product_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnableImportFindingsForProductResponse {
    #[serde(rename = "ProductSubscriptionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_subscription_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnableOrganizationAdminAccountRequest {
    #[serde(rename = "AdminAccountId")]
    #[serde(default)]
    pub admin_account_id: String,
    #[serde(rename = "Feature")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnableOrganizationAdminAccountResponse {
    #[serde(rename = "AdminAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_account_id: Option<String>,
    #[serde(rename = "Feature")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnableSecurityHubRequest {
    #[serde(rename = "ControlFindingGenerator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_finding_generator: Option<String>,
    #[serde(rename = "EnableDefaultStandards")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_default_standards: Option<bool>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnableSecurityHubResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnableSecurityHubV2Request {
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnableSecurityHubV2Response {
    #[serde(rename = "HubV2Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hub_v2_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAdministratorAccountRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAdministratorAccountResponse {
    #[serde(rename = "Administrator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub administrator: Option<Invitation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Invitation {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "InvitationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitation_id: Option<String>,
    #[serde(rename = "InvitedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invited_at: Option<String>,
    #[serde(rename = "MemberStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAggregatorV2Request {
    #[serde(rename = "AggregatorV2Arn")]
    #[serde(default)]
    pub aggregator_v2_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAggregatorV2Response {
    #[serde(rename = "AggregationRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_region: Option<String>,
    #[serde(rename = "AggregatorV2Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregator_v2_arn: Option<String>,
    #[serde(rename = "LinkedRegions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_regions: Option<Vec<String>>,
    #[serde(rename = "RegionLinkingMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_linking_mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAutomationRuleV2Request {
    #[serde(rename = "Identifier")]
    #[serde(default)]
    pub identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAutomationRuleV2Response {
    #[serde(rename = "Actions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<AutomationRulesActionV2>>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "Criteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub criteria: Option<Criteria>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "RuleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_arn: Option<String>,
    #[serde(rename = "RuleId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
    #[serde(rename = "RuleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
    #[serde(rename = "RuleOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_order: Option<f32>,
    #[serde(rename = "RuleStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_status: Option<String>,
    #[serde(rename = "UpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetConfigurationPolicyAssociationRequest {
    #[serde(rename = "Target")]
    #[serde(default)]
    pub target: Target,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetConfigurationPolicyAssociationResponse {
    #[serde(rename = "AssociationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_status: Option<String>,
    #[serde(rename = "AssociationStatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_status_message: Option<String>,
    #[serde(rename = "AssociationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_type: Option<String>,
    #[serde(rename = "ConfigurationPolicyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_policy_id: Option<String>,
    #[serde(rename = "TargetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
    #[serde(rename = "TargetType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,
    #[serde(rename = "UpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetConfigurationPolicyRequest {
    #[serde(rename = "Identifier")]
    #[serde(default)]
    pub identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetConfigurationPolicyResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "ConfigurationPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_policy: Option<Policy>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "UpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetConnectorV2Request {
    #[serde(rename = "ConnectorId")]
    #[serde(default)]
    pub connector_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetConnectorV2Response {
    #[serde(rename = "ConnectorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_arn: Option<String>,
    #[serde(rename = "ConnectorId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<String>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Health")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health: Option<HealthCheck>,
    #[serde(rename = "KmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    #[serde(rename = "LastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ProviderDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_detail: Option<ProviderDetail>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HealthCheck {
    #[serde(rename = "ConnectorStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_status: Option<String>,
    #[serde(rename = "LastCheckedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_checked_at: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProviderDetail {
    #[serde(rename = "JiraCloud")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jira_cloud: Option<JiraCloudDetail>,
    #[serde(rename = "ServiceNow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_now: Option<ServiceNowDetail>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JiraCloudDetail {
    #[serde(rename = "AuthStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_status: Option<String>,
    #[serde(rename = "AuthUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_url: Option<String>,
    #[serde(rename = "CloudId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_id: Option<String>,
    #[serde(rename = "Domain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "ProjectKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_key: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceNowDetail {
    #[serde(rename = "AuthStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_status: Option<String>,
    #[serde(rename = "InstanceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    #[serde(rename = "SecretArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetEnabledStandardsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StandardsSubscriptionArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standards_subscription_arns: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetEnabledStandardsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StandardsSubscriptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standards_subscriptions: Option<Vec<StandardsSubscription>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFindingAggregatorRequest {
    #[serde(rename = "FindingAggregatorArn")]
    #[serde(default)]
    pub finding_aggregator_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFindingAggregatorResponse {
    #[serde(rename = "FindingAggregationRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_aggregation_region: Option<String>,
    #[serde(rename = "FindingAggregatorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_aggregator_arn: Option<String>,
    #[serde(rename = "RegionLinkingMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_linking_mode: Option<String>,
    #[serde(rename = "Regions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFindingHistoryRequest {
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(rename = "FindingIdentifier")]
    #[serde(default)]
    pub finding_identifier: AwsSecurityFindingIdentifier,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFindingHistoryResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Records")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<FindingHistoryRecord>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FindingHistoryRecord {
    #[serde(rename = "FindingCreated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_created: Option<bool>,
    #[serde(rename = "FindingIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_identifier: Option<AwsSecurityFindingIdentifier>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "UpdateSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_source: Option<FindingHistoryUpdateSource>,
    #[serde(rename = "UpdateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    #[serde(rename = "Updates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updates: Option<Vec<FindingHistoryUpdate>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FindingHistoryUpdateSource {
    #[serde(rename = "Identity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FindingHistoryUpdate {
    #[serde(rename = "NewValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_value: Option<String>,
    #[serde(rename = "OldValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_value: Option<String>,
    #[serde(rename = "UpdatedField")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_field: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFindingStatisticsV2Request {
    #[serde(rename = "GroupByRules")]
    #[serde(default)]
    pub group_by_rules: Vec<GroupByRule>,
    #[serde(rename = "MaxStatisticResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_statistic_results: Option<i32>,
    #[serde(rename = "Scopes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<FindingScopes>,
    #[serde(rename = "SortOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GroupByRule {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<OcsfFindingFilters>,
    #[serde(rename = "GroupByField")]
    #[serde(default)]
    pub group_by_field: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FindingScopes {
    #[serde(rename = "AwsOrganizations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_organizations: Option<Vec<AwsOrganizationScope>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsOrganizationScope {
    #[serde(rename = "OrganizationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
    #[serde(rename = "OrganizationalUnitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizational_unit_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFindingStatisticsV2Response {
    #[serde(rename = "GroupByResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by_results: Option<Vec<GroupByResult>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GroupByResult {
    #[serde(rename = "GroupByField")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by_field: Option<String>,
    #[serde(rename = "GroupByValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by_values: Option<Vec<GroupByValue>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GroupByValue {
    #[serde(rename = "Count")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(rename = "FieldValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFindingsRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<AwsSecurityFindingFilters>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SortCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_criteria: Option<Vec<SortCriterion>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SortCriterion {
    #[serde(rename = "Field")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    #[serde(rename = "SortOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFindingsResponse {
    #[serde(rename = "Findings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub findings: Option<Vec<AwsSecurityFinding>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFindingsTrendsV2Request {
    #[serde(rename = "EndTime")]
    #[serde(default)]
    pub end_time: String,
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<FindingsTrendsFilters>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    pub start_time: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FindingsTrendsFilters {
    #[serde(rename = "CompositeFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub composite_filters: Option<Vec<FindingsTrendsCompositeFilter>>,
    #[serde(rename = "CompositeOperator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub composite_operator: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FindingsTrendsCompositeFilter {
    #[serde(rename = "NestedCompositeFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nested_composite_filters: Option<Vec<FindingsTrendsCompositeFilter>>,
    #[serde(rename = "Operator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    #[serde(rename = "StringFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_filters: Option<Vec<FindingsTrendsStringFilter>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FindingsTrendsStringFilter {
    #[serde(rename = "FieldName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<StringFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFindingsTrendsV2Response {
    #[serde(rename = "Granularity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granularity: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TrendsMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trends_metrics: Option<Vec<TrendsMetricsResult>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TrendsMetricsResult {
    #[serde(rename = "Timestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[serde(rename = "TrendsValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trends_values: Option<TrendsValues>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TrendsValues {
    #[serde(rename = "SeverityTrends")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity_trends: Option<SeverityTrendsCount>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SeverityTrendsCount {
    #[serde(rename = "Critical")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub critical: Option<i64>,
    #[serde(rename = "Fatal")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fatal: Option<i64>,
    #[serde(rename = "High")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high: Option<i64>,
    #[serde(rename = "Informational")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub informational: Option<i64>,
    #[serde(rename = "Low")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub low: Option<i64>,
    #[serde(rename = "Medium")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medium: Option<i64>,
    #[serde(rename = "Other")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other: Option<i64>,
    #[serde(rename = "Unknown")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFindingsV2Request {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<OcsfFindingFilters>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Scopes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<FindingScopes>,
    #[serde(rename = "SortCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_criteria: Option<Vec<SortCriterion>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFindingsV2Response {
    #[serde(rename = "Findings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub findings: Option<Vec<serde_json::Value>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetInsightResultsRequest {
    #[serde(rename = "InsightArn")]
    #[serde(default)]
    pub insight_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetInsightResultsResponse {
    #[serde(rename = "InsightResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insight_results: Option<InsightResults>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InsightResults {
    #[serde(rename = "GroupByAttribute")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by_attribute: Option<String>,
    #[serde(rename = "InsightArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insight_arn: Option<String>,
    #[serde(rename = "ResultValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_values: Option<Vec<InsightResultValue>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InsightResultValue {
    #[serde(rename = "Count")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(rename = "GroupByAttributeValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by_attribute_value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetInsightsRequest {
    #[serde(rename = "InsightArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insight_arns: Option<Vec<String>>,
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
pub struct GetInsightsResponse {
    #[serde(rename = "Insights")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insights: Option<Vec<Insight>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Insight {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<AwsSecurityFindingFilters>,
    #[serde(rename = "GroupByAttribute")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by_attribute: Option<String>,
    #[serde(rename = "InsightArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insight_arn: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetInvitationsCountRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetInvitationsCountResponse {
    #[serde(rename = "InvitationsCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitations_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMasterAccountRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMasterAccountResponse {
    #[serde(rename = "Master")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master: Option<Invitation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMembersRequest {
    #[serde(rename = "AccountIds")]
    #[serde(default)]
    pub account_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMembersResponse {
    #[serde(rename = "Members")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<Member>>,
    #[serde(rename = "UnprocessedAccounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_accounts: Option<Vec<Result_>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Member {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "AdministratorId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub administrator_id: Option<String>,
    #[serde(rename = "Email")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "InvitedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invited_at: Option<String>,
    #[serde(rename = "MasterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_id: Option<String>,
    #[serde(rename = "MemberStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_status: Option<String>,
    #[serde(rename = "UpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourcesStatisticsV2Request {
    #[serde(rename = "GroupByRules")]
    #[serde(default)]
    pub group_by_rules: Vec<ResourceGroupByRule>,
    #[serde(rename = "MaxStatisticResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_statistic_results: Option<i32>,
    #[serde(rename = "Scopes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<ResourceScopes>,
    #[serde(rename = "SortOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceGroupByRule {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<ResourcesFilters>,
    #[serde(rename = "GroupByField")]
    #[serde(default)]
    pub group_by_field: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourcesFilters {
    #[serde(rename = "CompositeFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub composite_filters: Option<Vec<ResourcesCompositeFilter>>,
    #[serde(rename = "CompositeOperator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub composite_operator: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourcesCompositeFilter {
    #[serde(rename = "DateFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_filters: Option<Vec<ResourcesDateFilter>>,
    #[serde(rename = "MapFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map_filters: Option<Vec<ResourcesMapFilter>>,
    #[serde(rename = "NestedCompositeFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nested_composite_filters: Option<Vec<ResourcesCompositeFilter>>,
    #[serde(rename = "NumberFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_filters: Option<Vec<ResourcesNumberFilter>>,
    #[serde(rename = "Operator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    #[serde(rename = "StringFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_filters: Option<Vec<ResourcesStringFilter>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourcesDateFilter {
    #[serde(rename = "FieldName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<DateFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourcesMapFilter {
    #[serde(rename = "FieldName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<MapFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourcesNumberFilter {
    #[serde(rename = "FieldName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<NumberFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourcesStringFilter {
    #[serde(rename = "FieldName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<StringFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceScopes {
    #[serde(rename = "AwsOrganizations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_organizations: Option<Vec<AwsOrganizationScope>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourcesStatisticsV2Response {
    #[serde(rename = "GroupByResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by_results: Option<Vec<GroupByResult>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourcesTrendsV2Request {
    #[serde(rename = "EndTime")]
    #[serde(default)]
    pub end_time: String,
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<ResourcesTrendsFilters>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    pub start_time: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourcesTrendsFilters {
    #[serde(rename = "CompositeFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub composite_filters: Option<Vec<ResourcesTrendsCompositeFilter>>,
    #[serde(rename = "CompositeOperator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub composite_operator: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourcesTrendsCompositeFilter {
    #[serde(rename = "NestedCompositeFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nested_composite_filters: Option<Vec<ResourcesTrendsCompositeFilter>>,
    #[serde(rename = "Operator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    #[serde(rename = "StringFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_filters: Option<Vec<ResourcesTrendsStringFilter>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourcesTrendsStringFilter {
    #[serde(rename = "FieldName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<StringFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourcesTrendsV2Response {
    #[serde(rename = "Granularity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granularity: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TrendsMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trends_metrics: Option<Vec<ResourcesTrendsMetricsResult>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourcesTrendsMetricsResult {
    #[serde(rename = "Timestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[serde(rename = "TrendsValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trends_values: Option<ResourcesTrendsValues>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourcesTrendsValues {
    #[serde(rename = "ResourcesCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources_count: Option<ResourcesCount>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourcesCount {
    #[serde(rename = "AllResources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_resources: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourcesV2Request {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<ResourcesFilters>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Scopes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<ResourceScopes>,
    #[serde(rename = "SortCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_criteria: Option<Vec<SortCriterion>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourcesV2Response {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Resources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<ResourceResult>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceResult {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "FindingsSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub findings_summary: Option<Vec<ResourceFindingsSummary>>,
    #[serde(rename = "Region")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "ResourceCategory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_category: Option<String>,
    #[serde(rename = "ResourceConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_config: Option<serde_json::Value>,
    #[serde(rename = "ResourceCreationTimeDt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_creation_time_dt: Option<String>,
    #[serde(rename = "ResourceDetailCaptureTimeDt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_detail_capture_time_dt: Option<String>,
    #[serde(rename = "ResourceGuid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_guid: Option<String>,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "ResourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    #[serde(rename = "ResourceTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_tags: Option<Vec<ResourceTag>>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceFindingsSummary {
    #[serde(rename = "FindingType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_type: Option<String>,
    #[serde(rename = "ProductName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
    #[serde(rename = "Severities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severities: Option<ResourceSeverityBreakdown>,
    #[serde(rename = "TotalFindings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_findings: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceSeverityBreakdown {
    #[serde(rename = "Critical")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub critical: Option<i32>,
    #[serde(rename = "Fatal")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fatal: Option<i32>,
    #[serde(rename = "High")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high: Option<i32>,
    #[serde(rename = "Informational")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub informational: Option<i32>,
    #[serde(rename = "Low")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub low: Option<i32>,
    #[serde(rename = "Medium")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medium: Option<i32>,
    #[serde(rename = "Other")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other: Option<i32>,
    #[serde(rename = "Unknown")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceTag {
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
pub struct GetSecurityControlDefinitionRequest {
    #[serde(rename = "SecurityControlId")]
    #[serde(default)]
    pub security_control_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSecurityControlDefinitionResponse {
    #[serde(rename = "SecurityControlDefinition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_control_definition: Option<SecurityControlDefinition>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SecurityControlDefinition {
    #[serde(rename = "CurrentRegionAvailability")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_region_availability: Option<String>,
    #[serde(rename = "CustomizableProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customizable_properties: Option<Vec<String>>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ParameterDefinitions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_definitions: Option<std::collections::HashMap<String, ParameterDefinition>>,
    #[serde(rename = "RemediationUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remediation_url: Option<String>,
    #[serde(rename = "SecurityControlId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_control_id: Option<String>,
    #[serde(rename = "SeverityRating")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity_rating: Option<String>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ParameterDefinition {
    #[serde(rename = "ConfigurationOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_options: Option<ConfigurationOptions>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConfigurationOptions {
    #[serde(rename = "Boolean")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boolean: Option<BooleanConfigurationOptions>,
    #[serde(rename = "Double")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub double: Option<DoubleConfigurationOptions>,
    #[serde(rename = "Enum")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#enum: Option<EnumConfigurationOptions>,
    #[serde(rename = "EnumList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enum_list: Option<EnumListConfigurationOptions>,
    #[serde(rename = "Integer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integer: Option<IntegerConfigurationOptions>,
    #[serde(rename = "IntegerList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integer_list: Option<IntegerListConfigurationOptions>,
    #[serde(rename = "String")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string: Option<StringConfigurationOptions>,
    #[serde(rename = "StringList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_list: Option<StringListConfigurationOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BooleanConfigurationOptions {
    #[serde(rename = "DefaultValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DoubleConfigurationOptions {
    #[serde(rename = "DefaultValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<f64>,
    #[serde(rename = "Max")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    #[serde(rename = "Min")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnumConfigurationOptions {
    #[serde(rename = "AllowedValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_values: Option<Vec<String>>,
    #[serde(rename = "DefaultValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnumListConfigurationOptions {
    #[serde(rename = "AllowedValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_values: Option<Vec<String>>,
    #[serde(rename = "DefaultValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<Vec<String>>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IntegerConfigurationOptions {
    #[serde(rename = "DefaultValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<i32>,
    #[serde(rename = "Max")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<i32>,
    #[serde(rename = "Min")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IntegerListConfigurationOptions {
    #[serde(rename = "DefaultValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<Vec<i32>>,
    #[serde(rename = "Max")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<i32>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "Min")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StringConfigurationOptions {
    #[serde(rename = "DefaultValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[serde(rename = "ExpressionDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_description: Option<String>,
    #[serde(rename = "Re2Expression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub re2_expression: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StringListConfigurationOptions {
    #[serde(rename = "DefaultValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<Vec<String>>,
    #[serde(rename = "ExpressionDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_description: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "Re2Expression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub re2_expression: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InviteMembersRequest {
    #[serde(rename = "AccountIds")]
    #[serde(default)]
    pub account_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InviteMembersResponse {
    #[serde(rename = "UnprocessedAccounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_accounts: Option<Vec<Result_>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAggregatorsV2Request {
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
pub struct ListAggregatorsV2Response {
    #[serde(rename = "AggregatorsV2")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregators_v2: Option<Vec<AggregatorV2>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AggregatorV2 {
    #[serde(rename = "AggregatorV2Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregator_v2_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAutomationRulesRequest {
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
pub struct ListAutomationRulesResponse {
    #[serde(rename = "AutomationRulesMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automation_rules_metadata: Option<Vec<AutomationRulesMetadata>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomationRulesMetadata {
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "CreatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "IsTerminal")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_terminal: Option<bool>,
    #[serde(rename = "RuleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_arn: Option<String>,
    #[serde(rename = "RuleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
    #[serde(rename = "RuleOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_order: Option<i32>,
    #[serde(rename = "RuleStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_status: Option<String>,
    #[serde(rename = "UpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAutomationRulesV2Request {
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
pub struct ListAutomationRulesV2Response {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Rules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<AutomationRulesMetadataV2>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomationRulesMetadataV2 {
    #[serde(rename = "Actions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<AutomationRulesActionTypeObjectV2>>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "RuleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_arn: Option<String>,
    #[serde(rename = "RuleId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
    #[serde(rename = "RuleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
    #[serde(rename = "RuleOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_order: Option<f32>,
    #[serde(rename = "RuleStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_status: Option<String>,
    #[serde(rename = "UpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomationRulesActionTypeObjectV2 {
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListConfigurationPoliciesRequest {
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
pub struct ListConfigurationPoliciesResponse {
    #[serde(rename = "ConfigurationPolicySummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_policy_summaries: Option<Vec<ConfigurationPolicySummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConfigurationPolicySummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ServiceEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_enabled: Option<bool>,
    #[serde(rename = "UpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListConfigurationPolicyAssociationsRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<AssociationFilters>,
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
pub struct AssociationFilters {
    #[serde(rename = "AssociationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_status: Option<String>,
    #[serde(rename = "AssociationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_type: Option<String>,
    #[serde(rename = "ConfigurationPolicyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_policy_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListConfigurationPolicyAssociationsResponse {
    #[serde(rename = "ConfigurationPolicyAssociationSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_policy_association_summaries:
        Option<Vec<ConfigurationPolicyAssociationSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListConnectorsV2Request {
    #[serde(rename = "ConnectorStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_status: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ProviderName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListConnectorsV2Response {
    #[serde(rename = "Connectors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connectors: Option<Vec<ConnectorSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectorSummary {
    #[serde(rename = "ConnectorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_arn: Option<String>,
    #[serde(rename = "ConnectorId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<String>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ProviderSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_summary: Option<ProviderSummary>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProviderSummary {
    #[serde(rename = "ConnectorStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_status: Option<String>,
    #[serde(rename = "ProviderName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListEnabledProductsForImportRequest {
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
pub struct ListEnabledProductsForImportResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ProductSubscriptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_subscriptions: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFindingAggregatorsRequest {
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
pub struct ListFindingAggregatorsResponse {
    #[serde(rename = "FindingAggregators")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_aggregators: Option<Vec<FindingAggregator>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FindingAggregator {
    #[serde(rename = "FindingAggregatorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_aggregator_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListInvitationsRequest {
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
pub struct ListInvitationsResponse {
    #[serde(rename = "Invitations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitations: Option<Vec<Invitation>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListMembersRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "OnlyAssociated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_associated: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListMembersResponse {
    #[serde(rename = "Members")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<Member>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListOrganizationAdminAccountsRequest {
    #[serde(rename = "Feature")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature: Option<String>,
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
pub struct ListOrganizationAdminAccountsResponse {
    #[serde(rename = "AdminAccounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_accounts: Option<Vec<AdminAccount>>,
    #[serde(rename = "Feature")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdminAccount {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSecurityControlDefinitionsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StandardsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standards_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSecurityControlDefinitionsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SecurityControlDefinitions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_control_definitions: Option<Vec<SecurityControlDefinition>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListStandardsControlAssociationsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SecurityControlId")]
    #[serde(default)]
    pub security_control_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListStandardsControlAssociationsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StandardsControlAssociationSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standards_control_association_summaries: Option<Vec<StandardsControlAssociationSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StandardsControlAssociationSummary {
    #[serde(rename = "AssociationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_status: Option<String>,
    #[serde(rename = "RelatedRequirements")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_requirements: Option<Vec<String>>,
    #[serde(rename = "SecurityControlArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_control_arn: Option<String>,
    #[serde(rename = "SecurityControlId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_control_id: Option<String>,
    #[serde(rename = "StandardsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standards_arn: Option<String>,
    #[serde(rename = "StandardsControlDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standards_control_description: Option<String>,
    #[serde(rename = "StandardsControlTitle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standards_control_title: Option<String>,
    #[serde(rename = "UpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "UpdatedReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterConnectorV2Request {
    #[serde(rename = "AuthCode")]
    #[serde(default)]
    pub auth_code: String,
    #[serde(rename = "AuthState")]
    #[serde(default)]
    pub auth_state: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterConnectorV2Response {
    #[serde(rename = "ConnectorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_arn: Option<String>,
    #[serde(rename = "ConnectorId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartConfigurationPolicyAssociationRequest {
    #[serde(rename = "ConfigurationPolicyIdentifier")]
    #[serde(default)]
    pub configuration_policy_identifier: String,
    #[serde(rename = "Target")]
    #[serde(default)]
    pub target: Target,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartConfigurationPolicyAssociationResponse {
    #[serde(rename = "AssociationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_status: Option<String>,
    #[serde(rename = "AssociationStatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_status_message: Option<String>,
    #[serde(rename = "AssociationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_type: Option<String>,
    #[serde(rename = "ConfigurationPolicyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_policy_id: Option<String>,
    #[serde(rename = "TargetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
    #[serde(rename = "TargetType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,
    #[serde(rename = "UpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartConfigurationPolicyDisassociationRequest {
    #[serde(rename = "ConfigurationPolicyIdentifier")]
    #[serde(default)]
    pub configuration_policy_identifier: String,
    #[serde(rename = "Target")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<Target>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartConfigurationPolicyDisassociationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateActionTargetRequest {
    #[serde(rename = "ActionTargetArn")]
    #[serde(default)]
    pub action_target_arn: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateActionTargetResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAggregatorV2Request {
    #[serde(rename = "AggregatorV2Arn")]
    #[serde(default)]
    pub aggregator_v2_arn: String,
    #[serde(rename = "LinkedRegions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_regions: Option<Vec<String>>,
    #[serde(rename = "RegionLinkingMode")]
    #[serde(default)]
    pub region_linking_mode: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAggregatorV2Response {
    #[serde(rename = "AggregationRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_region: Option<String>,
    #[serde(rename = "AggregatorV2Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregator_v2_arn: Option<String>,
    #[serde(rename = "LinkedRegions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_regions: Option<Vec<String>>,
    #[serde(rename = "RegionLinkingMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_linking_mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAutomationRuleV2Request {
    #[serde(rename = "Actions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<AutomationRulesActionV2>>,
    #[serde(rename = "Criteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub criteria: Option<Criteria>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Identifier")]
    #[serde(default)]
    pub identifier: String,
    #[serde(rename = "RuleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
    #[serde(rename = "RuleOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_order: Option<f32>,
    #[serde(rename = "RuleStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAutomationRuleV2Response {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateConfigurationPolicyRequest {
    #[serde(rename = "ConfigurationPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_policy: Option<Policy>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Identifier")]
    #[serde(default)]
    pub identifier: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "UpdatedReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateConfigurationPolicyResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "ConfigurationPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_policy: Option<Policy>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "UpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateConnectorV2Request {
    #[serde(rename = "ConnectorId")]
    #[serde(default)]
    pub connector_id: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Provider")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<ProviderUpdateConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProviderUpdateConfiguration {
    #[serde(rename = "JiraCloud")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jira_cloud: Option<JiraCloudUpdateConfiguration>,
    #[serde(rename = "ServiceNow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_now: Option<ServiceNowUpdateConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JiraCloudUpdateConfiguration {
    #[serde(rename = "ProjectKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_key: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceNowUpdateConfiguration {
    #[serde(rename = "SecretArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateConnectorV2Response {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFindingAggregatorRequest {
    #[serde(rename = "FindingAggregatorArn")]
    #[serde(default)]
    pub finding_aggregator_arn: String,
    #[serde(rename = "RegionLinkingMode")]
    #[serde(default)]
    pub region_linking_mode: String,
    #[serde(rename = "Regions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFindingAggregatorResponse {
    #[serde(rename = "FindingAggregationRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_aggregation_region: Option<String>,
    #[serde(rename = "FindingAggregatorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_aggregator_arn: Option<String>,
    #[serde(rename = "RegionLinkingMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_linking_mode: Option<String>,
    #[serde(rename = "Regions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFindingsRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    pub filters: AwsSecurityFindingFilters,
    #[serde(rename = "Note")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<NoteUpdate>,
    #[serde(rename = "RecordState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFindingsResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateInsightRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<AwsSecurityFindingFilters>,
    #[serde(rename = "GroupByAttribute")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by_attribute: Option<String>,
    #[serde(rename = "InsightArn")]
    #[serde(default)]
    pub insight_arn: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateInsightResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateOrganizationConfigurationRequest {
    #[serde(rename = "AutoEnable")]
    #[serde(default)]
    pub auto_enable: bool,
    #[serde(rename = "AutoEnableStandards")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_enable_standards: Option<String>,
    #[serde(rename = "OrganizationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_configuration: Option<OrganizationConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateOrganizationConfigurationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSecurityControlRequest {
    #[serde(rename = "LastUpdateReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_reason: Option<String>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    pub parameters: std::collections::HashMap<String, ParameterConfiguration>,
    #[serde(rename = "SecurityControlId")]
    #[serde(default)]
    pub security_control_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSecurityControlResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSecurityHubConfigurationRequest {
    #[serde(rename = "AutoEnableControls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_enable_controls: Option<bool>,
    #[serde(rename = "ControlFindingGenerator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_finding_generator: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSecurityHubConfigurationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateStandardsControlRequest {
    #[serde(rename = "ControlStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_status: Option<String>,
    #[serde(rename = "DisabledReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_reason: Option<String>,
    #[serde(rename = "StandardsControlArn")]
    #[serde(default)]
    pub standards_control_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateStandardsControlResponse {}
