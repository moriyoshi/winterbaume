use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
    StatefulService,
};

use crate::state::{Route53DomainsError, Route53DomainsState};
use crate::types::ContactDetail;
use crate::views::Route53DomainsStateView;
use crate::wire;

pub struct Route53DomainsService {
    pub(crate) state: Arc<BackendState<Route53DomainsState>>,
    pub(crate) notifier: StateChangeNotifier<Route53DomainsStateView>,
}

impl Route53DomainsService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for Route53DomainsService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for Route53DomainsService {
    fn service_name(&self) -> &str {
        "route53domains"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://route53domains\..*\.amazonaws\.com",
            r"https?://route53domains\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl Route53DomainsService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;

        let action = request
            .headers
            .get("x-amz-target")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.split('.').next_back())
            .map(|s| s.to_string());

        let action = match action {
            Some(a) => a,
            None => {
                return json_error_response(400, "MissingAction", "Missing X-Amz-Target header");
            }
        };

        if serde_json::from_slice::<Value>(&request.body).is_err() {
            return json_error_response(400, "SerializationException", "Invalid JSON body");
        }
        let body_bytes: &[u8] = &request.body;

        let state = self.state.get(account_id, &region);

        let response = match action.as_str() {
            "RegisterDomain" => self.handle_register_domain(&state, body_bytes).await,
            "GetDomainDetail" => self.handle_get_domain_detail(&state, body_bytes).await,
            "ListDomains" => self.handle_list_domains(&state, body_bytes).await,
            "CheckDomainAvailability" => {
                self.handle_check_domain_availability(&state, body_bytes)
                    .await
            }
            "DeleteDomain" => self.handle_delete_domain(&state, body_bytes).await,
            // --- Unimplemented operations ---
            "AcceptDomainTransferFromAnotherAwsAccount" => json_error_response(
                501,
                "NotImplementedError",
                "AcceptDomainTransferFromAnotherAwsAccount is not yet implemented in winterbaume-route53domains",
            ),
            "AssociateDelegationSignerToDomain" => json_error_response(
                501,
                "NotImplementedError",
                "AssociateDelegationSignerToDomain is not yet implemented in winterbaume-route53domains",
            ),
            "CancelDomainTransferToAnotherAwsAccount" => json_error_response(
                501,
                "NotImplementedError",
                "CancelDomainTransferToAnotherAwsAccount is not yet implemented in winterbaume-route53domains",
            ),
            "CheckDomainTransferability" => json_error_response(
                501,
                "NotImplementedError",
                "CheckDomainTransferability is not yet implemented in winterbaume-route53domains",
            ),
            "DeleteTagsForDomain" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteTagsForDomain is not yet implemented in winterbaume-route53domains",
            ),
            "DisableDomainAutoRenew" => json_error_response(
                501,
                "NotImplementedError",
                "DisableDomainAutoRenew is not yet implemented in winterbaume-route53domains",
            ),
            "DisableDomainTransferLock" => json_error_response(
                501,
                "NotImplementedError",
                "DisableDomainTransferLock is not yet implemented in winterbaume-route53domains",
            ),
            "DisassociateDelegationSignerFromDomain" => json_error_response(
                501,
                "NotImplementedError",
                "DisassociateDelegationSignerFromDomain is not yet implemented in winterbaume-route53domains",
            ),
            "EnableDomainAutoRenew" => json_error_response(
                501,
                "NotImplementedError",
                "EnableDomainAutoRenew is not yet implemented in winterbaume-route53domains",
            ),
            "EnableDomainTransferLock" => json_error_response(
                501,
                "NotImplementedError",
                "EnableDomainTransferLock is not yet implemented in winterbaume-route53domains",
            ),
            "GetContactReachabilityStatus" => json_error_response(
                501,
                "NotImplementedError",
                "GetContactReachabilityStatus is not yet implemented in winterbaume-route53domains",
            ),
            "GetDomainSuggestions" => json_error_response(
                501,
                "NotImplementedError",
                "GetDomainSuggestions is not yet implemented in winterbaume-route53domains",
            ),
            "GetOperationDetail" => json_error_response(
                501,
                "NotImplementedError",
                "GetOperationDetail is not yet implemented in winterbaume-route53domains",
            ),
            "ListOperations" => json_error_response(
                501,
                "NotImplementedError",
                "ListOperations is not yet implemented in winterbaume-route53domains",
            ),
            "ListPrices" => json_error_response(
                501,
                "NotImplementedError",
                "ListPrices is not yet implemented in winterbaume-route53domains",
            ),
            "ListTagsForDomain" => json_error_response(
                501,
                "NotImplementedError",
                "ListTagsForDomain is not yet implemented in winterbaume-route53domains",
            ),
            "PushDomain" => json_error_response(
                501,
                "NotImplementedError",
                "PushDomain is not yet implemented in winterbaume-route53domains",
            ),
            "RejectDomainTransferFromAnotherAwsAccount" => json_error_response(
                501,
                "NotImplementedError",
                "RejectDomainTransferFromAnotherAwsAccount is not yet implemented in winterbaume-route53domains",
            ),
            "RenewDomain" => json_error_response(
                501,
                "NotImplementedError",
                "RenewDomain is not yet implemented in winterbaume-route53domains",
            ),
            "ResendContactReachabilityEmail" => json_error_response(
                501,
                "NotImplementedError",
                "ResendContactReachabilityEmail is not yet implemented in winterbaume-route53domains",
            ),
            "ResendOperationAuthorization" => json_error_response(
                501,
                "NotImplementedError",
                "ResendOperationAuthorization is not yet implemented in winterbaume-route53domains",
            ),
            "RetrieveDomainAuthCode" => json_error_response(
                501,
                "NotImplementedError",
                "RetrieveDomainAuthCode is not yet implemented in winterbaume-route53domains",
            ),
            "TransferDomain" => json_error_response(
                501,
                "NotImplementedError",
                "TransferDomain is not yet implemented in winterbaume-route53domains",
            ),
            "TransferDomainToAnotherAwsAccount" => json_error_response(
                501,
                "NotImplementedError",
                "TransferDomainToAnotherAwsAccount is not yet implemented in winterbaume-route53domains",
            ),
            "UpdateDomainContact" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateDomainContact is not yet implemented in winterbaume-route53domains",
            ),
            "UpdateDomainContactPrivacy" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateDomainContactPrivacy is not yet implemented in winterbaume-route53domains",
            ),
            "UpdateDomainNameservers" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateDomainNameservers is not yet implemented in winterbaume-route53domains",
            ),
            "UpdateTagsForDomain" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateTagsForDomain is not yet implemented in winterbaume-route53domains",
            ),
            "ViewBilling" => json_error_response(
                501,
                "NotImplementedError",
                "ViewBilling is not yet implemented in winterbaume-route53domains",
            ),
            _ => json_error_response(400, "InvalidAction", &format!("Unknown operation {action}")),
        };
        if response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_register_domain(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53DomainsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_register_domain_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.domain_name.is_empty() {
            return json_error_response(400, "InvalidInput", "DomainName is required");
        }
        if is_empty_contact(&input.admin_contact) {
            return json_error_response(400, "InvalidInput", "AdminContact is required");
        }
        if is_empty_contact(&input.registrant_contact) {
            return json_error_response(400, "InvalidInput", "RegistrantContact is required");
        }
        if is_empty_contact(&input.tech_contact) {
            return json_error_response(400, "InvalidInput", "TechContact is required");
        }
        let domain_name = input.domain_name.as_str();
        let duration_in_years = if input.duration_in_years == 0 {
            1
        } else {
            input.duration_in_years
        };
        let auto_renew = input.auto_renew.unwrap_or(true);
        let admin_contact = wire_contact_to_state(&input.admin_contact);
        let registrant_contact = wire_contact_to_state(&input.registrant_contact);
        let tech_contact = wire_contact_to_state(&input.tech_contact);
        let privacy_admin = input.privacy_protect_admin_contact.unwrap_or(true);
        let privacy_registrant = input.privacy_protect_registrant_contact.unwrap_or(true);
        let privacy_tech = input.privacy_protect_tech_contact.unwrap_or(true);

        let mut state = state.write().await;
        match state.register_domain(
            domain_name,
            duration_in_years,
            auto_renew,
            admin_contact,
            registrant_contact,
            tech_contact,
            privacy_admin,
            privacy_registrant,
            privacy_tech,
        ) {
            Ok(operation_id) => {
                wire::serialize_register_domain_response(&wire::RegisterDomainResponse {
                    operation_id: Some(operation_id),
                })
            }
            Err(e) => route53domains_error_response(&e),
        }
    }

    async fn handle_get_domain_detail(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53DomainsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_domain_detail_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.domain_name.is_empty() {
            return json_error_response(400, "InvalidInput", "DomainName is required");
        }
        let domain_name = input.domain_name.as_str();

        let state = state.read().await;
        match state.get_domain_detail(domain_name) {
            Ok(domain) => {
                wire::serialize_get_domain_detail_response(&wire::GetDomainDetailResponse {
                    domain_name: Some(domain.domain_name.clone()),
                    nameservers: Some(
                        domain
                            .nameservers
                            .iter()
                            .map(|ns| wire::Nameserver {
                                name: ns.clone(),
                                glue_ips: None,
                            })
                            .collect(),
                    ),
                    auto_renew: Some(domain.auto_renew),
                    admin_contact: Some(contact_to_wire(&domain.admin_contact)),
                    registrant_contact: Some(contact_to_wire(&domain.registrant_contact)),
                    tech_contact: Some(contact_to_wire(&domain.tech_contact)),
                    admin_privacy: Some(domain.admin_privacy),
                    registrant_privacy: Some(domain.registrant_privacy),
                    tech_privacy: Some(domain.tech_privacy),
                    registrar_name: Some("Amazon Registrar, Inc.".to_string()),
                    who_is_server: Some("whois.registrar.amazon.com".to_string()),
                    registrar_url: Some("https://registrar.amazon.com".to_string()),
                    abuse_contact_email: Some("abuse@registrar.amazon.com".to_string()),
                    abuse_contact_phone: Some("+1.2062661000".to_string()),
                    creation_date: Some(domain.creation_date.timestamp() as f64),
                    updated_date: Some(domain.updated_date.timestamp() as f64),
                    expiration_date: Some(domain.expiration_date.timestamp() as f64),
                    reseller: Some("Amazon".to_string()),
                    status_list: Some(domain.status_list.clone()),
                    ..Default::default()
                })
            }
            Err(e) => route53domains_error_response(&e),
        }
    }

    /// Single-page response — pagination not implemented; full result set
    /// returned in one call. The `MaxItems` parameter is honoured as a soft
    /// truncation, but no `NextPageMarker` is returned even when the result is
    /// truncated, since the underlying collection lacks a deterministic order
    /// that would make a continuation token meaningful.
    async fn handle_list_domains(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53DomainsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_domains_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let max_items = input.max_items.map(|m| m.max(0) as usize).unwrap_or(20);

        let state = state.read().await;
        let domains = state.list_domains();
        let mut entries: Vec<wire::DomainSummary> = domains
            .iter()
            .map(|d| wire::DomainSummary {
                domain_name: Some(d.domain_name.clone()),
                auto_renew: Some(d.auto_renew),
                transfer_lock: Some(d.transfer_lock),
                expiry: Some(d.expiration_date.timestamp() as f64),
            })
            .collect();

        if entries.len() > max_items {
            entries.truncate(max_items);
        }

        wire::serialize_list_domains_response(&wire::ListDomainsResponse {
            domains: Some(entries),
            next_page_marker: None,
        })
    }

    async fn handle_check_domain_availability(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53DomainsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_check_domain_availability_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.domain_name.is_empty() {
            return json_error_response(400, "InvalidInput", "DomainName is required");
        }
        let domain_name = input.domain_name.as_str();

        let state = state.read().await;
        let availability = state.check_domain_availability(domain_name);
        wire::serialize_check_domain_availability_response(&wire::CheckDomainAvailabilityResponse {
            availability: Some(availability),
        })
    }

    async fn handle_delete_domain(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53DomainsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_domain_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.domain_name.is_empty() {
            return json_error_response(400, "InvalidInput", "DomainName is required");
        }
        let domain_name = input.domain_name.as_str();

        let mut state = state.write().await;
        match state.delete_domain(domain_name) {
            Ok(operation_id) => {
                wire::serialize_delete_domain_response(&wire::DeleteDomainResponse {
                    operation_id: Some(operation_id),
                })
            }
            Err(e) => route53domains_error_response(&e),
        }
    }
}

