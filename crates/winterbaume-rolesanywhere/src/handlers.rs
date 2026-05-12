use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::json;
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, StatefulService,
    default_account_id,
};

use crate::state::{RolesAnywhereError, RolesAnywhereState};
use crate::types::{MappingRule, NotificationSettingDetail, Source, SourceData};
use crate::views::RolesAnywhereStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct RolesAnywhereService {
    pub(crate) state: Arc<BackendState<RolesAnywhereState>>,
    pub(crate) notifier: StateChangeNotifier<RolesAnywhereStateView>,
}

impl RolesAnywhereService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for RolesAnywhereService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for RolesAnywhereService {
    fn service_name(&self) -> &str {
        "rolesanywhere"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://rolesanywhere\..*\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

fn rest_json_error(status: u16, code: &str, message: &str) -> MockResponse {
    let body = json!({ "message": message });
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers.insert(X_AMZN_ERRORTYPE, code.parse().unwrap());
    resp
}

fn service_error_response(e: &RolesAnywhereError) -> MockResponse {
    let (status, error_type) = match e {
        RolesAnywhereError::NotFound { .. } => (404, "ResourceNotFoundException"),
        RolesAnywhereError::Validation { .. } => (400, "ValidationException"),
        RolesAnywhereError::TooManyTags => (400, "TooManyTagsException"),
    };
    rest_json_error(status, error_type, &e.to_string())
}

fn extract_path(uri: &str) -> &str {
    let after_scheme = uri.find("://").map(|i| &uri[i + 3..]).unwrap_or(uri);
    let path_start = after_scheme.find('/').unwrap_or(after_scheme.len());
    let path_end = after_scheme[path_start..]
        .find('?')
        .map(|i| path_start + i)
        .unwrap_or(after_scheme.len());
    &after_scheme[path_start..path_end]
}

fn extract_query_string(uri: &str) -> &str {
    uri.find('?').map(|i| &uri[i + 1..]).unwrap_or("")
}

fn url_decode(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.bytes();
    while let Some(b) = chars.next() {
        if b == b'%' {
            let hi = chars.next().unwrap_or(b'0');
            let lo = chars.next().unwrap_or(b'0');
            let val = hex_val(hi) * 16 + hex_val(lo);
            result.push(val as char);
        } else if b == b'+' {
            result.push(' ');
        } else {
            result.push(b as char);
        }
    }
    result
}

fn hex_val(b: u8) -> u8 {
    match b {
        b'0'..=b'9' => b - b'0',
        b'a'..=b'f' => b - b'a' + 10,
        b'A'..=b'F' => b - b'A' + 10,
        _ => 0,
    }
}

fn tags_from_wire(tags: Option<Vec<wire::Tag>>) -> Vec<crate::types::Tag> {
    tags.unwrap_or_default()
        .into_iter()
        .map(|t| crate::types::Tag {
            key: t.key,
            value: t.value,
        })
        .collect()
}

fn source_from_wire(s: &wire::Source) -> Source {
    let source_data = s.source_data.as_ref().and_then(|sd| {
        if let Some(cert) = sd.x509_certificate_data.as_ref() {
            Some(SourceData::X509CertificateData(cert.clone()))
        } else {
            sd.acm_pca_arn
                .as_ref()
                .map(|arn| SourceData::AcmPcaArn(arn.clone()))
        }
    });
    Source {
        source_type: s.source_type.clone(),
        source_data,
    }
}

fn notification_settings_from_wire(
    settings: Option<Vec<wire::NotificationSetting>>,
) -> Vec<NotificationSettingDetail> {
    settings
        .unwrap_or_default()
        .into_iter()
        .map(|ns| NotificationSettingDetail {
            enabled: ns.enabled,
            event: ns.event,
            threshold: ns.threshold,
            channel: ns.channel,
            configured_by: None,
        })
        .collect()
}

fn profile_to_wire(p: &crate::types::Profile) -> wire::ProfileDetail {
    wire::ProfileDetail {
        profile_id: Some(p.profile_id.clone()),
        profile_arn: Some(p.profile_arn.clone()),
        name: Some(p.name.clone()),
        enabled: Some(p.enabled),
        role_arns: Some(p.role_arns.clone()),
        managed_policy_arns: Some(p.managed_policy_arns.clone()),
        session_policy: p.session_policy.clone(),
        duration_seconds: p.duration_seconds,
        require_instance_properties: p.require_instance_properties,
        accept_role_session_name: p.accept_role_session_name,
        attribute_mappings: if p.attribute_mappings.is_empty() {
            None
        } else {
            Some(
                p.attribute_mappings
                    .iter()
                    .map(|am| wire::AttributeMapping {
                        certificate_field: Some(am.certificate_field.clone()),
                        mapping_rules: Some(
                            am.mapping_rules
                                .iter()
                                .map(|mr| wire::MappingRule {
                                    specifier: mr.specifier.clone(),
                                    ..Default::default()
                                })
                                .collect(),
                        ),
                        ..Default::default()
                    })
                    .collect(),
            )
        },
        created_by: p.created_by.clone(),
        created_at: Some(p.created_at.to_rfc3339()),
        updated_at: Some(p.updated_at.to_rfc3339()),
        ..Default::default()
    }
}

fn trust_anchor_to_wire(ta: &crate::types::TrustAnchor) -> wire::TrustAnchorDetail {
    wire::TrustAnchorDetail {
        trust_anchor_id: Some(ta.trust_anchor_id.clone()),
        trust_anchor_arn: Some(ta.trust_anchor_arn.clone()),
        name: Some(ta.name.clone()),
        source: Some(wire::Source {
            source_type: ta.source.source_type.clone(),
            source_data: ta.source.source_data.as_ref().map(|sd| match sd {
                SourceData::X509CertificateData(data) => wire::SourceData {
                    x509_certificate_data: Some(data.clone()),
                    ..Default::default()
                },
                SourceData::AcmPcaArn(arn) => wire::SourceData {
                    acm_pca_arn: Some(arn.clone()),
                    ..Default::default()
                },
            }),
            ..Default::default()
        }),
        enabled: Some(ta.enabled),
        notification_settings: if ta.notification_settings.is_empty() {
            None
        } else {
            Some(
                ta.notification_settings
                    .iter()
                    .map(|ns| wire::NotificationSettingDetail {
                        enabled: Some(ns.enabled),
                        event: Some(ns.event.clone()),
                        threshold: ns.threshold,
                        channel: ns.channel.clone(),
                        configured_by: ns.configured_by.clone(),
                        ..Default::default()
                    })
                    .collect(),
            )
        },
        created_at: Some(ta.created_at.to_rfc3339()),
        updated_at: Some(ta.updated_at.to_rfc3339()),
        ..Default::default()
    }
}

fn crl_to_wire(c: &crate::types::Crl) -> wire::CrlDetail {
    use base64::Engine;
    wire::CrlDetail {
        crl_id: Some(c.crl_id.clone()),
        crl_arn: Some(c.crl_arn.clone()),
        name: Some(c.name.clone()),
        enabled: Some(c.enabled),
        crl_data: Some(base64::engine::general_purpose::STANDARD.encode(&c.crl_data)),
        trust_anchor_arn: Some(c.trust_anchor_arn.clone()),
        created_at: Some(c.created_at.to_rfc3339()),
        updated_at: Some(c.updated_at.to_rfc3339()),
        ..Default::default()
    }
}

impl RolesAnywhereService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let query_string = extract_query_string(&request.uri);
        let method = request.method.as_str();

        let segments: Vec<String> = path
            .trim_start_matches('/')
            .split('/')
            .filter(|s| !s.is_empty())
            .map(url_decode)
            .collect();
        let segments_ref: Vec<&str> = segments.iter().map(|s| s.as_str()).collect();

        let query_map: HashMap<String, String> = winterbaume_core::parse_query_string(query_string);

        let response = match (method, segments_ref.as_slice()) {
            // Profile operations
            ("POST", ["profiles"]) => {
                self.handle_create_profile(&state, &request, &[], &query_map, account_id, &region)
                    .await
            }
            ("GET", ["profiles"]) => self.handle_list_profiles(&state).await,
            ("GET", ["profile", profile_id]) => {
                let labels: &[(&str, &str)] = &[("profileId", profile_id)];
                self.handle_get_profile(&state, &request, labels, &query_map)
                    .await
            }
            ("PATCH", ["profile", profile_id]) => {
                let labels: &[(&str, &str)] = &[("profileId", profile_id)];
                self.handle_update_profile(&state, &request, labels, &query_map)
                    .await
            }
            ("DELETE", ["profile", profile_id]) => {
                let labels: &[(&str, &str)] = &[("profileId", profile_id)];
                self.handle_delete_profile(&state, &request, labels, &query_map)
                    .await
            }
            ("POST", ["profile", profile_id, "enable"]) => {
                let labels: &[(&str, &str)] = &[("profileId", profile_id)];
                self.handle_enable_profile(&state, &request, labels, &query_map)
                    .await
            }
            ("POST", ["profile", profile_id, "disable"]) => {
                let labels: &[(&str, &str)] = &[("profileId", profile_id)];
                self.handle_disable_profile(&state, &request, labels, &query_map)
                    .await
            }
            ("PUT", ["profiles", profile_id, "mappings"]) => {
                let labels: &[(&str, &str)] = &[("profileId", profile_id)];
                self.handle_put_attribute_mapping(&state, &request, labels, &query_map)
                    .await
            }
            ("DELETE", ["profiles", profile_id, "mappings"]) => {
                let labels: &[(&str, &str)] = &[("profileId", profile_id)];
                self.handle_delete_attribute_mapping(&state, &request, labels, &query_map)
                    .await
            }

            // Trust Anchor operations
            ("POST", ["trustanchors"]) => {
                self.handle_create_trust_anchor(
                    &state,
                    &request,
                    &[],
                    &query_map,
                    account_id,
                    &region,
                )
                .await
            }
            ("GET", ["trustanchors"]) => self.handle_list_trust_anchors(&state).await,
            ("GET", ["trustanchor", trust_anchor_id]) => {
                let labels: &[(&str, &str)] = &[("trustAnchorId", trust_anchor_id)];
                self.handle_get_trust_anchor(&state, &request, labels, &query_map)
                    .await
            }
            ("PATCH", ["trustanchor", trust_anchor_id]) => {
                let labels: &[(&str, &str)] = &[("trustAnchorId", trust_anchor_id)];
                self.handle_update_trust_anchor(&state, &request, labels, &query_map)
                    .await
            }
            ("DELETE", ["trustanchor", trust_anchor_id]) => {
                let labels: &[(&str, &str)] = &[("trustAnchorId", trust_anchor_id)];
                self.handle_delete_trust_anchor(&state, &request, labels, &query_map)
                    .await
            }
            ("POST", ["trustanchor", trust_anchor_id, "enable"]) => {
                let labels: &[(&str, &str)] = &[("trustAnchorId", trust_anchor_id)];
                self.handle_enable_trust_anchor(&state, &request, labels, &query_map)
                    .await
            }
            ("POST", ["trustanchor", trust_anchor_id, "disable"]) => {
                let labels: &[(&str, &str)] = &[("trustAnchorId", trust_anchor_id)];
                self.handle_disable_trust_anchor(&state, &request, labels, &query_map)
                    .await
            }

            // Notification settings
            ("PATCH", ["put-notifications-settings"]) => {
                self.handle_put_notification_settings(&state, &request, &[], &query_map, account_id)
                    .await
            }
            ("PATCH", ["reset-notifications-settings"]) => {
                self.handle_reset_notification_settings(&state, &request, &[], &query_map)
                    .await
            }

            // CRL operations
            ("POST", ["crls"]) => {
                self.handle_import_crl(&state, &request, &[], &query_map, account_id, &region)
                    .await
            }
            ("GET", ["crls"]) => self.handle_list_crls(&state).await,
            ("GET", ["crl", crl_id]) => {
                let labels: &[(&str, &str)] = &[("crlId", crl_id)];
                self.handle_get_crl(&state, &request, labels, &query_map)
                    .await
            }
            ("PATCH", ["crl", crl_id]) => {
                let labels: &[(&str, &str)] = &[("crlId", crl_id)];
                self.handle_update_crl(&state, &request, labels, &query_map)
                    .await
            }
            ("DELETE", ["crl", crl_id]) => {
                let labels: &[(&str, &str)] = &[("crlId", crl_id)];
                self.handle_delete_crl(&state, &request, labels, &query_map)
                    .await
            }
            ("POST", ["crl", crl_id, "enable"]) => {
                let labels: &[(&str, &str)] = &[("crlId", crl_id)];
                self.handle_enable_crl(&state, &request, labels, &query_map)
                    .await
            }
            ("POST", ["crl", crl_id, "disable"]) => {
                let labels: &[(&str, &str)] = &[("crlId", crl_id)];
                self.handle_disable_crl(&state, &request, labels, &query_map)
                    .await
            }

            // Subject operations
            ("GET", ["subjects"]) => self.handle_list_subjects(&state).await,
            ("GET", ["subject", subject_id]) => self.handle_get_subject(subject_id).await,

            // Tag operations
            ("POST", ["TagResource"]) => {
                self.handle_tag_resource(&state, &request, &[], &query_map)
                    .await
            }
            ("POST", ["UntagResource"]) => {
                self.handle_untag_resource(&state, &request, &[], &query_map)
                    .await
            }
            ("GET", ["ListTagsForResource"]) => {
                self.handle_list_tags_for_resource(&state, &request, &[], &query_map)
                    .await
            }

            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        };

        // Notify state changes for mutating operations
        if response.status / 100 == 2 {
            let is_mutating = matches!(method, "POST" | "PUT" | "PATCH" | "DELETE");
            if is_mutating {
                self.notify_state_changed(account_id, &region).await;
            }
        }
        response
    }

