use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::Value;
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, default_account_id,
    extract_path, rest_json_error,
};

use crate::state::{PanoramaError, PanoramaState};
use crate::views::PanoramaStateView;
use crate::wire;

/// Convert an RFC 3339 timestamp string into the wire's `f64` epoch seconds.
fn rfc3339_to_epoch(s: &str) -> Option<f64> {
    chrono::DateTime::parse_from_rfc3339(s)
        .ok()
        .map(|dt| dt.timestamp() as f64)
}

pub struct PanoramaService {
    pub(crate) state: Arc<BackendState<PanoramaState>>,
    pub(crate) notifier: StateChangeNotifier<PanoramaStateView>,
}

impl PanoramaService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for PanoramaService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for PanoramaService {
    fn service_name(&self) -> &str {
        "panorama"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://panorama\..*\.amazonaws\.com",
            r"https?://panorama\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl PanoramaService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let method = request.method.as_str().to_uppercase();

        let body: Value = if request.body.is_empty() {
            serde_json::json!({})
        } else {
            match serde_json::from_slice(&request.body) {
                Ok(v) => v,
                Err(_) => {
                    return rest_json_error(400, "ValidationException", "Invalid JSON body");
                }
            }
        };

        // Route based on method + path pattern
        let response = self
            .route(&state, &method, &path, &body, account_id, &region)
            .await;

        use winterbaume_core::StatefulService;
        if response.status / 100 == 2 {
            if matches!(method.as_str(), "POST" | "PUT" | "DELETE" | "PATCH") {
                self.notify_state_changed(account_id, &region).await;
            }
        }
        response
    }

    async fn route(
        &self,
        state: &Arc<tokio::sync::RwLock<PanoramaState>>,
        method: &str,
        path: &str,
        body: &Value,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        // POST /application-instances -> CreateApplicationInstance
        if method == "POST" && path == "/application-instances" {
            return self
                .handle_create_application_instance(state, body, account_id, region)
                .await;
        }

        // GET /application-instances/{id} -> DescribeApplicationInstance
        if method == "GET" && path.starts_with("/application-instances/") {
            let id = path.trim_start_matches("/application-instances/");
            if !id.contains('/') {
                return self.handle_describe_application_instance(state, id).await;
            }
        }

        // GET /application-instances -> ListApplicationInstances
        if method == "GET" && path == "/application-instances" {
            return self.handle_list_application_instances(state).await;
        }

        // POST /packages/template-job -> CreateNodeFromTemplateJob
        if method == "POST" && path == "/packages/template-job" {
            return self.handle_create_node_from_template_job(state, body).await;
        }

        // GET /packages/template-job/{id} -> DescribeNodeFromTemplateJob
        if method == "GET" && path.starts_with("/packages/template-job/") {
            let job_id = path.trim_start_matches("/packages/template-job/");
            if !job_id.contains('/') {
                return self
                    .handle_describe_node_from_template_job(state, job_id)
                    .await;
            }
        }

        // GET /nodes -> ListNodes
        if method == "GET" && path == "/nodes" {
            return self.handle_list_nodes(state).await;
        }

        // POST /devices -> ProvisionDevice
        if method == "POST" && path == "/devices" {
            return self
                .handle_provision_device(state, body, account_id, region)
                .await;
        }

        // GET /devices/{id} -> DescribeDevice
        if method == "GET" && path.starts_with("/devices/") {
            let id = path.trim_start_matches("/devices/");
            if !id.contains('/') {
                return self.handle_describe_device(state, id).await;
            }
        }

        // DELETE /devices/{id} -> DeleteDevice
        if method == "DELETE" && path.starts_with("/devices/") {
            let id = path.trim_start_matches("/devices/");
            if !id.contains('/') {
                return self.handle_delete_device(state, id).await;
            }
        }

        // GET /devices -> ListDevices
        if method == "GET" && path == "/devices" {
            return self.handle_list_devices(state).await;
        }

        // PUT /devices/{id} -> UpdateDeviceMetadata
        if method == "PUT" && path.starts_with("/devices/") {
            let id = path.trim_start_matches("/devices/");
            if !id.contains('/') {
                return self.handle_update_device_metadata(state, id, body).await;
            }
        }

        rest_json_error(404, "UnknownOperationException", "Not found")
    }

