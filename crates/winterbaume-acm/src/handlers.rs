use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
    StatefulService,
};

use crate::state::{AcmError, AcmState};
use crate::types::Tag;
use crate::views::AcmStateView;
use crate::wire;

pub struct AcmService {
    pub(crate) state: Arc<BackendState<AcmState>>,
    pub(crate) notifier: StateChangeNotifier<AcmStateView>,
}

impl AcmService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for AcmService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for AcmService {
    fn service_name(&self) -> &str {
        "acm"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://acm\..*\.amazonaws\.com",
            r"https?://acm\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl AcmService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;

        // Extract action from X-Amz-Target header
        // Format: "CertificateManager.RequestCertificate"
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

        // Validate that body parses as JSON; typed deserialisers re-parse per operation.
        if serde_json::from_slice::<Value>(&request.body).is_err() {
            return json_error_response(400, "SerializationException", "Invalid JSON body");
        }
        let body_bytes: &[u8] = &request.body;

        let state = self.state.get(account_id, &region);

        let response = match action.as_str() {
            "RequestCertificate" => {
                self.handle_request_certificate(&state, body_bytes, account_id, &region)
                    .await
            }
            "DescribeCertificate" => self.handle_describe_certificate(&state, body_bytes).await,
            "ListCertificates" => self.handle_list_certificates(&state, body_bytes).await,
            "DeleteCertificate" => self.handle_delete_certificate(&state, body_bytes).await,
            "AddTagsToCertificate" => {
                self.handle_add_tags_to_certificate(&state, body_bytes)
                    .await
            }
            "ListTagsForCertificate" => {
                self.handle_list_tags_for_certificate(&state, body_bytes)
                    .await
            }
            "RemoveTagsFromCertificate" => {
                self.handle_remove_tags_from_certificate(&state, body_bytes)
                    .await
            }
            "ImportCertificate" => {
                self.handle_import_certificate(&state, body_bytes, account_id, &region)
                    .await
            }
            "GetCertificate" => self.handle_get_certificate(&state, body_bytes).await,
            "ExportCertificate" => self.handle_export_certificate(&state, body_bytes).await,
            "GetAccountConfiguration" => self.handle_get_account_configuration(&state).await,
            "PutAccountConfiguration" => {
                self.handle_put_account_configuration(&state, body_bytes)
                    .await
            }
            "RenewCertificate" => self.handle_renew_certificate(&state, body_bytes).await,
            "ResendValidationEmail" => {
                self.handle_resend_validation_email(&state, body_bytes)
                    .await
            }
            "RevokeCertificate" => self.handle_revoke_certificate(&state, body_bytes).await,
            "UpdateCertificateOptions" => {
                self.handle_update_certificate_options(&state, body_bytes)
                    .await
            }
            _ => json_error_response(
                400,
                "InvalidAction",
                &format!("Could not find operation {action} for ACM"),
            ),
        };

        if response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_request_certificate(
        &self,
        state: &Arc<tokio::sync::RwLock<AcmState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_request_certificate_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.domain_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'DomainName'");
        }
        let domain_name = input.domain_name.as_str();
        let sans = input.subject_alternative_names.clone().unwrap_or_default();
        let tags = wire_tags_to_types(input.tags.as_deref().unwrap_or(&[]));

        // Validate tags
        for tag in &tags {
            if tag.key.len() > 128 {
                return json_error_response(
                    400,
                    "ValidationException",
                    "Member must have length less than or equal to 128",
                );
            }
            if let Some(ref v) = tag.value {
                if v.len() > 256 {
                    return json_error_response(
                        400,
                        "ValidationException",
                        "Member must have length less than or equal to 256",
                    );
                }
            }
            if tag.key.starts_with("aws:") {
                return json_error_response(
                    400,
                    "ValidationException",
                    "AWS internal tags cannot be changed with this API",
                );
            }
        }

        let certificate_authority_arn = input.certificate_authority_arn.as_deref();
        let idempotency_token = input.idempotency_token.as_deref();

        let mut state = state.write().await;
        match state.request_certificate(
            domain_name,
            sans,
            account_id,
            region,
            tags,
            certificate_authority_arn,
            idempotency_token,
        ) {
            Ok(cert) => {
                wire::serialize_request_certificate_response(&wire::RequestCertificateResponse {
                    certificate_arn: Some(cert.arn.clone()),
                })
            }
            Err(e) => acm_error_response(&e),
        }
    }

    async fn handle_describe_certificate(
        &self,
        state: &Arc<tokio::sync::RwLock<AcmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_certificate_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.certificate_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'CertificateArn'");
        }
        let arn = input.certificate_arn.as_str();

