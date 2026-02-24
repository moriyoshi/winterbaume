use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::json;
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
    StatefulService, extract_path, percent_decode, rest_json_error,
};

use crate::state::{SignerError, SignerState};
use crate::views::SignerStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct SignerService {
    pub(crate) state: Arc<BackendState<SignerState>>,
    pub(crate) notifier: StateChangeNotifier<SignerStateView>,
}

impl SignerService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for SignerService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for SignerService {
    fn service_name(&self) -> &str {
        "signer"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://signer\..*\.amazonaws\.com",
            r"https?://signer\.amazonaws\.com",
            r"https?://data-signer\..*\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl SignerService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let method = request.method.as_str();
        let query = winterbaume_core::extract_query_string(&request.uri);
        let query_map: HashMap<String, String> = winterbaume_core::parse_query_string(query);

        let segments: Vec<&str> = path
            .trim_start_matches('/')
            .split('/')
            .filter(|s| !s.is_empty())
            .collect();

        let response = match (method, segments.as_slice()) {
            // GET /signing-profiles - ListSigningProfiles
            ("GET", ["signing-profiles"]) => {
                let labels: &[(&str, &str)] = &[];
                self.handle_list_signing_profiles(&state, &request, labels, &query_map)
                    .await
            }
            // PUT /signing-profiles/{profileName} - PutSigningProfile
            ("PUT", ["signing-profiles", profile_name]) => {
                let profile_name_decoded = percent_decode(profile_name);
                let labels: &[(&str, &str)] = &[("profileName", profile_name_decoded.as_str())];
                self.handle_put_signing_profile(
                    &state, &request, labels, &query_map, &region, account_id,
                )
                .await
            }
            // GET /signing-profiles/{profileName}/permissions - ListProfilePermissions
            ("GET", ["signing-profiles", profile_name, "permissions"]) => {
                let profile_name_decoded = percent_decode(profile_name);
                let labels: &[(&str, &str)] = &[("profileName", profile_name_decoded.as_str())];
                self.handle_list_profile_permissions(&state, &request, labels, &query_map)
                    .await
            }
            // POST /signing-profiles/{profileName}/permissions - AddProfilePermission
            ("POST", ["signing-profiles", profile_name, "permissions"]) => {
                let profile_name_decoded = percent_decode(profile_name);
                let labels: &[(&str, &str)] = &[("profileName", profile_name_decoded.as_str())];
                self.handle_add_profile_permission(&state, &request, labels, &query_map)
                    .await
            }
            // DELETE /signing-profiles/{profileName}/permissions/{statementId} - RemoveProfilePermission
            (
                "DELETE",
                [
                    "signing-profiles",
                    profile_name,
                    "permissions",
                    statement_id,
                ],
            ) => {
                let profile_name_decoded = percent_decode(profile_name);
                let statement_id_decoded = percent_decode(statement_id);
                let labels: &[(&str, &str)] = &[
                    ("profileName", profile_name_decoded.as_str()),
                    ("statementId", statement_id_decoded.as_str()),
                ];
                self.handle_remove_profile_permission(&state, &request, labels, &query_map)
                    .await
            }
            // PUT /signing-profiles/{profileName}/revoke - RevokeSigningProfile
            ("PUT", ["signing-profiles", profile_name, "revoke"]) => {
                let profile_name_decoded = percent_decode(profile_name);
                let labels: &[(&str, &str)] = &[("profileName", profile_name_decoded.as_str())];
                self.handle_revoke_signing_profile(&state, &request, labels, &query_map)
                    .await
            }
            // GET /signing-profiles/{profileName} - GetSigningProfile
            ("GET", ["signing-profiles", profile_name]) => {
                let profile_name_decoded = percent_decode(profile_name);
                let labels: &[(&str, &str)] = &[("profileName", profile_name_decoded.as_str())];
                self.handle_get_signing_profile(&state, &request, labels, &query_map)
                    .await
            }
            // DELETE /signing-profiles/{profileName} - CancelSigningProfile
            ("DELETE", ["signing-profiles", profile_name]) => {
                let profile_name_decoded = percent_decode(profile_name);
                let labels: &[(&str, &str)] = &[("profileName", profile_name_decoded.as_str())];
                self.handle_cancel_signing_profile(&state, &request, labels, &query_map)
                    .await
            }
            // GET /signing-platforms/{platformId} - GetSigningPlatform
            ("GET", ["signing-platforms", platform_id]) => {
                let platform_id_decoded = percent_decode(platform_id);
                let labels: &[(&str, &str)] = &[("platformId", platform_id_decoded.as_str())];
                self.handle_get_signing_platform(&request, labels, &query_map)
                    .await
            }
            // GET /signing-platforms - ListSigningPlatforms
            ("GET", ["signing-platforms"]) => self.handle_list_signing_platforms().await,
            // POST /signing-jobs - StartSigningJob
            ("POST", ["signing-jobs"]) => {
                let labels: &[(&str, &str)] = &[];
                self.handle_start_signing_job(
                    &state, &request, labels, &query_map, account_id, &region,
                )
                .await
            }
            // POST /signing-jobs/with-payload - SignPayload
            ("POST", ["signing-jobs", "with-payload"]) => {
                let labels: &[(&str, &str)] = &[];
                self.handle_sign_payload(&state, &request, labels, &query_map, account_id, &region)
                    .await
            }
            // GET /signing-jobs/{jobId} - DescribeSigningJob
            ("GET", ["signing-jobs", job_id]) => {
                let job_id_decoded = percent_decode(job_id);
                let labels: &[(&str, &str)] = &[("jobId", job_id_decoded.as_str())];
                self.handle_describe_signing_job(&state, &request, labels, &query_map)
                    .await
            }
            // PUT /signing-jobs/{jobId}/revoke - RevokeSignature
            ("PUT", ["signing-jobs", job_id, "revoke"]) => {
                let job_id_decoded = percent_decode(job_id);
                let labels: &[(&str, &str)] = &[("jobId", job_id_decoded.as_str())];
                self.handle_revoke_signature(&state, &request, labels, &query_map)
                    .await
            }
            // GET /signing-jobs - ListSigningJobs
            ("GET", ["signing-jobs"]) => self.handle_list_signing_jobs(&state).await,
            // GET /revocations - GetRevocationStatus
            ("GET", ["revocations"]) => self.handle_get_revocation_status().await,
            // GET /tags/{resourceArn} - ListTagsForResource
            ("GET", ["tags", resource_arn_part @ ..]) => {
                let resource_arn = percent_decode(&resource_arn_part.join("/"));
                let labels: &[(&str, &str)] = &[("resourceArn", resource_arn.as_str())];
                self.handle_list_tags_for_resource(&state, &request, labels, &query_map)
                    .await
            }
            // POST /tags/{resourceArn} - TagResource
            ("POST", ["tags", resource_arn_part @ ..]) => {
                let resource_arn = percent_decode(&resource_arn_part.join("/"));
                let labels: &[(&str, &str)] = &[("resourceArn", resource_arn.as_str())];
                self.handle_tag_resource(&state, &request, labels, &query_map)
                    .await
            }
            // DELETE /tags/{resourceArn} - UntagResource
            ("DELETE", ["tags", resource_arn_part @ ..]) => {
                let resource_arn = percent_decode(&resource_arn_part.join("/"));
                let labels: &[(&str, &str)] = &[("resourceArn", resource_arn.as_str())];
                self.handle_untag_resource(&state, &request, labels, &query_map)
                    .await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        };
        if response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_put_signing_profile(
        &self,
        state: &Arc<tokio::sync::RwLock<SignerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_put_signing_profile_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let platform_id = if input.platform_id.is_empty() {
            "AWSLambda-SHA384-ECDSA"
        } else {
            input.platform_id.as_str()
        };

        let mut state = state.write().await;
        match state.put_signing_profile(&input.profile_name, platform_id, region, account_id) {
            Ok(profile) => {
                wire::serialize_put_signing_profile_response(&wire::PutSigningProfileResponse {
                    arn: Some(profile.arn.clone()),
                    profile_version: Some(profile.profile_version.clone()),
                    profile_version_arn: Some(profile.profile_version_arn.clone()),
                })
            }
            Err(e) => signer_error_response(&e),
        }
    }

    async fn handle_get_signing_profile(
        &self,
        state: &Arc<tokio::sync::RwLock<SignerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_signing_profile_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_signing_profile(&input.profile_name) {
            Ok(profile) => {
                wire::serialize_get_signing_profile_response(&wire::GetSigningProfileResponse {
                    profile_name: Some(profile.profile_name.clone()),
                    profile_version: Some(profile.profile_version.clone()),
                    profile_version_arn: Some(profile.profile_version_arn.clone()),
                    platform_id: Some(profile.platform_id.clone()),
                    status: Some(profile.status.clone()),
                    arn: Some(profile.arn.clone()),
                    ..Default::default()
                })
            }
            Err(e) => signer_error_response(&e),
        }
    }

    async fn handle_cancel_signing_profile(
        &self,
        state: &Arc<tokio::sync::RwLock<SignerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_cancel_signing_profile_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.cancel_signing_profile(&input.profile_name) {
            Ok(()) => wire::serialize_cancel_signing_profile_response(),
            Err(e) => signer_error_response(&e),
        }
    }

    async fn handle_list_signing_profiles(
        &self,
        state: &Arc<tokio::sync::RwLock<SignerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_signing_profiles_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let include_canceled = input.include_canceled.unwrap_or(false);
        let state = state.read().await;
        let profiles = state.list_signing_profiles(include_canceled);
        let entries: Vec<wire::SigningProfile> = profiles
            .iter()
            .map(|p| wire::SigningProfile {
                profile_name: Some(p.profile_name.clone()),
                profile_version: Some(p.profile_version.clone()),
                profile_version_arn: Some(p.profile_version_arn.clone()),
                platform_id: Some(p.platform_id.clone()),
                status: Some(p.status.clone()),
                arn: Some(p.arn.clone()),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_signing_profiles_response(&wire::ListSigningProfilesResponse {
            profiles: Some(entries),
            ..Default::default()
        })
    }

    async fn handle_list_signing_platforms(&self) -> MockResponse {
        // Return well-known signing platforms
        let platforms = vec![
            wire::SigningPlatform {
                platform_id: Some("AWSLambda-SHA384-ECDSA".to_string()),
                display_name: Some("AWS Lambda".to_string()),
                partner: Some("AWSLambda".to_string()),
                target: Some("SHA384-ECDSA".to_string()),
                category: Some("AWS".to_string()),
                max_size_in_m_b: Some(250),
                ..Default::default()
            },
            wire::SigningPlatform {
                platform_id: Some("AmazonFreeRTOS-TI-CC3220SF".to_string()),
                display_name: Some("Amazon FreeRTOS SHA1-RSA CC3220SF-Format".to_string()),
                partner: Some("AmazonFreeRTOS".to_string()),
                target: Some("SHA1-RSA-TISHA1".to_string()),
                category: Some("AWS".to_string()),
                max_size_in_m_b: Some(16),
                ..Default::default()
            },
        ];
        wire::serialize_list_signing_platforms_response(&wire::ListSigningPlatformsResponse {
            platforms: Some(platforms),
            ..Default::default()
        })
    }

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<SignerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.list_tags_for_resource(&input.resource_arn) {
            Ok(tags) => wire::serialize_list_tags_for_resource_response(
                &wire::ListTagsForResourceResponse {
                    tags: Some(tags.clone()),
                },
            ),
            Err(e) => signer_error_response(&e),
        }
    }

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<SignerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.tag_resource(&input.resource_arn, input.tags.clone()) {
            Ok(()) => wire::serialize_tag_resource_response(&wire::TagResourceResponse {}),
            Err(e) => signer_error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<SignerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.untag_resource(&input.resource_arn, &input.tag_keys) {
            Ok(()) => wire::serialize_untag_resource_response(&wire::UntagResourceResponse {}),
            Err(e) => signer_error_response(&e),
        }
    }

    async fn handle_add_profile_permission(
        &self,
        state: &Arc<tokio::sync::RwLock<SignerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_add_profile_permission_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.statement_id.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'statementId'");
        }
        if input.action.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'action'");
        }
        if input.principal.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'principal'");
        }

        let mut state = state.write().await;
        match state.add_profile_permission(
            &input.profile_name,
            &input.statement_id,
            &input.action,
            &input.principal,
            input.profile_version.clone(),
        ) {
            Ok(revision_id) => wire::serialize_add_profile_permission_response(
                &wire::AddProfilePermissionResponse {
                    revision_id: Some(revision_id),
                },
            ),
            Err(e) => signer_error_response(&e),
        }
    }

