use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
    urldecode,
};

use crate::state::{ConnectCampaignsError, ConnectCampaignsState};
use crate::types::Campaign;
use crate::views::ConnectCampaignsStateView;
use crate::wire;

fn extract_query_params(uri: &str) -> HashMap<String, Vec<String>> {
    let mut params: HashMap<String, Vec<String>> = HashMap::new();
    if let Some(q) = uri.find('?') {
        let query = &uri[q + 1..];
        for pair in query.split('&') {
            let mut parts = pair.splitn(2, '=');
            if let Some(key) = parts.next() {
                let value = parts.next().unwrap_or("").to_string();
                params.entry(key.to_string()).or_default().push(value);
            }
        }
    }
    params
}

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct ConnectCampaignsService {
    pub(crate) state: Arc<BackendState<ConnectCampaignsState>>,
    pub(crate) notifier: StateChangeNotifier<ConnectCampaignsStateView>,
}

impl ConnectCampaignsService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for ConnectCampaignsService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for ConnectCampaignsService {
    fn service_name(&self) -> &str {
        "connect-campaigns"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://connect-campaigns\..*\.amazonaws\.com",
            r"https?://connect-campaigns\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl ConnectCampaignsService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let raw_query = match request.uri.find('?') {
            Some(idx) => request.uri[idx + 1..].to_string(),
            None => String::new(),
        };
        let query_map: HashMap<String, String> = winterbaume_core::parse_query_string(&raw_query);
        let method = request.method.as_str();

        let segments: Vec<&str> = path
            .trim_start_matches('/')
            .split('/')
            .filter(|s| !s.is_empty())
            .collect();

        use winterbaume_core::StatefulService;
        let response = match (method, segments.as_slice()) {
            // PUT /campaigns - CreateCampaign
            ("PUT", ["campaigns"]) => {
                self.handle_create_campaign(&state, &request, &[], &query_map, account_id, &region)
                    .await
            }
            // GET /campaigns/{id} - DescribeCampaign
            ("GET", ["campaigns", id]) => self.handle_describe_campaign(&state, id).await,
            // DELETE /campaigns/{id} - DeleteCampaign
            ("DELETE", ["campaigns", id]) => self.handle_delete_campaign(&state, id).await,
            // POST /campaigns-summary - ListCampaigns
            ("POST", ["campaigns-summary"]) => self.handle_list_campaigns(&state).await,
            // GET /campaigns/{id}/state - GetCampaignState
            ("GET", ["campaigns", id, "state"]) => self.handle_get_campaign_state(&state, id).await,
            // POST /campaigns/{id}/pause - PauseCampaign
            ("POST", ["campaigns", id, "pause"]) => self.handle_pause_campaign(&state, id).await,
            // POST /campaigns/{id}/resume - ResumeCampaign
            ("POST", ["campaigns", id, "resume"]) => self.handle_resume_campaign(&state, id).await,
            // POST /campaigns/{id}/start - StartCampaign
            ("POST", ["campaigns", id, "start"]) => self.handle_start_campaign(&state, id).await,
            // POST /campaigns/{id}/stop - StopCampaign
            ("POST", ["campaigns", id, "stop"]) => self.handle_stop_campaign(&state, id).await,
            // GET /connect-instance/{connectInstanceId}/config - GetConnectInstanceConfig
            ("GET", ["connect-instance", instance_id, "config"]) => {
                self.handle_get_connect_instance_config(&state, instance_id)
                    .await
            }
            // PUT /connect-instance/{connectInstanceId}/onboarding - StartInstanceOnboardingJob
            ("PUT", ["connect-instance", instance_id, "onboarding"]) => {
                let labels: &[(&str, &str)] = &[("connectInstanceId", instance_id)];
                self.handle_start_instance_onboarding_job(
                    &state, &request, labels, &query_map, account_id, &region,
                )
                .await
            }
            // GET /tags/{arn+} - ListTagsForResource
            ("GET", ["tags", arn_parts @ ..]) if !arn_parts.is_empty() => {
                let arn = urldecode(&arn_parts.join("/"));
                self.handle_list_tags_for_resource(&state, &arn).await
            }
            // POST /tags/{arn+} - TagResource
            ("POST", ["tags", arn_parts @ ..]) if !arn_parts.is_empty() => {
                let arn = urldecode(&arn_parts.join("/"));
                let labels: &[(&str, &str)] = &[("arn", arn.as_str())];
                self.handle_tag_resource(&state, &request, labels, &query_map)
                    .await
            }
            // DELETE /tags/{arn+} - UntagResource
            ("DELETE", ["tags", arn_parts @ ..]) if !arn_parts.is_empty() => {
                let arn = urldecode(&arn_parts.join("/"));
                let query_params = extract_query_params(&request.uri);
                let tag_keys: Vec<String> =
                    query_params.get("tagKeys").cloned().unwrap_or_default();
                self.handle_untag_resource(&state, &arn, &tag_keys).await
            }
            // --- Unimplemented operations ---
            // DELETE /connect-instance/{connectInstanceId}/config => DeleteConnectInstanceConfig (not implemented)
            // DELETE /connect-instance/{connectInstanceId}/onboarding => DeleteInstanceOnboardingJob (not implemented)
            // POST /campaigns-state => GetCampaignStateBatch (not implemented)
            // GET /connect-instance/{connectInstanceId}/onboarding => GetInstanceOnboardingJobStatus (not implemented)
            // PUT /campaigns/{id}/dial-requests => PutDialRequestBatch (not implemented)
            // POST /campaigns/{id}/dialer-config => UpdateCampaignDialerConfig (not implemented)
            // POST /campaigns/{id}/name => UpdateCampaignName (not implemented)
            // POST /campaigns/{id}/outbound-call-config => UpdateCampaignOutboundCallConfig (not implemented)
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        };
        if response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_create_campaign(
        &self,
        state: &Arc<tokio::sync::RwLock<ConnectCampaignsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_campaign_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };

