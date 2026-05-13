use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::json;
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, StatefulService,
    default_account_id,
};

use crate::state::{MqError, MqState};
use crate::views::MqStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct MqService {
    pub(crate) state: Arc<BackendState<MqState>>,
    pub(crate) notifier: StateChangeNotifier<MqStateView>,
}

impl MqService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for MqService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for MqService {
    fn service_name(&self) -> &str {
        "mq"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://mq\..*\.amazonaws\.com",
            r"https?://mq\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl MqService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();
        let state = self.state.get(account_id, &region);

        let (path, query_string) = extract_path_and_query(&request.uri);
        let method = request.method.as_str();
        let query_map: HashMap<String, String> =
            winterbaume_core::parse_query_string(&query_string);

        let segments: Vec<&str> = path
            .trim_start_matches('/')
            .split('/')
            .filter(|s| !s.is_empty())
            .collect();

        if segments.is_empty() || segments[0] != "v1" {
            return rest_json_error(404, "NotFoundException", "Not found");
        }

        if segments.len() < 2 {
            return rest_json_error(404, "NotFoundException", "Not found");
        }

        let response = if segments[1] == "tags" && segments.len() >= 3 {
            // Handle /v1/tags/{ResourceArn+} routes
            let arn = percent_decode(&segments[2..].join("/"));
            let labels: &[(&str, &str)] = &[("ResourceArn", arn.as_str())];
            match method {
                "GET" => {
                    self.handle_list_tags(&state, &request, labels, &query_map)
                        .await
                }
                "POST" => {
                    self.handle_create_tags(&state, &request, labels, &query_map)
                        .await
                }
                "DELETE" => {
                    self.handle_delete_tags(&state, &request, labels, &query_map, &query_string)
                        .await
                }
                _ => rest_json_error(404, "NotFoundException", "Not found"),
            }
        } else if segments[1] == "broker-engine-types" {
            // Handle /v1/broker-engine-types
            match method {
                "GET" => {
                    self.handle_describe_broker_engine_types(&state, &request, &[], &query_map)
                        .await
                }
                _ => rest_json_error(404, "NotFoundException", "Not found"),
            }
        } else if segments[1] == "broker-instance-options" {
            // Handle /v1/broker-instance-options
            match method {
                "GET" => {
                    self.handle_describe_broker_instance_options(&request, &[], &query_map)
                        .await
                }
                _ => rest_json_error(404, "NotFoundException", "Not found"),
            }
        } else if segments[1] == "configurations" {
            // Handle /v1/configurations routes
            match (method, segments.len()) {
                // POST /v1/configurations - CreateConfiguration
                ("POST", 2) => {
                    self.handle_create_configuration(
                        &state,
                        &request,
                        &[],
                        &query_map,
                        account_id,
                        &region,
                    )
                    .await
                }
                // GET /v1/configurations - ListConfigurations
                ("GET", 2) => {
                    self.handle_list_configurations(&state, &request, &[], &query_map)
                        .await
                }
                // GET /v1/configurations/{ConfigurationId} - DescribeConfiguration
                ("GET", 3) => {
                    let config_id = percent_decode(segments[2]);
                    let labels: &[(&str, &str)] = &[("ConfigurationId", config_id.as_str())];
                    self.handle_describe_configuration(&state, &request, labels, &query_map)
                        .await
                }
                // DELETE /v1/configurations/{ConfigurationId} - DeleteConfiguration
                ("DELETE", 3) => {
                    let config_id = percent_decode(segments[2]);
                    let labels: &[(&str, &str)] = &[("ConfigurationId", config_id.as_str())];
                    self.handle_delete_configuration(&state, &request, labels, &query_map)
                        .await
                }
                // PUT /v1/configurations/{ConfigurationId} - UpdateConfiguration
                ("PUT", 3) => {
                    let config_id = percent_decode(segments[2]);
                    let labels: &[(&str, &str)] = &[("ConfigurationId", config_id.as_str())];
                    self.handle_update_configuration(&state, &request, labels, &query_map)
                        .await
                }
                // GET /v1/configurations/{ConfigurationId}/revisions - ListConfigurationRevisions
                ("GET", 4) if segments[3] == "revisions" => {
                    let config_id = percent_decode(segments[2]);
                    let labels: &[(&str, &str)] = &[("ConfigurationId", config_id.as_str())];
                    self.handle_list_configuration_revisions(&state, &request, labels, &query_map)
                        .await
                }
                // GET /v1/configurations/{ConfigurationId}/revisions/{Revision} - DescribeConfigurationRevision
                ("GET", 5) if segments[3] == "revisions" => {
                    let config_id = percent_decode(segments[2]);
                    let revision = percent_decode(segments[4]);
                    let labels: &[(&str, &str)] = &[
                        ("ConfigurationId", config_id.as_str()),
                        ("ConfigurationRevision", revision.as_str()),
                    ];
                    self.handle_describe_configuration_revision(
                        &state, &request, labels, &query_map,
                    )
                    .await
                }
                _ => rest_json_error(404, "NotFoundException", "Not found"),
            }
        } else if segments[1] != "brokers" {
            // Handle /v1/brokers routes
            rest_json_error(404, "NotFoundException", "Not found")
        } else {
            match (method, segments.as_slice()) {
                // POST /v1/brokers - CreateBroker
                ("POST", ["v1", "brokers"]) => {
                    self.handle_create_broker(
                        &state,
                        &request,
                        &[],
                        &query_map,
                        account_id,
                        &region,
                    )
                    .await
                }
                // GET /v1/brokers - ListBrokers
                ("GET", ["v1", "brokers"]) => {
                    self.handle_list_brokers(&state, &request, &[], &query_map)
                        .await
                }
                // GET /v1/brokers/{broker-id} - DescribeBroker
                ("GET", ["v1", "brokers", broker_id]) => {
                    let decoded = percent_decode(broker_id);
                    let labels: &[(&str, &str)] = &[("BrokerId", decoded.as_str())];
                    self.handle_describe_broker(&state, &request, labels, &query_map)
                        .await
                }
                // DELETE /v1/brokers/{broker-id} - DeleteBroker
                ("DELETE", ["v1", "brokers", broker_id]) => {
                    let decoded = percent_decode(broker_id);
                    let labels: &[(&str, &str)] = &[("BrokerId", decoded.as_str())];
                    self.handle_delete_broker(&state, &request, labels, &query_map)
                        .await
                }
                // POST /v1/brokers/{BrokerId}/reboot - RebootBroker
                ("POST", ["v1", "brokers", broker_id, "reboot"]) => {
                    let decoded = percent_decode(broker_id);
                    let labels: &[(&str, &str)] = &[("BrokerId", decoded.as_str())];
                    self.handle_reboot_broker(&state, &request, labels, &query_map)
                        .await
                }
                // POST /v1/brokers/{BrokerId}/promote - Promote
                ("POST", ["v1", "brokers", broker_id, "promote"]) => {
                    let decoded = percent_decode(broker_id);
                    let labels: &[(&str, &str)] = &[("BrokerId", decoded.as_str())];
                    self.handle_promote(&state, &request, labels, &query_map)
                        .await
                }
                // POST /v1/brokers/{BrokerId}/users/{Username} - CreateUser
                ("POST", ["v1", "brokers", broker_id, "users", username]) => {
                    let broker = percent_decode(broker_id);
                    let user = percent_decode(username);
                    let labels: &[(&str, &str)] =
                        &[("BrokerId", broker.as_str()), ("Username", user.as_str())];
                    self.handle_create_user(&state, &request, labels, &query_map)
                        .await
                }
                // DELETE /v1/brokers/{BrokerId}/users/{Username} - DeleteUser
                ("DELETE", ["v1", "brokers", broker_id, "users", username]) => {
                    let broker = percent_decode(broker_id);
                    let user = percent_decode(username);
                    let labels: &[(&str, &str)] =
                        &[("BrokerId", broker.as_str()), ("Username", user.as_str())];
                    self.handle_delete_user(&state, &request, labels, &query_map)
                        .await
                }
                // GET /v1/brokers/{BrokerId}/users/{Username} - DescribeUser
                ("GET", ["v1", "brokers", broker_id, "users", username]) => {
                    let broker = percent_decode(broker_id);
                    let user = percent_decode(username);
                    let labels: &[(&str, &str)] =
                        &[("BrokerId", broker.as_str()), ("Username", user.as_str())];
                    self.handle_describe_user(&state, &request, labels, &query_map)
                        .await
                }
                // GET /v1/brokers/{BrokerId}/users - ListUsers
                ("GET", ["v1", "brokers", broker_id, "users"]) => {
                    let decoded = percent_decode(broker_id);
                    let labels: &[(&str, &str)] = &[("BrokerId", decoded.as_str())];
                    self.handle_list_users(&state, &request, labels, &query_map)
                        .await
                }
                // PUT /v1/brokers/{broker-id} - UpdateBroker
                ("PUT", ["v1", "brokers", broker_id]) => {
                    let decoded = percent_decode(broker_id);
                    let labels: &[(&str, &str)] = &[("BrokerId", decoded.as_str())];
                    self.handle_update_broker(&state, &request, labels, &query_map)
                        .await
                }
                // PUT /v1/brokers/{BrokerId}/users/{Username} - UpdateUser
                ("PUT", ["v1", "brokers", broker_id, "users", username]) => {
                    let broker = percent_decode(broker_id);
                    let user = percent_decode(username);
                    let labels: &[(&str, &str)] =
                        &[("BrokerId", broker.as_str()), ("Username", user.as_str())];
                    self.handle_update_user(&state, &request, labels, &query_map)
                        .await
                }
                _ => rest_json_error(404, "NotFoundException", "Not found"),
            }
        };

        if matches!(method, "PUT" | "POST" | "DELETE") && response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_create_broker(
        &self,
        state: &Arc<tokio::sync::RwLock<MqState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_broker_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        if input.broker_name.is_empty() {
            return rest_json_error(
                400,
                "BadRequestException",
                "Missing required field: brokerName",
            );
        }
        let engine_type = if input.engine_type.is_empty() {
            "ACTIVEMQ"
        } else {
            input.engine_type.as_str()
        };
        let engine_version = input.engine_version.as_deref().unwrap_or("5.17.6");
        let host_instance_type = if input.host_instance_type.is_empty() {
            "mq.m5.large"
        } else {
            input.host_instance_type.as_str()
        };
        let deployment_mode = if input.deployment_mode.is_empty() {
            "SINGLE_INSTANCE"
        } else {
            input.deployment_mode.as_str()
        };
        let publicly_accessible = input.publicly_accessible;
        let auto_minor_version_upgrade = input.auto_minor_version_upgrade.unwrap_or(false);
        let tags = input.tags;

        let mut state = state.write().await;
        match state.create_broker(
            &input.broker_name,
            engine_type,
            engine_version,
            host_instance_type,
            deployment_mode,
            publicly_accessible,
            auto_minor_version_upgrade,
            account_id,
            region,
            tags,
        ) {
            Ok(broker) => wire::serialize_create_broker_response(&wire::CreateBrokerResponse {
                broker_arn: Some(broker.broker_arn.clone()),
                broker_id: Some(broker.broker_id.clone()),
            }),
            Err(e) => mq_error_response(&e),
        }
    }

    async fn handle_describe_broker(
        &self,
        state: &Arc<tokio::sync::RwLock<MqState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_broker_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let broker_id = &input.broker_id;
        let state = state.read().await;
        match state.describe_broker(broker_id) {
            Ok(broker) => {
                let user_summaries: Vec<wire::UserSummary> = broker
                    .users
                    .values()
                    .map(|u| wire::UserSummary {
                        username: Some(u.username.clone()),
                        ..Default::default()
                    })
                    .collect();
                wire::serialize_describe_broker_response(&wire::DescribeBrokerResponse {
                    broker_id: Some(broker.broker_id.clone()),
                    broker_name: Some(broker.broker_name.clone()),
                    broker_arn: Some(broker.broker_arn.clone()),
                    broker_state: Some(broker.broker_state.clone()),
                    engine_type: Some(broker.engine_type.clone()),
                    engine_version: Some(broker.engine_version.clone()),
                    host_instance_type: Some(broker.host_instance_type.clone()),
                    deployment_mode: Some(broker.deployment_mode.clone()),
                    created: Some(broker.created.to_rfc3339()),
                    publicly_accessible: Some(broker.publicly_accessible),
                    auto_minor_version_upgrade: Some(broker.auto_minor_version_upgrade),
                    broker_instances: Some(vec![]),
                    configurations: Some(wire::Configurations {
                        current: None,
                        history: Some(vec![]),
                        pending: None,
                    }),
                    logs: Some(wire::LogsSummary {
                        audit: Some(false),
                        general: Some(false),
                        general_log_group: Some(String::new()),
                        ..Default::default()
                    }),
                    maintenance_window_start_time: Some(wire::WeeklyStartTime {
                        day_of_week: "MONDAY".to_string(),
                        time_of_day: "00:00".to_string(),
                        time_zone: Some("UTC".to_string()),
                    }),
                    users: Some(user_summaries),
                    tags: Some(broker.tags.clone()),
                    subnet_ids: Some(vec![]),
                    security_groups: Some(vec![]),
                    storage_type: Some("EFS".to_string()),
                    authentication_strategy: Some("SIMPLE".to_string()),
                    ..Default::default()
                })
            }
            Err(e) => mq_error_response(&e),
        }
    }

    async fn handle_delete_broker(
        &self,
        state: &Arc<tokio::sync::RwLock<MqState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_broker_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let mut state = state.write().await;
        match state.delete_broker(&input.broker_id) {
            Ok(id) => wire::serialize_delete_broker_response(&wire::DeleteBrokerResponse {
                broker_id: Some(id),
            }),
            Err(e) => mq_error_response(&e),
        }
    }

    async fn handle_list_brokers(
        &self,
        state: &Arc<tokio::sync::RwLock<MqState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_list_brokers_request(request, labels, query) {
            return rest_json_error(400, "BadRequestException", &e);
        }
        let state = state.read().await;
        let summaries = state.list_brokers();
        let entries: Vec<wire::BrokerSummary> = summaries
            .iter()
            .map(|b| wire::BrokerSummary {
                broker_arn: Some(b.broker_arn.clone()),
                broker_id: Some(b.broker_id.clone()),
                broker_name: Some(b.broker_name.clone()),
                broker_state: Some(b.broker_state.clone()),
                engine_type: Some(b.engine_type.clone()),
                deployment_mode: Some(b.deployment_mode.clone()),
                created: Some(b.created.to_rfc3339()),
                host_instance_type: Some(b.host_instance_type.clone()),
            })
            .collect();
        wire::serialize_list_brokers_response(&wire::ListBrokersResponse {
            broker_summaries: Some(entries),
            ..Default::default()
        })
    }

    // --- Configuration handlers ---

    async fn handle_create_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<MqState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_configuration_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        if input.name.is_empty() {
            return rest_json_error(400, "BadRequestException", "Missing required field: name");
        }
        let engine_type = if input.engine_type.is_empty() {
            "ACTIVEMQ"
        } else {
            input.engine_type.as_str()
        };
        let engine_version = input.engine_version.as_deref().unwrap_or("5.17.6");
        let tags = input.tags;

        let mut state = state.write().await;
        match state.create_configuration(
            &input.name,
            engine_type,
            engine_version,
            account_id,
            region,
            tags,
        ) {
            Ok(config) => {
                let latest = config.revisions.last().unwrap();
                wire::serialize_create_configuration_response(&wire::CreateConfigurationResponse {
                    arn: Some(config.arn.clone()),
                    authentication_strategy: Some(config.authentication_strategy.clone()),
                    created: Some(config.created.to_rfc3339()),
                    id: Some(config.id.clone()),
                    latest_revision: Some(wire::ConfigurationRevision {
                        created: Some(latest.created.to_rfc3339()),
                        description: Some(latest.description.clone()),
                        revision: Some(latest.revision),
                    }),
                    name: Some(config.name.clone()),
                })
            }
            Err(e) => mq_error_response(&e),
        }
    }