    async fn handle_list_profile_permissions(
        &self,
        state: &Arc<tokio::sync::RwLock<SignerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_profile_permissions_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.list_profile_permissions(&input.profile_name) {
            Ok((permissions, revision_id)) => {
                let wire_perms: Vec<wire::Permission> = permissions
                    .iter()
                    .map(|p| wire::Permission {
                        action: Some(p.action.clone()),
                        principal: Some(p.principal.clone()),
                        profile_version: p.profile_version.clone(),
                        statement_id: Some(p.statement_id.clone()),
                    })
                    .collect();
                wire::serialize_list_profile_permissions_response(
                    &wire::ListProfilePermissionsResponse {
                        permissions: Some(wire_perms),
                        revision_id: Some(revision_id.to_string()),
                        ..Default::default()
                    },
                )
            }
            Err(e) => signer_error_response(&e),
        }
    }

    async fn handle_remove_profile_permission(
        &self,
        state: &Arc<tokio::sync::RwLock<SignerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_remove_profile_permission_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let mut state = state.write().await;
        match state.remove_profile_permission(&input.profile_name, &input.statement_id) {
            Ok(revision_id) => wire::serialize_remove_profile_permission_response(
                &wire::RemoveProfilePermissionResponse {
                    revision_id: Some(revision_id),
                },
            ),
            Err(e) => signer_error_response(&e),
        }
    }

