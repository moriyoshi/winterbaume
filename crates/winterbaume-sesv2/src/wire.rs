//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-sesv2

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

#[allow(unused_imports)]
use http::header::HeaderName;
use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for restJson protocol.
pub fn serialize_batch_get_metric_data_response(
    result: &BatchGetMetricDataResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_cancel_export_job_response(result: &CancelExportJobResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_configuration_set_response(
    result: &CreateConfigurationSetResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_configuration_set_event_destination_response(
    result: &CreateConfigurationSetEventDestinationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_contact_response(result: &CreateContactResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_contact_list_response(result: &CreateContactListResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_custom_verification_email_template_response(
    result: &CreateCustomVerificationEmailTemplateResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_dedicated_ip_pool_response(
    result: &CreateDedicatedIpPoolResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_deliverability_test_report_response(
    result: &CreateDeliverabilityTestReportResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_email_identity_response(
    result: &CreateEmailIdentityResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_email_identity_policy_response(
    result: &CreateEmailIdentityPolicyResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_email_template_response(
    result: &CreateEmailTemplateResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_export_job_response(result: &CreateExportJobResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_import_job_response(result: &CreateImportJobResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_multi_region_endpoint_response(
    result: &CreateMultiRegionEndpointResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_tenant_response(result: &CreateTenantResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_tenant_resource_association_response(
    result: &CreateTenantResourceAssociationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_configuration_set_response(
    result: &DeleteConfigurationSetResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_configuration_set_event_destination_response(
    result: &DeleteConfigurationSetEventDestinationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_contact_response(result: &DeleteContactResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_contact_list_response(result: &DeleteContactListResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_custom_verification_email_template_response(
    result: &DeleteCustomVerificationEmailTemplateResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_dedicated_ip_pool_response(
    result: &DeleteDedicatedIpPoolResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_email_identity_response(
    result: &DeleteEmailIdentityResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_email_identity_policy_response(
    result: &DeleteEmailIdentityPolicyResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_email_template_response(
    result: &DeleteEmailTemplateResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_multi_region_endpoint_response(
    result: &DeleteMultiRegionEndpointResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_suppressed_destination_response(
    result: &DeleteSuppressedDestinationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_tenant_response(result: &DeleteTenantResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_tenant_resource_association_response(
    result: &DeleteTenantResourceAssociationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_account_response(result: &GetAccountResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_blacklist_reports_response(
    result: &GetBlacklistReportsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_configuration_set_response(
    result: &GetConfigurationSetResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_configuration_set_event_destinations_response(
    result: &GetConfigurationSetEventDestinationsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_contact_response(result: &GetContactResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_contact_list_response(result: &GetContactListResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_custom_verification_email_template_response(
    result: &GetCustomVerificationEmailTemplateResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_dedicated_ip_response(result: &GetDedicatedIpResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_dedicated_ip_pool_response(
    result: &GetDedicatedIpPoolResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_dedicated_ips_response(result: &GetDedicatedIpsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_deliverability_dashboard_options_response(
    result: &GetDeliverabilityDashboardOptionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_deliverability_test_report_response(
    result: &GetDeliverabilityTestReportResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_domain_deliverability_campaign_response(
    result: &GetDomainDeliverabilityCampaignResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_domain_statistics_report_response(
    result: &GetDomainStatisticsReportResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_email_address_insights_response(
    result: &GetEmailAddressInsightsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_email_identity_response(result: &GetEmailIdentityResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_email_identity_policies_response(
    result: &GetEmailIdentityPoliciesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_email_template_response(result: &GetEmailTemplateResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_export_job_response(result: &GetExportJobResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_import_job_response(result: &GetImportJobResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_message_insights_response(
    result: &GetMessageInsightsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_multi_region_endpoint_response(
    result: &GetMultiRegionEndpointResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_reputation_entity_response(
    result: &GetReputationEntityResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_suppressed_destination_response(
    result: &GetSuppressedDestinationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_tenant_response(result: &GetTenantResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_configuration_sets_response(
    result: &ListConfigurationSetsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_contact_lists_response(result: &ListContactListsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_contacts_response(result: &ListContactsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_custom_verification_email_templates_response(
    result: &ListCustomVerificationEmailTemplatesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_dedicated_ip_pools_response(
    result: &ListDedicatedIpPoolsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_deliverability_test_reports_response(
    result: &ListDeliverabilityTestReportsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_domain_deliverability_campaigns_response(
    result: &ListDomainDeliverabilityCampaignsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_email_identities_response(
    result: &ListEmailIdentitiesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_email_templates_response(
    result: &ListEmailTemplatesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_export_jobs_response(result: &ListExportJobsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_import_jobs_response(result: &ListImportJobsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_multi_region_endpoints_response(
    result: &ListMultiRegionEndpointsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_recommendations_response(
    result: &ListRecommendationsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_reputation_entities_response(
    result: &ListReputationEntitiesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_resource_tenants_response(
    result: &ListResourceTenantsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_suppressed_destinations_response(
    result: &ListSuppressedDestinationsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_tags_for_resource_response(
    result: &ListTagsForResourceResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_tenant_resources_response(
    result: &ListTenantResourcesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_tenants_response(result: &ListTenantsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_account_dedicated_ip_warmup_attributes_response(
    result: &PutAccountDedicatedIpWarmupAttributesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_account_details_response(result: &PutAccountDetailsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_account_sending_attributes_response(
    result: &PutAccountSendingAttributesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_account_suppression_attributes_response(
    result: &PutAccountSuppressionAttributesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_account_vdm_attributes_response(
    result: &PutAccountVdmAttributesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_configuration_set_archiving_options_response(
    result: &PutConfigurationSetArchivingOptionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_configuration_set_delivery_options_response(
    result: &PutConfigurationSetDeliveryOptionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_configuration_set_reputation_options_response(
    result: &PutConfigurationSetReputationOptionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_configuration_set_sending_options_response(
    result: &PutConfigurationSetSendingOptionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_configuration_set_suppression_options_response(
    result: &PutConfigurationSetSuppressionOptionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_configuration_set_tracking_options_response(
    result: &PutConfigurationSetTrackingOptionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_configuration_set_vdm_options_response(
    result: &PutConfigurationSetVdmOptionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_dedicated_ip_in_pool_response(
    result: &PutDedicatedIpInPoolResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_dedicated_ip_pool_scaling_attributes_response(
    result: &PutDedicatedIpPoolScalingAttributesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_dedicated_ip_warmup_attributes_response(
    result: &PutDedicatedIpWarmupAttributesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_deliverability_dashboard_option_response(
    result: &PutDeliverabilityDashboardOptionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_email_identity_configuration_set_attributes_response(
    result: &PutEmailIdentityConfigurationSetAttributesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_email_identity_dkim_attributes_response(
    result: &PutEmailIdentityDkimAttributesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_email_identity_dkim_signing_attributes_response(
    result: &PutEmailIdentityDkimSigningAttributesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_email_identity_feedback_attributes_response(
    result: &PutEmailIdentityFeedbackAttributesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_email_identity_mail_from_attributes_response(
    result: &PutEmailIdentityMailFromAttributesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_suppressed_destination_response(
    result: &PutSuppressedDestinationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_send_bulk_email_response(result: &SendBulkEmailResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_send_custom_verification_email_response(
    result: &SendCustomVerificationEmailResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_send_email_response(result: &SendEmailResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_tag_resource_response(result: &TagResourceResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_test_render_email_template_response(
    result: &TestRenderEmailTemplateResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_untag_resource_response(result: &UntagResourceResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_configuration_set_event_destination_response(
    result: &UpdateConfigurationSetEventDestinationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_contact_response(result: &UpdateContactResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_contact_list_response(result: &UpdateContactListResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_custom_verification_email_template_response(
    result: &UpdateCustomVerificationEmailTemplateResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_email_identity_policy_response(
    result: &UpdateEmailIdentityPolicyResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_email_template_response(
    result: &UpdateEmailTemplateResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_reputation_entity_customer_managed_status_response(
    result: &UpdateReputationEntityCustomerManagedStatusResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_reputation_entity_policy_response(
    result: &UpdateReputationEntityPolicyResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_batch_get_metric_data_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<BatchGetMetricDataRequest, String> {
    let mut input = BatchGetMetricDataRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<BatchGetMetricDataRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize BatchGetMetricData request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_cancel_export_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CancelExportJobRequest, String> {
    let mut input = CancelExportJobRequest::default();
    for (name, value) in labels {
        match *name {
            "JobId" => {
                input.job_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_configuration_set_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateConfigurationSetRequest, String> {
    let mut input = CreateConfigurationSetRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateConfigurationSetRequest>(&request.body).map_err(
            |err| format!("failed to deserialize CreateConfigurationSet request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_configuration_set_event_destination_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateConfigurationSetEventDestinationRequest, String> {
    let mut input = CreateConfigurationSetEventDestinationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateConfigurationSetEventDestinationRequest>(
            &request.body,
        )
        .map_err(|err| {
            format!("failed to deserialize CreateConfigurationSetEventDestination request: {err}")
        })?;
    }
    for (name, value) in labels {
        match *name {
            "ConfigurationSetName" => {
                input.configuration_set_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_contact_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateContactRequest, String> {
    let mut input = CreateContactRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateContactRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateContact request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "ContactListName" => {
                input.contact_list_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_contact_list_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateContactListRequest, String> {
    let mut input = CreateContactListRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateContactListRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateContactList request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_custom_verification_email_template_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateCustomVerificationEmailTemplateRequest, String> {
    let mut input = CreateCustomVerificationEmailTemplateRequest::default();
    if !request.body.is_empty() {
        input =
            serde_json::from_slice::<CreateCustomVerificationEmailTemplateRequest>(&request.body)
                .map_err(|err| {
                format!(
                    "failed to deserialize CreateCustomVerificationEmailTemplate request: {err}"
                )
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_dedicated_ip_pool_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateDedicatedIpPoolRequest, String> {
    let mut input = CreateDedicatedIpPoolRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateDedicatedIpPoolRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateDedicatedIpPool request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_deliverability_test_report_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateDeliverabilityTestReportRequest, String> {
    let mut input = CreateDeliverabilityTestReportRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateDeliverabilityTestReportRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize CreateDeliverabilityTestReport request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_email_identity_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateEmailIdentityRequest, String> {
    let mut input = CreateEmailIdentityRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateEmailIdentityRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateEmailIdentity request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_email_identity_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateEmailIdentityPolicyRequest, String> {
    let mut input = CreateEmailIdentityPolicyRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateEmailIdentityPolicyRequest>(&request.body).map_err(
            |err| format!("failed to deserialize CreateEmailIdentityPolicy request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "EmailIdentity" => {
                input.email_identity = value.to_string();
            }
            "PolicyName" => {
                input.policy_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_email_template_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateEmailTemplateRequest, String> {
    let mut input = CreateEmailTemplateRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateEmailTemplateRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateEmailTemplate request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_export_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateExportJobRequest, String> {
    let mut input = CreateExportJobRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateExportJobRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateExportJob request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_import_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateImportJobRequest, String> {
    let mut input = CreateImportJobRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateImportJobRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateImportJob request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_multi_region_endpoint_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateMultiRegionEndpointRequest, String> {
    let mut input = CreateMultiRegionEndpointRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateMultiRegionEndpointRequest>(&request.body).map_err(
            |err| format!("failed to deserialize CreateMultiRegionEndpoint request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_tenant_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateTenantRequest, String> {
    let mut input = CreateTenantRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateTenantRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateTenant request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_tenant_resource_association_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateTenantResourceAssociationRequest, String> {
    let mut input = CreateTenantResourceAssociationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateTenantResourceAssociationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize CreateTenantResourceAssociation request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_configuration_set_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteConfigurationSetRequest, String> {
    let mut input = DeleteConfigurationSetRequest::default();
    for (name, value) in labels {
        match *name {
            "ConfigurationSetName" => {
                input.configuration_set_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_configuration_set_event_destination_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteConfigurationSetEventDestinationRequest, String> {
    let mut input = DeleteConfigurationSetEventDestinationRequest::default();
    for (name, value) in labels {
        match *name {
            "ConfigurationSetName" => {
                input.configuration_set_name = value.to_string();
            }
            "EventDestinationName" => {
                input.event_destination_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_contact_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteContactRequest, String> {
    let mut input = DeleteContactRequest::default();
    for (name, value) in labels {
        match *name {
            "ContactListName" => {
                input.contact_list_name = value.to_string();
            }
            "EmailAddress" => {
                input.email_address = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_contact_list_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteContactListRequest, String> {
    let mut input = DeleteContactListRequest::default();
    for (name, value) in labels {
        match *name {
            "ContactListName" => {
                input.contact_list_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_custom_verification_email_template_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteCustomVerificationEmailTemplateRequest, String> {
    let mut input = DeleteCustomVerificationEmailTemplateRequest::default();
    for (name, value) in labels {
        match *name {
            "TemplateName" => {
                input.template_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_dedicated_ip_pool_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteDedicatedIpPoolRequest, String> {
    let mut input = DeleteDedicatedIpPoolRequest::default();
    for (name, value) in labels {
        match *name {
            "PoolName" => {
                input.pool_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_email_identity_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteEmailIdentityRequest, String> {
    let mut input = DeleteEmailIdentityRequest::default();
    for (name, value) in labels {
        match *name {
            "EmailIdentity" => {
                input.email_identity = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_email_identity_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteEmailIdentityPolicyRequest, String> {
    let mut input = DeleteEmailIdentityPolicyRequest::default();
    for (name, value) in labels {
        match *name {
            "EmailIdentity" => {
                input.email_identity = value.to_string();
            }
            "PolicyName" => {
                input.policy_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_email_template_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteEmailTemplateRequest, String> {
    let mut input = DeleteEmailTemplateRequest::default();
    for (name, value) in labels {
        match *name {
            "TemplateName" => {
                input.template_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_multi_region_endpoint_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteMultiRegionEndpointRequest, String> {
    let mut input = DeleteMultiRegionEndpointRequest::default();
    for (name, value) in labels {
        match *name {
            "EndpointName" => {
                input.endpoint_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_suppressed_destination_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteSuppressedDestinationRequest, String> {
    let mut input = DeleteSuppressedDestinationRequest::default();
    for (name, value) in labels {
        match *name {
            "EmailAddress" => {
                input.email_address = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_tenant_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteTenantRequest, String> {
    let mut input = DeleteTenantRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DeleteTenantRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize DeleteTenant request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_tenant_resource_association_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteTenantResourceAssociationRequest, String> {
    let mut input = DeleteTenantResourceAssociationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DeleteTenantResourceAssociationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize DeleteTenantResourceAssociation request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_account_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetAccountRequest, String> {
    let input = GetAccountRequest {};
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_blacklist_reports_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetBlacklistReportsRequest, String> {
    let mut input = GetBlacklistReportsRequest::default();
    if let Some(value) = query.get("BlacklistItemNames") {
        input.blacklist_item_names = value
            .split(',')
            .filter(|item| !item.trim().is_empty())
            .map(|item| Ok(item.trim().to_string()))
            .collect::<Result<Vec<_>, String>>()?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_configuration_set_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetConfigurationSetRequest, String> {
    let mut input = GetConfigurationSetRequest::default();
    for (name, value) in labels {
        match *name {
            "ConfigurationSetName" => {
                input.configuration_set_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_configuration_set_event_destinations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetConfigurationSetEventDestinationsRequest, String> {
    let mut input = GetConfigurationSetEventDestinationsRequest::default();
    for (name, value) in labels {
        match *name {
            "ConfigurationSetName" => {
                input.configuration_set_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_contact_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetContactRequest, String> {
    let mut input = GetContactRequest::default();
    for (name, value) in labels {
        match *name {
            "ContactListName" => {
                input.contact_list_name = value.to_string();
            }
            "EmailAddress" => {
                input.email_address = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_contact_list_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetContactListRequest, String> {
    let mut input = GetContactListRequest::default();
    for (name, value) in labels {
        match *name {
            "ContactListName" => {
                input.contact_list_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_custom_verification_email_template_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetCustomVerificationEmailTemplateRequest, String> {
    let mut input = GetCustomVerificationEmailTemplateRequest::default();
    for (name, value) in labels {
        match *name {
            "TemplateName" => {
                input.template_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_dedicated_ip_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetDedicatedIpRequest, String> {
    let mut input = GetDedicatedIpRequest::default();
    for (name, value) in labels {
        match *name {
            "Ip" => {
                input.ip = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_dedicated_ip_pool_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetDedicatedIpPoolRequest, String> {
    let mut input = GetDedicatedIpPoolRequest::default();
    for (name, value) in labels {
        match *name {
            "PoolName" => {
                input.pool_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_dedicated_ips_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetDedicatedIpsRequest, String> {
    let mut input = GetDedicatedIpsRequest::default();
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("PageSize") {
        input.page_size = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("PoolName") {
        input.pool_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_deliverability_dashboard_options_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetDeliverabilityDashboardOptionsRequest, String> {
    let input = GetDeliverabilityDashboardOptionsRequest {};
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_deliverability_test_report_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetDeliverabilityTestReportRequest, String> {
    let mut input = GetDeliverabilityTestReportRequest::default();
    for (name, value) in labels {
        match *name {
            "ReportId" => {
                input.report_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_domain_deliverability_campaign_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetDomainDeliverabilityCampaignRequest, String> {
    let mut input = GetDomainDeliverabilityCampaignRequest::default();
    for (name, value) in labels {
        match *name {
            "CampaignId" => {
                input.campaign_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_domain_statistics_report_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetDomainStatisticsReportRequest, String> {
    let mut input = GetDomainStatisticsReportRequest::default();
    for (name, value) in labels {
        match *name {
            "Domain" => {
                input.domain = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("EndDate") {
        input.end_date = value
            .parse::<f64>()
            .ok()
            .or_else(|| {
                chrono::DateTime::parse_from_rfc3339(value)
                    .ok()
                    .map(|dt| dt.timestamp() as f64)
            })
            .ok_or_else(|| format!("failed to parse timestamp: {}", value))?;
    }
    if let Some(value) = query.get("StartDate") {
        input.start_date = value
            .parse::<f64>()
            .ok()
            .or_else(|| {
                chrono::DateTime::parse_from_rfc3339(value)
                    .ok()
                    .map(|dt| dt.timestamp() as f64)
            })
            .ok_or_else(|| format!("failed to parse timestamp: {}", value))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_email_address_insights_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetEmailAddressInsightsRequest, String> {
    let mut input = GetEmailAddressInsightsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetEmailAddressInsightsRequest>(&request.body).map_err(
            |err| format!("failed to deserialize GetEmailAddressInsights request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_email_identity_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetEmailIdentityRequest, String> {
    let mut input = GetEmailIdentityRequest::default();
    for (name, value) in labels {
        match *name {
            "EmailIdentity" => {
                input.email_identity = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_email_identity_policies_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetEmailIdentityPoliciesRequest, String> {
    let mut input = GetEmailIdentityPoliciesRequest::default();
    for (name, value) in labels {
        match *name {
            "EmailIdentity" => {
                input.email_identity = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_email_template_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetEmailTemplateRequest, String> {
    let mut input = GetEmailTemplateRequest::default();
    for (name, value) in labels {
        match *name {
            "TemplateName" => {
                input.template_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_export_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetExportJobRequest, String> {
    let mut input = GetExportJobRequest::default();
    for (name, value) in labels {
        match *name {
            "JobId" => {
                input.job_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_import_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetImportJobRequest, String> {
    let mut input = GetImportJobRequest::default();
    for (name, value) in labels {
        match *name {
            "JobId" => {
                input.job_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_message_insights_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetMessageInsightsRequest, String> {
    let mut input = GetMessageInsightsRequest::default();
    for (name, value) in labels {
        match *name {
            "MessageId" => {
                input.message_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_multi_region_endpoint_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetMultiRegionEndpointRequest, String> {
    let mut input = GetMultiRegionEndpointRequest::default();
    for (name, value) in labels {
        match *name {
            "EndpointName" => {
                input.endpoint_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_reputation_entity_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetReputationEntityRequest, String> {
    let mut input = GetReputationEntityRequest::default();
    for (name, value) in labels {
        match *name {
            "ReputationEntityReference" => {
                input.reputation_entity_reference = value.to_string();
            }
            "ReputationEntityType" => {
                input.reputation_entity_type = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_suppressed_destination_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetSuppressedDestinationRequest, String> {
    let mut input = GetSuppressedDestinationRequest::default();
    for (name, value) in labels {
        match *name {
            "EmailAddress" => {
                input.email_address = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_tenant_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetTenantRequest, String> {
    let mut input = GetTenantRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetTenantRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize GetTenant request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_configuration_sets_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListConfigurationSetsRequest, String> {
    let mut input = ListConfigurationSetsRequest::default();
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("PageSize") {
        input.page_size = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_contact_lists_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListContactListsRequest, String> {
    let mut input = ListContactListsRequest::default();
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("PageSize") {
        input.page_size = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_contacts_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListContactsRequest, String> {
    let mut input = ListContactsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListContactsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListContacts request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "ContactListName" => {
                input.contact_list_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_custom_verification_email_templates_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListCustomVerificationEmailTemplatesRequest, String> {
    let mut input = ListCustomVerificationEmailTemplatesRequest::default();
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("PageSize") {
        input.page_size = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_dedicated_ip_pools_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListDedicatedIpPoolsRequest, String> {
    let mut input = ListDedicatedIpPoolsRequest::default();
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("PageSize") {
        input.page_size = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_deliverability_test_reports_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListDeliverabilityTestReportsRequest, String> {
    let mut input = ListDeliverabilityTestReportsRequest::default();
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("PageSize") {
        input.page_size = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_domain_deliverability_campaigns_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListDomainDeliverabilityCampaignsRequest, String> {
    let mut input = ListDomainDeliverabilityCampaignsRequest::default();
    for (name, value) in labels {
        match *name {
            "SubscribedDomain" => {
                input.subscribed_domain = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("EndDate") {
        input.end_date = value
            .parse::<f64>()
            .ok()
            .or_else(|| {
                chrono::DateTime::parse_from_rfc3339(value)
                    .ok()
                    .map(|dt| dt.timestamp() as f64)
            })
            .ok_or_else(|| format!("failed to parse timestamp: {}", value))?;
    }
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("PageSize") {
        input.page_size = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("StartDate") {
        input.start_date = value
            .parse::<f64>()
            .ok()
            .or_else(|| {
                chrono::DateTime::parse_from_rfc3339(value)
                    .ok()
                    .map(|dt| dt.timestamp() as f64)
            })
            .ok_or_else(|| format!("failed to parse timestamp: {}", value))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_email_identities_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListEmailIdentitiesRequest, String> {
    let mut input = ListEmailIdentitiesRequest::default();
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("PageSize") {
        input.page_size = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_email_templates_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListEmailTemplatesRequest, String> {
    let mut input = ListEmailTemplatesRequest::default();
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("PageSize") {
        input.page_size = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_export_jobs_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListExportJobsRequest, String> {
    let mut input = ListExportJobsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListExportJobsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListExportJobs request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_import_jobs_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListImportJobsRequest, String> {
    let mut input = ListImportJobsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListImportJobsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListImportJobs request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_multi_region_endpoints_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListMultiRegionEndpointsRequest, String> {
    let mut input = ListMultiRegionEndpointsRequest::default();
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("PageSize") {
        input.page_size = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_recommendations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListRecommendationsRequest, String> {
    let mut input = ListRecommendationsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListRecommendationsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListRecommendations request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_reputation_entities_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListReputationEntitiesRequest, String> {
    let mut input = ListReputationEntitiesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListReputationEntitiesRequest>(&request.body).map_err(
            |err| format!("failed to deserialize ListReputationEntities request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_resource_tenants_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListResourceTenantsRequest, String> {
    let mut input = ListResourceTenantsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListResourceTenantsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListResourceTenants request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_suppressed_destinations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListSuppressedDestinationsRequest, String> {
    let mut input = ListSuppressedDestinationsRequest::default();
    if let Some(value) = query.get("EndDate") {
        input.end_date = Some(
            value
                .parse::<f64>()
                .ok()
                .or_else(|| {
                    chrono::DateTime::parse_from_rfc3339(value)
                        .ok()
                        .map(|dt| dt.timestamp() as f64)
                })
                .ok_or_else(|| format!("failed to parse timestamp: {}", value))?,
        );
    }
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("PageSize") {
        input.page_size = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("Reason") {
        input.reasons = Some(
            value
                .split(',')
                .filter(|item| !item.trim().is_empty())
                .map(|item| Ok(item.trim().to_string()))
                .collect::<Result<Vec<_>, String>>()?,
        );
    }
    if let Some(value) = query.get("StartDate") {
        input.start_date = Some(
            value
                .parse::<f64>()
                .ok()
                .or_else(|| {
                    chrono::DateTime::parse_from_rfc3339(value)
                        .ok()
                        .map(|dt| dt.timestamp() as f64)
                })
                .ok_or_else(|| format!("failed to parse timestamp: {}", value))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_tags_for_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListTagsForResourceRequest, String> {
    let mut input = ListTagsForResourceRequest::default();
    if let Some(value) = query.get("ResourceArn") {
        input.resource_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_tenant_resources_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListTenantResourcesRequest, String> {
    let mut input = ListTenantResourcesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListTenantResourcesRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListTenantResources request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_tenants_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListTenantsRequest, String> {
    let mut input = ListTenantsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListTenantsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListTenants request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_account_dedicated_ip_warmup_attributes_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutAccountDedicatedIpWarmupAttributesRequest, String> {
    let mut input = PutAccountDedicatedIpWarmupAttributesRequest::default();
    if !request.body.is_empty() {
        input =
            serde_json::from_slice::<PutAccountDedicatedIpWarmupAttributesRequest>(&request.body)
                .map_err(|err| {
                format!(
                    "failed to deserialize PutAccountDedicatedIpWarmupAttributes request: {err}"
                )
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_account_details_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutAccountDetailsRequest, String> {
    let mut input = PutAccountDetailsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutAccountDetailsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize PutAccountDetails request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_account_sending_attributes_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutAccountSendingAttributesRequest, String> {
    let mut input = PutAccountSendingAttributesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutAccountSendingAttributesRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize PutAccountSendingAttributes request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_account_suppression_attributes_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutAccountSuppressionAttributesRequest, String> {
    let mut input = PutAccountSuppressionAttributesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutAccountSuppressionAttributesRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize PutAccountSuppressionAttributes request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_account_vdm_attributes_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutAccountVdmAttributesRequest, String> {
    let mut input = PutAccountVdmAttributesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutAccountVdmAttributesRequest>(&request.body).map_err(
            |err| format!("failed to deserialize PutAccountVdmAttributes request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_configuration_set_archiving_options_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutConfigurationSetArchivingOptionsRequest, String> {
    let mut input = PutConfigurationSetArchivingOptionsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutConfigurationSetArchivingOptionsRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize PutConfigurationSetArchivingOptions request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "ConfigurationSetName" => {
                input.configuration_set_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_configuration_set_delivery_options_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutConfigurationSetDeliveryOptionsRequest, String> {
    let mut input = PutConfigurationSetDeliveryOptionsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutConfigurationSetDeliveryOptionsRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize PutConfigurationSetDeliveryOptions request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "ConfigurationSetName" => {
                input.configuration_set_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_configuration_set_reputation_options_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutConfigurationSetReputationOptionsRequest, String> {
    let mut input = PutConfigurationSetReputationOptionsRequest::default();
    if !request.body.is_empty() {
        input =
            serde_json::from_slice::<PutConfigurationSetReputationOptionsRequest>(&request.body)
                .map_err(|err| {
                    format!(
                        "failed to deserialize PutConfigurationSetReputationOptions request: {err}"
                    )
                })?;
    }
    for (name, value) in labels {
        match *name {
            "ConfigurationSetName" => {
                input.configuration_set_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_configuration_set_sending_options_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutConfigurationSetSendingOptionsRequest, String> {
    let mut input = PutConfigurationSetSendingOptionsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutConfigurationSetSendingOptionsRequest>(&request.body)
            .map_err(|err| {
            format!("failed to deserialize PutConfigurationSetSendingOptions request: {err}")
        })?;
    }
    for (name, value) in labels {
        match *name {
            "ConfigurationSetName" => {
                input.configuration_set_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_configuration_set_suppression_options_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutConfigurationSetSuppressionOptionsRequest, String> {
    let mut input = PutConfigurationSetSuppressionOptionsRequest::default();
    if !request.body.is_empty() {
        input =
            serde_json::from_slice::<PutConfigurationSetSuppressionOptionsRequest>(&request.body)
                .map_err(|err| {
                format!(
                    "failed to deserialize PutConfigurationSetSuppressionOptions request: {err}"
                )
            })?;
    }
    for (name, value) in labels {
        match *name {
            "ConfigurationSetName" => {
                input.configuration_set_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_configuration_set_tracking_options_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutConfigurationSetTrackingOptionsRequest, String> {
    let mut input = PutConfigurationSetTrackingOptionsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutConfigurationSetTrackingOptionsRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize PutConfigurationSetTrackingOptions request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "ConfigurationSetName" => {
                input.configuration_set_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_configuration_set_vdm_options_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutConfigurationSetVdmOptionsRequest, String> {
    let mut input = PutConfigurationSetVdmOptionsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutConfigurationSetVdmOptionsRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize PutConfigurationSetVdmOptions request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "ConfigurationSetName" => {
                input.configuration_set_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_dedicated_ip_in_pool_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutDedicatedIpInPoolRequest, String> {
    let mut input = PutDedicatedIpInPoolRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutDedicatedIpInPoolRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize PutDedicatedIpInPool request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "Ip" => {
                input.ip = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_dedicated_ip_pool_scaling_attributes_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutDedicatedIpPoolScalingAttributesRequest, String> {
    let mut input = PutDedicatedIpPoolScalingAttributesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutDedicatedIpPoolScalingAttributesRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize PutDedicatedIpPoolScalingAttributes request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "PoolName" => {
                input.pool_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_dedicated_ip_warmup_attributes_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutDedicatedIpWarmupAttributesRequest, String> {
    let mut input = PutDedicatedIpWarmupAttributesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutDedicatedIpWarmupAttributesRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize PutDedicatedIpWarmupAttributes request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "Ip" => {
                input.ip = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_deliverability_dashboard_option_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutDeliverabilityDashboardOptionRequest, String> {
    let mut input = PutDeliverabilityDashboardOptionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutDeliverabilityDashboardOptionRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize PutDeliverabilityDashboardOption request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_email_identity_configuration_set_attributes_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutEmailIdentityConfigurationSetAttributesRequest, String> {
    let mut input = PutEmailIdentityConfigurationSetAttributesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutEmailIdentityConfigurationSetAttributesRequest>(
            &request.body,
        )
        .map_err(|err| {
            format!(
                "failed to deserialize PutEmailIdentityConfigurationSetAttributes request: {err}"
            )
        })?;
    }
    for (name, value) in labels {
        match *name {
            "EmailIdentity" => {
                input.email_identity = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_email_identity_dkim_attributes_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutEmailIdentityDkimAttributesRequest, String> {
    let mut input = PutEmailIdentityDkimAttributesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutEmailIdentityDkimAttributesRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize PutEmailIdentityDkimAttributes request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "EmailIdentity" => {
                input.email_identity = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_email_identity_dkim_signing_attributes_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutEmailIdentityDkimSigningAttributesRequest, String> {
    let mut input = PutEmailIdentityDkimSigningAttributesRequest::default();
    if !request.body.is_empty() {
        input =
            serde_json::from_slice::<PutEmailIdentityDkimSigningAttributesRequest>(&request.body)
                .map_err(|err| {
                format!(
                    "failed to deserialize PutEmailIdentityDkimSigningAttributes request: {err}"
                )
            })?;
    }
    for (name, value) in labels {
        match *name {
            "EmailIdentity" => {
                input.email_identity = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_email_identity_feedback_attributes_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutEmailIdentityFeedbackAttributesRequest, String> {
    let mut input = PutEmailIdentityFeedbackAttributesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutEmailIdentityFeedbackAttributesRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize PutEmailIdentityFeedbackAttributes request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "EmailIdentity" => {
                input.email_identity = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_email_identity_mail_from_attributes_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutEmailIdentityMailFromAttributesRequest, String> {
    let mut input = PutEmailIdentityMailFromAttributesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutEmailIdentityMailFromAttributesRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize PutEmailIdentityMailFromAttributes request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "EmailIdentity" => {
                input.email_identity = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_suppressed_destination_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutSuppressedDestinationRequest, String> {
    let mut input = PutSuppressedDestinationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutSuppressedDestinationRequest>(&request.body).map_err(
            |err| format!("failed to deserialize PutSuppressedDestination request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_send_bulk_email_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<SendBulkEmailRequest, String> {
    let mut input = SendBulkEmailRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<SendBulkEmailRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize SendBulkEmail request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_send_custom_verification_email_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<SendCustomVerificationEmailRequest, String> {
    let mut input = SendCustomVerificationEmailRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<SendCustomVerificationEmailRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize SendCustomVerificationEmail request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_send_email_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<SendEmailRequest, String> {
    let mut input = SendEmailRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<SendEmailRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize SendEmail request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_tag_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<TagResourceRequest, String> {
    let mut input = TagResourceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<TagResourceRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize TagResource request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_test_render_email_template_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<TestRenderEmailTemplateRequest, String> {
    let mut input = TestRenderEmailTemplateRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<TestRenderEmailTemplateRequest>(&request.body).map_err(
            |err| format!("failed to deserialize TestRenderEmailTemplate request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "TemplateName" => {
                input.template_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_untag_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UntagResourceRequest, String> {
    let mut input = UntagResourceRequest::default();
    if let Some(value) = query.get("ResourceArn") {
        input.resource_arn = value.to_string();
    }
    if let Some(value) = query.get("TagKeys") {
        input.tag_keys = value
            .split(',')
            .filter(|item| !item.trim().is_empty())
            .map(|item| Ok(item.trim().to_string()))
            .collect::<Result<Vec<_>, String>>()?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_configuration_set_event_destination_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateConfigurationSetEventDestinationRequest, String> {
    let mut input = UpdateConfigurationSetEventDestinationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateConfigurationSetEventDestinationRequest>(
            &request.body,
        )
        .map_err(|err| {
            format!("failed to deserialize UpdateConfigurationSetEventDestination request: {err}")
        })?;
    }
    for (name, value) in labels {
        match *name {
            "ConfigurationSetName" => {
                input.configuration_set_name = value.to_string();
            }
            "EventDestinationName" => {
                input.event_destination_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_contact_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateContactRequest, String> {
    let mut input = UpdateContactRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateContactRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateContact request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "ContactListName" => {
                input.contact_list_name = value.to_string();
            }
            "EmailAddress" => {
                input.email_address = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_contact_list_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateContactListRequest, String> {
    let mut input = UpdateContactListRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateContactListRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateContactList request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "ContactListName" => {
                input.contact_list_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_custom_verification_email_template_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateCustomVerificationEmailTemplateRequest, String> {
    let mut input = UpdateCustomVerificationEmailTemplateRequest::default();
    if !request.body.is_empty() {
        input =
            serde_json::from_slice::<UpdateCustomVerificationEmailTemplateRequest>(&request.body)
                .map_err(|err| {
                format!(
                    "failed to deserialize UpdateCustomVerificationEmailTemplate request: {err}"
                )
            })?;
    }
    for (name, value) in labels {
        match *name {
            "TemplateName" => {
                input.template_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_email_identity_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateEmailIdentityPolicyRequest, String> {
    let mut input = UpdateEmailIdentityPolicyRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateEmailIdentityPolicyRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateEmailIdentityPolicy request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "EmailIdentity" => {
                input.email_identity = value.to_string();
            }
            "PolicyName" => {
                input.policy_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_email_template_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateEmailTemplateRequest, String> {
    let mut input = UpdateEmailTemplateRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateEmailTemplateRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateEmailTemplate request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "TemplateName" => {
                input.template_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_reputation_entity_customer_managed_status_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateReputationEntityCustomerManagedStatusRequest, String> {
    let mut input = UpdateReputationEntityCustomerManagedStatusRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateReputationEntityCustomerManagedStatusRequest>(
            &request.body,
        )
        .map_err(|err| {
            format!(
                "failed to deserialize UpdateReputationEntityCustomerManagedStatus request: {err}"
            )
        })?;
    }
    for (name, value) in labels {
        match *name {
            "ReputationEntityReference" => {
                input.reputation_entity_reference = value.to_string();
            }
            "ReputationEntityType" => {
                input.reputation_entity_type = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_reputation_entity_policy_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateReputationEntityPolicyRequest, String> {
    let mut input = UpdateReputationEntityPolicyRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateReputationEntityPolicyRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateReputationEntityPolicy request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "ReputationEntityReference" => {
                input.reputation_entity_reference = value.to_string();
            }
            "ReputationEntityType" => {
                input.reputation_entity_type = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}
