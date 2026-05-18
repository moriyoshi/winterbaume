use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, StatefulService,
    default_account_id,
};

use crate::state::{SesError, SesState};
use crate::types::TopicPreference;
use crate::views::SesStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct SesV2Service {
    pub(crate) state: Arc<BackendState<SesState>>,
    pub(crate) notifier: StateChangeNotifier<SesStateView>,
}

impl SesV2Service {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }

    /// Returns an `Arc` clone of the underlying per-account/region state
    /// holder. Used by `winterbaume_ses::SesService::with_sesv2_state` so
    /// the legacy v1 API can read and write the same `identities` map
    /// that the v2 API uses ; pass the same `Arc` to both services from
    /// the test harness and they will agree on which email identities
    /// exist regardless of which API created or deleted them.
    ///
    /// This is the canonical-store half of the v1/v2 state-coherence
    /// pattern: v2 owns the canonical state ( `SesState.identities` ),
    /// v1 reads / writes through this `Arc` when wired and falls back
    /// to its own `SesV1State.identities` map when not wired. See
    /// `.agents/docs/TODO.md` `ses-v1-v2-shared-backend` for the
    /// remaining families ( configuration sets, templates, suppression
    /// list, dedicated IP pools, account-level settings ) that will
    /// follow the same pattern.
    pub fn shared_state(&self) -> Arc<BackendState<SesState>> {
        Arc::clone(&self.state)
    }
}

