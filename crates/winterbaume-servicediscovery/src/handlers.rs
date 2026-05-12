use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, StatefulService,
    default_account_id,
};

use crate::model;
use crate::state::{ServiceDiscoveryError, ServiceDiscoveryState};
use crate::types;
use crate::views::ServiceDiscoveryStateView;
use crate::wire;

pub struct ServiceDiscoveryService {
    pub(crate) state: Arc<BackendState<ServiceDiscoveryState>>,
    pub(crate) notifier: StateChangeNotifier<ServiceDiscoveryStateView>,
}

impl ServiceDiscoveryService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for ServiceDiscoveryService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for ServiceDiscoveryService {
    fn service_name(&self) -> &str {
        "servicediscovery"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://(data-)?servicediscovery\..*\.amazonaws\.com",
            r"https?://servicediscovery\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl ServiceDiscoveryService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();

        // Extract action from X-Amz-Target header
        // Format: "Route53AutoNaming_v20170314.CreatePrivateDnsNamespace"
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

        // Validate the body is well-formed JSON up-front; the typed deserialisers in
        // `wire` re-parse the bytes per operation.
        if serde_json::from_slice::<Value>(&request.body).is_err() {
            return json_error_response(400, "SerializationException", "Invalid JSON body");
        }
        let body_bytes: &[u8] = &request.body;

        let state = self.state.get(account_id, &region);

        let response = match action.as_str() {
            "CreatePrivateDnsNamespace" => {
                self.handle_create_private_dns_namespace(&state, body_bytes, account_id, &region)
                    .await
            }
            "CreateHttpNamespace" => {
                self.handle_create_http_namespace(&state, body_bytes, account_id, &region)
                    .await
            }
            "CreatePublicDnsNamespace" => {
                self.handle_create_public_dns_namespace(&state, body_bytes, account_id, &region)
                    .await
            }
            "GetNamespace" => self.handle_get_namespace(&state, body_bytes).await,
            "DeleteNamespace" => self.handle_delete_namespace(&state, body_bytes).await,
            "ListNamespaces" => self.handle_list_namespaces(&state).await,
            "UpdateHttpNamespace" => self.handle_update_http_namespace(&state, body_bytes).await,
            "UpdatePrivateDnsNamespace" => {
                self.handle_update_private_dns_namespace(&state, body_bytes)
                    .await
            }
            "UpdatePublicDnsNamespace" => {
                self.handle_update_public_dns_namespace(&state, body_bytes)
                    .await
            }
            "CreateService" => {
                self.handle_create_service(&state, body_bytes, account_id, &region)
                    .await
            }
            "GetService" => self.handle_get_service(&state, body_bytes).await,
            "DeleteService" => self.handle_delete_service(&state, body_bytes).await,
            "ListServices" => self.handle_list_services(&state).await,
            "UpdateService" => self.handle_update_service(&state, body_bytes).await,
            "RegisterInstance" => self.handle_register_instance(&state, body_bytes).await,
            "DeregisterInstance" => self.handle_deregister_instance(&state, body_bytes).await,
            "GetInstance" => self.handle_get_instance(&state, body_bytes).await,
            "ListInstances" => self.handle_list_instances(&state, body_bytes).await,
            "GetInstancesHealthStatus" => {
                self.handle_get_instances_health_status(&state, body_bytes)
                    .await
            }
            "UpdateInstanceCustomHealthStatus" => {
                self.handle_update_instance_custom_health_status(&state, body_bytes)
                    .await
            }
            "DiscoverInstances" => self.handle_discover_instances(&state, body_bytes).await,
            "DiscoverInstancesRevision" => {
                self.handle_discover_instances_revision(&state, body_bytes)
                    .await
            }
            "GetOperation" => self.handle_get_operation(&state, body_bytes).await,
            "ListOperations" => self.handle_list_operations(&state).await,
            "TagResource" => self.handle_tag_resource(&state, body_bytes).await,
            "UntagResource" => self.handle_untag_resource(&state, body_bytes).await,
            "ListTagsForResource" => self.handle_list_tags_for_resource(&state, body_bytes).await,
            // --- Unimplemented operations (auto-generated stubs) ---
            "DeleteServiceAttributes" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteServiceAttributes is not yet implemented in winterbaume-servicediscovery",
            ),
            "GetServiceAttributes" => json_error_response(
                501,
                "NotImplementedError",
                "GetServiceAttributes is not yet implemented in winterbaume-servicediscovery",
            ),
            "UpdateServiceAttributes" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateServiceAttributes is not yet implemented in winterbaume-servicediscovery",
            ),
            _ => json_error_response(
                400,
                "InvalidAction",
                &format!("Could not find operation {action} for ServiceDiscovery"),
            ),
        };

        let is_mutating = !matches!(
            action.as_str(),
            "GetNamespace"
                | "ListNamespaces"
                | "GetService"
                | "ListServices"
                | "GetInstance"
                | "ListInstances"
                | "GetInstancesHealthStatus"
                | "DiscoverInstances"
                | "DiscoverInstancesRevision"
                | "GetOperation"
                | "ListOperations"
                | "ListTagsForResource"
        );
        if is_mutating && response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_create_private_dns_namespace(
        &self,
        state: &Arc<tokio::sync::RwLock<ServiceDiscoveryState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_private_dns_namespace_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "InvalidInput", "Missing required parameter 'Name'");
        }
        if input.vpc.is_empty() {
            return json_error_response(400, "InvalidInput", "Missing required parameter 'Vpc'");
        }

        let description = input.description.as_deref();
        let creator_request_id = input.creator_request_id.as_deref();
        let soa_ttl = input
            .properties
            .as_ref()
            .map(|p| p.dns_properties.s_o_a.t_t_l);
        let tags = tags_to_map(input.tags.as_deref());

        let mut state = state.write().await;
        match state.create_private_dns_namespace(
            &input.name,
            &input.vpc,
            description,
            creator_request_id,
            soa_ttl,
            tags,
            account_id,
            region,
        ) {
            Ok(op) => wire::serialize_create_private_dns_namespace_response(
                &wire::CreatePrivateDnsNamespaceResponse {
                    operation_id: Some(op.id.clone()),
                },
            ),
            Err(e) => sd_error_response(&e),
        }
    }

    async fn handle_create_http_namespace(
        &self,
        state: &Arc<tokio::sync::RwLock<ServiceDiscoveryState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_http_namespace_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "InvalidInput", "Missing required parameter 'Name'");
        }

        let description = input.description.as_deref();
        let creator_request_id = input.creator_request_id.as_deref();
        let tags = tags_to_map(input.tags.as_deref());

        let mut state = state.write().await;
        match state.create_http_namespace(
            &input.name,
            description,
            creator_request_id,
            tags,
            account_id,
            region,
        ) {
            Ok(op) => {
                wire::serialize_create_http_namespace_response(&wire::CreateHttpNamespaceResponse {
                    operation_id: Some(op.id.clone()),
                })
            }
            Err(e) => sd_error_response(&e),
        }
    }

    async fn handle_create_public_dns_namespace(
        &self,
        state: &Arc<tokio::sync::RwLock<ServiceDiscoveryState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_public_dns_namespace_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "InvalidInput", "Missing required parameter 'Name'");
        }

        let description = input.description.as_deref();
        let creator_request_id = input.creator_request_id.as_deref();
        let soa_ttl = input
            .properties
            .as_ref()
            .map(|p| p.dns_properties.s_o_a.t_t_l);
        let tags = tags_to_map(input.tags.as_deref());

        let mut state = state.write().await;
        match state.create_public_dns_namespace(
            &input.name,
            description,
            creator_request_id,
            soa_ttl,
            tags,
            account_id,
            region,
        ) {
            Ok(op) => wire::serialize_create_public_dns_namespace_response(
                &wire::CreatePublicDnsNamespaceResponse {
                    operation_id: Some(op.id.clone()),
                },
            ),
            Err(e) => sd_error_response(&e),
        }
    }

    async fn handle_get_namespace(
        &self,
        state: &Arc<tokio::sync::RwLock<ServiceDiscoveryState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_namespace_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.id.is_empty() {
            return json_error_response(400, "InvalidInput", "Missing required parameter 'Id'");
        }

        let state = state.read().await;
        match state.get_namespace(&input.id) {
            Ok(ns) => wire::serialize_get_namespace_response(&wire::GetNamespaceResponse {
                namespace: Some(namespace_to_wire(ns)),
            }),
            Err(e) => sd_error_response(&e),
        }
    }

    async fn handle_delete_namespace(
        &self,
        state: &Arc<tokio::sync::RwLock<ServiceDiscoveryState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_namespace_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.id.is_empty() {
            return json_error_response(400, "InvalidInput", "Missing required parameter 'Id'");
        }

        let mut state = state.write().await;
        match state.delete_namespace(&input.id) {
            Ok(op_id) => {
                wire::serialize_delete_namespace_response(&wire::DeleteNamespaceResponse {
                    operation_id: Some(op_id),
                })
            }
            Err(e) => sd_error_response(&e),
        }
    }

    async fn handle_list_namespaces(
        &self,
        state: &Arc<tokio::sync::RwLock<ServiceDiscoveryState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let namespaces = state.list_namespaces();
        let entries: Vec<wire::NamespaceSummary> = namespaces
            .iter()
            .map(|ns| namespace_to_summary(ns))
            .collect();

        wire::serialize_list_namespaces_response(&wire::ListNamespacesResponse {
            namespaces: Some(entries),
            next_token: None,
        })
    }

    async fn handle_update_http_namespace(
        &self,
        state: &Arc<tokio::sync::RwLock<ServiceDiscoveryState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_http_namespace_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.id.is_empty() {
            return json_error_response(400, "InvalidInput", "Missing required parameter 'Id'");
        }

        let description = if input.namespace.description.is_empty() {
            None
        } else {
            Some(input.namespace.description.as_str())
        };

        let mut state = state.write().await;
        match state.update_http_namespace(&input.id, description) {
            Ok(op) => {
                wire::serialize_update_http_namespace_response(&wire::UpdateHttpNamespaceResponse {
                    operation_id: Some(op.id.clone()),
                })
            }
            Err(e) => sd_error_response(&e),
        }
    }

    async fn handle_update_private_dns_namespace(
        &self,
        state: &Arc<tokio::sync::RwLock<ServiceDiscoveryState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_private_dns_namespace_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.id.is_empty() {
            return json_error_response(400, "InvalidInput", "Missing required parameter 'Id'");
        }

        let description = input.namespace.description.as_deref();
        let soa_ttl = input
            .namespace
            .properties
            .as_ref()
            .map(|p| p.dns_properties.s_o_a.t_t_l);

        let mut state = state.write().await;
        match state.update_private_dns_namespace(&input.id, description, soa_ttl) {
            Ok(op) => wire::serialize_update_private_dns_namespace_response(
                &wire::UpdatePrivateDnsNamespaceResponse {
                    operation_id: Some(op.id.clone()),
                },
            ),
            Err(e) => sd_error_response(&e),
        }
    }

    async fn handle_update_public_dns_namespace(
        &self,
        state: &Arc<tokio::sync::RwLock<ServiceDiscoveryState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_public_dns_namespace_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.id.is_empty() {
            return json_error_response(400, "InvalidInput", "Missing required parameter 'Id'");
        }

        let description = input.namespace.description.as_deref();
        let soa_ttl = input
            .namespace
            .properties
            .as_ref()
            .map(|p| p.dns_properties.s_o_a.t_t_l);

        let mut state = state.write().await;
        match state.update_public_dns_namespace(&input.id, description, soa_ttl) {
            Ok(op) => wire::serialize_update_public_dns_namespace_response(
                &wire::UpdatePublicDnsNamespaceResponse {
                    operation_id: Some(op.id.clone()),
                },
            ),
            Err(e) => sd_error_response(&e),
        }
    }

    async fn handle_create_service(
        &self,
        state: &Arc<tokio::sync::RwLock<ServiceDiscoveryState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_service_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "InvalidInput", "Missing required parameter 'Name'");
        }

        let namespace_id = input.namespace_id.as_deref();
        let description = input.description.as_deref();
        let creator_request_id = input.creator_request_id.as_deref();
        let service_type_override = input.r#type.as_deref();
        let tags = tags_to_map(input.tags.as_deref());

        let dns_config = input.dns_config.as_ref().map(model_dns_config_to_entry);
        let health_check_config = input
            .health_check_config
            .as_ref()
            .map(model_health_check_config_to_entry);
        let health_check_custom_config = input
            .health_check_custom_config
            .as_ref()
            .map(model_health_check_custom_config_to_entry);

        let mut state = state.write().await;
        match state.create_service(
            &input.name,
            namespace_id,
            description,
            creator_request_id,
            dns_config,
            health_check_config,
            health_check_custom_config,
            service_type_override,
            tags,
            account_id,
            region,
        ) {
            Ok(svc) => wire::serialize_create_service_response(&wire::CreateServiceResponse {
                service: Some(service_to_wire(svc)),
            }),
            Err(e) => sd_error_response(&e),
        }
    }

    async fn handle_get_service(
        &self,
        state: &Arc<tokio::sync::RwLock<ServiceDiscoveryState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_service_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.id.is_empty() {
            return json_error_response(400, "InvalidInput", "Missing required parameter 'Id'");
        }

        let state = state.read().await;
        match state.get_service(&input.id) {
            Ok(svc) => wire::serialize_get_service_response(&wire::GetServiceResponse {
                service: Some(service_to_wire(svc)),
            }),
            Err(e) => sd_error_response(&e),
        }
    }

    async fn handle_delete_service(
        &self,
        state: &Arc<tokio::sync::RwLock<ServiceDiscoveryState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_service_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.id.is_empty() {
            return json_error_response(400, "InvalidInput", "Missing required parameter 'Id'");
        }

        let mut state = state.write().await;
        match state.delete_service(&input.id) {
            Ok(()) => wire::serialize_delete_service_response(&wire::DeleteServiceResponse {}),
            Err(e) => sd_error_response(&e),
        }
    }

    async fn handle_list_services(
        &self,
        state: &Arc<tokio::sync::RwLock<ServiceDiscoveryState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let services = state.list_services();
        let entries: Vec<wire::ServiceSummary> =
            services.iter().map(|svc| service_to_summary(svc)).collect();

        wire::serialize_list_services_response(&wire::ListServicesResponse {
            services: Some(entries),
            next_token: None,
        })
    }

    async fn handle_update_service(
        &self,
        state: &Arc<tokio::sync::RwLock<ServiceDiscoveryState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_service_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.id.is_empty() {
            return json_error_response(400, "InvalidInput", "Missing required parameter 'Id'");
        }

        let description = input.service.description.as_deref();
        let has_dns_config = input.service.dns_config.is_some();
        let dns_config_change = input.service.dns_config.as_ref().map(|dc| {
            dc.dns_records
                .iter()
                .map(|r| types::DnsRecordEntry {
                    record_type: r.r#type.clone(),
                    ttl: r.t_t_l,
                })
                .collect()
        });
        let health_check_config = input
            .service
            .health_check_config
            .as_ref()
            .map(model_health_check_config_to_entry);

        let mut state = state.write().await;
        match state.update_service(
            &input.id,
            description,
            dns_config_change,
            has_dns_config,
            health_check_config,
        ) {
            Ok(op) => wire::serialize_update_service_response(&wire::UpdateServiceResponse {
                operation_id: Some(op.id.clone()),
            }),
            Err(e) => sd_error_response(&e),
        }
    }

    async fn handle_register_instance(
        &self,
        state: &Arc<tokio::sync::RwLock<ServiceDiscoveryState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_register_instance_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.service_id.is_empty() {
            return json_error_response(
                400,
                "InvalidInput",
                "Missing required parameter 'ServiceId'",
            );
        }
        if input.instance_id.is_empty() {
            return json_error_response(
                400,
                "InvalidInput",
                "Missing required parameter 'InstanceId'",
            );
        }

        let creator_request_id = input.creator_request_id.as_deref();

        let mut state = state.write().await;
        match state.register_instance(
            &input.service_id,
            &input.instance_id,
            creator_request_id,
            input.attributes,
        ) {
            Ok(op) => wire::serialize_register_instance_response(&wire::RegisterInstanceResponse {
                operation_id: Some(op.id.clone()),
            }),
            Err(e) => sd_error_response(&e),
        }
    }

    async fn handle_deregister_instance(
        &self,
        state: &Arc<tokio::sync::RwLock<ServiceDiscoveryState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_deregister_instance_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.service_id.is_empty() {
            return json_error_response(
                400,
                "InvalidInput",
                "Missing required parameter 'ServiceId'",
            );
        }
        if input.instance_id.is_empty() {
            return json_error_response(
                400,
                "InvalidInput",
                "Missing required parameter 'InstanceId'",
            );
        }

        let mut state = state.write().await;
        match state.deregister_instance(&input.service_id, &input.instance_id) {
            Ok(op) => {
                wire::serialize_deregister_instance_response(&wire::DeregisterInstanceResponse {
                    operation_id: Some(op.id.clone()),
                })
            }
            Err(e) => sd_error_response(&e),
        }
    }

    async fn handle_get_instance(
        &self,
        state: &Arc<tokio::sync::RwLock<ServiceDiscoveryState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_instance_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.service_id.is_empty() {
            return json_error_response(
                400,
                "InvalidInput",
                "Missing required parameter 'ServiceId'",
            );
        }
        if input.instance_id.is_empty() {
            return json_error_response(
                400,
                "InvalidInput",
                "Missing required parameter 'InstanceId'",
            );
        }

        let state = state.read().await;
        match state.get_instance(&input.service_id, &input.instance_id) {
            Ok(inst) => wire::serialize_get_instance_response(&wire::GetInstanceResponse {
                instance: Some(instance_to_wire(inst)),
                ..Default::default()
            }),
            Err(e) => sd_error_response(&e),
        }
    }

    async fn handle_list_instances(
        &self,
        state: &Arc<tokio::sync::RwLock<ServiceDiscoveryState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_instances_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.service_id.is_empty() {
            return json_error_response(
                400,
                "InvalidInput",
                "Missing required parameter 'ServiceId'",
            );
        }

        let max_results = input.max_results.map(|n| n as usize);
        let next_token = input.next_token.as_deref();

        let state = state.read().await;
        match state.list_instances(&input.service_id) {
            Ok(instances) => {
                let mut all_entries: Vec<wire::InstanceSummary> = instances
                    .iter()
                    .map(|inst| wire::InstanceSummary {
                        id: Some(inst.id.clone()),
                        attributes: if inst.attributes.is_empty() {
                            None
                        } else {
                            Some(inst.attributes.clone())
                        },
                        ..Default::default()
                    })
                    .collect();

                // Sort for stable pagination
                all_entries.sort_by(|a, b| a.id.cmp(&b.id));

                // Apply pagination
                let start_idx = if let Some(token) = next_token {
                    token.parse::<usize>().unwrap_or(0)
                } else {
                    0
                };

                let page_entries: Vec<wire::InstanceSummary> =
                    all_entries.into_iter().skip(start_idx).collect::<Vec<_>>();

                let (page, next_tok) = if let Some(max) = max_results {
                    if page_entries.len() > max {
                        let page = page_entries[..max].to_vec();
                        (page, Some((start_idx + max).to_string()))
                    } else {
                        (page_entries, None)
                    }
                } else {
                    (page_entries, None)
                };

                wire::serialize_list_instances_response(&wire::ListInstancesResponse {
                    instances: Some(page),
                    next_token: next_tok,
                    ..Default::default()
                })
            }
            Err(e) => sd_error_response(&e),
        }
    }

    async fn handle_get_instances_health_status(
        &self,
        state: &Arc<tokio::sync::RwLock<ServiceDiscoveryState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_instances_health_status_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.service_id.is_empty() {
            return json_error_response(
                400,
                "InvalidInput",
                "Missing required parameter 'ServiceId'",
            );
        }

        let state = state.read().await;
        match state.get_instances_health_status(&input.service_id) {
            Ok(status) => wire::serialize_get_instances_health_status_response(
                &wire::GetInstancesHealthStatusResponse {
                    status: if status.is_empty() {
                        None
                    } else {
                        Some(status)
                    },
                    next_token: None,
                },
            ),
            Err(e) => sd_error_response(&e),
        }
    }

    async fn handle_update_instance_custom_health_status(
        &self,
        state: &Arc<tokio::sync::RwLock<ServiceDiscoveryState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_instance_custom_health_status_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.service_id.is_empty() {
            return json_error_response(
                400,
                "InvalidInput",
                "Missing required parameter 'ServiceId'",
            );
        }
        if input.instance_id.is_empty() {
            return json_error_response(
                400,
                "InvalidInput",
                "Missing required parameter 'InstanceId'",
            );
        }
        if input.status.is_empty() {
            return json_error_response(400, "InvalidInput", "Missing required parameter 'Status'");
        }

        let mut state = state.write().await;
        match state.update_instance_custom_health_status(
            &input.service_id,
            &input.instance_id,
            &input.status,
        ) {
            Ok(()) => wire::serialize_update_instance_custom_health_status_response(),
            Err(e) => sd_error_response(&e),
        }
    }

    async fn handle_discover_instances(
        &self,
        state: &Arc<tokio::sync::RwLock<ServiceDiscoveryState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_discover_instances_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.namespace_name.is_empty() {
            return json_error_response(
                400,
                "InvalidInput",
                "Missing required parameter 'NamespaceName'",
            );
        }
        if input.service_name.is_empty() {
            return json_error_response(
                400,
                "InvalidInput",
                "Missing required parameter 'ServiceName'",
            );
        }

        let health_status = input.health_status.as_deref();
        let query_parameters = input.query_parameters;
        let optional_parameters = input.optional_parameters;
        let max_results = input.max_results.map(|n| n as usize);

        let state = state.read().await;
        match state.discover_instances(
            &input.namespace_name,
            &input.service_name,
            health_status,
            query_parameters.as_ref(),
            optional_parameters.as_ref(),
            max_results,
        ) {
            Ok((instances, revision)) => {
                let entries: Vec<wire::HttpInstanceSummary> = instances
                    .iter()
                    .map(|(inst, ns_name, svc_name)| wire::HttpInstanceSummary {
                        instance_id: Some(inst.id.clone()),
                        namespace_name: Some(ns_name.clone()),
                        service_name: Some(svc_name.clone()),
                        health_status: Some(inst.health_status.clone()),
                        attributes: if inst.attributes.is_empty() {
                            None
                        } else {
                            Some(inst.attributes.clone())
                        },
                    })
                    .collect();

                wire::serialize_discover_instances_response(&wire::DiscoverInstancesResponse {
                    instances: Some(entries),
                    instances_revision: Some(revision),
                })
            }
            Err(e) => sd_error_response(&e),
        }
    }

    async fn handle_discover_instances_revision(
        &self,
        state: &Arc<tokio::sync::RwLock<ServiceDiscoveryState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_discover_instances_revision_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.namespace_name.is_empty() {
            return json_error_response(
                400,
                "InvalidInput",
                "Missing required parameter 'NamespaceName'",
            );
        }
        if input.service_name.is_empty() {
            return json_error_response(
                400,
                "InvalidInput",
                "Missing required parameter 'ServiceName'",
            );
        }

        let state = state.read().await;
        match state.discover_instances_revision(&input.namespace_name, &input.service_name) {
            Ok(revision) => wire::serialize_discover_instances_revision_response(
                &wire::DiscoverInstancesRevisionResponse {
                    instances_revision: Some(revision),
                },
            ),
            Err(e) => sd_error_response(&e),
        }
    }

    async fn handle_get_operation(
        &self,
        state: &Arc<tokio::sync::RwLock<ServiceDiscoveryState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_operation_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.operation_id.is_empty() {
            return json_error_response(
                400,
                "InvalidInput",
                "Missing required parameter 'OperationId'",
            );
        }

        let state = state.read().await;
        match state.get_operation(&input.operation_id) {
            Ok(op) => wire::serialize_get_operation_response(&wire::GetOperationResponse {
                operation: Some(operation_to_wire(op)),
            }),
            Err(e) => sd_error_response(&e),
        }
    }

    async fn handle_list_operations(
        &self,
        state: &Arc<tokio::sync::RwLock<ServiceDiscoveryState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let operations = state.list_operations();
        let entries: Vec<wire::OperationSummary> = operations
            .iter()
            .map(|op| wire::OperationSummary {
                id: Some(op.id.clone()),
                status: Some(op.status.clone()),
            })
            .collect();

        wire::serialize_list_operations_response(&wire::ListOperationsResponse {
            operations: Some(entries),
            next_token: None,
        })
    }

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<ServiceDiscoveryState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_a_r_n.is_empty() {
            return json_error_response(
                400,
                "InvalidInput",
                "Missing required parameter 'ResourceARN'",
            );
        }

        let tags: HashMap<String, String> =
            input.tags.into_iter().map(|t| (t.key, t.value)).collect();

        let mut state = state.write().await;
        match state.tag_resource(&input.resource_a_r_n, tags) {
            Ok(()) => wire::serialize_tag_resource_response(&wire::TagResourceResponse {}),
            Err(e) => sd_error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<ServiceDiscoveryState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_a_r_n.is_empty() {
            return json_error_response(
                400,
                "InvalidInput",
                "Missing required parameter 'ResourceARN'",
            );
        }

        let mut state = state.write().await;
        match state.untag_resource(&input.resource_a_r_n, &input.tag_keys) {
            Ok(()) => wire::serialize_untag_resource_response(&wire::UntagResourceResponse {}),
            Err(e) => sd_error_response(&e),
        }
    }

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<ServiceDiscoveryState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_a_r_n.is_empty() {
            return json_error_response(
                400,
                "InvalidInput",
                "Missing required parameter 'ResourceARN'",
            );
        }

        let state = state.read().await;
        match state.list_tags_for_resource(&input.resource_a_r_n) {
            Ok(tags) => {
                let tag_list: Vec<wire::Tag> = tags
                    .iter()
                    .map(|(k, v)| wire::Tag {
                        key: k.clone(),
                        value: v.clone(),
                    })
                    .collect();

                wire::serialize_list_tags_for_resource_response(
                    &wire::ListTagsForResourceResponse {
                        tags: if tag_list.is_empty() {
                            None
                        } else {
                            Some(tag_list)
                        },
                    },
                )
            }
            Err(e) => sd_error_response(&e),
        }
    }
}

