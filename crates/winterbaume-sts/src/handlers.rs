use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use chrono::{Duration, Utc};
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, StatefulService,
    default_account_id,
};

use crate::state::{StsError, StsState};
use crate::types::AssumedRole;
use crate::views::StsStateView;
use crate::wire;

fn sts_error_response(err: StsError) -> MockResponse {
    match err {
        StsError::MissingAction => MockResponse::error(400, "MissingAction", &err.to_string()),
        StsError::InvalidAction { .. } => {
            MockResponse::error(400, "InvalidAction", &err.to_string())
        }
        StsError::InvalidParameterValue(_) => {
            MockResponse::error(400, "InvalidParameterValue", &err.to_string())
        }
        StsError::MissingParameter(_) => {
            MockResponse::error(400, "MissingParameter", &err.to_string())
        }
    }
}

/// STS service handler that processes awsQuery protocol requests.
pub struct StsService {
    pub(crate) state: Arc<BackendState<StsState>>,
    pub(crate) notifier: StateChangeNotifier<StsStateView>,
}

impl StsService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for StsService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for StsService {
    fn service_name(&self) -> &str {
        "sts"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://sts\..*\.amazonaws\.com",
            r"https?://sts\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl StsService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();

        // Parse form-encoded body (awsQuery protocol)
        let body_str = std::str::from_utf8(&request.body).unwrap_or("");
        let params = parse_query_string(body_str);

        let action = match params.get("Action") {
            Some(a) => a.as_str(),
            None => {
                return sts_error_response(StsError::MissingAction);
            }
        };

        let state = self.state.get(account_id, &region);

        let response = match action {
            "GetCallerIdentity" => self.handle_get_caller_identity(&state).await,
            "AssumeRole" => self.handle_assume_role(&state, &params, account_id).await,
            "GetSessionToken" => self.handle_get_session_token(&params).await,
            "AssumeRoleWithSAML" => {
                self.handle_assume_role_with_saml(&state, &params, account_id)
                    .await
            }
            "AssumeRoleWithWebIdentity" => {
                self.handle_assume_role_with_web_identity(&state, &params, account_id)
                    .await
            }
            "GetFederationToken" => {
                self.handle_get_federation_token(&state, &params, account_id)
                    .await
            }
            "AssumeRoot" => self.handle_assume_root(&params).await,
            "DecodeAuthorizationMessage" => self.handle_decode_authorization_message(&params).await,
            "GetAccessKeyInfo" => self.handle_get_access_key_info(&params, account_id).await,
            "GetDelegatedAccessToken" => self.handle_get_delegated_access_token(&params).await,
            "GetWebIdentityToken" => self.handle_get_web_identity_token(&params).await,
            _ => sts_error_response(StsError::InvalidAction {
                action: action.to_string(),
            }),
        };
        if response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_get_caller_identity(
        &self,
        state: &std::sync::Arc<tokio::sync::RwLock<StsState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let identity = state.get_caller_identity("AKIAIOSFODNN7EXAMPLE");

        wire::serialize_get_caller_identity_response(&wire::GetCallerIdentityResponse {
            arn: Some(identity.arn.clone()),
            user_id: Some(identity.user_id.clone()),
            account: Some(identity.account.clone()),
        })
    }

    async fn handle_assume_role(
        &self,
        state: &std::sync::Arc<tokio::sync::RwLock<StsState>>,
        params: &HashMap<String, String>,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_assume_role_request(params) {
            Ok(v) => v,
            Err(e) => return sts_error_response(StsError::InvalidParameterValue(e)),
        };
        if input.role_arn.is_empty() {
            return sts_error_response(StsError::MissingParameter("RoleArn"));
        }
        if input.role_session_name.is_empty() {
            return sts_error_response(StsError::MissingParameter("RoleSessionName"));
        }
        let duration_seconds: i64 = input.duration_seconds.map(|d| d as i64).unwrap_or(3600);

        // Extract account ID from role ARN if present
        let target_account =
            extract_account_from_arn(&input.role_arn).unwrap_or_else(|| account_id.to_string());

        // Generate temporary credentials
        let access_key_id = format!("ASIA{}", random_alphanumeric(16));
        let secret_access_key = random_alphanumeric(40);
        let session_token = generate_session_token();
        let assumed_role_id = format!("AROA{}", random_alphanumeric(13));
        let expiration = Utc::now() + Duration::seconds(duration_seconds);

        let assumed_role = AssumedRole {
            account_id: target_account.clone(),
            role_arn: input.role_arn.clone(),
            role_session_name: input.role_session_name.clone(),
            access_key_id: access_key_id.clone(),
            secret_access_key: secret_access_key.clone(),
            session_token: session_token.clone(),
            expiration,
            assumed_role_id: assumed_role_id.clone(),
        };

        let arn = assumed_role.arn();
        let user_id = assumed_role.user_id();

        {
            let mut state = state.write().await;
            state.assumed_roles.push(assumed_role);
        }

        wire::serialize_assume_role_response(&wire::AssumeRoleResponse {
            assumed_role_user: Some(wire::AssumedRoleUser {
                assumed_role_id: Some(user_id),
                arn: Some(arn),
            }),
            credentials: Some(wire::Credentials {
                access_key_id: Some(access_key_id),
                secret_access_key: Some(secret_access_key),
                session_token: Some(session_token),
                expiration: Some(expiration.format("%Y-%m-%dT%H:%M:%SZ").to_string()),
            }),
            packed_policy_size: Some(6),
            source_identity: None,
        })
    }

    async fn handle_assume_role_with_saml(
        &self,
        state: &std::sync::Arc<tokio::sync::RwLock<StsState>>,
        params: &HashMap<String, String>,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_assume_role_with_s_a_m_l_request(params) {
            Ok(v) => v,
            Err(e) => return sts_error_response(StsError::InvalidParameterValue(e)),
        };
        if input.role_arn.is_empty() {
            return sts_error_response(StsError::MissingParameter("RoleArn"));
        }
        if input.principal_arn.is_empty() {
            return sts_error_response(StsError::MissingParameter("PrincipalArn"));
        }
        let duration_seconds: i64 = input.duration_seconds.map(|d| d as i64).unwrap_or(3600);

        let target_account =
            extract_account_from_arn(&input.role_arn).unwrap_or_else(|| account_id.to_string());

        let access_key_id = format!("ASIA{}", random_alphanumeric(16));
        let secret_access_key = random_alphanumeric(40);
        let session_token = generate_session_token();
        let assumed_role_id = format!("AROA{}", random_alphanumeric(13));
        let role_session_name = "saml-session";
        let expiration = Utc::now() + Duration::seconds(duration_seconds);

        let assumed_role = AssumedRole {
            account_id: target_account.clone(),
            role_arn: input.role_arn.clone(),
            role_session_name: "saml-session".to_string(),
            access_key_id: access_key_id.clone(),
            secret_access_key: secret_access_key.clone(),
            session_token: session_token.clone(),
            expiration,
            assumed_role_id: assumed_role_id.clone(),
        };

        let arn = assumed_role.arn();
        let user_id = assumed_role.user_id();

        {
            let mut state = state.write().await;
            state.assumed_roles.push(assumed_role);
        }

        wire::serialize_assume_role_with_s_a_m_l_response(&wire::AssumeRoleWithSAMLResponse {
            assumed_role_user: Some(wire::AssumedRoleUser {
                assumed_role_id: Some(user_id),
                arn: Some(arn),
            }),
            credentials: Some(wire::Credentials {
                access_key_id: Some(access_key_id),
                secret_access_key: Some(secret_access_key),
                session_token: Some(session_token),
                expiration: Some(expiration.format("%Y-%m-%dT%H:%M:%SZ").to_string()),
            }),
            packed_policy_size: Some(6),
            issuer: Some(input.principal_arn.clone()),
            ..Default::default()
        })
    }

    async fn handle_assume_role_with_web_identity(
        &self,
        state: &std::sync::Arc<tokio::sync::RwLock<StsState>>,
        params: &HashMap<String, String>,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_assume_role_with_web_identity_request(params) {
            Ok(v) => v,
            Err(e) => return sts_error_response(StsError::InvalidParameterValue(e)),
        };
        if input.role_arn.is_empty() {
            return sts_error_response(StsError::MissingParameter("RoleArn"));
        }
        if input.role_session_name.is_empty() {
            return sts_error_response(StsError::MissingParameter("RoleSessionName"));
        }
        let duration_seconds: i64 = input.duration_seconds.map(|d| d as i64).unwrap_or(3600);

        let target_account =
            extract_account_from_arn(&input.role_arn).unwrap_or_else(|| account_id.to_string());

        let access_key_id = format!("ASIA{}", random_alphanumeric(16));
        let secret_access_key = random_alphanumeric(40);
        let session_token = generate_session_token();
        let assumed_role_id = format!("AROA{}", random_alphanumeric(13));
        let expiration = Utc::now() + Duration::seconds(duration_seconds);

        let assumed_role = AssumedRole {
            account_id: target_account.clone(),
            role_arn: input.role_arn.clone(),
            role_session_name: input.role_session_name.clone(),
            access_key_id: access_key_id.clone(),
            secret_access_key: secret_access_key.clone(),
            session_token: session_token.clone(),
            expiration,
            assumed_role_id: assumed_role_id.clone(),
        };

        let arn = assumed_role.arn();
        let user_id = assumed_role.user_id();

        {
            let mut state = state.write().await;
            state.assumed_roles.push(assumed_role);
        }

        wire::serialize_assume_role_with_web_identity_response(
            &wire::AssumeRoleWithWebIdentityResponse {
                assumed_role_user: Some(wire::AssumedRoleUser {
                    assumed_role_id: Some(user_id),
                    arn: Some(arn),
                }),
                credentials: Some(wire::Credentials {
                    access_key_id: Some(access_key_id),
                    secret_access_key: Some(secret_access_key),
                    session_token: Some(session_token),
                    expiration: Some(expiration.format("%Y-%m-%dT%H:%M:%SZ").to_string()),
                }),
                packed_policy_size: Some(6),
                ..Default::default()
            },
        )
    }

    async fn handle_get_federation_token(
        &self,
        _state: &std::sync::Arc<tokio::sync::RwLock<StsState>>,
        params: &HashMap<String, String>,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_get_federation_token_request(params) {
            Ok(v) => v,
            Err(e) => return sts_error_response(StsError::InvalidParameterValue(e)),
        };
        if input.name.is_empty() {
            return sts_error_response(StsError::MissingParameter("Name"));
        }
        let name = &input.name;
        let duration_seconds: i64 = input.duration_seconds.map(|d| d as i64).unwrap_or(43200);

        let access_key_id = format!("ASIA{}", random_alphanumeric(16));
        let secret_access_key = random_alphanumeric(40);
        let session_token = generate_session_token();
        let expiration = Utc::now() + Duration::seconds(duration_seconds);

        let federated_user_id = format!("{account_id}:{name}");
        let federated_user_arn = format!("arn:aws:sts::{account_id}:federated-user/{name}");

        wire::serialize_get_federation_token_response(&wire::GetFederationTokenResponse {
            credentials: Some(wire::Credentials {
                access_key_id: Some(access_key_id),
                secret_access_key: Some(secret_access_key),
                session_token: Some(session_token),
                expiration: Some(expiration.format("%Y-%m-%dT%H:%M:%SZ").to_string()),
            }),
            federated_user: Some(wire::FederatedUser {
                arn: Some(federated_user_arn),
                federated_user_id: Some(federated_user_id),
            }),
            packed_policy_size: Some(6),
        })
    }

    async fn handle_assume_root(&self, params: &HashMap<String, String>) -> MockResponse {
        let input = match wire::deserialize_assume_root_request(params) {
            Ok(v) => v,
            Err(e) => return sts_error_response(StsError::InvalidParameterValue(e)),
        };
        if input.target_principal.is_empty() {
            return sts_error_response(StsError::MissingParameter("TargetPrincipal"));
        }
        if input.task_policy_arn.arn.is_none() {
            return sts_error_response(StsError::MissingParameter("TaskPolicyArn"));
        }
        let duration_seconds: i64 = input.duration_seconds.map(|d| d as i64).unwrap_or(900);

        let access_key_id = format!("ASIA{}", random_alphanumeric(16));
        let secret_access_key = random_alphanumeric(40);
        let session_token = generate_session_token();
        let expiration = Utc::now() + Duration::seconds(duration_seconds);

        wire::serialize_assume_root_response(&wire::AssumeRootResponse {
            credentials: Some(wire::Credentials {
                access_key_id: Some(access_key_id),
                secret_access_key: Some(secret_access_key),
                session_token: Some(session_token),
                expiration: Some(expiration.format("%Y-%m-%dT%H:%M:%SZ").to_string()),
            }),
            source_identity: None,
        })
    }

    async fn handle_decode_authorization_message(
        &self,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_decode_authorization_message_request(params) {
            Ok(v) => v,
            Err(e) => return sts_error_response(StsError::InvalidParameterValue(e)),
        };
        if input.encoded_message.is_empty() {
            return sts_error_response(StsError::MissingParameter("EncodedMessage"));
        }
        // Return the encoded message as the "decoded" message (mock behaviour)
        wire::serialize_decode_authorization_message_response(
            &wire::DecodeAuthorizationMessageResponse {
                decoded_message: Some(input.encoded_message),
            },
        )
    }

    async fn handle_get_access_key_info(
        &self,
        params: &HashMap<String, String>,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_get_access_key_info_request(params) {
            Ok(v) => v,
            Err(e) => return sts_error_response(StsError::InvalidParameterValue(e)),
        };
        if input.access_key_id.is_empty() {
            return sts_error_response(StsError::MissingParameter("AccessKeyId"));
        }
        wire::serialize_get_access_key_info_response(&wire::GetAccessKeyInfoResponse {
            account: Some(account_id.to_string()),
        })
    }

    async fn handle_get_delegated_access_token(
        &self,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_delegated_access_token_request(params) {
            Ok(v) => v,
            Err(e) => return sts_error_response(StsError::InvalidParameterValue(e)),
        };
        if input.trade_in_token.is_empty() {
            return sts_error_response(StsError::MissingParameter("TradeInToken"));
        }

        let access_key_id = format!("ASIA{}", random_alphanumeric(16));
        let secret_access_key = random_alphanumeric(40);
        let session_token = generate_session_token();
        let expiration = Utc::now() + Duration::seconds(3600);

        wire::serialize_get_delegated_access_token_response(
            &wire::GetDelegatedAccessTokenResponse {
                credentials: Some(wire::Credentials {
                    access_key_id: Some(access_key_id),
                    secret_access_key: Some(secret_access_key),
                    session_token: Some(session_token),
                    expiration: Some(expiration.format("%Y-%m-%dT%H:%M:%SZ").to_string()),
                }),
                assumed_principal: None,
                packed_policy_size: None,
            },
        )
    }

    async fn handle_get_web_identity_token(
        &self,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_web_identity_token_request(params) {
            Ok(v) => v,
            Err(e) => return sts_error_response(StsError::InvalidParameterValue(e)),
        };
        let duration_seconds: i64 = input.duration_seconds.map(|d| d as i64).unwrap_or(3600);

        let expiration = Utc::now() + Duration::seconds(duration_seconds);
        let token = generate_session_token();

        wire::serialize_get_web_identity_token_response(&wire::GetWebIdentityTokenResponse {
            web_identity_token: Some(token),
            expiration: Some(expiration.format("%Y-%m-%dT%H:%M:%SZ").to_string()),
        })
    }

    async fn handle_get_session_token(&self, params: &HashMap<String, String>) -> MockResponse {
        let input = match wire::deserialize_get_session_token_request(params) {
            Ok(v) => v,
            Err(e) => return sts_error_response(StsError::InvalidParameterValue(e)),
        };
        let duration_seconds: i64 = input.duration_seconds.map(|d| d as i64).unwrap_or(43200);

        let expiration = Utc::now() + Duration::seconds(duration_seconds);
        let access_key_id = "ASIAIOSTOKENEXAMPLE";
        let secret_access_key = "wJalrXUtnFEMI/K7MDENG/bPxRfiCYzEXAMPLEKEY";
        let session_token = generate_session_token();

        wire::serialize_get_session_token_response(&wire::GetSessionTokenResponse {
            credentials: Some(wire::Credentials {
                access_key_id: Some(access_key_id.to_string()),
                secret_access_key: Some(secret_access_key.to_string()),
                session_token: Some(session_token),
                expiration: Some(expiration.format("%Y-%m-%dT%H:%M:%SZ").to_string()),
            }),
        })
    }
}

// --- Utility functions ---

fn parse_query_string(s: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for pair in s.split('&') {
        if let Some((key, value)) = pair.split_once('=') {
            let key = urldecode(key);
            let value = urldecode(value);
            map.insert(key, value);
        }
    }
    map
}

fn urldecode(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.bytes();
    while let Some(b) = chars.next() {
        match b {
            b'+' => result.push(' '),
            b'%' => {
                let hi = chars.next().and_then(hex_val);
                let lo = chars.next().and_then(hex_val);
                if let (Some(hi), Some(lo)) = (hi, lo) {
                    result.push((hi << 4 | lo) as char);
                }
            }
            _ => result.push(b as char),
        }
    }
    result
}

fn hex_val(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

fn extract_account_from_arn(arn: &str) -> Option<String> {
    // ARN format: arn:partition:service:region:account:resource
    let parts: Vec<&str> = arn.split(':').collect();
    if parts.len() >= 5 && !parts[4].is_empty() {
        Some(parts[4].to_string())
    } else {
        None
    }
}

fn random_alphanumeric(len: usize) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut result = String::with_capacity(len);
    let chars = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    while result.len() < len {
        let uuid = uuid::Uuid::new_v4();
        let mut hasher = DefaultHasher::new();
        uuid.hash(&mut hasher);
        let hash = hasher.finish();
        for i in 0..8 {
            if result.len() >= len {
                break;
            }
            let idx = ((hash >> (i * 8)) & 0xFF) as usize % chars.len();
            result.push(chars[idx] as char);
        }
    }
    result
}

fn generate_session_token() -> String {
    let prefix = "FQoGZXIvYXdzEBYaDwinterbaume";
    let suffix = random_alphanumeric(330);
    format!("{prefix}{suffix}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_query_string() {
        let qs = "Action=GetCallerIdentity&Version=2011-06-15";
        let params = parse_query_string(qs);
        assert_eq!(params.get("Action").unwrap(), "GetCallerIdentity");
        assert_eq!(params.get("Version").unwrap(), "2011-06-15");
    }

    #[test]
    fn test_urldecode() {
        assert_eq!(urldecode("hello+world"), "hello world");
        assert_eq!(urldecode("foo%20bar"), "foo bar");
        assert_eq!(urldecode("a%3Db"), "a=b");
    }

    #[test]
    fn test_xml_escape() {
        assert_eq!(xml_escape("a&b<c>d"), "a&amp;b&lt;c&gt;d");
    }

    #[test]
    fn test_extract_account_from_arn() {
        assert_eq!(
            extract_account_from_arn("arn:aws:iam::123456789012:role/test"),
            Some("123456789012".to_string())
        );
        assert_eq!(extract_account_from_arn("invalid"), None);
    }
}
