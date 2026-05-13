use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, default_account_id,
};

use crate::state::{SecretsManagerError, SecretsManagerState};
use crate::views::SecretsmanagerStateView;
use crate::wire;

pub struct SecretsManagerService {
    pub(crate) state: Arc<BackendState<SecretsManagerState>>,
    pub(crate) notifier: StateChangeNotifier<SecretsmanagerStateView>,
}

impl SecretsManagerService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for SecretsManagerService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for SecretsManagerService {
    fn service_name(&self) -> &str {
        "secretsmanager"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://secretsmanager\..*\.amazonaws\.com",
            r"https?://secretsmanager\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl SecretsManagerService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();

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

        let response = match action.as_str() {
            "CreateSecret" => {
                self.handle_create_secret(&state, body_bytes, account_id, &region)
                    .await
            }
            "GetSecretValue" => self.handle_get_secret_value(&state, body_bytes).await,
            "PutSecretValue" => self.handle_put_secret_value(&state, body_bytes).await,
            "DeleteSecret" => {
                self.handle_delete_secret(&state, body_bytes, account_id, &region)
                    .await
            }
            "RestoreSecret" => self.handle_restore_secret(&state, body_bytes).await,
            "DescribeSecret" => self.handle_describe_secret(&state, body_bytes).await,
            "ListSecrets" => self.handle_list_secrets(&state).await,
            "UpdateSecret" => self.handle_update_secret(&state, body_bytes).await,
            "BatchGetSecretValue" => self.handle_batch_get_secret_value(&state, body_bytes).await,
            "CancelRotateSecret" => self.handle_cancel_rotate_secret(&state, body_bytes).await,
            "DeleteResourcePolicy" => self.handle_delete_resource_policy(&state, body_bytes).await,
            "GetRandomPassword" => self.handle_get_random_password(&state, body_bytes).await,
            "GetResourcePolicy" => self.handle_get_resource_policy(&state, body_bytes).await,
            "ListSecretVersionIds" => {
                self.handle_list_secret_version_ids(&state, body_bytes)
                    .await
            }
            "PutResourcePolicy" => self.handle_put_resource_policy(&state, body_bytes).await,
            "RemoveRegionsFromReplication" => {
                self.handle_remove_regions_from_replication(&state, body_bytes)
                    .await
            }
            "ReplicateSecretToRegions" => {
                self.handle_replicate_secret_to_regions(&state, body_bytes)
                    .await
            }
            "RotateSecret" => self.handle_rotate_secret(&state, body_bytes).await,
            "TagResource" => self.handle_tag_resource(&state, body_bytes).await,
            "UntagResource" => self.handle_untag_resource(&state, body_bytes).await,
            "UpdateSecretVersionStage" => {
                self.handle_update_secret_version_stage(&state, body_bytes)
                    .await
            }
            "StopReplicationToReplica" => {
                self.handle_stop_replication_to_replica(&state, body_bytes)
                    .await
            }
            "ValidateResourcePolicy" => {
                self.handle_validate_resource_policy(&state, body_bytes)
                    .await
            }
            _ => json_error_response(
                400,
                "InvalidAction",
                &format!("Could not find operation {action} for SecretsManager"),
            ),
        };
        use winterbaume_core::StatefulService;
        if response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_create_secret(
        &self,
        state: &Arc<tokio::sync::RwLock<SecretsManagerState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_secret_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Name'");
        }
        let description = input.description.unwrap_or_default();
        let secret_string = input.secret_string.as_deref();
        let tags = tags_from_wire(input.tags.unwrap_or_default());

        let mut state = state.write().await;
        match state.create_secret(
            &input.name,
            &description,
            secret_string,
            None,
            account_id,
            region,
            tags,
        ) {
            Ok(secret) => wire::serialize_create_secret_response(&wire::CreateSecretResponse {
                a_r_n: Some(secret.arn.clone()),
                name: Some(secret.name.clone()),
                version_id: secret.current_version_id.clone(),
                ..Default::default()
            }),
            Err(e) => sm_error_response(&e),
        }
    }

    async fn handle_get_secret_value(
        &self,
        state: &Arc<tokio::sync::RwLock<SecretsManagerState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_secret_value_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.secret_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'SecretId'");
        }