fn build_dns_properties(ns: &crate::types::Namespace) -> wire::DnsProperties {
    wire::DnsProperties {
        hosted_zone_id: ns.hosted_zone_id.clone(),
        s_o_a: ns.soa_ttl.map(|ttl| wire::SOA { t_t_l: ttl }),
    }
}

fn build_namespace_properties(ns: &crate::types::Namespace) -> wire::NamespaceProperties {
    match ns.namespace_type.as_str() {
        "HTTP" => wire::NamespaceProperties {
            dns_properties: Some(wire::DnsProperties {
                hosted_zone_id: None,
                s_o_a: Some(wire::SOA { t_t_l: 0 }),
            }),
            http_properties: Some(wire::HttpProperties {
                http_name: Some(ns.name.clone()),
            }),
        },
        _ => wire::NamespaceProperties {
            dns_properties: Some(build_dns_properties(ns)),
            http_properties: Some(wire::HttpProperties::default()),
        },
    }
}

fn namespace_to_wire(ns: &crate::types::Namespace) -> wire::Namespace {
    wire::Namespace {
        arn: Some(ns.arn.clone()),
        create_date: Some(ns.create_date.timestamp() as f64),
        creator_request_id: ns.creator_request_id.clone(),
        description: ns.description.clone(),
        id: Some(ns.id.clone()),
        name: Some(ns.name.clone()),
        properties: Some(build_namespace_properties(ns)),
        service_count: Some(ns.service_count),
        r#type: Some(ns.namespace_type.clone()),
        ..Default::default()
    }
}

