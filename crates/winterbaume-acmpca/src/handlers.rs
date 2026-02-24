use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
};

use crate::state::{AcmPcaError, AcmPcaState};
use crate::types::*;
use crate::views::AcmPcaStateView;
use crate::wire;

pub struct AcmPcaService {
    pub(crate) state: Arc<BackendState<AcmPcaState>>,
    pub(crate) notifier: StateChangeNotifier<AcmPcaStateView>,
}

impl AcmPcaService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for AcmPcaService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for AcmPcaService {
    fn service_name(&self) -> &str {
        "acm-pca"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://acm-pca\..*\.amazonaws\.com",
            r"https?://acm-pca\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl AcmPcaService {
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

        // Validate the body is well-formed JSON up-front; the typed deserialisers in
        // `wire` re-parse the bytes per operation.
        if serde_json::from_slice::<Value>(&request.body).is_err() {
            return json_error_response(400, "SerializationException", "Invalid JSON body");
        }
        let body_bytes: &[u8] = &request.body;

        let state = self.state.get(account_id, &region);

        match action.as_str() {
            "CreateCertificateAuthority" => {
                self.handle_create_ca(&state, body_bytes, account_id, &region)
                    .await
            }
            "DescribeCertificateAuthority" => self.handle_describe_ca(&state, body_bytes).await,
            "UpdateCertificateAuthority" => self.handle_update_ca(&state, body_bytes).await,
            "DeleteCertificateAuthority" => self.handle_delete_ca(&state, body_bytes).await,
            "GetCertificateAuthorityCsr" => self.handle_get_ca_csr(&state, body_bytes).await,
            "GetCertificateAuthorityCertificate" => {
                self.handle_get_ca_certificate(&state, body_bytes).await
            }
            "ImportCertificateAuthorityCertificate" => {
                self.handle_import_ca_certificate(&state, body_bytes).await
            }
            "IssueCertificate" => {
                self.handle_issue_certificate(&state, body_bytes, account_id, &region)
                    .await
            }
            "GetCertificate" => self.handle_get_certificate(&state, body_bytes).await,
            "ListTags" => self.handle_list_tags(&state, body_bytes).await,
            "TagCertificateAuthority" => self.handle_tag_ca(&state, body_bytes).await,
            "UntagCertificateAuthority" => self.handle_untag_ca(&state, body_bytes).await,
            "ListCertificateAuthorities" => self.handle_list_cas(&state, body_bytes).await,
            "PutPolicy" => self.handle_put_policy(&state, body_bytes).await,
            "GetPolicy" => self.handle_get_policy(&state, body_bytes).await,
            "DeletePolicy" => self.handle_delete_policy(&state, body_bytes).await,
            "RevokeCertificate" => self.handle_revoke_certificate(&state, body_bytes).await,
            "RestoreCertificateAuthority" => self.handle_restore_ca(&state, body_bytes).await,
            "CreatePermission" => self.handle_create_permission(&state, body_bytes).await,
            "DeletePermission" => self.handle_delete_permission(&state, body_bytes).await,
            "ListPermissions" => self.handle_list_permissions(&state, body_bytes).await,
            "CreateCertificateAuthorityAuditReport" => {
                self.handle_create_audit_report(&state, body_bytes).await
            }
            "DescribeCertificateAuthorityAuditReport" => {
                self.handle_describe_audit_report(&state, body_bytes).await
            }
            _ => json_error_response(
                400,
                "InvalidAction",
                &format!("Could not find operation {action} for ACM PCA"),
            ),
        }
    }

    async fn handle_create_ca(
        &self,
        state: &Arc<tokio::sync::RwLock<AcmPcaState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_certificate_authority_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };

        let ca_config = ca_config_from_wire(&input.certificate_authority_configuration);
        let ca_type = if input.certificate_authority_type.is_empty() {
            "ROOT"
        } else {
            input.certificate_authority_type.as_str()
        };
        let key_storage = input.key_storage_security_standard.as_deref();
        let tags = tags_from_wire(input.tags.unwrap_or_default());

