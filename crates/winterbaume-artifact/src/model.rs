//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-artifact

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAccountSettingsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAccountSettingsResponse {
    #[serde(rename = "accountSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_settings: Option<AccountSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccountSettings {
    #[serde(rename = "notificationSubscriptionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_subscription_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetReportMetadataRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetReportMetadataResponse {
    #[serde(rename = "reportDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_details: Option<ReportDetail>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReportDetail {
    #[serde(rename = "acceptanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acceptance_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "companyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "deletedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "lastModifiedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "periodEnd")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_end: Option<String>,
    #[serde(rename = "periodStart")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_start: Option<String>,
    #[serde(rename = "productName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
    #[serde(rename = "sequenceNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_number: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "statusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(rename = "termArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub term_arn: Option<String>,
    #[serde(rename = "uploadState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_state: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetReportRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetReportResponse {
    #[serde(rename = "documentPresignedUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_presigned_url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTermForReportRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTermForReportResponse {
    #[serde(rename = "documentPresignedUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_presigned_url: Option<String>,
    #[serde(rename = "termToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub term_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCustomerAgreementsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCustomerAgreementsResponse {
    #[serde(rename = "customerAgreements")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_agreements: Option<Vec<CustomerAgreementSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomerAgreementSummary {
    #[serde(rename = "acceptanceTerms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acceptance_terms: Option<Vec<String>>,
    #[serde(rename = "agreementArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agreement_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "awsAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_account_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "effectiveEnd")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_end: Option<String>,
    #[serde(rename = "effectiveStart")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_start: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "organizationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "terminateTerms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminate_terms: Option<Vec<String>>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListReportVersionsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListReportVersionsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reports: Option<Vec<ReportSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReportSummary {
    #[serde(rename = "acceptanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acceptance_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "companyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "periodEnd")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_end: Option<String>,
    #[serde(rename = "periodStart")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_start: Option<String>,
    #[serde(rename = "productName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "statusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(rename = "uploadState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_state: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListReportsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListReportsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reports: Option<Vec<ReportSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutAccountSettingsRequest {
    #[serde(rename = "notificationSubscriptionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_subscription_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutAccountSettingsResponse {
    #[serde(rename = "accountSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_settings: Option<AccountSettings>,
}
