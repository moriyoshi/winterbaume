use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
    extract_path, percent_decode, rest_json_error,
};

use crate::state::{TaxSettingsError, TaxSettingsState};
use crate::views::TaxSettingsStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct TaxSettingsService {
    pub(crate) state: Arc<BackendState<TaxSettingsState>>,
    pub(crate) notifier: StateChangeNotifier<TaxSettingsStateView>,
}

impl TaxSettingsService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for TaxSettingsService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for TaxSettingsService {
    fn service_name(&self) -> &str {
        "tax"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://tax\..*\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

type SharedState = Arc<tokio::sync::RwLock<TaxSettingsState>>;

impl TaxSettingsService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let segments: Vec<String> = path
            .trim_start_matches('/')
            .split('/')
            .filter(|s| !s.is_empty())
            .map(percent_decode)
            .collect();
        let segs: Vec<&str> = segments.iter().map(|s| s.as_str()).collect();

        let raw_query = extract_query(&request.uri);
        let query_map: HashMap<String, String> = winterbaume_core::parse_query_string(&raw_query);

        let response = match segs.as_slice() {
            ["BatchDeleteTaxRegistration"] => {
                self.handle_batch_delete(&state, &request, &[], &query_map)
                    .await
            }
            ["BatchGetTaxExemptions"] => {
                self.handle_batch_get_exemptions(&state, &request, &[], &query_map)
                    .await
            }
            ["BatchPutTaxRegistration"] => {
                self.handle_batch_put(&state, account_id, &request, &[], &query_map)
                    .await
            }
            ["DeleteSupplementalTaxRegistration"] => {
                self.handle_delete_supplemental(&state, account_id, &request, &[], &query_map)
                    .await
            }
            ["DeleteTaxRegistration"] => {
                self.handle_delete_registration(&state, account_id, &request, &[], &query_map)
                    .await
            }
            ["GetTaxExemptionTypes"] => {
                self.handle_get_exemption_types(&request, &[], &query_map)
                    .await
            }
            ["GetTaxInheritance"] => {
                self.handle_get_inheritance(&state, &request, &[], &query_map)
                    .await
            }
            ["GetTaxRegistration"] => {
                self.handle_get_registration(&state, account_id, &request, &[], &query_map)
                    .await
            }
            ["GetTaxRegistrationDocument"] => {
                self.handle_get_registration_document(&request, &[], &query_map)
                    .await
            }
            ["ListSupplementalTaxRegistrations"] => {
                self.handle_list_supplemental(&state, account_id, &request, &[], &query_map)
                    .await
            }
            ["ListTaxExemptions"] => {
                self.handle_list_exemptions(&state, &request, &[], &query_map)
                    .await
            }
            ["ListTaxRegistrations"] => {
                self.handle_list_registrations(&state, &request, &[], &query_map)
                    .await
            }
            ["PutSupplementalTaxRegistration"] => {
                self.handle_put_supplemental(&state, account_id, &request, &[], &query_map)
                    .await
            }
            ["PutTaxExemption"] => {
                self.handle_put_exemption(&state, &request, &[], &query_map)
                    .await
            }
            ["PutTaxInheritance"] => {
                self.handle_put_inheritance(&state, &request, &[], &query_map)
                    .await
            }
            ["PutTaxRegistration"] => {
                self.handle_put_registration(&state, account_id, &request, &[], &query_map)
                    .await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        };

        use winterbaume_core::StatefulService;
        let read_only = matches!(
            segs.as_slice(),
            ["BatchGetTaxExemptions"]
                | ["GetTaxExemptionTypes"]
                | ["GetTaxInheritance"]
                | ["GetTaxRegistration"]
                | ["GetTaxRegistrationDocument"]
                | ["ListSupplementalTaxRegistrations"]
                | ["ListTaxExemptions"]
                | ["ListTaxRegistrations"]
        );
        if response.status / 100 == 2 && !read_only {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_put_registration(
        &self,
        state: &SharedState,
        default_account_id: &str,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_put_tax_registration_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        // PutTaxRegistration requires `taxRegistrationEntry`. The wire deserializer
        // populates a default `TaxRegistrationEntry` if missing; surface the validation
        // error when the request body did not include it.
        if request.body.is_empty() || !raw_body_has_field(&request.body, "taxRegistrationEntry") {
            return rest_json_error(
                400,
                "ValidationException",
                "taxRegistrationEntry is required",
            );
        }
        let account = input
            .account_id
            .as_deref()
            .unwrap_or(default_account_id)
            .to_string();
        // Re-serialize the typed entry to a JSON Value for state storage.
        let entry = serde_json::to_value(&input.tax_registration_entry).unwrap_or(Value::Null);
        let mut state = state.write().await;
        state.put_registration(&account, entry);
        wire::serialize_put_tax_registration_response(&wire::PutTaxRegistrationResponse {
            status: Some("Verified".to_string()),
        })
    }

    async fn handle_get_registration(
        &self,
        state: &SharedState,
        default_account_id: &str,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_tax_registration_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let account = input.account_id.as_deref().unwrap_or(default_account_id);
        let state = state.read().await;
        let reg = state.registrations.get(account);
        wire::serialize_get_tax_registration_response(&wire::GetTaxRegistrationResponse {
            tax_registration: reg.and_then(|v| serde_json::from_value(v.clone()).ok()),
        })
    }

    async fn handle_delete_registration(
        &self,
        state: &SharedState,
        default_account_id: &str,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_tax_registration_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let account = input.account_id.as_deref().unwrap_or(default_account_id);
        let mut state = state.write().await;
        match state.delete_registration(account) {
            Ok(()) => wire::serialize_delete_tax_registration_response(
                &wire::DeleteTaxRegistrationResponse {},
            ),
            Err(e) => err_response(&e),
        }
    }

    async fn handle_list_registrations(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_list_tax_registrations_request(request, labels, query) {
            return rest_json_error(400, "ValidationException", &e);
        }
        let state = state.read().await;
        let entries: Vec<wire::AccountDetails> = state
            .list_registrations()
            .into_iter()
            .map(|(account, raw)| {
                let tax_registration = serde_json::from_value(raw.clone())
                    .ok()
                    // Coerce None tax_registration into Some(default) so the response is always populated.
                    .or_else(|| Some(wire::TaxRegistrationWithJurisdiction::default()));
                wire::AccountDetails {
                    account_id: Some(account.clone()),
                    account_meta_data: None,
                    tax_inheritance_details: None,
                    tax_registration,
                }
            })
            .collect();
        wire::serialize_list_tax_registrations_response(&wire::ListTaxRegistrationsResponse {
            account_details: Some(entries),
            next_token: None,
        })
    }

    async fn handle_batch_put(
        &self,
        state: &SharedState,
        _default_account_id: &str,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_batch_put_tax_registration_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        // Re-serialize the typed entry to a JSON Value for state storage.
        let entry = serde_json::to_value(&input.tax_registration_entry).unwrap_or(json!({}));
        let mut state = state.write().await;
        for id in &input.account_ids {
            state.put_registration(id, entry.clone());
        }
        wire::serialize_batch_put_tax_registration_response(
            &wire::BatchPutTaxRegistrationResponse {
                errors: Some(vec![]),
                status: Some("Verified".to_string()),
            },
        )
    }

    async fn handle_batch_delete(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_batch_delete_tax_registration_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let mut errors: Vec<wire::BatchDeleteTaxRegistrationError> = vec![];
        let mut state = state.write().await;
        for id in &input.account_ids {
            if state.delete_registration(id).is_err() {
                errors.push(wire::BatchDeleteTaxRegistrationError {
                    account_id: Some(id.clone()),
                    code: Some("NOT_FOUND".to_string()),
                    message: Some(format!("No tax registration for account {id}")),
                });
            }
        }
        wire::serialize_batch_delete_tax_registration_response(
            &wire::BatchDeleteTaxRegistrationResponse {
                errors: if errors.is_empty() {
                    None
                } else {
                    Some(errors)
                },
            },
        )
    }

    async fn handle_put_supplemental(
        &self,
        state: &SharedState,
        default_account_id: &str,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_put_supplemental_tax_registration_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        // PutSupplementalTaxRegistration requires `taxRegistrationEntry`. The wire
        // deserializer populates a default if missing; surface the validation error
        // when the request body did not include it.
        if request.body.is_empty() || !raw_body_has_field(&request.body, "taxRegistrationEntry") {
            return rest_json_error(
                400,
                "ValidationException",
                "taxRegistrationEntry is required",
            );
        }
        // Re-serialize the typed entry to a JSON Value, then append metadata fields.
        let entry = serde_json::to_value(&input.tax_registration_entry).unwrap_or(Value::Null);
        let authority_id = format!("auth-{}", uuid::Uuid::new_v4().simple());
        let mut value = entry;
        if let Some(obj) = value.as_object_mut() {
            obj.insert(
                "authorityId".to_string(),
                Value::String(authority_id.clone()),
            );
            obj.insert("status".to_string(), Value::String("Verified".to_string()));
        }
        let mut state = state.write().await;
        state.put_supplemental(default_account_id, &authority_id, value);
        wire::serialize_put_supplemental_tax_registration_response(
            &wire::PutSupplementalTaxRegistrationResponse {
                authority_id: Some(authority_id),
                status: Some("Verified".to_string()),
            },
        )
    }

    async fn handle_delete_supplemental(
        &self,
        state: &SharedState,
        default_account_id: &str,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_supplemental_tax_registration_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.authority_id.is_empty() {
            return rest_json_error(400, "ValidationException", "authorityId is required");
        }
        let mut state = state.write().await;
        match state.delete_supplemental(default_account_id, &input.authority_id) {
            Ok(()) => wire::serialize_delete_supplemental_tax_registration_response(
                &wire::DeleteSupplementalTaxRegistrationResponse {},
            ),
            Err(e) => err_response(&e),
        }
    }

    async fn handle_list_supplemental(
        &self,
        state: &SharedState,
        default_account_id: &str,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) =
            wire::deserialize_list_supplemental_tax_registrations_request(request, labels, query)
        {
            return rest_json_error(400, "ValidationException", &e);
        }
        let state = state.read().await;
        let entries: Vec<wire::SupplementalTaxRegistration> = state
            .list_supplemental(default_account_id)
            .into_iter()
            .filter_map(|v| serde_json::from_value(v.clone()).ok())
            .collect();
        wire::serialize_list_supplemental_tax_registrations_response(
            &wire::ListSupplementalTaxRegistrationsResponse {
                next_token: None,
                tax_registrations: Some(entries),
            },
        )
    }

    async fn handle_get_inheritance(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_get_tax_inheritance_request(request, labels, query) {
            return rest_json_error(400, "ValidationException", &e);
        }
        let state = state.read().await;
        wire::serialize_get_tax_inheritance_response(&wire::GetTaxInheritanceResponse {
            heritage_status: if state.inheritance_state.is_empty() {
                Some("OptOut".to_string())
            } else {
                Some(state.inheritance_state.clone())
            },
        })
    }

    async fn handle_put_inheritance(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_put_tax_inheritance_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let status = input.heritage_status.as_deref().unwrap_or("");
        if status.is_empty() {
            return rest_json_error(400, "ValidationException", "heritageStatus is required");
        }
        let mut state = state.write().await;
        state.set_inheritance(status.to_string());
        wire::serialize_put_tax_inheritance_response(&wire::PutTaxInheritanceResponse {})
    }

    async fn handle_put_exemption(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_put_tax_exemption_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.account_ids.is_empty() {
            return rest_json_error(400, "ValidationException", "accountIds is required");
        }
        // Re-serialize the typed sub-fields to JSON Values for state storage; this
        // preserves the same shape state.put_exemption expects without re-reading the
        // raw request body.
        let authority = serde_json::to_value(&input.authority).unwrap_or(json!({}));
        let exemption_certificate =
            serde_json::to_value(&input.exemption_certificate).unwrap_or(json!({}));
        let exemption_type = if input.exemption_type.is_empty() {
            Value::Null
        } else {
            Value::String(input.exemption_type.clone())
        };
        let exemption = json!({
            "authority": authority,
            "exemptionCertificate": exemption_certificate,
            "exemptionType": exemption_type,
            "status": "Valid",
        });
        let mut state = state.write().await;
        for id in &input.account_ids {
            state.put_exemption(
                id,
                json!({
                    "taxExemptions": [exemption.clone()],
                }),
            );
        }
        wire::serialize_put_tax_exemption_response(&wire::PutTaxExemptionResponse {
            case_id: Some(format!("case-{}", uuid::Uuid::new_v4().simple())),
        })
    }

    async fn handle_batch_get_exemptions(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_batch_get_tax_exemptions_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        let mut map: HashMap<String, wire::TaxExemptionDetails> = HashMap::new();
        let mut failed: Vec<String> = vec![];
        for id in input.account_ids {
            match state.get_exemption(&id) {
                Some(v) => {
                    if let Ok(parsed) = serde_json::from_value(v.clone()) {
                        map.insert(id, parsed);
                    } else {
                        failed.push(id);
                    }
                }
                None => failed.push(id),
            }
        }
        wire::serialize_batch_get_tax_exemptions_response(&wire::BatchGetTaxExemptionsResponse {
            failed_accounts: if failed.is_empty() {
                None
            } else {
                Some(failed)
            },
            tax_exemption_details_map: if map.is_empty() { None } else { Some(map) },
        })
    }

    async fn handle_list_exemptions(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_list_tax_exemptions_request(request, labels, query) {
            return rest_json_error(400, "ValidationException", &e);
        }
        let state = state.read().await;
        let mut map: HashMap<String, wire::TaxExemptionDetails> = HashMap::new();
        for (id, raw) in state.list_exemptions() {
            if let Ok(parsed) = serde_json::from_value(raw.clone()) {
                map.insert(id.clone(), parsed);
            }
        }
        wire::serialize_list_tax_exemptions_response(&wire::ListTaxExemptionsResponse {
            next_token: None,
            tax_exemption_details_map: if map.is_empty() { None } else { Some(map) },
        })
    }

    // STUB[no-state]: Exemption type catalogue is AWS-managed; no create API exists to populate
    //   mock state. Returns an empty list.
    async fn handle_get_exemption_types(
        &self,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_get_tax_exemption_types_request(request, labels, query) {
            return rest_json_error(400, "ValidationException", &e);
        }
        wire::serialize_get_tax_exemption_types_response(&wire::GetTaxExemptionTypesResponse {
            tax_exemption_types: Some(vec![]),
        })
    }

    // STUB[s3-integration]: Document generation requires real S3 bucket access to produce a
    //   presigned URL; returns a mock URL instead.
    async fn handle_get_registration_document(
        &self,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) =
            wire::deserialize_get_tax_registration_document_request(request, labels, query)
        {
            return rest_json_error(400, "ValidationException", &e);
        }
        wire::serialize_get_tax_registration_document_response(
            &wire::GetTaxRegistrationDocumentResponse {
                destination_file_path: None,
                presigned_s3_url: Some(
                    "https://example.s3.us-east-1.amazonaws.com/tax-doc?signed=mock".to_string(),
                ),
            },
        )
    }
}

fn extract_query(uri: &str) -> String {
    match uri.find('?') {
        Some(idx) => uri[idx + 1..].to_string(),
        None => String::new(),
    }
}

/// Returns true when the raw JSON body parses to an object containing the given field.
/// Used to surface validation errors for fields the wire deserializer would otherwise
/// silently default.
fn raw_body_has_field(body: &[u8], field: &str) -> bool {
    let value: Value = match serde_json::from_slice(body) {
        Ok(v) => v,
        Err(_) => return false,
    };
    matches!(value.as_object(), Some(m) if m.contains_key(field))
}

fn err_response(err: &TaxSettingsError) -> MockResponse {
    let (status, error_type) = match err {
        TaxSettingsError::RegistrationNotFound { .. }
        | TaxSettingsError::SupplementalNotFound { .. } => (404, "ResourceNotFoundException"),
        TaxSettingsError::Validation { .. } => (400, "ValidationException"),
    };
    let body = json!({"message": err.to_string()});
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}
