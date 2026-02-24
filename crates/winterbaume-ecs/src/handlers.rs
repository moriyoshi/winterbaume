use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
};

use crate::state::{EcsError, EcsState};
use crate::types::*;
use crate::views::EcsStateView;
use crate::wire;

fn convert_tags(tags: Option<Vec<wire::Tag>>) -> Vec<EcsTag> {
    tags.unwrap_or_default()
        .into_iter()
        .map(|t| EcsTag {
            key: t.key.unwrap_or_default(),
            value: t.value.unwrap_or_default(),
        })
        .collect()
}

fn convert_tags_required(tags: Vec<wire::Tag>) -> Vec<EcsTag> {
    tags.into_iter()
        .map(|t| EcsTag {
            key: t.key.unwrap_or_default(),
            value: t.value.unwrap_or_default(),
        })
        .collect()
}

fn convert_load_balancers(lbs: Option<Vec<wire::LoadBalancer>>) -> Vec<EcsLoadBalancer> {
    lbs.unwrap_or_default()
        .into_iter()
        .map(|lb| EcsLoadBalancer {
            target_group_arn: lb.target_group_arn,
            load_balancer_name: lb.load_balancer_name,
            container_name: lb.container_name,
            container_port: lb.container_port,
        })
        .collect()
}

fn convert_container_definitions(defs: Vec<wire::ContainerDefinition>) -> Vec<ContainerDefinition> {
    defs.into_iter()
        .map(|cd| {
            let port_mappings = cd
                .port_mappings
                .unwrap_or_default()
                .into_iter()
                .map(|pm| PortMapping {
                    container_port: pm.container_port.unwrap_or(0),
                    host_port: pm.host_port.unwrap_or(0),
                    protocol: pm.protocol.unwrap_or_else(|| "tcp".to_string()),
                })
                .collect();
            let environment = cd
                .environment
                .unwrap_or_default()
                .into_iter()
                .map(|e| EnvVar {
                    name: e.name.unwrap_or_default(),
                    value: e.value.unwrap_or_default(),
                })
                .collect();
            let log_configuration = cd.log_configuration.map(|lc| LogConfiguration {
                log_driver: lc.log_driver,
            });
            ContainerDefinition {
                name: cd.name.unwrap_or_else(|| "container".to_string()),
                image: cd.image.unwrap_or_default(),
                cpu: cd.cpu.unwrap_or(0),
                memory: cd.memory,
                memory_reservation: cd.memory_reservation,
                essential: cd.essential.unwrap_or(true),
                environment,
                log_configuration,
                port_mappings,
            }
        })
        .collect()
}

pub struct EcsService {
    pub(crate) state: Arc<BackendState<EcsState>>,
    pub(crate) notifier: StateChangeNotifier<EcsStateView>,
}

impl EcsService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }

    /// Returns sorted `(account_id, region)` pairs that have state.
    pub fn scopes_with_state(&self) -> Vec<(String, String)> {
        self.state.scopes_with_state()
    }
}

impl Default for EcsService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for EcsService {
    fn service_name(&self) -> &str {
        "ecs"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://ecs\..*\.amazonaws\.com",
            r"https?://ecs\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl EcsService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;

        let action = request
            .headers
            .get("x-amz-target")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.split('.').next_back())
            .map(|s| s.to_string());

        let action = match action {
            Some(a) => a,
            None => {
                return json_error_response(400, "MissingAction", "Missing X-Amz-Target header");
            }
        };

        if serde_json::from_slice::<Value>(&request.body).is_err() {
            return json_error_response(400, "SerializationException", "Invalid JSON body");
        }
        let body_bytes: &[u8] = &request.body;

        let state = self.state.get(account_id, &region);

        let response = match action.as_str() {
            // Cluster operations
            "CreateCluster" => {
                self.handle_create_cluster(&state, body_bytes, account_id, &region)
                    .await
            }
            "DeleteCluster" => self.handle_delete_cluster(&state, body_bytes).await,
            "DescribeClusters" => {
                self.handle_describe_clusters(&state, body_bytes, account_id, &region)
                    .await
            }
            "ListClusters" => self.handle_list_clusters(&state).await,
            "UpdateCluster" => self.handle_update_cluster(&state, body_bytes).await,
            "PutClusterCapacityProviders" => {
                self.handle_put_cluster_capacity_providers(&state, body_bytes)
                    .await
            }

            // Task Definition operations
            "RegisterTaskDefinition" => {
                self.handle_register_task_definition(&state, body_bytes, account_id, &region)
                    .await
            }
            "DescribeTaskDefinition" => {
                self.handle_describe_task_definition(&state, body_bytes)
                    .await
            }
            "DeregisterTaskDefinition" => {
                self.handle_deregister_task_definition(&state, body_bytes)
                    .await
            }
            "DeleteTaskDefinitions" => {
                self.handle_delete_task_definitions(&state, body_bytes)
                    .await
            }
            "ListTaskDefinitions" => self.handle_list_task_definitions(&state).await,
            "ListTaskDefinitionFamilies" => self.handle_list_task_definition_families(&state).await,

            // Service operations
            "CreateService" => {
                self.handle_create_service(&state, body_bytes, account_id, &region)
                    .await
            }
            "DescribeServices" => {
                self.handle_describe_services(&state, body_bytes, account_id, &region)
                    .await
            }
            "DeleteService" => self.handle_delete_service(&state, body_bytes).await,
            "UpdateService" => {
                self.handle_update_service(&state, body_bytes, account_id, &region)
                    .await
            }
            "ListServices" => self.handle_list_services(&state, body_bytes).await,

            // Capacity Provider operations
            "CreateCapacityProvider" => {
                self.handle_create_capacity_provider(&state, body_bytes, account_id, &region)
                    .await
            }
            "DeleteCapacityProvider" => {
                self.handle_delete_capacity_provider(&state, body_bytes)
                    .await
            }
            "DescribeCapacityProviders" => {
                self.handle_describe_capacity_providers(&state, body_bytes)
                    .await
            }
            "UpdateCapacityProvider" => {
                self.handle_update_capacity_provider(&state, body_bytes)
                    .await
            }

            // Container Instance operations
            "RegisterContainerInstance" => {
                self.handle_register_container_instance(&state, body_bytes, account_id, &region)
                    .await
            }
            "DeregisterContainerInstance" => {
                self.handle_deregister_container_instance(&state, body_bytes)
                    .await
            }
            "DescribeContainerInstances" => {
                self.handle_describe_container_instances(&state, body_bytes)
                    .await
            }
            "ListContainerInstances" => {
                self.handle_list_container_instances(&state, body_bytes)
                    .await
            }
            "UpdateContainerInstancesState" => {
                self.handle_update_container_instances_state(&state, body_bytes)
                    .await
            }

            // Task operations
            "RunTask" => {
                self.handle_run_task(&state, body_bytes, account_id, &region)
                    .await
            }
            "StartTask" => {
                self.handle_start_task(&state, body_bytes, account_id, &region)
                    .await
            }
            "DescribeTasks" => self.handle_describe_tasks(&state, body_bytes).await,
            "StopTask" => self.handle_stop_task(&state, body_bytes).await,
            "ListTasks" => self.handle_list_tasks(&state, body_bytes).await,

            // Task Set operations
            "CreateTaskSet" => {
                self.handle_create_task_set(&state, body_bytes, account_id, &region)
                    .await
            }
            "DeleteTaskSet" => self.handle_delete_task_set(&state, body_bytes).await,
            "DescribeTaskSets" => self.handle_describe_task_sets(&state, body_bytes).await,
            "UpdateTaskSet" => self.handle_update_task_set(&state, body_bytes).await,
            "UpdateServicePrimaryTaskSet" => {
                self.handle_update_service_primary_task_set(&state, body_bytes)
                    .await
            }

            // Account Setting operations
            "PutAccountSetting" => {
                self.handle_put_account_setting(&state, body_bytes, account_id)
                    .await
            }
            "DeleteAccountSetting" => {
                self.handle_delete_account_setting(&state, body_bytes, account_id)
                    .await
            }
            "ListAccountSettings" => self.handle_list_account_settings(&state, body_bytes).await,

            // Attribute operations
            "PutAttributes" => self.handle_put_attributes(&state, body_bytes).await,
            "DeleteAttributes" => self.handle_delete_attributes(&state, body_bytes).await,
            "ListAttributes" => self.handle_list_attributes(&state, body_bytes).await,

            // Tag operations
            "TagResource" => self.handle_tag_resource(&state, body_bytes).await,
            "UntagResource" => self.handle_untag_resource(&state, body_bytes).await,
            "ListTagsForResource" => self.handle_list_tags_for_resource(&state, body_bytes).await,

            // Discover / Execute
            "DiscoverPollEndpoint" => self.handle_discover_poll_endpoint(&state, &region).await,
            "ExecuteCommand" => self.handle_execute_command(&state, body_bytes).await,

            // Task Protection
            "GetTaskProtection" => self.handle_get_task_protection(&state, body_bytes).await,
            "UpdateTaskProtection" => self.handle_update_task_protection(&state, body_bytes).await,

            // Account Setting Default
            "PutAccountSettingDefault" => {
                self.handle_put_account_setting_default(&state, body_bytes)
                    .await
            }

            // Submit state changes (internal ECS agent APIs)
            "SubmitAttachmentStateChanges" => {
                self.handle_submit_attachment_state_changes(&state).await
            }
            "SubmitContainerStateChange" => self.handle_submit_container_state_change(&state).await,
            "SubmitTaskStateChange" => self.handle_submit_task_state_change(&state).await,

            // Cluster settings
            "UpdateClusterSettings" => {
                self.handle_update_cluster_settings(&state, body_bytes)
                    .await
            }

            // Container Agent
            "UpdateContainerAgent" => self.handle_update_container_agent(&state, body_bytes).await,

            // Services by namespace
            "ListServicesByNamespace" => {
                self.handle_list_services_by_namespace(&state, body_bytes)
                    .await
            }

            // Service Deployments and Revisions
            "DescribeServiceDeployments" => {
                self.handle_describe_service_deployments(&state, body_bytes)
                    .await
            }
            "DescribeServiceRevisions" => {
                self.handle_describe_service_revisions(&state, body_bytes)
                    .await
            }
            "ListServiceDeployments" => {
                self.handle_list_service_deployments(&state, body_bytes)
                    .await
            }
            "StopServiceDeployment" => {
                self.handle_stop_service_deployment(&state, body_bytes)
                    .await
            }

            // ExpressGatewayService operations
            "CreateExpressGatewayService" => {
                self.handle_create_express_gateway_service(&state, body_bytes, account_id, &region)
                    .await
            }
            "DeleteExpressGatewayService" => {
                self.handle_delete_express_gateway_service(&state, body_bytes)
                    .await
            }
            "DescribeExpressGatewayService" => {
                self.handle_describe_express_gateway_service(&state, body_bytes)
                    .await
            }
            "UpdateExpressGatewayService" => {
                self.handle_update_express_gateway_service(&state, body_bytes)
                    .await
            }

            _ => json_error_response(
                400,
                "InvalidAction",
                &format!("Could not find operation {action} for ECS"),
            ),
        };