    // Profile handlers

    async fn handle_create_profile(
        &self,
        state: &Arc<tokio::sync::RwLock<RolesAnywhereState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_profile_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'name'");
        }
        let tags = tags_from_wire(input.tags);

        let mut guard = state.write().await;
        match guard.create_profile(
            &input.name,
            input.role_arns,
            input.managed_policy_arns.unwrap_or_default(),
            input.session_policy,
            input.duration_seconds,
            input.require_instance_properties,
            input.enabled,
            input.accept_role_session_name,
            tags,
            account_id,
            region,
        ) {
            Ok(profile) => wire::serialize_create_profile_response(&wire::ProfileDetailResponse {
                profile: Some(profile_to_wire(profile)),
                ..Default::default()
            }),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_get_profile(
        &self,
        state: &Arc<tokio::sync::RwLock<RolesAnywhereState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_profile_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let guard = state.read().await;
        match guard.get_profile(&input.profile_id) {
            Ok(profile) => wire::serialize_get_profile_response(&wire::ProfileDetailResponse {
                profile: Some(profile_to_wire(profile)),
                ..Default::default()
            }),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_delete_profile(
        &self,
        state: &Arc<tokio::sync::RwLock<RolesAnywhereState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_profile_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut guard = state.write().await;
        match guard.delete_profile(&input.profile_id) {
            Ok(profile) => wire::serialize_delete_profile_response(&wire::ProfileDetailResponse {
                profile: Some(profile_to_wire(&profile)),
                ..Default::default()
            }),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_list_profiles(
        &self,
        state: &Arc<tokio::sync::RwLock<RolesAnywhereState>>,
    ) -> MockResponse {
        let guard = state.read().await;
        let profiles: Vec<wire::ProfileDetail> = guard
            .list_profiles()
            .iter()
            .map(|p| profile_to_wire(p))
            .collect();
        wire::serialize_list_profiles_response(&wire::ListProfilesResponse {
            profiles: Some(profiles),
            ..Default::default()
        })
    }

    async fn handle_update_profile(
        &self,
        state: &Arc<tokio::sync::RwLock<RolesAnywhereState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_profile_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut guard = state.write().await;
        match guard.update_profile(
            &input.profile_id,
            input.name.as_deref(),
            input.session_policy.as_deref(),
            input.role_arns,
            input.managed_policy_arns,
            input.duration_seconds,
            input.accept_role_session_name,
        ) {
            Ok(profile) => wire::serialize_update_profile_response(&wire::ProfileDetailResponse {
                profile: Some(profile_to_wire(profile)),
                ..Default::default()
            }),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_enable_profile(
        &self,
        state: &Arc<tokio::sync::RwLock<RolesAnywhereState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_enable_profile_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut guard = state.write().await;
        match guard.enable_profile(&input.profile_id) {
            Ok(profile) => wire::serialize_enable_profile_response(&wire::ProfileDetailResponse {
                profile: Some(profile_to_wire(profile)),
                ..Default::default()
            }),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_disable_profile(
        &self,
        state: &Arc<tokio::sync::RwLock<RolesAnywhereState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_disable_profile_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut guard = state.write().await;
        match guard.disable_profile(&input.profile_id) {
            Ok(profile) => wire::serialize_disable_profile_response(&wire::ProfileDetailResponse {
                profile: Some(profile_to_wire(profile)),
                ..Default::default()
            }),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_put_attribute_mapping(
        &self,
        state: &Arc<tokio::sync::RwLock<RolesAnywhereState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_put_attribute_mapping_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.certificate_field.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'certificateField'");
        }
        let mapping_rules: Vec<MappingRule> = input
            .mapping_rules
            .into_iter()
            .map(|mr| MappingRule {
                specifier: mr.specifier,
            })
            .collect();

        let mut guard = state.write().await;
        match guard.put_attribute_mapping(
            &input.profile_id,
            &input.certificate_field,
            mapping_rules,
        ) {
            Ok(profile) => {
                wire::serialize_put_attribute_mapping_response(&wire::PutAttributeMappingResponse {
                    profile: Some(profile_to_wire(profile)),
                    ..Default::default()
                })
            }
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_delete_attribute_mapping(
        &self,
        state: &Arc<tokio::sync::RwLock<RolesAnywhereState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_attribute_mapping_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.certificate_field.is_empty() {
            return rest_json_error(
                400,
                "ValidationException",
                "Missing 'certificateField' query parameter",
            );
        }

        let mut guard = state.write().await;
        match guard.delete_attribute_mapping(
            &input.profile_id,
            &input.certificate_field,
            input.specifiers,
        ) {
            Ok(profile) => wire::serialize_delete_attribute_mapping_response(
                &wire::DeleteAttributeMappingResponse {
                    profile: Some(profile_to_wire(profile)),
                    ..Default::default()
                },
            ),
            Err(e) => service_error_response(&e),
        }
    }

    // Trust Anchor handlers

    async fn handle_create_trust_anchor(
        &self,
        state: &Arc<tokio::sync::RwLock<RolesAnywhereState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_trust_anchor_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'name'");
        }
        if input.source.source_type.is_none() && input.source.source_data.is_none() {
            return rest_json_error(400, "ValidationException", "Missing 'source'");
        }
        let source = source_from_wire(&input.source);
        let tags = tags_from_wire(input.tags);
        let notification_settings = notification_settings_from_wire(input.notification_settings);

        let mut guard = state.write().await;
        match guard.create_trust_anchor(
            &input.name,
            source,
            input.enabled,
            tags,
            notification_settings,
            account_id,
            region,
        ) {
            Ok(ta) => {
                wire::serialize_create_trust_anchor_response(&wire::TrustAnchorDetailResponse {
                    trust_anchor: Some(trust_anchor_to_wire(ta)),
                    ..Default::default()
                })
            }
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_get_trust_anchor(
        &self,
        state: &Arc<tokio::sync::RwLock<RolesAnywhereState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_trust_anchor_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let guard = state.read().await;
        match guard.get_trust_anchor(&input.trust_anchor_id) {
            Ok(ta) => wire::serialize_get_trust_anchor_response(&wire::TrustAnchorDetailResponse {
                trust_anchor: Some(trust_anchor_to_wire(ta)),
                ..Default::default()
            }),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_delete_trust_anchor(
        &self,
        state: &Arc<tokio::sync::RwLock<RolesAnywhereState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_trust_anchor_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut guard = state.write().await;
        match guard.delete_trust_anchor(&input.trust_anchor_id) {
            Ok(ta) => {
                wire::serialize_delete_trust_anchor_response(&wire::TrustAnchorDetailResponse {
                    trust_anchor: Some(trust_anchor_to_wire(&ta)),
                    ..Default::default()
                })
            }
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_list_trust_anchors(
        &self,
        state: &Arc<tokio::sync::RwLock<RolesAnywhereState>>,
    ) -> MockResponse {
        let guard = state.read().await;
        let anchors: Vec<wire::TrustAnchorDetail> = guard
            .list_trust_anchors()
            .iter()
            .map(|ta| trust_anchor_to_wire(ta))
            .collect();
        wire::serialize_list_trust_anchors_response(&wire::ListTrustAnchorsResponse {
            trust_anchors: Some(anchors),
            ..Default::default()
        })
    }

    async fn handle_update_trust_anchor(
        &self,
        state: &Arc<tokio::sync::RwLock<RolesAnywhereState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_trust_anchor_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let source = input.source.as_ref().map(source_from_wire);

        let mut guard = state.write().await;
        match guard.update_trust_anchor(&input.trust_anchor_id, input.name.as_deref(), source) {
            Ok(ta) => {
                wire::serialize_update_trust_anchor_response(&wire::TrustAnchorDetailResponse {
                    trust_anchor: Some(trust_anchor_to_wire(ta)),
                    ..Default::default()
                })
            }
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_enable_trust_anchor(
        &self,
        state: &Arc<tokio::sync::RwLock<RolesAnywhereState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_enable_trust_anchor_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut guard = state.write().await;
        match guard.enable_trust_anchor(&input.trust_anchor_id) {
            Ok(ta) => {
                wire::serialize_enable_trust_anchor_response(&wire::TrustAnchorDetailResponse {
                    trust_anchor: Some(trust_anchor_to_wire(ta)),
                    ..Default::default()
                })
            }
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_disable_trust_anchor(
        &self,
        state: &Arc<tokio::sync::RwLock<RolesAnywhereState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_disable_trust_anchor_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut guard = state.write().await;
        match guard.disable_trust_anchor(&input.trust_anchor_id) {
            Ok(ta) => {
                wire::serialize_disable_trust_anchor_response(&wire::TrustAnchorDetailResponse {
                    trust_anchor: Some(trust_anchor_to_wire(ta)),
                    ..Default::default()
                })
            }
            Err(e) => service_error_response(&e),
        }
    }

    // Notification settings handlers

    async fn handle_put_notification_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<RolesAnywhereState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
    ) -> MockResponse {
        let input =
            match wire::deserialize_put_notification_settings_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        if input.trust_anchor_id.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'trustAnchorId'");
        }
        let settings = notification_settings_from_wire(Some(input.notification_settings));

        let mut guard = state.write().await;
        match guard.put_notification_settings(&input.trust_anchor_id, settings, account_id) {
            Ok(ta) => wire::serialize_put_notification_settings_response(
                &wire::PutNotificationSettingsResponse {
                    trust_anchor: Some(trust_anchor_to_wire(ta)),
                    ..Default::default()
                },
            ),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_reset_notification_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<RolesAnywhereState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_reset_notification_settings_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        if input.trust_anchor_id.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'trustAnchorId'");
        }
        let keys: Vec<(String, Option<String>)> = input
            .notification_setting_keys
            .into_iter()
            .map(|k| (k.event, k.channel))
            .collect();

        let mut guard = state.write().await;
        match guard.reset_notification_settings(&input.trust_anchor_id, keys) {
            Ok(ta) => wire::serialize_reset_notification_settings_response(
                &wire::ResetNotificationSettingsResponse {
                    trust_anchor: Some(trust_anchor_to_wire(ta)),
                    ..Default::default()
                },
            ),
            Err(e) => service_error_response(&e),
        }
    }

    // CRL handlers

    async fn handle_import_crl(
        &self,
        state: &Arc<tokio::sync::RwLock<RolesAnywhereState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_import_crl_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'name'");
        }
        if input.crl_data.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'crlData'");
        }
        if input.trust_anchor_arn.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'trustAnchorArn'");
        }
        use base64::Engine;
        let crl_data = match base64::engine::general_purpose::STANDARD.decode(&input.crl_data) {
            Ok(d) => d,
            Err(_) => {
                return rest_json_error(400, "ValidationException", "Invalid base64 in 'crlData'");
            }
        };
        let tags = tags_from_wire(input.tags);

        let mut guard = state.write().await;
        match guard.import_crl(
            &input.name,
            crl_data,
            &input.trust_anchor_arn,
            input.enabled,
            tags,
            account_id,
            region,
        ) {
            Ok(crl) => wire::serialize_import_crl_response(&wire::CrlDetailResponse {
                crl: Some(crl_to_wire(crl)),
                ..Default::default()
            }),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_get_crl(
        &self,
        state: &Arc<tokio::sync::RwLock<RolesAnywhereState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_crl_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let guard = state.read().await;
        match guard.get_crl(&input.crl_id) {
            Ok(crl) => wire::serialize_get_crl_response(&wire::CrlDetailResponse {
                crl: Some(crl_to_wire(crl)),
                ..Default::default()
            }),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_delete_crl(
        &self,
        state: &Arc<tokio::sync::RwLock<RolesAnywhereState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_crl_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut guard = state.write().await;
        match guard.delete_crl(&input.crl_id) {
            Ok(crl) => wire::serialize_delete_crl_response(&wire::CrlDetailResponse {
                crl: Some(crl_to_wire(&crl)),
                ..Default::default()
            }),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_list_crls(
        &self,
        state: &Arc<tokio::sync::RwLock<RolesAnywhereState>>,
    ) -> MockResponse {
        let guard = state.read().await;
        let crls: Vec<wire::CrlDetail> = guard.list_crls().iter().map(|c| crl_to_wire(c)).collect();
        wire::serialize_list_crls_response(&wire::ListCrlsResponse {
            crls: Some(crls),
            ..Default::default()
        })
    }

    async fn handle_update_crl(
        &self,
        state: &Arc<tokio::sync::RwLock<RolesAnywhereState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_crl_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let crl_data: Option<Vec<u8>> = input.crl_data.as_deref().and_then(|b64| {
            use base64::Engine;
            base64::engine::general_purpose::STANDARD.decode(b64).ok()
        });

        let mut guard = state.write().await;
        match guard.update_crl(&input.crl_id, input.name.as_deref(), crl_data) {
            Ok(crl) => wire::serialize_update_crl_response(&wire::CrlDetailResponse {
                crl: Some(crl_to_wire(crl)),
                ..Default::default()
            }),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_enable_crl(
        &self,
        state: &Arc<tokio::sync::RwLock<RolesAnywhereState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_enable_crl_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut guard = state.write().await;
        match guard.enable_crl(&input.crl_id) {
            Ok(crl) => wire::serialize_enable_crl_response(&wire::CrlDetailResponse {
                crl: Some(crl_to_wire(crl)),
                ..Default::default()
            }),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_disable_crl(
        &self,
        state: &Arc<tokio::sync::RwLock<RolesAnywhereState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_disable_crl_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut guard = state.write().await;
        match guard.disable_crl(&input.crl_id) {
            Ok(crl) => wire::serialize_disable_crl_response(&wire::CrlDetailResponse {
                crl: Some(crl_to_wire(crl)),
                ..Default::default()
            }),
            Err(e) => service_error_response(&e),
        }
    }

    // Subject handlers

    // STUB[no-auth]: Subjects are created by real authentication attempts; the mock has no
    //   authentication layer, so no subjects can ever exist here.
    async fn handle_list_subjects(
        &self,
        _state: &Arc<tokio::sync::RwLock<RolesAnywhereState>>,
    ) -> MockResponse {
        wire::serialize_list_subjects_response(&wire::ListSubjectsResponse {
            ..Default::default()
        })
    }

    // STUB[no-auth]: Subjects are created by real authentication attempts; the mock has no
    //   authentication layer, so subjects are never present.
    async fn handle_get_subject(&self, _subject_id: &str) -> MockResponse {
        rest_json_error(404, "ResourceNotFoundException", "Subject not found.")
    }

    // Tag handlers

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<RolesAnywhereState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'resourceArn'");
        }
        let tags = tags_from_wire(Some(input.tags));
        let mut guard = state.write().await;
        match guard.tag_resource(&input.resource_arn, tags) {
            Ok(()) => wire::serialize_tag_resource_response(&wire::TagResourceResponse {
                ..Default::default()
            }),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<RolesAnywhereState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'resourceArn'");
        }

        let mut guard = state.write().await;
        match guard.untag_resource(&input.resource_arn, &input.tag_keys) {
            Ok(()) => wire::serialize_untag_resource_response(&wire::UntagResourceResponse {
                ..Default::default()
            }),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<RolesAnywhereState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'resourceArn'");
        }
        let guard = state.read().await;
        match guard.list_tags_for_resource(&input.resource_arn) {
            Ok(tags) => {
                let wire_tags: Vec<wire::Tag> = tags
                    .iter()
                    .map(|t| wire::Tag {
                        key: t.key.clone(),
                        value: t.value.clone(),
                        ..Default::default()
                    })
                    .collect();
                wire::serialize_list_tags_for_resource_response(
                    &wire::ListTagsForResourceResponse {
                        tags: Some(wire_tags),
                        ..Default::default()
                    },
                )
            }
            Err(e) => service_error_response(&e),
        }
    }
}