fn namespace_to_summary(ns: &crate::types::Namespace) -> wire::NamespaceSummary {
    wire::NamespaceSummary {
        arn: Some(ns.arn.clone()),
        create_date: Some(ns.create_date.timestamp() as f64),
        description: ns.description.clone(),
        id: Some(ns.id.clone()),
        name: Some(ns.name.clone()),
        properties: Some(build_namespace_properties(ns)),
        service_count: Some(ns.service_count),
        r#type: Some(ns.namespace_type.clone()),
        ..Default::default()
    }
}

fn service_to_wire(svc: &crate::types::ServiceEntry) -> wire::Service {
    let ns_id = if svc.namespace_id.is_empty() {
        None
    } else {
        Some(svc.namespace_id.clone())
    };

    wire::Service {
        arn: Some(svc.arn.clone()),
        create_date: Some(svc.create_date.timestamp() as f64),
        creator_request_id: svc.creator_request_id.clone(),
        description: svc.description.clone(),
        id: Some(svc.id.clone()),
        name: Some(svc.name.clone()),
        namespace_id: ns_id,
        instance_count: Some(svc.instance_count),
        dns_config: svc.dns_config.as_ref().map(|dc| wire::DnsConfig {
            dns_records: dc
                .dns_records
                .iter()
                .map(|r| wire::DnsRecord {
                    r#type: r.record_type.clone(),
                    t_t_l: r.ttl,
                })
                .collect(),
            namespace_id: dc.namespace_id.clone(),
            routing_policy: dc.routing_policy.clone(),
        }),
        health_check_config: svc
            .health_check_config
            .as_ref()
            .map(|hcc| wire::HealthCheckConfig {
                r#type: hcc.check_type.clone(),
                resource_path: hcc.resource_path.clone(),
                failure_threshold: hcc.failure_threshold,
            }),
        health_check_custom_config: svc.health_check_custom_config.as_ref().map(|hccc| {
            wire::HealthCheckCustomConfig {
                failure_threshold: hccc.failure_threshold,
            }
        }),
        r#type: svc.service_type.clone(),
        ..Default::default()
    }
}

