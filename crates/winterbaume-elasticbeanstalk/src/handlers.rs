use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, default_account_id,
};

use crate::state::{ElasticBeanstalkError, ElasticBeanstalkState};
use crate::views::ElasticBeanstalkStateView;
use crate::wire;

/// Pre-defined list of common solution stacks.
const SOLUTION_STACKS: &[&str] = &[
    "64bit Amazon Linux 2023 v4.3.0 running Python 3.11",
    "64bit Amazon Linux 2023 v4.3.0 running Python 3.12",
    "64bit Amazon Linux 2023 v6.1.0 running Node.js 20",
    "64bit Amazon Linux 2023 v6.1.0 running Node.js 18",
    "64bit Amazon Linux 2023 v4.1.0 running Corretto 17",
    "64bit Amazon Linux 2023 v4.1.0 running Corretto 21",
    "64bit Amazon Linux 2023 v3.4.0 running Docker",
    "64bit Amazon Linux 2023 v4.0.0 running PHP 8.3",
    "64bit Amazon Linux 2023 v4.0.0 running PHP 8.2",
    "64bit Amazon Linux 2023 v4.0.0 running Ruby 3.2",
    "64bit Amazon Linux 2023 v4.0.0 running Tomcat 10.1 with Corretto 21",
    "64bit Amazon Linux 2023 v4.0.0 running Go 1.21",
];

const MUTATING_ACTIONS: &[&str] = &[
    "CreateApplication",
    "DeleteApplication",
    "CreateEnvironment",
    "UpdateTagsForResource",
];

pub struct ElasticBeanstalkService {
    pub(crate) state: Arc<BackendState<ElasticBeanstalkState>>,
    pub(crate) notifier: StateChangeNotifier<ElasticBeanstalkStateView>,
}

impl ElasticBeanstalkService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for ElasticBeanstalkService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for ElasticBeanstalkService {
    fn service_name(&self) -> &str {
        "elasticbeanstalk"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://elasticbeanstalk\..*\.amazonaws\.com",
            r"https?://elasticbeanstalk\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl ElasticBeanstalkService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();

        let body_str = std::str::from_utf8(&request.body).unwrap_or("");
        let params = parse_query_string(body_str);

        let action = match params.get("Action") {
            Some(a) => a.clone(),
            None => {
                return MockResponse::error(400, "MissingAction", "Missing 'Action' parameter");
            }
        };

        let state = self.state.get(account_id, &region);

        let response = match action.as_str() {
            "CreateApplication" => {
                self.handle_create_application(&state, &params, account_id, &region)
                    .await
            }
            "DeleteApplication" => self.handle_delete_application(&state, &params).await,
            "CreateEnvironment" => {
                self.handle_create_environment(&state, &params, account_id, &region)
                    .await
            }
            "DescribeEnvironments" => self.handle_describe_environments(&state, &params).await,
            "ListAvailableSolutionStacks" => self.handle_list_available_solution_stacks().await,
            "ListTagsForResource" => self.handle_list_tags_for_resource(&state, &params).await,
            "UpdateTagsForResource" => self.handle_update_tags_for_resource(&state, &params).await,
            _ => MockResponse::error(400, "InvalidAction", &format!("Unknown action: {action}")),
        };

        use winterbaume_core::StatefulService;
        if response.status / 100 == 2 && MUTATING_ACTIONS.contains(&action.as_str()) {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_create_application(
        &self,
        state: &Arc<tokio::sync::RwLock<ElasticBeanstalkState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_application_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.application_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "ApplicationName is required");
        }
        let tags = input
            .tags
            .map(|t| {
                t.items
                    .into_iter()
                    .filter_map(|tag| Some((tag.key?, tag.value.unwrap_or_default())))
                    .collect::<HashMap<String, String>>()
            })
            .unwrap_or_default();

        let mut st = state.write().await;
        match st.create_application(
            input.application_name,
            input.description,
            tags,
            account_id,
            region,
        ) {
            Ok(app) => {
                let result = wire::ApplicationDescriptionMessage {
                    application: Some(to_wire_application(app)),
                };
                wire::serialize_create_application_response(&result)
            }
            Err(e) => elasticbeanstalk_error_response(&e),
        }
    }

