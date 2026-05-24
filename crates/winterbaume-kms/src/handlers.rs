use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use base64::{Engine, engine::general_purpose::STANDARD as BASE64};
use rand::Rng;
use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, default_account_id,
};

use crate::state::{KmsError, KmsState};
use crate::views::KmsStateView;
use crate::wire;

/// KMS service handler that processes awsJson1.1 protocol requests.
pub struct KmsService {
    pub(crate) state: Arc<BackendState<KmsState>>,
    pub(crate) notifier: StateChangeNotifier<KmsStateView>,
}

impl KmsService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for KmsService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for KmsService {
    fn service_name(&self) -> &str {
        "kms"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://kms\..*\.amazonaws\.com",
            r"https?://kms\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl KmsService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();

        // Extract action from X-Amz-Target header
        // Format: "TrentService.CreateKey"
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
            "CreateKey" => {
                self.handle_create_key(&state, body_bytes, account_id, &region)
                    .await
            }
            "DescribeKey" => self.handle_describe_key(&state, body_bytes).await,
            "ListKeys" => self.handle_list_keys(&state).await,
            "EnableKey" => self.handle_enable_key(&state, body_bytes).await,
            "DisableKey" => self.handle_disable_key(&state, body_bytes).await,
            "ScheduleKeyDeletion" => self.handle_schedule_key_deletion(&state, body_bytes).await,
            "CancelKeyDeletion" => self.handle_cancel_key_deletion(&state, body_bytes).await,
            "Encrypt" => self.handle_encrypt(&state, body_bytes).await,
            "Decrypt" => self.handle_decrypt(&state, body_bytes).await,
            "GenerateDataKey" => self.handle_generate_data_key(&state, body_bytes).await,
            "GenerateDataKeyWithoutPlaintext" => {
                self.handle_generate_data_key_without_plaintext(&state, body_bytes)
                    .await
            }
            "CreateAlias" => {
                self.handle_create_alias(&state, body_bytes, account_id, &region)
                    .await
            }
            "ListAliases" => self.handle_list_aliases(&state, body_bytes).await,
            "DeleteAlias" => self.handle_delete_alias(&state, body_bytes).await,
            "UpdateAlias" => {
                self.handle_update_alias(&state, body_bytes, account_id, &region)
                    .await
            }
            "UpdateKeyDescription" => self.handle_update_key_description(&state, body_bytes).await,
            "TagResource" => self.handle_tag_resource(&state, body_bytes).await,
            "UntagResource" => self.handle_untag_resource(&state, body_bytes).await,
            "ListResourceTags" => self.handle_list_resource_tags(&state, body_bytes).await,
            "EnableKeyRotation" => self.handle_enable_key_rotation(&state, body_bytes).await,
            "DisableKeyRotation" => self.handle_disable_key_rotation(&state, body_bytes).await,
            "GetKeyRotationStatus" => {
                self.handle_get_key_rotation_status(&state, body_bytes)
                    .await
            }
            "GetKeyPolicy" => self.handle_get_key_policy(&state, body_bytes).await,
            "PutKeyPolicy" => self.handle_put_key_policy(&state, body_bytes).await,
            "ListKeyPolicies" => self.handle_list_key_policies(&state, body_bytes).await,
            "GetPublicKey" => self.handle_get_public_key(&state, body_bytes).await,
            "GenerateRandom" => self.handle_generate_random(&state, body_bytes).await,
            "GenerateMac" => self.handle_generate_mac(&state, body_bytes).await,
            "VerifyMac" => self.handle_verify_mac(&state, body_bytes).await,
            "Sign" => self.handle_sign(&state, body_bytes).await,
            "Verify" => self.handle_verify(&state, body_bytes).await,
            "ReEncrypt" => self.handle_re_encrypt(&state, body_bytes).await,
            "CreateGrant" => {
                self.handle_create_grant(&state, body_bytes, account_id)
                    .await
            }
            "ListGrants" => self.handle_list_grants(&state, body_bytes).await,
            "ListRetirableGrants" => self.handle_list_retirable_grants(&state, body_bytes).await,
            "RetireGrant" => self.handle_retire_grant(&state, body_bytes).await,
            "RevokeGrant" => self.handle_revoke_grant(&state, body_bytes).await,
            "RotateKeyOnDemand" => self.handle_rotate_key_on_demand(&state, body_bytes).await,
            "ListKeyRotations" => self.handle_list_key_rotations(&state, body_bytes).await,
            "ReplicateKey" => {
                self.handle_replicate_key(&state, body_bytes, account_id, &region)
                    .await
            }
            "ConnectCustomKeyStore" => {
                self.handle_connect_custom_key_store(&state, body_bytes)
                    .await
            }
            "CreateCustomKeyStore" => {
                self.handle_create_custom_key_store(&state, body_bytes)
                    .await
            }
            "DeleteCustomKeyStore" => {
                self.handle_delete_custom_key_store(&state, body_bytes)
                    .await
            }
            "DescribeCustomKeyStores" => {
                self.handle_describe_custom_key_stores(&state, body_bytes)
                    .await
            }
            "DisconnectCustomKeyStore" => {
                self.handle_disconnect_custom_key_store(&state, body_bytes)
                    .await
            }
            "UpdateCustomKeyStore" => {
                self.handle_update_custom_key_store(&state, body_bytes)
                    .await
            }
            "GetParametersForImport" => {
                self.handle_get_parameters_for_import(&state, body_bytes)
                    .await
            }
            "ImportKeyMaterial" => self.handle_import_key_material(&state, body_bytes).await,
            "DeleteImportedKeyMaterial" => {
                self.handle_delete_imported_key_material(&state, body_bytes)
                    .await
            }
            "DeriveSharedSecret" => self.handle_derive_shared_secret(&state, body_bytes).await,
            "GenerateDataKeyPair" => self.handle_generate_data_key_pair(&state, body_bytes).await,
            "GenerateDataKeyPairWithoutPlaintext" => {
                self.handle_generate_data_key_pair_without_plaintext(&state, body_bytes)
                    .await
            }
            "UpdatePrimaryRegion" => self.handle_update_primary_region(&state, body_bytes).await,
            _ => json_error_response(
                400,
                "InvalidAction",
                &format!("Could not find operation {action} for KMS"),
            ),
        };
        use winterbaume_core::StatefulService;
        if response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_create_key(
        &self,
        state: &Arc<tokio::sync::RwLock<KmsState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_key_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let description = input.description.as_deref().unwrap_or("");
        let key_spec = input
            .key_spec
            .as_deref()
            .or(input.customer_master_key_spec.as_deref());
        let key_usage = input.key_usage.as_deref();
        let origin = input.origin.as_deref();
        let multi_region = input.multi_region.unwrap_or(false);
        let tags: HashMap<String, String> = input
            .tags
            .unwrap_or_default()
            .into_iter()
            .map(|t| (t.tag_key, t.tag_value))
            .collect();

        let mut state = state.write().await;
        match state.create_key(
            account_id,
            region,
            description,
            key_spec,
            key_usage,
            tags,
            origin,
            multi_region,
        ) {
            Ok(key) => wire::serialize_create_key_response(&wire::CreateKeyResponse {
                key_metadata: Some(key_to_metadata(key)),
            }),
            Err(e) => kms_error_response(&e),
        }
    }

