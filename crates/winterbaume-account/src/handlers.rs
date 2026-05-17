use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, default_account_id,
    extract_path, rest_json_error,
};

use crate::state::{AccountError, AccountState};
use crate::types::{AlternateContact, ContactInformation};
use crate::views::AccountStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct AccountService {
    pub(crate) state: Arc<BackendState<AccountState>>,
    pub(crate) notifier: StateChangeNotifier<AccountStateView>,
}

impl AccountService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for AccountService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for AccountService {
    fn service_name(&self) -> &str {
        "account"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://account\..*\.amazonaws\.com",
            r"https?://account\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl AccountService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);

        let body: Value = if request.body.is_empty() {
            json!({})
        } else {
            match serde_json::from_slice(&request.body) {
                Ok(v) => v,
                Err(_) => {
                    return rest_json_error(400, "BadRequestException", "Invalid JSON body");
                }
            }
        };

        let response = match path.as_str() {
            "/putAlternateContact" => self.handle_put_alternate_contact(&state, &body).await,
            "/getAlternateContact" => self.handle_get_alternate_contact(&state, &body).await,
            "/deleteAlternateContact" => self.handle_delete_alternate_contact(&state, &body).await,
            "/putContactInformation" => self.handle_put_contact_information(&state, &body).await,
            "/getContactInformation" => self.handle_get_contact_information(&state, &body).await,
            "/putAccountName" => self.handle_put_account_name(&state, &body).await,
            "/getAccountInformation" => {
                self.handle_get_account_information(&state, account_id)
                    .await
            }
            "/getGovCloudAccountInformation" => {
                self.handle_get_gov_cloud_account_information(&body).await
            }
            "/getPrimaryEmail" => self.handle_get_primary_email(&state, &body).await,
            "/startPrimaryEmailUpdate" => {
                self.handle_start_primary_email_update(&state, &body).await
            }
            "/acceptPrimaryEmailUpdate" => {
                self.handle_accept_primary_email_update(&state, &body).await
            }
            "/getRegionOptStatus" => self.handle_get_region_opt_status(&state, &body).await,
            "/enableRegion" => self.handle_enable_region(&state, &body).await,
            "/disableRegion" => self.handle_disable_region(&state, &body).await,
            "/listRegions" => self.handle_list_regions(&state, &body).await,
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        };

        use winterbaume_core::StatefulService;
        if response.status / 100 == 2 {
            let method = request.method.as_str();
            if matches!(
                path.as_str(),
                "/putAlternateContact"
                    | "/deleteAlternateContact"
                    | "/putContactInformation"
                    | "/putAccountName"
                    | "/startPrimaryEmailUpdate"
                    | "/acceptPrimaryEmailUpdate"
                    | "/enableRegion"
                    | "/disableRegion"
            ) && method == "POST"
            {
                self.notify_state_changed(account_id, &region).await;
            }
        }
        response
    }

    async fn handle_put_alternate_contact(
        &self,
        state: &Arc<tokio::sync::RwLock<AccountState>>,
        body: &Value,
    ) -> MockResponse {
        let contact_type = match body.get("AlternateContactType").and_then(|v| v.as_str()) {
            Some(v) => v,
            None => {
                return rest_json_error(
                    400,
                    "ValidationException",
                    "AlternateContactType is required",
                );
            }
        };
        let email = match body.get("EmailAddress").and_then(|v| v.as_str()) {
            Some(v) => v,
            None => return rest_json_error(400, "ValidationException", "EmailAddress is required"),
        };
        let name = match body.get("Name").and_then(|v| v.as_str()) {
            Some(v) => v,
            None => return rest_json_error(400, "ValidationException", "Name is required"),
        };
        let phone = match body.get("PhoneNumber").and_then(|v| v.as_str()) {
            Some(v) => v,
            None => return rest_json_error(400, "ValidationException", "PhoneNumber is required"),
        };
        let title = match body.get("Title").and_then(|v| v.as_str()) {
            Some(v) => v,
            None => return rest_json_error(400, "ValidationException", "Title is required"),
        };

        let mut state = state.write().await;
        match state.put_alternate_contact(contact_type, email, name, phone, title) {
            Ok(()) => wire::serialize_put_alternate_contact_response(),
            Err(e) => account_error_response(&e),
        }
    }