fn service_to_summary(svc: &crate::types::ServiceEntry) -> wire::ServiceSummary {
    wire::ServiceSummary {
        arn: Some(svc.arn.clone()),
        create_date: Some(svc.create_date.timestamp() as f64),
        description: svc.description.clone(),
        id: Some(svc.id.clone()),
        name: Some(svc.name.clone()),
        instance_count: Some(svc.instance_count),
        dns_config: svc.dns_config.as_ref().map(|dc| wire::DnsConfig {
            dns_records: dc
                .dns_records
                .iter()
                .map(|r| wire::DnsRecord {
                    r#type: r.record_type.clone(),
                    t_t_l: r.ttl,
                })
                .collect(),
            namespace_id: dc.namespace_id.clone(),
            routing_policy: dc.routing_policy.clone(),
        }),
        health_check_config: svc
            .health_check_config
            .as_ref()
            .map(|hcc| wire::HealthCheckConfig {
                r#type: hcc.check_type.clone(),
                resource_path: hcc.resource_path.clone(),
                failure_threshold: hcc.failure_threshold,
            }),
        health_check_custom_config: svc.health_check_custom_config.as_ref().map(|hccc| {
            wire::HealthCheckCustomConfig {
                failure_threshold: hccc.failure_threshold,
            }
        }),
        r#type: svc.service_type.clone(),
        ..Default::default()
    }
}

