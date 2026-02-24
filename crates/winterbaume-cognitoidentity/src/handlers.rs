use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
    StatefulService,
};

use crate::state::{CognitoIdentityError, CognitoIdentityState};
use crate::types::CognitoIdentityProvider;
use crate::views::CognitoIdentityStateView;
use crate::wire;

pub struct CognitoIdentityService {
    pub(crate) state: Arc<BackendState<CognitoIdentityState>>,
    pub(crate) notifier: StateChangeNotifier<CognitoIdentityStateView>,
}

impl CognitoIdentityService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for CognitoIdentityService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for CognitoIdentityService {
    fn service_name(&self) -> &str {
        "cognito-identity"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://cognito-identity\..*\.amazonaws\.com",
            r"https?://cognito-identity\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl CognitoIdentityService {
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

        let response = match action.as_str() {
            "CreateIdentityPool" => {
                self.handle_create_identity_pool(&state, body_bytes, &region)
                    .await
            }
            "DescribeIdentityPool" => self.handle_describe_identity_pool(&state, body_bytes).await,
            "UpdateIdentityPool" => self.handle_update_identity_pool(&state, body_bytes).await,
            "DeleteIdentityPool" => self.handle_delete_identity_pool(&state, body_bytes).await,
            "ListIdentityPools" => self.handle_list_identity_pools(&state).await,
            "GetId" => self.handle_get_id(&state, body_bytes, &region).await,
            "ListIdentities" => self.handle_list_identities(&state, body_bytes).await,
            "GetCredentialsForIdentity" => {
                self.handle_get_credentials_for_identity(body_bytes).await
            }
            "GetOpenIdToken" => self.handle_get_open_id_token(body_bytes).await,
            "GetOpenIdTokenForDeveloperIdentity" => {
                self.handle_get_open_id_token_for_developer_identity(body_bytes, &region)
                    .await
            }
            "DeleteIdentities" => self.handle_delete_identities(&state, body_bytes).await,
            "DescribeIdentity" => self.handle_describe_identity(&state, body_bytes).await,
            "GetIdentityPoolRoles" => {
                self.handle_get_identity_pool_roles(&state, body_bytes)
                    .await
            }
            "SetIdentityPoolRoles" => {
                self.handle_set_identity_pool_roles(&state, body_bytes)
                    .await
            }
            "GetPrincipalTagAttributeMap" => {
                self.handle_get_principal_tag_attribute_map(&state, body_bytes)
                    .await
            }
            "SetPrincipalTagAttributeMap" => {
                self.handle_set_principal_tag_attribute_map(&state, body_bytes)
                    .await
            }
            "TagResource" => self.handle_tag_resource(&state, body_bytes).await,
            "UntagResource" => self.handle_untag_resource(&state, body_bytes).await,
            "ListTagsForResource" => self.handle_list_tags_for_resource(&state, body_bytes).await,
            "LookupDeveloperIdentity" => {
                self.handle_lookup_developer_identity(&state, body_bytes)
                    .await
            }
            "MergeDeveloperIdentities" => {
                self.handle_merge_developer_identities(&state, body_bytes, &region)
                    .await
            }
            "UnlinkDeveloperIdentity" => {
                self.handle_unlink_developer_identity(&state, body_bytes)
                    .await
            }
            "UnlinkIdentity" => self.handle_unlink_identity(&state, body_bytes).await,
            _ => json_error_response(
                400,
                "InvalidAction",
                &format!("Could not find operation {action} for Cognito Identity"),
            ),
        };

        if matches!(
            action.as_str(),
            "CreateIdentityPool"
                | "UpdateIdentityPool"
                | "DeleteIdentityPool"
                | "DeleteIdentities"
                | "SetIdentityPoolRoles"
                | "SetPrincipalTagAttributeMap"
                | "TagResource"
                | "UntagResource"
                | "MergeDeveloperIdentities"
                | "UnlinkDeveloperIdentity"
                | "UnlinkIdentity"
        ) && response.status / 100 == 2
        {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_create_identity_pool(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdentityState>>,
        body: &[u8],
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_identity_pool_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.identity_pool_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'IdentityPoolName'");
        }

        let supported_login_providers = input.supported_login_providers.unwrap_or_default();
        let oidc_arns = input.open_id_connect_provider_a_r_ns.unwrap_or_default();
        let cognito_providers = input
            .cognito_identity_providers
            .map(wire_providers_to_domain)
            .unwrap_or_default();
        let saml_arns = input.saml_provider_a_r_ns.unwrap_or_default();

        let mut state = state.write().await;
        match state.create_identity_pool(
            &input.identity_pool_name,
            input.allow_unauthenticated_identities,
            supported_login_providers,
            input.developer_provider_name,
            oidc_arns,
            cognito_providers,
            saml_arns,
            region,
        ) {
            Ok(pool) => wire::serialize_create_identity_pool_response(&identity_pool_to_wire(pool)),
            Err(e) => cognito_identity_error_response(&e),
        }
    }

    async fn handle_describe_identity_pool(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdentityState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_identity_pool_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.identity_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'IdentityPoolId'");
        }

        let state = state.read().await;
        match state.describe_identity_pool(&input.identity_pool_id) {
            Ok(pool) => {
                wire::serialize_describe_identity_pool_response(&identity_pool_to_wire(pool))
            }
            Err(e) => cognito_identity_error_response(&e),
        }
    }