    async fn handle_create_application_instance(
        &self,
        state: &Arc<tokio::sync::RwLock<PanoramaState>>,
        body: &Value,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let name = body
            .get("Name")
            .and_then(|v| v.as_str())
            .unwrap_or("unnamed")
            .to_string();
        let description = body
            .get("Description")
            .and_then(|v| v.as_str())
            .map(str::to_string);
        let default_runtime_context_device = body
            .get("DefaultRuntimeContextDevice")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let application_instance_id_to_replace = body
            .get("ApplicationInstanceIdToReplace")
            .and_then(|v| v.as_str())
            .map(str::to_string);
        let tags: HashMap<String, String> = body
            .get("Tags")
            .and_then(|v| serde_json::from_value(v.clone()).ok())
            .unwrap_or_default();

        let id = format!("applicationInstance-{}", uuid::Uuid::new_v4());
        let mut st = state.write().await;
        let app_id = st.create_application_instance(
            id,
            name,
            description,
            default_runtime_context_device,
            application_instance_id_to_replace,
            tags,
            account_id,
            region,
        );
        let resp = wire::CreateApplicationInstanceResponse {
            application_instance_id: Some(app_id),
            ..Default::default()
        };
        wire::serialize_create_application_instance_response(&resp)
    }

    async fn handle_describe_application_instance(
        &self,
        state: &Arc<tokio::sync::RwLock<PanoramaState>>,
        id: &str,
    ) -> MockResponse {
        let st = state.read().await;
        match st.describe_application_instance(id) {
            Ok(app) => {
                let resp = wire::DescribeApplicationInstanceResponse {
                    application_instance_id: Some(app.application_instance_id.clone()),
                    name: Some(app.name.clone()),
                    status: Some(app.status.clone()),
                    description: app.description.clone(),
                    default_runtime_context_device: Some(
                        app.default_runtime_context_device.clone(),
                    ),
                    application_instance_id_to_replace: app
                        .application_instance_id_to_replace
                        .clone(),
                    arn: Some(app.arn.clone()),
                    created_time: rfc3339_to_epoch(&app.created_time),
                    tags: if app.tags.is_empty() {
                        None
                    } else {
                        Some(app.tags.clone())
                    },
                    ..Default::default()
                };
                wire::serialize_describe_application_instance_response(&resp)
            }
            Err(e) => panorama_error_response(&e),
        }
    }

    async fn handle_list_application_instances(
        &self,
        state: &Arc<tokio::sync::RwLock<PanoramaState>>,
    ) -> MockResponse {
        let st = state.read().await;
        let items: Vec<wire::ApplicationInstance> = st
            .list_application_instances()
            .into_iter()
            .map(|app| wire::ApplicationInstance {
                application_instance_id: Some(app.application_instance_id.clone()),
                name: Some(app.name.clone()),
                status: Some(app.status.clone()),
                description: app.description.clone(),
                default_runtime_context_device: Some(app.default_runtime_context_device.clone()),
                arn: Some(app.arn.clone()),
                created_time: rfc3339_to_epoch(&app.created_time),
                tags: if app.tags.is_empty() {
                    None
                } else {
                    Some(app.tags.clone())
                },
                ..Default::default()
            })
            .collect();
        let resp = wire::ListApplicationInstancesResponse {
            application_instances: if items.is_empty() { None } else { Some(items) },
            ..Default::default()
        };
        wire::serialize_list_application_instances_response(&resp)
    }

    async fn handle_create_node_from_template_job(
        &self,
        state: &Arc<tokio::sync::RwLock<PanoramaState>>,
        body: &Value,
    ) -> MockResponse {
        let node_name = body
            .get("NodeName")
            .and_then(|v| v.as_str())
            .unwrap_or("unnamed-node")
            .to_string();
        let template_type = body
            .get("TemplateType")
            .and_then(|v| v.as_str())
            .unwrap_or("RTSP_CAMERA_STREAM")
            .to_string();
        let job_id = format!("nodeFromTemplateJob-{}", uuid::Uuid::new_v4());
        let mut st = state.write().await;
        let jid = st.create_node_from_template_job(job_id, node_name, template_type);
        let resp = wire::CreateNodeFromTemplateJobResponse {
            job_id: Some(jid),
            ..Default::default()
        };
        wire::serialize_create_node_from_template_job_response(&resp)
    }

    async fn handle_describe_node_from_template_job(
        &self,
        state: &Arc<tokio::sync::RwLock<PanoramaState>>,
        job_id: &str,
    ) -> MockResponse {
        let st = state.read().await;
        match st.describe_node_from_template_job(job_id) {
            Ok(job) => {
                let resp = wire::DescribeNodeFromTemplateJobResponse {
                    job_id: Some(job.job_id.clone()),
                    node_name: Some(job.node_name.clone()),
                    status: Some(job.status.clone()),
                    template_type: Some(job.template_type.clone()),
                    created_time: rfc3339_to_epoch(&job.created_time),
                    ..Default::default()
                };
                wire::serialize_describe_node_from_template_job_response(&resp)
            }
            Err(e) => panorama_error_response(&e),
        }
    }