    async fn handle_get_alternate_contact(
        &self,
        state: &Arc<tokio::sync::RwLock<AccountState>>,
        body: &Value,
    ) -> MockResponse {
        let contact_type = match body.get("AlternateContactType").and_then(|v| v.as_str()) {
            Some(v) => v,
            None => {
                return rest_json_error(
                    400,
                    "ValidationException",
                    "AlternateContactType is required",
                );
            }
        };

        let state = state.read().await;
        match state.get_alternate_contact(contact_type) {
            Ok(contact) => {
                let response = wire::GetAlternateContactResponse {
                    alternate_contact: Some(to_alternate_contact_output(contact)),
                };
                wire::serialize_get_alternate_contact_response(&response)
            }
            Err(e) => account_error_response(&e),
        }
    }

    async fn handle_delete_alternate_contact(
        &self,
        state: &Arc<tokio::sync::RwLock<AccountState>>,
        body: &Value,
    ) -> MockResponse {
        let contact_type = match body.get("AlternateContactType").and_then(|v| v.as_str()) {
            Some(v) => v,
            None => {
                return rest_json_error(
                    400,
                    "ValidationException",
                    "AlternateContactType is required",
                );
            }
        };

        let mut state = state.write().await;
        match state.delete_alternate_contact(contact_type) {
            Ok(()) => wire::serialize_delete_alternate_contact_response(),
            Err(e) => account_error_response(&e),
        }
    }

    async fn handle_put_contact_information(
        &self,
        state: &Arc<tokio::sync::RwLock<AccountState>>,
        body: &Value,
    ) -> MockResponse {
        let ci_val = match body.get("ContactInformation") {
            Some(v) => v,
            None => {
                return rest_json_error(
                    400,
                    "ValidationException",
                    "ContactInformation is required",
                );
            }
        };
        let address_line1 = match ci_val.get("AddressLine1").and_then(|v| v.as_str()) {
            Some(v) => v.to_string(),
            None => return rest_json_error(400, "ValidationException", "AddressLine1 is required"),
        };
        let city = match ci_val.get("City").and_then(|v| v.as_str()) {
            Some(v) => v.to_string(),
            None => return rest_json_error(400, "ValidationException", "City is required"),
        };
        let country_code = match ci_val.get("CountryCode").and_then(|v| v.as_str()) {
            Some(v) => v.to_string(),
            None => return rest_json_error(400, "ValidationException", "CountryCode is required"),
        };
        let full_name = match ci_val.get("FullName").and_then(|v| v.as_str()) {
            Some(v) => v.to_string(),
            None => return rest_json_error(400, "ValidationException", "FullName is required"),
        };
        let phone_number = match ci_val.get("PhoneNumber").and_then(|v| v.as_str()) {
            Some(v) => v.to_string(),
            None => return rest_json_error(400, "ValidationException", "PhoneNumber is required"),
        };
        let postal_code = match ci_val.get("PostalCode").and_then(|v| v.as_str()) {
            Some(v) => v.to_string(),
            None => return rest_json_error(400, "ValidationException", "PostalCode is required"),
        };

        let info = ContactInformation {
            address_line1,
            address_line2: ci_val
                .get("AddressLine2")
                .and_then(|v| v.as_str())
                .map(str::to_string),
            address_line3: ci_val
                .get("AddressLine3")
                .and_then(|v| v.as_str())
                .map(str::to_string),
            city,
            company_name: ci_val
                .get("CompanyName")
                .and_then(|v| v.as_str())
                .map(str::to_string),
            country_code,
            district_or_county: ci_val
                .get("DistrictOrCounty")
                .and_then(|v| v.as_str())
                .map(str::to_string),
            full_name,
            phone_number,
            postal_code,
            state_or_region: ci_val
                .get("StateOrRegion")
                .and_then(|v| v.as_str())
                .map(str::to_string),
            website_url: ci_val
                .get("WebsiteUrl")
                .and_then(|v| v.as_str())
                .map(str::to_string),
        };

        let mut state = state.write().await;
        match state.put_contact_information(info) {
            Ok(()) => wire::serialize_put_contact_information_response(),
            Err(e) => account_error_response(&e),
        }
    }