    async fn handle_update_identity_pool(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdentityState>>,
        body: &[u8],
    ) -> MockResponse {
        // The wire request body is fully spread across the IdentityPool fields, so
        // we re-parse via serde_json::Value to detect which fields the client
        // actually sent (the typed struct cannot disambiguate "absent" from
        // "default-valued"). For this single edge case the typed deserializer is
        // still used to validate well-formedness and grab the simple scalar fields.
        let input = match wire::deserialize_update_identity_pool_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.identity_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'IdentityPoolId'");
        }
        if input.identity_pool_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'IdentityPoolName'");
        }

        // Re-parse to detect "field was sent" vs "field was omitted" for optional
        // collection fields whose typed deserialisation maps both to None/empty.
        let raw: Value = serde_json::from_slice(body).unwrap_or(Value::Null);

        let supported_login_providers = if raw.get("SupportedLoginProviders").is_some() {
            Some(input.supported_login_providers.unwrap_or_default())
        } else {
            None
        };
        let oidc_arns = if raw.get("OpenIdConnectProviderARNs").is_some() {
            Some(input.open_id_connect_provider_a_r_ns.unwrap_or_default())
        } else {
            None
        };
        let cognito_providers = if raw.get("CognitoIdentityProviders").is_some() {
            Some(
                input
                    .cognito_identity_providers
                    .map(wire_providers_to_domain)
                    .unwrap_or_default(),
            )
        } else {
            None
        };
        let saml_arns = if raw.get("SamlProviderARNs").is_some() {
            Some(input.saml_provider_a_r_ns.unwrap_or_default())
        } else {
            None
        };

        let mut state = state.write().await;
        match state.update_identity_pool(
            &input.identity_pool_id,
            &input.identity_pool_name,
            input.allow_unauthenticated_identities,
            supported_login_providers,
            input.developer_provider_name,
            oidc_arns,
            cognito_providers,
            saml_arns,
        ) {
            Ok(pool) => wire::serialize_update_identity_pool_response(&identity_pool_to_wire(pool)),
            Err(e) => cognito_identity_error_response(&e),
        }
    }

    async fn handle_delete_identity_pool(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdentityState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_identity_pool_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.identity_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'IdentityPoolId'");
        }

        let mut state = state.write().await;
        match state.delete_identity_pool(&input.identity_pool_id) {
            Ok(()) => wire::serialize_delete_identity_pool_response(),
            Err(e) => cognito_identity_error_response(&e),
        }
    }

    async fn handle_list_identity_pools(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdentityState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let pools = state.list_identity_pools();
        let entries: Vec<wire::IdentityPoolShortDescription> = pools
            .iter()
            .map(|p| wire::IdentityPoolShortDescription {
                identity_pool_id: Some(p.identity_pool_id.clone()),
                identity_pool_name: Some(p.identity_pool_name.clone()),
            })
            .collect();
        wire::serialize_list_identity_pools_response(&wire::ListIdentityPoolsResponse {
            identity_pools: Some(entries),
            next_token: None,
        })
    }

    async fn handle_get_id(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdentityState>>,
        body: &[u8],
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_get_id_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.identity_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'IdentityPoolId'");
        }

        let mut state = state.write().await;
        match state.get_id(&input.identity_pool_id, region) {
            Ok(identity_id) => wire::serialize_get_id_response(&wire::GetIdResponse {
                identity_id: Some(identity_id),
            }),
            Err(e) => cognito_identity_error_response(&e),
        }
    }

    async fn handle_list_identities(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdentityState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_identities_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.identity_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'IdentityPoolId'");
        }

        let state = state.read().await;
        match state.list_identities(&input.identity_pool_id) {
            Ok(identities) => {
                let entries: Vec<wire::IdentityDescription> = identities
                    .iter()
                    .map(|i| wire::IdentityDescription {
                        identity_id: Some(i.identity_id.clone()),
                        ..Default::default()
                    })
                    .collect();
                wire::serialize_list_identities_response(&wire::ListIdentitiesResponse {
                    identity_pool_id: Some(input.identity_pool_id.clone()),
                    identities: Some(entries),
                    next_token: None,
                })
            }
            Err(e) => cognito_identity_error_response(&e),
        }
    }

    // STUB[no-auth]: The mock has no real STS token vending; returns static example credentials.
    async fn handle_get_credentials_for_identity(&self, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_get_credentials_for_identity_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let identity_id = if input.identity_id.is_empty() {
            "unknown".to_string()
        } else {
            input.identity_id
        };

        let expiration = chrono::Utc::now() + chrono::Duration::hours(1);

        wire::serialize_get_credentials_for_identity_response(
            &wire::GetCredentialsForIdentityResponse {
                identity_id: Some(identity_id),
                credentials: Some(wire::Credentials {
                    access_key_id: Some("AKIAIOSFODNN7EXAMPLE".to_string()),
                    secret_key: Some("wJalrXUtnFEMI/K7MDENG/bPxRfiCYzEXAMPLEKEY".to_string()),
                    session_token: Some("AQoDYXdzEJr...".to_string()),
                    expiration: Some(expiration.timestamp() as f64),
                }),
            },
        )
    }

    // STUB[no-auth]: The mock has no OpenID Connect token issuance; returns a random UUID as token.
    async fn handle_get_open_id_token(&self, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_get_open_id_token_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let identity_id = if input.identity_id.is_empty() {
            "unknown".to_string()
        } else {
            input.identity_id
        };

        wire::serialize_get_open_id_token_response(&wire::GetOpenIdTokenResponse {
            identity_id: Some(identity_id),
            token: Some(uuid::Uuid::new_v4().to_string()),
        })
    }

    // STUB[no-auth]: The mock has no OpenID Connect token issuance; returns a random UUID as token.
    async fn handle_get_open_id_token_for_developer_identity(
        &self,
        body: &[u8],
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_get_open_id_token_for_developer_identity_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let identity_id = input
            .identity_id
            .unwrap_or_else(|| format!("{}:{}", region, uuid::Uuid::new_v4()));

        wire::serialize_get_open_id_token_for_developer_identity_response(
            &wire::GetOpenIdTokenForDeveloperIdentityResponse {
                identity_id: Some(identity_id),
                token: Some(uuid::Uuid::new_v4().to_string()),
            },
        )
    }

    async fn handle_delete_identities(
        &self,
        state: &Arc<tokio::sync::RwLock<crate::state::CognitoIdentityState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_identities_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        let _unprocessed = state.delete_identities(&input.identity_ids_to_delete);
        wire::serialize_delete_identities_response(&wire::DeleteIdentitiesResponse {
            unprocessed_identity_ids: Some(vec![]),
        })
    }

    async fn handle_describe_identity(
        &self,
        state: &Arc<tokio::sync::RwLock<crate::state::CognitoIdentityState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_identity_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.identity_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'IdentityId'");
        }
        let state = state.read().await;
        match state.describe_identity(&input.identity_id) {
            Ok(identity) => {
                wire::serialize_describe_identity_response(&wire::IdentityDescription {
                    identity_id: Some(identity.identity_id.clone()),
                    creation_date: Some(identity.creation_date.timestamp() as f64),
                    last_modified_date: Some(identity.last_modified_date.timestamp() as f64),
                    logins: Some(identity.logins.clone()),
                })
            }
            Err(e) => cognito_identity_error_response(&e),
        }
    }

    async fn handle_get_identity_pool_roles(
        &self,
        state: &Arc<tokio::sync::RwLock<crate::state::CognitoIdentityState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_identity_pool_roles_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.identity_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'IdentityPoolId'");
        }
        let state = state.read().await;
        match state.get_identity_pool_roles(&input.identity_pool_id) {
            Ok(roles) => wire::serialize_get_identity_pool_roles_response(
                &wire::GetIdentityPoolRolesResponse {
                    identity_pool_id: Some(input.identity_pool_id.clone()),
                    roles: Some(roles.roles.clone()),
                    role_mappings: None,
                },
            ),
            Err(e) => cognito_identity_error_response(&e),
        }
    }

    async fn handle_set_identity_pool_roles(
        &self,
        state: &Arc<tokio::sync::RwLock<crate::state::CognitoIdentityState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_set_identity_pool_roles_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.identity_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'IdentityPoolId'");
        }
        let mut state = state.write().await;
        match state.set_identity_pool_roles(&input.identity_pool_id, input.roles) {
            Ok(()) => wire::serialize_set_identity_pool_roles_response(),
            Err(e) => cognito_identity_error_response(&e),
        }
    }

    async fn handle_get_principal_tag_attribute_map(
        &self,
        state: &Arc<tokio::sync::RwLock<crate::state::CognitoIdentityState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_principal_tag_attribute_map_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.identity_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'IdentityPoolId'");
        }
        if input.identity_provider_name.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing 'IdentityProviderName'",
            );
        }
        let state = state.read().await;
        let entry = state.get_principal_tag_attribute_map(
            &input.identity_pool_id,
            &input.identity_provider_name,
        );
        match entry {
            Some(e) => wire::serialize_get_principal_tag_attribute_map_response(
                &wire::GetPrincipalTagAttributeMapResponse {
                    identity_pool_id: Some(e.identity_pool_id.clone()),
                    identity_provider_name: Some(e.identity_provider_name.clone()),
                    use_defaults: Some(e.use_defaults),
                    principal_tags: Some(e.principal_tags.clone()),
                },
            ),
            None => wire::serialize_get_principal_tag_attribute_map_response(
                &wire::GetPrincipalTagAttributeMapResponse {
                    identity_pool_id: Some(input.identity_pool_id.clone()),
                    identity_provider_name: Some(input.identity_provider_name.clone()),
                    use_defaults: Some(true),
                    principal_tags: Some(HashMap::new()),
                },
            ),
        }
    }

    async fn handle_set_principal_tag_attribute_map(
        &self,
        state: &Arc<tokio::sync::RwLock<crate::state::CognitoIdentityState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_set_principal_tag_attribute_map_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.identity_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'IdentityPoolId'");
        }
        if input.identity_provider_name.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing 'IdentityProviderName'",
            );
        }
        let use_defaults = input.use_defaults.unwrap_or(true);
        let principal_tags = input.principal_tags.unwrap_or_default();
        let mut state = state.write().await;
        state.set_principal_tag_attribute_map(
            &input.identity_pool_id,
            &input.identity_provider_name,
            use_defaults,
            principal_tags,
        );
        wire::serialize_set_principal_tag_attribute_map_response(
            &wire::SetPrincipalTagAttributeMapResponse {
                identity_pool_id: Some(input.identity_pool_id.clone()),
                identity_provider_name: Some(input.identity_provider_name.clone()),
                use_defaults: Some(use_defaults),
                principal_tags: None,
            },
        )
    }

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<crate::state::CognitoIdentityState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ResourceArn'");
        }
        let mut state = state.write().await;
        state.tag_resource(&input.resource_arn, input.tags);
        wire::serialize_tag_resource_response(&wire::TagResourceResponse {})
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<crate::state::CognitoIdentityState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ResourceArn'");
        }
        let mut state = state.write().await;
        state.untag_resource(&input.resource_arn, &input.tag_keys);
        wire::serialize_untag_resource_response(&wire::UntagResourceResponse {})
    }

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<crate::state::CognitoIdentityState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'ResourceArn'");
        }
        let state = state.read().await;
        let tags = state.list_tags_for_resource(&input.resource_arn);
        wire::serialize_list_tags_for_resource_response(&wire::ListTagsForResourceResponse {
            tags: Some(tags),
        })
    }

    async fn handle_lookup_developer_identity(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdentityState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_lookup_developer_identity_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.identity_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'IdentityPoolId'");
        }

        let state = state.read().await;
        match state.lookup_developer_identity(
            &input.identity_pool_id,
            input.identity_id.as_deref(),
            input.developer_user_identifier.as_deref(),
        ) {
            Ok((iid, dev_uids)) => wire::serialize_lookup_developer_identity_response(
                &wire::LookupDeveloperIdentityResponse {
                    identity_id: Some(iid),
                    developer_user_identifier_list: Some(dev_uids),
                    next_token: None,
                },
            ),
            Err(e) => cognito_identity_error_response(&e),
        }
    }

    async fn handle_merge_developer_identities(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdentityState>>,
        body: &[u8],
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_merge_developer_identities_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.identity_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'IdentityPoolId'");
        }
        if input.source_user_identifier.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing 'SourceUserIdentifier'",
            );
        }
        if input.destination_user_identifier.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing 'DestinationUserIdentifier'",
            );
        }
        if input.developer_provider_name.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing 'DeveloperProviderName'",
            );
        }

        let mut state = state.write().await;
        match state.merge_developer_identities(
            &input.identity_pool_id,
            &input.source_user_identifier,
            &input.destination_user_identifier,
            &input.developer_provider_name,
            region,
        ) {
            Ok(identity_id) => wire::serialize_merge_developer_identities_response(
                &wire::MergeDeveloperIdentitiesResponse {
                    identity_id: Some(identity_id),
                },
            ),
            Err(e) => cognito_identity_error_response(&e),
        }
    }

    async fn handle_unlink_developer_identity(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdentityState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_unlink_developer_identity_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.identity_pool_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'IdentityPoolId'");
        }
        if input.identity_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'IdentityId'");
        }
        if input.developer_provider_name.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing 'DeveloperProviderName'",
            );
        }
        if input.developer_user_identifier.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing 'DeveloperUserIdentifier'",
            );
        }

        let mut state = state.write().await;
        match state.unlink_developer_identity(
            &input.identity_pool_id,
            &input.identity_id,
            &input.developer_provider_name,
            &input.developer_user_identifier,
        ) {
            Ok(()) => wire::serialize_unlink_developer_identity_response(),
            Err(e) => cognito_identity_error_response(&e),
        }
    }

    async fn handle_unlink_identity(
        &self,
        state: &Arc<tokio::sync::RwLock<CognitoIdentityState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_unlink_identity_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.identity_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing 'IdentityId'");
        }

        let mut state = state.write().await;
        match state.unlink_identity(&input.identity_id, &input.logins) {
            Ok(()) => wire::serialize_unlink_identity_response(),
            Err(e) => cognito_identity_error_response(&e),
        }
    }
}