    async fn handle_describe_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<MqState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_configuration_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let state = state.read().await;
        match state.describe_configuration(&input.configuration_id) {
            Ok(config) => {
                let latest = config.revisions.last().unwrap();
                wire::serialize_describe_configuration_response(
                    &wire::DescribeConfigurationResponse {
                        arn: Some(config.arn.clone()),
                        authentication_strategy: Some(config.authentication_strategy.clone()),
                        created: Some(config.created.to_rfc3339()),
                        description: Some(config.description.clone()),
                        engine_type: Some(config.engine_type.clone()),
                        engine_version: Some(config.engine_version.clone()),
                        id: Some(config.id.clone()),
                        latest_revision: Some(wire::ConfigurationRevision {
                            created: Some(latest.created.to_rfc3339()),
                            description: Some(latest.description.clone()),
                            revision: Some(latest.revision),
                        }),
                        name: Some(config.name.clone()),
                        tags: Some(config.tags.clone()),
                    },
                )
            }
            Err(e) => mq_error_response(&e),
        }
    }

    async fn handle_describe_configuration_revision(
        &self,
        state: &Arc<tokio::sync::RwLock<MqState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_describe_configuration_revision_request(request, labels, query)
            {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "BadRequestException", &e),
            };
        let revision: i32 = match input.configuration_revision.parse() {
            Ok(r) => r,
            Err(_) => {
                return rest_json_error(400, "BadRequestException", "Invalid revision number");
            }
        };
        let config_id = &input.configuration_id;
        let state = state.read().await;
        match state.describe_configuration_revision(config_id, revision) {
            Ok((_config, rev)) => wire::serialize_describe_configuration_revision_response(
                &wire::DescribeConfigurationRevisionResponse {
                    configuration_id: Some(config_id.to_string()),
                    created: Some(rev.created.to_rfc3339()),
                    data: Some(rev.data.clone()),
                    description: Some(rev.description.clone()),
                },
            ),
            Err(e) => mq_error_response(&e),
        }
    }

    async fn handle_update_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<MqState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_configuration_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let description = input.description.as_deref();

        let mut state = state.write().await;
        match state.update_configuration(&input.configuration_id, &input.data, description) {
            Ok(config) => {
                let latest = config.revisions.last().unwrap();
                wire::serialize_update_configuration_response(&wire::UpdateConfigurationResponse {
                    arn: Some(config.arn.clone()),
                    created: Some(latest.created.to_rfc3339()),
                    id: Some(config.id.clone()),
                    latest_revision: Some(wire::ConfigurationRevision {
                        created: Some(latest.created.to_rfc3339()),
                        description: Some(latest.description.clone()),
                        revision: Some(latest.revision),
                    }),
                    name: Some(config.name.clone()),
                    ..Default::default()
                })
            }
            Err(e) => mq_error_response(&e),
        }
    }

    async fn handle_list_configurations(
        &self,
        state: &Arc<tokio::sync::RwLock<MqState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_list_configurations_request(request, labels, query) {
            return rest_json_error(400, "BadRequestException", &e);
        }
        let state = state.read().await;
        let configs = state.list_configurations();
        let entries: Vec<wire::Configuration> = configs
            .iter()
            .map(|c| {
                let latest = c.revisions.last().unwrap();
                wire::Configuration {
                    arn: Some(c.arn.clone()),
                    authentication_strategy: Some(c.authentication_strategy.clone()),
                    created: Some(c.created.to_rfc3339()),
                    description: Some(c.description.clone()),
                    engine_type: Some(c.engine_type.clone()),
                    engine_version: Some(c.engine_version.clone()),
                    id: Some(c.id.clone()),
                    latest_revision: Some(wire::ConfigurationRevision {
                        created: Some(latest.created.to_rfc3339()),
                        description: Some(latest.description.clone()),
                        revision: Some(latest.revision),
                    }),
                    name: Some(c.name.clone()),
                    tags: Some(c.tags.clone()),
                }
            })
            .collect();
        wire::serialize_list_configurations_response(&wire::ListConfigurationsResponse {
            configurations: Some(entries),
            ..Default::default()
        })
    }

    async fn handle_list_configuration_revisions(
        &self,
        state: &Arc<tokio::sync::RwLock<MqState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_list_configuration_revisions_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "BadRequestException", &e),
            };
        let config_id = &input.configuration_id;
        let state = state.read().await;
        match state.list_configuration_revisions(config_id) {
            Ok(config) => {
                let revisions: Vec<wire::ConfigurationRevision> = config
                    .revisions
                    .iter()
                    .map(|r| wire::ConfigurationRevision {
                        created: Some(r.created.to_rfc3339()),
                        description: Some(r.description.clone()),
                        revision: Some(r.revision),
                    })
                    .collect();
                wire::serialize_list_configuration_revisions_response(
                    &wire::ListConfigurationRevisionsResponse {
                        configuration_id: Some(config_id.to_string()),
                        revisions: Some(revisions),
                        ..Default::default()
                    },
                )
            }
            Err(e) => mq_error_response(&e),
        }
    }

    // --- Tag handlers ---

    async fn handle_create_tags(
        &self,
        state: &Arc<tokio::sync::RwLock<MqState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_tags_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let tags: HashMap<String, String> = input.tags.unwrap_or_default();

        let mut state = state.write().await;
        match state.create_tags(&input.resource_arn, tags) {
            Ok(()) => wire::serialize_create_tags_response(),
            Err(e) => mq_error_response(&e),
        }
    }

    async fn handle_delete_tags(
        &self,
        state: &Arc<tokio::sync::RwLock<MqState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        raw_query: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_tags_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        // The AWS SDK sends tag keys as repeated query params (?tagKeys=k1&tagKeys=k2),
        // but parse_query_string keeps only the last value, so parse them from the raw
        // query string here. Fall back to whatever the wire deserializer parsed (which
        // assumes a comma-separated single param) if no repeated params were found.
        let mut tag_keys = extract_query_param_list(raw_query, "tagKeys");
        if tag_keys.is_empty() {
            tag_keys = input.tag_keys;
        }
        let mut state = state.write().await;
        match state.delete_tags(&input.resource_arn, &tag_keys) {
            Ok(()) => wire::serialize_delete_tags_response(),
            Err(e) => mq_error_response(&e),
        }
    }

    async fn handle_list_tags(
        &self,
        state: &Arc<tokio::sync::RwLock<MqState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let state = state.read().await;
        match state.list_tags(&input.resource_arn) {
            Ok(tags) => {
                wire::serialize_list_tags_response(&wire::ListTagsResponse { tags: Some(tags) })
            }
            Err(e) => mq_error_response(&e),
        }
    }

    // --- User handlers ---

    async fn handle_create_user(
        &self,
        state: &Arc<tokio::sync::RwLock<MqState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_user_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let console_access = input.console_access.unwrap_or(false);
        let groups = input.groups.unwrap_or_default();

        let mut state = state.write().await;
        match state.create_user(&input.broker_id, &input.username, console_access, groups) {
            Ok(()) => wire::serialize_create_user_response(&wire::CreateUserResponse {}),
            Err(e) => mq_error_response(&e),
        }
    }

    async fn handle_delete_user(
        &self,
        state: &Arc<tokio::sync::RwLock<MqState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_user_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let mut state = state.write().await;
        match state.delete_user(&input.broker_id, &input.username) {
            Ok(()) => wire::serialize_delete_user_response(&wire::DeleteUserResponse {}),
            Err(e) => mq_error_response(&e),
        }
    }

    async fn handle_describe_user(
        &self,
        state: &Arc<tokio::sync::RwLock<MqState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_user_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let state = state.read().await;
        match state.describe_user(&input.broker_id, &input.username) {
            Ok(user) => wire::serialize_describe_user_response(&wire::DescribeUserResponse {
                broker_id: Some(input.broker_id.clone()),
                console_access: Some(user.console_access),
                groups: Some(user.groups.clone()),
                username: Some(user.username.clone()),
                replication_user: Some(user.replication_user),
                ..Default::default()
            }),
            Err(e) => mq_error_response(&e),
        }
    }

    async fn handle_list_users(
        &self,
        state: &Arc<tokio::sync::RwLock<MqState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_users_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let broker_id = &input.broker_id;
        let state = state.read().await;
        match state.list_users(broker_id) {
            Ok(users) => {
                let summaries: Vec<wire::UserSummary> = users
                    .iter()
                    .map(|u| wire::UserSummary {
                        username: Some(u.username.clone()),
                        ..Default::default()
                    })
                    .collect();
                wire::serialize_list_users_response(&wire::ListUsersResponse {
                    broker_id: Some(broker_id.to_string()),
                    users: Some(summaries),
                    ..Default::default()
                })
            }
            Err(e) => mq_error_response(&e),
        }
    }

    // --- Reboot handler ---

    async fn handle_reboot_broker(
        &self,
        state: &Arc<tokio::sync::RwLock<MqState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_reboot_broker_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let state = state.read().await;
        match state.reboot_broker(&input.broker_id) {
            Ok(()) => wire::serialize_reboot_broker_response(&wire::RebootBrokerResponse {}),
            Err(e) => mq_error_response(&e),
        }
    }

    // --- UpdateBroker handler ---

    async fn handle_update_broker(
        &self,
        state: &Arc<tokio::sync::RwLock<MqState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_broker_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let auto_minor = input.auto_minor_version_upgrade;
        let engine_version = input.engine_version.as_deref();

        let mut state = state.write().await;
        match state.update_broker(&input.broker_id, auto_minor, engine_version) {
            Ok(broker) => wire::serialize_update_broker_response(&wire::UpdateBrokerResponse {
                broker_id: Some(broker.broker_id.clone()),
                engine_version: Some(broker.engine_version.clone()),
                auto_minor_version_upgrade: Some(broker.auto_minor_version_upgrade),
                ..Default::default()
            }),
            Err(e) => mq_error_response(&e),
        }
    }

    // --- UpdateUser handler ---

    async fn handle_update_user(
        &self,
        state: &Arc<tokio::sync::RwLock<MqState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_user_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let console_access = input.console_access;
        let groups = input.groups;

        let mut state = state.write().await;
        match state.update_user(&input.broker_id, &input.username, console_access, groups) {
            Ok(()) => wire::serialize_update_user_response(&wire::UpdateUserResponse {}),
            Err(e) => mq_error_response(&e),
        }
    }

    // --- DeleteConfiguration handler ---

    async fn handle_delete_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<MqState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_configuration_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let mut state = state.write().await;
        match state.delete_configuration(&input.configuration_id) {
            Ok(id) => {
                wire::serialize_delete_configuration_response(&wire::DeleteConfigurationResponse {
                    configuration_id: Some(id),
                })
            }
            Err(e) => mq_error_response(&e),
        }
    }

    // --- DescribeBrokerInstanceOptions handler ---

    // STUB[no-telemetry]: Broker instance options are driven by real AWS infrastructure
    //   capacity and availability data; the mock returns an empty list.
    async fn handle_describe_broker_instance_options(
        &self,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) =
            wire::deserialize_describe_broker_instance_options_request(request, labels, query)
        {
            return rest_json_error(400, "BadRequestException", &e);
        }
        wire::serialize_describe_broker_instance_options_response(
            &wire::DescribeBrokerInstanceOptionsResponse {
                broker_instance_options: Some(vec![]),
                ..Default::default()
            },
        )
    }

    // --- Promote handler ---

    async fn handle_promote(
        &self,
        state: &Arc<tokio::sync::RwLock<MqState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_promote_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "BadRequestException", &e),
        };
        let mode = if input.mode.is_empty() {
            "SWITCHOVER"
        } else {
            input.mode.as_str()
        };
        let state = state.read().await;
        match state.promote_broker(&input.broker_id, mode) {
            Ok(id) => wire::serialize_promote_response(&wire::PromoteResponse {
                broker_id: Some(id),
            }),
            Err(e) => mq_error_response(&e),
        }
    }

    // --- DescribeBrokerEngineTypes handler ---

    async fn handle_describe_broker_engine_types(
        &self,
        state: &Arc<tokio::sync::RwLock<MqState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) =
            wire::deserialize_describe_broker_engine_types_request(request, labels, query)
        {
            return rest_json_error(400, "BadRequestException", &e);
        }
        let state = state.read().await;
        let engine_types = state.describe_broker_engine_types();
        let entries: Vec<wire::BrokerEngineType> = engine_types
            .iter()
            .map(|(et, versions)| wire::BrokerEngineType {
                engine_type: Some(et.clone()),
                engine_versions: Some(
                    versions
                        .iter()
                        .map(|v| wire::EngineVersion {
                            name: Some(v.clone()),
                        })
                        .collect(),
                ),
            })
            .collect();
        wire::serialize_describe_broker_engine_types_response(
            &wire::DescribeBrokerEngineTypesResponse {
                broker_engine_types: Some(entries),
                ..Default::default()
            },
        )
    }
}