        let mut state = state.write().await;
        match state.create_certificate_authority(
            ca_config,
            ca_type,
            key_storage,
            tags,
            account_id,
            region,
        ) {
            Ok(arn) => wire::serialize_create_certificate_authority_response(
                &wire::CreateCertificateAuthorityResponse {
                    certificate_authority_arn: Some(arn),
                },
            ),
            Err(e) => acmpca_error_response(&e),
        }
    }

    async fn handle_describe_ca(
        &self,
        state: &Arc<tokio::sync::RwLock<AcmPcaState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_certificate_authority_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.certificate_authority_arn.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing CertificateAuthorityArn",
            );
        }

        let state = state.read().await;
        match state.describe_certificate_authority(&input.certificate_authority_arn) {
            Ok(ca) => wire::serialize_describe_certificate_authority_response(
                &wire::DescribeCertificateAuthorityResponse {
                    certificate_authority: Some(ca_to_wire(ca)),
                },
            ),
            Err(e) => acmpca_error_response(&e),
        }
    }

    async fn handle_update_ca(
        &self,
        state: &Arc<tokio::sync::RwLock<AcmPcaState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_certificate_authority_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.certificate_authority_arn.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing CertificateAuthorityArn",
            );
        }

        let revocation_configuration = input
            .revocation_configuration
            .as_ref()
            .map(revocation_configuration_from_wire);

        let mut state = state.write().await;
        match state.update_certificate_authority(
            &input.certificate_authority_arn,
            input.status.as_deref(),
            revocation_configuration,
        ) {
            Ok(()) => wire::serialize_update_certificate_authority_response(),
            Err(e) => acmpca_error_response(&e),
        }
    }

    async fn handle_delete_ca(
        &self,
        state: &Arc<tokio::sync::RwLock<AcmPcaState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_certificate_authority_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.certificate_authority_arn.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing CertificateAuthorityArn",
            );
        }

        let mut state = state.write().await;
        match state.delete_certificate_authority(&input.certificate_authority_arn) {
            Ok(()) => wire::serialize_delete_certificate_authority_response(),
            Err(e) => acmpca_error_response(&e),
        }
    }

    async fn handle_get_ca_csr(
        &self,
        state: &Arc<tokio::sync::RwLock<AcmPcaState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_certificate_authority_csr_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.certificate_authority_arn.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing CertificateAuthorityArn",
            );
        }

        let state = state.read().await;
        match state.get_certificate_authority_csr(&input.certificate_authority_arn) {
            Ok(csr) => wire::serialize_get_certificate_authority_csr_response(
                &wire::GetCertificateAuthorityCsrResponse { csr: Some(csr) },
            ),
            Err(e) => acmpca_error_response(&e),
        }
    }

    async fn handle_get_ca_certificate(
        &self,
        state: &Arc<tokio::sync::RwLock<AcmPcaState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_certificate_authority_certificate_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.certificate_authority_arn.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing CertificateAuthorityArn",
            );
        }

        let state = state.read().await;
        match state.get_certificate_authority_certificate(&input.certificate_authority_arn) {
            Ok((cert, chain)) => wire::serialize_get_certificate_authority_certificate_response(
                &wire::GetCertificateAuthorityCertificateResponse {
                    certificate: Some(cert),
                    certificate_chain: chain,
                },
            ),
            Err(e) => acmpca_error_response(&e),
        }
    }

    async fn handle_import_ca_certificate(
        &self,
        state: &Arc<tokio::sync::RwLock<AcmPcaState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_import_certificate_authority_certificate_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.certificate_authority_arn.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing CertificateAuthorityArn",
            );
        }
        if input.certificate.is_empty() {
            return json_error_response(400, "ValidationException", "Missing Certificate");
        }

        let certificate = decode_certificate_value(&input.certificate);
        let chain = input
            .certificate_chain
            .as_deref()
            .map(decode_certificate_value);

        let mut state = state.write().await;
        match state.import_certificate_authority_certificate(
            &input.certificate_authority_arn,
            &certificate,
            chain.as_deref(),
        ) {
            Ok(()) => wire::serialize_import_certificate_authority_certificate_response(),
            Err(e) => acmpca_error_response(&e),
        }
    }

    async fn handle_issue_certificate(
        &self,
        state: &Arc<tokio::sync::RwLock<AcmPcaState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_issue_certificate_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.certificate_authority_arn.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing CertificateAuthorityArn",
            );
        }
        if input.csr.is_empty() {
            return json_error_response(400, "ValidationException", "Missing Csr");
        }
        // Detect "Validity missing" by checking if both the type and value are empty/zero.
        // Re-parse only this single field to preserve the original strict missing-validity check.
        let raw: Value = serde_json::from_slice(body).unwrap_or(Value::Null);
        if raw.get("Validity").is_none() {
            return json_error_response(400, "ValidationException", "Missing Validity");
        }

        let csr = decode_certificate_value(&input.csr);
        let signing_algorithm = if input.signing_algorithm.is_empty() {
            "SHA256WITHRSA"
        } else {
            input.signing_algorithm.as_str()
        };
        let validity_type = if input.validity.r#type.is_empty() {
            "YEARS"
        } else {
            input.validity.r#type.as_str()
        };
        let validity_value = if input.validity.value == 0 {
            1
        } else {
            input.validity.value
        };
        let template_arn = input.template_arn.as_deref();

        let mut state = state.write().await;
        match state.issue_certificate(
            &input.certificate_authority_arn,
            &csr,
            signing_algorithm,
            validity_value,
            validity_type,
            template_arn,
            account_id,
            region,
        ) {
            Ok(cert_arn) => {
                wire::serialize_issue_certificate_response(&wire::IssueCertificateResponse {
                    certificate_arn: Some(cert_arn),
                })
            }
            Err(e) => acmpca_error_response(&e),
        }
    }

    async fn handle_get_certificate(
        &self,
        state: &Arc<tokio::sync::RwLock<AcmPcaState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_certificate_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.certificate_authority_arn.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing CertificateAuthorityArn",
            );
        }
        if input.certificate_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing CertificateArn");
        }

        let state = state.read().await;
        match state.get_certificate(&input.certificate_authority_arn, &input.certificate_arn) {
            Ok((cert, chain)) => {
                wire::serialize_get_certificate_response(&wire::GetCertificateResponse {
                    certificate: Some(cert),
                    certificate_chain: chain,
                })
            }
            Err(e) => acmpca_error_response(&e),
        }
    }

    async fn handle_list_tags(
        &self,
        state: &Arc<tokio::sync::RwLock<AcmPcaState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.certificate_authority_arn.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing CertificateAuthorityArn",
            );
        }

        let state = state.read().await;
        match state.list_tags(&input.certificate_authority_arn) {
            Ok(tags) => {
                let tag_entries: Vec<wire::Tag> = tags
                    .iter()
                    .map(|t| wire::Tag {
                        key: t.key.clone(),
                        value: Some(t.value.clone()),
                    })
                    .collect();
                wire::serialize_list_tags_response(&wire::ListTagsResponse {
                    tags: Some(tag_entries),
                    next_token: None,
                })
            }
            Err(e) => acmpca_error_response(&e),
        }
    }

    async fn handle_tag_ca(
        &self,
        state: &Arc<tokio::sync::RwLock<AcmPcaState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_tag_certificate_authority_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.certificate_authority_arn.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing CertificateAuthorityArn",
            );
        }
        let tags = tags_from_wire(input.tags);

        let mut state = state.write().await;
        match state.tag_certificate_authority(&input.certificate_authority_arn, tags) {
            Ok(()) => wire::serialize_tag_certificate_authority_response(),
            Err(e) => acmpca_error_response(&e),
        }
    }

    async fn handle_untag_ca(
        &self,
        state: &Arc<tokio::sync::RwLock<AcmPcaState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_untag_certificate_authority_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.certificate_authority_arn.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing CertificateAuthorityArn",
            );
        }
        let tags = tags_from_wire(input.tags);

        let mut state = state.write().await;
        match state.untag_certificate_authority(&input.certificate_authority_arn, tags) {
            Ok(()) => wire::serialize_untag_certificate_authority_response(),
            Err(e) => acmpca_error_response(&e),
        }
    }

    async fn handle_list_cas(
        &self,
        state: &Arc<tokio::sync::RwLock<AcmPcaState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_certificate_authorities_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let max_results = input.max_results.map(|v| v as usize);
        let next_token = input.next_token.as_deref();

        let state = state.read().await;
        let (cas, new_token) =
            match state.list_certificate_authorities_paginated(max_results, next_token) {
                Ok(result) => result,
                Err(e) => return acmpca_error_response(&e),
            };

        let entries: Vec<wire::CertificateAuthority> =
            cas.iter().map(|ca| ca_to_wire(ca)).collect();
        wire::serialize_list_certificate_authorities_response(
            &wire::ListCertificateAuthoritiesResponse {
                certificate_authorities: Some(entries),
                next_token: new_token,
            },
        )
    }

    async fn handle_put_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<AcmPcaState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_put_policy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing ResourceArn");
        }
        if input.policy.is_empty() {
            return json_error_response(400, "ValidationException", "Missing Policy");
        }

        let mut state = state.write().await;
        match state.put_policy(&input.resource_arn, &input.policy) {
            Ok(()) => wire::serialize_put_policy_response(),
            Err(e) => acmpca_error_response(&e),
        }
    }

    async fn handle_get_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<AcmPcaState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_policy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing ResourceArn");
        }

        let state = state.read().await;
        match state.get_policy(&input.resource_arn) {
            Ok(policy) => wire::serialize_get_policy_response(&wire::GetPolicyResponse {
                policy: Some(policy),
            }),
            Err(e) => acmpca_error_response(&e),
        }
    }

    async fn handle_delete_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<AcmPcaState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_policy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing ResourceArn");
        }

        let mut state = state.write().await;
        match state.delete_policy(&input.resource_arn) {
            Ok(()) => wire::serialize_delete_policy_response(),
            Err(e) => acmpca_error_response(&e),
        }
    }

    async fn handle_revoke_certificate(
        &self,
        state: &Arc<tokio::sync::RwLock<AcmPcaState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_revoke_certificate_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.certificate_authority_arn.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing CertificateAuthorityArn",
            );
        }
        if input.certificate_serial.is_empty() {
            return json_error_response(400, "ValidationException", "Missing CertificateSerial");
        }
        let reason = if input.revocation_reason.is_empty() {
            "UNSPECIFIED"
        } else {
            input.revocation_reason.as_str()
        };

        let mut state = state.write().await;
        match state.revoke_certificate(
            &input.certificate_authority_arn,
            &input.certificate_serial,
            reason,
        ) {
            Ok(()) => wire::serialize_revoke_certificate_response(),
            Err(e) => acmpca_error_response(&e),
        }
    }

    async fn handle_restore_ca(
        &self,
        state: &Arc<tokio::sync::RwLock<AcmPcaState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_restore_certificate_authority_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.certificate_authority_arn.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing CertificateAuthorityArn",
            );
        }

        let mut state = state.write().await;
        match state.restore_certificate_authority(&input.certificate_authority_arn) {
            Ok(()) => wire::serialize_restore_certificate_authority_response(),
            Err(e) => acmpca_error_response(&e),
        }
    }

    async fn handle_create_permission(
        &self,
        state: &Arc<tokio::sync::RwLock<AcmPcaState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_create_permission_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.certificate_authority_arn.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing CertificateAuthorityArn",
            );
        }
        if input.principal.is_empty() {
            return json_error_response(400, "ValidationException", "Missing Principal");
        }

        let mut state = state.write().await;
        match state.create_permission(
            &input.certificate_authority_arn,
            &input.principal,
            input.actions,
            input.source_account,
        ) {
            Ok(()) => wire::serialize_create_permission_response(),
            Err(e) => acmpca_error_response(&e),
        }
    }

    async fn handle_delete_permission(
        &self,
        state: &Arc<tokio::sync::RwLock<AcmPcaState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_permission_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.certificate_authority_arn.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing CertificateAuthorityArn",
            );
        }
        if input.principal.is_empty() {
            return json_error_response(400, "ValidationException", "Missing Principal");
        }

        let mut state = state.write().await;
        match state.delete_permission(&input.certificate_authority_arn, &input.principal) {
            Ok(()) => wire::serialize_delete_permission_response(),
            Err(e) => acmpca_error_response(&e),
        }
    }

    async fn handle_list_permissions(
        &self,
        state: &Arc<tokio::sync::RwLock<AcmPcaState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_permissions_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.certificate_authority_arn.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing CertificateAuthorityArn",
            );
        }

        let state = state.read().await;
        match state.list_permissions(&input.certificate_authority_arn) {
            Ok(perms) => {
                let permissions: Vec<wire::Permission> = perms
                    .iter()
                    .map(|p| wire::Permission {
                        certificate_authority_arn: Some(input.certificate_authority_arn.clone()),
                        principal: Some(p.principal.clone()),
                        actions: Some(p.actions.clone()),
                        source_account: p.source_account.clone(),
                        created_at: Some(p.created_at.timestamp() as f64),
                        policy: None,
                    })
                    .collect();
                wire::serialize_list_permissions_response(&wire::ListPermissionsResponse {
                    permissions: Some(permissions),
                    next_token: None,
                })
            }
            Err(e) => acmpca_error_response(&e),
        }
    }

    async fn handle_create_audit_report(
        &self,
        state: &Arc<tokio::sync::RwLock<AcmPcaState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_create_certificate_authority_audit_report_request(body)
        {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.certificate_authority_arn.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing CertificateAuthorityArn",
            );
        }
        if input.s3_bucket_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing S3BucketName");
        }
        let audit_report_response_format = if input.audit_report_response_format.is_empty() {
            "JSON"
        } else {
            input.audit_report_response_format.as_str()
        };

        let mut state = state.write().await;
        match state.create_certificate_authority_audit_report(
            &input.certificate_authority_arn,
            &input.s3_bucket_name,
            audit_report_response_format,
        ) {
            Ok((audit_report_id, s3_key)) => {
                wire::serialize_create_certificate_authority_audit_report_response(
                    &wire::CreateCertificateAuthorityAuditReportResponse {
                        audit_report_id: Some(audit_report_id),
                        s3_key: Some(s3_key),
                    },
                )
            }
            Err(e) => acmpca_error_response(&e),
        }
    }

    async fn handle_describe_audit_report(
        &self,
        state: &Arc<tokio::sync::RwLock<AcmPcaState>>,
        body: &[u8],
    ) -> MockResponse {
        let input =
            match wire::deserialize_describe_certificate_authority_audit_report_request(body) {
                Ok(v) => v,
                Err(e) => return json_error_response(400, "ValidationException", &e),
            };
        if input.certificate_authority_arn.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing CertificateAuthorityArn",
            );
        }
        if input.audit_report_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing AuditReportId");
        }

        let state = state.read().await;
        match state.describe_certificate_authority_audit_report(
            &input.certificate_authority_arn,
            &input.audit_report_id,
        ) {
            Ok(report) => wire::serialize_describe_certificate_authority_audit_report_response(
                &wire::DescribeCertificateAuthorityAuditReportResponse {
                    audit_report_status: Some(report.status.clone()),
                    s3_bucket_name: Some(report.s3_bucket_name.clone()),
                    s3_key: Some(report.s3_key.clone()),
                    created_at: Some(report.created_at.timestamp() as f64),
                },
            ),
            Err(e) => acmpca_error_response(&e),
        }
    }
}