impl Default for SesV2Service {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for SesV2Service {
    fn service_name(&self) -> &str {
        "ses"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://email\..*\.amazonaws\.com",
            r"https?://email\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl SesV2Service {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let raw_query = extract_query(&request.uri);
        let query_map: HashMap<String, String> = winterbaume_core::parse_query_string(&raw_query);
        let method = request.method.as_str();

        let segments: Vec<String> = path
            .trim_start_matches('/')
            .split('/')
            .map(percent_decode)
            .collect();
        let segs: Vec<&str> = segments.iter().map(|s| s.as_str()).collect();

        // All routes start with /v2/email/
        if segs.len() < 3 || segs[0] != "v2" || segs[1] != "email" {
            return rest_json_error(404, "UnknownOperationException", "Not found");
        }

        let response = self
            .dispatch_route(&state, method, &segs, &request, &query_map, &raw_query)
            .await;

        if response.status >= 200 && response.status < 300 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn dispatch_route(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        method: &str,
        segs: &[&str],
        request: &MockRequest,
        query_map: &HashMap<String, String>,
        raw_query: &str,
    ) -> MockResponse {
        match (method, segs[2], segs.len()) {
            // POST /v2/email/identities - CreateEmailIdentity
            ("POST", "identities", 3) => {
                self.handle_create_email_identity(state, request, &[], query_map)
                    .await
            }
            // GET /v2/email/identities - ListEmailIdentities
            ("GET", "identities", 3) => {
                self.handle_list_email_identities(state, request, &[], query_map)
                    .await
            }
            // GET /v2/email/identities/{name} - GetEmailIdentity
            ("GET", "identities", 4) => {
                let labels: &[(&str, &str)] = &[("EmailIdentity", segs[3])];
                self.handle_get_email_identity(state, request, labels, query_map)
                    .await
            }
            // DELETE /v2/email/identities/{name} - DeleteEmailIdentity
            ("DELETE", "identities", 4) => {
                let labels: &[(&str, &str)] = &[("EmailIdentity", segs[3])];
                self.handle_delete_email_identity(state, request, labels, query_map)
                    .await
            }
            // GET /v2/email/identities/{name}/policies - GetEmailIdentityPolicies
            ("GET", "identities", 5) if segs[4] == "policies" => {
                let labels: &[(&str, &str)] = &[("EmailIdentity", segs[3])];
                self.handle_get_email_identity_policies(state, request, labels, query_map)
                    .await
            }
            // POST /v2/email/identities/{name}/policies/{policyName} - CreateEmailIdentityPolicy
            ("POST", "identities", 6) if segs[4] == "policies" => {
                let labels: &[(&str, &str)] =
                    &[("EmailIdentity", segs[3]), ("PolicyName", segs[5])];
                self.handle_create_email_identity_policy(state, request, labels, query_map)
                    .await
            }
            // DELETE /v2/email/identities/{name}/policies/{policyName} - DeleteEmailIdentityPolicy
            ("DELETE", "identities", 6) if segs[4] == "policies" => {
                let labels: &[(&str, &str)] =
                    &[("EmailIdentity", segs[3]), ("PolicyName", segs[5])];
                self.handle_delete_email_identity_policy(state, request, labels, query_map)
                    .await
            }
            // PUT /v2/email/identities/{name}/policies/{policyName} - UpdateEmailIdentityPolicy
            ("PUT", "identities", 6) if segs[4] == "policies" => {
                let labels: &[(&str, &str)] =
                    &[("EmailIdentity", segs[3]), ("PolicyName", segs[5])];
                self.handle_update_email_identity_policy(state, request, labels, query_map)
                    .await
            }
            // POST /v2/email/outbound-emails - SendEmail
            ("POST", "outbound-emails", 3) => {
                self.handle_send_email(state, request, &[], query_map).await
            }
            // POST /v2/email/configuration-sets - CreateConfigurationSet
            ("POST", "configuration-sets", 3) => {
                self.handle_create_configuration_set(state, request, &[], query_map)
                    .await
            }
            // GET /v2/email/configuration-sets - ListConfigurationSets
            ("GET", "configuration-sets", 3) => {
                self.handle_list_configuration_sets(state, request, &[], query_map)
                    .await
            }
            // GET /v2/email/configuration-sets/{name} - GetConfigurationSet
            ("GET", "configuration-sets", 4) => {
                let labels: &[(&str, &str)] = &[("ConfigurationSetName", segs[3])];
                self.handle_get_configuration_set(state, request, labels, query_map)
                    .await
            }
            // DELETE /v2/email/configuration-sets/{name} - DeleteConfigurationSet
            ("DELETE", "configuration-sets", 4) => {
                let labels: &[(&str, &str)] = &[("ConfigurationSetName", segs[3])];
                self.handle_delete_configuration_set(state, request, labels, query_map)
                    .await
            }
            // POST /v2/email/contact-lists - CreateContactList
            ("POST", "contact-lists", 3) => {
                self.handle_create_contact_list(state, request, &[], query_map)
                    .await
            }
            // GET /v2/email/contact-lists - ListContactLists
            ("GET", "contact-lists", 3) => {
                self.handle_list_contact_lists(state, request, &[], query_map)
                    .await
            }
            // GET /v2/email/contact-lists/{name} - GetContactList
            ("GET", "contact-lists", 4) => {
                let labels: &[(&str, &str)] = &[("ContactListName", segs[3])];
                self.handle_get_contact_list(state, request, labels, query_map)
                    .await
            }
            // DELETE /v2/email/contact-lists/{name} - DeleteContactList
            ("DELETE", "contact-lists", 4) => {
                let labels: &[(&str, &str)] = &[("ContactListName", segs[3])];
                self.handle_delete_contact_list(state, request, labels, query_map)
                    .await
            }
            // POST /v2/email/contact-lists/{name}/contacts - CreateContact
            ("POST", "contact-lists", 5) if segs[4] == "contacts" => {
                let labels: &[(&str, &str)] = &[("ContactListName", segs[3])];
                self.handle_create_contact(state, request, labels, query_map)
                    .await
            }
            // POST /v2/email/contact-lists/{name}/contacts/list - ListContacts
            ("POST", "contact-lists", 6) if segs[4] == "contacts" && segs[5] == "list" => {
                let labels: &[(&str, &str)] = &[("ContactListName", segs[3])];
                self.handle_list_contacts(state, request, labels, query_map)
                    .await
            }
            // GET /v2/email/contact-lists/{name}/contacts/{email} - GetContact
            ("GET", "contact-lists", 6) if segs[4] == "contacts" => {
                let labels: &[(&str, &str)] =
                    &[("ContactListName", segs[3]), ("EmailAddress", segs[5])];
                self.handle_get_contact(state, request, labels, query_map)
                    .await
            }
            // DELETE /v2/email/contact-lists/{name}/contacts/{email} - DeleteContact
            ("DELETE", "contact-lists", 6) if segs[4] == "contacts" => {
                let labels: &[(&str, &str)] =
                    &[("ContactListName", segs[3]), ("EmailAddress", segs[5])];
                self.handle_delete_contact(state, request, labels, query_map)
                    .await
            }
            // POST /v2/email/dedicated-ip-pools - CreateDedicatedIpPool
            ("POST", "dedicated-ip-pools", 3) => {
                self.handle_create_dedicated_ip_pool(state, request, &[], query_map)
                    .await
            }
            // GET /v2/email/dedicated-ip-pools - ListDedicatedIpPools
            ("GET", "dedicated-ip-pools", 3) => {
                self.handle_list_dedicated_ip_pools(state, request, &[], query_map)
                    .await
            }
            // GET /v2/email/dedicated-ip-pools/{name} - GetDedicatedIpPool
            ("GET", "dedicated-ip-pools", 4) => {
                let labels: &[(&str, &str)] = &[("PoolName", segs[3])];
                self.handle_get_dedicated_ip_pool(state, request, labels, query_map)
                    .await
            }
            // DELETE /v2/email/dedicated-ip-pools/{name} - DeleteDedicatedIpPool
            ("DELETE", "dedicated-ip-pools", 4) => {
                let labels: &[(&str, &str)] = &[("PoolName", segs[3])];
                self.handle_delete_dedicated_ip_pool(state, request, labels, query_map)
                    .await
            }
            // GET /v2/email/tags - ListTagsForResource
            ("GET", "tags", 3) => {
                self.handle_list_tags_for_resource(state, request, &[], query_map)
                    .await
            }
            // POST /v2/email/tags - TagResource
            ("POST", "tags", 3) => {
                self.handle_tag_resource(state, request, &[], query_map)
                    .await
            }
            // DELETE /v2/email/tags - UntagResource
            ("DELETE", "tags", 3) => {
                self.handle_untag_resource(state, request, &[], query_map, raw_query)
                    .await
            }
            // POST /v2/email/metrics/batch - BatchGetMetricData
            ("POST", "metrics", 4) if segs[3] == "batch" => {
                self.handle_batch_get_metric_data(request, &[], query_map)
                    .await
            }
            // PUT /v2/email/export-jobs/{JobId}/cancel - CancelExportJob
            ("PUT", "export-jobs", 5) if segs[4] == "cancel" => {
                let labels: &[(&str, &str)] = &[("JobId", segs[3])];
                self.handle_cancel_export_job(state, request, labels, query_map)
                    .await
            }
            // POST /v2/email/configuration-sets/{name}/event-destinations - CreateConfigurationSetEventDestination
            ("POST", "configuration-sets", 5) if segs[4] == "event-destinations" => {
                let labels: &[(&str, &str)] = &[("ConfigurationSetName", segs[3])];
                self.handle_create_configuration_set_event_destination(
                    state, request, labels, query_map,
                )
                .await
            }
            // GET /v2/email/configuration-sets/{name}/event-destinations - GetConfigurationSetEventDestinations
            ("GET", "configuration-sets", 5) if segs[4] == "event-destinations" => {
                let labels: &[(&str, &str)] = &[("ConfigurationSetName", segs[3])];
                self.handle_get_configuration_set_event_destinations(
                    state, request, labels, query_map,
                )
                .await
            }
            // DELETE /v2/email/configuration-sets/{name}/event-destinations/{dest} - DeleteConfigurationSetEventDestination
            ("DELETE", "configuration-sets", 6) if segs[4] == "event-destinations" => {
                let labels: &[(&str, &str)] = &[
                    ("ConfigurationSetName", segs[3]),
                    ("EventDestinationName", segs[5]),
                ];
                self.handle_delete_configuration_set_event_destination(
                    state, request, labels, query_map,
                )
                .await
            }
            // PUT /v2/email/configuration-sets/{name}/event-destinations/{dest} - UpdateConfigurationSetEventDestination
            ("PUT", "configuration-sets", 6) if segs[4] == "event-destinations" => {
                let labels: &[(&str, &str)] = &[
                    ("ConfigurationSetName", segs[3]),
                    ("EventDestinationName", segs[5]),
                ];
                self.handle_update_configuration_set_event_destination(
                    state, request, labels, query_map,
                )
                .await
            }
            // PUT /v2/email/configuration-sets/{name}/archiving-options - PutConfigurationSetArchivingOptions
            ("PUT", "configuration-sets", 5) if segs[4] == "archiving-options" => {
                let labels: &[(&str, &str)] = &[("ConfigurationSetName", segs[3])];
                self.handle_put_configuration_set_archiving_options(
                    state, request, labels, query_map,
                )
                .await
            }
            // PUT /v2/email/configuration-sets/{name}/delivery-options - PutConfigurationSetDeliveryOptions
            ("PUT", "configuration-sets", 5) if segs[4] == "delivery-options" => {
                let labels: &[(&str, &str)] = &[("ConfigurationSetName", segs[3])];
                self.handle_put_configuration_set_delivery_options(
                    state, request, labels, query_map,
                )
                .await
            }
            // PUT /v2/email/configuration-sets/{name}/reputation-options - PutConfigurationSetReputationOptions
            ("PUT", "configuration-sets", 5) if segs[4] == "reputation-options" => {
                let labels: &[(&str, &str)] = &[("ConfigurationSetName", segs[3])];
                self.handle_put_configuration_set_reputation_options(
                    state, request, labels, query_map,
                )
                .await
            }
            // PUT /v2/email/configuration-sets/{name}/sending - PutConfigurationSetSendingOptions
            ("PUT", "configuration-sets", 5) if segs[4] == "sending" => {
                let labels: &[(&str, &str)] = &[("ConfigurationSetName", segs[3])];
                self.handle_put_configuration_set_sending_options(state, request, labels, query_map)
                    .await
            }
            // PUT /v2/email/configuration-sets/{name}/suppression-options - PutConfigurationSetSuppressionOptions
            ("PUT", "configuration-sets", 5) if segs[4] == "suppression-options" => {
                let labels: &[(&str, &str)] = &[("ConfigurationSetName", segs[3])];
                self.handle_put_configuration_set_suppression_options(
                    state, request, labels, query_map,
                )
                .await
            }
            // PUT /v2/email/configuration-sets/{name}/tracking-options - PutConfigurationSetTrackingOptions
            ("PUT", "configuration-sets", 5) if segs[4] == "tracking-options" => {
                let labels: &[(&str, &str)] = &[("ConfigurationSetName", segs[3])];
                self.handle_put_configuration_set_tracking_options(
                    state, request, labels, query_map,
                )
                .await
            }
            // PUT /v2/email/configuration-sets/{name}/vdm-options - PutConfigurationSetVdmOptions
            ("PUT", "configuration-sets", 5) if segs[4] == "vdm-options" => {
                let labels: &[(&str, &str)] = &[("ConfigurationSetName", segs[3])];
                self.handle_put_configuration_set_vdm_options(state, request, labels, query_map)
                    .await
            }
            // POST /v2/email/custom-verification-email-templates - CreateCustomVerificationEmailTemplate
            ("POST", "custom-verification-email-templates", 3) => {
                self.handle_create_custom_verification_email_template(
                    state,
                    request,
                    &[],
                    query_map,
                )
                .await
            }
            // GET /v2/email/custom-verification-email-templates - ListCustomVerificationEmailTemplates
            ("GET", "custom-verification-email-templates", 3) => {
                self.handle_list_custom_verification_email_templates(state, request, &[], query_map)
                    .await
            }
            // GET /v2/email/custom-verification-email-templates/{name} - GetCustomVerificationEmailTemplate
            ("GET", "custom-verification-email-templates", 4) => {
                let labels: &[(&str, &str)] = &[("TemplateName", segs[3])];
                self.handle_get_custom_verification_email_template(
                    state, request, labels, query_map,
                )
                .await
            }
            // DELETE /v2/email/custom-verification-email-templates/{name} - DeleteCustomVerificationEmailTemplate
            ("DELETE", "custom-verification-email-templates", 4) => {
                let labels: &[(&str, &str)] = &[("TemplateName", segs[3])];
                self.handle_delete_custom_verification_email_template(
                    state, request, labels, query_map,
                )
                .await
            }
            // PUT /v2/email/custom-verification-email-templates/{name} - UpdateCustomVerificationEmailTemplate
            ("PUT", "custom-verification-email-templates", 4) => {
                let labels: &[(&str, &str)] = &[("TemplateName", segs[3])];
                self.handle_update_custom_verification_email_template(
                    state, request, labels, query_map,
                )
                .await
            }
            // POST /v2/email/deliverability-dashboard/test - CreateDeliverabilityTestReport
            ("POST", "deliverability-dashboard", 4) if segs[3] == "test" => {
                self.handle_create_deliverability_test_report(state, request, &[], query_map)
                    .await
            }
            // GET /v2/email/deliverability-dashboard - GetDeliverabilityDashboardOptions
            ("GET", "deliverability-dashboard", 3) => {
                self.handle_get_deliverability_dashboard_options(state, request, &[], query_map)
                    .await
            }
            // PUT /v2/email/deliverability-dashboard - PutDeliverabilityDashboardOption
            ("PUT", "deliverability-dashboard", 3) => {
                self.handle_put_deliverability_dashboard_option(state, request, &[], query_map)
                    .await
            }
            // GET /v2/email/deliverability-dashboard/blacklist-report - GetBlacklistReports
            ("GET", "deliverability-dashboard", 4) if segs[3] == "blacklist-report" => {
                self.handle_get_blacklist_reports().await
            }
            // GET /v2/email/deliverability-dashboard/test-reports - ListDeliverabilityTestReports
            ("GET", "deliverability-dashboard", 4) if segs[3] == "test-reports" => {
                self.handle_list_deliverability_test_reports(state, request, &[], query_map)
                    .await
            }
            // GET /v2/email/deliverability-dashboard/test-reports/{id} - GetDeliverabilityTestReport
            ("GET", "deliverability-dashboard", 5) if segs[3] == "test-reports" => {
                let labels: &[(&str, &str)] = &[("ReportId", segs[4])];
                self.handle_get_deliverability_test_report(state, request, labels, query_map)
                    .await
            }
            // GET /v2/email/deliverability-dashboard/campaigns/{id} - GetDomainDeliverabilityCampaign
            ("GET", "deliverability-dashboard", 5) if segs[3] == "campaigns" => {
                self.handle_get_domain_deliverability_campaign().await
            }
            // GET /v2/email/deliverability-dashboard/statistics-report/{domain} - GetDomainStatisticsReport
            ("GET", "deliverability-dashboard", 5) if segs[3] == "statistics-report" => {
                self.handle_get_domain_statistics_report().await
            }
            // GET /v2/email/deliverability-dashboard/domains/{domain}/campaigns - ListDomainDeliverabilityCampaigns
            ("GET", "deliverability-dashboard", 6)
                if segs[3] == "domains" && segs[5] == "campaigns" =>
            {
                self.handle_list_domain_deliverability_campaigns().await
            }
            // POST /v2/email/templates - CreateEmailTemplate
            ("POST", "templates", 3) => {
                self.handle_create_email_template(state, request, &[], query_map)
                    .await
            }
            // GET /v2/email/templates - ListEmailTemplates
            ("GET", "templates", 3) => {
                self.handle_list_email_templates(state, request, &[], query_map)
                    .await
            }
            // GET /v2/email/templates/{name} - GetEmailTemplate
            ("GET", "templates", 4) => {
                let labels: &[(&str, &str)] = &[("TemplateName", segs[3])];
                self.handle_get_email_template(state, request, labels, query_map)
                    .await
            }
            // DELETE /v2/email/templates/{name} - DeleteEmailTemplate
            ("DELETE", "templates", 4) => {
                let labels: &[(&str, &str)] = &[("TemplateName", segs[3])];
                self.handle_delete_email_template(state, request, labels, query_map)
                    .await
            }
            // PUT /v2/email/templates/{name} - UpdateEmailTemplate
            ("PUT", "templates", 4) => {
                let labels: &[(&str, &str)] = &[("TemplateName", segs[3])];
                self.handle_update_email_template(state, request, labels, query_map)
                    .await
            }
            // POST /v2/email/templates/{name}/render - TestRenderEmailTemplate
            ("POST", "templates", 5) if segs[4] == "render" => {
                let labels: &[(&str, &str)] = &[("TemplateName", segs[3])];
                self.handle_test_render_email_template(state, request, labels, query_map)
                    .await
            }
            // POST /v2/email/export-jobs - CreateExportJob
            ("POST", "export-jobs", 3) => {
                self.handle_create_export_job(state, request, &[], query_map)
                    .await
            }
            // GET /v2/email/export-jobs/{id} - GetExportJob
            ("GET", "export-jobs", 4) => {
                let labels: &[(&str, &str)] = &[("JobId", segs[3])];
                self.handle_get_export_job(state, request, labels, query_map)
                    .await
            }
            // POST /v2/email/list-export-jobs - ListExportJobs
            ("POST", "list-export-jobs", 3) => {
                self.handle_list_export_jobs(state, request, &[], query_map)
                    .await
            }
            // POST /v2/email/import-jobs - CreateImportJob
            ("POST", "import-jobs", 3) => {
                self.handle_create_import_job(state, request, &[], query_map)
                    .await
            }
            // GET /v2/email/import-jobs/{id} - GetImportJob
            ("GET", "import-jobs", 4) => {
                let labels: &[(&str, &str)] = &[("JobId", segs[3])];
                self.handle_get_import_job(state, request, labels, query_map)
                    .await
            }
            // POST /v2/email/import-jobs/list - ListImportJobs
            ("POST", "import-jobs", 4) if segs[3] == "list" => {
                self.handle_list_import_jobs(state, request, &[], query_map)
                    .await
            }
            // POST /v2/email/multi-region-endpoints - CreateMultiRegionEndpoint
            ("POST", "multi-region-endpoints", 3) => {
                self.handle_create_multi_region_endpoint(state, request, &[], query_map)
                    .await
            }
            // GET /v2/email/multi-region-endpoints - ListMultiRegionEndpoints
            ("GET", "multi-region-endpoints", 3) => {
                self.handle_list_multi_region_endpoints(state, request, &[], query_map)
                    .await
            }
            // GET /v2/email/multi-region-endpoints/{name} - GetMultiRegionEndpoint
            ("GET", "multi-region-endpoints", 4) => {
                let labels: &[(&str, &str)] = &[("EndpointName", segs[3])];
                self.handle_get_multi_region_endpoint(state, request, labels, query_map)
                    .await
            }
            // DELETE /v2/email/multi-region-endpoints/{name} - DeleteMultiRegionEndpoint
            ("DELETE", "multi-region-endpoints", 4) => {
                let labels: &[(&str, &str)] = &[("EndpointName", segs[3])];
                self.handle_delete_multi_region_endpoint(state, request, labels, query_map)
                    .await
            }
            // GET /v2/email/account - GetAccount
            ("GET", "account", 3) => {
                self.handle_get_account(state, request, &[], query_map)
                    .await
            }
            // POST /v2/email/account/details - PutAccountDetails
            ("POST", "account", 4) if segs[3] == "details" => {
                self.handle_put_account_details(state, request, &[], query_map)
                    .await
            }
            // PUT /v2/email/account/dedicated-ips/warmup - PutAccountDedicatedIpWarmupAttributes
            ("PUT", "account", 5) if segs[3] == "dedicated-ips" && segs[4] == "warmup" => {
                self.handle_put_account_dedicated_ip_warmup_attributes(
                    state,
                    request,
                    &[],
                    query_map,
                )
                .await
            }
            // PUT /v2/email/account/sending - PutAccountSendingAttributes
            ("PUT", "account", 4) if segs[3] == "sending" => {
                self.handle_put_account_sending_attributes(state, request, &[], query_map)
                    .await
            }
            // PUT /v2/email/account/suppression - PutAccountSuppressionAttributes
            ("PUT", "account", 4) if segs[3] == "suppression" => {
                self.handle_put_account_suppression_attributes(state, request, &[], query_map)
                    .await
            }
            // PUT /v2/email/account/vdm - PutAccountVdmAttributes
            ("PUT", "account", 4) if segs[3] == "vdm" => {
                self.handle_put_account_vdm_attributes(state, request, &[], query_map)
                    .await
            }
            // GET /v2/email/dedicated-ips - GetDedicatedIps
            ("GET", "dedicated-ips", 3) => {
                self.handle_get_dedicated_ips(state, request, &[], query_map)
                    .await
            }
            // GET /v2/email/dedicated-ips/{ip} - GetDedicatedIp
            ("GET", "dedicated-ips", 4) => {
                let labels: &[(&str, &str)] = &[("Ip", segs[3])];
                self.handle_get_dedicated_ip(state, request, labels, query_map)
                    .await
            }
            // PUT /v2/email/dedicated-ips/{ip}/pool - PutDedicatedIpInPool
            ("PUT", "dedicated-ips", 5) if segs[4] == "pool" => {
                let labels: &[(&str, &str)] = &[("Ip", segs[3])];
                self.handle_put_dedicated_ip_in_pool(state, request, labels, query_map)
                    .await
            }
            // PUT /v2/email/dedicated-ips/{ip}/warmup - PutDedicatedIpWarmupAttributes
            ("PUT", "dedicated-ips", 5) if segs[4] == "warmup" => {
                let labels: &[(&str, &str)] = &[("Ip", segs[3])];
                self.handle_put_dedicated_ip_warmup_attributes(state, request, labels, query_map)
                    .await
            }
            // PUT /v2/email/dedicated-ip-pools/{name}/scaling - PutDedicatedIpPoolScalingAttributes
            ("PUT", "dedicated-ip-pools", 5) if segs[4] == "scaling" => {
                let labels: &[(&str, &str)] = &[("PoolName", segs[3])];
                self.handle_put_dedicated_ip_pool_scaling_attributes(
                    state, request, labels, query_map,
                )
                .await
            }
            // PUT /v2/email/identities/{name}/configuration-set - PutEmailIdentityConfigurationSetAttributes
            ("PUT", "identities", 5) if segs[4] == "configuration-set" => {
                let labels: &[(&str, &str)] = &[("EmailIdentity", segs[3])];
                self.handle_put_email_identity_configuration_set_attributes(
                    state, request, labels, query_map,
                )
                .await
            }
            // PUT /v2/email/identities/{name}/dkim - PutEmailIdentityDkimAttributes
            ("PUT", "identities", 5) if segs[4] == "dkim" => {
                let labels: &[(&str, &str)] = &[("EmailIdentity", segs[3])];
                self.handle_put_email_identity_dkim_attributes(state, request, labels, query_map)
                    .await
            }
            // PUT /v2/email/identities/{name}/dkim/signing - PutEmailIdentityDkimSigningAttributes
            ("PUT", "identities", 6) if segs[4] == "dkim" && segs[5] == "signing" => {
                let labels: &[(&str, &str)] = &[("EmailIdentity", segs[3])];
                self.handle_put_email_identity_dkim_signing_attributes(
                    state, request, labels, query_map,
                )
                .await
            }
            // PUT /v2/email/identities/{name}/feedback - PutEmailIdentityFeedbackAttributes
            ("PUT", "identities", 5) if segs[4] == "feedback" => {
                let labels: &[(&str, &str)] = &[("EmailIdentity", segs[3])];
                self.handle_put_email_identity_feedback_attributes(
                    state, request, labels, query_map,
                )
                .await
            }
            // PUT /v2/email/identities/{name}/mail-from - PutEmailIdentityMailFromAttributes
            ("PUT", "identities", 5) if segs[4] == "mail-from" => {
                let labels: &[(&str, &str)] = &[("EmailIdentity", segs[3])];
                self.handle_put_email_identity_mail_from_attributes(
                    state, request, labels, query_map,
                )
                .await
            }
            // GET /v2/email/suppression/addresses/{email} - GetSuppressedDestination
            ("GET", "suppression", 5) if segs[3] == "addresses" => {
                let labels: &[(&str, &str)] = &[("EmailAddress", segs[4])];
                self.handle_get_suppressed_destination(state, request, labels, query_map)
                    .await
            }
            // DELETE /v2/email/suppression/addresses/{email} - DeleteSuppressedDestination
            ("DELETE", "suppression", 5) if segs[3] == "addresses" => {
                let labels: &[(&str, &str)] = &[("EmailAddress", segs[4])];
                self.handle_delete_suppressed_destination(state, request, labels, query_map)
                    .await
            }
            // GET /v2/email/suppression/addresses - ListSuppressedDestinations
            ("GET", "suppression", 4) if segs[3] == "addresses" => {
                self.handle_list_suppressed_destinations(state, request, &[], query_map)
                    .await
            }
            // PUT /v2/email/suppression/addresses - PutSuppressedDestination
            ("PUT", "suppression", 4) if segs[3] == "addresses" => {
                self.handle_put_suppressed_destination(state, request, &[], query_map)
                    .await
            }
            // POST /v2/email/tenants - CreateTenant
            ("POST", "tenants", 3) => {
                self.handle_create_tenant(state, request, &[], query_map)
                    .await
            }
            // POST /v2/email/tenants/delete - DeleteTenant
            ("POST", "tenants", 4) if segs[3] == "delete" => {
                self.handle_delete_tenant(state, request, &[], query_map)
                    .await
            }
            // POST /v2/email/tenants/get - GetTenant
            ("POST", "tenants", 4) if segs[3] == "get" => {
                self.handle_get_tenant(state, request, &[], query_map).await
            }
            // POST /v2/email/tenants/list - ListTenants
            ("POST", "tenants", 4) if segs[3] == "list" => {
                self.handle_list_tenants(state, request, &[], query_map)
                    .await
            }
            // POST /v2/email/tenants/resources - CreateTenantResourceAssociation
            ("POST", "tenants", 4) if segs[3] == "resources" => {
                self.handle_create_tenant_resource_association(state, request, &[], query_map)
                    .await
            }
            // POST /v2/email/tenants/resources/delete - DeleteTenantResourceAssociation
            ("POST", "tenants", 5) if segs[3] == "resources" && segs[4] == "delete" => {
                self.handle_delete_tenant_resource_association(state, request, &[], query_map)
                    .await
            }
            // POST /v2/email/tenants/resources/list - ListTenantResources
            ("POST", "tenants", 5) if segs[3] == "resources" && segs[4] == "list" => {
                self.handle_list_tenant_resources(state, request, &[], query_map)
                    .await
            }
            // POST /v2/email/resources/tenants/list - ListResourceTenants
            ("POST", "resources", 5) if segs[3] == "tenants" && segs[4] == "list" => {
                self.handle_list_resource_tenants(state, request, &[], query_map)
                    .await
            }
            // GET /v2/email/insights/{messageId} - GetMessageInsights
            ("GET", "insights", 4) => {
                let labels: &[(&str, &str)] = &[("MessageId", segs[3])];
                self.handle_get_message_insights(state, request, labels, query_map)
                    .await
            }
            // POST /v2/email/email-address-insights - GetEmailAddressInsights
            ("POST", "email-address-insights", 3) => {
                self.handle_get_email_address_insights(request, &[], query_map)
                    .await
            }
            // POST /v2/email/vdm/recommendations - ListRecommendations
            ("POST", "vdm", 4) if segs[3] == "recommendations" => {
                self.handle_list_recommendations().await
            }
            // POST /v2/email/reputation/entities - ListReputationEntities
            ("POST", "reputation", 4) if segs[3] == "entities" => {
                self.handle_list_reputation_entities(state, request, &[], query_map)
                    .await
            }
            // GET /v2/email/reputation/entities/{type}/{ref} - GetReputationEntity
            ("GET", "reputation", 6) if segs[3] == "entities" => {
                let labels: &[(&str, &str)] = &[
                    ("ReputationEntityType", segs[4]),
                    ("ReputationEntityReference", segs[5]),
                ];
                self.handle_get_reputation_entity(state, request, labels, query_map)
                    .await
            }
            // PUT /v2/email/reputation/entities/{type}/{ref}/customer-managed-status - UpdateReputationEntityCustomerManagedStatus
            ("PUT", "reputation", 7)
                if segs[3] == "entities" && segs[6] == "customer-managed-status" =>
            {
                let labels: &[(&str, &str)] = &[
                    ("ReputationEntityType", segs[4]),
                    ("ReputationEntityReference", segs[5]),
                ];
                self.handle_update_reputation_entity_customer_managed_status(
                    state, request, labels, query_map,
                )
                .await
            }
            // PUT /v2/email/reputation/entities/{type}/{ref}/policy - UpdateReputationEntityPolicy
            ("PUT", "reputation", 7) if segs[3] == "entities" && segs[6] == "policy" => {
                let labels: &[(&str, &str)] = &[
                    ("ReputationEntityType", segs[4]),
                    ("ReputationEntityReference", segs[5]),
                ];
                self.handle_update_reputation_entity_policy(state, request, labels, query_map)
                    .await
            }
            // POST /v2/email/outbound-bulk-emails - SendBulkEmail
            ("POST", "outbound-bulk-emails", 3) => {
                self.handle_send_bulk_email(state, request, &[], query_map)
                    .await
            }
            // POST /v2/email/outbound-custom-verification-emails - SendCustomVerificationEmail
            ("POST", "outbound-custom-verification-emails", 3) => {
                self.handle_send_custom_verification_email(state, request, &[], query_map)
                    .await
            }
            // PUT /v2/email/contact-lists/{name} - UpdateContactList
            ("PUT", "contact-lists", 4) => {
                let labels: &[(&str, &str)] = &[("ContactListName", segs[3])];
                self.handle_update_contact_list(state, request, labels, query_map)
                    .await
            }
            // PUT /v2/email/contact-lists/{name}/contacts/{email} - UpdateContact
            ("PUT", "contact-lists", 6) if segs[4] == "contacts" => {
                let labels: &[(&str, &str)] =
                    &[("ContactListName", segs[3]), ("EmailAddress", segs[5])];
                self.handle_update_contact(state, request, labels, query_map)
                    .await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        }
    }

    // --- Email Identity handlers ---

    async fn handle_create_email_identity(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_email_identity_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.email_identity.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing 'EmailIdentity'");
        }
        let tags = tags_from_input(input.tags.as_deref());
        let mut state = state.write().await;
        match state.create_email_identity(&input.email_identity, tags) {
            Ok(identity) => {
                wire::serialize_create_email_identity_response(&wire::CreateEmailIdentityResponse {
                    identity_type: Some(identity.identity_type.clone()),
                    verified_for_sending_status: Some(identity.verified),
                    ..Default::default()
                })
            }
            Err(e) => ses_error_response(&e),
        }
    }

    async fn handle_get_email_identity(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_email_identity_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_email_identity(&input.email_identity) {
            Ok(identity) => {
                wire::serialize_get_email_identity_response(&wire::GetEmailIdentityResponse {
                    identity_type: Some(identity.identity_type.clone()),
                    verified_for_sending_status: Some(identity.verified),
                    feedback_forwarding_status: Some(true),
                    tags: Some(tags_to_wire(&identity.tags)),
                    ..Default::default()
                })
            }
            Err(e) => ses_error_response(&e),
        }
    }

    async fn handle_delete_email_identity(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_email_identity_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.delete_email_identity(&input.email_identity) {
            Ok(()) => wire::serialize_delete_email_identity_response(
                &wire::DeleteEmailIdentityResponse {},
            ),
            Err(e) => ses_error_response(&e),
        }
    }

    async fn handle_list_email_identities(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_list_email_identities_request(request, labels, query) {
            return rest_json_error(400, "ValidationException", &e);
        }
        let state = state.read().await;
        let identities = state.list_email_identities();
        let entries: Vec<wire::IdentityInfo> = identities
            .iter()
            .map(|id| wire::IdentityInfo {
                identity_name: Some(id.name.clone()),
                identity_type: Some(id.identity_type.clone()),
                sending_enabled: Some(id.verified),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_email_identities_response(&wire::ListEmailIdentitiesResponse {
            email_identities: Some(entries),
            ..Default::default()
        })
    }

    async fn handle_get_email_identity_policies(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_get_email_identity_policies_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let state = state.read().await;
        match state.get_email_identity_policies(&input.email_identity) {
            Ok(policies) => wire::serialize_get_email_identity_policies_response(
                &wire::GetEmailIdentityPoliciesResponse {
                    policies: Some(policies.clone()),
                },
            ),
            Err(e) => ses_error_response(&e),
        }
    }

    async fn handle_create_email_identity_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_create_email_identity_policy_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        if input.policy.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing 'Policy'");
        }
        let mut state = state.write().await;
        match state.create_email_identity_policy(
            &input.email_identity,
            &input.policy_name,
            &input.policy,
        ) {
            Ok(()) => wire::serialize_create_email_identity_policy_response(
                &wire::CreateEmailIdentityPolicyResponse {},
            ),
            Err(e) => ses_error_response(&e),
        }
    }

    async fn handle_delete_email_identity_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_delete_email_identity_policy_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let mut state = state.write().await;
        match state.delete_email_identity_policy(&input.email_identity, &input.policy_name) {
            Ok(()) => wire::serialize_delete_email_identity_policy_response(
                &wire::DeleteEmailIdentityPolicyResponse {},
            ),
            Err(e) => ses_error_response(&e),
        }
    }

