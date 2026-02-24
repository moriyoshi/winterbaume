use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::Value;
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
    StatefulService,
};

use crate::model;
use crate::state::{AppRunnerError, AppRunnerState};
use crate::views::AppRunnerStateView;
use crate::wire;

pub struct AppRunnerService {
    pub(crate) state: Arc<BackendState<AppRunnerState>>,
    pub(crate) notifier: StateChangeNotifier<AppRunnerStateView>,
}

impl AppRunnerService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for AppRunnerService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for AppRunnerService {
    fn service_name(&self) -> &str {
        "apprunner"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://apprunner\..*\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

fn error_response(status: u16, error_type: &str, message: &str) -> MockResponse {
    MockResponse::error(status, error_type, message)
}

fn service_error_response(e: &AppRunnerError) -> MockResponse {
    let (status, error_type) = match e {
        AppRunnerError::ServiceAlreadyExists(_) => (400, "InvalidStateException"),
        AppRunnerError::ConnectionAlreadyExists(_) => (400, "InvalidStateException"),
        AppRunnerError::ServiceNotFound(_) => (400, "ResourceNotFoundException"),
        AppRunnerError::ConnectionNotFound(_) => (400, "ResourceNotFoundException"),
        AppRunnerError::AutoScalingConfigNotFound(_) => (400, "ResourceNotFoundException"),
    };
    error_response(status, error_type, &e.to_string())
}

fn service_to_model(s: &crate::types::AppRunnerService) -> model::Service {
    model::Service {
        service_id: Some(s.service_id.clone()),
        service_name: Some(s.service_name.clone()),
        service_arn: Some(s.service_arn.clone()),
        service_url: Some(s.service_url.clone()),
        status: Some(s.status.clone()),
        created_at: Some(s.created_at),
        updated_at: Some(s.updated_at),
        ..Default::default()
    }
}

fn service_to_summary(s: &crate::types::AppRunnerService) -> model::ServiceSummary {
    model::ServiceSummary {
        service_id: Some(s.service_id.clone()),
        service_name: Some(s.service_name.clone()),
        service_arn: Some(s.service_arn.clone()),
        service_url: Some(s.service_url.clone()),
        status: Some(s.status.clone()),
        created_at: Some(s.created_at),
        updated_at: Some(s.updated_at),
    }
}

fn connection_to_model(c: &crate::types::Connection) -> model::Connection {
    model::Connection {
        connection_name: Some(c.connection_name.clone()),
        connection_arn: Some(c.connection_arn.clone()),
        provider_type: Some(c.provider_type.clone()),
        status: Some(c.status.clone()),
        created_at: Some(c.created_at),
    }
}

fn connection_to_summary(c: &crate::types::Connection) -> model::ConnectionSummary {
    model::ConnectionSummary {
        connection_name: Some(c.connection_name.clone()),
        connection_arn: Some(c.connection_arn.clone()),
        provider_type: Some(c.provider_type.clone()),
        status: Some(c.status.clone()),
        created_at: Some(c.created_at),
    }
}

fn asc_to_model(c: &crate::types::AutoScalingConfig) -> model::AutoScalingConfiguration {
    model::AutoScalingConfiguration {
        auto_scaling_configuration_arn: Some(c.arn.clone()),
        auto_scaling_configuration_name: Some(c.name.clone()),
        auto_scaling_configuration_revision: Some(c.revision),
        status: Some(c.status.clone()),
        is_default: Some(c.is_default),
        latest: Some(c.latest),
        min_size: Some(c.min_size),
        max_size: Some(c.max_size),
        max_concurrency: Some(c.max_concurrency),
        created_at: Some(c.created_at),
        ..Default::default()
    }
}

async fn asc_to_summary(
    c: &crate::types::AutoScalingConfig,
) -> model::AutoScalingConfigurationSummary {
    model::AutoScalingConfigurationSummary {
        auto_scaling_configuration_arn: Some(c.arn.clone()),
        auto_scaling_configuration_name: Some(c.name.clone()),
        auto_scaling_configuration_revision: Some(c.revision),
        is_default: Some(c.is_default),
        created_at: Some(c.created_at),
        ..Default::default()
    }
}

impl AppRunnerService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let account_id = DEFAULT_ACCOUNT_ID;
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);