        let dialer_config = serde_json::to_value(&input.dialer_config).unwrap_or(json!({}));
        let outbound_call_config =
            serde_json::to_value(&input.outbound_call_config).unwrap_or(json!({}));
        let tags = input.tags.clone().unwrap_or_default();

        let mut state = state.write().await;
        match state.create_campaign(
            &input.name,
            &input.connect_instance_id,
            dialer_config,
            outbound_call_config,
            tags,
            account_id,
            region,
        ) {
            Ok(campaign) => {
                let resp = wire::CreateCampaignResponse {
                    id: Some(campaign.id.clone()),
                    arn: Some(campaign.arn.clone()),
                    tags: if campaign.tags.is_empty() {
                        None
                    } else {
                        Some(campaign.tags.clone())
                    },
                };
                wire::serialize_create_campaign_response(&resp)
            }
            Err(e) => connectcampaigns_error_response(&e),
        }
    }

    async fn handle_describe_campaign(
        &self,
        state: &Arc<tokio::sync::RwLock<ConnectCampaignsState>>,
        id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_campaign(id) {
            Ok(campaign) => {
                let resp = wire::DescribeCampaignResponse {
                    campaign: Some(campaign_to_wire(campaign)),
                };
                wire::serialize_describe_campaign_response(&resp)
            }
            Err(e) => connectcampaigns_error_response(&e),
        }
    }

    async fn handle_delete_campaign(
        &self,
        state: &Arc<tokio::sync::RwLock<ConnectCampaignsState>>,
        id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_campaign(id) {
            Ok(()) => wire::serialize_delete_campaign_response(),
            Err(e) => connectcampaigns_error_response(&e),
        }
    }

    async fn handle_list_campaigns(
        &self,
        state: &Arc<tokio::sync::RwLock<ConnectCampaignsState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let campaigns = state.list_campaigns();
        let summaries: Vec<wire::CampaignSummary> = campaigns
            .iter()
            .map(|c| wire::CampaignSummary {
                id: Some(c.id.clone()),
                arn: Some(c.arn.clone()),
                name: Some(c.name.clone()),
                connect_instance_id: Some(c.connect_instance_id.clone()),
            })
            .collect();
        let resp = wire::ListCampaignsResponse {
            campaign_summary_list: Some(summaries),
            next_token: None,
        };
        wire::serialize_list_campaigns_response(&resp)
    }

    async fn handle_get_campaign_state(
        &self,
        state: &Arc<tokio::sync::RwLock<ConnectCampaignsState>>,
        id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_campaign_state(id) {
            Ok(campaign_state) => {
                let resp = wire::GetCampaignStateResponse {
                    state: Some(campaign_state.as_str().to_string()),
                    ..Default::default()
                };
                wire::serialize_get_campaign_state_response(&resp)
            }
            Err(e) => connectcampaigns_error_response(&e),
        }
    }

    async fn handle_start_campaign(
        &self,
        state: &Arc<tokio::sync::RwLock<ConnectCampaignsState>>,
        id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.start_campaign(id) {
            Ok(()) => wire::serialize_start_campaign_response(),
            Err(e) => connectcampaigns_error_response(&e),
        }
    }

    async fn handle_stop_campaign(
        &self,
        state: &Arc<tokio::sync::RwLock<ConnectCampaignsState>>,
        id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.stop_campaign(id) {
            Ok(()) => wire::serialize_stop_campaign_response(),
            Err(e) => connectcampaigns_error_response(&e),
        }
    }

    async fn handle_pause_campaign(
        &self,
        state: &Arc<tokio::sync::RwLock<ConnectCampaignsState>>,
        id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.pause_campaign(id) {
            Ok(()) => wire::serialize_pause_campaign_response(),
            Err(e) => connectcampaigns_error_response(&e),
        }
    }

    async fn handle_resume_campaign(
        &self,
        state: &Arc<tokio::sync::RwLock<ConnectCampaignsState>>,
        id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.resume_campaign(id) {
            Ok(()) => wire::serialize_resume_campaign_response(),
            Err(e) => connectcampaigns_error_response(&e),
        }
    }

    async fn handle_get_connect_instance_config(
        &self,
        state: &Arc<tokio::sync::RwLock<ConnectCampaignsState>>,
        connect_instance_id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_connect_instance_config(connect_instance_id) {
            Ok(config) => {
                let resp = wire::GetConnectInstanceConfigResponse {
                    connect_instance_config: Some(wire::InstanceConfig {
                        connect_instance_id: Some(config.connect_instance_id.clone()),
                        encryption_config: Some(wire::EncryptionConfig {
                            enabled: config.encryption_enabled,
                            encryption_type: config.encryption_type.clone(),
                            key_arn: config.key_arn.clone(),
                        }),
                        service_linked_role_arn: Some(config.service_linked_role_arn.clone()),
                    }),
                    ..Default::default()
                };
                wire::serialize_get_connect_instance_config_response(&resp)
            }
            Err(e) => connectcampaigns_error_response(&e),
        }
    }

    async fn handle_start_instance_onboarding_job(
        &self,
        state: &Arc<tokio::sync::RwLock<ConnectCampaignsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input =
            match wire::deserialize_start_instance_onboarding_job_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let encryption_config = serde_json::to_value(&input.encryption_config).unwrap_or(json!({}));
        let mut state = state.write().await;
        match state.start_instance_onboarding_job(
            &input.connect_instance_id,
            &encryption_config,
            account_id,
            region,
        ) {
            Ok(job) => {
                let resp = wire::StartInstanceOnboardingJobResponse {
                    connect_instance_onboarding_job_status: Some(
                        wire::InstanceOnboardingJobStatus {
                            connect_instance_id: Some(job.connect_instance_id),
                            status: Some(job.status),
                            failure_code: None,
                        },
                    ),
                    ..Default::default()
                };
                wire::serialize_start_instance_onboarding_job_response(&resp)
            }
            Err(e) => connectcampaigns_error_response(&e),
        }
    }

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<ConnectCampaignsState>>,
        arn: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.list_tags_for_resource(arn) {
            Ok(tags) => {
                let resp = wire::ListTagsForResourceResponse {
                    tags: if tags.is_empty() { None } else { Some(tags) },
                    ..Default::default()
                };
                wire::serialize_list_tags_for_resource_response(&resp)
            }
            Err(e) => connectcampaigns_error_response(&e),
        }
    }

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<ConnectCampaignsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.tag_resource(&input.arn, input.tags) {
            Ok(()) => wire::serialize_tag_resource_response(),
            Err(e) => connectcampaigns_error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<ConnectCampaignsState>>,
        arn: &str,
        tag_keys: &[String],
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.untag_resource(arn, tag_keys) {
            Ok(()) => wire::serialize_untag_resource_response(),
            Err(e) => connectcampaigns_error_response(&e),
        }
    }
}