        use winterbaume_core::StatefulService;
        if response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    // ========================================================================
    // Cluster handlers
    // ========================================================================

    async fn handle_create_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<EcsState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_cluster_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let name = input.cluster_name.as_deref().unwrap_or("default");

        let mut state = state.write().await;
        match state.create_cluster(name, account_id, region) {
            Ok(cluster) => wire::serialize_create_cluster_response(&wire::CreateClusterResponse {
                cluster: Some(to_wire_cluster(cluster)),
            }),
            Err(e) => ecs_error_response(&e),
        }
    }

    async fn handle_delete_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<EcsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_cluster_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.cluster.is_empty() {
            return json_error_response(400, "InvalidParameterException", "Missing 'cluster'");
        }

        let mut state = state.write().await;
        match state.delete_cluster(&input.cluster) {
            Ok(c) => wire::serialize_delete_cluster_response(&wire::DeleteClusterResponse {
                cluster: Some(to_wire_cluster(c)),
            }),
            Err(e) => ecs_error_response(&e),
        }
    }

    async fn handle_describe_clusters(
        &self,
        state: &Arc<tokio::sync::RwLock<EcsState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_clusters_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let cluster_strs = input.clusters.unwrap_or_default();
        let clusters: Vec<&str> = cluster_strs.iter().map(|s| s.as_str()).collect();

        let state = state.read().await;
        let (found, failures) = state.describe_clusters(&clusters, account_id, region);

        let cluster_entries: Vec<wire::Cluster> =
            found.iter().map(|c| to_wire_cluster(c)).collect();
        let failure_entries: Vec<wire::Failure> = failures
            .iter()
            .map(|name| wire::Failure {
                arn: Some(name.clone()),
                reason: Some("MISSING".to_string()),
                ..Default::default()
            })
            .collect();

        wire::serialize_describe_clusters_response(&wire::DescribeClustersResponse {
            clusters: Some(cluster_entries),
            failures: Some(failure_entries),
        })
    }

    async fn handle_list_clusters(
        &self,
        state: &Arc<tokio::sync::RwLock<EcsState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let arns: Vec<String> = state
            .list_clusters()
            .iter()
            .map(|s| s.to_string())
            .collect();
        wire::serialize_list_clusters_response(&wire::ListClustersResponse {
            cluster_arns: Some(arns),
            ..Default::default()
        })
    }

    async fn handle_update_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<EcsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_cluster_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.cluster.is_empty() {
            return json_error_response(400, "InvalidParameterException", "Missing 'cluster'");
        }

        let mut state = state.write().await;
        match state.update_cluster(&input.cluster) {
            Ok(c) => wire::serialize_update_cluster_response(&wire::UpdateClusterResponse {
                cluster: Some(to_wire_cluster(c)),
                ..Default::default()
            }),
            Err(e) => ecs_error_response(&e),
        }
    }

    async fn handle_put_cluster_capacity_providers(
        &self,
        state: &Arc<tokio::sync::RwLock<EcsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_put_cluster_capacity_providers_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.cluster.is_empty() {
            return json_error_response(400, "InvalidParameterException", "Missing 'cluster'");
        }
        let default_strategy: Vec<CapacityProviderStrategyItem> = input
            .default_capacity_provider_strategy
            .into_iter()
            .map(|item| CapacityProviderStrategyItem {
                capacity_provider: item.capacity_provider,
                weight: item.weight.unwrap_or(0),
                base: item.base.unwrap_or(0),
            })
            .collect();

        let mut state = state.write().await;
        match state.put_cluster_capacity_providers(
            &input.cluster,
            input.capacity_providers,
            default_strategy,
        ) {
            Ok(c) => wire::serialize_put_cluster_capacity_providers_response(
                &wire::PutClusterCapacityProvidersResponse {
                    cluster: Some(to_wire_cluster(c)),
                    ..Default::default()
                },
            ),
            Err(e) => ecs_error_response(&e),
        }
    }

    // ========================================================================
    // Task Definition handlers
    // ========================================================================

    async fn handle_register_task_definition(
        &self,
        state: &Arc<tokio::sync::RwLock<EcsState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_register_task_definition_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.family.is_empty() {
            return json_error_response(400, "InvalidParameterException", "Missing 'family'");
        }

        let container_defs = convert_container_definitions(input.container_definitions);
        let requires_compatibilities = input.requires_compatibilities.unwrap_or_default();
        let tags = convert_tags(input.tags);

        let mut state = state.write().await;
        match state.register_task_definition(
            &input.family,
            container_defs,
            account_id,
            region,
            input.task_role_arn.as_deref(),
            input.execution_role_arn.as_deref(),
            requires_compatibilities,
            input.cpu.as_deref(),
            input.memory.as_deref(),
        ) {
            Ok(td) => {
                let td_arn = td.arn.clone();
                let resp = wire::serialize_register_task_definition_response(
                    &wire::RegisterTaskDefinitionResponse {
                        task_definition: Some(to_wire_task_definition(td)),
                        ..Default::default()
                    },
                );
                // Store tags in resource_tags
                if !tags.is_empty() {
                    state.resource_tags.insert(td_arn, tags);
                }
                resp
            }
            Err(e) => ecs_error_response(&e),
        }
    }

    async fn handle_describe_task_definition(
        &self,
        state: &Arc<tokio::sync::RwLock<EcsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_task_definition_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.task_definition.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "Missing 'taskDefinition'",
            );
        }
        let task_def = input.task_definition.as_str();

        let state = state.read().await;
        match state.describe_task_definition(task_def) {
            Ok(td) => wire::serialize_describe_task_definition_response(
                &wire::DescribeTaskDefinitionResponse {
                    task_definition: Some(to_wire_task_definition(td)),
                    ..Default::default()
                },
            ),
            Err(e) => ecs_error_response(&e),
        }
    }

    async fn handle_deregister_task_definition(
        &self,
        state: &Arc<tokio::sync::RwLock<EcsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_deregister_task_definition_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.task_definition.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "Missing 'taskDefinition'",
            );
        }
        let task_def = input.task_definition.as_str();

        let mut state = state.write().await;
        match state.deregister_task_definition(task_def) {
            Ok(td) => wire::serialize_deregister_task_definition_response(
                &wire::DeregisterTaskDefinitionResponse {
                    task_definition: Some(to_wire_task_definition(td)),
                    ..Default::default()
                },
            ),
            Err(e) => ecs_error_response(&e),
        }
    }

    async fn handle_delete_task_definitions(
        &self,
        state: &Arc<tokio::sync::RwLock<EcsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_task_definitions_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let task_defs: Vec<&str> = input.task_definitions.iter().map(|s| s.as_str()).collect();

        let mut state = state.write().await;
        match state.delete_task_definitions(&task_defs) {
            Ok(tds) => {
                let wire_tds: Vec<wire::TaskDefinition> =
                    tds.iter().map(|td| to_wire_task_definition(td)).collect();
                wire::serialize_delete_task_definitions_response(
                    &wire::DeleteTaskDefinitionsResponse {
                        task_definitions: Some(wire_tds),
                        ..Default::default()
                    },
                )
            }
            Err(e) => ecs_error_response(&e),
        }
    }

    async fn handle_list_task_definitions(
        &self,
        state: &Arc<tokio::sync::RwLock<EcsState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let arns = state.list_task_definitions();
        wire::serialize_list_task_definitions_response(&wire::ListTaskDefinitionsResponse {
            task_definition_arns: Some(arns.into_iter().map(|s| s.to_string()).collect()),
            ..Default::default()
        })
    }

    async fn handle_list_task_definition_families(
        &self,
        state: &Arc<tokio::sync::RwLock<EcsState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let families = state.list_task_definition_families();
        wire::serialize_list_task_definition_families_response(
            &wire::ListTaskDefinitionFamiliesResponse {
                families: Some(families),
                ..Default::default()
            },
        )
    }

    // ========================================================================
    // Service handlers
    // ========================================================================

    async fn handle_create_service(
        &self,
        state: &Arc<tokio::sync::RwLock<EcsState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_service_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let cluster = input.cluster.as_deref().unwrap_or("default");
        if input.service_name.is_empty() {
            return json_error_response(400, "InvalidParameterException", "Missing 'serviceName'");
        }
        let task_definition = match input.task_definition.as_deref() {
            Some(td) if !td.is_empty() => td,
            _ => {
                return json_error_response(
                    400,
                    "InvalidParameterException",
                    "Missing 'taskDefinition'",
                );
            }
        };
        let desired_count = input.desired_count.unwrap_or(1);
        let launch_type = input.launch_type.as_deref().unwrap_or("EC2");
        let scheduling_strategy = input.scheduling_strategy.as_deref().unwrap_or("REPLICA");
        let tags = convert_tags(input.tags);
        let load_balancers = convert_load_balancers(input.load_balancers);
        let deployment_controller_type = input
            .deployment_controller
            .as_ref()
            .map(|dc| dc.r#type.as_str());

        let mut state = state.write().await;
        match state.create_service(
            cluster,
            &input.service_name,
            task_definition,
            desired_count,
            account_id,
            region,
            launch_type,
            scheduling_strategy,
            tags,
            load_balancers,
            deployment_controller_type,
        ) {
            Ok(svc) => wire::serialize_create_service_response(&wire::CreateServiceResponse {
                service: Some(to_wire_service(svc)),
            }),
            Err(e) => ecs_error_response(&e),
        }
    }

    async fn handle_describe_services(
        &self,
        state: &Arc<tokio::sync::RwLock<EcsState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_services_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let cluster = input.cluster.as_deref().unwrap_or("default");
        let service_names: Vec<&str> = input.services.iter().map(|s| s.as_str()).collect();

        let state = state.read().await;
        let (found, failures) =
            match state.describe_services(cluster, &service_names, account_id, region) {
                Ok(result) => result,
                Err(e) => return ecs_error_response(&e),
            };

        let service_entries: Vec<wire::Service> =
            found.iter().map(|s| to_wire_service(s)).collect();
        let failure_entries: Vec<wire::Failure> = failures
            .iter()
            .map(|arn| wire::Failure {
                arn: Some(arn.clone()),
                reason: Some("MISSING".to_string()),
                ..Default::default()
            })
            .collect();

        wire::serialize_describe_services_response(&wire::DescribeServicesResponse {
            services: Some(service_entries),
            failures: Some(failure_entries),
        })
    }

    async fn handle_delete_service(
        &self,
        state: &Arc<tokio::sync::RwLock<EcsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_service_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let cluster = input.cluster.as_deref().unwrap_or("default");
        if input.service.is_empty() {
            return json_error_response(400, "InvalidParameterException", "Missing 'service'");
        }

        let mut state = state.write().await;
        match state.delete_service(cluster, &input.service) {
            Ok(svc) => wire::serialize_delete_service_response(&wire::DeleteServiceResponse {
                service: Some(to_wire_service(svc)),
                ..Default::default()
            }),
            Err(e) => ecs_error_response(&e),
        }
    }

    async fn handle_update_service(
        &self,
        state: &Arc<tokio::sync::RwLock<EcsState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_update_service_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let cluster = input.cluster.as_deref().unwrap_or("default");
        if input.service.is_empty() {
            return json_error_response(400, "InvalidParameterException", "Missing 'service'");
        }

        let mut state = state.write().await;
        match state.update_service(
            cluster,
            &input.service,
            input.task_definition.as_deref(),
            input.desired_count,
            account_id,
            region,
        ) {
            Ok(svc) => wire::serialize_update_service_response(&wire::UpdateServiceResponse {
                service: Some(to_wire_service(svc)),
                ..Default::default()
            }),
            Err(e) => ecs_error_response(&e),
        }
    }

    async fn handle_list_services(
        &self,
        state: &Arc<tokio::sync::RwLock<EcsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_services_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let cluster = input.cluster.as_deref().unwrap_or("default");

        let state = state.read().await;
        match state.list_services(cluster) {
            Ok(arns) => wire::serialize_list_services_response(&wire::ListServicesResponse {
                service_arns: Some(arns.into_iter().map(|s| s.to_string()).collect()),
                ..Default::default()
            }),
            Err(e) => ecs_error_response(&e),
        }
    }

    // ========================================================================
    // Capacity Provider handlers
    // ========================================================================

    async fn handle_create_capacity_provider(
        &self,
        state: &Arc<tokio::sync::RwLock<EcsState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_capacity_provider_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "InvalidParameterException", "Missing 'name'");
        }
        let auto_scaling_group_arn = input
            .auto_scaling_group_provider
            .as_ref()
            .map(|p| p.auto_scaling_group_arn.as_str())
            .unwrap_or("");
        let tags = convert_tags(input.tags);

        let mut state = state.write().await;
        match state.create_capacity_provider(
            &input.name,
            auto_scaling_group_arn,
            account_id,
            region,
            tags,
        ) {
            Ok(cp) => wire::serialize_create_capacity_provider_response(
                &wire::CreateCapacityProviderResponse {
                    capacity_provider: Some(to_wire_capacity_provider(cp)),
                    ..Default::default()
                },
            ),
            Err(e) => ecs_error_response(&e),
        }
    }

    async fn handle_delete_capacity_provider(
        &self,
        state: &Arc<tokio::sync::RwLock<EcsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_capacity_provider_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.capacity_provider.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "Missing 'capacityProvider'",
            );
        }
        let cp = input.capacity_provider.as_str();

        let mut state = state.write().await;
        match state.delete_capacity_provider(cp) {
            Ok(cp) => wire::serialize_delete_capacity_provider_response(
                &wire::DeleteCapacityProviderResponse {
                    capacity_provider: Some(to_wire_capacity_provider(cp)),
                    ..Default::default()
                },
            ),
            Err(e) => ecs_error_response(&e),
        }
    }

    async fn handle_describe_capacity_providers(
        &self,
        state: &Arc<tokio::sync::RwLock<EcsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_capacity_providers_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let name_strs = input.capacity_providers.unwrap_or_default();
        let names: Vec<&str> = name_strs.iter().map(|s| s.as_str()).collect();

        let state = state.read().await;
        let (found, failures) = state.describe_capacity_providers(&names);
        let wire_cps: Vec<wire::CapacityProvider> = found
            .iter()
            .map(|cp| to_wire_capacity_provider(cp))
            .collect();
        let wire_failures: Vec<wire::Failure> = failures
            .iter()
            .map(|arn| wire::Failure {
                arn: Some(arn.clone()),
                reason: Some("MISSING".to_string()),
                ..Default::default()
            })
            .collect();

        wire::serialize_describe_capacity_providers_response(
            &wire::DescribeCapacityProvidersResponse {
                capacity_providers: Some(wire_cps),
                failures: Some(wire_failures),
                ..Default::default()
            },
        )
    }

    async fn handle_update_capacity_provider(
        &self,
        state: &Arc<tokio::sync::RwLock<EcsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_capacity_provider_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "InvalidParameterException", "Missing 'name'");
        }

        let mut state = state.write().await;
        match state.update_capacity_provider(&input.name) {
            Ok(cp) => wire::serialize_update_capacity_provider_response(
                &wire::UpdateCapacityProviderResponse {
                    capacity_provider: Some(to_wire_capacity_provider(cp)),
                    ..Default::default()
                },
            ),
            Err(e) => ecs_error_response(&e),
        }
    }

    // ========================================================================
    // Container Instance handlers
    // ========================================================================

    async fn handle_register_container_instance(
        &self,
        state: &Arc<tokio::sync::RwLock<EcsState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_register_container_instance_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let cluster = input.cluster.as_deref().unwrap_or("default");
        let instance_id = input
            .instance_identity_document
            .as_deref()
            .or(input.container_instance_arn.as_deref())
            .unwrap_or("i-unknown");

        let mut state = state.write().await;
        match state.register_container_instance(cluster, instance_id, account_id, region) {
            Ok(ci) => wire::serialize_register_container_instance_response(
                &wire::RegisterContainerInstanceResponse {
                    container_instance: Some(to_wire_container_instance(ci)),
                    ..Default::default()
                },
            ),
            Err(e) => ecs_error_response(&e),
        }
    }

    async fn handle_deregister_container_instance(
        &self,
        state: &Arc<tokio::sync::RwLock<EcsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_deregister_container_instance_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let cluster = input.cluster.as_deref().unwrap_or("default");
        if input.container_instance.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "Missing 'containerInstance'",
            );
        }
        let ci = input.container_instance.as_str();

        let mut state = state.write().await;
        match state.deregister_container_instance(cluster, ci) {
            Ok(ci) => wire::serialize_deregister_container_instance_response(
                &wire::DeregisterContainerInstanceResponse {
                    container_instance: Some(to_wire_container_instance(ci)),
                    ..Default::default()
                },
            ),
            Err(e) => ecs_error_response(&e),
        }
    }

    async fn handle_describe_container_instances(
        &self,
        state: &Arc<tokio::sync::RwLock<EcsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_container_instances_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let cluster = input.cluster.as_deref().unwrap_or("default");
        let cis: Vec<&str> = input
            .container_instances
            .iter()
            .map(|s| s.as_str())
            .collect();

        let state = state.read().await;
        match state.describe_container_instances(cluster, &cis) {
            Ok((found, failures)) => {
                let wire_cis: Vec<wire::ContainerInstance> = found
                    .iter()
                    .map(|ci| to_wire_container_instance(ci))
                    .collect();
                let wire_failures: Vec<wire::Failure> = failures
                    .iter()
                    .map(|arn| wire::Failure {
                        arn: Some(arn.clone()),
                        reason: Some("MISSING".to_string()),
                        ..Default::default()
                    })
                    .collect();

                wire::serialize_describe_container_instances_response(
                    &wire::DescribeContainerInstancesResponse {
                        container_instances: Some(wire_cis),
                        failures: Some(wire_failures),
                        ..Default::default()
                    },
                )
            }
            Err(e) => ecs_error_response(&e),
        }
    }

    async fn handle_list_container_instances(
        &self,
        state: &Arc<tokio::sync::RwLock<EcsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_container_instances_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let cluster = input.cluster.as_deref().unwrap_or("default");

        let state = state.read().await;
        match state.list_container_instances(cluster) {
            Ok(arns) => wire::serialize_list_container_instances_response(
                &wire::ListContainerInstancesResponse {
                    container_instance_arns: Some(
                        arns.into_iter().map(|s| s.to_string()).collect(),
                    ),
                    ..Default::default()
                },
            ),
            Err(e) => ecs_error_response(&e),
        }
    }

    async fn handle_update_container_instances_state(
        &self,
        state: &Arc<tokio::sync::RwLock<EcsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_container_instances_state_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let cluster = input.cluster.as_deref().unwrap_or("default");
        let cis: Vec<&str> = input
            .container_instances
            .iter()
            .map(|s| s.as_str())
            .collect();
        let status = if input.status.is_empty() {
            "ACTIVE"
        } else {
            input.status.as_str()
        };

        let mut state_guard = state.write().await;
        match state_guard.update_container_instances_state(cluster, &cis, status) {
            Ok((updated_arns, failures)) => {
                let wire_cis: Vec<wire::ContainerInstance> = updated_arns
                    .iter()
                    .filter_map(|arn| {
                        state_guard
                            .container_instances
                            .get(arn)
                            .map(to_wire_container_instance)
                    })
                    .collect();
                let wire_failures: Vec<wire::Failure> = failures
                    .iter()
                    .map(|arn| wire::Failure {
                        arn: Some(arn.clone()),
                        reason: Some("MISSING".to_string()),
                        ..Default::default()
                    })
                    .collect();

                wire::serialize_update_container_instances_state_response(
                    &wire::UpdateContainerInstancesStateResponse {
                        container_instances: Some(wire_cis),
                        failures: Some(wire_failures),
                        ..Default::default()
                    },
                )
            }
            Err(e) => ecs_error_response(&e),
        }
    }

    // ========================================================================
    // Task handlers
    // ========================================================================

    async fn handle_run_task(
        &self,
        state: &Arc<tokio::sync::RwLock<EcsState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_run_task_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let cluster = input.cluster.as_deref().unwrap_or("default");
        if input.task_definition.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "Missing 'taskDefinition'",
            );
        }
        let count = input.count.unwrap_or(1);
        let launch_type = input.launch_type.as_deref().unwrap_or("EC2");
        let overrides = input.overrides.and_then(|o| serde_json::to_value(o).ok());
        let tags = convert_tags(input.tags);

        let mut state = state.write().await;
        match state.run_task(
            cluster,
            &input.task_definition,
            count,
            launch_type,
            input.started_by.as_deref(),
            input.group.as_deref(),
            overrides,
            account_id,
            region,
            tags,
        ) {
            Ok(tasks) => {
                let wire_tasks: Vec<wire::Task> = tasks.iter().map(|t| to_wire_task(t)).collect();
                wire::serialize_run_task_response(&wire::RunTaskResponse {
                    tasks: Some(wire_tasks),
                    failures: Some(vec![]),
                    ..Default::default()
                })
            }
            Err(e) => ecs_error_response(&e),
        }
    }

    async fn handle_start_task(
        &self,
        state: &Arc<tokio::sync::RwLock<EcsState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_start_task_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let cluster = input.cluster.as_deref().unwrap_or("default");
        if input.task_definition.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "Missing 'taskDefinition'",
            );
        }
        let container_instances: Vec<&str> = input
            .container_instances
            .iter()
            .map(|s| s.as_str())
            .collect();
        let overrides = input.overrides.and_then(|o| serde_json::to_value(o).ok());
        let tags = convert_tags(input.tags);

        let mut state_guard = state.write().await;
        match state_guard.start_task(
            cluster,
            &input.task_definition,
            &container_instances,
            input.started_by.as_deref(),
            input.group.as_deref(),
            overrides,
            account_id,
            region,
            tags,
        ) {
            Ok((task_arns, failure_arns)) => {
                let wire_tasks: Vec<wire::Task> = task_arns
                    .iter()
                    .filter_map(|arn| state_guard.tasks.get(arn).map(to_wire_task))
                    .collect();
                let wire_failures: Vec<wire::Failure> = failure_arns
                    .iter()
                    .map(|arn| wire::Failure {
                        arn: Some(arn.clone()),
                        reason: Some("MISSING".to_string()),
                        ..Default::default()
                    })
                    .collect();

                wire::serialize_start_task_response(&wire::StartTaskResponse {
                    tasks: Some(wire_tasks),
                    failures: Some(wire_failures),
                    ..Default::default()
                })
            }
            Err(e) => ecs_error_response(&e),
        }
    }

    async fn handle_describe_tasks(
        &self,
        state: &Arc<tokio::sync::RwLock<EcsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_tasks_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let cluster = input.cluster.as_deref().unwrap_or("default");
        let task_arns: Vec<&str> = input.tasks.iter().map(|s| s.as_str()).collect();

        let state = state.read().await;
        match state.describe_tasks(cluster, &task_arns) {
            Ok((found, failures)) => {
                let wire_tasks: Vec<wire::Task> = found.iter().map(|t| to_wire_task(t)).collect();
                let wire_failures: Vec<wire::Failure> = failures
                    .iter()
                    .map(|arn| wire::Failure {
                        arn: Some(arn.clone()),
                        reason: Some("MISSING".to_string()),
                        ..Default::default()
                    })
                    .collect();

                wire::serialize_describe_tasks_response(&wire::DescribeTasksResponse {
                    tasks: Some(wire_tasks),
                    failures: Some(wire_failures),
                    ..Default::default()
                })
            }
            Err(e) => ecs_error_response(&e),
        }
    }

    async fn handle_stop_task(
        &self,
        state: &Arc<tokio::sync::RwLock<EcsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_stop_task_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let cluster = input.cluster.as_deref().unwrap_or("default");
        if input.task.is_empty() {
            return json_error_response(400, "InvalidParameterException", "Missing 'task'");
        }

        let mut state = state.write().await;
        match state.stop_task(cluster, &input.task, input.reason.as_deref()) {
            Ok(t) => wire::serialize_stop_task_response(&wire::StopTaskResponse {
                task: Some(to_wire_task(t)),
                ..Default::default()
            }),
            Err(e) => ecs_error_response(&e),
        }
    }

    async fn handle_list_tasks(
        &self,
        state: &Arc<tokio::sync::RwLock<EcsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_tasks_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let cluster = input.cluster.as_deref().unwrap_or("default");

        let state = state.read().await;
        match state.list_tasks(
            cluster,
            input.family.as_deref(),
            input.service_name.as_deref(),
            input.desired_status.as_deref(),
            input.started_by.as_deref(),
            input.container_instance.as_deref(),
        ) {
            Ok(arns) => wire::serialize_list_tasks_response(&wire::ListTasksResponse {
                task_arns: Some(arns.into_iter().map(|s| s.to_string()).collect()),
                ..Default::default()
            }),
            Err(e) => ecs_error_response(&e),
        }
    }

    // ========================================================================
    // Task Set handlers
    // ========================================================================

    async fn handle_create_task_set(
        &self,
        state: &Arc<tokio::sync::RwLock<EcsState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_task_set_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.cluster.is_empty() {
            return json_error_response(400, "InvalidParameterException", "Missing 'cluster'");
        }
        if input.service.is_empty() {
            return json_error_response(400, "InvalidParameterException", "Missing 'service'");
        }
        if input.task_definition.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "Missing 'taskDefinition'",
            );
        }
        let launch_type = input.launch_type.as_deref().unwrap_or("EC2");
        let scale_value = input.scale.and_then(|s| s.value);
        let tags = convert_tags(input.tags);

        let mut state = state.write().await;
        match state.create_task_set(
            &input.cluster,
            &input.service,
            &input.task_definition,
            launch_type,
            scale_value,
            account_id,
            region,
            tags,
        ) {
            Ok(ts) => wire::serialize_create_task_set_response(&wire::CreateTaskSetResponse {
                task_set: Some(to_wire_task_set(ts)),
                ..Default::default()
            }),
            Err(e) => ecs_error_response(&e),
        }
    }

    async fn handle_delete_task_set(
        &self,
        state: &Arc<tokio::sync::RwLock<EcsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_task_set_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.cluster.is_empty() {
            return json_error_response(400, "InvalidParameterException", "Missing 'cluster'");
        }
        if input.service.is_empty() {
            return json_error_response(400, "InvalidParameterException", "Missing 'service'");
        }
        if input.task_set.is_empty() {
            return json_error_response(400, "InvalidParameterException", "Missing 'taskSet'");
        }

        let mut state = state.write().await;
        match state.delete_task_set(&input.cluster, &input.service, &input.task_set) {
            Ok(ts) => wire::serialize_delete_task_set_response(&wire::DeleteTaskSetResponse {
                task_set: Some(to_wire_task_set(ts)),
                ..Default::default()
            }),
            Err(e) => ecs_error_response(&e),
        }
    }

    async fn handle_describe_task_sets(
        &self,
        state: &Arc<tokio::sync::RwLock<EcsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_task_sets_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.cluster.is_empty() {
            return json_error_response(400, "InvalidParameterException", "Missing 'cluster'");
        }
        if input.service.is_empty() {
            return json_error_response(400, "InvalidParameterException", "Missing 'service'");
        }
        let task_sets_strs = input.task_sets;
        let task_sets_refs: Option<Vec<&str>> = task_sets_strs
            .as_ref()
            .map(|v| v.iter().map(|s| s.as_str()).collect());

        let state = state.read().await;
        match state.describe_task_sets(&input.cluster, &input.service, task_sets_refs.as_deref()) {
            Ok(task_sets) => {
                let wire_ts: Vec<wire::TaskSet> =
                    task_sets.iter().map(|ts| to_wire_task_set(ts)).collect();
                wire::serialize_describe_task_sets_response(&wire::DescribeTaskSetsResponse {
                    task_sets: Some(wire_ts),
                    failures: Some(vec![]),
                    ..Default::default()
                })
            }
            Err(e) => ecs_error_response(&e),
        }
    }

    async fn handle_update_task_set(
        &self,
        state: &Arc<tokio::sync::RwLock<EcsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_task_set_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.cluster.is_empty() {
            return json_error_response(400, "InvalidParameterException", "Missing 'cluster'");
        }
        if input.service.is_empty() {
            return json_error_response(400, "InvalidParameterException", "Missing 'service'");
        }
        if input.task_set.is_empty() {
            return json_error_response(400, "InvalidParameterException", "Missing 'taskSet'");
        }
        let scale_value = input.scale.value.unwrap_or(100.0);

        let mut state = state.write().await;
        match state.update_task_set(&input.cluster, &input.service, &input.task_set, scale_value) {
            Ok(ts) => wire::serialize_update_task_set_response(&wire::UpdateTaskSetResponse {
                task_set: Some(to_wire_task_set(ts)),
                ..Default::default()
            }),
            Err(e) => ecs_error_response(&e),
        }
    }

    async fn handle_update_service_primary_task_set(
        &self,
        state: &Arc<tokio::sync::RwLock<EcsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_service_primary_task_set_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.cluster.is_empty() {
            return json_error_response(400, "InvalidParameterException", "Missing 'cluster'");
        }
        if input.service.is_empty() {
            return json_error_response(400, "InvalidParameterException", "Missing 'service'");
        }
        if input.primary_task_set.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "Missing 'primaryTaskSet'",
            );
        }

        let mut state = state.write().await;
        match state.update_service_primary_task_set(
            &input.cluster,
            &input.service,
            &input.primary_task_set,
        ) {
            Ok(ts) => wire::serialize_update_service_primary_task_set_response(
                &wire::UpdateServicePrimaryTaskSetResponse {
                    task_set: Some(to_wire_task_set(ts)),
                    ..Default::default()
                },
            ),
            Err(e) => ecs_error_response(&e),
        }
    }

    // ========================================================================
    // Account Setting handlers
    // ========================================================================

    async fn handle_put_account_setting(
        &self,
        state: &Arc<tokio::sync::RwLock<EcsState>>,
        body: &[u8],
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_put_account_setting_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "InvalidParameterException", "Missing 'name'");
        }
        if input.value.is_empty() {
            return json_error_response(400, "InvalidParameterException", "Missing 'value'");
        }
        let default_principal = format!("arn:aws:iam::{account_id}:root");
        let principal_arn = input.principal_arn.as_deref().unwrap_or(&default_principal);

        let mut state = state.write().await;
        let setting = state.put_account_setting(&input.name, &input.value, principal_arn);
        wire::serialize_put_account_setting_response(&wire::PutAccountSettingResponse {
            setting: Some(wire::Setting {
                name: Some(setting.name.clone()),
                value: Some(setting.value.clone()),
                principal_arn: Some(setting.principal_arn.clone()),
                ..Default::default()
            }),
            ..Default::default()
        })
    }

    async fn handle_delete_account_setting(
        &self,
        state: &Arc<tokio::sync::RwLock<EcsState>>,
        body: &[u8],
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_account_setting_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "InvalidParameterException", "Missing 'name'");
        }
        let default_principal = format!("arn:aws:iam::{account_id}:root");
        let principal_arn = input.principal_arn.as_deref().unwrap_or(&default_principal);

        let mut state = state.write().await;
        match state.delete_account_setting(&input.name, principal_arn) {
            Ok(setting) => wire::serialize_delete_account_setting_response(
                &wire::DeleteAccountSettingResponse {
                    setting: Some(wire::Setting {
                        name: Some(setting.name),
                        value: Some(setting.value),
                        principal_arn: Some(setting.principal_arn),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
            ),
            Err(e) => ecs_error_response(&e),
        }
    }

    async fn handle_list_account_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<EcsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_account_settings_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };

        let state = state.read().await;
        let settings = state.list_account_settings(input.name.as_deref());
        let wire_settings: Vec<wire::Setting> = settings
            .iter()
            .map(|s| wire::Setting {
                name: Some(s.name.clone()),
                value: Some(s.value.clone()),
                principal_arn: Some(s.principal_arn.clone()),
                ..Default::default()
            })
            .collect();

        wire::serialize_list_account_settings_response(&wire::ListAccountSettingsResponse {
            settings: Some(wire_settings),
            ..Default::default()
        })
    }

    // ========================================================================
    // Attribute handlers
    // ========================================================================

    async fn handle_put_attributes(
        &self,
        state: &Arc<tokio::sync::RwLock<EcsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_put_attributes_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let attributes: Vec<EcsAttribute> = input
            .attributes
            .into_iter()
            .map(|a| EcsAttribute {
                name: a.name,
                value: a.value,
                target_type: a.target_type,
                target_id: a.target_id,
            })
            .collect();

        let mut state = state.write().await;
        let result = state.put_attributes(attributes);
        let wire_attrs: Vec<wire::Attribute> = result
            .iter()
            .map(|a| wire::Attribute {
                name: a.name.clone(),
                value: a.value.clone(),
                target_type: a.target_type.clone(),
                target_id: a.target_id.clone(),
            })
            .collect();

        wire::serialize_put_attributes_response(&wire::PutAttributesResponse {
            attributes: Some(wire_attrs),
            ..Default::default()
        })
    }

    async fn handle_delete_attributes(
        &self,
        state: &Arc<tokio::sync::RwLock<EcsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_attributes_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let attributes: Vec<(String, Option<String>, Option<String>)> = input
            .attributes
            .into_iter()
            .map(|a| (a.name, a.target_type, a.target_id))
            .collect();

        let mut state = state.write().await;
        let deleted = state.delete_attributes(&attributes);
        let wire_attrs: Vec<wire::Attribute> = deleted
            .iter()
            .map(|a| wire::Attribute {
                name: a.name.clone(),
                value: a.value.clone(),
                target_type: a.target_type.clone(),
                target_id: a.target_id.clone(),
            })
            .collect();

        wire::serialize_delete_attributes_response(&wire::DeleteAttributesResponse {
            attributes: Some(wire_attrs),
            ..Default::default()
        })
    }

    async fn handle_list_attributes(
        &self,
        state: &Arc<tokio::sync::RwLock<EcsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_attributes_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let target_type = if input.target_type.is_empty() {
            None
        } else {
            Some(input.target_type.as_str())
        };

        let state = state.read().await;
        let attrs = state.list_attributes(target_type);
        let wire_attrs: Vec<wire::Attribute> = attrs
            .iter()
            .map(|a| wire::Attribute {
                name: a.name.clone(),
                value: a.value.clone(),
                target_type: a.target_type.clone(),
                target_id: a.target_id.clone(),
            })
            .collect();

        wire::serialize_list_attributes_response(&wire::ListAttributesResponse {
            attributes: Some(wire_attrs),
            ..Default::default()
        })
    }

    // ========================================================================
    // Tag handlers
    // ========================================================================

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<EcsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(400, "InvalidParameterException", "Missing 'resourceArn'");
        }
        let tags = convert_tags_required(input.tags);

        let mut state = state.write().await;
        match state.tag_resource(&input.resource_arn, tags) {
            Ok(()) => wire::serialize_tag_resource_response(&wire::TagResourceResponse {}),
            Err(e) => ecs_error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<EcsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(400, "InvalidParameterException", "Missing 'resourceArn'");
        }
        let tag_keys: Vec<&str> = input.tag_keys.iter().map(|s| s.as_str()).collect();

        let mut state = state.write().await;
        match state.untag_resource(&input.resource_arn, &tag_keys) {
            Ok(()) => wire::serialize_untag_resource_response(&wire::UntagResourceResponse {}),
            Err(e) => ecs_error_response(&e),
        }
    }

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<EcsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(400, "InvalidParameterException", "Missing 'resourceArn'");
        }

        let state = state.read().await;
        match state.list_tags_for_resource(&input.resource_arn) {
            Ok(tags) => {
                let wire_tags: Vec<wire::Tag> = tags
                    .iter()
                    .map(|t| wire::Tag {
                        key: Some(t.key.clone()),
                        value: Some(t.value.clone()),
                    })
                    .collect();
                wire::serialize_list_tags_for_resource_response(
                    &wire::ListTagsForResourceResponse {
                        tags: Some(wire_tags),
                        ..Default::default()
                    },
                )
            }
            Err(e) => ecs_error_response(&e),
        }
    }

    // ========================================================================
    // DiscoverPollEndpoint handler
    // ========================================================================

    async fn handle_discover_poll_endpoint(
        &self,
        state: &Arc<tokio::sync::RwLock<EcsState>>,
        region: &str,
    ) -> MockResponse {
        let mut state_guard = state.write().await;
        let ep = state_guard.get_or_create_poll_endpoint(region);
        wire::serialize_discover_poll_endpoint_response(&wire::DiscoverPollEndpointResponse {
            endpoint: Some(ep.endpoint.clone()),
            telemetry_endpoint: Some(ep.telemetry_endpoint.clone()),
            ..Default::default()
        })
    }

    // ========================================================================
    // ExecuteCommand handler
    // ========================================================================

    // STUB[no-engine]: ExecuteCommand requires a real interactive shell
    //   execution engine (SSM Session Manager); the mock returns a synthetic
    //   session token with no real process spawned.
    async fn handle_execute_command(
        &self,
        state: &Arc<tokio::sync::RwLock<EcsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_execute_command_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let cluster = input.cluster.as_deref().unwrap_or("default");
        let task = input.task.as_str();
        let container = input.container.as_deref().unwrap_or("");
        let interactive = input.interactive;

        let state = state.read().await;
        // Resolve cluster ARN
        let cluster_name = if cluster.starts_with("arn:aws:ecs:") {
            cluster.rsplit('/').next().unwrap_or(cluster).to_string()
        } else {
            cluster.to_string()
        };
        let cluster_arn = state
            .clusters
            .get(&cluster_name)
            .map(|c| c.arn.clone())
            .unwrap_or_else(|| {
                format!("arn:aws:ecs:us-east-1:123456789012:cluster/{cluster_name}")
            });

        let session_id = uuid::Uuid::new_v4().to_string();
        wire::serialize_execute_command_response(&wire::ExecuteCommandResponse {
            cluster_arn: Some(cluster_arn),
            container_name: Some(container.to_string()),
            task_arn: Some(task.to_string()),
            interactive: Some(interactive),
            session: Some(wire::Session {
                session_id: Some(session_id.clone()),
                stream_url: Some(format!(
                    "wss://ssmmessages.us-east-1.amazonaws.com/v1/data-channel/{session_id}"
                )),
                token_value: Some("mock-token-value".to_string()),
            }),
            ..Default::default()
        })
    }

    // ========================================================================
    // GetTaskProtection / UpdateTaskProtection handlers
    // ========================================================================

    async fn handle_get_task_protection(
        &self,
        state: &Arc<tokio::sync::RwLock<EcsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_task_protection_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let cluster = if input.cluster.is_empty() {
            "default"
        } else {
            input.cluster.as_str()
        };
        let task_strs = input.tasks.unwrap_or_default();
        let tasks: Vec<&str> = task_strs.iter().map(|s| s.as_str()).collect();

        let state = state.read().await;
        match state.get_task_protection(cluster, &tasks) {
            Ok(protections) => {
                let wire_tasks: Vec<wire::ProtectedTask> = protections
                    .iter()
                    .map(|p| wire::ProtectedTask {
                        task_arn: Some(p.task_arn.clone()),
                        protection_enabled: Some(p.protection_enabled),
                        expiration_date: p.expiration_date,
                    })
                    .collect();
                wire::serialize_get_task_protection_response(&wire::GetTaskProtectionResponse {
                    protected_tasks: Some(wire_tasks),
                    failures: Some(vec![]),
                    ..Default::default()
                })
            }
            Err(e) => ecs_error_response(&e),
        }
    }

    async fn handle_update_task_protection(
        &self,
        state: &Arc<tokio::sync::RwLock<EcsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_task_protection_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let cluster = if input.cluster.is_empty() {
            "default"
        } else {
            input.cluster.as_str()
        };
        let tasks: Vec<&str> = input.tasks.iter().map(|s| s.as_str()).collect();
        let protection_enabled = input.protection_enabled;
        let expires_in_minutes = input.expires_in_minutes.map(|v| v as i64);

        let mut state = state.write().await;
        match state.update_task_protection(cluster, &tasks, protection_enabled, expires_in_minutes)
        {
            Ok(protections) => {
                let wire_tasks: Vec<wire::ProtectedTask> = protections
                    .iter()
                    .map(|p| wire::ProtectedTask {
                        task_arn: Some(p.task_arn.clone()),
                        protection_enabled: Some(p.protection_enabled),
                        expiration_date: p.expiration_date,
                    })
                    .collect();
                wire::serialize_update_task_protection_response(
                    &wire::UpdateTaskProtectionResponse {
                        protected_tasks: Some(wire_tasks),
                        failures: Some(vec![]),
                        ..Default::default()
                    },
                )
            }
            Err(e) => ecs_error_response(&e),
        }
    }

    // ========================================================================
    // PutAccountSettingDefault handler
    // ========================================================================

    async fn handle_put_account_setting_default(
        &self,
        state: &Arc<tokio::sync::RwLock<EcsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_put_account_setting_default_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "InvalidParameterException", "Missing 'name'");
        }
        if input.value.is_empty() {
            return json_error_response(400, "InvalidParameterException", "Missing 'value'");
        }

        let mut state = state.write().await;
        let (n, v) = state.put_account_setting_default(&input.name, &input.value);
        wire::serialize_put_account_setting_default_response(
            &wire::PutAccountSettingDefaultResponse {
                setting: Some(wire::Setting {
                    name: Some(n),
                    value: Some(v),
                    ..Default::default()
                }),
                ..Default::default()
            },
        )
    }

    // ========================================================================
    // Submit state changes (internal ECS agent APIs)
    // ========================================================================

    async fn handle_submit_attachment_state_changes(
        &self,
        state: &Arc<tokio::sync::RwLock<EcsState>>,
    ) -> MockResponse {
        let mut state_guard = state.write().await;
        let ack = state_guard.submit_attachment_state_change();
        wire::serialize_submit_attachment_state_changes_response(
            &wire::SubmitAttachmentStateChangesResponse {
                acknowledgment: Some(ack),
                ..Default::default()
            },
        )
    }

    async fn handle_submit_container_state_change(
        &self,
        state: &Arc<tokio::sync::RwLock<EcsState>>,
    ) -> MockResponse {
        let mut state_guard = state.write().await;
        let ack = state_guard.submit_container_state_change();
        wire::serialize_submit_container_state_change_response(
            &wire::SubmitContainerStateChangeResponse {
                acknowledgment: Some(ack),
            },
        )
    }

    async fn handle_submit_task_state_change(
        &self,
        state: &Arc<tokio::sync::RwLock<EcsState>>,
    ) -> MockResponse {
        let mut state_guard = state.write().await;
        let ack = state_guard.submit_task_state_change();
        wire::serialize_submit_task_state_change_response(&wire::SubmitTaskStateChangeResponse {
            acknowledgment: Some(ack),
        })
    }

    // ========================================================================
    // UpdateClusterSettings handler
    // ========================================================================

    async fn handle_update_cluster_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<EcsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_cluster_settings_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.cluster.is_empty() {
            return json_error_response(400, "InvalidParameterException", "Missing 'cluster'");
        }
        let settings: Vec<crate::types::EcsClusterSetting> = input
            .settings
            .into_iter()
            .map(|s| crate::types::EcsClusterSetting {
                name: s.name.unwrap_or_default(),
                value: s.value.unwrap_or_default(),
            })
            .collect();

        let mut state = state.write().await;
        match state.update_cluster_settings(&input.cluster, settings) {
            Ok(c) => wire::serialize_update_cluster_settings_response(
                &wire::UpdateClusterSettingsResponse {
                    cluster: Some(to_wire_cluster(c)),
                    ..Default::default()
                },
            ),
            Err(e) => ecs_error_response(&e),
        }
    }

    // ========================================================================
    // UpdateContainerAgent handler
    // ========================================================================

    async fn handle_update_container_agent(
        &self,
        state: &Arc<tokio::sync::RwLock<EcsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_container_agent_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let cluster = input.cluster.as_deref().unwrap_or("default");
        if input.container_instance.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "Missing 'containerInstance'",
            );
        }
        let container_instance = input.container_instance.as_str();

        let state = state.read().await;
        let cluster_name = if cluster.starts_with("arn:aws:ecs:") {
            cluster.rsplit('/').next().unwrap_or(cluster).to_string()
        } else {
            cluster.to_string()
        };
        // Resolve CI ARN
        let ci_arn = if container_instance.starts_with("arn:aws:ecs:") {
            container_instance.to_string()
        } else {
            state
                .container_instances
                .keys()
                .find(|k| k.ends_with(container_instance))
                .cloned()
                .unwrap_or_else(|| container_instance.to_string())
        };

        match state.container_instances.get(&ci_arn) {
            Some(ci) => wire::serialize_update_container_agent_response(
                &wire::UpdateContainerAgentResponse {
                    container_instance: Some(to_wire_container_instance(ci)),
                    ..Default::default()
                },
            ),
            None => {
                // Return a minimal mock response when CI not found
                let _ = cluster_name;
                wire::serialize_update_container_agent_response(
                    &wire::UpdateContainerAgentResponse {
                        container_instance: Some(wire::ContainerInstance {
                            container_instance_arn: Some(ci_arn),
                            agent_update_status: Some("UPDATED".to_string()),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                )
            }
        }
    }

    // ========================================================================
    // ListServicesByNamespace handler
    // ========================================================================

    async fn handle_list_services_by_namespace(
        &self,
        state: &Arc<tokio::sync::RwLock<EcsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_services_by_namespace_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let namespace = input.namespace.as_str();
        let state = state.read().await;
        let arns = state.list_services_by_namespace(namespace);
        wire::serialize_list_services_by_namespace_response(
            &wire::ListServicesByNamespaceResponse {
                service_arns: Some(arns.into_iter().map(|s| s.to_string()).collect()),
                ..Default::default()
            },
        )
    }

    // ========================================================================
    // Service Deployments and Revisions handlers
    // ========================================================================

    async fn handle_describe_service_deployments(
        &self,
        state: &Arc<tokio::sync::RwLock<EcsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_service_deployments_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let deployment_arns: Vec<&str> = input
            .service_deployment_arns
            .iter()
            .map(|s| s.as_str())
            .collect();
        let state_guard = state.read().await;
        let (found, failure_arns) = state_guard.describe_service_deployments(&deployment_arns);
        let wire_deployments: Vec<wire::ServiceDeployment> = found
            .iter()
            .map(|d| wire::ServiceDeployment {
                service_deployment_arn: Some(d.service_deployment_arn.clone()),
                service_arn: Some(d.service_arn.clone()),
                cluster_arn: Some(d.cluster_arn.clone()),
                status: Some(d.status.clone()),
                ..Default::default()
            })
            .collect();
        let wire_failures: Vec<wire::Failure> = failure_arns
            .iter()
            .map(|arn| wire::Failure {
                arn: Some(arn.clone()),
                reason: Some("MISSING".to_string()),
                detail: None,
            })
            .collect();
        wire::serialize_describe_service_deployments_response(
            &wire::DescribeServiceDeploymentsResponse {
                service_deployments: Some(wire_deployments),
                failures: Some(wire_failures),
                ..Default::default()
            },
        )
    }

    async fn handle_describe_service_revisions(
        &self,
        state: &Arc<tokio::sync::RwLock<EcsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_service_revisions_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let revision_arns: Vec<&str> = input
            .service_revision_arns
            .iter()
            .map(|s| s.as_str())
            .collect();
        let state_guard = state.read().await;
        let (found, failure_arns) = state_guard.describe_service_revisions(&revision_arns);
        let wire_revisions: Vec<wire::ServiceRevision> = found
            .iter()
            .map(|r| wire::ServiceRevision {
                service_revision_arn: Some(r.service_revision_arn.clone()),
                service_arn: Some(r.service_arn.clone()),
                cluster_arn: Some(r.cluster_arn.clone()),
                ..Default::default()
            })
            .collect();
        let wire_failures: Vec<wire::Failure> = failure_arns
            .iter()
            .map(|arn| wire::Failure {
                arn: Some(arn.clone()),
                reason: Some("MISSING".to_string()),
                detail: None,
            })
            .collect();
        wire::serialize_describe_service_revisions_response(
            &wire::DescribeServiceRevisionsResponse {
                service_revisions: Some(wire_revisions),
                failures: Some(wire_failures),
                ..Default::default()
            },
        )
    }

    async fn handle_list_service_deployments(
        &self,
        state: &Arc<tokio::sync::RwLock<EcsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_service_deployments_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let service_arn = if input.service.is_empty() {
            None
        } else {
            Some(input.service.as_str())
        };
        let state_guard = state.read().await;
        let deployments = state_guard.list_service_deployments(service_arn);
        let wire_deployments: Vec<wire::ServiceDeploymentBrief> = deployments
            .iter()
            .map(|d| wire::ServiceDeploymentBrief {
                service_deployment_arn: Some(d.service_deployment_arn.clone()),
                service_arn: Some(d.service_arn.clone()),
                cluster_arn: Some(d.cluster_arn.clone()),
                status: Some(d.status.clone()),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_service_deployments_response(&wire::ListServiceDeploymentsResponse {
            service_deployments: Some(wire_deployments),
            ..Default::default()
        })
    }

    async fn handle_stop_service_deployment(
        &self,
        state: &Arc<tokio::sync::RwLock<EcsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_stop_service_deployment_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let service_deployment_arn = input.service_deployment_arn;
        let mut state_guard = state.write().await;
        let _ = state_guard.stop_service_deployment(&service_deployment_arn);
        wire::serialize_stop_service_deployment_response(&wire::StopServiceDeploymentResponse {
            service_deployment_arn: Some(service_deployment_arn),
            ..Default::default()
        })
    }

    // ========================================================================
    // ExpressGatewayService handlers
    // ========================================================================

    async fn handle_create_express_gateway_service(
        &self,
        state: &Arc<tokio::sync::RwLock<EcsState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_express_gateway_service_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let service_name = match input.service_name.as_deref() {
            Some(n) if !n.is_empty() => n,
            _ => {
                return json_error_response(
                    400,
                    "InvalidParameterException",
                    "Missing 'serviceName'",
                );
            }
        };
        let cluster = input.cluster.as_deref().unwrap_or("default");

        let mut state = state.write().await;
        match state.create_express_gateway_service(service_name, cluster, account_id, region) {
            Ok(svc) => {
                let wire_svc = to_wire_express_gateway_service(svc);
                wire::serialize_create_express_gateway_service_response(
                    &wire::CreateExpressGatewayServiceResponse {
                        service: Some(wire_svc),
                        ..Default::default()
                    },
                )
            }
            Err(e) => ecs_error_response(&e),
        }
    }

    async fn handle_delete_express_gateway_service(
        &self,
        state: &Arc<tokio::sync::RwLock<EcsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_express_gateway_service_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.service_arn.is_empty() {
            return json_error_response(400, "InvalidParameterException", "Missing 'serviceArn'");
        }

        let mut state = state.write().await;
        match state.delete_express_gateway_service(&input.service_arn) {
            Ok(svc) => {
                let wire_svc = to_wire_express_gateway_service(&svc);
                wire::serialize_delete_express_gateway_service_response(
                    &wire::DeleteExpressGatewayServiceResponse {
                        service: Some(wire_svc),
                        ..Default::default()
                    },
                )
            }
            Err(e) => ecs_error_response(&e),
        }
    }

    async fn handle_describe_express_gateway_service(
        &self,
        state: &Arc<tokio::sync::RwLock<EcsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_express_gateway_service_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.service_arn.is_empty() {
            return json_error_response(400, "InvalidParameterException", "Missing 'serviceArn'");
        }

        let state = state.read().await;
        match state.describe_express_gateway_service(&input.service_arn) {
            Ok(svc) => {
                let wire_svc = to_wire_express_gateway_service(svc);
                wire::serialize_describe_express_gateway_service_response(
                    &wire::DescribeExpressGatewayServiceResponse {
                        service: Some(wire_svc),
                        ..Default::default()
                    },
                )
            }
            Err(e) => ecs_error_response(&e),
        }
    }

    async fn handle_update_express_gateway_service(
        &self,
        state: &Arc<tokio::sync::RwLock<EcsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_express_gateway_service_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.service_arn.is_empty() {
            return json_error_response(400, "InvalidParameterException", "Missing 'serviceArn'");
        }

        let mut state = state.write().await;
        match state.update_express_gateway_service(&input.service_arn) {
            Ok(svc) => {
                let wire_svc = to_wire_updated_express_gateway_service(svc);
                wire::serialize_update_express_gateway_service_response(
                    &wire::UpdateExpressGatewayServiceResponse {
                        service: Some(wire_svc),
                        ..Default::default()
                    },
                )
            }
            Err(e) => ecs_error_response(&e),
        }
    }
}

// ============================================================================
// Wire conversion helpers (free functions)
// ============================================================================

fn to_wire_task_definition(td: &TaskDefinition) -> wire::TaskDefinition {
    let containers: Vec<wire::ContainerDefinition> = td
        .container_definitions
        .iter()
        .map(|cd| {
            let port_mappings: Vec<wire::PortMapping> = cd
                .port_mappings
                .iter()
                .map(|pm| wire::PortMapping {
                    container_port: Some(pm.container_port),
                    host_port: Some(pm.host_port),
                    protocol: Some(pm.protocol.clone()),
                    ..Default::default()
                })
                .collect();
            let environment: Vec<wire::KeyValuePair> = cd
                .environment
                .iter()
                .map(|e| wire::KeyValuePair {
                    name: Some(e.name.clone()),
                    value: Some(e.value.clone()),
                })
                .collect();

            let log_configuration =
                cd.log_configuration
                    .as_ref()
                    .map(|lc| wire::LogConfiguration {
                        log_driver: lc.log_driver.clone(),
                        ..Default::default()
                    });

            wire::ContainerDefinition {
                name: Some(cd.name.clone()),
                image: Some(cd.image.clone()),
                cpu: Some(cd.cpu),
                essential: Some(cd.essential),
                port_mappings: Some(port_mappings),
                environment: Some(environment),
                mount_points: Some(vec![]),
                volumes_from: Some(vec![]),
                memory: cd.memory,
                memory_reservation: cd.memory_reservation,
                log_configuration,
                ..Default::default()
            }
        })
        .collect();

    let mut compatibilities = vec!["EC2".to_string()];
    let requires_compatibilities = if !td.requires_compatibilities.is_empty() {
        for rc in &td.requires_compatibilities {
            if rc != "EC2" && !compatibilities.contains(rc) {
                compatibilities.push(rc.clone());
            }
        }
        Some(td.requires_compatibilities.clone())
    } else {
        None
    };

    wire::TaskDefinition {
        task_definition_arn: Some(td.arn.clone()),
        family: Some(td.family.clone()),
        revision: Some(td.revision),
        container_definitions: Some(containers),
        status: Some(td.status.clone()),
        volumes: Some(vec![]),
        placement_constraints: Some(vec![]),
        compatibilities: Some(compatibilities),
        network_mode: Some(td.network_mode.clone()),
        task_role_arn: td.task_role_arn.clone(),
        execution_role_arn: td.execution_role_arn.clone(),
        requires_compatibilities,
        cpu: td.cpu.clone(),
        memory: td.memory.clone(),
        ..Default::default()
    }
}

fn to_wire_cluster(c: &EcsCluster) -> wire::Cluster {
    let tags: Vec<wire::Tag> = c
        .tags
        .iter()
        .map(|t| wire::Tag {
            key: Some(t.key.clone()),
            value: Some(t.value.clone()),
        })
        .collect();

    wire::Cluster {
        cluster_name: Some(c.name.clone()),
        cluster_arn: Some(c.arn.clone()),
        status: Some(c.status.clone()),
        registered_container_instances_count: Some(c.registered_container_instances_count),
        running_tasks_count: Some(c.running_tasks_count),
        active_services_count: Some(0),
        pending_tasks_count: Some(0),
        capacity_providers: if c.capacity_providers.is_empty() {
            None
        } else {
            Some(c.capacity_providers.clone())
        },
        tags: if tags.is_empty() { None } else { Some(tags) },
        ..Default::default()
    }
}

fn to_wire_service(svc: &EcsServiceDef) -> wire::Service {
    let tags: Vec<wire::Tag> = svc
        .tags
        .iter()
        .map(|t| wire::Tag {
            key: Some(t.key.clone()),
            value: Some(t.value.clone()),
        })
        .collect();

    let wire_lbs: Vec<wire::LoadBalancer> = svc
        .load_balancers
        .iter()
        .map(|lb| wire::LoadBalancer {
            target_group_arn: lb.target_group_arn.clone(),
            load_balancer_name: lb.load_balancer_name.clone(),
            container_name: lb.container_name.clone(),
            container_port: lb.container_port,
            ..Default::default()
        })
        .collect();

    wire::Service {
        service_name: Some(svc.name.clone()),
        service_arn: Some(svc.arn.clone()),
        cluster_arn: Some(svc.cluster_arn.clone()),
        task_definition: Some(svc.task_definition.clone()),
        desired_count: Some(svc.desired_count),
        running_count: Some(svc.running_count),
        pending_count: Some(svc.pending_count),
        status: Some(svc.status.clone()),
        launch_type: Some(svc.launch_type.clone()),
        scheduling_strategy: Some(svc.scheduling_strategy.clone()),
        events: Some(vec![]),
        load_balancers: Some(wire_lbs),
        deployments: Some(vec![wire::Deployment {
            desired_count: Some(svc.desired_count),
            pending_count: Some(svc.pending_count),
            running_count: Some(svc.running_count),
            status: Some("PRIMARY".to_string()),
            launch_type: Some(svc.launch_type.clone()),
            ..Default::default()
        }]),
        tags: if tags.is_empty() { None } else { Some(tags) },
        ..Default::default()
    }
}

fn to_wire_capacity_provider(cp: &EcsCapacityProvider) -> wire::CapacityProvider {
    let tags: Vec<wire::Tag> = cp
        .tags
        .iter()
        .map(|t| wire::Tag {
            key: Some(t.key.clone()),
            value: Some(t.value.clone()),
        })
        .collect();

    wire::CapacityProvider {
        name: Some(cp.name.clone()),
        capacity_provider_arn: Some(cp.arn.clone()),
        status: Some(cp.status.clone()),
        auto_scaling_group_provider: Some(wire::AutoScalingGroupProvider {
            auto_scaling_group_arn: cp.auto_scaling_group_arn.clone(),
            ..Default::default()
        }),
        tags: if tags.is_empty() { None } else { Some(tags) },
        ..Default::default()
    }
}

fn to_wire_container_instance(ci: &EcsContainerInstance) -> wire::ContainerInstance {
    wire::ContainerInstance {
        container_instance_arn: Some(ci.arn.clone()),
        ec2_instance_id: Some(ci.ec2_instance_id.clone()),
        status: Some(ci.status.clone()),
        running_tasks_count: Some(ci.running_tasks_count),
        pending_tasks_count: Some(ci.pending_tasks_count),
        agent_connected: Some(ci.agent_connected),
        version: Some(ci.version),
        ..Default::default()
    }
}

fn to_wire_task(t: &EcsTask) -> wire::Task {
    let containers: Vec<wire::Container> = t
        .containers
        .iter()
        .map(|c| wire::Container {
            container_arn: Some(c.container_arn.clone()),
            name: Some(c.name.clone()),
            last_status: Some(c.last_status.clone()),
            task_arn: Some(c.task_arn.clone()),
            ..Default::default()
        })
        .collect();

    let tags: Vec<wire::Tag> = t
        .tags
        .iter()
        .map(|tag| wire::Tag {
            key: Some(tag.key.clone()),
            value: Some(tag.value.clone()),
        })
        .collect();

    wire::Task {
        task_arn: Some(t.task_arn.clone()),
        task_definition_arn: Some(t.task_definition_arn.clone()),
        cluster_arn: Some(t.cluster_arn.clone()),
        container_instance_arn: t.container_instance_arn.clone(),
        last_status: Some(t.last_status.clone()),
        desired_status: Some(t.desired_status.clone()),
        started_by: t.started_by.clone(),
        group: t.group.clone(),
        launch_type: Some(t.launch_type.clone()),
        containers: Some(containers),
        tags: if tags.is_empty() { None } else { Some(tags) },
        stopped_reason: t.stopped_reason.clone(),
        ..Default::default()
    }
}

fn to_wire_task_set(ts: &EcsTaskSet) -> wire::TaskSet {
    let tags: Vec<wire::Tag> = ts
        .tags
        .iter()
        .map(|t| wire::Tag {
            key: Some(t.key.clone()),
            value: Some(t.value.clone()),
        })
        .collect();

    wire::TaskSet {
        id: Some(ts.id.clone()),
        task_set_arn: Some(ts.task_set_arn.clone()),
        service_arn: Some(ts.service_arn.clone()),
        cluster_arn: Some(ts.cluster_arn.clone()),
        task_definition: Some(ts.task_definition.clone()),
        status: Some(ts.status.clone()),
        scale: Some(wire::Scale {
            value: Some(ts.scale_value),
            unit: Some(ts.scale_unit.clone()),
        }),
        running_count: Some(ts.running_count),
        pending_count: Some(ts.pending_count),
        launch_type: Some(ts.launch_type.clone()),
        tags: if tags.is_empty() { None } else { Some(tags) },
        ..Default::default()
    }
}

fn to_wire_express_gateway_service(
    svc: &EcsExpressGatewayService,
) -> wire::ECSExpressGatewayService {
    wire::ECSExpressGatewayService {
        service_arn: Some(svc.service_arn.clone()),
        service_name: Some(svc.service_name.clone()),
        cluster: Some(svc.cluster.clone()),
        created_at: Some(svc.created_at),
        updated_at: Some(svc.updated_at),
        status: Some(wire::ExpressGatewayServiceStatus {
            status_code: Some("ACTIVE".to_string()),
            ..Default::default()
        }),
        ..Default::default()
    }
}

fn to_wire_updated_express_gateway_service(
    svc: &EcsExpressGatewayService,
) -> wire::UpdatedExpressGatewayService {
    wire::UpdatedExpressGatewayService {
        service_arn: Some(svc.service_arn.clone()),
        service_name: Some(svc.service_name.clone()),
        cluster: Some(svc.cluster.clone()),
        created_at: Some(svc.created_at),
        updated_at: Some(svc.updated_at),
        status: Some(wire::ExpressGatewayServiceStatus {
            status_code: Some("ACTIVE".to_string()),
            ..Default::default()
        }),
        ..Default::default()
    }
}

fn ecs_error_response(err: &EcsError) -> MockResponse {
    let (status, error_type) = match err {
        EcsError::ClusterNotFound { .. } => (400, "ClusterNotFoundException"),
        EcsError::ClusterNotFoundPlain => (400, "ClusterNotFoundException"),
        EcsError::TaskDefinitionNotDescribable { .. } => (400, "ClientException"),
        EcsError::TaskDefinitionNotFound { .. } => (400, "InvalidParameterException"),
        EcsError::ServiceAlreadyExists { .. } => (400, "InvalidParameterException"),
        EcsError::ServiceNotFound { .. } => (400, "ServiceNotFoundException"),
        EcsError::ServiceNotFoundPlain => (400, "ServiceNotFoundException"),
        EcsError::CapacityProviderAlreadyExists { .. } => (400, "InvalidParameterException"),
        EcsError::CapacityProviderNotFound { .. } => (400, "InvalidParameterException"),
        EcsError::ContainerInstanceNotFound { .. } => (400, "InvalidParameterException"),
        EcsError::TaskNotFound { .. } => (400, "InvalidParameterException"),
        EcsError::TaskSetNotFound { .. } => (400, "InvalidParameterException"),
        EcsError::ResourceNotFound { .. } => (400, "InvalidParameterException"),
        EcsError::TaskDefinitionResourceNotFound => (400, "ClientException"),
        EcsError::ExpressGatewayServiceAlreadyExists { .. } => (400, "InvalidParameterException"),
        EcsError::ExpressGatewayServiceNotFound { .. } => (400, "InvalidParameterException"),
    };
    let body = json!({
        "__type": error_type,
        "message": err.to_string(),
    });
    MockResponse::json(status, body.to_string())
}

fn json_error_response(status: u16, code: &str, message: &str) -> MockResponse {
    let body = json!({
        "__type": code,
        "message": message,
    });
    MockResponse::json(status, body.to_string())
}