/// Returns true if the wire ContactDetail is the all-None default (i.e. the
/// caller did not actually supply a contact in the JSON body).
fn is_empty_contact(c: &wire::ContactDetail) -> bool {
    c.first_name.is_none()
        && c.last_name.is_none()
        && c.email.is_none()
        && c.phone_number.is_none()
        && c.organization_name.is_none()
        && c.address_line1.is_none()
        && c.address_line2.is_none()
        && c.city.is_none()
        && c.state.is_none()
        && c.country_code.is_none()
        && c.zip_code.is_none()
        && c.contact_type.is_none()
        && c.fax.is_none()
        && c.extra_params.is_none()
}

/// Convert a wire ContactDetail to a state ContactDetail.
fn wire_contact_to_state(c: &wire::ContactDetail) -> ContactDetail {
    ContactDetail {
        first_name: c.first_name.clone(),
        last_name: c.last_name.clone(),
        email: c.email.clone(),
        phone_number: c.phone_number.clone(),
        organization_name: c.organization_name.clone(),
        address_line_1: c.address_line1.clone(),
        address_line_2: c.address_line2.clone(),
        city: c.city.clone(),
        state: c.state.clone(),
        country_code: c.country_code.clone(),
        zip_code: c.zip_code.clone(),
        contact_type: c.contact_type.clone(),
    }
}

/// Convert a state ContactDetail to a wire ContactDetail.
fn contact_to_wire(c: &ContactDetail) -> wire::ContactDetail {
    wire::ContactDetail {
        first_name: c.first_name.clone(),
        last_name: c.last_name.clone(),
        email: c.email.clone(),
        phone_number: c.phone_number.clone(),
        organization_name: c.organization_name.clone(),
        address_line1: c.address_line_1.clone(),
        address_line2: c.address_line_2.clone(),
        city: c.city.clone(),
        state: c.state.clone(),
        country_code: c.country_code.clone(),
        zip_code: c.zip_code.clone(),
        contact_type: c.contact_type.clone(),
        ..Default::default()
    }
}

fn json_error_response(status: u16, code: &str, message: &str) -> MockResponse {
    let body = json!({
        "__type": code,
        "message": message,
    });
    MockResponse::json(status, body.to_string())
}

fn route53domains_error_response(err: &Route53DomainsError) -> MockResponse {
    match err {
        Route53DomainsError::DuplicateRequest { .. } => {
            json_error_response(400, "DuplicateRequest", &err.to_string())
        }
        Route53DomainsError::DomainNotFound { .. } => {
            json_error_response(400, "InvalidInput", &err.to_string())
        }
    }
}
