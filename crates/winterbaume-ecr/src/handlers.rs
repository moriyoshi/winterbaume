use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
    StatefulService,
};

use crate::model;
use crate::state::{EcrError, EcrState, ImageId};
use crate::types::{
    EncryptionConfiguration, ImageScanningConfiguration, RegistryScanningConfig,
    RegistryScanningRuleData, ReplicationConfig, ReplicationDestinationData, ReplicationRuleData,
    Repository, RepositoryCreationTemplateData, RepositoryFilterData, ScanningRepositoryFilterData,
    SigningConfigData, SigningRepositoryFilterData, SigningRuleData,
};
use crate::views::EcrStateView;
use crate::wire;

/// ECR service handler that processes awsJson1.1 protocol requests.
pub struct EcrService {
    pub(crate) state: Arc<BackendState<EcrState>>,
    pub(crate) notifier: StateChangeNotifier<EcrStateView>,
}

impl EcrService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for EcrService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for EcrService {
    fn service_name(&self) -> &str {
        "ecr"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://ecr\..*\.amazonaws\.com",
            r"https?://api\.ecr\..*\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl EcrService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_headers(&request.headers)
            .unwrap_or_else(|| winterbaume_core::auth::extract_region_from_uri(&request.uri));
        let account_id = DEFAULT_ACCOUNT_ID;

        // Extract action from X-Amz-Target header
        // Format: "AmazonEC2ContainerRegistry_V20150921.CreateRepository"
        let action = request
            .headers
            .get("x-amz-target")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.rsplit('.').next())
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

        let response = match action.as_str() {
            "CreateRepository" => {
                self.handle_create_repository(&state, body_bytes, account_id, &region)
                    .await
            }
            "DeleteRepository" => self.handle_delete_repository(&state, body_bytes).await,
            "DescribeRepositories" => self.handle_describe_repositories(&state, body_bytes).await,
            "ListImages" => self.handle_list_images(&state, body_bytes).await,
            "PutImage" => self.handle_put_image(&state, body_bytes).await,
            "BatchGetImage" => self.handle_batch_get_image(&state, body_bytes).await,
            "BatchDeleteImage" => self.handle_batch_delete_image(&state, body_bytes).await,
            "GetAuthorizationToken" => {
                self.handle_get_authorization_token(account_id, &region)
                    .await
            }
            "DescribeImages" => self.handle_describe_images(&state, body_bytes).await,
            "PutLifecyclePolicy" => self.handle_put_lifecycle_policy(&state, body_bytes).await,
            "GetLifecyclePolicy" => self.handle_get_lifecycle_policy(&state, body_bytes).await,
            "DeleteLifecyclePolicy" => {
                self.handle_delete_lifecycle_policy(&state, body_bytes)
                    .await
            }
            "SetRepositoryPolicy" => self.handle_set_repository_policy(&state, body_bytes).await,
            "GetRepositoryPolicy" => self.handle_get_repository_policy(&state, body_bytes).await,
            "DeleteRepositoryPolicy" => {
                self.handle_delete_repository_policy(&state, body_bytes)
                    .await
            }
            "PutImageScanningConfiguration" => {
                self.handle_put_image_scanning_configuration(&state, body_bytes)
                    .await
            }
            "PutImageTagMutability" => {
                self.handle_put_image_tag_mutability(&state, body_bytes)
                    .await
            }
            "StartImageScan" => self.handle_start_image_scan(&state, body_bytes).await,
            "DescribeImageScanFindings" => {
                self.handle_describe_image_scan_findings(&state, body_bytes)
                    .await
            }
            "TagResource" => self.handle_tag_resource(&state, body_bytes).await,
            "UntagResource" => self.handle_untag_resource(&state, body_bytes).await,
            "ListTagsForResource" => self.handle_list_tags_for_resource(&state, body_bytes).await,
            "PutRegistryPolicy" => {
                self.handle_put_registry_policy(&state, body_bytes, account_id)
                    .await
            }
            "GetRegistryPolicy" => self.handle_get_registry_policy(&state, account_id).await,
            "DeleteRegistryPolicy" => self.handle_delete_registry_policy(&state, account_id).await,
            "DescribeRegistry" => self.handle_describe_registry(&state, account_id).await,
            "PutRegistryScanningConfiguration" => {
                self.handle_put_registry_scanning_configuration(&state, body_bytes, account_id)
                    .await
            }
            "GetRegistryScanningConfiguration" => {
                self.handle_get_registry_scanning_configuration(&state, account_id)
                    .await
            }
            "PutReplicationConfiguration" => {
                self.handle_put_replication_configuration(&state, body_bytes)
                    .await
            }
            "BatchGetRepositoryScanningConfiguration" => {
                self.handle_batch_get_repository_scanning_configuration(&state, body_bytes)
                    .await
            }
            // Layer upload operations
            "InitiateLayerUpload" => {
                self.handle_initiate_layer_upload(&state, body_bytes, account_id)
                    .await
            }
            "UploadLayerPart" => {
                self.handle_upload_layer_part(&state, body_bytes, account_id)
                    .await
            }
            "CompleteLayerUpload" => {
                self.handle_complete_layer_upload(&state, body_bytes, account_id)
                    .await
            }
            "BatchCheckLayerAvailability" => {
                self.handle_batch_check_layer_availability(&state, body_bytes)
                    .await
            }
            "GetDownloadUrlForLayer" => {
                self.handle_get_download_url_for_layer(&state, body_bytes, account_id, &region)
                    .await
            }
            // Pull-through cache rules
            "CreatePullThroughCacheRule" => {
                self.handle_create_pull_through_cache_rule(&state, body_bytes, account_id)
                    .await
            }
            "DeletePullThroughCacheRule" => {
                self.handle_delete_pull_through_cache_rule(&state, body_bytes, account_id)
                    .await
            }
            "DescribePullThroughCacheRules" => {
                self.handle_describe_pull_through_cache_rules(&state, body_bytes)
                    .await
            }
            "UpdatePullThroughCacheRule" => {
                self.handle_update_pull_through_cache_rule(&state, body_bytes, account_id)
                    .await
            }
            "ValidatePullThroughCacheRule" => {
                self.handle_validate_pull_through_cache_rule(&state, body_bytes, account_id)
                    .await
            }
            // Repository creation templates
            "CreateRepositoryCreationTemplate" => {
                self.handle_create_repository_creation_template(&state, body_bytes, account_id)
                    .await
            }
            "DeleteRepositoryCreationTemplate" => {
                self.handle_delete_repository_creation_template(&state, body_bytes, account_id)
                    .await
            }
            "DescribeRepositoryCreationTemplates" => {
                self.handle_describe_repository_creation_templates(&state, body_bytes, account_id)
                    .await
            }
            "UpdateRepositoryCreationTemplate" => {
                self.handle_update_repository_creation_template(&state, body_bytes, account_id)
                    .await
            }
            // Signing configuration
            "GetSigningConfiguration" => {
                self.handle_get_signing_configuration(&state, account_id)
                    .await
            }
            "PutSigningConfiguration" => {
                self.handle_put_signing_configuration(&state, body_bytes, account_id)
                    .await
            }
            "DeleteSigningConfiguration" => {
                self.handle_delete_signing_configuration(&state, account_id)
                    .await
            }
            // Pull-time update exclusions
            "RegisterPullTimeUpdateExclusion" => {
                self.handle_register_pull_time_update_exclusion(&state, body_bytes)
                    .await
            }
            "DeregisterPullTimeUpdateExclusion" => {
                self.handle_deregister_pull_time_update_exclusion(&state, body_bytes)
                    .await
            }
            "ListPullTimeUpdateExclusions" => {
                self.handle_list_pull_time_update_exclusions(&state).await
            }
            // Account settings
            "GetAccountSetting" => self.handle_get_account_setting(&state, body_bytes).await,
            "PutAccountSetting" => self.handle_put_account_setting(&state, body_bytes).await,
            // Lifecycle policy preview
            "StartLifecyclePolicyPreview" => {
                self.handle_start_lifecycle_policy_preview(&state, body_bytes, account_id)
                    .await
            }
            "GetLifecyclePolicyPreview" => {
                self.handle_get_lifecycle_policy_preview(&state, body_bytes, account_id)
                    .await
            }
            // Misc operations
            "DescribeImageReplicationStatus" => {
                self.handle_describe_image_replication_status(&state, body_bytes)
                    .await
            }
            "DescribeImageSigningStatus" => {
                self.handle_describe_image_signing_status(&state, body_bytes, account_id)
                    .await
            }
            "ListImageReferrers" => self.handle_list_image_referrers(&state, body_bytes).await,
            "UpdateImageStorageClass" => {
                self.handle_update_image_storage_class(&state, body_bytes, account_id)
                    .await
            }
            _ => json_error_response(
                400,
                "InvalidAction",
                &format!("Could not find operation {action} for ECR"),
            ),
        };