    async fn handle_delete_application(
        &self,
        state: &Arc<tokio::sync::RwLock<ElasticBeanstalkState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_application_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.application_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "ApplicationName is required");
        }
        let mut st = state.write().await;
        match st.delete_application(&input.application_name) {
            Ok(()) => wire::serialize_delete_application_response(),
            Err(e) => elasticbeanstalk_error_response(&e),
        }
    }

    async fn handle_create_environment(
        &self,
        state: &Arc<tokio::sync::RwLock<ElasticBeanstalkState>>,
        params: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_environment_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let environment_name = match input.environment_name {
            Some(n) if !n.is_empty() => n,
            _ => {
                return MockResponse::error(400, "MissingParameter", "EnvironmentName is required");
            }
        };
        if input.application_name.is_empty() {
            return MockResponse::error(400, "MissingParameter", "ApplicationName is required");
        }
        let tier_name = input
            .tier
            .as_ref()
            .and_then(|t| t.name.clone())
            .unwrap_or_else(|| "WebServer".to_string());
        let tier_type = input
            .tier
            .as_ref()
            .and_then(|t| t.r#type.clone())
            .unwrap_or_else(|| "Standard".to_string());
        let tags = input
            .tags
            .map(|t| {
                t.items
                    .into_iter()
                    .filter_map(|tag| Some((tag.key?, tag.value.unwrap_or_default())))
                    .collect::<HashMap<String, String>>()
            })
            .unwrap_or_default();

        let mut st = state.write().await;
        match st.create_environment(
            environment_name,
            input.application_name,
            input.description,
            input.solution_stack_name,
            tier_name,
            tier_type,
            tags,
            account_id,
            region,
        ) {
            Ok(env) => wire::serialize_create_environment_response(&to_wire_environment(env)),
            Err(e) => elasticbeanstalk_error_response(&e),
        }
    }

    async fn handle_describe_environments(
        &self,
        state: &Arc<tokio::sync::RwLock<ElasticBeanstalkState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_environments_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        let env_names: Vec<String> = input.environment_names.map(|n| n.items).unwrap_or_default();

        let st = state.read().await;
        let envs: Vec<wire::EnvironmentDescription> = st
            .describe_environments(input.application_name.as_deref(), &env_names)
            .into_iter()
            .map(to_wire_environment)
            .collect();
        let result = wire::EnvironmentDescriptionsMessage {
            environments: if envs.is_empty() {
                None
            } else {
                Some(envs.into())
            },
            ..Default::default()
        };
        wire::serialize_describe_environments_response(&result)
    }

    async fn handle_list_available_solution_stacks(&self) -> MockResponse {
        let stacks: Vec<String> = SOLUTION_STACKS.iter().map(|s| s.to_string()).collect();
        let result = wire::ListAvailableSolutionStacksResultMessage {
            solution_stacks: Some(stacks.into()),
            solution_stack_details: None,
        };
        wire::serialize_list_available_solution_stacks_response(&result)
    }

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<ElasticBeanstalkState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.resource_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "ResourceArn is required");
        }
        let resource_arn = input.resource_arn;

        let env_name = extract_environment_name_from_arn(&resource_arn);
        let st = state.read().await;
        match st.get_environment_tags(&env_name) {
            Ok(tags) => {
                let tag_list: Vec<wire::Tag> = tags
                    .iter()
                    .map(|(k, v)| wire::Tag {
                        key: Some(k.clone()),
                        value: Some(v.clone()),
                    })
                    .collect();
                let result = wire::ResourceTagsDescriptionMessage {
                    resource_arn: Some(resource_arn),
                    resource_tags: Some(tag_list.into()),
                };
                wire::serialize_list_tags_for_resource_response(&result)
            }
            Err(e) => elasticbeanstalk_error_response(&e),
        }
    }

    async fn handle_update_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<ElasticBeanstalkState>>,
        params: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_tags_for_resource_request(params) {
            Ok(v) => v,
            Err(e) => return MockResponse::error(400, "InvalidParameterValue", &e),
        };
        if input.resource_arn.is_empty() {
            return MockResponse::error(400, "MissingParameter", "ResourceArn is required");
        }

        let env_name = extract_environment_name_from_arn(&input.resource_arn);

        let tags_to_add: HashMap<String, String> = input
            .tags_to_add
            .map(|t| {
                t.items
                    .into_iter()
                    .filter_map(|tag| Some((tag.key?, tag.value.unwrap_or_default())))
                    .collect()
            })
            .unwrap_or_default();
        let tags_to_remove: Vec<String> = input.tags_to_remove.map(|t| t.items).unwrap_or_default();

        let mut st = state.write().await;
        match st.update_environment_tags(&env_name, tags_to_add, tags_to_remove) {
            Ok(()) => wire::serialize_update_tags_for_resource_response(),
            Err(e) => elasticbeanstalk_error_response(&e),
        }
    }
}