        let state = state.read().await;
        let result = if let Some(vid) = input.version_id.as_deref() {
            state.get_secret_value_by_version_id(&input.secret_id, vid)
        } else {
            state.get_secret_value_by_stage(&input.secret_id, input.version_stage.as_deref())
        };
        match result {
            Ok((secret, version)) => {
                wire::serialize_get_secret_value_response(&wire::GetSecretValueResponse {
                    a_r_n: Some(secret.arn.clone()),
                    name: Some(secret.name.clone()),
                    version_id: Some(version.version_id.clone()),
                    version_stages: Some(version.version_stages.clone()),
                    created_date: Some(version.created_date.timestamp() as f64),
                    secret_string: version.secret_string.clone(),
                    ..Default::default()
                })
            }
            Err(e) => sm_error_response(&e),
        }
    }

    async fn handle_put_secret_value(
        &self,
        state: &Arc<tokio::sync::RwLock<SecretsManagerState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_put_secret_value_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.secret_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'SecretId'");
        }

        let mut state = state.write().await;
        match state.put_secret_value_ext(
            &input.secret_id,
            input.secret_string.as_deref(),
            None,
            input.client_request_token.as_deref(),
            input.version_stages.as_deref(),
        ) {
            Ok((secret, version_id, stages)) => {
                wire::serialize_put_secret_value_response(&wire::PutSecretValueResponse {
                    a_r_n: Some(secret.arn.clone()),
                    name: Some(secret.name.clone()),
                    version_id: Some(version_id),
                    version_stages: Some(stages),
                })
            }
            Err(e) => sm_error_response(&e),
        }
    }

    async fn handle_delete_secret(
        &self,
        state: &Arc<tokio::sync::RwLock<SecretsManagerState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_secret_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.secret_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'SecretId'");
        }
        let force_delete = input.force_delete_without_recovery.unwrap_or(false);

        let mut state = state.write().await;
        match state.delete_secret(
            &input.secret_id,
            input.recovery_window_in_days,
            force_delete,
            account_id,
            region,
        ) {
            Ok(secret) => wire::serialize_delete_secret_response(&wire::DeleteSecretResponse {
                a_r_n: Some(secret.arn.clone()),
                name: Some(secret.name.clone()),
                deletion_date: secret.deleted_date.map(|d| d.timestamp() as f64),
            }),
            Err(e) => sm_error_response(&e),
        }
    }

    async fn handle_restore_secret(
        &self,
        state: &Arc<tokio::sync::RwLock<SecretsManagerState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_restore_secret_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.secret_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'SecretId'");
        }

        let mut state = state.write().await;
        match state.restore_secret(&input.secret_id) {
            Ok(secret) => wire::serialize_restore_secret_response(&wire::RestoreSecretResponse {
                a_r_n: Some(secret.arn.clone()),
                name: Some(secret.name.clone()),
            }),
            Err(e) => sm_error_response(&e),
        }
    }

    async fn handle_describe_secret(
        &self,
        state: &Arc<tokio::sync::RwLock<SecretsManagerState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_secret_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.secret_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'SecretId'");
        }

        let state = state.read().await;
        match state.describe_secret(&input.secret_id) {
            Ok(secret) => {
                let description = if secret.description.is_empty() {
                    None
                } else {
                    Some(secret.description.clone())
                };

                let version_ids_to_stages = if secret.versions.is_empty() {
                    None
                } else {
                    let mut map = std::collections::HashMap::new();
                    for version in secret.versions.values() {
                        map.insert(version.version_id.clone(), version.version_stages.clone());
                    }
                    Some(map)
                };

                let tags = if secret.tags.is_empty() {
                    None
                } else {
                    Some(tags_to_wire(&secret.tags))
                };

                let replication_status = replication_status_to_wire(&secret.replication_status);

                wire::serialize_describe_secret_response(&wire::DescribeSecretResponse {
                    a_r_n: Some(secret.arn.clone()),
                    name: Some(secret.name.clone()),
                    created_date: Some(secret.created_date.timestamp() as f64),
                    last_changed_date: Some(secret.last_changed_date.timestamp() as f64),
                    description,
                    deleted_date: secret.deleted_date.map(|d| d.timestamp() as f64),
                    version_ids_to_stages,
                    tags,
                    rotation_enabled: secret.rotation_enabled,
                    rotation_lambda_a_r_n: secret.rotation_lambda_arn.clone(),
                    rotation_rules: secret.rotation_rules.as_ref().map(|r| {
                        wire::RotationRulesType {
                            automatically_after_days: r.automatically_after_days,
                            duration: r.duration.clone(),
                            schedule_expression: r.schedule_expression.clone(),
                        }
                    }),
                    last_rotated_date: secret.last_rotated_date.map(|d| d.timestamp() as f64),
                    primary_region: secret.primary_region.clone(),
                    replication_status: if replication_status.is_empty() {
                        None
                    } else {
                        Some(replication_status)
                    },
                    ..Default::default()
                })
            }
            Err(e) => sm_error_response(&e),
        }
    }

    async fn handle_list_secrets(
        &self,
        state: &Arc<tokio::sync::RwLock<SecretsManagerState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let secrets = state.list_secrets();
        let entries: Vec<wire::SecretListEntry> = secrets
            .iter()
            .map(|s| {
                let description = if s.description.is_empty() {
                    None
                } else {
                    Some(s.description.clone())
                };

                let secret_versions_to_stages = if s.versions.is_empty() {
                    None
                } else {
                    let mut map = std::collections::HashMap::new();
                    for version in s.versions.values() {
                        map.insert(version.version_id.clone(), version.version_stages.clone());
                    }
                    Some(map)
                };

                let tags = if s.tags.is_empty() {
                    None
                } else {
                    Some(tags_to_wire(&s.tags))
                };

                wire::SecretListEntry {
                    a_r_n: Some(s.arn.clone()),
                    name: Some(s.name.clone()),
                    created_date: Some(s.created_date.timestamp() as f64),
                    last_changed_date: Some(s.last_changed_date.timestamp() as f64),
                    description,
                    secret_versions_to_stages,
                    tags,
                    ..Default::default()
                }
            })
            .collect();

        wire::serialize_list_secrets_response(&wire::ListSecretsResponse {
            secret_list: Some(entries),
            next_token: None,
        })
    }

    async fn handle_update_secret(
        &self,
        state: &Arc<tokio::sync::RwLock<SecretsManagerState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_secret_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.secret_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'SecretId'");
        }
        let has_new_value = input.secret_string.is_some();

        let mut state = state.write().await;
        match state.update_secret(
            &input.secret_id,
            input.description.as_deref(),
            input.secret_string.as_deref(),
            None,
        ) {
            Ok(secret) => {
                let version_id = if has_new_value {
                    secret.current_version_id.clone()
                } else {
                    None
                };
                wire::serialize_update_secret_response(&wire::UpdateSecretResponse {
                    a_r_n: Some(secret.arn.clone()),
                    name: Some(secret.name.clone()),
                    version_id,
                })
            }
            Err(e) => sm_error_response(&e),
        }
    }

    async fn handle_batch_get_secret_value(
        &self,
        state: &Arc<tokio::sync::RwLock<SecretsManagerState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_batch_get_secret_value_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let secret_ids = input.secret_id_list.unwrap_or_default();

        if secret_ids.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "You must provide SecretIdList",
            );
        }

        let state = state.read().await;
        let mut values = Vec::new();
        let mut errors = Vec::new();

        for secret_id in &secret_ids {
            match state.get_secret_value(secret_id) {
                Ok((secret, version)) => {
                    values.push(wire::SecretValueEntry {
                        a_r_n: Some(secret.arn.clone()),
                        name: Some(secret.name.clone()),
                        version_id: Some(version.version_id.clone()),
                        version_stages: Some(version.version_stages.clone()),
                        created_date: Some(version.created_date.timestamp() as f64),
                        secret_string: version.secret_string.clone(),
                        ..Default::default()
                    });
                }
                Err(e) => {
                    let (_, error_type) = match &e {
                        SecretsManagerError::ResourceExists => (400, "ResourceExistsException"),
                        SecretsManagerError::SecretMarkedDeleted => {
                            (400, "InvalidRequestException")
                        }
                        SecretsManagerError::SecretVersionNotFoundById { .. } => {
                            (400, "ResourceNotFoundException")
                        }
                        SecretsManagerError::SecretVersionNotFoundByStage { .. } => {
                            (400, "ResourceNotFoundException")
                        }
                        SecretsManagerError::InvalidRecoveryWindow => {
                            (400, "InvalidParameterException")
                        }
                        SecretsManagerError::ForceDeleteWithRecoveryWindow => {
                            (400, "InvalidParameterException")
                        }
                        SecretsManagerError::SecretNotFound => (400, "ResourceNotFoundException"),
                        SecretsManagerError::PasswordTooShort => (400, "ClientError"),
                        SecretsManagerError::PasswordTooLong => (400, "InvalidParameterValue"),
                        SecretsManagerError::NoValidCharsForPassword => {
                            (400, "InvalidParameterException")
                        }
                        SecretsManagerError::InvalidRotationDays => {
                            (400, "InvalidParameterException")
                        }
                        SecretsManagerError::MissingRemoveFromVersionId { .. } => {
                            (400, "InvalidParameterException")
                        }
                        SecretsManagerError::NotAValidVersion { .. } => {
                            (400, "InvalidParameterException")
                        }
                    };
                    errors.push(wire::APIErrorType {
                        secret_id: Some(secret_id.clone()),
                        error_code: Some(error_type.to_string()),
                        message: Some(e.to_string()),
                    });
                }
            }
        }

        wire::serialize_batch_get_secret_value_response(&wire::BatchGetSecretValueResponse {
            secret_values: if values.is_empty() {
                None
            } else {
                Some(values)
            },
            errors: if errors.is_empty() {
                None
            } else {
                Some(errors)
            },
            next_token: None,
        })
    }

    async fn handle_cancel_rotate_secret(
        &self,
        state: &Arc<tokio::sync::RwLock<SecretsManagerState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_cancel_rotate_secret_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.secret_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'SecretId'");
        }

        let mut state = state.write().await;
        match state.cancel_rotate_secret(&input.secret_id) {
            Ok(secret) => {
                wire::serialize_cancel_rotate_secret_response(&wire::CancelRotateSecretResponse {
                    a_r_n: Some(secret.arn.clone()),
                    name: Some(secret.name.clone()),
                    version_id: secret.current_version_id.clone(),
                })
            }
            Err(e) => sm_error_response(&e),
        }
    }

    async fn handle_delete_resource_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<SecretsManagerState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_resource_policy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.secret_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'SecretId'");
        }

        let mut state = state.write().await;
        match state.delete_resource_policy(&input.secret_id) {
            Ok(secret) => wire::serialize_delete_resource_policy_response(
                &wire::DeleteResourcePolicyResponse {
                    a_r_n: Some(secret.arn.clone()),
                    name: Some(secret.name.clone()),
                },
            ),
            Err(e) => sm_error_response(&e),
        }
    }

    async fn handle_get_random_password(
        &self,
        state: &Arc<tokio::sync::RwLock<SecretsManagerState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_random_password_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };

        let state = state.read().await;
        match state.get_random_password(
            input.password_length,
            input.exclude_characters.as_deref(),
            input.exclude_numbers.unwrap_or(false),
            input.exclude_punctuation.unwrap_or(false),
            input.exclude_uppercase.unwrap_or(false),
            input.exclude_lowercase.unwrap_or(false),
            input.include_space.unwrap_or(false),
            input.require_each_included_type.unwrap_or(true),
        ) {
            Ok(password) => {
                wire::serialize_get_random_password_response(&wire::GetRandomPasswordResponse {
                    random_password: Some(password),
                })
            }
            Err(e) => sm_error_response(&e),
        }
    }

    async fn handle_get_resource_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<SecretsManagerState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_resource_policy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.secret_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'SecretId'");
        }

        let state = state.read().await;
        match state.get_resource_policy(&input.secret_id) {
            Ok(secret) => {
                wire::serialize_get_resource_policy_response(&wire::GetResourcePolicyResponse {
                    a_r_n: Some(secret.arn.clone()),
                    name: Some(secret.name.clone()),
                    resource_policy: secret.resource_policy.clone(),
                })
            }
            Err(e) => sm_error_response(&e),
        }
    }

    async fn handle_list_secret_version_ids(
        &self,
        state: &Arc<tokio::sync::RwLock<SecretsManagerState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_secret_version_ids_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.secret_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'SecretId'");
        }

        let state = state.read().await;
        match state.list_secret_version_ids(&input.secret_id) {
            Ok(secret) => {
                let versions: Vec<wire::SecretVersionsListEntry> = secret
                    .versions
                    .values()
                    .map(|v| wire::SecretVersionsListEntry {
                        version_id: Some(v.version_id.clone()),
                        version_stages: Some(v.version_stages.clone()),
                        created_date: Some(v.created_date.timestamp() as f64),
                        ..Default::default()
                    })
                    .collect();

                wire::serialize_list_secret_version_ids_response(
                    &wire::ListSecretVersionIdsResponse {
                        a_r_n: Some(secret.arn.clone()),
                        name: Some(secret.name.clone()),
                        versions: if versions.is_empty() {
                            None
                        } else {
                            Some(versions)
                        },
                        next_token: None,
                    },
                )
            }
            Err(e) => sm_error_response(&e),
        }
    }

    async fn handle_put_resource_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<SecretsManagerState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_put_resource_policy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.secret_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'SecretId'");
        }
        if input.resource_policy.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ResourcePolicy'");
        }

        let mut state = state.write().await;
        match state.put_resource_policy(&input.secret_id, &input.resource_policy) {
            Ok(secret) => {
                wire::serialize_put_resource_policy_response(&wire::PutResourcePolicyResponse {
                    a_r_n: Some(secret.arn.clone()),
                    name: Some(secret.name.clone()),
                })
            }
            Err(e) => sm_error_response(&e),
        }
    }

    async fn handle_remove_regions_from_replication(
        &self,
        state: &Arc<tokio::sync::RwLock<SecretsManagerState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_remove_regions_from_replication_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.secret_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'SecretId'");
        }

        let mut state = state.write().await;
        match state.remove_regions_from_replication(&input.secret_id, &input.remove_replica_regions)
        {
            Ok(secret) => {
                let replication_status = replication_status_to_wire(&secret.replication_status);
                wire::serialize_remove_regions_from_replication_response(
                    &wire::RemoveRegionsFromReplicationResponse {
                        a_r_n: Some(secret.arn.clone()),
                        replication_status: if replication_status.is_empty() {
                            None
                        } else {
                            Some(replication_status)
                        },
                    },
                )
            }
            Err(e) => sm_error_response(&e),
        }
    }

    async fn handle_replicate_secret_to_regions(
        &self,
        state: &Arc<tokio::sync::RwLock<SecretsManagerState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_replicate_secret_to_regions_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.secret_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'SecretId'");
        }
        let regions: Vec<String> = input
            .add_replica_regions
            .into_iter()
            .filter_map(|r| r.region)
            .collect();

        let mut state = state.write().await;
        match state.replicate_secret_to_regions(&input.secret_id, &regions) {
            Ok(secret) => {
                let replication_status = replication_status_to_wire(&secret.replication_status);
                wire::serialize_replicate_secret_to_regions_response(
                    &wire::ReplicateSecretToRegionsResponse {
                        a_r_n: Some(secret.arn.clone()),
                        replication_status: if replication_status.is_empty() {
                            None
                        } else {
                            Some(replication_status)
                        },
                    },
                )
            }
            Err(e) => sm_error_response(&e),
        }
    }

    async fn handle_rotate_secret(
        &self,
        state: &Arc<tokio::sync::RwLock<SecretsManagerState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_rotate_secret_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.secret_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'SecretId'");
        }
        let rotation_rules = input.rotation_rules.map(|r| crate::types::RotationRules {
            automatically_after_days: r.automatically_after_days,
            duration: r.duration,
            schedule_expression: r.schedule_expression,
        });

        let mut state = state.write().await;
        match state.rotate_secret(
            &input.secret_id,
            input.rotation_lambda_a_r_n.as_deref(),
            rotation_rules,
        ) {
            Ok((secret, version_id)) => {
                wire::serialize_rotate_secret_response(&wire::RotateSecretResponse {
                    a_r_n: Some(secret.arn.clone()),
                    name: Some(secret.name.clone()),
                    version_id: Some(version_id),
                })
            }
            Err(e) => sm_error_response(&e),
        }
    }

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<SecretsManagerState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.secret_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'SecretId'");
        }
        let tags = tags_from_wire(input.tags);

        let mut state = state.write().await;
        match state.tag_resource(&input.secret_id, tags) {
            Ok(()) => wire::serialize_tag_resource_response(),
            Err(e) => sm_error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<SecretsManagerState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.secret_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'SecretId'");
        }

        let mut state = state.write().await;
        match state.untag_resource(&input.secret_id, &input.tag_keys) {
            Ok(()) => wire::serialize_untag_resource_response(),
            Err(e) => sm_error_response(&e),
        }
    }

    async fn handle_update_secret_version_stage(
        &self,
        state: &Arc<tokio::sync::RwLock<SecretsManagerState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_secret_version_stage_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.secret_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'SecretId'");
        }
        if input.version_stage.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'VersionStage'");
        }

        let mut state = state.write().await;
        match state.update_secret_version_stage(
            &input.secret_id,
            &input.version_stage,
            input.move_to_version_id.as_deref(),
            input.remove_from_version_id.as_deref(),
        ) {
            Ok(secret) => wire::serialize_update_secret_version_stage_response(
                &wire::UpdateSecretVersionStageResponse {
                    a_r_n: Some(secret.arn.clone()),
                    name: Some(secret.name.clone()),
                },
            ),
            Err(e) => sm_error_response(&e),
        }
    }

    async fn handle_stop_replication_to_replica(
        &self,
        state: &Arc<tokio::sync::RwLock<SecretsManagerState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_stop_replication_to_replica_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.secret_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'SecretId'");
        }

        let state = state.read().await;
        match state.describe_secret(&input.secret_id) {
            Ok(secret) => wire::serialize_stop_replication_to_replica_response(
                &wire::StopReplicationToReplicaResponse {
                    a_r_n: Some(secret.arn.clone()),
                },
            ),
            Err(e) => sm_error_response(&e),
        }
    }

    // STUB[no-engine]: Policy validation requires a real IAM policy evaluation engine;
    //   always returns validation passed.
    async fn handle_validate_resource_policy(
        &self,
        _state: &Arc<tokio::sync::RwLock<SecretsManagerState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_validate_resource_policy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_policy.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ResourcePolicy'");
        }

        wire::serialize_validate_resource_policy_response(&wire::ValidateResourcePolicyResponse {
            policy_validation_passed: Some(true),
            validation_errors: Some(Vec::new()),
        })
    }
}

