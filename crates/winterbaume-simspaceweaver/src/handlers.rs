use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, default_account_id,
    extract_path, extract_query_string, parse_query_string, percent_decode, rest_json_error,
};

use crate::state::{AppRecord, SimSpaceWeaverError, SimSpaceWeaverState, SimulationRecord};
use crate::views::SimSpaceWeaverStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct SimSpaceWeaverService {
    pub(crate) state: Arc<BackendState<SimSpaceWeaverState>>,
    pub(crate) notifier: StateChangeNotifier<SimSpaceWeaverStateView>,
}

impl SimSpaceWeaverService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for SimSpaceWeaverService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for SimSpaceWeaverService {
    fn service_name(&self) -> &str {
        "simspaceweaver"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://simspaceweaver\..*\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

type SharedState = Arc<tokio::sync::RwLock<SimSpaceWeaverState>>;

impl SimSpaceWeaverService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();
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
        let qs = parse_query_string(extract_query_string(&request.uri));

        let response = match (method.as_str(), segs.as_slice()) {
            ("POST", ["createsnapshot"]) => self.handle_create_snapshot(&state, &body).await,
            ("DELETE", ["deleteapp"]) => self.handle_delete_app(&state, &qs).await,
            ("DELETE", ["deletesimulation"]) => self.handle_delete_simulation(&state, &qs).await,
            ("GET", ["describeapp"]) => self.handle_describe_app(&state, &qs).await,
            ("GET", ["describesimulation"]) => self.handle_describe_simulation(&state, &qs).await,
            ("GET", ["listapps"]) => self.handle_list_apps(&state, &qs).await,
            ("GET", ["listsimulations"]) => self.handle_list_simulations(&state).await,
            ("POST", ["startapp"]) => self.handle_start_app(&state, &body).await,
            ("POST", ["startclock"]) => self.handle_start_clock(&state, &body).await,
            ("POST", ["startsimulation"]) => {
                self.handle_start_simulation(&state, account_id, &region, &body)
                    .await
            }
            ("POST", ["stopapp"]) => self.handle_stop_app(&state, &body).await,
            ("POST", ["stopclock"]) => self.handle_stop_clock(&state, &body).await,
            ("POST", ["stopsimulation"]) => self.handle_stop_simulation(&state, &body).await,
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

    async fn handle_create_snapshot(&self, state: &SharedState, body: &Value) -> MockResponse {
        let simulation = match require_str(body, "Simulation") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let _destination = body.get("Destination").cloned().unwrap_or(json!({}));
        let state = state.read().await;
        if state.get_simulation(&simulation).is_err() {
            return err_response(&SimSpaceWeaverError::SimulationNotFound { name: simulation });
        }
        wire::serialize_create_snapshot_response(&wire::CreateSnapshotOutput {})
    }

    async fn handle_describe_simulation(
        &self,
        state: &SharedState,
        qs: &std::collections::HashMap<String, String>,
    ) -> MockResponse {
        let simulation = match require_qs(qs, "simulation") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let state = state.read().await;
        match state.get_simulation(&simulation) {
            Ok(s) => {
                wire::serialize_describe_simulation_response(&wire::DescribeSimulationOutput {
                    arn: Some(s.arn.clone()),
                    creation_time: Some(s.creation_time),
                    description: s.description.clone(),
                    execution_id: Some(s.execution_id.clone()),
                    live_simulation_state: Some(wire::LiveSimulationState {
                        clocks: Some(vec![wire::SimulationClock {
                            status: Some(s.clock_status.clone()),
                            target_status: Some(s.clock_status.clone()),
                        }]),
                        domains: Some(vec![]),
                    }),
                    logging_configuration: None,
                    maximum_duration: s.maximum_duration.clone(),
                    name: Some(s.name.clone()),
                    role_arn: Some(s.role_arn.clone()),
                    schema_error: None,
                    schema_s3_location: None,
                    snapshot_s3_location: None,
                    start_error: None,
                    status: Some(s.status.clone()),
                    target_status: Some(s.target_status.clone()),
                })
            }
            Err(e) => err_response(&e),
        }
    }

    async fn handle_list_simulations(&self, state: &SharedState) -> MockResponse {
        let state = state.read().await;
        let metadata: Vec<wire::SimulationMetadata> = state
            .list_simulations()
            .into_iter()
            .map(|s| wire::SimulationMetadata {
                arn: Some(s.arn.clone()),
                creation_time: Some(s.creation_time),
                name: Some(s.name.clone()),
                status: Some(s.status.clone()),
                target_status: Some(s.target_status.clone()),
            })
            .collect();
        wire::serialize_list_simulations_response(&wire::ListSimulationsOutput {
            next_token: None,
            simulations: Some(metadata),
        })
    }

    async fn handle_start_simulation(
        &self,
        state: &SharedState,
        account_id: &str,
        region: &str,
        body: &Value,
    ) -> MockResponse {
        let name = match require_str(body, "Name") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let role_arn = match require_str(body, "RoleArn") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let arn = format!("arn:aws:simspaceweaver:{region}:{account_id}:simulation/{name}");
        let now = chrono::Utc::now().timestamp() as f64;
        let execution_id = format!("exec-{}", uuid::Uuid::new_v4().simple());
        let tags = parse_object_string_map(body.get("Tags"));
        let sim = SimulationRecord {
            name: name.clone(),
            arn: arn.clone(),
            execution_id: execution_id.clone(),
            description: body
                .get("Description")
                .and_then(|v| v.as_str())
                .map(String::from),
            role_arn,
            status: "STARTING".to_string(),
            target_status: "STARTED".to_string(),
            creation_time: now,
            maximum_duration: body
                .get("MaximumDuration")
                .and_then(|v| v.as_str())
                .map(String::from),
            clock_status: "UNKNOWN".to_string(),
            tags,
        };
        let mut state = state.write().await;
        match state.create_simulation(sim) {
            Ok(_) => wire::serialize_start_simulation_response(&wire::StartSimulationOutput {
                arn: Some(arn),
                creation_time: Some(now),
                execution_id: Some(execution_id),
            }),
            Err(e) => err_response(&e),
        }
    }

    async fn handle_stop_simulation(&self, state: &SharedState, body: &Value) -> MockResponse {
        let simulation = match require_str(body, "Simulation") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let mut state = state.write().await;
        match state.set_simulation_status(&simulation, "STOPPED", "STOPPING") {
            Ok(()) => wire::serialize_stop_simulation_response(&wire::StopSimulationOutput {}),
            Err(e) => err_response(&e),
        }
    }

    async fn handle_delete_simulation(
        &self,
        state: &SharedState,
        qs: &std::collections::HashMap<String, String>,
    ) -> MockResponse {
        let simulation = match require_qs(qs, "simulation") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let mut state = state.write().await;
        match state.delete_simulation(&simulation) {
            Ok(()) => wire::serialize_delete_simulation_response(&wire::DeleteSimulationOutput {}),
            Err(e) => err_response(&e),
        }
    }

    async fn handle_describe_app(
        &self,
        state: &SharedState,
        qs: &std::collections::HashMap<String, String>,
    ) -> MockResponse {
        let simulation = match require_qs(qs, "simulation") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let domain = match require_qs(qs, "domain") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let app = match require_qs(qs, "app") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let state = state.read().await;
        match state.get_app(&simulation, &domain, &app) {
            Ok(a) => wire::serialize_describe_app_response(&wire::DescribeAppOutput {
                description: a.description.clone(),
                domain: Some(a.domain.clone()),
                endpoint_info: None,
                launch_overrides: None,
                name: Some(a.name.clone()),
                simulation: Some(a.simulation.clone()),
                status: Some(a.status.clone()),
                target_status: Some(a.target_status.clone()),
            }),
            Err(e) => err_response(&e),
        }
    }

    async fn handle_list_apps(
        &self,
        state: &SharedState,
        qs: &std::collections::HashMap<String, String>,
    ) -> MockResponse {
        let simulation = match require_qs(qs, "simulation") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let state = state.read().await;
        let apps: Vec<wire::SimulationAppMetadata> = state
            .list_apps(&simulation)
            .into_iter()
            .map(|a| wire::SimulationAppMetadata {
                domain: Some(a.domain.clone()),
                name: Some(a.name.clone()),
                simulation: Some(a.simulation.clone()),
                status: Some(a.status.clone()),
                target_status: Some(a.target_status.clone()),
            })
            .collect();
        wire::serialize_list_apps_response(&wire::ListAppsOutput {
            apps: Some(apps),
            next_token: None,
        })
    }

    async fn handle_start_app(&self, state: &SharedState, body: &Value) -> MockResponse {
        let simulation = match require_str(body, "Simulation") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let domain = match require_str(body, "Domain") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let name = match require_str(body, "Name") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let app = AppRecord {
            simulation: simulation.clone(),
            domain: domain.clone(),
            name: name.clone(),
            description: body
                .get("Description")
                .and_then(|v| v.as_str())
                .map(String::from),
            status: "STARTING".to_string(),
            target_status: "STARTED".to_string(),
        };
        let mut state = state.write().await;
        if state.get_simulation(&simulation).is_err() {
            return err_response(&SimSpaceWeaverError::SimulationNotFound { name: simulation });
        }
        state.upsert_app(app);
        wire::serialize_start_app_response(&wire::StartAppOutput {
            domain: Some(domain),
            name: Some(name),
            simulation: Some(simulation),
        })
    }

    async fn handle_stop_app(&self, state: &SharedState, body: &Value) -> MockResponse {
        let simulation = match require_str(body, "Simulation") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let domain = match require_str(body, "Domain") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let app = match require_str(body, "App") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let mut state = state.write().await;
        match state.set_app_status(&simulation, &domain, &app, "STOPPED", "STOPPING") {
            Ok(()) => wire::serialize_stop_app_response(&wire::StopAppOutput {}),
            Err(e) => err_response(&e),
        }
    }

    async fn handle_delete_app(
        &self,
        state: &SharedState,
        qs: &std::collections::HashMap<String, String>,
    ) -> MockResponse {
        let simulation = match require_qs(qs, "simulation") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let domain = match require_qs(qs, "domain") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let app = match require_qs(qs, "app") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let mut state = state.write().await;
        match state.delete_app(&simulation, &domain, &app) {
            Ok(()) => wire::serialize_delete_app_response(&wire::DeleteAppOutput {}),
            Err(e) => err_response(&e),
        }
    }

    async fn handle_start_clock(&self, state: &SharedState, body: &Value) -> MockResponse {
        let simulation = match require_str(body, "Simulation") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let mut state = state.write().await;
        match state.set_clock_status(&simulation, "STARTED") {
            Ok(()) => wire::serialize_start_clock_response(&wire::StartClockOutput {}),
            Err(e) => err_response(&e),
        }
    }

    async fn handle_stop_clock(&self, state: &SharedState, body: &Value) -> MockResponse {
        let simulation = match require_str(body, "Simulation") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let mut state = state.write().await;
        match state.set_clock_status(&simulation, "STOPPED") {
            Ok(()) => wire::serialize_stop_clock_response(&wire::StopClockOutput {}),
            Err(e) => err_response(&e),
        }
    }

    async fn handle_list_tags(&self, state: &SharedState, arn: &str) -> MockResponse {
        let state = state.read().await;
        let tags_map = state.list_tags(arn);
        wire::serialize_list_tags_for_resource_response(&wire::ListTagsForResourceOutput {
            tags: if tags_map.is_empty() {
                None
            } else {
                Some(tags_map)
            },
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
        wire::serialize_tag_resource_response(&wire::TagResourceOutput {})
    }

    async fn handle_untag_resource(
        &self,
        state: &SharedState,
        arn: &str,
        uri: &str,
    ) -> MockResponse {
        // Use the raw URI rather than the deduplicated query map so that
        // multiple `tagKeys=...` values are preserved.
        let keys: Vec<String> = extract_query_string(uri)
            .split('&')
            .filter_map(|pair| pair.split_once('='))
            .filter(|(k, _)| *k == "tagKeys")
            .map(|(_, v)| winterbaume_core::percent_decode(v))
            .collect();
        let mut state = state.write().await;
        state.untag_resource(arn, &keys);
        wire::serialize_untag_resource_response(&wire::UntagResourceOutput {})
    }
}

fn require_str(body: &Value, field: &str) -> Result<String, MockResponse> {
    body.get(field)
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .map(String::from)
        .ok_or_else(|| rest_json_error(400, "ValidationException", &format!("{field} is required")))
}

fn require_qs(
    qs: &std::collections::HashMap<String, String>,
    key: &str,
) -> Result<String, MockResponse> {
    qs.get(key)
        .filter(|v| !v.is_empty())
        .cloned()
        .ok_or_else(|| {
            rest_json_error(
                400,
                "ValidationException",
                &format!("{key} query parameter is required"),
            )
        })
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

fn err_response(err: &SimSpaceWeaverError) -> MockResponse {
    let (status, error_type) = match err {
        SimSpaceWeaverError::SimulationNotFound { .. }
        | SimSpaceWeaverError::AppNotFound { .. } => (404, "ResourceNotFoundException"),
        SimSpaceWeaverError::SimulationAlreadyExists { .. } => (409, "ConflictException"),
        SimSpaceWeaverError::Validation { .. } => (400, "ValidationException"),
    };
    let body = json!({"Message": err.to_string()});
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}
