use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::Value;
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, json_error_response,
};

use crate::state::{Ec2InstanceConnectError, Ec2InstanceConnectState};
use crate::views::Ec2InstanceConnectStateView;
use crate::wire;

pub struct Ec2InstanceConnectService {
    pub(crate) state: Arc<BackendState<Ec2InstanceConnectState>>,
    pub(crate) notifier: StateChangeNotifier<Ec2InstanceConnectStateView>,
}

impl Ec2InstanceConnectService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for Ec2InstanceConnectService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for Ec2InstanceConnectService {
    fn service_name(&self) -> &str {
        "ec2-instance-connect"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://ec2-instance-connect\..*\.amazonaws\.com",
            r"https?://ec2-instance-connect\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

async fn shape_error(err: Ec2InstanceConnectError) -> MockResponse {
    match err {
        Ec2InstanceConnectError::MissingAction => {
            json_error_response(400, "MissingAction", &err.to_string())
        }
        Ec2InstanceConnectError::SerializationException => {
            json_error_response(400, "SerializationException", &err.to_string())
        }
        Ec2InstanceConnectError::InvalidAction { .. } => {
            json_error_response(400, "InvalidAction", &err.to_string())
        }
        Ec2InstanceConnectError::InstanceIdRequired => {
            json_error_response(400, "InvalidArgsException", &err.to_string())
        }
        Ec2InstanceConnectError::InstanceOSUserRequired => {
            json_error_response(400, "InvalidArgsException", &err.to_string())
        }
        Ec2InstanceConnectError::SSHPublicKeyRequired => {
            json_error_response(400, "InvalidArgsException", &err.to_string())
        }
    }
}

impl Ec2InstanceConnectService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = winterbaume_core::DEFAULT_ACCOUNT_ID;

        // Extract action from X-Amz-Target header
        // Format: "AWSEC2InstanceConnectService.SendSSHPublicKey"
        let action = request
            .headers
            .get("x-amz-target")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.split('.').next_back())
            .map(|s| s.to_string());

        let action = match action {
            Some(a) => a,
            None => {
                return shape_error(Ec2InstanceConnectError::MissingAction).await;
            }
        };

        let body: Value = match serde_json::from_slice(&request.body) {
            Ok(v) => v,
            Err(_) => {
                return shape_error(Ec2InstanceConnectError::SerializationException).await;
            }
        };

        // Ensure state is initialized for the region (even though near-stateless)
        let _state = self.state.get(account_id, &region);

        use winterbaume_core::StatefulService;
        let response = match action.as_str() {
            "SendSSHPublicKey" => self.handle_send_ssh_public_key(&body).await,
            "SendSerialConsoleSSHPublicKey" => {
                self.handle_send_serial_console_ssh_public_key(&body).await
            }
            _ => {
                shape_error(Ec2InstanceConnectError::InvalidAction {
                    action: action.clone(),
                })
                .await
            }
        };
        if response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_send_ssh_public_key(&self, body: &Value) -> MockResponse {
        // Validate required fields
        if body.get("InstanceId").and_then(|v| v.as_str()).is_none() {
            return shape_error(Ec2InstanceConnectError::InstanceIdRequired).await;
        }
        if body
            .get("InstanceOSUser")
            .and_then(|v| v.as_str())
            .is_none()
        {
            return shape_error(Ec2InstanceConnectError::InstanceOSUserRequired).await;
        }
        if body.get("SSHPublicKey").and_then(|v| v.as_str()).is_none() {
            return shape_error(Ec2InstanceConnectError::SSHPublicKeyRequired).await;
        }

        wire::serialize_send_s_s_h_public_key_response(&wire::SendSSHPublicKeyResponse {
            request_id: Some(uuid::Uuid::new_v4().to_string()),
            success: Some(true),
        })
    }

    async fn handle_send_serial_console_ssh_public_key(&self, body: &Value) -> MockResponse {
        // Validate required fields
        if body.get("InstanceId").and_then(|v| v.as_str()).is_none() {
            return shape_error(Ec2InstanceConnectError::InstanceIdRequired).await;
        }
        if body.get("SSHPublicKey").and_then(|v| v.as_str()).is_none() {
            return shape_error(Ec2InstanceConnectError::SSHPublicKeyRequired).await;
        }

        wire::serialize_send_serial_console_s_s_h_public_key_response(
            &wire::SendSerialConsoleSSHPublicKeyResponse {
                request_id: Some(uuid::Uuid::new_v4().to_string()),
                success: Some(true),
            },
        )
    }
}