fn tags_from_wire(tags: Vec<wire::Tag>) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for tag in tags {
        if let (Some(k), Some(v)) = (tag.key, tag.value) {
            map.insert(k, v);
        }
    }
    map
}

fn tags_to_wire(tags: &HashMap<String, String>) -> Vec<wire::Tag> {
    tags.iter()
        .map(|(k, v)| wire::Tag {
            key: Some(k.clone()),
            value: Some(v.clone()),
        })
        .collect()
}

fn replication_status_to_wire(
    statuses: &[crate::types::ReplicationStatus],
) -> Vec<wire::ReplicationStatusType> {
    statuses
        .iter()
        .map(|r| wire::ReplicationStatusType {
            region: Some(r.region.clone()),
            status: Some(r.status.clone()),
            status_message: r.status_message.clone(),
            kms_key_id: r.kms_key_id.clone(),
            last_accessed_date: r.last_accessed_date.map(|d| d.timestamp() as f64),
        })
        .collect()
}

fn sm_error_response(err: &SecretsManagerError) -> MockResponse {
    let (status, error_type) = match err {
        SecretsManagerError::ResourceExists => (400, "ResourceExistsException"),
        SecretsManagerError::SecretMarkedDeleted => (400, "InvalidRequestException"),
        SecretsManagerError::SecretVersionNotFoundById { .. } => (400, "ResourceNotFoundException"),
        SecretsManagerError::SecretVersionNotFoundByStage { .. } => {
            (400, "ResourceNotFoundException")
        }
        SecretsManagerError::InvalidRecoveryWindow => (400, "InvalidParameterException"),
        SecretsManagerError::ForceDeleteWithRecoveryWindow => (400, "InvalidParameterException"),
        SecretsManagerError::SecretNotFound => (400, "ResourceNotFoundException"),
        SecretsManagerError::PasswordTooShort => (400, "ClientError"),
        SecretsManagerError::PasswordTooLong => (400, "InvalidParameterValue"),
        SecretsManagerError::NoValidCharsForPassword => (400, "InvalidParameterException"),
        SecretsManagerError::InvalidRotationDays => (400, "InvalidParameterException"),
        SecretsManagerError::MissingRemoveFromVersionId { .. } => {
            (400, "InvalidParameterException")
        }
        SecretsManagerError::NotAValidVersion { .. } => (400, "InvalidParameterException"),
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