    async fn handle_describe_key(
        &self,
        state: &Arc<tokio::sync::RwLock<KmsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_key_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.key_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'KeyId'");
        }

        let state = state.read().await;
        match state.describe_key(&input.key_id) {
            Ok(key) => wire::serialize_describe_key_response(&wire::DescribeKeyResponse {
                key_metadata: Some(key_to_metadata(key)),
            }),
            Err(e) => kms_error_response(&e),
        }
    }

    async fn handle_list_keys(&self, state: &Arc<tokio::sync::RwLock<KmsState>>) -> MockResponse {
        let state = state.read().await;
        let keys = state.list_keys();
        let entries: Vec<wire::KeyListEntry> = keys
            .iter()
            .map(|k| wire::KeyListEntry {
                key_id: Some(k.key_id.clone()),
                key_arn: Some(k.arn.clone()),
            })
            .collect();

        wire::serialize_list_keys_response(&wire::ListKeysResponse {
            keys: Some(entries),
            truncated: Some(false),
            next_marker: None,
        })
    }

    async fn handle_enable_key(
        &self,
        state: &Arc<tokio::sync::RwLock<KmsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_enable_key_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.key_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'KeyId'");
        }

        let mut state = state.write().await;
        match state.enable_key(&input.key_id) {
            Ok(()) => wire::serialize_enable_key_response(),
            Err(e) => kms_error_response(&e),
        }
    }

    async fn handle_disable_key(
        &self,
        state: &Arc<tokio::sync::RwLock<KmsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_disable_key_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.key_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'KeyId'");
        }

        let mut state = state.write().await;
        match state.disable_key(&input.key_id) {
            Ok(()) => wire::serialize_disable_key_response(),
            Err(e) => kms_error_response(&e),
        }
    }

    async fn handle_schedule_key_deletion(
        &self,
        state: &Arc<tokio::sync::RwLock<KmsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_schedule_key_deletion_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.key_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'KeyId'");
        }
        let pending_days = input.pending_window_in_days.map(i64::from);

        let mut state = state.write().await;
        match state.schedule_key_deletion(&input.key_id, pending_days) {
            Ok((key, deletion_timestamp)) => {
                wire::serialize_schedule_key_deletion_response(&wire::ScheduleKeyDeletionResponse {
                    key_id: Some(key.key_id.clone()),
                    key_state: Some("PendingDeletion".to_string()),
                    deletion_date: Some(deletion_timestamp as f64),
                    pending_window_in_days: None,
                })
            }
            Err(e) => kms_error_response(&e),
        }
    }

    async fn handle_cancel_key_deletion(
        &self,
        state: &Arc<tokio::sync::RwLock<KmsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_cancel_key_deletion_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.key_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'KeyId'");
        }

        let mut state = state.write().await;
        match state.cancel_key_deletion(&input.key_id) {
            Ok(key) => {
                wire::serialize_cancel_key_deletion_response(&wire::CancelKeyDeletionResponse {
                    key_id: Some(key.key_id.clone()),
                })
            }
            Err(e) => kms_error_response(&e),
        }
    }

    async fn handle_encrypt(
        &self,
        state: &Arc<tokio::sync::RwLock<KmsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_encrypt_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.key_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'KeyId'");
        }
        if input.plaintext.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Plaintext'");
        }
        let plaintext = match BASE64.decode(&input.plaintext) {
            Ok(p) => p,
            Err(_) => {
                return json_error_response(
                    400,
                    "ValidationException",
                    "Invalid base64 in 'Plaintext'",
                );
            }
        };
        let encryption_context = input.encryption_context.unwrap_or_default();

        let state = state.read().await;
        match state.encrypt(&input.key_id, &plaintext, &encryption_context) {
            Ok(result) => wire::serialize_encrypt_response(&wire::EncryptResponse {
                ciphertext_blob: Some(BASE64.encode(&result.ciphertext_blob)),
                key_id: Some(result.key_arn.clone()),
                encryption_algorithm: Some("SYMMETRIC_DEFAULT".to_string()),
            }),
            Err(e) => kms_error_response(&e),
        }
    }

    async fn handle_decrypt(
        &self,
        state: &Arc<tokio::sync::RwLock<KmsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_decrypt_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let ciphertext_b64 = match input.ciphertext_blob.as_deref() {
            Some(c) if !c.is_empty() => c,
            _ => {
                return json_error_response(400, "ValidationException", "Missing 'CiphertextBlob'");
            }
        };
        let ciphertext_blob = match BASE64.decode(ciphertext_b64) {
            Ok(c) => c,
            Err(_) => {
                return json_error_response(
                    400,
                    "ValidationException",
                    "Invalid base64 in 'CiphertextBlob'",
                );
            }
        };
        let encryption_context = input.encryption_context.unwrap_or_default();

        let state = state.read().await;
        match state.decrypt(
            &ciphertext_blob,
            &encryption_context,
            input.key_id.as_deref(),
        ) {
            Ok(result) => wire::serialize_decrypt_response(&wire::DecryptResponse {
                plaintext: Some(BASE64.encode(&result.plaintext)),
                key_id: Some(result.key_arn.clone()),
                encryption_algorithm: Some("SYMMETRIC_DEFAULT".to_string()),
                ciphertext_for_recipient: None,
                key_material_id: None,
            }),
            Err(e) => kms_error_response(&e),
        }
    }

    async fn handle_generate_data_key(
        &self,
        state: &Arc<tokio::sync::RwLock<KmsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_generate_data_key_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.key_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'KeyId'");
        }
        let key_spec = input.key_spec.as_deref();
        let number_of_bytes = input.number_of_bytes.map(|v| v as usize);
        let encryption_context = input.encryption_context.unwrap_or_default();

        let state = state.read().await;
        match state.generate_data_key(
            &input.key_id,
            key_spec,
            number_of_bytes,
            &encryption_context,
        ) {
            Ok(result) => {
                wire::serialize_generate_data_key_response(&wire::GenerateDataKeyResponse {
                    plaintext: Some(BASE64.encode(&result.plaintext)),
                    ciphertext_blob: Some(BASE64.encode(&result.ciphertext_blob)),
                    key_id: Some(result.key_arn.clone()),
                    ciphertext_for_recipient: None,
                    key_material_id: None,
                })
            }
            Err(e) => kms_error_response(&e),
        }
    }

    async fn handle_generate_data_key_without_plaintext(
        &self,
        state: &Arc<tokio::sync::RwLock<KmsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_generate_data_key_without_plaintext_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.key_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'KeyId'");
        }
        let key_spec = input.key_spec.as_deref();
        let number_of_bytes = input.number_of_bytes.map(|v| v as usize);
        let encryption_context = input.encryption_context.unwrap_or_default();

        let state = state.read().await;
        match state.generate_data_key(
            &input.key_id,
            key_spec,
            number_of_bytes,
            &encryption_context,
        ) {
            Ok(result) => wire::serialize_generate_data_key_without_plaintext_response(
                &wire::GenerateDataKeyWithoutPlaintextResponse {
                    ciphertext_blob: Some(BASE64.encode(&result.ciphertext_blob)),
                    key_id: Some(result.key_arn.clone()),
                    key_material_id: None,
                },
            ),
            Err(e) => kms_error_response(&e),
        }
    }

    async fn handle_create_alias(
        &self,
        state: &Arc<tokio::sync::RwLock<KmsState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_alias_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.alias_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'AliasName'");
        }
        if input.target_key_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'TargetKeyId'");
        }

        let mut state = state.write().await;
        match state.create_alias(&input.alias_name, &input.target_key_id, account_id, region) {
            Ok(()) => wire::serialize_create_alias_response(),
            Err(e) => kms_error_response(&e),
        }
    }

    async fn handle_list_aliases(
        &self,
        state: &Arc<tokio::sync::RwLock<KmsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_aliases_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let key_id = input.key_id.as_deref();

        let state = state.read().await;
        let aliases = state.list_aliases(key_id);
        let entries: Vec<wire::AliasListEntry> = aliases
            .iter()
            .map(|a| wire::AliasListEntry {
                alias_name: Some(a.alias_name.clone()),
                alias_arn: Some(a.alias_arn.clone()),
                target_key_id: Some(a.target_key_id.clone()),
                creation_date: None,
                last_updated_date: None,
            })
            .collect();

        wire::serialize_list_aliases_response(&wire::ListAliasesResponse {
            aliases: Some(entries),
            truncated: Some(false),
            next_marker: None,
        })
    }

    async fn handle_delete_alias(
        &self,
        state: &Arc<tokio::sync::RwLock<KmsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_alias_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.alias_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'AliasName'");
        }

        let mut state = state.write().await;
        match state.delete_alias(&input.alias_name) {
            Ok(()) => wire::serialize_delete_alias_response(),
            Err(e) => kms_error_response(&e),
        }
    }
    async fn handle_update_alias(
        &self,
        state: &Arc<tokio::sync::RwLock<KmsState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_update_alias_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.alias_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'AliasName'");
        }
        if input.target_key_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'TargetKeyId'");
        }

        let mut state = state.write().await;
        match state.update_alias(&input.alias_name, &input.target_key_id, account_id, region) {
            Ok(()) => wire::serialize_update_alias_response(),
            Err(e) => kms_error_response(&e),
        }
    }

    async fn handle_update_key_description(
        &self,
        state: &Arc<tokio::sync::RwLock<KmsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_key_description_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.key_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'KeyId'");
        }

        let mut state = state.write().await;
        match state.update_key_description(&input.key_id, &input.description) {
            Ok(()) => wire::serialize_update_key_description_response(),
            Err(e) => kms_error_response(&e),
        }
    }

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<KmsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.key_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'KeyId'");
        }
        let tags: HashMap<String, String> = input
            .tags
            .into_iter()
            .map(|t| (t.tag_key, t.tag_value))
            .collect();

        let mut state = state.write().await;
        match state.tag_resource(&input.key_id, tags) {
            Ok(()) => wire::serialize_tag_resource_response(),
            Err(e) => kms_error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<KmsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.key_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'KeyId'");
        }

        let mut state = state.write().await;
        match state.untag_resource(&input.key_id, &input.tag_keys) {
            Ok(()) => wire::serialize_untag_resource_response(),
            Err(e) => kms_error_response(&e),
        }
    }

    async fn handle_list_resource_tags(
        &self,
        state: &Arc<tokio::sync::RwLock<KmsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_resource_tags_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.key_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'KeyId'");
        }

        let state = state.read().await;
        match state.list_resource_tags(&input.key_id) {
            Ok(tags) => {
                let tag_list: Vec<wire::Tag> = tags
                    .iter()
                    .map(|(k, v)| wire::Tag {
                        tag_key: k.clone(),
                        tag_value: v.clone(),
                    })
                    .collect();

                wire::serialize_list_resource_tags_response(&wire::ListResourceTagsResponse {
                    tags: Some(tag_list),
                    truncated: Some(false),
                    next_marker: None,
                })
            }
            Err(e) => kms_error_response(&e),
        }
    }

    async fn handle_enable_key_rotation(
        &self,
        state: &Arc<tokio::sync::RwLock<KmsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_enable_key_rotation_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.key_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'KeyId'");
        }

        let mut state = state.write().await;
        match state.enable_key_rotation(&input.key_id) {
            Ok(()) => wire::serialize_enable_key_rotation_response(),
            Err(e) => kms_error_response(&e),
        }
    }

    async fn handle_disable_key_rotation(
        &self,
        state: &Arc<tokio::sync::RwLock<KmsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_disable_key_rotation_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.key_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'KeyId'");
        }

        let mut state = state.write().await;
        match state.disable_key_rotation(&input.key_id) {
            Ok(()) => wire::serialize_disable_key_rotation_response(),
            Err(e) => kms_error_response(&e),
        }
    }

    async fn handle_get_key_rotation_status(
        &self,
        state: &Arc<tokio::sync::RwLock<KmsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_key_rotation_status_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.key_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'KeyId'");
        }

        let state = state.read().await;
        match state.get_key_rotation_status(&input.key_id) {
            Ok(key) => wire::serialize_get_key_rotation_status_response(
                &wire::GetKeyRotationStatusResponse {
                    key_id: Some(key.key_id.clone()),
                    key_rotation_enabled: Some(key.key_rotation_enabled),
                    ..Default::default()
                },
            ),
            Err(e) => kms_error_response(&e),
        }
    }

    async fn handle_get_key_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<KmsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_key_policy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.key_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'KeyId'");
        }

        let state = state.read().await;
        match state.get_key_policy(&input.key_id) {
            Ok((_key_id, policy)) => {
                wire::serialize_get_key_policy_response(&wire::GetKeyPolicyResponse {
                    policy: Some(policy),
                    policy_name: Some("default".to_string()),
                })
            }
            Err(e) => kms_error_response(&e),
        }
    }

    async fn handle_put_key_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<KmsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_put_key_policy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.key_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'KeyId'");
        }
        if input.policy.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Policy'");
        }

        let mut state = state.write().await;
        match state.put_key_policy(&input.key_id, &input.policy) {
            Ok(()) => wire::serialize_put_key_policy_response(),
            Err(e) => kms_error_response(&e),
        }
    }

    async fn handle_list_key_policies(
        &self,
        state: &Arc<tokio::sync::RwLock<KmsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_key_policies_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.key_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'KeyId'");
        }

        let state = state.read().await;
        match state.list_key_policies(&input.key_id) {
            Ok(()) => wire::serialize_list_key_policies_response(&wire::ListKeyPoliciesResponse {
                policy_names: Some(vec!["default".to_string()]),
                truncated: Some(false),
                next_marker: None,
            }),
            Err(e) => kms_error_response(&e),
        }
    }

    async fn handle_get_public_key(
        &self,
        state: &Arc<tokio::sync::RwLock<KmsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_public_key_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.key_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'KeyId'");
        }

        let state = state.read().await;
        match state.get_public_key(&input.key_id) {
            Ok(key) => {
                let public_key_b64 = key.public_key_der.as_deref().map(|der| BASE64.encode(der));
                let mut resp = wire::GetPublicKeyResponse {
                    key_id: Some(key.arn.clone()),
                    key_spec: Some(key.key_spec.clone()),
                    key_usage: Some(key.key_usage.clone()),
                    public_key: public_key_b64,
                    ..Default::default()
                };
                if let Some(enc_algos) = key.encryption_algorithms() {
                    resp.encryption_algorithms =
                        Some(enc_algos.iter().map(|s| s.to_string()).collect());
                }
                if let Some(sign_algos) = key.signing_algorithms() {
                    resp.signing_algorithms =
                        Some(sign_algos.iter().map(|s| s.to_string()).collect());
                }
                wire::serialize_get_public_key_response(&resp)
            }
            Err(e) => kms_error_response(&e),
        }
    }

    async fn handle_generate_random(
        &self,
        state: &Arc<tokio::sync::RwLock<KmsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_generate_random_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let number_of_bytes = input.number_of_bytes.map(|v| v as usize).unwrap_or(32);

        let state = state.read().await;
        match state.generate_random(number_of_bytes) {
            Ok(bytes) => wire::serialize_generate_random_response(&wire::GenerateRandomResponse {
                plaintext: Some(BASE64.encode(&bytes)),
                ciphertext_for_recipient: None,
            }),
            Err(e) => kms_error_response(&e),
        }
    }

    async fn handle_generate_mac(
        &self,
        state: &Arc<tokio::sync::RwLock<KmsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_generate_mac_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.key_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'KeyId'");
        }
        if input.message.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Message'");
        }
        if input.mac_algorithm.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'MacAlgorithm'");
        }
        let message = match BASE64.decode(&input.message) {
            Ok(m) => m,
            Err(_) => {
                return json_error_response(
                    400,
                    "ValidationException",
                    "Invalid base64 in 'Message'",
                );
            }
        };

        let state = state.read().await;
        match state.generate_mac(&input.key_id, &message, &input.mac_algorithm) {
            Ok((key_arn, mac_bytes, algo)) => {
                wire::serialize_generate_mac_response(&wire::GenerateMacResponse {
                    key_id: Some(key_arn),
                    mac: Some(BASE64.encode(&mac_bytes)),
                    mac_algorithm: Some(algo),
                })
            }
            Err(e) => kms_error_response(&e),
        }
    }

    async fn handle_verify_mac(
        &self,
        state: &Arc<tokio::sync::RwLock<KmsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_verify_mac_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.key_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'KeyId'");
        }
        if input.message.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Message'");
        }
        if input.mac_algorithm.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'MacAlgorithm'");
        }
        if input.mac.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Mac'");
        }
        let message = match BASE64.decode(&input.message) {
            Ok(m) => m,
            Err(_) => {
                return json_error_response(
                    400,
                    "ValidationException",
                    "Invalid base64 in 'Message'",
                );
            }
        };
        let mac_to_verify = match BASE64.decode(&input.mac) {
            Ok(m) => m,
            Err(_) => {
                return json_error_response(400, "ValidationException", "Invalid base64 in 'Mac'");
            }
        };

        let state = state.read().await;
        match state.verify_mac(
            &input.key_id,
            &message,
            &input.mac_algorithm,
            &mac_to_verify,
        ) {
            Ok((key_arn, valid, algo)) => {
                wire::serialize_verify_mac_response(&wire::VerifyMacResponse {
                    key_id: Some(key_arn),
                    mac_valid: Some(valid),
                    mac_algorithm: Some(algo),
                })
            }
            Err(e) => kms_error_response(&e),
        }
    }

    async fn handle_sign(
        &self,
        state: &Arc<tokio::sync::RwLock<KmsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_sign_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.key_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'KeyId'");
        }
        if input.message.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Message'");
        }
        if input.signing_algorithm.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'SigningAlgorithm'");
        }
        let message = match BASE64.decode(&input.message) {
            Ok(m) => m,
            Err(_) => {
                return json_error_response(
                    400,
                    "ValidationException",
                    "Invalid base64 in 'Message'",
                );
            }
        };

        let state = state.read().await;
        match state.sign(&input.key_id, &message, &input.signing_algorithm) {
            Ok((key_arn, signature, algo)) => wire::serialize_sign_response(&wire::SignResponse {
                key_id: Some(key_arn),
                signature: Some(BASE64.encode(&signature)),
                signing_algorithm: Some(algo),
            }),
            Err(e) => kms_error_response(&e),
        }
    }

    async fn handle_verify(
        &self,
        state: &Arc<tokio::sync::RwLock<KmsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_verify_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.key_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'KeyId'");
        }
        if input.message.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Message'");
        }
        if input.signing_algorithm.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'SigningAlgorithm'");
        }
        if input.signature.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'Signature'");
        }
        let message = match BASE64.decode(&input.message) {
            Ok(m) => m,
            Err(_) => {
                return json_error_response(
                    400,
                    "ValidationException",
                    "Invalid base64 in 'Message'",
                );
            }
        };
        let signature = match BASE64.decode(&input.signature) {
            Ok(s) => s,
            Err(_) => {
                return json_error_response(
                    400,
                    "ValidationException",
                    "Invalid base64 in 'Signature'",
                );
            }
        };

        let state = state.read().await;
        match state.verify(
            &input.key_id,
            &message,
            &input.signing_algorithm,
            &signature,
        ) {
            Ok((key_arn, valid, algo)) => wire::serialize_verify_response(&wire::VerifyResponse {
                key_id: Some(key_arn),
                signature_valid: Some(valid),
                signing_algorithm: Some(algo),
            }),
            Err(e) => kms_error_response(&e),
        }
    }

    async fn handle_re_encrypt(
        &self,
        state: &Arc<tokio::sync::RwLock<KmsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_re_encrypt_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let ciphertext_b64 = match input.ciphertext_blob.as_deref() {
            Some(c) if !c.is_empty() => c,
            _ => {
                return json_error_response(400, "ValidationException", "Missing 'CiphertextBlob'");
            }
        };
        let ciphertext_blob = match BASE64.decode(ciphertext_b64) {
            Ok(c) => c,
            Err(_) => {
                return json_error_response(
                    400,
                    "ValidationException",
                    "Invalid base64 in 'CiphertextBlob'",
                );
            }
        };
        if input.destination_key_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'DestinationKeyId'");
        }
        let source_encryption_context = input.source_encryption_context.unwrap_or_default();
        let destination_encryption_context =
            input.destination_encryption_context.unwrap_or_default();

        let state = state.read().await;
        match state.re_encrypt(
            &ciphertext_blob,
            &source_encryption_context,
            input.source_key_id.as_deref(),
            &input.destination_key_id,
            &destination_encryption_context,
        ) {
            Ok(result) => wire::serialize_re_encrypt_response(&wire::ReEncryptResponse {
                ciphertext_blob: Some(BASE64.encode(&result.ciphertext_blob)),
                key_id: Some(result.key_id),
                source_key_id: Some(result.source_key_id),
                source_encryption_algorithm: Some("SYMMETRIC_DEFAULT".to_string()),
                destination_encryption_algorithm: Some("SYMMETRIC_DEFAULT".to_string()),
                ..Default::default()
            }),
            Err(e) => kms_error_response(&e),
        }
    }

    async fn handle_create_grant(
        &self,
        state: &Arc<tokio::sync::RwLock<KmsState>>,
        body: &[u8],
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_grant_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.key_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'KeyId'");
        }
        if input.grantee_principal.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'GranteePrincipal'");
        }
        let retiring_principal = input.retiring_principal.as_deref();
        let operations = input.operations;
        let name = input.name.as_deref();

        let constraints = input.constraints.and_then(|c| {
            let ece = c.encryption_context_equals;
            let ecs = c.encryption_context_subset;
            if ece.is_some() || ecs.is_some() {
                Some(crate::types::GrantConstraints {
                    encryption_context_equals: ece,
                    encryption_context_subset: ecs,
                })
            } else {
                None
            }
        });

        let mut state = state.write().await;
        match state.create_grant(
            &input.key_id,
            &input.grantee_principal,
            retiring_principal,
            operations,
            constraints,
            name,
            account_id,
        ) {
            Ok((grant_id, grant_token)) => {
                wire::serialize_create_grant_response(&wire::CreateGrantResponse {
                    grant_id: Some(grant_id),
                    grant_token: Some(grant_token),
                })
            }
            Err(e) => kms_error_response(&e),
        }
    }

    async fn handle_list_grants(
        &self,
        state: &Arc<tokio::sync::RwLock<KmsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_grants_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        // KeyId is required by the Smithy model but the legacy handler treated it
        // as optional; preserve the prior behaviour by passing an empty string as
        // None-equivalent for the list call.
        let key_id = if input.key_id.is_empty() {
            None
        } else {
            Some(input.key_id.as_str())
        };
        let grant_id = input.grant_id.as_deref();

        let state = state.read().await;
        match state.list_grants(key_id, grant_id) {
            Ok(grants) => {
                let entries: Vec<wire::GrantListEntry> = grants
                    .iter()
                    .map(|g| wire::GrantListEntry {
                        grant_id: Some(g.grant_id.clone()),
                        key_id: Some(g.key_id.clone()),
                        grantee_principal: Some(g.grantee_principal.clone()),
                        retiring_principal: g.retiring_principal.clone(),
                        issuing_account: Some(g.issuing_account.clone()),
                        operations: Some(g.operations.clone()),
                        name: g.name.clone(),
                        creation_date: Some(g.creation_date.timestamp() as f64),
                        constraints: g.constraints.as_ref().map(|c| wire::GrantConstraints {
                            encryption_context_equals: c.encryption_context_equals.clone(),
                            encryption_context_subset: c.encryption_context_subset.clone(),
                        }),
                    })
                    .collect();

                wire::serialize_list_grants_response(&wire::ListGrantsResponse {
                    grants: Some(entries),
                    truncated: Some(false),
                    next_marker: None,
                })
            }
            Err(e) => kms_error_response(&e),
        }
    }

    async fn handle_list_retirable_grants(
        &self,
        state: &Arc<tokio::sync::RwLock<KmsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_retirable_grants_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.retiring_principal.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'RetiringPrincipal'");
        }

        let state = state.read().await;
        let grants = state.list_retirable_grants(&input.retiring_principal);
        let entries: Vec<wire::GrantListEntry> = grants
            .iter()
            .map(|g| wire::GrantListEntry {
                grant_id: Some(g.grant_id.clone()),
                key_id: Some(g.key_id.clone()),
                grantee_principal: Some(g.grantee_principal.clone()),
                retiring_principal: g.retiring_principal.clone(),
                issuing_account: Some(g.issuing_account.clone()),
                operations: Some(g.operations.clone()),
                name: g.name.clone(),
                creation_date: Some(g.creation_date.timestamp() as f64),
                constraints: g.constraints.as_ref().map(|c| wire::GrantConstraints {
                    encryption_context_equals: c.encryption_context_equals.clone(),
                    encryption_context_subset: c.encryption_context_subset.clone(),
                }),
            })
            .collect();

        wire::serialize_list_retirable_grants_response(&wire::ListGrantsResponse {
            grants: Some(entries),
            truncated: Some(false),
            next_marker: None,
        })
    }

    async fn handle_retire_grant(
        &self,
        state: &Arc<tokio::sync::RwLock<KmsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_retire_grant_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let grant_id = input.grant_id.as_deref();
        let grant_token = input.grant_token.as_deref();
        let key_id = input.key_id.as_deref();

        let mut state = state.write().await;
        match state.retire_grant(grant_id, grant_token, key_id) {
            Ok(()) => wire::serialize_retire_grant_response(),
            Err(e) => kms_error_response(&e),
        }
    }

    async fn handle_revoke_grant(
        &self,
        state: &Arc<tokio::sync::RwLock<KmsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_revoke_grant_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.key_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'KeyId'");
        }
        if input.grant_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'GrantId'");
        }

        let mut state = state.write().await;
        match state.revoke_grant(&input.key_id, &input.grant_id) {
            Ok(()) => wire::serialize_revoke_grant_response(),
            Err(e) => kms_error_response(&e),
        }
    }

    async fn handle_rotate_key_on_demand(
        &self,
        state: &Arc<tokio::sync::RwLock<KmsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_rotate_key_on_demand_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.key_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'KeyId'");
        }

        let mut state = state.write().await;
        match state.rotate_key_on_demand(&input.key_id) {
            Ok(kid) => {
                wire::serialize_rotate_key_on_demand_response(&wire::RotateKeyOnDemandResponse {
                    key_id: Some(kid),
                })
            }
            Err(e) => kms_error_response(&e),
        }
    }

    async fn handle_list_key_rotations(
        &self,
        state: &Arc<tokio::sync::RwLock<KmsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_key_rotations_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.key_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'KeyId'");
        }
        let limit = input.limit.map(|v| v as usize);
        let marker = input.marker.as_deref();

        let state = state.read().await;
        match state.list_key_rotations(&input.key_id) {
            Ok(rotations) => {
                let all_entries: Vec<wire::RotationsListEntry> = rotations
                    .iter()
                    .map(|r| wire::RotationsListEntry {
                        key_id: Some(r.key_id.clone()),
                        rotation_date: Some(r.rotation_date.timestamp() as f64),
                        rotation_type: Some(r.rotation_type.clone()),
                        ..Default::default()
                    })
                    .collect();

                // Apply pagination
                let start = if let Some(m) = marker {
                    m.parse::<usize>().unwrap_or(0)
                } else {
                    0
                };

                let (entries, truncated, next_marker) = if let Some(lim) = limit {
                    let end = std::cmp::min(start + lim, all_entries.len());
                    let page = all_entries[start..end].to_vec();
                    let is_truncated = end < all_entries.len();
                    let nm = if is_truncated {
                        Some(end.to_string())
                    } else {
                        None
                    };
                    (page, is_truncated, nm)
                } else {
                    let page = all_entries[start..].to_vec();
                    (page, false, None)
                };

                wire::serialize_list_key_rotations_response(&wire::ListKeyRotationsResponse {
                    rotations: Some(entries),
                    truncated: Some(truncated),
                    next_marker,
                })
            }
            Err(e) => kms_error_response(&e),
        }
    }

    async fn handle_replicate_key(
        &self,
        state: &Arc<tokio::sync::RwLock<KmsState>>,
        body: &[u8],
        account_id: &str,
        _region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_replicate_key_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.key_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'KeyId'");
        }
        if input.replica_region.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ReplicaRegion'");
        }
        let description = input.description.as_deref();
        let tags: HashMap<String, String> = input
            .tags
            .unwrap_or_default()
            .into_iter()
            .map(|t| (t.tag_key, t.tag_value))
            .collect();

        let mut state = state.write().await;
        match state.replicate_key(
            &input.key_id,
            &input.replica_region,
            account_id,
            description,
            tags,
        ) {
            Ok(key) => {
                let tag_list: Vec<wire::Tag> = key
                    .tags
                    .iter()
                    .map(|(k, v)| wire::Tag {
                        tag_key: k.clone(),
                        tag_value: v.clone(),
                    })
                    .collect();

                wire::serialize_replicate_key_response(&wire::ReplicateKeyResponse {
                    replica_key_metadata: Some(key_to_metadata(key)),
                    replica_tags: if tag_list.is_empty() {
                        None
                    } else {
                        Some(tag_list)
                    },
                    replica_policy: None,
                })
            }
            Err(e) => kms_error_response(&e),
        }
    }
    // ---- Custom Key Store handlers ----

    async fn handle_create_custom_key_store(
        &self,
        state: &Arc<tokio::sync::RwLock<KmsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_create_custom_key_store_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.custom_key_store_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'CustomKeyStoreName'");
        }
        let cloud_hsm_cluster_id = input.cloud_hsm_cluster_id.as_deref();
        let trust_anchor_certificate = input.trust_anchor_certificate.as_deref();
        let custom_key_store_type = input.custom_key_store_type.as_deref();

        let mut state = state.write().await;
        match state.create_custom_key_store(
            &input.custom_key_store_name,
            cloud_hsm_cluster_id,
            trust_anchor_certificate,
            custom_key_store_type,
        ) {
            Ok(id) => wire::serialize_create_custom_key_store_response(
                &wire::CreateCustomKeyStoreResponse {
                    custom_key_store_id: Some(id),
                },
            ),
            Err(e) => kms_error_response(&e),
        }
    }

    async fn handle_describe_custom_key_stores(
        &self,
        state: &Arc<tokio::sync::RwLock<KmsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_custom_key_stores_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let custom_key_store_id = input.custom_key_store_id.as_deref();
        let custom_key_store_name = input.custom_key_store_name.as_deref();

        let state = state.read().await;
        let stores = state.describe_custom_key_stores(custom_key_store_id, custom_key_store_name);
        let entries: Vec<wire::CustomKeyStoresListEntry> = stores
            .iter()
            .map(|cks| wire::CustomKeyStoresListEntry {
                custom_key_store_id: Some(cks.custom_key_store_id.clone()),
                custom_key_store_name: Some(cks.custom_key_store_name.clone()),
                cloud_hsm_cluster_id: cks.cloud_hsm_cluster_id.clone(),
                trust_anchor_certificate: cks.trust_anchor_certificate.clone(),
                connection_state: Some(cks.connection_state.clone()),
                creation_date: Some(cks.creation_date.timestamp() as f64),
                custom_key_store_type: Some(cks.custom_key_store_type.clone()),
                ..Default::default()
            })
            .collect();

        wire::serialize_describe_custom_key_stores_response(
            &wire::DescribeCustomKeyStoresResponse {
                custom_key_stores: Some(entries),
                truncated: Some(false),
                next_marker: None,
            },
        )
    }

    async fn handle_update_custom_key_store(
        &self,
        state: &Arc<tokio::sync::RwLock<KmsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_custom_key_store_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.custom_key_store_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'CustomKeyStoreId'");
        }
        let new_name = input.new_custom_key_store_name.as_deref();
        // UpdateCustomKeyStoreRequest does not carry a TrustAnchorCertificate field;
        // preserve historical handler behaviour by passing None.
        let trust_anchor_certificate: Option<&str> = None;

        let mut state = state.write().await;
        match state.update_custom_key_store(
            &input.custom_key_store_id,
            new_name,
            trust_anchor_certificate,
        ) {
            Ok(()) => wire::serialize_update_custom_key_store_response(
                &wire::UpdateCustomKeyStoreResponse {},
            ),
            Err(e) => kms_error_response(&e),
        }
    }

    async fn handle_delete_custom_key_store(
        &self,
        state: &Arc<tokio::sync::RwLock<KmsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_custom_key_store_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.custom_key_store_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'CustomKeyStoreId'");
        }

        let mut state = state.write().await;
        match state.delete_custom_key_store(&input.custom_key_store_id) {
            Ok(()) => wire::serialize_delete_custom_key_store_response(
                &wire::DeleteCustomKeyStoreResponse {},
            ),
            Err(e) => kms_error_response(&e),
        }
    }

    async fn handle_connect_custom_key_store(
        &self,
        state: &Arc<tokio::sync::RwLock<KmsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_connect_custom_key_store_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.custom_key_store_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'CustomKeyStoreId'");
        }

        let mut state = state.write().await;
        match state.connect_custom_key_store(&input.custom_key_store_id) {
            Ok(()) => wire::serialize_connect_custom_key_store_response(
                &wire::ConnectCustomKeyStoreResponse {},
            ),
            Err(e) => kms_error_response(&e),
        }
    }

    async fn handle_disconnect_custom_key_store(
        &self,
        state: &Arc<tokio::sync::RwLock<KmsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_disconnect_custom_key_store_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.custom_key_store_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'CustomKeyStoreId'");
        }

        let mut state = state.write().await;
        match state.disconnect_custom_key_store(&input.custom_key_store_id) {
            Ok(()) => wire::serialize_disconnect_custom_key_store_response(
                &wire::DisconnectCustomKeyStoreResponse {},
            ),
            Err(e) => kms_error_response(&e),
        }
    }

    // ---- Key Material Import handlers ----

    async fn handle_get_parameters_for_import(
        &self,
        state: &Arc<tokio::sync::RwLock<KmsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_parameters_for_import_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.key_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'KeyId'");
        }

        let state = state.read().await;
        match state.get_parameters_for_import(&input.key_id) {
            Ok((resolved_key_id, import_token, public_key, valid_to)) => {
                wire::serialize_get_parameters_for_import_response(
                    &wire::GetParametersForImportResponse {
                        key_id: Some(resolved_key_id),
                        import_token: Some(import_token),
                        public_key: Some(public_key),
                        parameters_valid_to: Some(valid_to),
                    },
                )
            }
            Err(e) => kms_error_response(&e),
        }
    }

    async fn handle_import_key_material(
        &self,
        state: &Arc<tokio::sync::RwLock<KmsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_import_key_material_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.key_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'KeyId'");
        }

        let mut state = state.write().await;
        match state.import_key_material(&input.key_id) {
            Ok(()) => wire::serialize_import_key_material_response(
                &wire::ImportKeyMaterialResponse::default(),
            ),
            Err(e) => kms_error_response(&e),
        }
    }

    async fn handle_delete_imported_key_material(
        &self,
        state: &Arc<tokio::sync::RwLock<KmsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_imported_key_material_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.key_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'KeyId'");
        }

        let mut state = state.write().await;
        match state.delete_imported_key_material(&input.key_id) {
            Ok(()) => wire::serialize_delete_imported_key_material_response(
                &wire::DeleteImportedKeyMaterialResponse::default(),
            ),
            Err(e) => kms_error_response(&e),
        }
    }

    // ---- DeriveSharedSecret handler ----

    async fn handle_derive_shared_secret(
        &self,
        state: &Arc<tokio::sync::RwLock<KmsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_derive_shared_secret_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.key_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'KeyId'");
        }
        let key_agreement_algorithm = if input.key_agreement_algorithm.is_empty() {
            "ECDH".to_string()
        } else {
            input.key_agreement_algorithm
        };

        let state = state.read().await;
        match state.describe_key(&input.key_id) {
            Ok(key) => {
                // Return mock 32-byte shared secret
                let mut secret_bytes = [0u8; 32];
                rand::rng().fill(&mut secret_bytes);
                wire::serialize_derive_shared_secret_response(&wire::DeriveSharedSecretResponse {
                    key_id: Some(key.arn.clone()),
                    key_agreement_algorithm: Some(key_agreement_algorithm),
                    shared_secret: Some(BASE64.encode(secret_bytes)),
                    key_origin: Some(key.origin.clone()),
                    ..Default::default()
                })
            }
            Err(e) => kms_error_response(&e),
        }
    }

    // ---- GenerateDataKeyPair handlers ----

    async fn handle_generate_data_key_pair(
        &self,
        state: &Arc<tokio::sync::RwLock<KmsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_generate_data_key_pair_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.key_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'KeyId'");
        }
        let key_pair_spec = if input.key_pair_spec.is_empty() {
            "RSA_2048".to_string()
        } else {
            input.key_pair_spec
        };
        let encryption_context = input.encryption_context.unwrap_or_default();

        let state = state.read().await;
        match state.describe_key(&input.key_id) {
            Ok(key) => {
                let mut private_key_bytes = vec![0u8; 64];
                let mut public_key_bytes = vec![0u8; 64];
                rand::rng().fill(&mut private_key_bytes[..]);
                rand::rng().fill(&mut public_key_bytes[..]);
                // Produce mock ciphertext for private key
                let ciphertext =
                    match state.encrypt(&key.key_id, &private_key_bytes, &encryption_context) {
                        Ok(r) => BASE64.encode(&r.ciphertext_blob),
                        Err(e) => return kms_error_response(&e),
                    };
                wire::serialize_generate_data_key_pair_response(
                    &wire::GenerateDataKeyPairResponse {
                        key_id: Some(key.arn.clone()),
                        key_pair_spec: Some(key_pair_spec),
                        private_key_ciphertext_blob: Some(ciphertext),
                        private_key_plaintext: Some(BASE64.encode(&private_key_bytes)),
                        public_key: Some(BASE64.encode(&public_key_bytes)),
                        ..Default::default()
                    },
                )
            }
            Err(e) => kms_error_response(&e),
        }
    }

    async fn handle_generate_data_key_pair_without_plaintext(
        &self,
        state: &Arc<tokio::sync::RwLock<KmsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_generate_data_key_pair_without_plaintext_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.key_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'KeyId'");
        }
        let key_pair_spec = if input.key_pair_spec.is_empty() {
            "RSA_2048".to_string()
        } else {
            input.key_pair_spec
        };
        let encryption_context = input.encryption_context.unwrap_or_default();

        let state = state.read().await;
        match state.describe_key(&input.key_id) {
            Ok(key) => {
                let mut private_key_bytes = vec![0u8; 64];
                let mut public_key_bytes = vec![0u8; 64];
                rand::rng().fill(&mut private_key_bytes[..]);
                rand::rng().fill(&mut public_key_bytes[..]);
                let ciphertext =
                    match state.encrypt(&key.key_id, &private_key_bytes, &encryption_context) {
                        Ok(r) => BASE64.encode(&r.ciphertext_blob),
                        Err(e) => return kms_error_response(&e),
                    };
                wire::serialize_generate_data_key_pair_without_plaintext_response(
                    &wire::GenerateDataKeyPairWithoutPlaintextResponse {
                        key_id: Some(key.arn.clone()),
                        key_pair_spec: Some(key_pair_spec),
                        private_key_ciphertext_blob: Some(ciphertext),
                        public_key: Some(BASE64.encode(&public_key_bytes)),
                        ..Default::default()
                    },
                )
            }
            Err(e) => kms_error_response(&e),
        }
    }

    // ---- UpdatePrimaryRegion handler ----

    async fn handle_update_primary_region(
        &self,
        state: &Arc<tokio::sync::RwLock<KmsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_primary_region_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.key_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'KeyId'");
        }
        if input.primary_region.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'PrimaryRegion'");
        }

        let state = state.read().await;
        match state.update_primary_region(&input.key_id, &input.primary_region) {
            Ok(()) => wire::serialize_update_primary_region_response(),
            Err(e) => kms_error_response(&e),
        }
    }
}