/// Convert a state CertificateAuthority to a wire CertificateAuthority.
fn ca_to_wire(ca: &CertificateAuthority) -> wire::CertificateAuthority {
    let subject = wire::ASN1Subject {
        common_name: ca.ca_config.subject.common_name.clone(),
        country: ca.ca_config.subject.country.clone(),
        state: ca.ca_config.subject.state.clone(),
        organization: ca.ca_config.subject.organization.clone(),
        organizational_unit: ca.ca_config.subject.organizational_unit.clone(),
        locality: ca.ca_config.subject.locality.clone(),
        ..Default::default()
    };

    let ca_config = wire::CertificateAuthorityConfiguration {
        key_algorithm: ca.ca_config.key_algorithm.clone(),
        signing_algorithm: ca.ca_config.signing_algorithm.clone(),
        subject,
        csr_extensions: None,
    };

    let revocation_configuration =
        ca.revocation_configuration
            .as_ref()
            .map(|rc| wire::RevocationConfiguration {
                crl_configuration: rc.crl_configuration.as_ref().map(|crl| {
                    wire::CrlConfiguration {
                        enabled: crl.enabled,
                        s3_object_acl: Some(crl.s3_object_acl.clone()),
                        ..Default::default()
                    }
                }),
                ocsp_configuration: None,
            });

    wire::CertificateAuthority {
        arn: Some(ca.arn.clone()),
        owner_account: Some(ca.owner_account.clone()),
        r#type: Some(ca.ca_type.clone()),
        status: Some(ca.status.clone()),
        created_at: Some(ca.created_at.timestamp() as f64),
        last_state_change_at: ca.last_state_change_at.map(|t| t.timestamp() as f64),
        not_before: ca.not_before.map(|t| t.timestamp() as f64),
        not_after: ca.not_after.map(|t| t.timestamp() as f64),
        certificate_authority_configuration: Some(ca_config),
        key_storage_security_standard: Some(ca.key_storage_security_standard.clone()),
        revocation_configuration,
        ..Default::default()
    }
}