    async fn handle_get_contact_information(
        &self,
        state: &Arc<tokio::sync::RwLock<AccountState>>,
        _body: &Value,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_contact_information() {
            Ok(info) => {
                let response = wire::GetContactInformationResponse {
                    contact_information: Some(to_contact_information_output(info)),
                };
                wire::serialize_get_contact_information_response(&response)
            }
            Err(e) => account_error_response(&e),
        }
    }

    async fn handle_put_account_name(
        &self,
        state: &Arc<tokio::sync::RwLock<AccountState>>,
        body: &Value,
    ) -> MockResponse {
        let name = match body.get("AccountName").and_then(|v| v.as_str()) {
            Some(v) => v,
            None => return rest_json_error(400, "ValidationException", "AccountName is required"),
        };
        let mut state = state.write().await;
        match state.put_account_name(name) {
            Ok(()) => wire::serialize_put_account_name_response(),
            Err(e) => account_error_response(&e),
        }
    }

    async fn handle_get_account_information(
        &self,
        state: &Arc<tokio::sync::RwLock<AccountState>>,
        account_id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        let (acc_id, acc_name) = state.get_account_information(account_id);
        let response = wire::GetAccountInformationResponse {
            account_id: Some(acc_id),
            account_name: acc_name,
            account_created_date: None,
            account_state: None,
        };
        wire::serialize_get_account_information_response(&response)
    }

    // STUB[org-integration]: GovCloud is a separate AWS partition; the mock has no cross-partition
    //   account state, so a fixed minimal response is returned.
    async fn handle_get_gov_cloud_account_information(&self, _body: &Value) -> MockResponse {
        // Return a minimal mock GovCloud account info
        let response = wire::GetGovCloudAccountInformationResponse {
            gov_cloud_account_id: None,
            account_state: Some("ACTIVE".to_string()),
        };
        wire::serialize_get_gov_cloud_account_information_response(&response)
    }

    async fn handle_get_primary_email(
        &self,
        state: &Arc<tokio::sync::RwLock<AccountState>>,
        _body: &Value,
    ) -> MockResponse {
        let state = state.read().await;
        let response = wire::GetPrimaryEmailResponse {
            primary_email: state.get_primary_email().map(str::to_string),
        };
        wire::serialize_get_primary_email_response(&response)
    }

    async fn handle_start_primary_email_update(
        &self,
        state: &Arc<tokio::sync::RwLock<AccountState>>,
        body: &Value,
    ) -> MockResponse {
        let new_email = match body.get("PrimaryEmail").and_then(|v| v.as_str()) {
            Some(v) => v.to_string(),
            None => return rest_json_error(400, "ValidationException", "PrimaryEmail is required"),
        };
        let mut state = state.write().await;
        state.start_primary_email_update(&new_email);
        let response = wire::StartPrimaryEmailUpdateResponse {
            status: Some("PENDING".to_string()),
        };
        wire::serialize_start_primary_email_update_response(&response)
    }

    async fn handle_accept_primary_email_update(
        &self,
        state: &Arc<tokio::sync::RwLock<AccountState>>,
        body: &Value,
    ) -> MockResponse {
        let new_email = match body.get("PrimaryEmail").and_then(|v| v.as_str()) {
            Some(v) => v.to_string(),
            None => return rest_json_error(400, "ValidationException", "PrimaryEmail is required"),
        };
        let otp = match body.get("Otp").and_then(|v| v.as_str()) {
            Some(v) => v.to_string(),
            None => return rest_json_error(400, "ValidationException", "Otp is required"),
        };
        let mut state = state.write().await;
        match state.accept_primary_email_update(&new_email, &otp) {
            Ok(()) => {
                let response = wire::AcceptPrimaryEmailUpdateResponse {
                    status: Some("ACCEPTED".to_string()),
                };
                wire::serialize_accept_primary_email_update_response(&response)
            }
            Err(e) => account_error_response(&e),
        }
    }

    async fn handle_get_region_opt_status(
        &self,
        state: &Arc<tokio::sync::RwLock<AccountState>>,
        body: &Value,
    ) -> MockResponse {
        let region_name = match body.get("RegionName").and_then(|v| v.as_str()) {
            Some(v) => v.to_string(),
            None => return rest_json_error(400, "ValidationException", "RegionName is required"),
        };
        let state = state.read().await;
        let opt_status = state.get_region_opt_status(&region_name);
        let response = wire::GetRegionOptStatusResponse {
            region_name: Some(region_name),
            region_opt_status: Some(opt_status.as_str().to_string()),
        };
        wire::serialize_get_region_opt_status_response(&response)
    }