        if response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_create_repository(
        &self,
        state: &Arc<tokio::sync::RwLock<EcrState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_repository_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.repository_name.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "Missing 'repositoryName'",
            );
        }
        let image_tag_mutability = input.image_tag_mutability.clone();

        let image_scanning_configuration = input
            .image_scanning_configuration
            .as_ref()
            .and_then(|v| v.scan_on_push)
            .map(|scan_on_push| ImageScanningConfiguration { scan_on_push });

        let encryption_configuration = input.encryption_configuration.as_ref().map(|v| {
            let encryption_type = if v.encryption_type.is_empty() {
                "AES256".to_string()
            } else {
                v.encryption_type.clone()
            };
            EncryptionConfiguration {
                encryption_type,
                kms_key: v.kms_key.clone(),
            }
        });

        let mut state = state.write().await;
        match state.create_repository(
            &input.repository_name,
            account_id,
            region,
            image_tag_mutability,
            image_scanning_configuration,
            encryption_configuration,
        ) {
            Ok(repo) => {
                wire::serialize_create_repository_response(&wire::CreateRepositoryResponse {
                    repository: Some(repository_to_wire(repo)),
                })
            }
            Err(e) => ecr_error_response(&e),
        }
    }

    async fn handle_delete_repository(
        &self,
        state: &Arc<tokio::sync::RwLock<EcrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_repository_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.repository_name.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "Missing 'repositoryName'",
            );
        }
        let force = input.force.unwrap_or(false);

        let mut state = state.write().await;
        match state.delete_repository(&input.repository_name, force) {
            Ok(repo) => {
                wire::serialize_delete_repository_response(&wire::DeleteRepositoryResponse {
                    repository: Some(repository_to_wire(&repo)),
                })
            }
            Err(e) => ecr_error_response(&e),
        }
    }

    async fn handle_describe_repositories(
        &self,
        state: &Arc<tokio::sync::RwLock<EcrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_repositories_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let names = input.repository_names;

        let state = state.read().await;
        match state.describe_repositories(names.as_deref()) {
            Ok(repos) => {
                let entries: Vec<wire::Repository> =
                    repos.iter().map(|r| repository_to_wire(r)).collect();
                wire::serialize_describe_repositories_response(
                    &wire::DescribeRepositoriesResponse {
                        repositories: Some(entries),
                        next_token: None,
                    },
                )
            }
            Err(e) => ecr_error_response(&e),
        }
    }

    async fn handle_list_images(
        &self,
        state: &Arc<tokio::sync::RwLock<EcrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_images_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.repository_name.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "Missing 'repositoryName'",
            );
        }

        let state = state.read().await;
        match state.list_images(&input.repository_name) {
            Ok(entries) => {
                let image_ids: Vec<wire::ImageIdentifier> = entries
                    .iter()
                    .map(|entry| wire::ImageIdentifier {
                        image_digest: Some(entry.image_digest.clone()),
                        image_tag: entry.image_tag.clone(),
                    })
                    .collect();
                wire::serialize_list_images_response(&wire::ListImagesResponse {
                    image_ids: Some(image_ids),
                    next_token: None,
                })
            }
            Err(e) => ecr_error_response(&e),
        }
    }

    async fn handle_put_image(
        &self,
        state: &Arc<tokio::sync::RwLock<EcrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_put_image_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.repository_name.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "Missing 'repositoryName'",
            );
        }
        if input.image_manifest.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "Missing 'imageManifest'",
            );
        }
        let image_tag = input.image_tag.as_deref();

        let mut state = state.write().await;
        match state.put_image(&input.repository_name, image_tag, &input.image_manifest) {
            Ok((result, registry_id)) => {
                wire::serialize_put_image_response(&wire::PutImageResponse {
                    image: Some(wire::Image {
                        registry_id: Some(registry_id),
                        repository_name: Some(input.repository_name.clone()),
                        image_id: Some(wire::ImageIdentifier {
                            image_digest: Some(result.image_digest),
                            image_tag: result.image_tag,
                        }),
                        image_manifest: Some(result.image_manifest),
                        image_manifest_media_type: None,
                    }),
                })
            }
            Err(e) => ecr_error_response(&e),
        }
    }

    async fn handle_batch_get_image(
        &self,
        state: &Arc<tokio::sync::RwLock<EcrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_batch_get_image_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.repository_name.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "Missing 'repositoryName'",
            );
        }

        let image_ids = model_image_ids_to_state(&input.image_ids);

        let state = state.read().await;
        match state.batch_get_image(&input.repository_name, &image_ids) {
            Ok(result) => {
                let image_entries: Vec<wire::Image> = result
                    .images
                    .iter()
                    .map(|img| wire::Image {
                        registry_id: Some(
                            state
                                .repositories
                                .get(input.repository_name.as_str())
                                .map(|r| r.registry_id.as_str())
                                .unwrap_or(DEFAULT_ACCOUNT_ID)
                                .to_string(),
                        ),
                        repository_name: Some(input.repository_name.clone()),
                        image_id: Some(wire::ImageIdentifier {
                            image_digest: Some(img.image_digest.clone()),
                            image_tag: img.image_tags.first().cloned(),
                        }),
                        image_manifest: Some(img.image_manifest.clone()),
                        image_manifest_media_type: None,
                    })
                    .collect();

                let failure_entries: Vec<wire::ImageFailure> = result
                    .failures
                    .iter()
                    .map(|f| wire::ImageFailure {
                        image_id: Some(wire::ImageIdentifier {
                            image_digest: f.image_id.image_digest.clone(),
                            image_tag: f.image_id.image_tag.clone(),
                        }),
                        failure_code: Some(f.failure_code.clone()),
                        failure_reason: Some(f.failure_reason.clone()),
                    })
                    .collect();

                wire::serialize_batch_get_image_response(&wire::BatchGetImageResponse {
                    images: Some(image_entries),
                    failures: Some(failure_entries),
                })
            }
            Err(e) => ecr_error_response(&e),
        }
    }

    async fn handle_batch_delete_image(
        &self,
        state: &Arc<tokio::sync::RwLock<EcrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_batch_delete_image_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.repository_name.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "Missing 'repositoryName'",
            );
        }

        let image_ids = model_image_ids_to_state(&input.image_ids);

        let mut state = state.write().await;
        match state.batch_delete_image(&input.repository_name, &image_ids) {
            Ok(result) => {
                let id_entries: Vec<wire::ImageIdentifier> = result
                    .image_ids
                    .iter()
                    .map(|id| wire::ImageIdentifier {
                        image_digest: id.image_digest.clone(),
                        image_tag: id.image_tag.clone(),
                    })
                    .collect();

                let failure_entries: Vec<wire::ImageFailure> = result
                    .failures
                    .iter()
                    .map(|f| wire::ImageFailure {
                        image_id: Some(wire::ImageIdentifier {
                            image_digest: f.image_id.image_digest.clone(),
                            image_tag: f.image_id.image_tag.clone(),
                        }),
                        failure_code: Some(f.failure_code.clone()),
                        failure_reason: Some(f.failure_reason.clone()),
                    })
                    .collect();

                wire::serialize_batch_delete_image_response(&wire::BatchDeleteImageResponse {
                    image_ids: Some(id_entries),
                    failures: Some(failure_entries),
                })
            }
            Err(e) => ecr_error_response(&e),
        }
    }

    async fn handle_get_authorization_token(&self, account_id: &str, region: &str) -> MockResponse {
        let state = self.state.get(account_id, region);
        let state = state.read().await;
        let auth = state.get_authorization_token(account_id, region);

        wire::serialize_get_authorization_token_response(&wire::GetAuthorizationTokenResponse {
            authorization_data: Some(vec![wire::AuthorizationData {
                authorization_token: Some(auth.authorization_token),
                proxy_endpoint: Some(auth.proxy_endpoint),
                expires_at: Some(auth.expires_at.timestamp() as f64),
            }]),
        })
    }

    // --- New operations ---

    async fn handle_describe_images(
        &self,
        state: &Arc<tokio::sync::RwLock<EcrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_images_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.repository_name.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "Missing 'repositoryName'",
            );
        }

        let image_ids = input
            .image_ids
            .as_ref()
            .map(|ids| model_image_ids_to_state(ids));

        let state = state.read().await;
        match state.describe_images(&input.repository_name, image_ids.as_deref()) {
            Ok(details) => {
                let image_details: Vec<wire::ImageDetail> = details
                    .iter()
                    .map(|d| wire::ImageDetail {
                        image_digest: Some(d.image.image_digest.clone()),
                        image_tags: if d.image.image_tags.is_empty() {
                            None
                        } else {
                            Some(d.image.image_tags.clone())
                        },
                        image_pushed_at: Some(d.image.pushed_at.timestamp() as f64),
                        repository_name: Some(d.repository_name.clone()),
                        registry_id: Some(d.registry_id.clone()),
                        image_size_in_bytes: Some(d.image.image_manifest.len() as i64),
                        image_manifest_media_type: Some(
                            "application/vnd.docker.distribution.manifest.v2+json".to_string(),
                        ),
                        image_scan_status: d.image.image_scan_status.as_ref().map(|s| {
                            wire::ImageScanStatus {
                                status: Some(s.status.clone()),
                                description: Some(s.description.clone()),
                            }
                        }),
                        ..Default::default()
                    })
                    .collect();
                wire::serialize_describe_images_response(&wire::DescribeImagesResponse {
                    image_details: Some(image_details),
                    next_token: None,
                })
            }
            Err(e) => ecr_error_response(&e),
        }
    }

    async fn handle_put_lifecycle_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<EcrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_put_lifecycle_policy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.repository_name.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "Missing 'repositoryName'",
            );
        }
        if input.lifecycle_policy_text.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "Missing 'lifecyclePolicyText'",
            );
        }

        let mut state = state.write().await;
        match state.put_lifecycle_policy(&input.repository_name, &input.lifecycle_policy_text) {
            Ok((registry_id, repo_name)) => {
                wire::serialize_put_lifecycle_policy_response(&wire::PutLifecyclePolicyResponse {
                    registry_id: Some(registry_id.to_string()),
                    repository_name: Some(repo_name.to_string()),
                    lifecycle_policy_text: Some(input.lifecycle_policy_text.clone()),
                })
            }
            Err(e) => ecr_error_response(&e),
        }
    }

    async fn handle_get_lifecycle_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<EcrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_lifecycle_policy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.repository_name.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "Missing 'repositoryName'",
            );
        }

        let state = state.read().await;
        match state.get_lifecycle_policy(&input.repository_name) {
            Ok((registry_id, repo_name, policy_text)) => {
                wire::serialize_get_lifecycle_policy_response(&wire::GetLifecyclePolicyResponse {
                    registry_id: Some(registry_id.to_string()),
                    repository_name: Some(repo_name.to_string()),
                    lifecycle_policy_text: Some(policy_text.to_string()),
                    last_evaluated_at: None,
                })
            }
            Err(e) => ecr_error_response(&e),
        }
    }

    async fn handle_delete_lifecycle_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<EcrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_lifecycle_policy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.repository_name.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "Missing 'repositoryName'",
            );
        }

        let mut state = state.write().await;
        match state.delete_lifecycle_policy(&input.repository_name) {
            Ok((registry_id, repo_name, policy_text)) => {
                wire::serialize_delete_lifecycle_policy_response(
                    &wire::DeleteLifecyclePolicyResponse {
                        registry_id: Some(registry_id),
                        repository_name: Some(repo_name),
                        lifecycle_policy_text: Some(policy_text),
                        last_evaluated_at: None,
                    },
                )
            }
            Err(e) => ecr_error_response(&e),
        }
    }

    async fn handle_set_repository_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<EcrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_set_repository_policy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.repository_name.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "Missing 'repositoryName'",
            );
        }
        if input.policy_text.is_empty() {
            return json_error_response(400, "InvalidParameterException", "Missing 'policyText'");
        }

        let mut state = state.write().await;
        match state.set_repository_policy(&input.repository_name, &input.policy_text) {
            Ok((registry_id, repo_name)) => {
                wire::serialize_set_repository_policy_response(&wire::SetRepositoryPolicyResponse {
                    registry_id: Some(registry_id.to_string()),
                    repository_name: Some(repo_name.to_string()),
                    policy_text: Some(input.policy_text.clone()),
                })
            }
            Err(e) => ecr_error_response(&e),
        }
    }

    async fn handle_get_repository_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<EcrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_repository_policy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.repository_name.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "Missing 'repositoryName'",
            );
        }

        let state = state.read().await;
        match state.get_repository_policy(&input.repository_name) {
            Ok((registry_id, repo_name, policy_text)) => {
                wire::serialize_get_repository_policy_response(&wire::GetRepositoryPolicyResponse {
                    registry_id: Some(registry_id.to_string()),
                    repository_name: Some(repo_name.to_string()),
                    policy_text: Some(policy_text.to_string()),
                })
            }
            Err(e) => ecr_error_response(&e),
        }
    }

    async fn handle_delete_repository_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<EcrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_repository_policy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.repository_name.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "Missing 'repositoryName'",
            );
        }

        let mut state = state.write().await;
        match state.delete_repository_policy(&input.repository_name) {
            Ok((registry_id, repo_name, policy_text)) => {
                wire::serialize_delete_repository_policy_response(
                    &wire::DeleteRepositoryPolicyResponse {
                        registry_id: Some(registry_id),
                        repository_name: Some(repo_name),
                        policy_text: Some(policy_text),
                    },
                )
            }
            Err(e) => ecr_error_response(&e),
        }
    }

    async fn handle_put_image_scanning_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<EcrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_put_image_scanning_configuration_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.repository_name.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "Missing 'repositoryName'",
            );
        }
        let scan_on_push = input
            .image_scanning_configuration
            .scan_on_push
            .unwrap_or(false);

        let mut state = state.write().await;
        match state.put_image_scanning_configuration(&input.repository_name, scan_on_push) {
            Ok((registry_id, repo_name, scan_on_push)) => {
                wire::serialize_put_image_scanning_configuration_response(
                    &wire::PutImageScanningConfigurationResponse {
                        registry_id: Some(registry_id.to_string()),
                        repository_name: Some(repo_name.to_string()),
                        image_scanning_configuration: Some(wire::ImageScanningConfiguration {
                            scan_on_push: Some(scan_on_push),
                        }),
                    },
                )
            }
            Err(e) => ecr_error_response(&e),
        }
    }

    async fn handle_put_image_tag_mutability(
        &self,
        state: &Arc<tokio::sync::RwLock<EcrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_put_image_tag_mutability_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.repository_name.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "Missing 'repositoryName'",
            );
        }
        if input.image_tag_mutability.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "Missing 'imageTagMutability'",
            );
        }

        let mut state = state.write().await;
        match state.put_image_tag_mutability(&input.repository_name, &input.image_tag_mutability) {
            Ok((registry_id, repo_name, mutability)) => {
                wire::serialize_put_image_tag_mutability_response(
                    &wire::PutImageTagMutabilityResponse {
                        registry_id: Some(registry_id.to_string()),
                        repository_name: Some(repo_name.to_string()),
                        image_tag_mutability: Some(mutability.to_string()),
                        ..Default::default()
                    },
                )
            }
            Err(e) => ecr_error_response(&e),
        }
    }

    async fn handle_start_image_scan(
        &self,
        state: &Arc<tokio::sync::RwLock<EcrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_start_image_scan_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.repository_name.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "Missing 'repositoryName'",
            );
        }
        let image_id = ImageId {
            image_digest: input.image_id.image_digest.clone(),
            image_tag: input.image_id.image_tag.clone(),
        };

        let mut state = state.write().await;
        match state.start_image_scan(&input.repository_name, &image_id) {
            Ok((image, registry_id)) => {
                wire::serialize_start_image_scan_response(&wire::StartImageScanResponse {
                    registry_id: Some(registry_id.to_string()),
                    repository_name: Some(input.repository_name.clone()),
                    image_id: Some(wire::ImageIdentifier {
                        image_digest: Some(image.image_digest.clone()),
                        image_tag: image.image_tags.first().cloned(),
                    }),
                    image_scan_status: image.image_scan_status.as_ref().map(|s| {
                        wire::ImageScanStatus {
                            status: Some(s.status.clone()),
                            description: Some(s.description.clone()),
                        }
                    }),
                })
            }
            Err(e) => ecr_error_response(&e),
        }
    }

    async fn handle_describe_image_scan_findings(
        &self,
        state: &Arc<tokio::sync::RwLock<EcrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_image_scan_findings_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.repository_name.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "Missing 'repositoryName'",
            );
        }
        let image_id = ImageId {
            image_digest: input.image_id.image_digest.clone(),
            image_tag: input.image_id.image_tag.clone(),
        };

        let state = state.read().await;
        match state.describe_image_scan_findings(&input.repository_name, &image_id) {
            Ok((image, registry_id)) => {
                let findings =
                    image
                        .image_scan_findings
                        .as_ref()
                        .map(|f| wire::ImageScanFindings {
                            image_scan_completed_at: Some(
                                f.image_scan_completed_at.timestamp() as f64
                            ),
                            vulnerability_source_updated_at: Some(
                                f.vulnerability_source_updated_at.timestamp() as f64,
                            ),
                            finding_severity_counts: if f.finding_severity_counts.is_empty() {
                                None
                            } else {
                                Some(f.finding_severity_counts.clone())
                            },
                            findings: Some(Vec::new()),
                            enhanced_findings: None,
                        });

                wire::serialize_describe_image_scan_findings_response(
                    &wire::DescribeImageScanFindingsResponse {
                        registry_id: Some(registry_id.to_string()),
                        repository_name: Some(input.repository_name.clone()),
                        image_id: Some(wire::ImageIdentifier {
                            image_digest: Some(image.image_digest.clone()),
                            image_tag: image.image_tags.first().cloned(),
                        }),
                        image_scan_status: image.image_scan_status.as_ref().map(|s| {
                            wire::ImageScanStatus {
                                status: Some(s.status.clone()),
                                description: Some(s.description.clone()),
                            }
                        }),
                        image_scan_findings: findings,
                        next_token: None,
                    },
                )
            }
            Err(e) => ecr_error_response(&e),
        }
    }

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<EcrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(400, "InvalidParameterException", "Missing 'resourceArn'");
        }
        let tags: Vec<(String, String)> =
            input.tags.into_iter().map(|t| (t.key, t.value)).collect();

        let mut state = state.write().await;
        match state.tag_resource(&input.resource_arn, &tags) {
            Ok(()) => wire::serialize_tag_resource_response(&wire::TagResourceResponse {}),
            Err(e) => ecr_error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<EcrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(400, "InvalidParameterException", "Missing 'resourceArn'");
        }

        let mut state = state.write().await;
        match state.untag_resource(&input.resource_arn, &input.tag_keys) {
            Ok(()) => wire::serialize_untag_resource_response(&wire::UntagResourceResponse {}),
            Err(e) => ecr_error_response(&e),
        }
    }

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<EcrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(400, "InvalidParameterException", "Missing 'resourceArn'");
        }

        let state = state.read().await;
        match state.list_tags_for_resource(&input.resource_arn) {
            Ok(tags) => {
                let wire_tags: Vec<wire::Tag> = tags
                    .iter()
                    .map(|(k, v)| wire::Tag {
                        key: k.clone(),
                        value: v.clone(),
                    })
                    .collect();
                wire::serialize_list_tags_for_resource_response(
                    &wire::ListTagsForResourceResponse {
                        tags: Some(wire_tags),
                    },
                )
            }
            Err(e) => ecr_error_response(&e),
        }
    }

    async fn handle_put_registry_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<EcrState>>,
        body: &[u8],
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_put_registry_policy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.policy_text.is_empty() {
            return json_error_response(400, "InvalidParameterException", "Missing 'policyText'");
        }

        let mut state = state.write().await;
        state.put_registry_policy(&input.policy_text);
        wire::serialize_put_registry_policy_response(&wire::PutRegistryPolicyResponse {
            registry_id: Some(account_id.to_string()),
            policy_text: Some(input.policy_text.clone()),
        })
    }

    async fn handle_get_registry_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<EcrState>>,
        account_id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_registry_policy() {
            Ok(policy_text) => {
                wire::serialize_get_registry_policy_response(&wire::GetRegistryPolicyResponse {
                    registry_id: Some(account_id.to_string()),
                    policy_text: Some(policy_text.to_string()),
                })
            }
            Err(e) => ecr_error_response(&e),
        }
    }

    async fn handle_delete_registry_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<EcrState>>,
        account_id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_registry_policy() {
            Ok(policy_text) => wire::serialize_delete_registry_policy_response(
                &wire::DeleteRegistryPolicyResponse {
                    registry_id: Some(account_id.to_string()),
                    policy_text: Some(policy_text),
                },
            ),
            Err(e) => ecr_error_response(&e),
        }
    }

    async fn handle_describe_registry(
        &self,
        state: &Arc<tokio::sync::RwLock<EcrState>>,
        account_id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        let repl_config = state.describe_registry();
        let wire_repl = replication_config_to_wire(repl_config);
        let response = wire::DescribeRegistryResponse {
            registry_id: Some(account_id.to_string()),
            replication_configuration: Some(wire_repl),
        };
        wire::serialize_describe_registry_response(&response)
    }

    async fn handle_put_registry_scanning_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<EcrState>>,
        body: &[u8],
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_put_registry_scanning_configuration_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let scan_type = input.scan_type.as_deref();
        let rules: Vec<RegistryScanningRuleData> = input
            .rules
            .unwrap_or_default()
            .into_iter()
            .map(|rule| RegistryScanningRuleData {
                scan_frequency: if rule.scan_frequency.is_empty() {
                    "MANUAL".to_string()
                } else {
                    rule.scan_frequency
                },
                repository_filters: rule
                    .repository_filters
                    .into_iter()
                    .map(|f| ScanningRepositoryFilterData {
                        filter: f.filter,
                        filter_type: if f.filter_type.is_empty() {
                            "WILDCARD".to_string()
                        } else {
                            f.filter_type
                        },
                    })
                    .collect(),
            })
            .collect();

        let mut state_guard = state.write().await;
        let config = state_guard.put_registry_scanning_configuration(scan_type, rules);
        let _ = account_id;
        wire::serialize_put_registry_scanning_configuration_response(
            &wire::PutRegistryScanningConfigurationResponse {
                registry_scanning_configuration: Some(scanning_config_to_wire(config)),
            },
        )
    }

    async fn handle_get_registry_scanning_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<EcrState>>,
        account_id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        let config = state.get_registry_scanning_configuration();
        wire::serialize_get_registry_scanning_configuration_response(
            &wire::GetRegistryScanningConfigurationResponse {
                registry_id: Some(account_id.to_string()),
                scanning_configuration: Some(scanning_config_to_wire(config)),
            },
        )
    }

    async fn handle_put_replication_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<EcrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_put_replication_configuration_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let rules: Vec<ReplicationRuleData> = input
            .replication_configuration
            .rules
            .into_iter()
            .map(|rule| ReplicationRuleData {
                destinations: rule
                    .destinations
                    .into_iter()
                    .map(|d| ReplicationDestinationData {
                        region: d.region,
                        registry_id: d.registry_id,
                    })
                    .collect(),
                repository_filters: rule
                    .repository_filters
                    .unwrap_or_default()
                    .into_iter()
                    .map(|f| RepositoryFilterData {
                        filter: f.filter,
                        filter_type: if f.filter_type.is_empty() {
                            "PREFIX_MATCH".to_string()
                        } else {
                            f.filter_type
                        },
                    })
                    .collect(),
            })
            .collect();

        let mut state_guard = state.write().await;
        let config = state_guard.put_replication_configuration(rules);
        wire::serialize_put_replication_configuration_response(
            &wire::PutReplicationConfigurationResponse {
                replication_configuration: Some(replication_config_to_wire(config)),
            },
        )
    }

    // --- Layer uploads ---

    async fn handle_initiate_layer_upload(
        &self,
        state: &Arc<tokio::sync::RwLock<EcrState>>,
        body: &[u8],
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_initiate_layer_upload_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.repository_name.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "Missing 'repositoryName'",
            );
        }
        let registry_id = input.registry_id.as_deref().unwrap_or(account_id);
        let mut state = state.write().await;
        match state.initiate_layer_upload(registry_id, &input.repository_name) {
            Ok(upload_id) => {
                wire::serialize_initiate_layer_upload_response(&wire::InitiateLayerUploadResponse {
                    upload_id: Some(upload_id),
                    part_size: Some(10 * 1024 * 1024),
                })
            }
            Err(e) => ecr_error_response(&e),
        }
    }

    async fn handle_upload_layer_part(
        &self,
        state: &Arc<tokio::sync::RwLock<EcrState>>,
        body: &[u8],
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_upload_layer_part_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.upload_id.is_empty() {
            return json_error_response(400, "InvalidParameterException", "Missing 'uploadId'");
        }
        // layer_part_blob is base64 encoded but we don't need to decode it for the mock
        let mut state = state.write().await;
        match state.upload_layer_part(
            &input.upload_id,
            &[],
            input.part_first_byte,
            input.part_last_byte,
        ) {
            Ok(last_byte) => {
                wire::serialize_upload_layer_part_response(&wire::UploadLayerPartResponse {
                    registry_id: Some(account_id.to_string()),
                    repository_name: Some(input.repository_name.clone()),
                    upload_id: Some(input.upload_id.clone()),
                    last_byte_received: Some(last_byte),
                })
            }
            Err(e) => ecr_error_response(&e),
        }
    }

    async fn handle_complete_layer_upload(
        &self,
        state: &Arc<tokio::sync::RwLock<EcrState>>,
        body: &[u8],
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_complete_layer_upload_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.upload_id.is_empty() {
            return json_error_response(400, "InvalidParameterException", "Missing 'uploadId'");
        }
        let mut state = state.write().await;
        match state.complete_layer_upload(&input.upload_id, &input.layer_digests, account_id) {
            Ok((registry_id, repo_name, layer_digest)) => {
                wire::serialize_complete_layer_upload_response(&wire::CompleteLayerUploadResponse {
                    registry_id: Some(registry_id),
                    repository_name: Some(repo_name),
                    upload_id: Some(input.upload_id.clone()),
                    layer_digest: Some(layer_digest),
                })
            }
            Err(e) => ecr_error_response(&e),
        }
    }

    async fn handle_batch_check_layer_availability(
        &self,
        state: &Arc<tokio::sync::RwLock<EcrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_batch_check_layer_availability_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.repository_name.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "Missing 'repositoryName'",
            );
        }
        let state = state.read().await;
        match state.batch_check_layer_availability(&input.repository_name, &input.layer_digests) {
            Ok((layers, _failures)) => {
                let wire_layers: Vec<wire::Layer> = layers
                    .iter()
                    .map(|(digest, avail)| wire::Layer {
                        layer_digest: Some(digest.clone()),
                        layer_availability: Some(avail.clone()),
                        layer_size: None,
                        media_type: None,
                    })
                    .collect();
                wire::serialize_batch_check_layer_availability_response(
                    &wire::BatchCheckLayerAvailabilityResponse {
                        layers: Some(wire_layers),
                        failures: Some(Vec::new()),
                    },
                )
            }
            Err(e) => ecr_error_response(&e),
        }
    }

    async fn handle_get_download_url_for_layer(
        &self,
        state: &Arc<tokio::sync::RwLock<EcrState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_get_download_url_for_layer_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.repository_name.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "Missing 'repositoryName'",
            );
        }
        if input.layer_digest.is_empty() {
            return json_error_response(400, "InvalidParameterException", "Missing 'layerDigest'");
        }
        let state = state.read().await;
        match state.get_download_url_for_layer(
            &input.repository_name,
            &input.layer_digest,
            account_id,
            region,
        ) {
            Ok(url) => wire::serialize_get_download_url_for_layer_response(
                &wire::GetDownloadUrlForLayerResponse {
                    download_url: Some(url),
                    layer_digest: Some(input.layer_digest.clone()),
                },
            ),
            Err(e) => ecr_error_response(&e),
        }
    }

    // --- Pull-through cache rules ---

    async fn handle_create_pull_through_cache_rule(
        &self,
        state: &Arc<tokio::sync::RwLock<EcrState>>,
        body: &[u8],
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_pull_through_cache_rule_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.ecr_repository_prefix.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "Missing 'ecrRepositoryPrefix'",
            );
        }
        if input.upstream_registry_url.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "Missing 'upstreamRegistryUrl'",
            );
        }
        let upstream_registry = input.upstream_registry.as_deref();
        let upstream_repository_prefix = input.upstream_repository_prefix.as_deref();
        let credential_arn = input.credential_arn.as_deref();
        let custom_role_arn = input.custom_role_arn.as_deref();

        let mut state = state.write().await;
        match state.create_pull_through_cache_rule(
            &input.ecr_repository_prefix,
            &input.upstream_registry_url,
            account_id,
            upstream_registry,
            upstream_repository_prefix,
            credential_arn,
            custom_role_arn,
        ) {
            Ok(rule) => wire::serialize_create_pull_through_cache_rule_response(
                &wire::CreatePullThroughCacheRuleResponse {
                    ecr_repository_prefix: Some(rule.ecr_repository_prefix.clone()),
                    upstream_registry_url: Some(rule.upstream_registry_url.clone()),
                    created_at: Some(rule.created_at.timestamp() as f64),
                    registry_id: Some(rule.registry_id.clone()),
                    upstream_registry: rule.upstream_registry.clone(),
                    upstream_repository_prefix: rule.upstream_repository_prefix.clone(),
                    credential_arn: rule.credential_arn.clone(),
                    custom_role_arn: rule.custom_role_arn.clone(),
                },
            ),
            Err(e) => ecr_error_response(&e),
        }
    }

    async fn handle_delete_pull_through_cache_rule(
        &self,
        state: &Arc<tokio::sync::RwLock<EcrState>>,
        body: &[u8],
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_pull_through_cache_rule_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.ecr_repository_prefix.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "Missing 'ecrRepositoryPrefix'",
            );
        }
        let mut state = state.write().await;
        match state.delete_pull_through_cache_rule(&input.ecr_repository_prefix) {
            Ok(rule) => wire::serialize_delete_pull_through_cache_rule_response(
                &wire::DeletePullThroughCacheRuleResponse {
                    ecr_repository_prefix: Some(rule.ecr_repository_prefix),
                    upstream_registry_url: Some(rule.upstream_registry_url),
                    created_at: Some(rule.created_at.timestamp() as f64),
                    registry_id: Some(account_id.to_string()),
                    credential_arn: rule.credential_arn,
                    custom_role_arn: rule.custom_role_arn,
                    upstream_repository_prefix: rule.upstream_repository_prefix,
                },
            ),
            Err(e) => ecr_error_response(&e),
        }
    }

    async fn handle_describe_pull_through_cache_rules(
        &self,
        state: &Arc<tokio::sync::RwLock<EcrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_pull_through_cache_rules_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let prefixes = input.ecr_repository_prefixes;
        let state = state.read().await;
        let rules = state.describe_pull_through_cache_rules(prefixes.as_deref());
        let wire_rules: Vec<wire::PullThroughCacheRule> = rules
            .iter()
            .map(|r| wire::PullThroughCacheRule {
                ecr_repository_prefix: Some(r.ecr_repository_prefix.clone()),
                upstream_registry_url: Some(r.upstream_registry_url.clone()),
                created_at: Some(r.created_at.timestamp() as f64),
                registry_id: Some(r.registry_id.clone()),
                upstream_registry: r.upstream_registry.clone(),
                upstream_repository_prefix: r.upstream_repository_prefix.clone(),
                credential_arn: r.credential_arn.clone(),
                custom_role_arn: r.custom_role_arn.clone(),
                updated_at: None,
            })
            .collect();
        wire::serialize_describe_pull_through_cache_rules_response(
            &wire::DescribePullThroughCacheRulesResponse {
                pull_through_cache_rules: Some(wire_rules),
                next_token: None,
            },
        )
    }

    async fn handle_update_pull_through_cache_rule(
        &self,
        state: &Arc<tokio::sync::RwLock<EcrState>>,
        body: &[u8],
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_update_pull_through_cache_rule_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.ecr_repository_prefix.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "Missing 'ecrRepositoryPrefix'",
            );
        }
        let credential_arn = input.credential_arn.as_deref();
        let custom_role_arn = input.custom_role_arn.as_deref();
        let mut state = state.write().await;
        match state.update_pull_through_cache_rule(
            &input.ecr_repository_prefix,
            credential_arn,
            custom_role_arn,
            account_id,
        ) {
            Ok(rule) => wire::serialize_update_pull_through_cache_rule_response(
                &wire::UpdatePullThroughCacheRuleResponse {
                    ecr_repository_prefix: Some(rule.ecr_repository_prefix.clone()),
                    registry_id: Some(account_id.to_string()),
                    credential_arn: rule.credential_arn.clone(),
                    custom_role_arn: rule.custom_role_arn.clone(),
                    updated_at: None,
                    upstream_repository_prefix: rule.upstream_repository_prefix.clone(),
                },
            ),
            Err(e) => ecr_error_response(&e),
        }
    }

    async fn handle_validate_pull_through_cache_rule(
        &self,
        state: &Arc<tokio::sync::RwLock<EcrState>>,
        body: &[u8],
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_validate_pull_through_cache_rule_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.ecr_repository_prefix.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "Missing 'ecrRepositoryPrefix'",
            );
        }
        let state = state.read().await;
        match state.validate_pull_through_cache_rule(&input.ecr_repository_prefix, account_id) {
            Ok(rule) => wire::serialize_validate_pull_through_cache_rule_response(
                &wire::ValidatePullThroughCacheRuleResponse {
                    ecr_repository_prefix: Some(rule.ecr_repository_prefix.clone()),
                    registry_id: Some(account_id.to_string()),
                    upstream_registry_url: Some(rule.upstream_registry_url.clone()),
                    credential_arn: rule.credential_arn.clone(),
                    custom_role_arn: rule.custom_role_arn.clone(),
                    is_valid: Some(true),
                    failure: None,
                    upstream_repository_prefix: rule.upstream_repository_prefix.clone(),
                },
            ),
            Err(e) => ecr_error_response(&e),
        }
    }

    // --- Repository creation templates ---

    async fn handle_create_repository_creation_template(
        &self,
        state: &Arc<tokio::sync::RwLock<EcrState>>,
        body: &[u8],
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_repository_creation_template_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.prefix.is_empty() {
            return json_error_response(400, "InvalidParameterException", "Missing 'prefix'");
        }
        let description = input.description.as_deref();
        let lifecycle_policy = input.lifecycle_policy.as_deref();
        let repository_policy = input.repository_policy.as_deref();
        let image_tag_mutability = input.image_tag_mutability.as_deref();
        let custom_role_arn = input.custom_role_arn.as_deref();
        let applied_for = input.applied_for;

        let mut state = state.write().await;
        match state.create_repository_creation_template(
            &input.prefix,
            description,
            lifecycle_policy,
            repository_policy,
            image_tag_mutability,
            custom_role_arn,
            applied_for,
        ) {
            Ok(t) => wire::serialize_create_repository_creation_template_response(
                &wire::CreateRepositoryCreationTemplateResponse {
                    registry_id: Some(account_id.to_string()),
                    repository_creation_template: Some(template_to_wire(t)),
                },
            ),
            Err(e) => ecr_error_response(&e),
        }
    }

    async fn handle_delete_repository_creation_template(
        &self,
        state: &Arc<tokio::sync::RwLock<EcrState>>,
        body: &[u8],
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_repository_creation_template_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.prefix.is_empty() {
            return json_error_response(400, "InvalidParameterException", "Missing 'prefix'");
        }
        let mut state = state.write().await;
        match state.delete_repository_creation_template(&input.prefix) {
            Ok(t) => wire::serialize_delete_repository_creation_template_response(
                &wire::DeleteRepositoryCreationTemplateResponse {
                    registry_id: Some(account_id.to_string()),
                    repository_creation_template: Some(template_to_wire(&t)),
                },
            ),
            Err(e) => ecr_error_response(&e),
        }
    }

    async fn handle_describe_repository_creation_templates(
        &self,
        state: &Arc<tokio::sync::RwLock<EcrState>>,
        body: &[u8],
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_repository_creation_templates_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let prefixes = input.prefixes;
        let state = state.read().await;
        let templates = state.describe_repository_creation_templates(prefixes.as_deref());
        let wire_templates: Vec<wire::RepositoryCreationTemplate> =
            templates.iter().map(|t| template_to_wire(t)).collect();
        wire::serialize_describe_repository_creation_templates_response(
            &wire::DescribeRepositoryCreationTemplatesResponse {
                registry_id: Some(account_id.to_string()),
                repository_creation_templates: Some(wire_templates),
                next_token: None,
            },
        )
    }

    async fn handle_update_repository_creation_template(
        &self,
        state: &Arc<tokio::sync::RwLock<EcrState>>,
        body: &[u8],
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_update_repository_creation_template_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.prefix.is_empty() {
            return json_error_response(400, "InvalidParameterException", "Missing 'prefix'");
        }
        let description = input.description.as_deref();
        let lifecycle_policy = input.lifecycle_policy.as_deref();
        let repository_policy = input.repository_policy.as_deref();
        let image_tag_mutability = input.image_tag_mutability.as_deref();
        let custom_role_arn = input.custom_role_arn.as_deref();
        let applied_for = input.applied_for;

        let mut state = state.write().await;
        match state.update_repository_creation_template(
            &input.prefix,
            description,
            lifecycle_policy,
            repository_policy,
            image_tag_mutability,
            custom_role_arn,
            applied_for,
        ) {
            Ok(t) => wire::serialize_update_repository_creation_template_response(
                &wire::UpdateRepositoryCreationTemplateResponse {
                    registry_id: Some(account_id.to_string()),
                    repository_creation_template: Some(template_to_wire(t)),
                },
            ),
            Err(e) => ecr_error_response(&e),
        }
    }

    // --- Signing configuration ---

    async fn handle_get_signing_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<EcrState>>,
        account_id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        let config = state.get_signing_configuration();
        wire::serialize_get_signing_configuration_response(&wire::GetSigningConfigurationResponse {
            registry_id: Some(account_id.to_string()),
            signing_configuration: Some(signing_config_data_to_wire(config)),
        })
    }

    async fn handle_put_signing_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<EcrState>>,
        body: &[u8],
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_put_signing_configuration_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let _ = account_id;
        let rules: Vec<SigningRuleData> = input
            .signing_configuration
            .rules
            .into_iter()
            .map(|r| SigningRuleData {
                signing_profile_arn: r.signing_profile_arn,
                repository_filters: r
                    .repository_filters
                    .unwrap_or_default()
                    .into_iter()
                    .map(|f| SigningRepositoryFilterData {
                        filter: f.filter,
                        filter_type: if f.filter_type.is_empty() {
                            "WILDCARD".to_string()
                        } else {
                            f.filter_type
                        },
                    })
                    .collect(),
            })
            .collect();
        let mut state = state.write().await;
        let config = state.put_signing_configuration(rules);
        wire::serialize_put_signing_configuration_response(&wire::PutSigningConfigurationResponse {
            signing_configuration: Some(signing_config_data_to_wire(config)),
        })
    }

    async fn handle_delete_signing_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<EcrState>>,
        account_id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        let config_snap = state.get_signing_configuration().clone();
        state.delete_signing_configuration();
        wire::serialize_delete_signing_configuration_response(
            &wire::DeleteSigningConfigurationResponse {
                registry_id: Some(account_id.to_string()),
                signing_configuration: Some(signing_config_data_to_wire(&config_snap)),
            },
        )
    }

    // --- Pull-time update exclusions ---

    async fn handle_register_pull_time_update_exclusion(
        &self,
        state: &Arc<tokio::sync::RwLock<EcrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_register_pull_time_update_exclusion_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.principal_arn.is_empty() {
            return json_error_response(400, "InvalidParameterException", "Missing 'principalArn'");
        }
        let mut state = state.write().await;
        let entry = state.register_pull_time_update_exclusion(&input.principal_arn);
        wire::serialize_register_pull_time_update_exclusion_response(
            &wire::RegisterPullTimeUpdateExclusionResponse {
                principal_arn: Some(entry.principal_arn.clone()),
                created_at: Some(entry.created_at.timestamp() as f64),
            },
        )
    }

    async fn handle_deregister_pull_time_update_exclusion(
        &self,
        state: &Arc<tokio::sync::RwLock<EcrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_deregister_pull_time_update_exclusion_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.principal_arn.is_empty() {
            return json_error_response(400, "InvalidParameterException", "Missing 'principalArn'");
        }
        let mut state = state.write().await;
        match state.deregister_pull_time_update_exclusion(&input.principal_arn) {
            Ok(arn) => wire::serialize_deregister_pull_time_update_exclusion_response(
                &wire::DeregisterPullTimeUpdateExclusionResponse {
                    principal_arn: Some(arn),
                },
            ),
            Err(e) => ecr_error_response(&e),
        }
    }

    async fn handle_list_pull_time_update_exclusions(
        &self,
        state: &Arc<tokio::sync::RwLock<EcrState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let exclusions = state.list_pull_time_update_exclusions();
        wire::serialize_list_pull_time_update_exclusions_response(
            &wire::ListPullTimeUpdateExclusionsResponse {
                pull_time_update_exclusions: Some(exclusions),
                next_token: None,
            },
        )
    }

    // --- Account settings ---

    async fn handle_get_account_setting(
        &self,
        state: &Arc<tokio::sync::RwLock<EcrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_account_setting_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "InvalidParameterException", "Missing 'name'");
        }
        let state = state.read().await;
        let value = state
            .get_account_setting(&input.name)
            .cloned()
            .unwrap_or_default();
        wire::serialize_get_account_setting_response(&wire::GetAccountSettingResponse {
            name: Some(input.name.clone()),
            value: Some(value),
        })
    }

    async fn handle_put_account_setting(
        &self,
        state: &Arc<tokio::sync::RwLock<EcrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_put_account_setting_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "InvalidParameterException", "Missing 'name'");
        }
        if input.value.is_empty() {
            return json_error_response(400, "InvalidParameterException", "Missing 'value'");
        }
        let mut state = state.write().await;
        let (n, v) = state.put_account_setting(&input.name, &input.value);
        wire::serialize_put_account_setting_response(&wire::PutAccountSettingResponse {
            name: Some(n.to_string()),
            value: Some(v.to_string()),
        })
    }

    // --- Lifecycle policy preview ---

    async fn handle_start_lifecycle_policy_preview(
        &self,
        state: &Arc<tokio::sync::RwLock<EcrState>>,
        body: &[u8],
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_start_lifecycle_policy_preview_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.repository_name.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "Missing 'repositoryName'",
            );
        }
        let lifecycle_policy_text = input.lifecycle_policy_text.clone();
        let state = state.read().await;
        if !state
            .repositories
            .contains_key(input.repository_name.as_str())
        {
            return ecr_error_response(&EcrError::RepositoryNotFound {
                repository_name: input.repository_name.clone(),
            });
        }
        let policy = lifecycle_policy_text.or_else(|| {
            state
                .repositories
                .get(input.repository_name.as_str())
                .and_then(|r| r.lifecycle_policy.clone())
        });
        wire::serialize_start_lifecycle_policy_preview_response(
            &wire::StartLifecyclePolicyPreviewResponse {
                registry_id: Some(account_id.to_string()),
                repository_name: Some(input.repository_name.clone()),
                lifecycle_policy_text: policy,
                status: Some("IN_PROGRESS".to_string()),
            },
        )
    }

    async fn handle_get_lifecycle_policy_preview(
        &self,
        state: &Arc<tokio::sync::RwLock<EcrState>>,
        body: &[u8],
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_get_lifecycle_policy_preview_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.repository_name.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "Missing 'repositoryName'",
            );
        }
        let state = state.read().await;
        if !state
            .repositories
            .contains_key(input.repository_name.as_str())
        {
            return ecr_error_response(&EcrError::RepositoryNotFound {
                repository_name: input.repository_name.clone(),
            });
        }
        let policy = state
            .repositories
            .get(input.repository_name.as_str())
            .and_then(|r| r.lifecycle_policy.clone());
        wire::serialize_get_lifecycle_policy_preview_response(
            &wire::GetLifecyclePolicyPreviewResponse {
                registry_id: Some(account_id.to_string()),
                repository_name: Some(input.repository_name.clone()),
                lifecycle_policy_text: policy,
                status: Some("COMPLETE".to_string()),
                preview_results: Some(Vec::new()),
                next_token: None,
                summary: None,
            },
        )
    }

    // --- Misc operations ---

    async fn handle_describe_image_replication_status(
        &self,
        state: &Arc<tokio::sync::RwLock<EcrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_image_replication_status_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.repository_name.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "Missing 'repositoryName'",
            );
        }
        let state = state.read().await;
        if !state
            .repositories
            .contains_key(input.repository_name.as_str())
        {
            return json_error_response(
                400,
                "RepositoryNotFoundException",
                &format!(
                    "The repository with name '{}' does not exist in the registry",
                    input.repository_name
                ),
            );
        }
        wire::serialize_describe_image_replication_status_response(
            &wire::DescribeImageReplicationStatusResponse {
                repository_name: Some(input.repository_name.clone()),
                image_id: Some(wire::ImageIdentifier {
                    image_digest: input.image_id.image_digest.clone(),
                    image_tag: input.image_id.image_tag.clone(),
                }),
                replication_statuses: Some(Vec::new()),
            },
        )
    }

    async fn handle_describe_image_signing_status(
        &self,
        state: &Arc<tokio::sync::RwLock<EcrState>>,
        body: &[u8],
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_image_signing_status_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.repository_name.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "Missing 'repositoryName'",
            );
        }
        let state = state.read().await;
        if !state
            .repositories
            .contains_key(input.repository_name.as_str())
        {
            return json_error_response(
                400,
                "RepositoryNotFoundException",
                &format!(
                    "The repository with name '{}' does not exist in the registry",
                    input.repository_name
                ),
            );
        }
        wire::serialize_describe_image_signing_status_response(
            &wire::DescribeImageSigningStatusResponse {
                registry_id: Some(account_id.to_string()),
                repository_name: Some(input.repository_name.clone()),
                image_id: Some(wire::ImageIdentifier {
                    image_digest: input.image_id.image_digest.clone(),
                    image_tag: input.image_id.image_tag.clone(),
                }),
                signing_statuses: Some(Vec::new()),
            },
        )
    }

    async fn handle_list_image_referrers(
        &self,
        state: &Arc<tokio::sync::RwLock<EcrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_image_referrers_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.repository_name.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "Missing 'repositoryName'",
            );
        }
        let state = state.read().await;
        if !state
            .repositories
            .contains_key(input.repository_name.as_str())
        {
            return json_error_response(
                400,
                "RepositoryNotFoundException",
                &format!(
                    "The repository with name '{}' does not exist in the registry",
                    input.repository_name
                ),
            );
        }
        wire::serialize_list_image_referrers_response(&wire::ListImageReferrersResponse {
            referrers: Some(Vec::new()),
            next_token: None,
        })
    }

    async fn handle_update_image_storage_class(
        &self,
        state: &Arc<tokio::sync::RwLock<EcrState>>,
        body: &[u8],
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_update_image_storage_class_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.repository_name.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "Missing 'repositoryName'",
            );
        }
        wire::serialize_update_image_storage_class_response(
            &wire::UpdateImageStorageClassResponse {
                registry_id: Some(account_id.to_string()),
                repository_name: Some(input.repository_name.clone()),
                image_id: Some(wire::ImageIdentifier {
                    image_digest: input.image_id.image_digest.clone(),
                    image_tag: input.image_id.image_tag.clone(),
                }),
                image_status: Some("UPDATE_COMPLETE".to_string()),
            },
        )
    }

    async fn handle_batch_get_repository_scanning_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<EcrState>>,
        body: &[u8],
    ) -> MockResponse {
        let input =
            match wire::deserialize_batch_get_repository_scanning_configuration_request(body) {
                Ok(v) => v,
                Err(e) => return json_error_response(400, "ValidationException", &e),
            };
        let repository_names = input.repository_names;

        let state = state.read().await;
        let result = state.batch_get_repository_scanning_configuration(&repository_names);

        let configs: Vec<wire::RepositoryScanningConfiguration> = result
            .scanning_configurations
            .iter()
            .map(|c| wire::RepositoryScanningConfiguration {
                repository_name: Some(c.repository_name.clone()),
                repository_arn: Some(c.repository_arn.clone()),
                scan_on_push: Some(c.scan_on_push),
                scan_frequency: Some(c.scan_frequency.clone()),
                applied_scan_filters: if c.applied_scan_filters.is_empty() {
                    None
                } else {
                    Some(
                        c.applied_scan_filters
                            .iter()
                            .map(|f| wire::ScanningRepositoryFilter {
                                filter: f.filter.clone(),
                                filter_type: f.filter_type.clone(),
                            })
                            .collect(),
                    )
                },
            })
            .collect();

        let failures: Vec<wire::RepositoryScanningConfigurationFailure> = result
            .failures
            .iter()
            .map(|f| wire::RepositoryScanningConfigurationFailure {
                repository_name: Some(f.repository_name.clone()),
                failure_code: Some(f.failure_code.clone()),
                failure_reason: Some(f.failure_reason.clone()),
            })
            .collect();

        wire::serialize_batch_get_repository_scanning_configuration_response(
            &wire::BatchGetRepositoryScanningConfigurationResponse {
                scanning_configurations: Some(configs),
                failures: Some(failures),
            },
        )
    }
}