        let state = state.read().await;
        match state.describe_certificate(arn) {
            Ok(cert) => {
                let dvos = if cert.domain_validation_options.is_empty() {
                    None
                } else {
                    Some(
                        cert.domain_validation_options
                            .iter()
                            .map(|dvo| wire::DomainValidation {
                                domain_name: Some(dvo.domain_name.clone()),
                                validation_domain: Some(dvo.validation_domain.clone()),
                                validation_status: Some(dvo.validation_status.clone()),
                                ..Default::default()
                            })
                            .collect(),
                    )
                };

                wire::serialize_describe_certificate_response(&wire::DescribeCertificateResponse {
                    certificate: Some(wire::CertificateDetail {
                        certificate_arn: Some(cert.arn.clone()),
                        domain_name: Some(cert.domain_name.clone()),
                        status: Some(cert.status.clone()),
                        subject_alternative_names: Some(cert.subject_alternative_names.clone()),
                        created_at: Some(cert.created_at.timestamp() as f64),
                        r#type: Some(cert.certificate_type.clone()),
                        issuer: Some(cert.issuer.clone()),
                        key_algorithm: Some(cert.key_algorithm.clone()),
                        renewal_eligibility: Some(cert.renewal_eligibility.clone()),
                        options: Some(wire::CertificateOptions {
                            certificate_transparency_logging_preference: Some(
                                cert.options
                                    .certificate_transparency_logging_preference
                                    .clone(),
                            ),
                            ..Default::default()
                        }),
                        in_use_by: Some(vec![]),
                        domain_validation_options: dvos,
                        certificate_authority_arn: cert.certificate_authority_arn.clone(),
                        not_before: cert.not_before.map(|nb| nb.timestamp() as f64),
                        not_after: cert.not_after.map(|na| na.timestamp() as f64),
                        ..Default::default()
                    }),
                })
            }
            Err(e) => acm_error_response(&e),
        }
    }

    async fn handle_list_certificates(
        &self,
        state: &Arc<tokio::sync::RwLock<AcmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_certificates_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let statuses: Vec<String> = input.certificate_statuses.unwrap_or_default();
        let key_types: Vec<String> = input.includes.and_then(|f| f.key_types).unwrap_or_default();

        let state = state.read().await;
        let certs = state.list_certificates_with_status(&statuses);

        let filtered: Vec<_> = if key_types.is_empty() {
            certs
        } else {
            certs
                .into_iter()
                .filter(|c| key_types.contains(&c.key_algorithm))
                .collect()
        };

        let entries: Vec<wire::CertificateSummary> = filtered
            .iter()
            .map(|c| wire::CertificateSummary {
                certificate_arn: Some(c.arn.clone()),
                domain_name: Some(c.domain_name.clone()),
                status: Some(c.status.clone()),
                r#type: Some(c.certificate_type.clone()),
                key_algorithm: Some(c.key_algorithm.clone()),
                renewal_eligibility: Some(c.renewal_eligibility.clone()),
                in_use: Some(false),
                not_before: c.not_before.map(|nb| nb.timestamp() as f64),
                not_after: c.not_after.map(|na| na.timestamp() as f64),
                ..Default::default()
            })
            .collect();

        wire::serialize_list_certificates_response(&wire::ListCertificatesResponse {
            certificate_summary_list: Some(entries),
            next_token: None,
        })
    }

    async fn handle_delete_certificate(
        &self,
        state: &Arc<tokio::sync::RwLock<AcmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_certificate_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.certificate_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'CertificateArn'");
        }
        let arn = input.certificate_arn.as_str();

        let mut state = state.write().await;
        match state.delete_certificate(arn) {
            Ok(()) => wire::serialize_delete_certificate_response(),
            Err(e) => acm_error_response(&e),
        }
    }

    async fn handle_add_tags_to_certificate(
        &self,
        state: &Arc<tokio::sync::RwLock<AcmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_add_tags_to_certificate_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.certificate_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'CertificateArn'");
        }
        let arn = input.certificate_arn.as_str();
        let tags = wire_tags_to_types(&input.tags);

        let mut state = state.write().await;
        match state.add_tags_to_certificate(arn, tags) {
            Ok(()) => wire::serialize_add_tags_to_certificate_response(),
            Err(e) => acm_error_response(&e),
        }
    }

    async fn handle_list_tags_for_certificate(
        &self,
        state: &Arc<tokio::sync::RwLock<AcmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_certificate_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.certificate_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'CertificateArn'");
        }
        let arn = input.certificate_arn.as_str();

        let state = state.read().await;
        match state.list_tags_for_certificate(arn) {
            Ok(tags) => {
                let tag_entries: Vec<wire::Tag> = tags
                    .iter()
                    .map(|t| wire::Tag {
                        key: t.key.clone(),
                        value: t.value.clone(),
                    })
                    .collect();
                wire::serialize_list_tags_for_certificate_response(
                    &wire::ListTagsForCertificateResponse {
                        tags: Some(tag_entries),
                    },
                )
            }
            Err(e) => acm_error_response(&e),
        }
    }

    async fn handle_remove_tags_from_certificate(
        &self,
        state: &Arc<tokio::sync::RwLock<AcmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_remove_tags_from_certificate_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.certificate_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'CertificateArn'");
        }
        let arn = input.certificate_arn.as_str();
        let tags = wire_tags_to_types(&input.tags);

        let mut state = state.write().await;
        match state.remove_tags_from_certificate(arn, tags) {
            Ok(()) => wire::serialize_remove_tags_from_certificate_response(),
            Err(e) => acm_error_response(&e),
        }
    }

    async fn handle_import_certificate(
        &self,
        state: &Arc<tokio::sync::RwLock<AcmState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_import_certificate_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.certificate.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Certificate'");
        }
        if input.private_key.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'PrivateKey'");
        }
        let certificate = decode_base64_or_string(&input.certificate);
        let private_key = decode_base64_or_string(&input.private_key);
        let certificate_chain = input
            .certificate_chain
            .as_deref()
            .map(decode_base64_or_string);
        let tags = wire_tags_to_types(input.tags.as_deref().unwrap_or(&[]));

        // Validate tags
        for tag in &tags {
            if tag.key.len() > 128 {
                return json_error_response(
                    400,
                    "ValidationException",
                    "Member must have length less than or equal to 128",
                );
            }
            if let Some(ref v) = tag.value {
                if v.len() > 256 {
                    return json_error_response(
                        400,
                        "ValidationException",
                        "Member must have length less than or equal to 256",
                    );
                }
            }
            if tag.key.starts_with("aws:") {
                return json_error_response(
                    400,
                    "ValidationException",
                    "AWS internal tags cannot be changed with this API",
                );
            }
        }

        let mut state = state.write().await;
        match state.import_certificate(
            &certificate,
            &private_key,
            certificate_chain.as_deref(),
            account_id,
            region,
            tags,
        ) {
            Ok(cert) => {
                wire::serialize_import_certificate_response(&wire::ImportCertificateResponse {
                    certificate_arn: Some(cert.arn.clone()),
                })
            }
            Err(e) => acm_error_response(&e),
        }
    }

    async fn handle_get_certificate(
        &self,
        state: &Arc<tokio::sync::RwLock<AcmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_certificate_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.certificate_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'CertificateArn'");
        }
        let arn = input.certificate_arn.as_str();

        let state = state.read().await;
        match state.get_certificate(arn) {
            Ok(cert) => wire::serialize_get_certificate_response(&wire::GetCertificateResponse {
                certificate: cert.certificate_pem.clone(),
                certificate_chain: cert.certificate_chain.clone(),
            }),
            Err(e) => acm_error_response(&e),
        }
    }

    async fn handle_get_account_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<AcmState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let config = state.get_account_configuration();
        wire::serialize_get_account_configuration_response(&wire::GetAccountConfigurationResponse {
            expiry_events: Some(wire::ExpiryEventsConfiguration {
                days_before_expiry: Some(config.days_before_expiry),
            }),
        })
    }

    async fn handle_put_account_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<AcmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_put_account_configuration_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let days_before_expiry = input.expiry_events.and_then(|e| e.days_before_expiry);

        let days = match days_before_expiry {
            Some(d) if d >= 1 => d,
            Some(_) => {
                return json_error_response(
                    400,
                    "ValidationException",
                    "DaysBeforeExpiry must be at least 1",
                );
            }
            None => {
                return json_error_response(
                    400,
                    "ValidationException",
                    "Missing ExpiryEvents.DaysBeforeExpiry",
                );
            }
        };

        let mut state = state.write().await;
        state.put_account_configuration(crate::types::ExpiryEventsConfiguration {
            days_before_expiry: days,
        });
        wire::serialize_put_account_configuration_response()
    }

    async fn handle_export_certificate(
        &self,
        state: &Arc<tokio::sync::RwLock<AcmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_export_certificate_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.certificate_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'CertificateArn'");
        }
        let arn = input.certificate_arn.as_str();
        let passphrase = input.passphrase.as_str();

        if passphrase.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "The passphrase must not be empty.",
            );
        }

        let state = state.read().await;
        match state.export_certificate(arn) {
            Ok(cert) => {
                wire::serialize_export_certificate_response(&wire::ExportCertificateResponse {
                    certificate: Some(cert.certificate_pem.as_deref().unwrap_or("").to_string()),
                    certificate_chain: Some(
                        cert.certificate_chain.as_deref().unwrap_or("").to_string(),
                    ),
                    private_key: Some(cert.private_key.as_deref().unwrap_or("").to_string()),
                })
            }
            Err(e) => acm_error_response(&e),
        }
    }

    async fn handle_renew_certificate(
        &self,
        state: &Arc<tokio::sync::RwLock<AcmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_renew_certificate_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.certificate_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'CertificateArn'");
        }
        let arn = input.certificate_arn.as_str();

        let state = state.read().await;
        match state.renew_certificate(arn) {
            Ok(()) => wire::serialize_renew_certificate_response(),
            Err(e) => acm_error_response(&e),
        }
    }

    async fn handle_resend_validation_email(
        &self,
        state: &Arc<tokio::sync::RwLock<AcmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_resend_validation_email_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.certificate_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'CertificateArn'");
        }
        if input.domain.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Domain'");
        }
        if input.validation_domain.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ValidationDomain'");
        }
        let arn = input.certificate_arn.as_str();
        let domain = input.domain.as_str();
        let validation_domain = input.validation_domain.as_str();

        let state = state.read().await;
        match state.resend_validation_email(arn, domain, validation_domain) {
            Ok(()) => wire::serialize_resend_validation_email_response(),
            Err(e) => acm_error_response(&e),
        }
    }

    async fn handle_revoke_certificate(
        &self,
        state: &Arc<tokio::sync::RwLock<AcmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_revoke_certificate_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.certificate_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'CertificateArn'");
        }
        let arn = input.certificate_arn.as_str();
        let revocation_reason = if input.revocation_reason.is_empty() {
            "UNSPECIFIED"
        } else {
            input.revocation_reason.as_str()
        };

        let mut state = state.write().await;
        match state.revoke_certificate(arn, revocation_reason) {
            Ok(()) => {
                wire::serialize_revoke_certificate_response(&wire::RevokeCertificateResponse {
                    ..Default::default()
                })
            }
            Err(e) => acm_error_response(&e),
        }
    }

    async fn handle_update_certificate_options(
        &self,
        state: &Arc<tokio::sync::RwLock<AcmState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_certificate_options_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.certificate_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'CertificateArn'");
        }
        let arn = input.certificate_arn.as_str();
        let transparency_pref = input
            .options
            .certificate_transparency_logging_preference
            .as_deref();

        let mut state = state.write().await;
        match state.update_certificate_options(arn, transparency_pref) {
            Ok(()) => wire::serialize_update_certificate_options_response(),
            Err(e) => acm_error_response(&e),
        }
    }
}