/// Convert a state `Campaign` to the wire `Campaign` model type.
fn campaign_to_wire(c: &Campaign) -> wire::Campaign {
    wire::Campaign {
        id: Some(c.id.clone()),
        arn: Some(c.arn.clone()),
        name: Some(c.name.clone()),
        connect_instance_id: Some(c.connect_instance_id.clone()),
        dialer_config: Some(dialer_config_to_wire(&c.dialer_config)),
        outbound_call_config: Some(outbound_call_config_to_wire(&c.outbound_call_config)),
        tags: if c.tags.is_empty() {
            None
        } else {
            Some(c.tags.clone())
        },
    }
}

/// Convert a `serde_json::Value` dialer config into the wire `DialerConfig`.
fn dialer_config_to_wire(v: &Value) -> wire::DialerConfig {
    wire::DialerConfig {
        agentless_dialer_config: v.get("agentlessDialerConfig").map(|cfg| {
            wire::AgentlessDialerConfig {
                dialing_capacity: cfg.get("dialingCapacity").and_then(|v| v.as_f64()),
            }
        }),
        predictive_dialer_config: v.get("predictiveDialerConfig").map(|cfg| {
            wire::PredictiveDialerConfig {
                bandwidth_allocation: cfg
                    .get("bandwidthAllocation")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(0.0),
                dialing_capacity: cfg.get("dialingCapacity").and_then(|v| v.as_f64()),
            }
        }),
        progressive_dialer_config: v.get("progressiveDialerConfig").map(|cfg| {
            wire::ProgressiveDialerConfig {
                bandwidth_allocation: cfg
                    .get("bandwidthAllocation")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(0.0),
                dialing_capacity: cfg.get("dialingCapacity").and_then(|v| v.as_f64()),
            }
        }),
    }
}