// --- Utility functions ---

/// Convert a state Key to a wire KeyMetadata.
fn key_to_metadata(key: &crate::types::Key) -> wire::KeyMetadata {
    let mut meta = wire::KeyMetadata {
        a_w_s_account_id: Some(key.account_id.clone()),
        arn: Some(key.arn.clone()),
        creation_date: Some(key.creation_date.timestamp() as f64),
        description: Some(key.description.clone()),
        enabled: Some(key.enabled),
        key_id: Some(key.key_id.clone()),
        key_manager: Some(key.key_manager.clone()),
        key_spec: Some(key.key_spec.clone()),
        key_state: Some(key.key_state.as_str().to_string()),
        key_usage: Some(key.key_usage.clone()),
        origin: Some(key.origin.clone()),
        customer_master_key_spec: Some(key.key_spec.clone()),
        multi_region: Some(key.multi_region),
        ..Default::default()
    };

    if let Some(enc_algos) = key.encryption_algorithms() {
        meta.encryption_algorithms = Some(enc_algos.iter().map(|s| s.to_string()).collect());
    }

    if let Some(sign_algos) = key.signing_algorithms() {
        meta.signing_algorithms = Some(sign_algos.iter().map(|s| s.to_string()).collect());
    }

    if let Some(ref dd) = key.deletion_date {
        meta.deletion_date = Some(dd.timestamp() as f64);
    }

    meta
}