fn instance_to_wire(inst: &crate::types::InstanceEntry) -> wire::Instance {
    wire::Instance {
        id: Some(inst.id.clone()),
        creator_request_id: inst.creator_request_id.clone(),
        attributes: if inst.attributes.is_empty() {
            None
        } else {
            Some(inst.attributes.clone())
        },
        ..Default::default()
    }
}

fn operation_to_wire(op: &crate::types::Operation) -> wire::Operation {
    wire::Operation {
        id: Some(op.id.clone()),
        r#type: Some(op.operation_type.clone()),
        status: Some(op.status.clone()),
        create_date: Some(op.create_date.timestamp() as f64),
        update_date: Some(op.create_date.timestamp() as f64),
        targets: Some(op.targets.clone()),
        ..Default::default()
    }
}

fn tags_to_map(tags: Option<&[model::Tag]>) -> HashMap<String, String> {
    let mut out = HashMap::new();
    if let Some(arr) = tags {
        for tag in arr {
            out.insert(tag.key.clone(), tag.value.clone());
        }
    }
    out
}

fn model_dns_config_to_entry(dc: &model::DnsConfig) -> types::DnsConfigEntry {
    types::DnsConfigEntry {
        namespace_id: dc.namespace_id.clone(),
        routing_policy: dc.routing_policy.clone(),
        dns_records: dc
            .dns_records
            .iter()
            .map(|r| types::DnsRecordEntry {
                record_type: r.r#type.clone(),
                ttl: r.t_t_l,
            })
            .collect(),
    }
}