    async fn handle_revoke_signing_profile(
        &self,
        state: &Arc<tokio::sync::RwLock<SignerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_revoke_signing_profile_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.profile_version.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'profileVersion'");
        }
        let mut state = state.write().await;
        match state.revoke_signing_profile(&input.profile_name, &input.profile_version) {
            Ok(()) => wire::serialize_revoke_signing_profile_response(),
            Err(e) => signer_error_response(&e),
        }
    }

    async fn handle_get_signing_platform(
        &self,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_signing_platform_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let platform_id = &input.platform_id;
        // Look up from the well-known platforms list
        let platforms = vec![
            wire::GetSigningPlatformResponse {
                platform_id: Some("AWSLambda-SHA384-ECDSA".to_string()),
                display_name: Some("AWS Lambda".to_string()),
                partner: Some("AWSLambda".to_string()),
                target: Some("SHA384-ECDSA".to_string()),
                category: Some("AWS".to_string()),
                max_size_in_m_b: Some(250),
                revocation_supported: Some(true),
                ..Default::default()
            },
            wire::GetSigningPlatformResponse {
                platform_id: Some("AmazonFreeRTOS-TI-CC3220SF".to_string()),
                display_name: Some("Amazon FreeRTOS SHA1-RSA CC3220SF-Format".to_string()),
                partner: Some("AmazonFreeRTOS".to_string()),
                target: Some("SHA1-RSA-TISHA1".to_string()),
                category: Some("AWS".to_string()),
                max_size_in_m_b: Some(16),
                revocation_supported: Some(false),
                ..Default::default()
            },
        ];
        match platforms
            .into_iter()
            .find(|p| p.platform_id.as_deref() == Some(platform_id.as_str()))
        {
            Some(platform) => wire::serialize_get_signing_platform_response(&platform),
            None => rest_json_error(
                404,
                "ResourceNotFoundException",
                &format!("Signing platform {platform_id} not found."),
            ),
        }
    }

    async fn handle_start_signing_job(
        &self,
        state: &Arc<tokio::sync::RwLock<SignerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_start_signing_job_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.profile_name.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'profileName'");
        }
        let mut state = state.write().await;
        match state.start_signing_job(&input.profile_name, account_id, region) {
            Ok((job_id, job_owner)) => {
                wire::serialize_start_signing_job_response(&wire::StartSigningJobResponse {
                    job_id: Some(job_id),
                    job_owner: Some(job_owner),
                })
            }
            Err(e) => signer_error_response(&e),
        }
    }

    async fn handle_sign_payload(
        &self,
        state: &Arc<tokio::sync::RwLock<SignerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_sign_payload_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.profile_name.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'profileName'");
        }
        let mut state = state.write().await;
        match state.sign_payload(
            &input.profile_name,
            input.payload.as_bytes(),
            account_id,
            region,
        ) {
            Ok((job_id, job_owner, signature)) => {
                wire::serialize_sign_payload_response(&wire::SignPayloadResponse {
                    job_id: Some(job_id),
                    job_owner: Some(job_owner),
                    signature: Some(signature),
                    ..Default::default()
                })
            }
            Err(e) => signer_error_response(&e),
        }
    }

    async fn handle_describe_signing_job(
        &self,
        state: &Arc<tokio::sync::RwLock<SignerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_signing_job_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.describe_signing_job(&input.job_id) {
            Ok(job) => {
                wire::serialize_describe_signing_job_response(&wire::DescribeSigningJobResponse {
                    job_id: Some(job.job_id.clone()),
                    job_owner: Some(job.job_owner.clone()),
                    profile_name: Some(job.profile_name.clone()),
                    profile_version: Some(job.profile_version.clone()),
                    platform_id: Some(job.platform_id.clone()),
                    status: Some(job.status.clone()),
                    created_at: Some(job.created_at),
                    ..Default::default()
                })
            }
            Err(e) => signer_error_response(&e),
        }
    }

    async fn handle_list_signing_jobs(
        &self,
        state: &Arc<tokio::sync::RwLock<SignerState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let jobs: Vec<wire::SigningJob> = state
            .list_signing_jobs()
            .into_iter()
            .map(|j| wire::SigningJob {
                job_id: Some(j.job_id.clone()),
                job_owner: Some(j.job_owner.clone()),
                profile_name: Some(j.profile_name.clone()),
                profile_version: Some(j.profile_version.clone()),
                platform_id: Some(j.platform_id.clone()),
                status: Some(j.status.clone()),
                created_at: Some(j.created_at),
                is_revoked: Some(j.is_revoked),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_signing_jobs_response(&wire::ListSigningJobsResponse {
            jobs: Some(jobs),
            ..Default::default()
        })
    }

    async fn handle_revoke_signature(
        &self,
        state: &Arc<tokio::sync::RwLock<SignerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_revoke_signature_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.revoke_signature(&input.job_id) {
            Ok(()) => wire::serialize_revoke_signature_response(),
            Err(e) => signer_error_response(&e),
        }
    }

    async fn handle_get_revocation_status(&self) -> MockResponse {
        // Return empty list — no revocations tracked at the granularity requested
        wire::serialize_get_revocation_status_response(&wire::GetRevocationStatusResponse {
            revoked_entities: Some(vec![]),
        })
    }
}

fn signer_error_response(err: &SignerError) -> MockResponse {
    let (status, error_type) = match err {
        SignerError::ProfileNameEmpty => (400, "ValidationException"),
        SignerError::ProfileVersionMismatch { .. } => (400, "ValidationException"),
        SignerError::ProfileNotActive(_) => (400, "ValidationException"),
        SignerError::ProfileNotFound(_) => (404, "ResourceNotFoundException"),
        SignerError::ResourceNotFound(_) => (404, "ResourceNotFoundException"),
        SignerError::PermissionStatementNotFound(_) => (404, "ResourceNotFoundException"),
        SignerError::SigningJobNotFound(_) => (404, "ResourceNotFoundException"),
    };
    let body = json!({
        "Type": "User",
        "Message": err.to_string(),
    });
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}
