use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
    extract_path, extract_query_string, percent_decode, rest_json_error,
};

use crate::state::{
    ChallengeRecord, ConnectorRecord, PcaConnectorScepError, PcaConnectorScepState,
};
use crate::views::PcaConnectorScepStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct PcaConnectorScepService {
    pub(crate) state: Arc<BackendState<PcaConnectorScepState>>,
    pub(crate) notifier: StateChangeNotifier<PcaConnectorScepStateView>,
}

impl PcaConnectorScepService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for PcaConnectorScepService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for PcaConnectorScepService {
    fn service_name(&self) -> &str {
        "pca-connector-scep"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://pca-connector-scep\..*\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

type SharedState = Arc<tokio::sync::RwLock<PcaConnectorScepState>>;

impl PcaConnectorScepService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let method = request.method.as_str().to_uppercase();

        let body: Value = if request.body.is_empty() {
            json!({})
        } else {
            match serde_json::from_slice(&request.body) {
                Ok(v) => v,
                Err(_) => return rest_json_error(400, "BadRequestException", "Invalid JSON body"),
            }
        };

        let segments: Vec<String> = path
            .trim_start_matches('/')
            .split('/')
            .filter(|s| !s.is_empty())
            .map(percent_decode)
            .collect();
        let segs: Vec<&str> = segments.iter().map(|s| s.as_str()).collect();

        let response = match (method.as_str(), segs.as_slice()) {
            ("POST", ["connectors"]) => {
                self.handle_create_connector(&state, account_id, &region, &body)
                    .await
            }
            ("GET", ["connectors", arn]) => self.handle_get_connector(&state, arn).await,
            ("DELETE", ["connectors", arn]) => self.handle_delete_connector(&state, arn).await,
            ("GET", ["connectors"]) => self.handle_list_connectors(&state).await,
            ("POST", ["challenges"]) => {
                self.handle_create_challenge(&state, account_id, &region, &body)
                    .await
            }
            ("DELETE", ["challenges", arn]) => self.handle_delete_challenge(&state, arn).await,
            ("GET", ["challengeMetadata", arn]) => {
                self.handle_get_challenge_metadata(&state, arn).await
            }
            ("GET", ["challengePasswords", arn]) => {
                self.handle_get_challenge_password(&state, arn).await
            }
            ("GET", ["challengeMetadata"]) => {
                self.handle_list_challenge_metadata(&state, &request.uri)
                    .await
            }
            ("GET", ["tags", arn]) => self.handle_list_tags(&state, arn).await,
            ("POST", ["tags", arn]) => self.handle_tag_resource(&state, arn, &body).await,
            ("DELETE", ["tags", arn]) => {
                self.handle_untag_resource(&state, arn, &request.uri).await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        };

        use winterbaume_core::StatefulService;
        if response.status / 100 == 2 && method != "GET" {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_create_connector(
        &self,
        state: &SharedState,
        account_id: &str,
        region: &str,
        body: &Value,
    ) -> MockResponse {
        let ca_arn = match require_str(body, "CertificateAuthorityArn") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let id = uuid::Uuid::new_v4().simple().to_string();
        let arn = format!("arn:aws:pca-connector-scep:{region}:{account_id}:connector/{id}");
        let now = chrono::Utc::now().timestamp() as f64;
        let connector_type = if body
            .get("MobileDeviceManagement")
            .and_then(|v| v.as_object())
            .map(|o| !o.is_empty())
            .unwrap_or(false)
        {
            "MOBILE_DEVICE_MANAGEMENT".to_string()
        } else {
            "GENERAL_PURPOSE".to_string()
        };
        let tags = parse_object_string_map(body.get("Tags"));
        let record = ConnectorRecord {
            arn: arn.clone(),
            certificate_authority_arn: ca_arn,
            created_at: now,
            updated_at: now,
            status: "ACTIVE".to_string(),
            connector_type,
            endpoint: format!("https://scep.{region}.amazonaws.com/{id}"),
        };
        let mut state = state.write().await;
        state.create_connector(record);
        if !tags.is_empty() {
            state.tag_resource(&arn, tags);
        }
        let mut resp = wire::serialize_create_connector_response(&wire::CreateConnectorResponse {
            connector_arn: Some(arn),
        });
        resp.status = 202;
        resp
    }

    async fn handle_get_connector(&self, state: &SharedState, arn: &str) -> MockResponse {
        let state = state.read().await;
        match state.get_connector(arn) {
            Ok(c) => wire::serialize_get_connector_response(&wire::GetConnectorResponse {
                connector: Some(connector_to_wire(c)),
            }),
            Err(e) => err_response(&e),
        }
    }

    async fn handle_delete_connector(&self, state: &SharedState, arn: &str) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_connector(arn) {
            Ok(()) => {
                let mut resp = wire::serialize_delete_connector_response();
                resp.status = 202;
                resp
            }
            Err(e) => err_response(&e),
        }
    }

    async fn handle_list_connectors(&self, state: &SharedState) -> MockResponse {
        let state = state.read().await;
        let connectors: Vec<wire::ConnectorSummary> = state
            .list_connectors()
            .into_iter()
            .map(|c| wire::ConnectorSummary {
                arn: Some(c.arn.clone()),
                certificate_authority_arn: Some(c.certificate_authority_arn.clone()),
                created_at: Some(c.created_at),
                endpoint: Some(c.endpoint.clone()),
                mobile_device_management: None,
                open_id_configuration: None,
                status: Some(c.status.clone()),
                status_reason: None,
                r#type: Some(c.connector_type.clone()),
                updated_at: Some(c.updated_at),
            })
            .collect();
        wire::serialize_list_connectors_response(&wire::ListConnectorsResponse {
            connectors: Some(connectors),
            next_token: None,
        })
    }

    async fn handle_create_challenge(
        &self,
        state: &SharedState,
        account_id: &str,
        region: &str,
        body: &Value,
    ) -> MockResponse {
        let connector_arn = match require_str(body, "ConnectorArn") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let id = uuid::Uuid::new_v4().simple().to_string();
        let arn = format!("arn:aws:pca-connector-scep:{region}:{account_id}:challenge/{id}");
        let now = chrono::Utc::now().timestamp() as f64;
        let password = format!("scep-{}", uuid::Uuid::new_v4().simple());
        let tags = parse_object_string_map(body.get("Tags"));
        let record = ChallengeRecord {
            arn: arn.clone(),
            connector_arn,
            password: password.clone(),
            created_at: now,
            updated_at: now,
        };
        let mut state = state.write().await;
        match state.create_challenge(record) {
            Ok(c) => {
                let challenge_wire = wire::Challenge {
                    arn: Some(c.arn.clone()),
                    connector_arn: Some(c.connector_arn.clone()),
                    created_at: Some(c.created_at),
                    password: Some(c.password.clone()),
                    updated_at: Some(c.updated_at),
                };
                if !tags.is_empty() {
                    state.tag_resource(&arn, tags);
                }
                let mut resp =
                    wire::serialize_create_challenge_response(&wire::CreateChallengeResponse {
                        challenge: Some(challenge_wire),
                    });
                resp.status = 202;
                resp
            }
            Err(e) => err_response(&e),
        }
    }

    async fn handle_delete_challenge(&self, state: &SharedState, arn: &str) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_challenge(arn) {
            Ok(()) => {
                let mut resp = wire::serialize_delete_challenge_response();
                resp.status = 202;
                resp
            }
            Err(e) => err_response(&e),
        }
    }

    async fn handle_get_challenge_metadata(&self, state: &SharedState, arn: &str) -> MockResponse {
        let state = state.read().await;
        match state.get_challenge(arn) {
            Ok(c) => wire::serialize_get_challenge_metadata_response(
                &wire::GetChallengeMetadataResponse {
                    challenge_metadata: Some(wire::ChallengeMetadata {
                        arn: Some(c.arn.clone()),
                        connector_arn: Some(c.connector_arn.clone()),
                        created_at: Some(c.created_at),
                        updated_at: Some(c.updated_at),
                    }),
                },
            ),
            Err(e) => err_response(&e),
        }
    }

    async fn handle_get_challenge_password(&self, state: &SharedState, arn: &str) -> MockResponse {
        let state = state.read().await;
        match state.get_challenge(arn) {
            Ok(c) => wire::serialize_get_challenge_password_response(
                &wire::GetChallengePasswordResponse {
                    password: Some(c.password.clone()),
                },
            ),
            Err(e) => err_response(&e),
        }
    }

    async fn handle_list_challenge_metadata(&self, state: &SharedState, uri: &str) -> MockResponse {
        let qs = winterbaume_core::parse_query_string(extract_query_string(uri));
        let connector_arn = qs.get("ConnectorArn").map(String::as_str);
        let state = state.read().await;
        let challenges: Vec<wire::ChallengeMetadataSummary> = state
            .list_challenges(connector_arn)
            .into_iter()
            .map(|c| wire::ChallengeMetadataSummary {
                arn: Some(c.arn.clone()),
                connector_arn: Some(c.connector_arn.clone()),
                created_at: Some(c.created_at),
                updated_at: Some(c.updated_at),
            })
            .collect();
        wire::serialize_list_challenge_metadata_response(&wire::ListChallengeMetadataResponse {
            challenges: Some(challenges),
            next_token: None,
        })
    }

    async fn handle_list_tags(&self, state: &SharedState, arn: &str) -> MockResponse {
        let state = state.read().await;
        let tags = state.list_tags(arn);
        wire::serialize_list_tags_for_resource_response(&wire::ListTagsForResourceResponse {
            tags: if tags.is_empty() { None } else { Some(tags) },
        })
    }

    async fn handle_tag_resource(
        &self,
        state: &SharedState,
        arn: &str,
        body: &Value,
    ) -> MockResponse {
        let tags = parse_object_string_map(body.get("Tags"));
        if tags.is_empty() {
            return rest_json_error(400, "ValidationException", "Tags is required");
        }
        let mut state = state.write().await;
        state.tag_resource(arn, tags);
        wire::serialize_tag_resource_response()
    }

    async fn handle_untag_resource(
        &self,
        state: &SharedState,
        arn: &str,
        uri: &str,
    ) -> MockResponse {
        let keys: Vec<String> = extract_query_string(uri)
            .split('&')
            .filter_map(|pair| pair.split_once('='))
            .filter(|(k, _)| *k == "TagKeys")
            .map(|(_, v)| percent_decode(v))
            .collect();
        let mut state = state.write().await;
        state.untag_resource(arn, &keys);
        wire::serialize_untag_resource_response()
    }
}

fn connector_to_wire(c: &ConnectorRecord) -> wire::Connector {
    wire::Connector {
        arn: Some(c.arn.clone()),
        certificate_authority_arn: Some(c.certificate_authority_arn.clone()),
        created_at: Some(c.created_at),
        endpoint: Some(c.endpoint.clone()),
        mobile_device_management: None,
        open_id_configuration: None,
        status: Some(c.status.clone()),
        status_reason: None,
        r#type: Some(c.connector_type.clone()),
        updated_at: Some(c.updated_at),
    }
}

fn require_str(body: &Value, field: &str) -> Result<String, MockResponse> {
    body.get(field)
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .map(String::from)
        .ok_or_else(|| rest_json_error(400, "ValidationException", &format!("{field} is required")))
}

fn parse_object_string_map(v: Option<&Value>) -> HashMap<String, String> {
    v.and_then(|v| v.as_object())
        .map(|m| {
            m.iter()
                .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                .collect()
        })
        .unwrap_or_default()
}

fn err_response(err: &PcaConnectorScepError) -> MockResponse {
    let (status, error_type) = match err {
        PcaConnectorScepError::ConnectorNotFound { .. }
        | PcaConnectorScepError::ChallengeNotFound { .. } => (404, "ResourceNotFoundException"),
        PcaConnectorScepError::Validation { .. } => (400, "ValidationException"),
    };
    let body = json!({"Message": err.to_string()});
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}