fn extract_path_and_query(uri: &str) -> (String, String) {
    let path_and_query = if let Some(idx) = uri.find("amazonaws.com") {
        &uri[idx + "amazonaws.com".len()..]
    } else {
        uri
    };
    match path_and_query.find('?') {
        Some(q) => (
            path_and_query[..q].to_string(),
            path_and_query[q + 1..].to_string(),
        ),
        None => (path_and_query.to_string(), String::new()),
    }
}

fn extract_query_param_list(query_string: &str, param_name: &str) -> Vec<String> {
    if query_string.is_empty() {
        return Vec::new();
    }
    query_string
        .split('&')
        .filter_map(|part| {
            let (key, value) = part.split_once('=')?;
            if key == param_name {
                Some(percent_decode(value))
            } else {
                None
            }
        })
        .collect()
}

fn percent_decode(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut bytes = s.bytes();
    while let Some(b) = bytes.next() {
        match b {
            b'%' => {
                let hi = bytes.next().and_then(hex_val);
                let lo = bytes.next().and_then(hex_val);
                if let (Some(hi), Some(lo)) = (hi, lo) {
                    result.push((hi << 4 | lo) as char);
                }
            }
            b'+' => result.push(' '),
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

fn mq_error_response(err: &MqError) -> MockResponse {
    let (status, error_type) = match err {
        MqError::BrokerNameConflict { .. } => (409u16, "ConflictException"),
        MqError::BrokerNotFound { .. } => (404, "NotFoundException"),
        MqError::ConfigurationNotFound { .. } => (404, "NotFoundException"),
        MqError::ConfigurationRevisionNotFound { .. } => (404, "NotFoundException"),
        MqError::ResourceNotFound { .. } => (404, "NotFoundException"),
        MqError::UserConflict { .. } => (409, "ConflictException"),
        MqError::UserNotFound { .. } => (404, "NotFoundException"),
        MqError::UserNotFoundShort { .. } => (404, "NotFoundException"),
    };
    let body = json!({
        "errorAttribute": error_type,
        "message": err.to_string(),
    });
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}

fn rest_json_error(status: u16, code: &str, message: &str) -> MockResponse {
    let body = json!({
        "errorAttribute": code,
        "message": message,
    });
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers.insert(X_AMZN_ERRORTYPE, code.parse().unwrap());
    resp
}
