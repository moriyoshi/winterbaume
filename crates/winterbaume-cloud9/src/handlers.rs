use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, default_account_id,
};

use crate::state::{Cloud9Error, Cloud9State, EnvironmentRecord, MembershipRecord};
use crate::views::Cloud9StateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct Cloud9Service {
    pub(crate) state: Arc<BackendState<Cloud9State>>,
    pub(crate) notifier: StateChangeNotifier<Cloud9StateView>,
}

impl Cloud9Service {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for Cloud9Service {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for Cloud9Service {
    fn service_name(&self) -> &str {
        "cloud9"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://cloud9\..*\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

type SharedState = Arc<tokio::sync::RwLock<Cloud9State>>;

const MUTATING_ACTIONS: &[&str] = &[
    "CreateEnvironmentEC2",
    "CreateEnvironmentMembership",
    "DeleteEnvironment",
    "DeleteEnvironmentMembership",
    "TagResource",
    "UntagResource",
    "UpdateEnvironment",
    "UpdateEnvironmentMembership",
];

impl Cloud9Service {
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
            None => return aws_json_error(400, "MissingAction", "Missing X-Amz-Target"),
        };

        if !request.body.is_empty() && serde_json::from_slice::<Value>(&request.body).is_err() {
            return aws_json_error(400, "SerializationException", "Invalid JSON body");
        }
        let body_owned: Vec<u8> = if request.body.is_empty() {
            b"{}".to_vec()
        } else {
            request.body.to_vec()
        };
        let body_bytes: &[u8] = &body_owned;

        let state = self.state.get(account_id, &region);

        let response = match action.as_str() {
            "CreateEnvironmentEC2" => {
                self.handle_create_env(&state, account_id, &region, body_bytes)
                    .await
            }
            "CreateEnvironmentMembership" => {
                self.handle_create_membership(&state, body_bytes).await
            }
            "DeleteEnvironment" => self.handle_delete_env(&state, body_bytes).await,
            "DeleteEnvironmentMembership" => {
                self.handle_delete_membership(&state, body_bytes).await
            }
            "DescribeEnvironmentMemberships" => {
                self.handle_describe_memberships(&state, body_bytes).await
            }
            "DescribeEnvironmentStatus" => self.handle_describe_status(&state, body_bytes).await,
            "DescribeEnvironments" => self.handle_describe_envs(&state, body_bytes).await,
            "ListEnvironments" => self.handle_list_envs(&state).await,
            "ListTagsForResource" => self.handle_list_tags(&state, body_bytes).await,
            "TagResource" => self.handle_tag_resource(&state, body_bytes).await,
            "UntagResource" => self.handle_untag_resource(&state, body_bytes).await,
            "UpdateEnvironment" => self.handle_update_env(&state, body_bytes).await,
            "UpdateEnvironmentMembership" => {
                self.handle_update_membership(&state, body_bytes).await
            }
            other => aws_json_error(
                400,
                "BadRequestException",
                &format!("Unknown action: {other}"),
            ),
        };

        use winterbaume_core::StatefulService;
        if response.status / 100 == 2 && MUTATING_ACTIONS.contains(&action.as_str()) {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_create_env(
        &self,
        state: &SharedState,
        account_id: &str,
        region: &str,
        body: &[u8],
    ) -> MockResponse {
        let raw: Value = match serde_json::from_slice(body) {
            Ok(v) => v,
            Err(e) => return aws_json_error(400, "ValidationException", &e.to_string()),
        };
        let input = match wire::deserialize_create_environment_e_c2_request(body) {
            Ok(v) => v,
            Err(e) => return aws_json_error(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return aws_json_error(400, "ValidationException", "name is required");
        }
        let name = input.name.clone();
        if input.instance_type.is_empty() {
            return aws_json_error(400, "ValidationException", "instanceType is required");
        }
        let instance_type = input.instance_type.clone();
        // imageId is `String` in the typed model but the existing handler
        // treats it as optional. Detect presence via raw JSON.
        let image_id = if raw.get("imageId").is_some() {
            Some(input.image_id.clone())
        } else {
            None
        };
        let owner_arn = input
            .owner_arn
            .clone()
            .unwrap_or_else(|| format!("arn:aws:iam::{account_id}:root"));
        let id = uuid::Uuid::new_v4().simple().to_string();
        let arn = format!("arn:aws:cloud9:{region}:{account_id}:environment:{id}");
        let env = EnvironmentRecord {
            id: id.clone(),
            arn,
            name,
            description: input.description.clone(),
            env_type: "ec2".to_string(),
            connection_type: input.connection_type.clone(),
            owner_arn: owner_arn.clone(),
            instance_type: Some(instance_type),
            image_id,
            managed_credentials_status: "ENABLED_ON_CREATE".to_string(),
            status: "ready".to_string(),
            status_reason: None,
        };
        let mut state = state.write().await;
        state.create_environment(env);
        // Auto-create owner membership.
        let user_id = owner_arn
            .split('/')
            .next_back()
            .unwrap_or("owner")
            .to_string();
        state.upsert_membership(MembershipRecord {
            environment_id: id.clone(),
            user_arn: owner_arn,
            user_id,
            permissions: "owner".to_string(),
            last_access: None,
        });
        wire::serialize_create_environment_e_c2_response(&wire::CreateEnvironmentEC2Result {
            environment_id: Some(id),
        })
    }

    async fn handle_delete_env(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_delete_environment_request(body) {
            Ok(v) => v,
            Err(e) => return aws_json_error(400, "ValidationException", &e),
        };
        if input.environment_id.is_empty() {
            return aws_json_error(400, "ValidationException", "environmentId is required");
        }
        let id = input.environment_id.clone();
        let mut state = state.write().await;
        match state.delete_environment(&id) {
            Ok(()) => {
                wire::serialize_delete_environment_response(&wire::DeleteEnvironmentResult {})
            }
            Err(e) => err_response(&e),
        }
    }

    async fn handle_update_env(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_update_environment_request(body) {
            Ok(v) => v,
            Err(e) => return aws_json_error(400, "ValidationException", &e),
        };
        if input.environment_id.is_empty() {
            return aws_json_error(400, "ValidationException", "environmentId is required");
        }
        let id = input.environment_id.clone();
        let new_name = input.name.clone();
        let new_desc = input.description.clone();
        let mut state = state.write().await;
        match state.update_environment(&id, |e| {
            if let Some(n) = new_name {
                e.name = n;
            }
            if let Some(d) = new_desc {
                e.description = Some(d);
            }
        }) {
            Ok(_) => wire::serialize_update_environment_response(&wire::UpdateEnvironmentResult {}),
            Err(e) => err_response(&e),
        }
    }

    async fn handle_describe_envs(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_describe_environments_request(body) {
            Ok(v) => v,
            Err(e) => return aws_json_error(400, "ValidationException", &e),
        };
        let ids = input.environment_ids.clone();
        let state = state.read().await;
        let envs: Vec<wire::Environment> = state
            .describe_environments(&ids)
            .into_iter()
            .map(env_to_wire)
            .collect();
        wire::serialize_describe_environments_response(&wire::DescribeEnvironmentsResult {
            environments: Some(envs),
        })
    }

    async fn handle_describe_status(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_describe_environment_status_request(body) {
            Ok(v) => v,
            Err(e) => return aws_json_error(400, "ValidationException", &e),
        };
        if input.environment_id.is_empty() {
            return aws_json_error(400, "ValidationException", "environmentId is required");
        }
        let id = input.environment_id.clone();
        let state = state.read().await;
        match state.get_environment(&id) {
            Ok(e) => wire::serialize_describe_environment_status_response(
                &wire::DescribeEnvironmentStatusResult {
                    message: e.status_reason.clone(),
                    status: Some(e.status.clone()),
                },
            ),
            Err(e) => err_response(&e),
        }
    }

    async fn handle_list_envs(&self, state: &SharedState) -> MockResponse {
        let state = state.read().await;
        let ids = state.list_environment_ids();
        wire::serialize_list_environments_response(&wire::ListEnvironmentsResult {
            environment_ids: Some(ids),
            next_token: None,
        })
    }

    async fn handle_create_membership(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_create_environment_membership_request(body) {
            Ok(v) => v,
            Err(e) => return aws_json_error(400, "ValidationException", &e),
        };
        if input.environment_id.is_empty() {
            return aws_json_error(400, "ValidationException", "environmentId is required");
        }
        let environment_id = input.environment_id.clone();
        if input.user_arn.is_empty() {
            return aws_json_error(400, "ValidationException", "userArn is required");
        }
        let user_arn = input.user_arn.clone();
        if input.permissions.is_empty() {
            return aws_json_error(400, "ValidationException", "permissions is required");
        }
        let permissions = input.permissions.clone();
        let mut state = state.write().await;
        if state.get_environment(&environment_id).is_err() {
            return err_response(&Cloud9Error::EnvironmentNotFound {
                id: environment_id.clone(),
            });
        }
        let user_id = user_arn
            .split('/')
            .next_back()
            .unwrap_or("user")
            .to_string();
        let m = state.upsert_membership(MembershipRecord {
            environment_id: environment_id.clone(),
            user_arn: user_arn.clone(),
            user_id: user_id.clone(),
            permissions: permissions.clone(),
            last_access: None,
        });
        let wire_m = membership_to_wire(m);
        wire::serialize_create_environment_membership_response(
            &wire::CreateEnvironmentMembershipResult {
                membership: Some(wire_m),
            },
        )
    }

    async fn handle_update_membership(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_update_environment_membership_request(body) {
            Ok(v) => v,
            Err(e) => return aws_json_error(400, "ValidationException", &e),
        };
        if input.environment_id.is_empty() {
            return aws_json_error(400, "ValidationException", "environmentId is required");
        }
        let environment_id = input.environment_id.clone();
        if input.user_arn.is_empty() {
            return aws_json_error(400, "ValidationException", "userArn is required");
        }
        let user_arn = input.user_arn.clone();
        if input.permissions.is_empty() {
            return aws_json_error(400, "ValidationException", "permissions is required");
        }
        let permissions = input.permissions.clone();
        let mut state = state.write().await;
        match state.update_membership(&environment_id, &user_arn, permissions) {
            Ok(m) => wire::serialize_update_environment_membership_response(
                &wire::UpdateEnvironmentMembershipResult {
                    membership: Some(membership_to_wire(m)),
                },
            ),
            Err(e) => err_response(&e),
        }
    }

    async fn handle_delete_membership(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_delete_environment_membership_request(body) {
            Ok(v) => v,
            Err(e) => return aws_json_error(400, "ValidationException", &e),
        };
        if input.environment_id.is_empty() {
            return aws_json_error(400, "ValidationException", "environmentId is required");
        }
        let environment_id = input.environment_id.clone();
        if input.user_arn.is_empty() {
            return aws_json_error(400, "ValidationException", "userArn is required");
        }
        let user_arn = input.user_arn.clone();
        let mut state = state.write().await;
        match state.delete_membership(&environment_id, &user_arn) {
            Ok(()) => wire::serialize_delete_environment_membership_response(
                &wire::DeleteEnvironmentMembershipResult {},
            ),
            Err(e) => err_response(&e),
        }
    }

    async fn handle_describe_memberships(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_describe_environment_memberships_request(body) {
            Ok(v) => v,
            Err(e) => return aws_json_error(400, "ValidationException", &e),
        };
        let environment_id = input.environment_id.as_deref();
        let user_arn = input.user_arn.as_deref();
        let permissions: Option<Vec<String>> = input.permissions.clone();
        let state = state.read().await;
        let memberships: Vec<wire::EnvironmentMember> = state
            .describe_memberships(environment_id, user_arn, permissions.as_deref())
            .into_iter()
            .map(membership_to_wire)
            .collect();
        wire::serialize_describe_environment_memberships_response(
            &wire::DescribeEnvironmentMembershipsResult {
                memberships: Some(memberships),
                next_token: None,
            },
        )
    }

    async fn handle_list_tags(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(body) {
            Ok(v) => v,
            Err(e) => return aws_json_error(400, "ValidationException", &e),
        };
        if input.resource_a_r_n.is_empty() {
            return aws_json_error(400, "ValidationException", "ResourceARN is required");
        }
        let arn = input.resource_a_r_n.clone();
        let state = state.read().await;
        let tags_map = state.list_tags(&arn);
        wire::serialize_list_tags_for_resource_response(&wire::ListTagsForResourceResponse {
            tags: if tags_map.is_empty() {
                None
            } else {
                Some(tags_to_wire(&tags_map))
            },
        })
    }

    async fn handle_tag_resource(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return aws_json_error(400, "ValidationException", &e),
        };
        if input.resource_a_r_n.is_empty() {
            return aws_json_error(400, "ValidationException", "ResourceARN is required");
        }
        let arn = input.resource_a_r_n.clone();
        let tags: HashMap<String, String> = input
            .tags
            .iter()
            .map(|t| (t.key.clone(), t.value.clone()))
            .collect();
        if tags.is_empty() {
            return aws_json_error(400, "ValidationException", "Tags is required");
        }
        let mut state = state.write().await;
        state.tag_resource(&arn, tags);
        wire::serialize_tag_resource_response(&wire::TagResourceResponse {})
    }

    async fn handle_untag_resource(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return aws_json_error(400, "ValidationException", &e),
        };
        if input.resource_a_r_n.is_empty() {
            return aws_json_error(400, "ValidationException", "ResourceARN is required");
        }
        let arn = input.resource_a_r_n.clone();
        let keys = input.tag_keys.clone();
        let mut state = state.write().await;
        state.untag_resource(&arn, &keys);
        wire::serialize_untag_resource_response(&wire::UntagResourceResponse {})
    }
}

fn tags_to_wire(tags: &HashMap<String, String>) -> Vec<wire::Tag> {
    let mut v: Vec<wire::Tag> = tags
        .iter()
        .map(|(k, val)| wire::Tag {
            key: k.clone(),
            value: val.clone(),
        })
        .collect();
    v.sort_by(|a, b| a.key.cmp(&b.key));
    v
}

fn env_to_wire(e: &EnvironmentRecord) -> wire::Environment {
    wire::Environment {
        arn: Some(e.arn.clone()),
        connection_type: e.connection_type.clone(),
        description: e.description.clone(),
        id: Some(e.id.clone()),
        lifecycle: Some(wire::EnvironmentLifecycle {
            failure_resource: None,
            reason: e.status_reason.clone(),
            status: Some("CREATED".to_string()),
        }),
        managed_credentials_status: Some(e.managed_credentials_status.clone()),
        name: Some(e.name.clone()),
        owner_arn: Some(e.owner_arn.clone()),
        r#type: Some(e.env_type.clone()),
    }
}

fn membership_to_wire(m: &MembershipRecord) -> wire::EnvironmentMember {
    wire::EnvironmentMember {
        environment_id: Some(m.environment_id.clone()),
        last_access: m.last_access,
        permissions: Some(m.permissions.clone()),
        user_arn: Some(m.user_arn.clone()),
        user_id: Some(m.user_id.clone()),
    }
}

fn err_response(err: &Cloud9Error) -> MockResponse {
    let (status, error_type) = match err {
        Cloud9Error::EnvironmentNotFound { .. } | Cloud9Error::MembershipNotFound { .. } => {
            (404, "NotFoundException")
        }
        Cloud9Error::Validation { .. } => (400, "BadRequestException"),
    };
    aws_json_error(status, error_type, &err.to_string())
}

fn aws_json_error(status: u16, error_type: &str, message: &str) -> MockResponse {
    let body = json!({"__type": error_type, "Message": message});
    let mut resp = MockResponse::json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}