    async fn handle_enable_region(
        &self,
        state: &Arc<tokio::sync::RwLock<AccountState>>,
        body: &Value,
    ) -> MockResponse {
        let region_name = match body.get("RegionName").and_then(|v| v.as_str()) {
            Some(v) => v.to_string(),
            None => return rest_json_error(400, "ValidationException", "RegionName is required"),
        };
        let mut state = state.write().await;
        match state.enable_region(&region_name) {
            Ok(()) => wire::serialize_enable_region_response(),
            Err(e) => account_error_response(&e),
        }
    }

    async fn handle_disable_region(
        &self,
        state: &Arc<tokio::sync::RwLock<AccountState>>,
        body: &Value,
    ) -> MockResponse {
        let region_name = match body.get("RegionName").and_then(|v| v.as_str()) {
            Some(v) => v.to_string(),
            None => return rest_json_error(400, "ValidationException", "RegionName is required"),
        };
        let mut state = state.write().await;
        match state.disable_region(&region_name) {
            Ok(()) => wire::serialize_disable_region_response(),
            Err(e) => account_error_response(&e),
        }
    }

    async fn handle_list_regions(
        &self,
        state: &Arc<tokio::sync::RwLock<AccountState>>,
        body: &Value,
    ) -> MockResponse {
        let max_results = body
            .get("MaxResults")
            .and_then(|v| v.as_i64())
            .map(|v| v as i32);
        let next_token = body.get("NextToken").and_then(|v| v.as_str());
        let filter: Option<Vec<String>> = body
            .get("RegionOptStatusContains")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(str::to_string))
                    .collect()
            });

        let state = state.read().await;
        let (regions, new_next_token) =
            state.list_regions(filter.as_deref(), max_results, next_token);

        let wire_regions: Vec<wire::Region> = regions
            .into_iter()
            .map(|(name, status)| wire::Region {
                region_name: Some(name),
                region_opt_status: Some(status.as_str().to_string()),
            })
            .collect();

        let response = wire::ListRegionsResponse {
            regions: if wire_regions.is_empty() {
                None
            } else {
                Some(wire_regions)
            },
            next_token: new_next_token,
        };
        wire::serialize_list_regions_response(&response)
    }
}

/// Convert state AlternateContact to wire AlternateContact.
fn to_alternate_contact_output(contact: &AlternateContact) -> wire::AlternateContact {
    wire::AlternateContact {
        alternate_contact_type: Some(contact.alternate_contact_type.clone()),
        email_address: Some(contact.email_address.clone()),
        name: Some(contact.name.clone()),
        phone_number: Some(contact.phone_number.clone()),
        title: Some(contact.title.clone()),
    }
}

/// Convert state ContactInformation to wire ContactInformation.
fn to_contact_information_output(info: &ContactInformation) -> wire::ContactInformation {
    wire::ContactInformation {
        address_line1: info.address_line1.clone(),
        address_line2: info.address_line2.clone(),
        address_line3: info.address_line3.clone(),
        city: info.city.clone(),
        company_name: info.company_name.clone(),
        country_code: info.country_code.clone(),
        district_or_county: info.district_or_county.clone(),
        full_name: info.full_name.clone(),
        phone_number: info.phone_number.clone(),
        postal_code: info.postal_code.clone(),
        state_or_region: info.state_or_region.clone(),
        website_url: info.website_url.clone(),
    }
}

fn account_error_response(err: &AccountError) -> MockResponse {
    let (status, error_type) = match err {
        AccountError::InvalidAlternateContactType { .. } => (400, "ValidationException"),
        AccountError::AlternateContactNotFound { .. } => (404, "ResourceNotFoundException"),
        AccountError::ContactInformationNotFound => (404, "ResourceNotFoundException"),
        AccountError::OtpOrEmailMismatch => (400, "ValidationException"),
        AccountError::NoPendingEmailUpdate => (404, "ResourceNotFoundException"),
        AccountError::RegionEnabledByDefaultCannotOptIn { .. } => (400, "ValidationException"),
        AccountError::RegionEnabledByDefaultCannotDisable { .. } => (400, "ValidationException"),
    };
    let body = json!({
        "message": err.to_string(),
    });
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}