    async fn handle_update_email_identity_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_update_email_identity_policy_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        if input.policy.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing 'Policy'");
        }
        let mut state = state.write().await;
        match state.update_email_identity_policy(
            &input.email_identity,
            &input.policy_name,
            &input.policy,
        ) {
            Ok(()) => wire::serialize_update_email_identity_policy_response(
                &wire::UpdateEmailIdentityPolicyResponse {},
            ),
            Err(e) => ses_error_response(&e),
        }
    }

    // --- SendEmail handler ---

    async fn handle_send_email(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        // Pre-check for content presence (the Smithy model defaults Content,
        // so a missing `Content` field still parses — we want to reject it here
        // to match SES behaviour).
        let body_json: Option<Value> = if request.body.is_empty() {
            None
        } else {
            match serde_json::from_slice::<Value>(&request.body) {
                Ok(v) => Some(v),
                Err(_) => {
                    return rest_json_error(400, "BadRequestException", "Invalid JSON body");
                }
            }
        };

        let input = match wire::deserialize_send_email_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };

        let from = input.from_email_address.as_deref().unwrap_or("");
        if from.is_empty() {
            return rest_json_error(400, "BadRequestException", "FromEmailAddress is required");
        }

        let has_content = body_json
            .as_ref()
            .map(|b| b.get("Content").is_some())
            .unwrap_or(false);
        if !has_content {
            return rest_json_error(400, "BadRequestException", "Content is required");
        }

        let (to, cc, bcc) = match input.destination.as_ref() {
            Some(d) => (
                d.to_addresses.clone().unwrap_or_default(),
                d.cc_addresses.clone().unwrap_or_default(),
                d.bcc_addresses.clone().unwrap_or_default(),
            ),
            None => (Vec::new(), Vec::new(), Vec::new()),
        };

        if to.is_empty() && cc.is_empty() && bcc.is_empty() {
            return rest_json_error(
                400,
                "BadRequestException",
                "At least one destination address is required",
            );
        }

        let subject = input
            .content
            .simple
            .as_ref()
            .map(|s| s.subject.data.as_str())
            .unwrap_or("");

        let email_body = input
            .content
            .simple
            .as_ref()
            .and_then(|s| s.body.text.as_ref())
            .map(|c| c.data.as_str())
            .unwrap_or("");

        let mut state = state.write().await;
        match state.send_email(from, to, subject, email_body) {
            Ok(message_id) => wire::serialize_send_email_response(&wire::SendEmailResponse {
                message_id: Some(message_id),
            }),
            Err(e) => ses_error_response(&e),
        }
    }

    // --- Configuration Set handlers ---

    async fn handle_create_configuration_set(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_configuration_set_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.configuration_set_name.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing 'ConfigurationSetName'");
        }
        let tags = tags_from_input(input.tags.as_deref());
        let mut state = state.write().await;
        match state.create_configuration_set(&input.configuration_set_name, tags) {
            Ok(()) => wire::serialize_create_configuration_set_response(
                &wire::CreateConfigurationSetResponse {},
            ),
            Err(e) => ses_error_response(&e),
        }
    }

    async fn handle_get_configuration_set(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_configuration_set_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_configuration_set(&input.configuration_set_name) {
            Ok(cs) => {
                wire::serialize_get_configuration_set_response(&wire::GetConfigurationSetResponse {
                    configuration_set_name: Some(cs.name.clone()),
                    tags: Some(tags_to_wire(&cs.tags)),
                    ..Default::default()
                })
            }
            Err(e) => ses_error_response(&e),
        }
    }

    async fn handle_delete_configuration_set(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_configuration_set_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.delete_configuration_set(&input.configuration_set_name) {
            Ok(()) => wire::serialize_delete_configuration_set_response(
                &wire::DeleteConfigurationSetResponse {},
            ),
            Err(e) => ses_error_response(&e),
        }
    }

    async fn handle_list_configuration_sets(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_list_configuration_sets_request(request, labels, query) {
            return rest_json_error(400, "ValidationException", &e);
        }
        let state = state.read().await;
        let sets = state.list_configuration_sets();
        let names: Vec<String> = sets.iter().map(|cs| cs.name.clone()).collect();
        wire::serialize_list_configuration_sets_response(&wire::ListConfigurationSetsResponse {
            configuration_sets: Some(names),
            ..Default::default()
        })
    }

    // --- Contact List handlers ---

    async fn handle_create_contact_list(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_contact_list_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.contact_list_name.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing 'ContactListName'");
        }
        let tags = tags_from_input(input.tags.as_deref());
        let mut state = state.write().await;
        match state.create_contact_list(
            &input.contact_list_name,
            input.description.as_deref(),
            tags,
        ) {
            Ok(()) => {
                wire::serialize_create_contact_list_response(&wire::CreateContactListResponse {})
            }
            Err(e) => ses_error_response(&e),
        }
    }

    async fn handle_get_contact_list(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_contact_list_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_contact_list(&input.contact_list_name) {
            Ok(cl) => wire::serialize_get_contact_list_response(&wire::GetContactListResponse {
                contact_list_name: Some(cl.name.clone()),
                description: cl.description.clone(),
                tags: Some(tags_to_wire(&cl.tags)),
                created_timestamp: Some(cl.created_timestamp.timestamp() as f64),
                last_updated_timestamp: Some(cl.last_updated_timestamp.timestamp() as f64),
                ..Default::default()
            }),
            Err(e) => ses_error_response(&e),
        }
    }

    async fn handle_delete_contact_list(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_contact_list_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.delete_contact_list(&input.contact_list_name) {
            Ok(()) => {
                wire::serialize_delete_contact_list_response(&wire::DeleteContactListResponse {})
            }
            Err(e) => ses_error_response(&e),
        }
    }

    async fn handle_list_contact_lists(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_list_contact_lists_request(request, labels, query) {
            return rest_json_error(400, "ValidationException", &e);
        }
        let state = state.read().await;
        let lists = state.list_contact_lists();
        let entries: Vec<wire::ContactList> = lists
            .iter()
            .map(|cl| wire::ContactList {
                contact_list_name: Some(cl.name.clone()),
                last_updated_timestamp: Some(cl.last_updated_timestamp.timestamp() as f64),
            })
            .collect();
        wire::serialize_list_contact_lists_response(&wire::ListContactListsResponse {
            contact_lists: Some(entries),
            ..Default::default()
        })
    }

    // --- Contact handlers ---

    async fn handle_create_contact(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_contact_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.email_address.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing 'EmailAddress'");
        }
        let unsubscribe_all = input.unsubscribe_all.unwrap_or(false);
        let topic_preferences: Vec<TopicPreference> = input
            .topic_preferences
            .unwrap_or_default()
            .into_iter()
            .map(|tp| TopicPreference {
                topic_name: tp.topic_name,
                subscription_status: tp.subscription_status,
            })
            .collect();
        let mut state = state.write().await;
        match state.create_contact(
            &input.contact_list_name,
            &input.email_address,
            unsubscribe_all,
            topic_preferences,
        ) {
            Ok(()) => wire::serialize_create_contact_response(&wire::CreateContactResponse {}),
            Err(e) => ses_error_response(&e),
        }
    }

    async fn handle_get_contact(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_contact_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_contact(&input.contact_list_name, &input.email_address) {
            Ok(contact) => {
                let prefs: Vec<wire::TopicPreference> = contact
                    .topic_preferences
                    .iter()
                    .map(|tp| wire::TopicPreference {
                        topic_name: tp.topic_name.clone(),
                        subscription_status: tp.subscription_status.clone(),
                    })
                    .collect();
                wire::serialize_get_contact_response(&wire::GetContactResponse {
                    email_address: Some(contact.email_address.clone()),
                    topic_preferences: Some(prefs),
                    unsubscribe_all: Some(contact.unsubscribe_all),
                    created_timestamp: Some(contact.created_timestamp.timestamp() as f64),
                    last_updated_timestamp: Some(contact.last_updated_timestamp.timestamp() as f64),
                    ..Default::default()
                })
            }
            Err(e) => ses_error_response(&e),
        }
    }

    async fn handle_delete_contact(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_contact_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.delete_contact(&input.contact_list_name, &input.email_address) {
            Ok(()) => wire::serialize_delete_contact_response(&wire::DeleteContactResponse {}),
            Err(e) => ses_error_response(&e),
        }
    }

    async fn handle_list_contacts(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_contacts_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.list_contacts(&input.contact_list_name) {
            Ok(contacts) => {
                let entries: Vec<wire::Contact> = contacts
                    .iter()
                    .map(|c| wire::Contact {
                        email_address: Some(c.email_address.clone()),
                        unsubscribe_all: Some(c.unsubscribe_all),
                        ..Default::default()
                    })
                    .collect();
                wire::serialize_list_contacts_response(&wire::ListContactsResponse {
                    contacts: Some(entries),
                    ..Default::default()
                })
            }
            Err(e) => ses_error_response(&e),
        }
    }

    // --- Dedicated IP Pool handlers ---

    async fn handle_create_dedicated_ip_pool(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_dedicated_ip_pool_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.pool_name.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing 'PoolName'");
        }
        let scaling_mode = input.scaling_mode.as_deref().unwrap_or("STANDARD");
        let tags = tags_from_input(input.tags.as_deref());
        let mut state = state.write().await;
        match state.create_dedicated_ip_pool(&input.pool_name, scaling_mode, tags) {
            Ok(()) => wire::serialize_create_dedicated_ip_pool_response(
                &wire::CreateDedicatedIpPoolResponse {},
            ),
            Err(e) => ses_error_response(&e),
        }
    }

    async fn handle_get_dedicated_ip_pool(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_dedicated_ip_pool_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_dedicated_ip_pool(&input.pool_name) {
            Ok(pool) => {
                wire::serialize_get_dedicated_ip_pool_response(&wire::GetDedicatedIpPoolResponse {
                    dedicated_ip_pool: Some(wire::DedicatedIpPool {
                        pool_name: Some(pool.pool_name.clone()),
                        scaling_mode: Some(pool.scaling_mode.clone()),
                    }),
                })
            }
            Err(e) => ses_error_response(&e),
        }
    }

    async fn handle_delete_dedicated_ip_pool(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_dedicated_ip_pool_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.delete_dedicated_ip_pool(&input.pool_name) {
            Ok(()) => wire::serialize_delete_dedicated_ip_pool_response(
                &wire::DeleteDedicatedIpPoolResponse {},
            ),
            Err(e) => ses_error_response(&e),
        }
    }

    async fn handle_list_dedicated_ip_pools(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_list_dedicated_ip_pools_request(request, labels, query) {
            return rest_json_error(400, "ValidationException", &e);
        }
        let state = state.read().await;
        let pools = state.list_dedicated_ip_pools();
        let names: Vec<String> = pools.iter().map(|p| p.pool_name.clone()).collect();
        wire::serialize_list_dedicated_ip_pools_response(&wire::ListDedicatedIpPoolsResponse {
            dedicated_ip_pools: Some(names),
            ..Default::default()
        })
    }

    // --- Tag handlers ---

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing 'ResourceArn'");
        }
        let state = state.read().await;
        let tags = state.list_tags_for_resource(&input.resource_arn);
        let tag_list: Vec<wire::Tag> = tags
            .iter()
            .map(|(k, v)| wire::Tag {
                key: k.clone(),
                value: v.clone(),
            })
            .collect();
        wire::serialize_list_tags_for_resource_response(&wire::ListTagsForResourceResponse {
            tags: Some(tag_list),
        })
    }

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing 'ResourceArn'");
        }
        let tags: HashMap<String, String> =
            input.tags.into_iter().map(|t| (t.key, t.value)).collect();
        let mut state = state.write().await;
        match state.tag_resource(&input.resource_arn, tags) {
            Ok(()) => wire::serialize_tag_resource_response(&wire::TagResourceResponse {}),
            Err(e) => ses_error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        raw_query: &str,
    ) -> MockResponse {
        // AWS SDKs serialize the `TagKeys` httpQuery as repeated params
        // (?TagKeys=a&TagKeys=b), but `parse_query_string` collapses repeated
        // keys to the last value. Re-parse from the raw query string so multi-key
        // untag requests are not silently truncated.
        let input = match wire::deserialize_untag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing 'ResourceArn'");
        }
        let multi_keys = parse_tag_keys_from_query(raw_query);
        let tag_keys = if multi_keys.is_empty() {
            input.tag_keys
        } else {
            multi_keys
        };
        let mut state = state.write().await;
        match state.untag_resource(&input.resource_arn, &tag_keys) {
            Ok(()) => wire::serialize_untag_resource_response(&wire::UntagResourceResponse {}),
            Err(e) => ses_error_response(&e),
        }
    }

    // --- Previously-stub handlers, now with real implementations ---

    // STUB[no-telemetry]: SES metrics are driven by real sending infrastructure; the mock
    //   returns empty result sets per query with no timestamps or values.
    async fn handle_batch_get_metric_data(
        &self,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_batch_get_metric_data_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let results: Vec<wire::MetricDataResult> = input
            .queries
            .iter()
            .map(|q| wire::MetricDataResult {
                id: Some(q.id.clone()),
                timestamps: Some(vec![]),
                values: Some(vec![]),
            })
            .collect();
        wire::serialize_batch_get_metric_data_response(&wire::BatchGetMetricDataResponse {
            results: Some(results),
            errors: Some(vec![]),
        })
    }

    async fn handle_cancel_export_job(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_cancel_export_job_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.cancel_export_job(&input.job_id) {
            Ok(()) => wire::serialize_cancel_export_job_response(&wire::CancelExportJobResponse {}),
            Err(e) => ses_error_response(&e),
        }
    }

    async fn handle_create_configuration_set_event_destination(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_configuration_set_event_destination_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.event_destination_name.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing 'EventDestinationName'");
        }
        let enabled = input.event_destination.enabled.unwrap_or(true);
        let matching_event_types = input
            .event_destination
            .matching_event_types
            .clone()
            .unwrap_or_default();
        let destination = serde_json::to_value(&input.event_destination).unwrap_or(Value::Null);
        let mut state = state.write().await;
        match state.create_configuration_set_event_destination(
            &input.configuration_set_name,
            &input.event_destination_name,
            enabled,
            matching_event_types,
            destination,
        ) {
            Ok(()) => wire::serialize_create_configuration_set_event_destination_response(
                &wire::CreateConfigurationSetEventDestinationResponse {},
            ),
            Err(e) => ses_error_response(&e),
        }
    }

    async fn handle_get_configuration_set_event_destinations(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_configuration_set_event_destinations_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_configuration_set_event_destinations(&input.configuration_set_name) {
            Ok(dests) => {
                let dest_list: Vec<wire::EventDestination> = dests
                    .iter()
                    .map(|d| wire::EventDestination {
                        name: Some(d.name.clone()),
                        enabled: Some(d.enabled),
                        matching_event_types: Some(d.matching_event_types.clone()),
                        ..Default::default()
                    })
                    .collect();
                wire::serialize_get_configuration_set_event_destinations_response(
                    &wire::GetConfigurationSetEventDestinationsResponse {
                        event_destinations: Some(dest_list),
                    },
                )
            }
            Err(e) => ses_error_response(&e),
        }
    }

    async fn handle_delete_configuration_set_event_destination(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_configuration_set_event_destination_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.delete_configuration_set_event_destination(
            &input.configuration_set_name,
            &input.event_destination_name,
        ) {
            Ok(()) => wire::serialize_delete_configuration_set_event_destination_response(
                &wire::DeleteConfigurationSetEventDestinationResponse {},
            ),
            Err(e) => ses_error_response(&e),
        }
    }

    async fn handle_update_configuration_set_event_destination(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_configuration_set_event_destination_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let enabled = input.event_destination.enabled.unwrap_or(true);
        let matching_event_types = input
            .event_destination
            .matching_event_types
            .clone()
            .unwrap_or_default();
        let destination = serde_json::to_value(&input.event_destination).unwrap_or(Value::Null);
        let mut state = state.write().await;
        match state.update_configuration_set_event_destination(
            &input.configuration_set_name,
            &input.event_destination_name,
            enabled,
            matching_event_types,
            destination,
        ) {
            Ok(()) => wire::serialize_update_configuration_set_event_destination_response(
                &wire::UpdateConfigurationSetEventDestinationResponse {},
            ),
            Err(e) => ses_error_response(&e),
        }
    }

    async fn handle_put_configuration_set_archiving_options(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_put_configuration_set_archiving_options_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let value = serde_json::to_value(&input).unwrap_or(Value::Null);
        let mut state = state.write().await;
        match state.put_configuration_set_option(
            &input.configuration_set_name,
            "archiving-options",
            value,
        ) {
            Ok(()) => wire::serialize_put_configuration_set_archiving_options_response(
                &wire::PutConfigurationSetArchivingOptionsResponse {},
            ),
            Err(e) => ses_error_response(&e),
        }
    }

    async fn handle_put_configuration_set_delivery_options(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_put_configuration_set_delivery_options_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let value = serde_json::to_value(&input).unwrap_or(Value::Null);
        let mut state = state.write().await;
        match state.put_configuration_set_option(
            &input.configuration_set_name,
            "delivery-options",
            value,
        ) {
            Ok(()) => wire::serialize_put_configuration_set_delivery_options_response(
                &wire::PutConfigurationSetDeliveryOptionsResponse {},
            ),
            Err(e) => ses_error_response(&e),
        }
    }

    async fn handle_put_configuration_set_reputation_options(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_put_configuration_set_reputation_options_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let value = serde_json::to_value(&input).unwrap_or(Value::Null);
        let mut state = state.write().await;
        match state.put_configuration_set_option(
            &input.configuration_set_name,
            "reputation-options",
            value,
        ) {
            Ok(()) => wire::serialize_put_configuration_set_reputation_options_response(
                &wire::PutConfigurationSetReputationOptionsResponse {},
            ),
            Err(e) => ses_error_response(&e),
        }
    }

    async fn handle_put_configuration_set_sending_options(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_put_configuration_set_sending_options_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let value = serde_json::to_value(&input).unwrap_or(Value::Null);
        let mut state = state.write().await;
        match state.put_configuration_set_option(&input.configuration_set_name, "sending", value) {
            Ok(()) => wire::serialize_put_configuration_set_sending_options_response(
                &wire::PutConfigurationSetSendingOptionsResponse {},
            ),
            Err(e) => ses_error_response(&e),
        }
    }

    async fn handle_put_configuration_set_suppression_options(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_put_configuration_set_suppression_options_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let value = serde_json::to_value(&input).unwrap_or(Value::Null);
        let mut state = state.write().await;
        match state.put_configuration_set_option(
            &input.configuration_set_name,
            "suppression-options",
            value,
        ) {
            Ok(()) => wire::serialize_put_configuration_set_suppression_options_response(
                &wire::PutConfigurationSetSuppressionOptionsResponse {},
            ),
            Err(e) => ses_error_response(&e),
        }
    }

    async fn handle_put_configuration_set_tracking_options(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_put_configuration_set_tracking_options_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let value = serde_json::to_value(&input).unwrap_or(Value::Null);
        let mut state = state.write().await;
        match state.put_configuration_set_option(
            &input.configuration_set_name,
            "tracking-options",
            value,
        ) {
            Ok(()) => wire::serialize_put_configuration_set_tracking_options_response(
                &wire::PutConfigurationSetTrackingOptionsResponse {},
            ),
            Err(e) => ses_error_response(&e),
        }
    }

    async fn handle_put_configuration_set_vdm_options(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_put_configuration_set_vdm_options_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let value = serde_json::to_value(&input).unwrap_or(Value::Null);
        let mut state = state.write().await;
        match state.put_configuration_set_option(
            &input.configuration_set_name,
            "vdm-options",
            value,
        ) {
            Ok(()) => wire::serialize_put_configuration_set_vdm_options_response(
                &wire::PutConfigurationSetVdmOptionsResponse {},
            ),
            Err(e) => ses_error_response(&e),
        }
    }

    async fn handle_create_custom_verification_email_template(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_custom_verification_email_template_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.template_name.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing 'TemplateName'");
        }
        let mut state = state.write().await;
        match state.create_custom_verification_email_template(
            &input.template_name,
            &input.from_email_address,
            &input.template_subject,
            &input.template_content,
            &input.success_redirection_u_r_l,
            &input.failure_redirection_u_r_l,
        ) {
            Ok(()) => wire::serialize_create_custom_verification_email_template_response(
                &wire::CreateCustomVerificationEmailTemplateResponse {},
            ),
            Err(e) => ses_error_response(&e),
        }
    }

    async fn handle_list_custom_verification_email_templates(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_list_custom_verification_email_templates_request(
            request, labels, query,
        ) {
            return rest_json_error(400, "ValidationException", &e);
        }
        let state = state.read().await;
        let templates = state.list_custom_verification_email_templates();
        let list: Vec<wire::CustomVerificationEmailTemplateMetadata> = templates
            .iter()
            .map(|t| wire::CustomVerificationEmailTemplateMetadata {
                template_name: Some(t.template_name.clone()),
                from_email_address: Some(t.from_email_address.clone()),
                template_subject: Some(t.template_subject.clone()),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_custom_verification_email_templates_response(
            &wire::ListCustomVerificationEmailTemplatesResponse {
                custom_verification_email_templates: Some(list),
                ..Default::default()
            },
        )
    }

    async fn handle_get_custom_verification_email_template(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_custom_verification_email_template_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_custom_verification_email_template(&input.template_name) {
            Ok(t) => wire::serialize_get_custom_verification_email_template_response(
                &wire::GetCustomVerificationEmailTemplateResponse {
                    template_name: Some(t.template_name.clone()),
                    from_email_address: Some(t.from_email_address.clone()),
                    template_subject: Some(t.template_subject.clone()),
                    template_content: Some(t.template_content.clone()),
                    success_redirection_u_r_l: Some(t.success_redirection_url.clone()),
                    failure_redirection_u_r_l: Some(t.failure_redirection_url.clone()),
                    ..Default::default()
                },
            ),
            Err(e) => ses_error_response(&e),
        }
    }

    async fn handle_delete_custom_verification_email_template(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_custom_verification_email_template_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.delete_custom_verification_email_template(&input.template_name) {
            Ok(()) => wire::serialize_delete_custom_verification_email_template_response(
                &wire::DeleteCustomVerificationEmailTemplateResponse {},
            ),
            Err(e) => ses_error_response(&e),
        }
    }

    async fn handle_update_custom_verification_email_template(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        // The update operation on AWS treats missing fields as "do not change".
        // The Smithy model marks all fields as required so they default to "".
        // Detect "field absent" by inspecting the raw body (when present).
        let body_json: Option<Value> = if request.body.is_empty() {
            None
        } else {
            serde_json::from_slice(&request.body).ok()
        };
        let input = match wire::deserialize_update_custom_verification_email_template_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let from_email_address = body_json
            .as_ref()
            .and_then(|b| b.get("FromEmailAddress"))
            .and_then(|v| v.as_str())
            .map(|_| input.from_email_address.as_str());
        let template_subject = body_json
            .as_ref()
            .and_then(|b| b.get("TemplateSubject"))
            .and_then(|v| v.as_str())
            .map(|_| input.template_subject.as_str());
        let template_content = body_json
            .as_ref()
            .and_then(|b| b.get("TemplateContent"))
            .and_then(|v| v.as_str())
            .map(|_| input.template_content.as_str());
        let success_url = body_json
            .as_ref()
            .and_then(|b| b.get("SuccessRedirectionURL"))
            .and_then(|v| v.as_str())
            .map(|_| input.success_redirection_u_r_l.as_str());
        let failure_url = body_json
            .as_ref()
            .and_then(|b| b.get("FailureRedirectionURL"))
            .and_then(|v| v.as_str())
            .map(|_| input.failure_redirection_u_r_l.as_str());
        let mut state = state.write().await;
        match state.update_custom_verification_email_template(
            &input.template_name,
            from_email_address,
            template_subject,
            template_content,
            success_url,
            failure_url,
        ) {
            Ok(()) => wire::serialize_update_custom_verification_email_template_response(
                &wire::UpdateCustomVerificationEmailTemplateResponse {},
            ),
            Err(e) => ses_error_response(&e),
        }
    }

    async fn handle_create_deliverability_test_report(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_deliverability_test_report_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.from_email_address.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing 'FromEmailAddress'");
        }
        let mut state = state.write().await;
        let report_id = state.create_deliverability_test_report(
            &input.from_email_address,
            input.report_name.as_deref(),
        );
        wire::serialize_create_deliverability_test_report_response(
            &wire::CreateDeliverabilityTestReportResponse {
                report_id: Some(report_id),
                deliverability_test_status: Some("IN_PROGRESS".to_string()),
            },
        )
    }

    async fn handle_get_deliverability_dashboard_options(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) =
            wire::deserialize_get_deliverability_dashboard_options_request(request, labels, query)
        {
            return rest_json_error(400, "ValidationException", &e);
        }
        let state = state.read().await;
        let opts = &state.deliverability_dashboard;
        wire::serialize_get_deliverability_dashboard_options_response(
            &wire::GetDeliverabilityDashboardOptionsResponse {
                dashboard_enabled: Some(opts.dashboard_enabled),
                subscription_expiry_date: opts.subscription_expiry_date,
                account_status: Some(opts.account_status.clone()),
                active_subscribed_domains: Some(vec![]),
                pending_expiration_subscribed_domains: Some(vec![]),
            },
        )
    }

    async fn handle_put_deliverability_dashboard_option(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_put_deliverability_dashboard_option_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        state.deliverability_dashboard.dashboard_enabled = input.dashboard_enabled;
        state.deliverability_dashboard.account_status = if input.dashboard_enabled {
            "ENABLED".to_string()
        } else {
            "DISABLED".to_string()
        };
        wire::serialize_put_deliverability_dashboard_option_response(
            &wire::PutDeliverabilityDashboardOptionResponse {},
        )
    }

    async fn handle_get_blacklist_reports(&self) -> MockResponse {
        wire::serialize_get_blacklist_reports_response(&wire::GetBlacklistReportsResponse {
            blacklist_report: Some(std::collections::HashMap::new()),
        })
    }

    async fn handle_list_deliverability_test_reports(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) =
            wire::deserialize_list_deliverability_test_reports_request(request, labels, query)
        {
            return rest_json_error(400, "ValidationException", &e);
        }
        let state = state.read().await;
        let reports = state.list_deliverability_test_reports();
        let list: Vec<wire::DeliverabilityTestReport> = reports
            .iter()
            .map(|r| wire::DeliverabilityTestReport {
                report_id: Some(r.report_id.clone()),
                report_name: r.report_name.clone(),
                from_email_address: Some(r.from_email_address.clone()),
                create_date: Some(r.create_date),
                deliverability_test_status: Some(r.deliverability_test_status.clone()),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_deliverability_test_reports_response(
            &wire::ListDeliverabilityTestReportsResponse {
                deliverability_test_reports: Some(list),
                ..Default::default()
            },
        )
    }

    async fn handle_get_deliverability_test_report(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_deliverability_test_report_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_deliverability_test_report(&input.report_id) {
            Ok(r) => wire::serialize_get_deliverability_test_report_response(
                &wire::GetDeliverabilityTestReportResponse {
                    deliverability_test_report: Some(wire::DeliverabilityTestReport {
                        report_id: Some(r.report_id.clone()),
                        report_name: r.report_name.clone(),
                        from_email_address: Some(r.from_email_address.clone()),
                        create_date: Some(r.create_date),
                        deliverability_test_status: Some(r.deliverability_test_status.clone()),
                        ..Default::default()
                    }),
                    overall_placement: Some(wire::PlacementStatistics {
                        inbox_percentage: Some(0.0),
                        spam_percentage: Some(0.0),
                        missing_percentage: Some(0.0),
                        spf_percentage: Some(0.0),
                        dkim_percentage: Some(0.0),
                    }),
                    isp_placements: Some(vec![]),
                    message: None,
                    tags: Some(vec![]),
                },
            ),
            Err(e) => ses_error_response(&e),
        }
    }

    async fn handle_get_domain_deliverability_campaign(&self) -> MockResponse {
        wire::serialize_get_domain_deliverability_campaign_response(
            &wire::GetDomainDeliverabilityCampaignResponse {
                ..Default::default()
            },
        )
    }

    async fn handle_get_domain_statistics_report(&self) -> MockResponse {
        wire::serialize_get_domain_statistics_report_response(
            &wire::GetDomainStatisticsReportResponse {
                ..Default::default()
            },
        )
    }

    async fn handle_list_domain_deliverability_campaigns(&self) -> MockResponse {
        wire::serialize_list_domain_deliverability_campaigns_response(
            &wire::ListDomainDeliverabilityCampaignsResponse {
                domain_deliverability_campaigns: Some(vec![]),
                ..Default::default()
            },
        )
    }

    async fn handle_create_email_template(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_email_template_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.template_name.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing 'TemplateName'");
        }
        let mut state = state.write().await;
        match state.create_email_template(
            &input.template_name,
            input.template_content.subject.as_deref(),
            input.template_content.text.as_deref(),
            input.template_content.html.as_deref(),
        ) {
            Ok(()) => wire::serialize_create_email_template_response(
                &wire::CreateEmailTemplateResponse {},
            ),
            Err(e) => ses_error_response(&e),
        }
    }

    async fn handle_list_email_templates(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_list_email_templates_request(request, labels, query) {
            return rest_json_error(400, "ValidationException", &e);
        }
        let state = state.read().await;
        let templates = state.list_email_templates();
        let metadata: Vec<wire::EmailTemplateMetadata> = templates
            .iter()
            .map(|t| wire::EmailTemplateMetadata {
                template_name: Some(t.template_name.clone()),
                created_timestamp: Some(t.created_timestamp.timestamp_millis() as f64 / 1000.0),
            })
            .collect();
        wire::serialize_list_email_templates_response(&wire::ListEmailTemplatesResponse {
            templates_metadata: Some(metadata),
            ..Default::default()
        })
    }

    async fn handle_get_email_template(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_email_template_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_email_template(&input.template_name) {
            Ok(t) => wire::serialize_get_email_template_response(&wire::GetEmailTemplateResponse {
                template_name: Some(t.template_name.clone()),
                template_content: Some(wire::EmailTemplateContent {
                    subject: t.subject_part.clone(),
                    text: t.text_part.clone(),
                    html: t.html_part.clone(),
                }),
                ..Default::default()
            }),
            Err(e) => ses_error_response(&e),
        }
    }

    async fn handle_delete_email_template(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_email_template_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.delete_email_template(&input.template_name) {
            Ok(()) => wire::serialize_delete_email_template_response(
                &wire::DeleteEmailTemplateResponse {},
            ),
            Err(e) => ses_error_response(&e),
        }
    }

    async fn handle_update_email_template(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_email_template_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.update_email_template(
            &input.template_name,
            input.template_content.subject.as_deref(),
            input.template_content.text.as_deref(),
            input.template_content.html.as_deref(),
        ) {
            Ok(()) => wire::serialize_update_email_template_response(
                &wire::UpdateEmailTemplateResponse {},
            ),
            Err(e) => ses_error_response(&e),
        }
    }

    async fn handle_test_render_email_template(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_test_render_email_template_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let template_data = if input.template_data.is_empty() {
            "{}"
        } else {
            input.template_data.as_str()
        };
        let state = state.read().await;
        match state.test_render_email_template(&input.template_name, template_data) {
            Ok(rendered) => wire::serialize_test_render_email_template_response(
                &wire::TestRenderEmailTemplateResponse {
                    rendered_template: Some(rendered),
                },
            ),
            Err(e) => ses_error_response(&e),
        }
    }

    async fn handle_create_export_job(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_export_job_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        // Determine export source type from data source
        let export_source_type = if input.export_data_source.metrics_data_source.is_some() {
            "METRICS_DATA"
        } else if input
            .export_data_source
            .message_insights_data_source
            .is_some()
        {
            "MESSAGE_INSIGHTS"
        } else {
            "METRICS_DATA"
        };
        let export_destination =
            serde_json::to_value(&input.export_destination).unwrap_or(Value::Null);
        let mut state = state.write().await;
        let job_id = state.create_export_job(export_source_type, export_destination);
        wire::serialize_create_export_job_response(&wire::CreateExportJobResponse {
            job_id: Some(job_id),
        })
    }

    async fn handle_get_export_job(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_export_job_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_export_job(&input.job_id) {
            Ok(job) => wire::serialize_get_export_job_response(&wire::GetExportJobResponse {
                job_id: Some(job.job_id.clone()),
                export_source_type: Some(job.export_source_type.clone()),
                job_status: Some(job.job_status.clone()),
                created_timestamp: Some(job.created_timestamp.timestamp_millis() as f64 / 1000.0),
                export_destination: serde_json::from_value(job.export_destination.clone()).ok(),
                ..Default::default()
            }),
            Err(e) => ses_error_response(&e),
        }
    }

    async fn handle_list_export_jobs(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_list_export_jobs_request(request, labels, query) {
            return rest_json_error(400, "ValidationException", &e);
        }
        let state = state.read().await;
        let jobs = state.list_export_jobs();
        let list: Vec<wire::ExportJobSummary> = jobs
            .iter()
            .map(|j| wire::ExportJobSummary {
                job_id: Some(j.job_id.clone()),
                export_source_type: Some(j.export_source_type.clone()),
                job_status: Some(j.job_status.clone()),
                created_timestamp: Some(j.created_timestamp.timestamp_millis() as f64 / 1000.0),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_export_jobs_response(&wire::ListExportJobsResponse {
            export_jobs: Some(list),
            ..Default::default()
        })
    }

    async fn handle_create_import_job(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_import_job_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let import_destination =
            serde_json::to_value(&input.import_destination).unwrap_or(Value::Null);
        let import_data_source =
            serde_json::to_value(&input.import_data_source).unwrap_or(Value::Null);
        let mut state = state.write().await;
        let job_id = state.create_import_job(import_destination, import_data_source);
        wire::serialize_create_import_job_response(&wire::CreateImportJobResponse {
            job_id: Some(job_id),
        })
    }

    async fn handle_get_import_job(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_import_job_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_import_job(&input.job_id) {
            Ok(job) => wire::serialize_get_import_job_response(&wire::GetImportJobResponse {
                job_id: Some(job.job_id.clone()),
                import_destination: serde_json::from_value(job.import_destination.clone()).ok(),
                import_data_source: serde_json::from_value(job.import_data_source.clone()).ok(),
                job_status: Some(job.job_status.clone()),
                created_timestamp: Some(job.created_timestamp.timestamp_millis() as f64 / 1000.0),
                ..Default::default()
            }),
            Err(e) => ses_error_response(&e),
        }
    }

    async fn handle_list_import_jobs(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_list_import_jobs_request(request, labels, query) {
            return rest_json_error(400, "ValidationException", &e);
        }
        let state = state.read().await;
        let jobs = state.list_import_jobs();
        let job_list: Vec<wire::ImportJobSummary> = jobs
            .iter()
            .map(|j| wire::ImportJobSummary {
                job_id: Some(j.job_id.clone()),
                import_destination: serde_json::from_value(j.import_destination.clone()).ok(),
                job_status: Some(j.job_status.clone()),
                created_timestamp: Some(j.created_timestamp.timestamp_millis() as f64 / 1000.0),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_import_jobs_response(&wire::ListImportJobsResponse {
            import_jobs: Some(job_list),
            ..Default::default()
        })
    }

    async fn handle_create_multi_region_endpoint(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_create_multi_region_endpoint_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        if input.endpoint_name.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing 'EndpointName'");
        }
        let regions: Vec<String> = input
            .details
            .routes_details
            .iter()
            .map(|r| r.region.clone())
            .collect();
        let mut state = state.write().await;
        let endpoint_id = state.create_multi_region_endpoint(&input.endpoint_name, regions);
        wire::serialize_create_multi_region_endpoint_response(
            &wire::CreateMultiRegionEndpointResponse {
                endpoint_id: Some(endpoint_id),
                status: None,
            },
        )
    }

    async fn handle_list_multi_region_endpoints(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) =
            wire::deserialize_list_multi_region_endpoints_request(request, labels, query)
        {
            return rest_json_error(400, "ValidationException", &e);
        }
        let state = state.read().await;
        let endpoints = state.list_multi_region_endpoints();
        let list: Vec<wire::MultiRegionEndpoint> = endpoints
            .iter()
            .map(|ep| wire::MultiRegionEndpoint {
                endpoint_name: Some(ep.endpoint_name.clone()),
                endpoint_id: Some(ep.endpoint_id.clone()),
                status: Some(ep.status.clone()),
                regions: Some(ep.regions.clone()),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_multi_region_endpoints_response(
            &wire::ListMultiRegionEndpointsResponse {
                multi_region_endpoints: Some(list),
                ..Default::default()
            },
        )
    }

    async fn handle_get_multi_region_endpoint(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_get_multi_region_endpoint_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let state = state.read().await;
        match state.get_multi_region_endpoint(&input.endpoint_name) {
            Ok(ep) => {
                let routes: Vec<wire::Route> = ep
                    .regions
                    .iter()
                    .map(|r| wire::Route {
                        region: Some(r.clone()),
                    })
                    .collect();
                wire::serialize_get_multi_region_endpoint_response(
                    &wire::GetMultiRegionEndpointResponse {
                        endpoint_name: Some(ep.endpoint_name.clone()),
                        endpoint_id: Some(ep.endpoint_id.clone()),
                        status: Some(ep.status.clone()),
                        routes: Some(routes),
                        created_timestamp: Some(
                            ep.created_timestamp.timestamp_millis() as f64 / 1000.0,
                        ),
                        last_updated_timestamp: Some(
                            ep.last_updated_timestamp.timestamp_millis() as f64 / 1000.0,
                        ),
                    },
                )
            }
            Err(e) => ses_error_response(&e),
        }
    }

    async fn handle_delete_multi_region_endpoint(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_delete_multi_region_endpoint_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let mut state = state.write().await;
        match state.delete_multi_region_endpoint(&input.endpoint_name) {
            Ok(()) => wire::serialize_delete_multi_region_endpoint_response(
                &wire::DeleteMultiRegionEndpointResponse {
                    ..Default::default()
                },
            ),
            Err(e) => ses_error_response(&e),
        }
    }

    async fn handle_get_account(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_get_account_request(request, labels, query) {
            return rest_json_error(400, "ValidationException", &e);
        }
        let state = state.read().await;
        let acct = &state.account_settings;
        wire::serialize_get_account_response(&wire::GetAccountResponse {
            sending_enabled: Some(acct.sending_enabled),
            production_access_enabled: Some(acct.production_access_enabled),
            send_quota: Some(wire::SendQuota {
                max24_hour_send: Some(acct.send_quota.max_24_hour_send),
                max_send_rate: Some(acct.send_quota.max_send_rate),
                sent_last24_hours: Some(acct.send_quota.sent_last_24_hours),
            }),
            dedicated_ip_auto_warmup_enabled: Some(acct.dedicated_ip_auto_warmup_enabled),
            enforcement_status: Some(acct.enforcement_status.clone()),
            ..Default::default()
        })
    }

    async fn handle_put_account_details(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_put_account_details_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        state.account_settings.details = serde_json::to_value(&input).ok();
        wire::serialize_put_account_details_response(&wire::PutAccountDetailsResponse {})
    }

    async fn handle_put_account_dedicated_ip_warmup_attributes(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_put_account_dedicated_ip_warmup_attributes_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let auto_warmup = input.auto_warmup_enabled.unwrap_or(false);
        let mut state = state.write().await;
        state.put_account_dedicated_ip_warmup_attributes(auto_warmup);
        wire::serialize_put_account_dedicated_ip_warmup_attributes_response(
            &wire::PutAccountDedicatedIpWarmupAttributesResponse {},
        )
    }

    async fn handle_put_account_sending_attributes(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_put_account_sending_attributes_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        if let Some(enabled) = input.sending_enabled {
            state.account_settings.sending_enabled = enabled;
        }
        wire::serialize_put_account_sending_attributes_response(
            &wire::PutAccountSendingAttributesResponse {},
        )
    }

    async fn handle_put_account_suppression_attributes(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_put_account_suppression_attributes_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let suppressed_reasons = input.suppressed_reasons.unwrap_or_default();
        let mut state = state.write().await;
        state.put_account_suppression_attributes(suppressed_reasons);
        wire::serialize_put_account_suppression_attributes_response(
            &wire::PutAccountSuppressionAttributesResponse {},
        )
    }

    async fn handle_put_account_vdm_attributes(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_put_account_vdm_attributes_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let vdm_attributes = serde_json::to_value(&input.vdm_attributes).unwrap_or(Value::Null);
        let mut state = state.write().await;
        state.put_account_vdm_attributes(vdm_attributes);
        wire::serialize_put_account_vdm_attributes_response(
            &wire::PutAccountVdmAttributesResponse {},
        )
    }

    async fn handle_get_dedicated_ips(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_dedicated_ips_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        let ips = state.get_dedicated_ips(input.pool_name.as_deref());
        let list: Vec<wire::DedicatedIp> = ips
            .iter()
            .map(|ip| wire::DedicatedIp {
                ip: Some(ip.ip.clone()),
                warmup_status: Some(ip.warmup_status.clone()),
                warmup_percentage: Some(ip.warmup_percentage),
                pool_name: ip.pool_name.clone(),
            })
            .collect();
        wire::serialize_get_dedicated_ips_response(&wire::GetDedicatedIpsResponse {
            dedicated_ips: Some(list),
            ..Default::default()
        })
    }

    async fn handle_get_dedicated_ip(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_dedicated_ip_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_dedicated_ip(&input.ip) {
            Ok(d) => wire::serialize_get_dedicated_ip_response(&wire::GetDedicatedIpResponse {
                dedicated_ip: Some(wire::DedicatedIp {
                    ip: Some(d.ip.clone()),
                    warmup_status: Some(d.warmup_status.clone()),
                    warmup_percentage: Some(d.warmup_percentage),
                    pool_name: d.pool_name.clone(),
                }),
            }),
            Err(e) => ses_error_response(&e),
        }
    }

    async fn handle_put_dedicated_ip_in_pool(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_put_dedicated_ip_in_pool_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.destination_pool_name.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing 'DestinationPoolName'");
        }
        let mut state = state.write().await;
        match state.put_dedicated_ip_in_pool(&input.ip, &input.destination_pool_name) {
            Ok(()) => wire::serialize_put_dedicated_ip_in_pool_response(
                &wire::PutDedicatedIpInPoolResponse {},
            ),
            Err(e) => ses_error_response(&e),
        }
    }

    async fn handle_put_dedicated_ip_warmup_attributes(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_put_dedicated_ip_warmup_attributes_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.put_dedicated_ip_warmup_attributes(&input.ip, input.warmup_percentage) {
            Ok(()) => wire::serialize_put_dedicated_ip_warmup_attributes_response(
                &wire::PutDedicatedIpWarmupAttributesResponse {},
            ),
            Err(e) => ses_error_response(&e),
        }
    }

    async fn handle_put_dedicated_ip_pool_scaling_attributes(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_put_dedicated_ip_pool_scaling_attributes_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let scaling_mode = if input.scaling_mode.is_empty() {
            "STANDARD"
        } else {
            input.scaling_mode.as_str()
        };
        let mut state = state.write().await;
        match state.put_dedicated_ip_pool_scaling_attributes(&input.pool_name, scaling_mode) {
            Ok(()) => wire::serialize_put_dedicated_ip_pool_scaling_attributes_response(
                &wire::PutDedicatedIpPoolScalingAttributesResponse {},
            ),
            Err(e) => ses_error_response(&e),
        }
    }

    async fn handle_put_email_identity_configuration_set_attributes(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_put_email_identity_configuration_set_attributes_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.put_email_identity_configuration_set_attributes(
            &input.email_identity,
            input.configuration_set_name.as_deref(),
        ) {
            Ok(()) => wire::serialize_put_email_identity_configuration_set_attributes_response(
                &wire::PutEmailIdentityConfigurationSetAttributesResponse {},
            ),
            Err(e) => ses_error_response(&e),
        }
    }

    async fn handle_put_email_identity_dkim_attributes(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_put_email_identity_dkim_attributes_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let signing_enabled = input.signing_enabled.unwrap_or(false);
        let mut state = state.write().await;
        match state.put_email_identity_dkim_attributes(&input.email_identity, signing_enabled) {
            Ok(()) => wire::serialize_put_email_identity_dkim_attributes_response(
                &wire::PutEmailIdentityDkimAttributesResponse {},
            ),
            Err(e) => ses_error_response(&e),
        }
    }

    async fn handle_put_email_identity_dkim_signing_attributes(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_put_email_identity_dkim_signing_attributes_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let origin = if input.signing_attributes_origin.is_empty() {
            "AWS_SES"
        } else {
            input.signing_attributes_origin.as_str()
        };
        let domain = input
            .signing_attributes
            .as_ref()
            .and_then(|s| s.domain_signing_selector.as_deref());
        let key_type = input
            .signing_attributes
            .as_ref()
            .and_then(|s| s.next_signing_key_length.as_deref());
        let mut state = state.write().await;
        match state.put_email_identity_dkim_signing_attributes(
            &input.email_identity,
            origin,
            domain,
            key_type,
        ) {
            Ok(()) => wire::serialize_put_email_identity_dkim_signing_attributes_response(
                &wire::PutEmailIdentityDkimSigningAttributesResponse {
                    ..Default::default()
                },
            ),
            Err(e) => ses_error_response(&e),
        }
    }

    async fn handle_put_email_identity_feedback_attributes(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_put_email_identity_feedback_attributes_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let enabled = input.email_forwarding_enabled.unwrap_or(true);
        let mut state = state.write().await;
        match state.put_email_identity_feedback_attributes(&input.email_identity, enabled) {
            Ok(()) => wire::serialize_put_email_identity_feedback_attributes_response(
                &wire::PutEmailIdentityFeedbackAttributesResponse {},
            ),
            Err(e) => ses_error_response(&e),
        }
    }

    async fn handle_put_email_identity_mail_from_attributes(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_put_email_identity_mail_from_attributes_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.put_email_identity_mail_from_attributes(
            &input.email_identity,
            input.mail_from_domain.as_deref(),
            input.behavior_on_mx_failure.as_deref(),
        ) {
            Ok(()) => wire::serialize_put_email_identity_mail_from_attributes_response(
                &wire::PutEmailIdentityMailFromAttributesResponse {},
            ),
            Err(e) => ses_error_response(&e),
        }
    }

    async fn handle_get_suppressed_destination(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_get_suppressed_destination_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let state = state.read().await;
        match state.get_suppressed_destination(&input.email_address) {
            Ok(entry) => wire::serialize_get_suppressed_destination_response(
                &wire::GetSuppressedDestinationResponse {
                    suppressed_destination: Some(wire::SuppressedDestination {
                        email_address: Some(entry.email_address.clone()),
                        reason: Some(entry.reason.clone()),
                        last_update_time: Some(entry.last_update_time),
                        ..Default::default()
                    }),
                },
            ),
            Err(e) => ses_error_response(&e),
        }
    }

    async fn handle_delete_suppressed_destination(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_delete_suppressed_destination_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let mut state = state.write().await;
        match state.delete_suppressed_destination(&input.email_address) {
            Ok(()) => wire::serialize_delete_suppressed_destination_response(
                &wire::DeleteSuppressedDestinationResponse {},
            ),
            Err(e) => ses_error_response(&e),
        }
    }

    async fn handle_list_suppressed_destinations(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) =
            wire::deserialize_list_suppressed_destinations_request(request, labels, query)
        {
            return rest_json_error(400, "ValidationException", &e);
        }
        let state = state.read().await;
        let entries = state.list_suppressed_destinations();
        let list: Vec<wire::SuppressedDestinationSummary> = entries
            .iter()
            .map(|e| wire::SuppressedDestinationSummary {
                email_address: Some(e.email_address.clone()),
                reason: Some(e.reason.clone()),
                last_update_time: Some(e.last_update_time),
            })
            .collect();
        wire::serialize_list_suppressed_destinations_response(
            &wire::ListSuppressedDestinationsResponse {
                suppressed_destination_summaries: Some(list),
                ..Default::default()
            },
        )
    }

    async fn handle_put_suppressed_destination(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_put_suppressed_destination_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        if input.email_address.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing 'EmailAddress'");
        }
        let reason = if input.reason.is_empty() {
            "BOUNCE"
        } else {
            input.reason.as_str()
        };
        let mut state = state.write().await;
        state.put_suppressed_destination(&input.email_address, reason);
        wire::serialize_put_suppressed_destination_response(
            &wire::PutSuppressedDestinationResponse {},
        )
    }

    async fn handle_create_tenant(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_tenant_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.tenant_name.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing 'TenantName'");
        }
        let tags = tags_from_input(input.tags.as_deref());
        let mut state = state.write().await;
        match state.create_tenant(&input.tenant_name, tags) {
            Ok(tenant_id) => wire::serialize_create_tenant_response(&wire::CreateTenantResponse {
                tenant_name: Some(input.tenant_name.clone()),
                tenant_id: Some(tenant_id),
                ..Default::default()
            }),
            Err(e) => ses_error_response(&e),
        }
    }

    async fn handle_delete_tenant(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_tenant_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.tenant_name.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing 'TenantName'");
        }
        let mut state = state.write().await;
        match state.delete_tenant(&input.tenant_name) {
            Ok(()) => wire::serialize_delete_tenant_response(&wire::DeleteTenantResponse {}),
            Err(e) => ses_error_response(&e),
        }
    }

    async fn handle_get_tenant(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_tenant_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.tenant_name.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing 'TenantName'");
        }
        let state = state.read().await;
        match state.get_tenant(&input.tenant_name) {
            Ok(t) => wire::serialize_get_tenant_response(&wire::GetTenantResponse {
                tenant: Some(wire::Tenant {
                    tenant_name: Some(t.tenant_name.clone()),
                    tenant_id: Some(t.tenant_id.clone()),
                    tags: Some(tags_to_wire(&t.tags)),
                    created_timestamp: Some(t.created_timestamp.timestamp_millis() as f64 / 1000.0),
                    ..Default::default()
                }),
            }),
            Err(e) => ses_error_response(&e),
        }
    }

    async fn handle_list_tenants(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_list_tenants_request(request, labels, query) {
            return rest_json_error(400, "ValidationException", &e);
        }
        let state = state.read().await;
        let tenants = state.list_tenants();
        let list: Vec<wire::TenantInfo> = tenants
            .iter()
            .map(|t| wire::TenantInfo {
                tenant_name: Some(t.tenant_name.clone()),
                tenant_id: Some(t.tenant_id.clone()),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_tenants_response(&wire::ListTenantsResponse {
            tenants: Some(list),
            ..Default::default()
        })
    }

    async fn handle_create_tenant_resource_association(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_tenant_resource_association_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.tenant_name.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing 'TenantName'");
        }
        if input.resource_arn.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing 'ResourceArn'");
        }
        let mut state = state.write().await;
        match state.create_tenant_resource_association(&input.tenant_name, &input.resource_arn) {
            Ok(()) => wire::serialize_create_tenant_resource_association_response(
                &wire::CreateTenantResourceAssociationResponse {},
            ),
            Err(e) => ses_error_response(&e),
        }
    }

    async fn handle_delete_tenant_resource_association(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_tenant_resource_association_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.tenant_name.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing 'TenantName'");
        }
        if input.resource_arn.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing 'ResourceArn'");
        }
        let mut state = state.write().await;
        match state.delete_tenant_resource_association(&input.tenant_name, &input.resource_arn) {
            Ok(()) => wire::serialize_delete_tenant_resource_association_response(
                &wire::DeleteTenantResourceAssociationResponse {},
            ),
            Err(e) => ses_error_response(&e),
        }
    }

    async fn handle_list_tenant_resources(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_tenant_resources_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.tenant_name.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing 'TenantName'");
        }
        let state = state.read().await;
        let associations = state.list_tenant_resources(&input.tenant_name);
        let list: Vec<wire::TenantResource> = associations
            .iter()
            .map(|a| wire::TenantResource {
                resource_arn: Some(a.resource_arn.clone()),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_tenant_resources_response(&wire::ListTenantResourcesResponse {
            tenant_resources: Some(list),
            ..Default::default()
        })
    }

    async fn handle_list_resource_tenants(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_resource_tenants_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing 'ResourceArn'");
        }
        let state = state.read().await;
        let associations = state.list_resource_tenants(&input.resource_arn);
        let list: Vec<wire::ResourceTenantMetadata> = associations
            .iter()
            .map(|a| wire::ResourceTenantMetadata {
                tenant_name: Some(a.tenant_name.clone()),
                resource_arn: Some(a.resource_arn.clone()),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_resource_tenants_response(&wire::ListResourceTenantsResponse {
            resource_tenants: Some(list),
            ..Default::default()
        })
    }

    async fn handle_get_message_insights(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_message_insights_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_message_insights(&input.message_id) {
            Some(email) => {
                wire::serialize_get_message_insights_response(&wire::GetMessageInsightsResponse {
                    from_email_address: Some(email.from.clone()),
                    subject: Some(email.subject.clone()),
                    email_tags: Some(vec![]),
                    insights: Some(vec![]),
                    ..Default::default()
                })
            }
            None => rest_json_error(
                404,
                "NotFoundException",
                &format!("Message {} not found", input.message_id),
            ),
        }
    }

    // STUB[no-telemetry]: email address delivery insights are driven by real sending telemetry;
    //   the mock returns zeroed-out metric objects.
    async fn handle_get_email_address_insights(
        &self,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_get_email_address_insights_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        if input.email_address.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing 'EmailAddress'");
        }
        wire::serialize_get_email_address_insights_response(
            &wire::GetEmailAddressInsightsResponse {
                ..Default::default()
            },
        )
    }

    // STUB[no-telemetry]: VDM recommendations are generated by SES from real sending data;
    //   the mock always returns an empty list.
    async fn handle_list_recommendations(&self) -> MockResponse {
        wire::serialize_list_recommendations_response(&wire::ListRecommendationsResponse {
            recommendations: Some(vec![]),
            ..Default::default()
        })
    }

    async fn handle_list_reputation_entities(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_list_reputation_entities_request(request, labels, query) {
            return rest_json_error(400, "ValidationException", &e);
        }
        let state = state.read().await;
        let entities = state.list_reputation_entities();
        let list: Vec<wire::ReputationEntity> = entities
            .iter()
            .map(|e| wire::ReputationEntity {
                reputation_entity_type: Some(e.entity_type.clone()),
                reputation_entity_reference: Some(e.entity_reference.clone()),
                customer_managed_status: Some(wire::StatusRecord {
                    status: Some(e.customer_managed_sending_status.clone()),
                    ..Default::default()
                }),
                reputation_management_policy: e.policy.clone(),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_reputation_entities_response(&wire::ListReputationEntitiesResponse {
            reputation_entities: Some(list),
            ..Default::default()
        })
    }

    async fn handle_get_reputation_entity(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_reputation_entity_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_reputation_entity(
            &input.reputation_entity_type,
            &input.reputation_entity_reference,
        ) {
            Ok(e) => {
                wire::serialize_get_reputation_entity_response(&wire::GetReputationEntityResponse {
                    reputation_entity: Some(wire::ReputationEntity {
                        reputation_entity_type: Some(e.entity_type.clone()),
                        reputation_entity_reference: Some(e.entity_reference.clone()),
                        customer_managed_status: Some(wire::StatusRecord {
                            status: Some(e.customer_managed_sending_status.clone()),
                            ..Default::default()
                        }),
                        reputation_management_policy: e.policy.clone(),
                        ..Default::default()
                    }),
                })
            }
            Err(e) => ses_error_response(&e),
        }
    }

    async fn handle_update_reputation_entity_customer_managed_status(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_reputation_entity_customer_managed_status_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let sending_status = if input.sending_status.is_empty() {
            "ENABLED"
        } else {
            input.sending_status.as_str()
        };
        let mut state = state.write().await;
        state.update_reputation_entity_customer_managed_status(
            &input.reputation_entity_type,
            &input.reputation_entity_reference,
            sending_status,
        );
        wire::serialize_update_reputation_entity_customer_managed_status_response(
            &wire::UpdateReputationEntityCustomerManagedStatusResponse {},
        )
    }

    async fn handle_update_reputation_entity_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_update_reputation_entity_policy_request(request, labels, query)
            {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let policy = if input.reputation_entity_policy.is_empty() {
            None
        } else {
            Some(input.reputation_entity_policy.as_str())
        };
        let mut state = state.write().await;
        state.update_reputation_entity_policy(
            &input.reputation_entity_type,
            &input.reputation_entity_reference,
            policy,
        );
        wire::serialize_update_reputation_entity_policy_response(
            &wire::UpdateReputationEntityPolicyResponse {},
        )
    }

    async fn handle_send_bulk_email(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_send_bulk_email_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let from = input.from_email_address.as_deref().unwrap_or("");
        // SendBulk semantics: at minimum send one entry per BulkEmailEntry.
        // If no entries specified, fall back to 1 (matches the prior behaviour).
        let count = if input.bulk_email_entries.is_empty() {
            1
        } else {
            input.bulk_email_entries.len()
        };
        let mut state = state.write().await;
        let message_ids = state.send_bulk_email(from, count);
        let results: Vec<wire::BulkEmailEntryResult> = message_ids
            .iter()
            .map(|id| wire::BulkEmailEntryResult {
                message_id: Some(id.clone()),
                status: Some("SUCCESS".to_string()),
                ..Default::default()
            })
            .collect();
        wire::serialize_send_bulk_email_response(&wire::SendBulkEmailResponse {
            bulk_email_entry_results: Some(results),
        })
    }

    async fn handle_send_custom_verification_email(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_send_custom_verification_email_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.email_address.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing 'EmailAddress'");
        }
        if input.template_name.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing 'TemplateName'");
        }
        let mut state = state.write().await;
        match state.send_custom_verification_email(&input.email_address, &input.template_name) {
            Ok(message_id) => wire::serialize_send_custom_verification_email_response(
                &wire::SendCustomVerificationEmailResponse {
                    message_id: Some(message_id),
                },
            ),
            Err(e) => ses_error_response(&e),
        }
    }

    async fn handle_update_contact_list(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_contact_list_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        // UpdateContactListRequest does not have a Tags field in the Smithy
        // model, and the SDK builder doesn't expose one — pass None.
        let tags: Option<HashMap<String, String>> = None;
        let mut state = state.write().await;
        match state.update_contact_list(
            &input.contact_list_name,
            input.description.as_deref(),
            tags,
        ) {
            Ok(()) => {
                wire::serialize_update_contact_list_response(&wire::UpdateContactListResponse {})
            }
            Err(e) => ses_error_response(&e),
        }
    }

    async fn handle_update_contact(
        &self,
        state: &Arc<tokio::sync::RwLock<SesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_contact_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let topic_preferences: Option<Vec<TopicPreference>> = input.topic_preferences.map(|v| {
            v.into_iter()
                .map(|tp| TopicPreference {
                    topic_name: tp.topic_name,
                    subscription_status: if tp.subscription_status.is_empty() {
                        "OPT_IN".to_string()
                    } else {
                        tp.subscription_status
                    },
                })
                .collect()
        });
        let mut state = state.write().await;
        match state.update_contact(
            &input.contact_list_name,
            &input.email_address,
            input.unsubscribe_all,
            topic_preferences,
        ) {
            Ok(()) => wire::serialize_update_contact_response(&wire::UpdateContactResponse {}),
            Err(e) => ses_error_response(&e),
        }
    }
}