// --- Utility functions ---

fn repository_to_wire(repo: &Repository) -> wire::Repository {
    wire::Repository {
        repository_name: Some(repo.repository_name.clone()),
        repository_arn: Some(repo.repository_arn.clone()),
        repository_uri: Some(repo.repository_uri.clone()),
        registry_id: Some(repo.registry_id.clone()),
        created_at: Some(repo.created_at.timestamp() as f64),
        image_tag_mutability: Some(repo.image_tag_mutability.clone()),
        image_scanning_configuration: Some(wire::ImageScanningConfiguration {
            scan_on_push: Some(repo.image_scanning_configuration.scan_on_push),
        }),
        encryption_configuration: Some(wire::EncryptionConfiguration {
            encryption_type: repo.encryption_configuration.encryption_type.clone(),
            kms_key: repo.encryption_configuration.kms_key.clone(),
        }),
        ..Default::default()
    }
}

fn scanning_config_to_wire(config: &RegistryScanningConfig) -> wire::RegistryScanningConfiguration {
    wire::RegistryScanningConfiguration {
        scan_type: Some(config.scan_type.clone()),
        rules: if config.rules.is_empty() {
            None
        } else {
            Some(
                config
                    .rules
                    .iter()
                    .map(|r| wire::RegistryScanningRule {
                        scan_frequency: r.scan_frequency.clone(),
                        repository_filters: r
                            .repository_filters
                            .iter()
                            .map(|f| wire::ScanningRepositoryFilter {
                                filter: f.filter.clone(),
                                filter_type: f.filter_type.clone(),
                            })
                            .collect(),
                    })
                    .collect(),
            )
        },
    }
}