fn to_wire_application(app: &crate::types::Application) -> wire::ApplicationDescription {
    wire::ApplicationDescription {
        application_name: Some(app.application_name.clone()),
        description: app.description.clone(),
        date_created: Some(app.date_created.clone()),
        date_updated: Some(app.date_updated.clone()),
        application_arn: Some(app.arn.clone()),
        ..Default::default()
    }
}

fn to_wire_environment(env: &crate::types::Environment) -> wire::EnvironmentDescription {
    wire::EnvironmentDescription {
        environment_name: Some(env.environment_name.clone()),
        application_name: Some(env.application_name.clone()),
        environment_id: Some(env.environment_id.clone()),
        description: env.description.clone(),
        status: Some(env.status.clone()),
        health: Some(env.health.clone()),
        solution_stack_name: env.solution_stack_name.clone(),
        environment_arn: Some(env.arn.clone()),
        date_created: Some(env.date_created.clone()),
        date_updated: Some(env.date_updated.clone()),
        tier: Some(wire::EnvironmentTier {
            name: Some(env.tier_name.clone()),
            r#type: Some(env.tier_type.clone()),
            ..Default::default()
        }),
        ..Default::default()
    }
}

fn extract_environment_name_from_arn(arn: &str) -> String {
    // ARN format: arn:aws:elasticbeanstalk:region:account:environment/app/env
    // or just use the last path segment
    arn.rsplit('/').next().unwrap_or(arn).to_string()
}

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
    let mut bytes = s.bytes();
    while let Some(b) = bytes.next() {
        match b {
            b'+' => result.push(' '),
            b'%' => {
                let hi = bytes.next().and_then(hex_val);
                let lo = bytes.next().and_then(hex_val);
                if let (Some(h), Some(l)) = (hi, lo) {
                    result.push(char::from(h * 16 + l));
                }
            }
            _ => result.push(char::from(b)),
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

fn elasticbeanstalk_error_response(err: &ElasticBeanstalkError) -> MockResponse {
    let (status, error_code) = match err {
        ElasticBeanstalkError::ApplicationAlreadyExists { .. } => (400, "InvalidParameterValue"),
        ElasticBeanstalkError::ApplicationNotFound { .. } => (400, "InvalidParameterValue"),
        ElasticBeanstalkError::EnvironmentNotFound { .. } => (400, "InvalidParameterValue"),
    };
    let request_id = uuid::Uuid::new_v4();
    let message = err.to_string();
    let xml = format!(
        r#"<ErrorResponse xmlns="http://elasticbeanstalk.amazonaws.com/docs/2010-12-01/">
  <Error>
    <Type>Sender</Type>
    <Code>{error_code}</Code>
    <Message>{message}</Message>
  </Error>
  <RequestId>{request_id}</RequestId>
</ErrorResponse>"#
    );
    MockResponse::xml(status, xml)
}