fn identity_pool_to_wire(pool: &crate::types::IdentityPool) -> wire::IdentityPool {
    let providers: Vec<wire::CognitoIdentityProvider> = pool
        .cognito_identity_providers
        .iter()
        .map(|p| wire::CognitoIdentityProvider {
            provider_name: Some(p.provider_name.clone()),
            client_id: Some(p.client_id.clone()),
            server_side_token_check: Some(p.server_side_token_check),
        })
        .collect();

    wire::IdentityPool {
        identity_pool_id: pool.identity_pool_id.clone(),
        identity_pool_name: pool.identity_pool_name.clone(),
        allow_unauthenticated_identities: pool.allow_unauthenticated_identities,
        supported_login_providers: Some(pool.supported_login_providers.clone()),
        developer_provider_name: pool.developer_provider_name.clone(),
        open_id_connect_provider_a_r_ns: Some(pool.open_id_connect_provider_arns.clone()),
        cognito_identity_providers: Some(providers),
        saml_provider_a_r_ns: Some(pool.saml_provider_arns.clone()),
        ..Default::default()
    }
}

fn wire_providers_to_domain(
    providers: Vec<wire::CognitoIdentityProvider>,
) -> Vec<CognitoIdentityProvider> {
    providers
        .into_iter()
        .map(|p| CognitoIdentityProvider {
            provider_name: p.provider_name.unwrap_or_default(),
            client_id: p.client_id.unwrap_or_default(),
            server_side_token_check: p.server_side_token_check.unwrap_or(false),
        })
        .collect()
}

fn cognito_identity_error_response(err: &CognitoIdentityError) -> MockResponse {
    let (status, error_type) = match err {
        CognitoIdentityError::InvalidPoolName { .. } => (400, "ValidationException"),
        CognitoIdentityError::ResourceNotFound { .. } => (400, "ResourceNotFoundException"),
    };
    MockResponse::json(
        status,
        json!({"__type": error_type, "message": err.to_string()}).to_string(),
    )
}

fn json_error_response(status: u16, code: &str, message: &str) -> MockResponse {
    let body = json!({
        "__type": code,
        "message": message,
    });
    MockResponse::json(status, body.to_string())
}