fn wire_tags_to_types(tags: &[wire::Tag]) -> Vec<Tag> {
    tags.iter()
        .filter(|t| !t.key.is_empty())
        .map(|t| Tag {
            key: t.key.clone(),
            value: t.value.clone(),
        })
        .collect()
}

/// Decode a string that may be base64-encoded (from SDK) or a plain string
fn decode_base64_or_string(s: &str) -> String {
    // The AWS SDK sends certificate data as base64-encoded blob
    use base64::Engine;
    if let Ok(decoded) = base64::engine::general_purpose::STANDARD.decode(s) {
        if let Ok(text) = String::from_utf8(decoded) {
            return text;
        }
    }
    s.to_string()
}

fn acm_error_response(err: &AcmError) -> MockResponse {
    let (status, code) = match err {
        AcmError::ResourceNotFound { .. } => (400, "ResourceNotFoundException"),
        AcmError::RequestInProgress => (400, "RequestInProgressException"),
        AcmError::AwsInternalTagsReadOnly => (400, "ValidationException"),
        AcmError::TagKeyTooLong => (400, "ValidationException"),
        AcmError::TagValueTooLong => (400, "ValidationException"),
        AcmError::TooManyTags { .. } => (400, "TooManyTagsException"),
        AcmError::NotPrivateCertificate { .. } => (400, "ValidationException"),
    };
    let body = json!({
        "__type": code,
        "message": err.to_string(),
    });
    MockResponse::json(status, body.to_string())
}

fn json_error_response(status: u16, code: &str, message: &str) -> MockResponse {
    let body = json!({
        "__type": code,
        "message": message,
    });
    MockResponse::json(status, body.to_string())
}