fn kms_error_response(err: &KmsError) -> MockResponse {
    let (status, error_type) = match err {
        KmsError::InvalidKeySpec(_) => (400, "ValidationException"),
        KmsError::KeyPendingDeletion(_) => (400, "KMSInvalidStateException"),
        KmsError::KeyNotPendingDeletion(_) => (400, "KMSInvalidStateException"),
        KmsError::InvalidPendingWindowDays(_) => (400, "ValidationException"),
        KmsError::PlaintextLengthInvalid => (400, "ValidationException"),
        KmsError::InvalidDataKeySpec(_) => (400, "ValidationException"),
        KmsError::AliasAlreadyExists(_) => (400, "AlreadyExistsException"),
        KmsError::AliasNotFound(_) => (400, "NotFoundException"),
        KmsError::KeyNotValidForRotation(_) => (400, "UnsupportedOperationException"),
        KmsError::GetPublicKeyUnsupported(_) => (400, "UnsupportedOperationException"),
        KmsError::InvalidNumberOfBytes => (400, "ValidationException"),
        KmsError::KeyUsageIncompatibleWithGenerateMac(_, _) => (400, "InvalidKeyUsageException"),
        KmsError::KeySpecIncompatibleWithGenerateMac(_, _) => (400, "InvalidKeyUsageException"),
        KmsError::MacGenerationFailed => (500, "KMSInternalException"),
        KmsError::UnsupportedMacAlgorithm(_) => (400, "ValidationException"),
        KmsError::MacVerificationFailed => (400, "KMSInvalidMacException"),
        KmsError::KeyUsageIncompatibleWithSign(_, _) => (400, "InvalidKeyUsageException"),
        KmsError::GrantNotFoundForKey(_, _) => (400, "NotFoundException"),
        KmsError::GrantNotFound => (400, "NotFoundException"),
        KmsError::GrantIdOrTokenRequired => (400, "ValidationException"),
        KmsError::GrantIdNotFound(_) => (400, "NotFoundException"),
        KmsError::GrantIdNotFoundForKey(_) => (400, "NotFoundException"),
        KmsError::KeyNotValidForOnDemandRotation(_) => (400, "UnsupportedOperationException"),
        KmsError::KeyNotMultiRegion(_) => (400, "UnsupportedOperationException"),
        KmsError::CustomKeyStoreNameAlreadyExists(_) => {
            (400, "CloudHsmClusterInvalidConfigurationException")
        }
        KmsError::CustomKeyStoreNotFound(_) => (400, "CustomKeyStoreNotFoundException"),
        KmsError::KeyNotFound(_) => (400, "NotFoundException"),
        KmsError::InvalidCiphertext => (400, "InvalidCiphertextException"),
        KmsError::EncryptionFailed => (500, "KMSInternalException"),
        KmsError::KeyDisabled(_) => (400, "DisabledException"),
        KmsError::IncorrectKey => (400, "IncorrectKeyException"),
    };
    json_error_response(status, error_type, &err.to_string())
}

fn json_error_response(status: u16, code: &str, message: &str) -> MockResponse {
    let body = json!({
        "__type": code,
        "message": message,
    });
    MockResponse::json(status, body.to_string())
}