fn extract_path(uri: &str) -> String {
    // Delegate to the shared core helper, which correctly strips the scheme
    // and host (including custom-endpoint hostnames like `127.0.0.1:PORT`)
    // before returning the path. The previous implementation only matched on
    // `amazonaws.com` and returned the entire URI for non-AWS endpoints,
    // causing dispatch to fail with 404 against the in-process mock server.
    winterbaume_core::extract_path(uri)
}

fn extract_query(uri: &str) -> String {
    match uri.find('?') {
        Some(idx) => uri[idx + 1..].to_string(),
        None => String::new(),
    }
}

fn percent_decode(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut bytes = s.bytes();
    while let Some(b) = bytes.next() {
        match b {
            b'%' => {
                let hi = bytes.next().and_then(hex_val);
                let lo = bytes.next().and_then(hex_val);
                if let (Some(hi), Some(lo)) = (hi, lo) {
                    result.push((hi << 4 | lo) as char);
                }
            }
            b'+' => result.push(' '),
            _ => result.push(b as char),
        }
    }
    result
}

fn hex_val(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

fn parse_tag_keys_from_query(query: &str) -> Vec<String> {
    query
        .split('&')
        .filter_map(|param| {
            let parts: Vec<&str> = param.splitn(2, '=').collect();
            if parts.len() == 2 && parts[0] == "TagKeys" {
                Some(percent_decode(parts[1]))
            } else {
                None
            }
        })
        .collect()
}

fn tags_from_input(tags: Option<&[wire::Tag]>) -> HashMap<String, String> {
    let mut out = HashMap::new();
    if let Some(arr) = tags {
        for t in arr {
            out.insert(t.key.clone(), t.value.clone());
        }
    }
    out
}

fn tags_to_wire(tags: &HashMap<String, String>) -> Vec<wire::Tag> {
    tags.iter()
        .map(|(k, v)| wire::Tag {
            key: k.clone(),
            value: v.clone(),
        })
        .collect()
}

fn ses_error_response(err: &SesError) -> MockResponse {
    let (status, error_type) = match err {
        SesError::IdentityAlreadyExists(_) => (400u16, "AlreadyExistsException"),
        SesError::PolicyAlreadyExists(_) => (400, "AlreadyExistsException"),
        SesError::ConfigurationSetAlreadyExists(_) => (400, "AlreadyExistsException"),
        SesError::EventDestinationAlreadyExists(_, _) => (400, "AlreadyExistsException"),
        SesError::ContactListAlreadyExists(_) => (400, "AlreadyExistsException"),
        SesError::ContactAlreadyExists(_) => (400, "AlreadyExistsException"),
        SesError::DedicatedIpPoolAlreadyExists(_) => (400, "AlreadyExistsException"),
        SesError::EmailTemplateAlreadyExists(_) => (400, "AlreadyExistsException"),
        SesError::CustomVerificationEmailTemplateAlreadyExists(_) => {
            (400, "AlreadyExistsException")
        }
        SesError::TenantAlreadyExists(_) => (400, "AlreadyExistsException"),
        SesError::TenantResourceAssociationAlreadyExists(_, _) => (400, "AlreadyExistsException"),
        SesError::IdentityNotFound(_) => (404, "NotFoundException"),
        SesError::PolicyNotFound(_) => (404, "NotFoundException"),
        SesError::ConfigurationSetNotFound(_) => (404, "NotFoundException"),
        SesError::EventDestinationNotFound(_, _) => (404, "NotFoundException"),
        SesError::ContactListNotFound(_) => (404, "NotFoundException"),
        SesError::ContactNotFound(_, _) => (404, "NotFoundException"),
        SesError::DedicatedIpPoolNotFound(_) => (404, "NotFoundException"),
        SesError::DedicatedIpNotFound(_) => (404, "NotFoundException"),
        SesError::EmailTemplateNotFound(_) => (404, "NotFoundException"),
        SesError::CustomVerificationEmailTemplateNotFound(_) => (404, "NotFoundException"),
        SesError::ImportJobNotFound(_) => (404, "NotFoundException"),
        SesError::ExportJobNotFound(_) => (404, "NotFoundException"),
        SesError::MultiRegionEndpointNotFound(_) => (404, "NotFoundException"),
        SesError::ResourceNotFound(_) => (404, "NotFoundException"),
        SesError::SuppressedDestinationNotFound(_) => (404, "NotFoundException"),
        SesError::TenantNotFound(_) => (404, "NotFoundException"),
        SesError::TenantResourceAssociationNotFound(_, _) => (404, "NotFoundException"),
        SesError::DeliverabilityTestReportNotFound(_) => (404, "NotFoundException"),
        SesError::ReputationEntityNotFound(_, _) => (404, "NotFoundException"),
    };
    rest_json_error(status, error_type, &err.to_string())
}

fn rest_json_error(status: u16, code: &str, message: &str) -> MockResponse {
    let body = json!({
        "Type": "User",
        "Message": message,
    });
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers.insert(X_AMZN_ERRORTYPE, code.parse().unwrap());
    resp
}
