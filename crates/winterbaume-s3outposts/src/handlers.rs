use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
    extract_path, extract_query_string, parse_query_string, rest_json_error,
};

use crate::state::{EndpointRecord, S3OutpostsError, S3OutpostsState};
use crate::views::S3OutpostsStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct S3OutpostsService {
    pub(crate) state: Arc<BackendState<S3OutpostsState>>,
    pub(crate) notifier: StateChangeNotifier<S3OutpostsStateView>,
}

impl S3OutpostsService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for S3OutpostsService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for S3OutpostsService {
    fn service_name(&self) -> &str {
        "s3-outposts"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://s3-outposts\..*\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

type SharedState = Arc<tokio::sync::RwLock<S3OutpostsState>>;

impl S3OutpostsService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let path = path.trim_start_matches('/');
        let path = path.trim_start_matches("S3Outposts/");
        let path = path.trim_end_matches('/');
        let method = request.method.as_str().to_uppercase();

        let body: Value = if request.body.is_empty() {
            json!({})
        } else {
            match serde_json::from_slice(&request.body) {
                Ok(v) => v,
                Err(_) => return rest_json_error(400, "BadRequestException", "Invalid JSON body"),
            }
        };

        let response = match (method.as_str(), path) {
            ("POST", "CreateEndpoint") => {
                self.handle_create_endpoint(&state, account_id, &region, &body)
                    .await
            }
            ("DELETE", "DeleteEndpoint") => self.handle_delete_endpoint(&state, &request.uri).await,
            ("GET", "ListEndpoints") => self.handle_list_endpoints(&state).await,
            ("GET", "ListOutpostsWithS3") => self.handle_list_outposts(&state, account_id).await,
            ("GET", "ListSharedEndpoints") => self.handle_list_shared_endpoints(&state).await,
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        };

        use winterbaume_core::StatefulService;
        if response.status / 100 == 2 && method != "GET" {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_create_endpoint(
        &self,
        state: &SharedState,
        account_id: &str,
        region: &str,
        body: &Value,
    ) -> MockResponse {
        let outpost_id = match require_str(body, "OutpostId") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let subnet_id = match require_str(body, "SubnetId") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let security_group_id = match require_str(body, "SecurityGroupId") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let access_type = body
            .get("AccessType")
            .and_then(|v| v.as_str())
            .unwrap_or("Private")
            .to_string();
        let customer_owned_ipv4_pool = body
            .get("CustomerOwnedIpv4Pool")
            .and_then(|v| v.as_str())
            .map(String::from);
        let id = uuid::Uuid::new_v4().simple().to_string();
        let arn =
            format!("arn:aws:s3-outposts:{region}:{account_id}:outpost/{outpost_id}/endpoint/{id}");
        let endpoint = EndpointRecord {
            arn: arn.clone(),
            outpost_id,
            subnet_id,
            security_group_id,
            access_type,
            customer_owned_ipv4_pool,
            vpc_id: format!("vpc-{}", &id[..8]),
            cidr_block: "10.0.0.0/24".to_string(),
            creation_time: chrono::Utc::now().timestamp() as f64,
            status: "Available".to_string(),
            network_interface_ids: vec![format!("eni-{}", &id[..8])],
        };
        let mut state = state.write().await;
        state.create_endpoint(endpoint);
        wire::serialize_create_endpoint_response(&wire::CreateEndpointResult {
            endpoint_arn: Some(arn),
        })
    }

    async fn handle_delete_endpoint(&self, state: &SharedState, uri: &str) -> MockResponse {
        let qs = parse_query_string(extract_query_string(uri));
        let endpoint_id = qs
            .iter()
            .find(|(k, _)| k.as_str() == "endpointId")
            .map(|(_, v)| v.clone());
        let outpost_id = qs
            .iter()
            .find(|(k, _)| k.as_str() == "outpostId")
            .map(|(_, v)| v.clone());
        let endpoint_id = match endpoint_id {
            Some(v) if !v.is_empty() => v,
            _ => {
                return rest_json_error(
                    400,
                    "ValidationException",
                    "endpointId query parameter is required",
                );
            }
        };
        let outpost_id = match outpost_id {
            Some(v) if !v.is_empty() => v,
            _ => {
                return rest_json_error(
                    400,
                    "ValidationException",
                    "outpostId query parameter is required",
                );
            }
        };
        let mut state = state.write().await;
        // Endpoints are stored by ARN; resolve from outpost_id + endpoint_id.
        let arn = state
            .list_endpoints()
            .iter()
            .find(|e| {
                e.outpost_id == outpost_id && e.arn.ends_with(&format!("/endpoint/{endpoint_id}"))
            })
            .map(|e| e.arn.clone());
        let arn = match arn {
            Some(a) => a,
            None => {
                return err_response(&S3OutpostsError::EndpointNotFound {
                    arn: format!("{outpost_id}/{endpoint_id}"),
                });
            }
        };
        match state.delete_endpoint(&arn) {
            Ok(()) => wire::serialize_delete_endpoint_response(),
            Err(e) => err_response(&e),
        }
    }

    async fn handle_list_endpoints(&self, state: &SharedState) -> MockResponse {
        let state = state.read().await;
        let endpoints: Vec<wire::Endpoint> = state
            .list_endpoints()
            .into_iter()
            .map(endpoint_to_wire)
            .collect();
        wire::serialize_list_endpoints_response(&wire::ListEndpointsResult {
            endpoints: Some(endpoints),
            next_token: None,
        })
    }

    // STUB[org-integration]: ListSharedEndpoints requires Resource Access Manager cross-account
    //   sharing state that cannot exist in a single-account mock; always returns empty.
    async fn handle_list_shared_endpoints(&self, _state: &SharedState) -> MockResponse {
        wire::serialize_list_shared_endpoints_response(&wire::ListSharedEndpointsResult {
            endpoints: Some(vec![]),
            next_token: None,
        })
    }

    async fn handle_list_outposts(&self, state: &SharedState, account_id: &str) -> MockResponse {
        let mut state = state.write().await;
        state.ensure_default_outposts(account_id);
        let outposts: Vec<wire::Outpost> = state
            .list_outposts()
            .iter()
            .map(|o| wire::Outpost {
                capacity_in_bytes: Some(o.capacity_in_bytes),
                outpost_arn: Some(o.outpost_arn.clone()),
                outpost_id: Some(o.outpost_id.clone()),
                owner_id: Some(o.owner_id.clone()),
                s3_outpost_arn: Some(o.s3_outpost_arn.clone()),
            })
            .collect();
        wire::serialize_list_outposts_with_s3_response(&wire::ListOutpostsWithS3Result {
            next_token: None,
            outposts: Some(outposts),
        })
    }
}

fn endpoint_to_wire(e: &EndpointRecord) -> wire::Endpoint {
    wire::Endpoint {
        access_type: Some(e.access_type.clone()),
        cidr_block: Some(e.cidr_block.clone()),
        creation_time: Some(e.creation_time),
        customer_owned_ipv4_pool: e.customer_owned_ipv4_pool.clone(),
        endpoint_arn: Some(e.arn.clone()),
        failed_reason: None,
        network_interfaces: Some(
            e.network_interface_ids
                .iter()
                .map(|id| wire::NetworkInterface {
                    network_interface_id: Some(id.clone()),
                })
                .collect(),
        ),
        outposts_id: Some(e.outpost_id.clone()),
        security_group_id: Some(e.security_group_id.clone()),
        status: Some(e.status.clone()),
        subnet_id: Some(e.subnet_id.clone()),
        vpc_id: Some(e.vpc_id.clone()),
    }
}

fn require_str(body: &Value, field: &str) -> Result<String, MockResponse> {
    body.get(field)
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .map(String::from)
        .ok_or_else(|| rest_json_error(400, "ValidationException", &format!("{field} is required")))
}

fn err_response(err: &S3OutpostsError) -> MockResponse {
    let (status, error_type) = match err {
        S3OutpostsError::EndpointNotFound { .. } => (404, "ResourceNotFoundException"),
        S3OutpostsError::Validation { .. } => (400, "ValidationException"),
    };
    let body = json!({"Message": err.to_string()});
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}