        let target = match request
            .headers
            .get("x-amz-target")
            .and_then(|v| v.to_str().ok())
        {
            Some(t) => t.to_string(),
            None => {
                return error_response(400, "InvalidAction", "Missing X-Amz-Target header");
            }
        };

        let action = target.split('.').next_back().unwrap_or(&target).to_string();

        let body: Value = if request.body.is_empty() {
            Value::Object(Default::default())
        } else {
            match serde_json::from_slice(&request.body) {
                Ok(v) => v,
                Err(_) => {
                    return error_response(400, "InvalidInput", "Invalid JSON body");
                }
            }
        };

        let state = self.state.get(account_id, &region);

        let is_mutating = matches!(
            action.as_str(),
            "CreateService"
                | "DeleteService"
                | "UpdateService"
                | "PauseService"
                | "ResumeService"
                | "StartDeployment"
                | "CreateConnection"
                | "DeleteConnection"
                | "CreateAutoScalingConfiguration"
                | "DeleteAutoScalingConfiguration"
                | "UpdateDefaultAutoScalingConfiguration"
                | "TagResource"
                | "UntagResource"
        );

        let response = match action.as_str() {
            "CreateService" => {
                self.handle_create_service(&state, &body, account_id, &region)
                    .await
            }
            "DescribeService" => self.handle_describe_service(&state, &body).await,
            "ListServices" => self.handle_list_services(&state).await,
            "DeleteService" => self.handle_delete_service(&state, &body).await,
            "UpdateService" => self.handle_update_service(&state, &body).await,
            "PauseService" => self.handle_pause_service(&state, &body).await,
            "ResumeService" => self.handle_resume_service(&state, &body).await,
            "StartDeployment" => self.handle_start_deployment(&state, &body).await,
            "CreateConnection" => {
                self.handle_create_connection(&state, &body, account_id, &region)
                    .await
            }
            "ListConnections" => self.handle_list_connections(&state).await,
            "DeleteConnection" => self.handle_delete_connection(&state, &body).await,
            "CreateAutoScalingConfiguration" => {
                self.handle_create_auto_scaling_configuration(&state, &body, account_id, &region)
                    .await
            }
            "DescribeAutoScalingConfiguration" => {
                self.handle_describe_auto_scaling_configuration(&state, &body)
                    .await
            }
            "ListAutoScalingConfigurations" => {
                self.handle_list_auto_scaling_configurations(&state).await
            }
            "DeleteAutoScalingConfiguration" => {
                self.handle_delete_auto_scaling_configuration(&state, &body)
                    .await
            }
            "UpdateDefaultAutoScalingConfiguration" => {
                self.handle_update_default_auto_scaling_configuration(&state, &body)
                    .await
            }
            "TagResource" => self.handle_tag_resource(&state, &body).await,
            "UntagResource" => self.handle_untag_resource(&state, &body).await,
            "ListTagsForResource" => self.handle_list_tags_for_resource(&state, &body).await,
            "AssociateCustomDomain" => error_response(
                501,
                "NotImplemented",
                "AssociateCustomDomain not implemented",
            ),
            "CreateObservabilityConfiguration" => error_response(
                501,
                "NotImplemented",
                "CreateObservabilityConfiguration not implemented",
            ),
            "CreateVpcConnector" => {
                error_response(501, "NotImplemented", "CreateVpcConnector not implemented")
            }
            "CreateVpcIngressConnection" => error_response(
                501,
                "NotImplemented",
                "CreateVpcIngressConnection not implemented",
            ),
            "DeleteObservabilityConfiguration" => error_response(
                501,
                "NotImplemented",
                "DeleteObservabilityConfiguration not implemented",
            ),
            "DeleteVpcConnector" => {
                error_response(501, "NotImplemented", "DeleteVpcConnector not implemented")
            }
            "DeleteVpcIngressConnection" => error_response(
                501,
                "NotImplemented",
                "DeleteVpcIngressConnection not implemented",
            ),
            "DescribeCustomDomains" => error_response(
                501,
                "NotImplemented",
                "DescribeCustomDomains not implemented",
            ),
            "DescribeObservabilityConfiguration" => error_response(
                501,
                "NotImplemented",
                "DescribeObservabilityConfiguration not implemented",
            ),
            "DescribeVpcConnector" => error_response(
                501,
                "NotImplemented",
                "DescribeVpcConnector not implemented",
            ),
            "DescribeVpcIngressConnection" => error_response(
                501,
                "NotImplemented",
                "DescribeVpcIngressConnection not implemented",
            ),
            "DisassociateCustomDomain" => error_response(
                501,
                "NotImplemented",
                "DisassociateCustomDomain not implemented",
            ),
            "ListObservabilityConfigurations" => error_response(
                501,
                "NotImplemented",
                "ListObservabilityConfigurations not implemented",
            ),
            "ListOperations" => {
                error_response(501, "NotImplemented", "ListOperations not implemented")
            }
            "ListServicesForAutoScalingConfiguration" => error_response(
                501,
                "NotImplemented",
                "ListServicesForAutoScalingConfiguration not implemented",
            ),
            "ListVpcConnectors" => {
                error_response(501, "NotImplemented", "ListVpcConnectors not implemented")
            }
            "ListVpcIngressConnections" => error_response(
                501,
                "NotImplemented",
                "ListVpcIngressConnections not implemented",
            ),
            "UpdateVpcIngressConnection" => error_response(
                501,
                "NotImplemented",
                "UpdateVpcIngressConnection not implemented",
            ),
            _ => error_response(400, "InvalidAction", &format!("Unknown action: {action}")),
        };