fn ca_config_from_wire(cfg: &wire::CertificateAuthorityConfiguration) -> CaConfiguration {
    let key_algorithm = if cfg.key_algorithm.is_empty() {
        "RSA_2048".to_string()
    } else {
        cfg.key_algorithm.clone()
    };
    let signing_algorithm = if cfg.signing_algorithm.is_empty() {
        "SHA256WITHRSA".to_string()
    } else {
        cfg.signing_algorithm.clone()
    };
    CaConfiguration {
        key_algorithm,
        signing_algorithm,
        subject: CaSubject {
            common_name: cfg.subject.common_name.clone(),
            country: cfg.subject.country.clone(),
            state: cfg.subject.state.clone(),
            organization: cfg.subject.organization.clone(),
            organizational_unit: cfg.subject.organizational_unit.clone(),
            locality: cfg.subject.locality.clone(),
        },
    }
}

fn revocation_configuration_from_wire(
    rev: &wire::RevocationConfiguration,
) -> RevocationConfiguration {
    let crl = rev.crl_configuration.as_ref().map(|crl| CrlConfiguration {
        enabled: crl.enabled,
        s3_object_acl: crl
            .s3_object_acl
            .clone()
            .unwrap_or_else(|| "PUBLIC_READ".to_string()),
    });
    RevocationConfiguration {
        crl_configuration: crl,
    }
}