fn replication_config_to_wire(config: &ReplicationConfig) -> wire::ReplicationConfiguration {
    wire::ReplicationConfiguration {
        rules: config
            .rules
            .iter()
            .map(|r| wire::ReplicationRule {
                destinations: r
                    .destinations
                    .iter()
                    .map(|d| wire::ReplicationDestination {
                        region: d.region.clone(),
                        registry_id: d.registry_id.clone(),
                    })
                    .collect(),
                repository_filters: if r.repository_filters.is_empty() {
                    None
                } else {
                    Some(
                        r.repository_filters
                            .iter()
                            .map(|f| wire::RepositoryFilter {
                                filter: f.filter.clone(),
                                filter_type: f.filter_type.clone(),
                            })
                            .collect(),
                    )
                },
            })
            .collect(),
    }
}

fn model_image_ids_to_state(ids: &[model::ImageIdentifier]) -> Vec<ImageId> {
    ids.iter()
        .map(|id| ImageId {
            image_digest: id.image_digest.clone(),
            image_tag: id.image_tag.clone(),
        })
        .collect()
}

fn template_to_wire(t: &RepositoryCreationTemplateData) -> wire::RepositoryCreationTemplate {
    wire::RepositoryCreationTemplate {
        prefix: Some(t.prefix.clone()),
        description: t.description.clone(),
        lifecycle_policy: t.lifecycle_policy.clone(),
        repository_policy: t.repository_policy.clone(),
        image_tag_mutability: t.image_tag_mutability.clone(),
        custom_role_arn: t.custom_role_arn.clone(),
        applied_for: if t.applied_for.is_empty() {
            None
        } else {
            Some(t.applied_for.clone())
        },
        created_at: Some(t.created_at.timestamp() as f64),
        updated_at: Some(t.updated_at.timestamp() as f64),
        ..Default::default()
    }
}

