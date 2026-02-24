//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-servicequotas

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateServiceQuotaTemplateRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateServiceQuotaTemplateResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSupportCaseRequest {
    #[serde(rename = "RequestId")]
    #[serde(default)]
    pub request_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSupportCaseResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteServiceQuotaIncreaseRequestFromTemplateRequest {
    #[serde(rename = "AwsRegion")]
    #[serde(default)]
    pub aws_region: String,
    #[serde(rename = "QuotaCode")]
    #[serde(default)]
    pub quota_code: String,
    #[serde(rename = "ServiceCode")]
    #[serde(default)]
    pub service_code: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteServiceQuotaIncreaseRequestFromTemplateResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateServiceQuotaTemplateRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateServiceQuotaTemplateResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAWSDefaultServiceQuotaRequest {
    #[serde(rename = "QuotaCode")]
    #[serde(default)]
    pub quota_code: String,
    #[serde(rename = "ServiceCode")]
    #[serde(default)]
    pub service_code: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAWSDefaultServiceQuotaResponse {
    #[serde(rename = "Quota")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota: Option<ServiceQuota>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceQuota {
    #[serde(rename = "Adjustable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjustable: Option<bool>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ErrorReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_reason: Option<ErrorReason>,
    #[serde(rename = "GlobalQuota")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_quota: Option<bool>,
    #[serde(rename = "Period")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<QuotaPeriod>,
    #[serde(rename = "QuotaAppliedAtLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota_applied_at_level: Option<String>,
    #[serde(rename = "QuotaArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota_arn: Option<String>,
    #[serde(rename = "QuotaCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota_code: Option<String>,
    #[serde(rename = "QuotaContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota_context: Option<QuotaContextInfo>,
    #[serde(rename = "QuotaName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota_name: Option<String>,
    #[serde(rename = "ServiceCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_code: Option<String>,
    #[serde(rename = "ServiceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    #[serde(rename = "Unit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(rename = "UsageMetric")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_metric: Option<MetricInfo>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ErrorReason {
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QuotaPeriod {
    #[serde(rename = "PeriodUnit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_unit: Option<String>,
    #[serde(rename = "PeriodValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_value: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QuotaContextInfo {
    #[serde(rename = "ContextId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_id: Option<String>,
    #[serde(rename = "ContextScope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_scope: Option<String>,
    #[serde(rename = "ContextScopeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_scope_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MetricInfo {
    #[serde(rename = "MetricDimensions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_dimensions: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "MetricName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    #[serde(rename = "MetricNamespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_namespace: Option<String>,
    #[serde(rename = "MetricStatisticRecommendation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_statistic_recommendation: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAssociationForServiceQuotaTemplateRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAssociationForServiceQuotaTemplateResponse {
    #[serde(rename = "ServiceQuotaTemplateAssociationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_quota_template_association_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAutoManagementConfigurationRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAutoManagementConfigurationResponse {
    #[serde(rename = "ExclusionList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusion_list: Option<std::collections::HashMap<String, Vec<QuotaInfo>>>,
    #[serde(rename = "NotificationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_arn: Option<String>,
    #[serde(rename = "OptInLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opt_in_level: Option<String>,
    #[serde(rename = "OptInStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opt_in_status: Option<String>,
    #[serde(rename = "OptInType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opt_in_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QuotaInfo {
    #[serde(rename = "QuotaCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota_code: Option<String>,
    #[serde(rename = "QuotaName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetQuotaUtilizationReportRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ReportId")]
    #[serde(default)]
    pub report_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetQuotaUtilizationReportResponse {
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "GeneratedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_at: Option<f64>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Quotas")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quotas: Option<Vec<QuotaUtilizationInfo>>,
    #[serde(rename = "ReportId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TotalCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QuotaUtilizationInfo {
    #[serde(rename = "Adjustable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjustable: Option<bool>,
    #[serde(rename = "AppliedValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applied_value: Option<f64>,
    #[serde(rename = "DefaultValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<f64>,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "QuotaCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota_code: Option<String>,
    #[serde(rename = "QuotaName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota_name: Option<String>,
    #[serde(rename = "ServiceCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_code: Option<String>,
    #[serde(rename = "ServiceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    #[serde(rename = "Utilization")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utilization: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRequestedServiceQuotaChangeRequest {
    #[serde(rename = "RequestId")]
    #[serde(default)]
    pub request_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRequestedServiceQuotaChangeResponse {
    #[serde(rename = "RequestedQuota")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_quota: Option<RequestedServiceQuotaChange>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RequestedServiceQuotaChange {
    #[serde(rename = "CaseId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub case_id: Option<String>,
    #[serde(rename = "Created")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<f64>,
    #[serde(rename = "DesiredValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_value: Option<f64>,
    #[serde(rename = "GlobalQuota")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_quota: Option<bool>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LastUpdated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<f64>,
    #[serde(rename = "QuotaArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota_arn: Option<String>,
    #[serde(rename = "QuotaCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota_code: Option<String>,
    #[serde(rename = "QuotaContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota_context: Option<QuotaContextInfo>,
    #[serde(rename = "QuotaName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota_name: Option<String>,
    #[serde(rename = "QuotaRequestedAtLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota_requested_at_level: Option<String>,
    #[serde(rename = "RequestType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_type: Option<String>,
    #[serde(rename = "Requester")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requester: Option<String>,
    #[serde(rename = "ServiceCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_code: Option<String>,
    #[serde(rename = "ServiceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Unit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetServiceQuotaIncreaseRequestFromTemplateRequest {
    #[serde(rename = "AwsRegion")]
    #[serde(default)]
    pub aws_region: String,
    #[serde(rename = "QuotaCode")]
    #[serde(default)]
    pub quota_code: String,
    #[serde(rename = "ServiceCode")]
    #[serde(default)]
    pub service_code: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetServiceQuotaIncreaseRequestFromTemplateResponse {
    #[serde(rename = "ServiceQuotaIncreaseRequestInTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_quota_increase_request_in_template: Option<ServiceQuotaIncreaseRequestInTemplate>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceQuotaIncreaseRequestInTemplate {
    #[serde(rename = "AwsRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_region: Option<String>,
    #[serde(rename = "DesiredValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_value: Option<f64>,
    #[serde(rename = "GlobalQuota")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_quota: Option<bool>,
    #[serde(rename = "QuotaCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota_code: Option<String>,
    #[serde(rename = "QuotaName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota_name: Option<String>,
    #[serde(rename = "ServiceCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_code: Option<String>,
    #[serde(rename = "ServiceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    #[serde(rename = "Unit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetServiceQuotaRequest {
    #[serde(rename = "ContextId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_id: Option<String>,
    #[serde(rename = "QuotaCode")]
    #[serde(default)]
    pub quota_code: String,
    #[serde(rename = "ServiceCode")]
    #[serde(default)]
    pub service_code: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetServiceQuotaResponse {
    #[serde(rename = "Quota")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota: Option<ServiceQuota>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAWSDefaultServiceQuotasRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ServiceCode")]
    #[serde(default)]
    pub service_code: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAWSDefaultServiceQuotasResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Quotas")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quotas: Option<Vec<ServiceQuota>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRequestedServiceQuotaChangeHistoryByQuotaRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "QuotaCode")]
    #[serde(default)]
    pub quota_code: String,
    #[serde(rename = "QuotaRequestedAtLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota_requested_at_level: Option<String>,
    #[serde(rename = "ServiceCode")]
    #[serde(default)]
    pub service_code: String,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRequestedServiceQuotaChangeHistoryByQuotaResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RequestedQuotas")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_quotas: Option<Vec<RequestedServiceQuotaChange>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRequestedServiceQuotaChangeHistoryRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "QuotaRequestedAtLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota_requested_at_level: Option<String>,
    #[serde(rename = "ServiceCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_code: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRequestedServiceQuotaChangeHistoryResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RequestedQuotas")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_quotas: Option<Vec<RequestedServiceQuotaChange>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListServiceQuotaIncreaseRequestsInTemplateRequest {
    #[serde(rename = "AwsRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_region: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ServiceCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_code: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListServiceQuotaIncreaseRequestsInTemplateResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ServiceQuotaIncreaseRequestInTemplateList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_quota_increase_request_in_template_list:
        Option<Vec<ServiceQuotaIncreaseRequestInTemplate>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListServiceQuotasRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "QuotaAppliedAtLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota_applied_at_level: Option<String>,
    #[serde(rename = "QuotaCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota_code: Option<String>,
    #[serde(rename = "ServiceCode")]
    #[serde(default)]
    pub service_code: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListServiceQuotasResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Quotas")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quotas: Option<Vec<ServiceQuota>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListServicesRequest {
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
pub struct ListServicesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Services")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<ServiceInfo>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceInfo {
    #[serde(rename = "ServiceCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_code: Option<String>,
    #[serde(rename = "ServiceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
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
pub struct PutServiceQuotaIncreaseRequestIntoTemplateRequest {
    #[serde(rename = "AwsRegion")]
    #[serde(default)]
    pub aws_region: String,
    #[serde(rename = "DesiredValue")]
    #[serde(default)]
    pub desired_value: f64,
    #[serde(rename = "QuotaCode")]
    #[serde(default)]
    pub quota_code: String,
    #[serde(rename = "ServiceCode")]
    #[serde(default)]
    pub service_code: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutServiceQuotaIncreaseRequestIntoTemplateResponse {
    #[serde(rename = "ServiceQuotaIncreaseRequestInTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_quota_increase_request_in_template: Option<ServiceQuotaIncreaseRequestInTemplate>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RequestServiceQuotaIncreaseRequest {
    #[serde(rename = "ContextId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_id: Option<String>,
    #[serde(rename = "DesiredValue")]
    #[serde(default)]
    pub desired_value: f64,
    #[serde(rename = "QuotaCode")]
    #[serde(default)]
    pub quota_code: String,
    #[serde(rename = "ServiceCode")]
    #[serde(default)]
    pub service_code: String,
    #[serde(rename = "SupportCaseAllowed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_case_allowed: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RequestServiceQuotaIncreaseResponse {
    #[serde(rename = "RequestedQuota")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_quota: Option<RequestedServiceQuotaChange>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartAutoManagementRequest {
    #[serde(rename = "ExclusionList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusion_list: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "NotificationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_arn: Option<String>,
    #[serde(rename = "OptInLevel")]
    #[serde(default)]
    pub opt_in_level: String,
    #[serde(rename = "OptInType")]
    #[serde(default)]
    pub opt_in_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartAutoManagementResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartQuotaUtilizationReportRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartQuotaUtilizationReportResponse {
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "ReportId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopAutoManagementRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopAutoManagementResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAutoManagementRequest {
    #[serde(rename = "ExclusionList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusion_list: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "NotificationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_arn: Option<String>,
    #[serde(rename = "OptInType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opt_in_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAutoManagementResponse {}