fn model_health_check_config_to_entry(
    hcc: &model::HealthCheckConfig,
) -> types::HealthCheckConfigEntry {
    types::HealthCheckConfigEntry {
        check_type: hcc.r#type.clone(),
        resource_path: hcc.resource_path.clone(),
        failure_threshold: hcc.failure_threshold,
    }
}

fn model_health_check_custom_config_to_entry(
    hccc: &model::HealthCheckCustomConfig,
) -> types::HealthCheckCustomConfigEntry {
    types::HealthCheckCustomConfigEntry {
        failure_threshold: hccc.failure_threshold,
    }
}

fn sd_error_response(err: &ServiceDiscoveryError) -> MockResponse {
    let (status, error_type) = match err {
        ServiceDiscoveryError::ConflictingDomainExists { .. } => (400, "ConflictingDomainExists"),
        ServiceDiscoveryError::NamespaceAlreadyExists { .. } => (400, "NamespaceAlreadyExists"),
        ServiceDiscoveryError::NamespaceNotFound { .. } => (400, "NamespaceNotFound"),
        ServiceDiscoveryError::NamespaceInUse => (400, "ResourceInUse"),
        ServiceDiscoveryError::InvalidUpdateHttpNamespace => (400, "InvalidInput"),
        ServiceDiscoveryError::InvalidUpdatePrivateDnsNamespace => (400, "InvalidInput"),
        ServiceDiscoveryError::InvalidUpdatePublicDnsNamespace => (400, "InvalidInput"),
        ServiceDiscoveryError::ServiceAlreadyExists { .. } => (400, "ServiceAlreadyExists"),
        ServiceDiscoveryError::ServiceNotFound { .. } => (400, "ServiceNotFound"),
        ServiceDiscoveryError::ServiceInUse => (400, "ResourceInUse"),
        ServiceDiscoveryError::InstanceNotFound { .. } => (400, "InstanceNotFound"),
        ServiceDiscoveryError::OperationNotFound { .. } => (400, "OperationNotFound"),
        ServiceDiscoveryError::ResourceNotFound { .. } => (400, "ResourceNotFoundException"),
    };
    json_error_response(status, error_type, &err.to_string())
}

fn json_error_response(status: u16, code: &str, message: &str) -> MockResponse {
    let body = json!({
        "__type": code,
        "message": message,
    });
    MockResponse::json(status, body.to_string())
}