fn tags_from_wire(tags: Vec<wire::Tag>) -> Vec<Tag> {
    tags.into_iter()
        .map(|t| Tag {
            key: t.key,
            value: t.value.unwrap_or_default(),
        })
        .collect()
}

/// PEM/base64 disambiguation: AWS clients send blobs as base64. If the value
/// already begins with the PEM header, take it verbatim; otherwise attempt to
/// decode base64 and fall back to the raw string.
fn decode_certificate_value(s: &str) -> String {
    if s.starts_with("-----BEGIN") {
        return s.to_string();
    }
    use base64::Engine;
    match base64::engine::general_purpose::STANDARD.decode(s) {
        Ok(bytes) => String::from_utf8(bytes).unwrap_or_else(|_| s.to_string()),
        Err(_) => s.to_string(),
    }
}

fn acmpca_error_response(err: &AcmPcaError) -> MockResponse {
    let (status, error_type) = match err {
        AcmPcaError::ResourceNotFound(_) => (400, "ResourceNotFoundException"),
        AcmPcaError::CertificateNotFound(_) => (400, "ResourceNotFoundException"),
        AcmPcaError::PolicyNotFound(_) => (400, "ResourceNotFoundException"),
        AcmPcaError::PermissionNotFound(_, _) => (400, "ResourceNotFoundException"),
        AcmPcaError::AuditReportNotFound(_, _) => (400, "ResourceNotFoundException"),
        AcmPcaError::InvalidS3ObjectAclInCrlConfiguration(_) => {
            (400, "InvalidS3ObjectAclInCrlConfiguration")
        }
        AcmPcaError::InvalidStateCertificate(_) => (409, "InvalidStateException"),
        AcmPcaError::CertificateAuthorityNotDeleted(_) => (409, "InvalidStateException"),
        AcmPcaError::MalformedCertificate => (400, "MalformedCertificateException"),
        AcmPcaError::CertificateRevoked(_) => (400, "RequestInProgressException"),
        AcmPcaError::InvalidNextToken => (400, "InvalidNextTokenException"),
    };
    let body = json!({
        "__type": error_type,
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