fn signing_config_data_to_wire(config: &SigningConfigData) -> wire::SigningConfiguration {
    wire::SigningConfiguration {
        rules: config
            .rules
            .iter()
            .map(|r| wire::SigningRule {
                signing_profile_arn: r.signing_profile_arn.clone(),
                repository_filters: if r.repository_filters.is_empty() {
                    None
                } else {
                    Some(
                        r.repository_filters
                            .iter()
                            .map(|f| wire::SigningRepositoryFilter {
                                filter: f.filter.clone(),
                                filter_type: f.filter_type.clone(),
                            })
                            .collect(),
                    )
                },
            })
            .collect(),
    }
}

fn ecr_error_response(err: &EcrError) -> MockResponse {
    let (status, error_type) = match err {
        EcrError::RepositoryAlreadyExists { .. } => (400, "RepositoryAlreadyExistsException"),
        EcrError::RepositoryNotEmpty { .. } => (400, "RepositoryNotEmptyException"),
        EcrError::RepositoryNotFound { .. } => (400, "RepositoryNotFoundException"),
        EcrError::ImageAlreadyExists { .. } => (400, "ImageAlreadyExistsException"),
        EcrError::ImageNotFound { .. } => (400, "ImageNotFoundException"),
        EcrError::LifecyclePolicyNotFound { .. } => (400, "LifecyclePolicyNotFoundException"),
        EcrError::RepositoryPolicyNotFound { .. } => (400, "RepositoryPolicyNotFoundException"),
        EcrError::ScanNotFound { .. } => (400, "ScanNotFoundException"),
        EcrError::InvalidParameter { .. } => (400, "InvalidParameterException"),
        EcrError::RegistryPolicyNotFound => (400, "RegistryPolicyNotFoundException"),
        EcrError::UploadNotFound { .. } => (400, "UploadNotFoundException"),
        EcrError::PullThroughCacheRuleAlreadyExists { .. } => {
            (400, "PullThroughCacheRuleAlreadyExistsException")
        }
        EcrError::PullThroughCacheRuleNotFound { .. } => {
            (400, "PullThroughCacheRuleNotFoundException")
        }
        EcrError::TemplateAlreadyExists { .. } => (400, "TemplateAlreadyExistsException"),
        EcrError::TemplateNotFound { .. } => (400, "TemplateNotFoundException"),
        EcrError::PullTimeUpdateExclusionNotFound { .. } => {
            (400, "PullTimeUpdateExclusionNotFoundException")
        }
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