        if is_mutating && response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }

        response
    }

    async fn handle_create_service(
        &self,
        state: &Arc<tokio::sync::RwLock<AppRunnerState>>,
        body: &Value,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let req: model::CreateServiceRequest = match serde_json::from_value(body.clone()) {
            Ok(r) => r,
            Err(_) => {
                return error_response(400, "ValidationException", "Invalid CreateService request");
            }
        };
        let tags = req
            .tags
            .unwrap_or_default()
            .into_iter()
            .filter_map(|t| Some((t.key?, t.value?)))
            .collect();
        let mut state = state.write().await;
        match state.create_service(&req.service_name, tags, account_id, region) {
            Ok(svc) => wire::serialize_create_service_response(&model::CreateServiceResponse {
                service: Some(service_to_model(svc)),
                operation_id: Some(uuid::Uuid::new_v4().to_string()),
            }),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_describe_service(
        &self,
        state: &Arc<tokio::sync::RwLock<AppRunnerState>>,
        body: &Value,
    ) -> MockResponse {
        let req: model::DescribeServiceRequest = match serde_json::from_value(body.clone()) {
            Ok(r) => r,
            Err(_) => {
                return error_response(
                    400,
                    "ValidationException",
                    "Invalid DescribeService request",
                );
            }
        };
        let state = state.read().await;
        match state.describe_service(&req.service_arn) {
            Ok(svc) => wire::serialize_describe_service_response(&model::DescribeServiceResponse {
                service: Some(service_to_model(svc)),
            }),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_list_services(
        &self,
        state: &Arc<tokio::sync::RwLock<AppRunnerState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let summaries = state
            .list_services()
            .into_iter()
            .map(service_to_summary)
            .collect();
        wire::serialize_list_services_response(&model::ListServicesResponse {
            service_summary_list: Some(summaries),
            next_token: None,
        })
    }

    async fn handle_delete_service(
        &self,
        state: &Arc<tokio::sync::RwLock<AppRunnerState>>,
        body: &Value,
    ) -> MockResponse {
        let req: model::DeleteServiceRequest = match serde_json::from_value(body.clone()) {
            Ok(r) => r,
            Err(_) => {
                return error_response(400, "ValidationException", "Invalid DeleteService request");
            }
        };
        let mut state = state.write().await;
        match state.delete_service(&req.service_arn) {
            Ok(svc) => wire::serialize_delete_service_response(&model::DeleteServiceResponse {
                service: Some(service_to_model(&svc)),
                operation_id: Some(uuid::Uuid::new_v4().to_string()),
            }),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_update_service(
        &self,
        state: &Arc<tokio::sync::RwLock<AppRunnerState>>,
        body: &Value,
    ) -> MockResponse {
        let req: model::UpdateServiceRequest = match serde_json::from_value(body.clone()) {
            Ok(r) => r,
            Err(_) => {
                return error_response(400, "ValidationException", "Invalid UpdateService request");
            }
        };
        let mut state = state.write().await;
        match state.update_service(&req.service_arn) {
            Ok(svc) => wire::serialize_update_service_response(&model::UpdateServiceResponse {
                service: Some(service_to_model(svc)),
                operation_id: Some(uuid::Uuid::new_v4().to_string()),
            }),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_pause_service(
        &self,
        state: &Arc<tokio::sync::RwLock<AppRunnerState>>,
        body: &Value,
    ) -> MockResponse {
        let req: model::PauseServiceRequest = match serde_json::from_value(body.clone()) {
            Ok(r) => r,
            Err(_) => {
                return error_response(400, "ValidationException", "Invalid PauseService request");
            }
        };
        let mut state = state.write().await;
        match state.pause_service(&req.service_arn) {
            Ok(svc) => wire::serialize_pause_service_response(&model::PauseServiceResponse {
                service: Some(service_to_model(svc)),
                operation_id: Some(uuid::Uuid::new_v4().to_string()),
            }),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_resume_service(
        &self,
        state: &Arc<tokio::sync::RwLock<AppRunnerState>>,
        body: &Value,
    ) -> MockResponse {
        let req: model::ResumeServiceRequest = match serde_json::from_value(body.clone()) {
            Ok(r) => r,
            Err(_) => {
                return error_response(400, "ValidationException", "Invalid ResumeService request");
            }
        };
        let mut state = state.write().await;
        match state.resume_service(&req.service_arn) {
            Ok(svc) => wire::serialize_resume_service_response(&model::ResumeServiceResponse {
                service: Some(service_to_model(svc)),
                operation_id: Some(uuid::Uuid::new_v4().to_string()),
            }),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_start_deployment(
        &self,
        state: &Arc<tokio::sync::RwLock<AppRunnerState>>,
        body: &Value,
    ) -> MockResponse {
        let req: model::StartDeploymentRequest = match serde_json::from_value(body.clone()) {
            Ok(r) => r,
            Err(_) => {
                return error_response(
                    400,
                    "ValidationException",
                    "Invalid StartDeployment request",
                );
            }
        };
        let state = state.read().await;
        match state.describe_service(&req.service_arn) {
            Ok(_) => wire::serialize_start_deployment_response(&model::StartDeploymentResponse {
                operation_id: Some(uuid::Uuid::new_v4().to_string()),
            }),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_create_connection(
        &self,
        state: &Arc<tokio::sync::RwLock<AppRunnerState>>,
        body: &Value,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let req: model::CreateConnectionRequest = match serde_json::from_value(body.clone()) {
            Ok(r) => r,
            Err(_) => {
                return error_response(
                    400,
                    "ValidationException",
                    "Invalid CreateConnection request",
                );
            }
        };
        let tags = req
            .tags
            .unwrap_or_default()
            .into_iter()
            .filter_map(|t| Some((t.key?, t.value?)))
            .collect();
        let mut state = state.write().await;
        match state.create_connection(
            &req.connection_name,
            &req.provider_type,
            tags,
            account_id,
            region,
        ) {
            Ok(conn) => {
                wire::serialize_create_connection_response(&model::CreateConnectionResponse {
                    connection: Some(connection_to_model(conn)),
                })
            }
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_list_connections(
        &self,
        state: &Arc<tokio::sync::RwLock<AppRunnerState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let summaries = state
            .list_connections()
            .into_iter()
            .map(connection_to_summary)
            .collect();
        wire::serialize_list_connections_response(&model::ListConnectionsResponse {
            connection_summary_list: Some(summaries),
            next_token: None,
        })
    }

    async fn handle_delete_connection(
        &self,
        state: &Arc<tokio::sync::RwLock<AppRunnerState>>,
        body: &Value,
    ) -> MockResponse {
        let req: model::DeleteConnectionRequest = match serde_json::from_value(body.clone()) {
            Ok(r) => r,
            Err(_) => {
                return error_response(
                    400,
                    "ValidationException",
                    "Invalid DeleteConnection request",
                );
            }
        };
        let mut state = state.write().await;
        match state.delete_connection(&req.connection_arn) {
            Ok(conn) => {
                wire::serialize_delete_connection_response(&model::DeleteConnectionResponse {
                    connection: Some(connection_to_model(&conn)),
                })
            }
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_create_auto_scaling_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<AppRunnerState>>,
        body: &Value,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let req: model::CreateAutoScalingConfigurationRequest =
            match serde_json::from_value(body.clone()) {
                Ok(r) => r,
                Err(_) => {
                    return error_response(
                        400,
                        "ValidationException",
                        "Invalid CreateAutoScalingConfiguration request",
                    );
                }
            };
        let name = &req.auto_scaling_configuration_name;
        let min_size = req.min_size.unwrap_or(1);
        let max_size = req.max_size.unwrap_or(25);
        let max_concurrency = req.max_concurrency.unwrap_or(100);
        let tags = req
            .tags
            .unwrap_or_default()
            .into_iter()
            .filter_map(|t| Some((t.key?, t.value?)))
            .collect();
        let mut state = state.write().await;
        match state.create_auto_scaling_configuration(
            name,
            min_size,
            max_size,
            max_concurrency,
            tags,
            account_id,
            region,
        ) {
            Ok(cfg) => wire::serialize_create_auto_scaling_configuration_response(
                &model::CreateAutoScalingConfigurationResponse {
                    auto_scaling_configuration: Some(asc_to_model(cfg)),
                },
            ),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_describe_auto_scaling_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<AppRunnerState>>,
        body: &Value,
    ) -> MockResponse {
        let req: model::DescribeAutoScalingConfigurationRequest =
            match serde_json::from_value(body.clone()) {
                Ok(r) => r,
                Err(_) => {
                    return error_response(
                        400,
                        "ValidationException",
                        "Invalid DescribeAutoScalingConfiguration request",
                    );
                }
            };
        let state = state.read().await;
        match state.describe_auto_scaling_configuration(&req.auto_scaling_configuration_arn) {
            Ok(cfg) => wire::serialize_describe_auto_scaling_configuration_response(
                &model::DescribeAutoScalingConfigurationResponse {
                    auto_scaling_configuration: Some(asc_to_model(cfg)),
                },
            ),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_list_auto_scaling_configurations(
        &self,
        state: &Arc<tokio::sync::RwLock<AppRunnerState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let mut summaries = Vec::new();
        for cfg in state.list_auto_scaling_configurations().into_iter() {
            summaries.push(asc_to_summary(cfg).await);
        }
        wire::serialize_list_auto_scaling_configurations_response(
            &model::ListAutoScalingConfigurationsResponse {
                auto_scaling_configuration_summary_list: Some(summaries),
                next_token: None,
            },
        )
    }

    async fn handle_delete_auto_scaling_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<AppRunnerState>>,
        body: &Value,
    ) -> MockResponse {
        let req: model::DeleteAutoScalingConfigurationRequest =
            match serde_json::from_value(body.clone()) {
                Ok(r) => r,
                Err(_) => {
                    return error_response(
                        400,
                        "ValidationException",
                        "Invalid DeleteAutoScalingConfiguration request",
                    );
                }
            };
        let mut state = state.write().await;
        match state.delete_auto_scaling_configuration(&req.auto_scaling_configuration_arn) {
            Ok(cfg) => wire::serialize_delete_auto_scaling_configuration_response(
                &model::DeleteAutoScalingConfigurationResponse {
                    auto_scaling_configuration: Some(asc_to_model(&cfg)),
                },
            ),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_update_default_auto_scaling_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<AppRunnerState>>,
        body: &Value,
    ) -> MockResponse {
        let req: model::UpdateDefaultAutoScalingConfigurationRequest =
            match serde_json::from_value(body.clone()) {
                Ok(r) => r,
                Err(_) => {
                    return error_response(
                        400,
                        "ValidationException",
                        "Invalid UpdateDefaultAutoScalingConfiguration request",
                    );
                }
            };
        let mut state = state.write().await;
        match state.update_default_auto_scaling_configuration(&req.auto_scaling_configuration_arn) {
            Ok(cfg) => wire::serialize_update_default_auto_scaling_configuration_response(
                &model::UpdateDefaultAutoScalingConfigurationResponse {
                    auto_scaling_configuration: Some(asc_to_model(cfg)),
                },
            ),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<AppRunnerState>>,
        body: &Value,
    ) -> MockResponse {
        let req: model::TagResourceRequest = match serde_json::from_value(body.clone()) {
            Ok(r) => r,
            Err(_) => {
                return error_response(400, "ValidationException", "Invalid TagResource request");
            }
        };
        let tags = req
            .tags
            .into_iter()
            .filter_map(|t| Some((t.key?, t.value?)))
            .collect();
        let mut state = state.write().await;
        match state.tag_resource(&req.resource_arn, tags) {
            Ok(()) => wire::serialize_tag_resource_response(&model::TagResourceResponse {}),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<AppRunnerState>>,
        body: &Value,
    ) -> MockResponse {
        let req: model::UntagResourceRequest = match serde_json::from_value(body.clone()) {
            Ok(r) => r,
            Err(_) => {
                return error_response(400, "ValidationException", "Invalid UntagResource request");
            }
        };
        let mut state = state.write().await;
        match state.untag_resource(&req.resource_arn, &req.tag_keys) {
            Ok(()) => wire::serialize_untag_resource_response(&model::UntagResourceResponse {}),
            Err(e) => service_error_response(&e),
        }
    }

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<AppRunnerState>>,
        body: &Value,
    ) -> MockResponse {
        let req: model::ListTagsForResourceRequest = match serde_json::from_value(body.clone()) {
            Ok(r) => r,
            Err(_) => {
                return error_response(
                    400,
                    "ValidationException",
                    "Invalid ListTagsForResource request",
                );
            }
        };
        let state = state.read().await;
        match state.list_tags_for_resource(&req.resource_arn) {
            Ok(tags) => {
                let tag_list = tags
                    .into_iter()
                    .map(|(k, v)| model::Tag {
                        key: Some(k),
                        value: Some(v),
                    })
                    .collect();
                wire::serialize_list_tags_for_resource_response(
                    &model::ListTagsForResourceResponse {
                        tags: Some(tag_list),
                    },
                )
            }
            Err(e) => service_error_response(&e),
        }
    }
}