/// Convert a `serde_json::Value` outbound call config into the wire `OutboundCallConfig`.
fn outbound_call_config_to_wire(v: &Value) -> wire::OutboundCallConfig {
    wire::OutboundCallConfig {
        connect_contact_flow_id: v
            .get("connectContactFlowId")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string(),
        connect_queue_id: v
            .get("connectQueueId")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()),
        connect_source_phone_number: v
            .get("connectSourcePhoneNumber")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()),
        answer_machine_detection_config: v.get("answerMachineDetectionConfig").map(|cfg| {
            wire::AnswerMachineDetectionConfig {
                enable_answer_machine_detection: cfg
                    .get("enableAnswerMachineDetection")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false),
                await_answer_machine_prompt: cfg
                    .get("awaitAnswerMachinePrompt")
                    .and_then(|v| v.as_bool()),
            }
        }),
    }
}

fn extract_path(uri: &str) -> String {
    if let Some(idx) = uri.find("amazonaws.com") {
        let after_host = &uri[idx + "amazonaws.com".len()..];
        if let Some(q) = after_host.find('?') {
            after_host[..q].to_string()
        } else {
            after_host.to_string()
        }
    } else {
        uri.split('?').next().unwrap_or(uri).to_string()
    }
}

fn rest_json_error(status: u16, code: &str, message: &str) -> MockResponse {
    let body = json!({
        "Type": "User",
        "Message": message,
    });
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers.insert(X_AMZN_ERRORTYPE, code.parse().unwrap());
    resp
}

fn connectcampaigns_error_response(err: &ConnectCampaignsError) -> MockResponse {
    let (status, error_type) = match err {
        ConnectCampaignsError::CampaignNameEmpty => (400, "ValidationException"),
        ConnectCampaignsError::CampaignNotFound(_) => (404, "ResourceNotFoundException"),
        ConnectCampaignsError::InstanceConfigNotFound(_) => (404, "ResourceNotFoundException"),
        ConnectCampaignsError::CampaignAlreadyRunning => (409, "ConflictException"),
        ConnectCampaignsError::CampaignAlreadyStopped => (409, "ConflictException"),
        ConnectCampaignsError::CampaignMustBeRunningToPause => (409, "ConflictException"),
        ConnectCampaignsError::CampaignMustBePausedToResume => (409, "ConflictException"),
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