    // STUB[no-state]: ListNodes returns package-marketplace nodes; the mock has no node catalogue.
    async fn handle_list_nodes(
        &self,
        _state: &Arc<tokio::sync::RwLock<PanoramaState>>,
    ) -> MockResponse {
        let resp = wire::ListNodesResponse {
            nodes: None,
            ..Default::default()
        };
        wire::serialize_list_nodes_response(&resp)
    }

    async fn handle_provision_device(
        &self,
        state: &Arc<tokio::sync::RwLock<PanoramaState>>,
        body: &Value,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let name = body
            .get("Name")
            .and_then(|v| v.as_str())
            .unwrap_or("unnamed-device")
            .to_string();
        let description = body
            .get("Description")
            .and_then(|v| v.as_str())
            .map(str::to_string);
        let tags: HashMap<String, String> = body
            .get("Tags")
            .and_then(|v| serde_json::from_value(v.clone()).ok())
            .unwrap_or_default();

        let device_id = format!("device-{}", uuid::Uuid::new_v4());
        let mut st = state.write().await;
        let device = st.provision_device(device_id, name, description, tags, account_id, region);
        let resp = wire::ProvisionDeviceResponse {
            device_id: Some(device.device_id.clone()),
            arn: Some(device.arn.clone()),
            status: Some(device.status.clone()),
            ..Default::default()
        };
        wire::serialize_provision_device_response(&resp)
    }

    async fn handle_describe_device(
        &self,
        state: &Arc<tokio::sync::RwLock<PanoramaState>>,
        id: &str,
    ) -> MockResponse {
        let st = state.read().await;
        match st.describe_device(id) {
            Ok(device) => {
                let resp = wire::DescribeDeviceResponse {
                    device_id: Some(device.device_id.clone()),
                    name: Some(device.name.clone()),
                    arn: Some(device.arn.clone()),
                    provisioning_status: Some(device.status.clone()),
                    description: device.description.clone(),
                    created_time: rfc3339_to_epoch(&device.created_time),
                    tags: if device.tags.is_empty() {
                        None
                    } else {
                        Some(device.tags.clone())
                    },
                    ..Default::default()
                };
                wire::serialize_describe_device_response(&resp)
            }
            Err(e) => panorama_error_response(&e),
        }
    }

    async fn handle_delete_device(
        &self,
        state: &Arc<tokio::sync::RwLock<PanoramaState>>,
        id: &str,
    ) -> MockResponse {
        let mut st = state.write().await;
        match st.delete_device(id) {
            Ok(device_id) => {
                let resp = wire::DeleteDeviceResponse {
                    device_id: Some(device_id),
                    ..Default::default()
                };
                wire::serialize_delete_device_response(&resp)
            }
            Err(e) => panorama_error_response(&e),
        }
    }

    async fn handle_list_devices(
        &self,
        state: &Arc<tokio::sync::RwLock<PanoramaState>>,
    ) -> MockResponse {
        let st = state.read().await;
        let items: Vec<wire::Device> = st
            .list_devices()
            .into_iter()
            .map(|d| wire::Device {
                device_id: Some(d.device_id.clone()),
                name: Some(d.name.clone()),
                description: d.description.clone(),
                created_time: rfc3339_to_epoch(&d.created_time),
                ..Default::default()
            })
            .collect();
        let resp = wire::ListDevicesResponse {
            devices: if items.is_empty() { None } else { Some(items) },
            ..Default::default()
        };
        wire::serialize_list_devices_response(&resp)
    }

    async fn handle_update_device_metadata(
        &self,
        state: &Arc<tokio::sync::RwLock<PanoramaState>>,
        id: &str,
        body: &Value,
    ) -> MockResponse {
        let description = body
            .get("Description")
            .and_then(|v| v.as_str())
            .map(str::to_string);
        let mut st = state.write().await;
        match st.update_device_metadata(id, description) {
            Ok(device_id) => {
                let resp = wire::UpdateDeviceMetadataResponse {
                    device_id: Some(device_id),
                    ..Default::default()
                };
                wire::serialize_update_device_metadata_response(&resp)
            }
            Err(e) => panorama_error_response(&e),
        }
    }
}

fn panorama_error_response(err: &PanoramaError) -> MockResponse {
    use http::header::HeaderName;
    const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");
    let (status, error_type) = match err {
        PanoramaError::ApplicationInstanceNotFound(_) => (404u16, "ResourceNotFoundException"),
        PanoramaError::DeviceNotFound(_) => (404u16, "ResourceNotFoundException"),
        PanoramaError::NodeFromTemplateJobNotFound(_) => (404u16, "ResourceNotFoundException"),
    };
    let body = serde_json::json!({ "message": err.to_string() });
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}
