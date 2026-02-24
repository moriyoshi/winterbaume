use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::Value;
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
    StatefulService, json_error_response,
};

use crate::model::{
    Certificate as ModelCertificate, Connection as ModelConnection, Endpoint as ModelEndpoint,
    EventSubscription as ModelEventSubscription, ReplicationInstance as ModelReplicationInstance,
    ReplicationSubnetGroup as ModelSubnetGroup, ReplicationTask as ModelReplicationTask, Subnet,
    Tag,
};
use crate::state::{DmsError, DmsState};
use crate::views::DmsStateView;
use crate::wire;

pub struct DatabaseMigrationService {
    pub(crate) state: Arc<BackendState<DmsState>>,
    pub(crate) notifier: StateChangeNotifier<DmsStateView>,
}

impl DatabaseMigrationService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for DatabaseMigrationService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for DatabaseMigrationService {
    fn service_name(&self) -> &str {
        "dms"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://dms\..*\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

fn dms_error_response(err: &DmsError) -> MockResponse {
    let (status, error_type) = match err {
        DmsError::ReplicationInstanceAlreadyExists(_) => (400, "ResourceAlreadyExistsFault"),
        DmsError::ReplicationInstanceNotFound(_) => (400, "ResourceNotFoundFault"),
        DmsError::EndpointAlreadyExists(_) => (400, "ResourceAlreadyExistsFault"),
        DmsError::EndpointNotFound(_) => (400, "ResourceNotFoundFault"),
        DmsError::ReplicationTaskAlreadyExists(_) => (400, "ResourceAlreadyExistsFault"),
        DmsError::ReplicationTaskNotFound(_) => (400, "ResourceNotFoundFault"),
        DmsError::ReplicationSubnetGroupAlreadyExists(_) => (400, "ResourceAlreadyExistsFault"),
        DmsError::ReplicationSubnetGroupNotFound(_) => (400, "ResourceNotFoundFault"),
        DmsError::CertificateAlreadyExists(_) => (400, "ResourceAlreadyExistsFault"),
        DmsError::CertificateNotFound(_) => (400, "ResourceNotFoundFault"),
        DmsError::EventSubscriptionAlreadyExists(_) => (400, "ResourceAlreadyExistsFault"),
        DmsError::EventSubscriptionNotFound(_) => (400, "ResourceNotFoundFault"),
    };
    json_error_response(status, error_type, &err.to_string())
}

fn wire_tags_to_map(tags: Option<&[Tag]>) -> HashMap<String, String> {
    tags.map(|arr| {
        arr.iter()
            .filter_map(|t| {
                let key = t.key.as_deref()?;
                let value = t.value.as_deref().unwrap_or("");
                Some((key.to_string(), value.to_string()))
            })
            .collect()
    })
    .unwrap_or_default()
}

impl DatabaseMigrationService {
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

        // Validate the body is well-formed JSON up-front; the typed deserialisers in
        // `wire` re-parse the bytes per operation.
        if serde_json::from_slice::<Value>(&request.body).is_err() {
            return json_error_response(400, "SerializationException", "Invalid JSON body");
        }
        let body_bytes: &[u8] = &request.body;

        let state = self.state.get(account_id, &region);

        let is_mutating = matches!(
            action.as_str(),
            "CreateReplicationInstance"
                | "DeleteReplicationInstance"
                | "ModifyReplicationInstance"
                | "CreateEndpoint"
                | "DeleteEndpoint"
                | "ModifyEndpoint"
                | "CreateReplicationTask"
                | "DeleteReplicationTask"
                | "ModifyReplicationTask"
                | "StartReplicationTask"
                | "StopReplicationTask"
                | "TestConnection"
                | "AddTagsToResource"
                | "RemoveTagsFromResource"
                | "CreateReplicationSubnetGroup"
                | "DeleteReplicationSubnetGroup"
                | "ModifyReplicationSubnetGroup"
                | "ImportCertificate"
                | "DeleteCertificate"
                | "CreateEventSubscription"
                | "DeleteEventSubscription"
                | "ModifyEventSubscription"
        );

        let response = match action.as_str() {
            "CreateReplicationInstance" => {
                self.handle_create_replication_instance(&state, body_bytes, account_id, &region)
                    .await
            }
            "DescribeReplicationInstances" => {
                self.handle_describe_replication_instances(&state).await
            }
            "DeleteReplicationInstance" => {
                self.handle_delete_replication_instance(&state, body_bytes)
                    .await
            }
            "ModifyReplicationInstance" => {
                self.handle_modify_replication_instance(&state, body_bytes)
                    .await
            }
            "CreateEndpoint" => {
                self.handle_create_endpoint(&state, body_bytes, account_id, &region)
                    .await
            }
            "DescribeEndpoints" => self.handle_describe_endpoints(&state).await,
            "DeleteEndpoint" => self.handle_delete_endpoint(&state, body_bytes).await,
            "ModifyEndpoint" => self.handle_modify_endpoint(&state, body_bytes).await,
            "CreateReplicationTask" => {
                self.handle_create_replication_task(&state, body_bytes, account_id, &region)
                    .await
            }
            "DescribeReplicationTasks" => self.handle_describe_replication_tasks(&state).await,
            "DeleteReplicationTask" => {
                self.handle_delete_replication_task(&state, body_bytes)
                    .await
            }
            "ModifyReplicationTask" => {
                self.handle_modify_replication_task(&state, body_bytes)
                    .await
            }
            "StartReplicationTask" => self.handle_start_replication_task(&state, body_bytes).await,
            "StopReplicationTask" => self.handle_stop_replication_task(&state, body_bytes).await,
            "DescribeConnections" => self.handle_describe_connections(&state).await,
            "TestConnection" => self.handle_test_connection(&state, body_bytes).await,
            "AddTagsToResource" => self.handle_add_tags_to_resource(&state, body_bytes).await,
            "ListTagsForResource" => self.handle_list_tags_for_resource(&state, body_bytes).await,
            "RemoveTagsFromResource" => {
                self.handle_remove_tags_from_resource(&state, body_bytes)
                    .await
            }
            "DescribeReplicationSubnetGroups" => {
                self.handle_describe_replication_subnet_groups(&state).await
            }
            "CreateReplicationSubnetGroup" => {
                self.handle_create_replication_subnet_group(&state, body_bytes, account_id, &region)
                    .await
            }
            "DeleteReplicationSubnetGroup" => {
                self.handle_delete_replication_subnet_group(&state, body_bytes)
                    .await
            }
            "ModifyReplicationSubnetGroup" => {
                self.handle_modify_replication_subnet_group(&state, body_bytes)
                    .await
            }
            "ImportCertificate" => {
                self.handle_import_certificate(&state, body_bytes, account_id, &region)
                    .await
            }
            "DescribeCertificates" => self.handle_describe_certificates(&state).await,
            "DeleteCertificate" => self.handle_delete_certificate(&state, body_bytes).await,
            "CreateEventSubscription" => {
                self.handle_create_event_subscription(&state, body_bytes, account_id)
                    .await
            }
            "DescribeEventSubscriptions" => self.handle_describe_event_subscriptions(&state).await,
            "DeleteEventSubscription" => {
                self.handle_delete_event_subscription(&state, body_bytes)
                    .await
            }
            "ModifyEventSubscription" => {
                self.handle_modify_event_subscription(&state, body_bytes)
                    .await
            }
            "DescribeAccountAttributes" => self.handle_describe_account_attributes(&state).await,
            "DescribeEventCategories" => self.handle_describe_event_categories(&state).await,
            "DescribeOrderableReplicationInstances" => {
                self.handle_describe_orderable_replication_instances(&state)
                    .await
            }
            "DescribeEngineVersions" => self.handle_describe_engine_versions(&state).await,
            "DescribeEndpointTypes" => self.handle_describe_endpoint_types(&state).await,
            "DescribeEndpointSettings" => self.handle_describe_endpoint_settings(&state).await,
            "DescribePendingMaintenanceActions" => {
                self.handle_describe_pending_maintenance_actions(&state)
                    .await
            }
            "DescribeApplicableIndividualAssessments" => {
                self.handle_describe_applicable_individual_assessments(&state)
                    .await
            }
            "DescribeEvents" => self.handle_describe_events(&state).await,
            "DescribeReplicationTaskAssessmentResults" => {
                self.handle_describe_replication_task_assessment_results(&state)
                    .await
            }
            "DescribeReplicationTaskAssessmentRuns" => {
                self.handle_describe_replication_task_assessment_runs(&state)
                    .await
            }
            "DescribeReplicationTaskIndividualAssessments" => {
                self.handle_describe_replication_task_individual_assessments(&state)
                    .await
            }
            // --- Unimplemented operations (auto-generated stubs) ---
            _ => json_error_response(
                501,
                "NotImplementedError",
                &format!("{action} is not yet implemented in winterbaume-dms"),
            ),
        };

        if is_mutating && response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }

        response
    }

    async fn handle_create_replication_instance(
        &self,
        state: &Arc<tokio::sync::RwLock<DmsState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_replication_instance_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.replication_instance_identifier.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing ReplicationInstanceIdentifier",
            );
        }
        if input.replication_instance_class.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing ReplicationInstanceClass",
            );
        }
        let allocated_storage = input.allocated_storage;
        let availability_zone = input.availability_zone.as_deref();
        let publicly_accessible = input.publicly_accessible.unwrap_or(true);
        let multi_az = input.multi_a_z.unwrap_or(false);
        let engine_version = input.engine_version.as_deref();
        let tags = wire_tags_to_map(input.tags.as_deref());

        let mut s = state.write().await;
        match s.create_replication_instance(
            &input.replication_instance_identifier,
            &input.replication_instance_class,
            allocated_storage,
            availability_zone,
            publicly_accessible,
            multi_az,
            engine_version,
            account_id,
            region,
            tags,
        ) {
            Ok(instance) => {
                let model_instance = to_model_replication_instance(instance);
                wire::serialize_create_replication_instance_response(
                    &crate::model::CreateReplicationInstanceResponse {
                        replication_instance: Some(model_instance),
                    },
                )
            }
            Err(e) => dms_error_response(&e),
        }
    }

    async fn handle_describe_replication_instances(
        &self,
        state: &Arc<tokio::sync::RwLock<DmsState>>,
    ) -> MockResponse {
        let s = state.read().await;
        let instances: Vec<ModelReplicationInstance> = s
            .describe_replication_instances()
            .into_iter()
            .map(to_model_replication_instance)
            .collect();
        wire::serialize_describe_replication_instances_response(
            &crate::model::DescribeReplicationInstancesResponse {
                replication_instances: Some(instances),
                marker: None,
            },
        )
    }

    async fn handle_delete_replication_instance(
        &self,
        state: &Arc<tokio::sync::RwLock<DmsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_replication_instance_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.replication_instance_arn.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing ReplicationInstanceArn",
            );
        }
        let arn = input.replication_instance_arn.as_str();
        // Find by ARN
        let identifier = {
            let s = state.read().await;
            s.replication_instances
                .values()
                .find(|i| i.replication_instance_arn == arn)
                .map(|i| i.replication_instance_identifier.clone())
        };
        let identifier = match identifier {
            Some(id) => id,
            None => {
                return dms_error_response(&DmsError::ReplicationInstanceNotFound(arn.to_string()));
            }
        };
        let mut s = state.write().await;
        match s.delete_replication_instance(&identifier) {
            Ok(instance) => {
                let model_instance = to_model_replication_instance(&instance);
                wire::serialize_delete_replication_instance_response(
                    &crate::model::DeleteReplicationInstanceResponse {
                        replication_instance: Some(model_instance),
                    },
                )
            }
            Err(e) => dms_error_response(&e),
        }
    }

    async fn handle_modify_replication_instance(
        &self,
        state: &Arc<tokio::sync::RwLock<DmsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_modify_replication_instance_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.replication_instance_arn.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing ReplicationInstanceArn",
            );
        }
        let arn = input.replication_instance_arn.as_str();
        let identifier = {
            let s = state.read().await;
            s.replication_instances
                .values()
                .find(|i| i.replication_instance_arn == arn)
                .map(|i| i.replication_instance_identifier.clone())
        };
        let identifier = match identifier {
            Some(id) => id,
            None => {
                return dms_error_response(&DmsError::ReplicationInstanceNotFound(arn.to_string()));
            }
        };
        let instance_class = input.replication_instance_class.as_deref();
        let allocated_storage = input.allocated_storage;
        let multi_az = input.multi_a_z;
        let mut s = state.write().await;
        match s.modify_replication_instance(
            &identifier,
            instance_class,
            allocated_storage,
            multi_az,
        ) {
            Ok(instance) => {
                let model_instance = to_model_replication_instance(instance);
                wire::serialize_modify_replication_instance_response(
                    &crate::model::ModifyReplicationInstanceResponse {
                        replication_instance: Some(model_instance),
                    },
                )
            }
            Err(e) => dms_error_response(&e),
        }
    }

    async fn handle_create_endpoint(
        &self,
        state: &Arc<tokio::sync::RwLock<DmsState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_endpoint_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.endpoint_identifier.is_empty() {
            return json_error_response(400, "ValidationException", "Missing EndpointIdentifier");
        }
        if input.endpoint_type.is_empty() {
            return json_error_response(400, "ValidationException", "Missing EndpointType");
        }
        if input.engine_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing EngineName");
        }
        let username = input.username.as_deref();
        let server_name = input.server_name.as_deref();
        let port = input.port;
        let database_name = input.database_name.as_deref();
        let extra_connection_attributes = input.extra_connection_attributes.as_deref();
        let tags = wire_tags_to_map(input.tags.as_deref());

        let mut s = state.write().await;
        match s.create_endpoint(
            &input.endpoint_identifier,
            &input.endpoint_type,
            &input.engine_name,
            username,
            server_name,
            port,
            database_name,
            extra_connection_attributes,
            account_id,
            region,
            tags,
        ) {
            Ok(endpoint) => {
                let model_endpoint = to_model_endpoint(endpoint);
                wire::serialize_create_endpoint_response(&crate::model::CreateEndpointResponse {
                    endpoint: Some(model_endpoint),
                })
            }
            Err(e) => dms_error_response(&e),
        }
    }

    async fn handle_describe_endpoints(
        &self,
        state: &Arc<tokio::sync::RwLock<DmsState>>,
    ) -> MockResponse {
        let s = state.read().await;
        let endpoints: Vec<ModelEndpoint> = s
            .describe_endpoints()
            .into_iter()
            .map(to_model_endpoint)
            .collect();
        wire::serialize_describe_endpoints_response(&crate::model::DescribeEndpointsResponse {
            endpoints: Some(endpoints),
            marker: None,
        })
    }

    async fn handle_delete_endpoint(
        &self,
        state: &Arc<tokio::sync::RwLock<DmsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_endpoint_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.endpoint_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing EndpointArn");
        }
        let mut s = state.write().await;
        match s.delete_endpoint(&input.endpoint_arn) {
            Ok(endpoint) => {
                let model_endpoint = to_model_endpoint(&endpoint);
                wire::serialize_delete_endpoint_response(&crate::model::DeleteEndpointResponse {
                    endpoint: Some(model_endpoint),
                })
            }
            Err(e) => dms_error_response(&e),
        }
    }

    async fn handle_modify_endpoint(
        &self,
        state: &Arc<tokio::sync::RwLock<DmsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_modify_endpoint_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.endpoint_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing EndpointArn");
        }
        let endpoint_type = input.endpoint_type.as_deref();
        let engine_name = input.engine_name.as_deref();
        let username = input.username.as_deref();
        let server_name = input.server_name.as_deref();
        let port = input.port;
        let database_name = input.database_name.as_deref();

        let mut s = state.write().await;
        match s.modify_endpoint(
            &input.endpoint_arn,
            endpoint_type,
            engine_name,
            username,
            server_name,
            port,
            database_name,
        ) {
            Ok(endpoint) => {
                let model_endpoint = to_model_endpoint(endpoint);
                wire::serialize_modify_endpoint_response(&crate::model::ModifyEndpointResponse {
                    endpoint: Some(model_endpoint),
                })
            }
            Err(e) => dms_error_response(&e),
        }
    }

    async fn handle_create_replication_task(
        &self,
        state: &Arc<tokio::sync::RwLock<DmsState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_replication_task_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.replication_task_identifier.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing ReplicationTaskIdentifier",
            );
        }
        if input.source_endpoint_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing SourceEndpointArn");
        }
        if input.target_endpoint_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing TargetEndpointArn");
        }
        if input.replication_instance_arn.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing ReplicationInstanceArn",
            );
        }
        if input.migration_type.is_empty() {
            return json_error_response(400, "ValidationException", "Missing MigrationType");
        }
        if input.table_mappings.is_empty() {
            return json_error_response(400, "ValidationException", "Missing TableMappings");
        }
        let replication_task_settings = input.replication_task_settings.as_deref();
        let tags = wire_tags_to_map(input.tags.as_deref());

        let mut s = state.write().await;
        match s.create_replication_task(
            &input.replication_task_identifier,
            &input.source_endpoint_arn,
            &input.target_endpoint_arn,
            &input.replication_instance_arn,
            &input.migration_type,
            &input.table_mappings,
            replication_task_settings,
            account_id,
            region,
            tags,
        ) {
            Ok(task) => {
                let model_task = to_model_replication_task(task);
                wire::serialize_create_replication_task_response(
                    &crate::model::CreateReplicationTaskResponse {
                        replication_task: Some(model_task),
                    },
                )
            }
            Err(e) => dms_error_response(&e),
        }
    }

    async fn handle_describe_replication_tasks(
        &self,
        state: &Arc<tokio::sync::RwLock<DmsState>>,
    ) -> MockResponse {
        let s = state.read().await;
        let tasks: Vec<ModelReplicationTask> = s
            .describe_replication_tasks()
            .into_iter()
            .map(to_model_replication_task)
            .collect();
        wire::serialize_describe_replication_tasks_response(
            &crate::model::DescribeReplicationTasksResponse {
                replication_tasks: Some(tasks),
                marker: None,
            },
        )
    }

    async fn handle_delete_replication_task(
        &self,
        state: &Arc<tokio::sync::RwLock<DmsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_replication_task_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.replication_task_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing ReplicationTaskArn");
        }
        let mut s = state.write().await;
        match s.delete_replication_task(&input.replication_task_arn) {
            Ok(task) => {
                let model_task = to_model_replication_task(&task);
                wire::serialize_delete_replication_task_response(
                    &crate::model::DeleteReplicationTaskResponse {
                        replication_task: Some(model_task),
                    },
                )
            }
            Err(e) => dms_error_response(&e),
        }
    }

    async fn handle_modify_replication_task(
        &self,
        state: &Arc<tokio::sync::RwLock<DmsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_modify_replication_task_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.replication_task_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing ReplicationTaskArn");
        }
        let migration_type = input.migration_type.as_deref();
        let table_mappings = input.table_mappings.as_deref();
        let replication_task_settings = input.replication_task_settings.as_deref();

        let mut s = state.write().await;
        match s.modify_replication_task(
            &input.replication_task_arn,
            migration_type,
            table_mappings,
            replication_task_settings,
        ) {
            Ok(task) => {
                let model_task = to_model_replication_task(task);
                wire::serialize_modify_replication_task_response(
                    &crate::model::ModifyReplicationTaskResponse {
                        replication_task: Some(model_task),
                    },
                )
            }
            Err(e) => dms_error_response(&e),
        }
    }

    async fn handle_start_replication_task(
        &self,
        state: &Arc<tokio::sync::RwLock<DmsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_start_replication_task_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.replication_task_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing ReplicationTaskArn");
        }
        let mut s = state.write().await;
        match s.start_replication_task(&input.replication_task_arn) {
            Ok(task) => {
                let model_task = to_model_replication_task(task);
                wire::serialize_start_replication_task_response(
                    &crate::model::StartReplicationTaskResponse {
                        replication_task: Some(model_task),
                    },
                )
            }
            Err(e) => dms_error_response(&e),
        }
    }

    async fn handle_stop_replication_task(
        &self,
        state: &Arc<tokio::sync::RwLock<DmsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_stop_replication_task_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.replication_task_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing ReplicationTaskArn");
        }
        let mut s = state.write().await;
        match s.stop_replication_task(&input.replication_task_arn) {
            Ok(task) => {
                let model_task = to_model_replication_task(task);
                wire::serialize_stop_replication_task_response(
                    &crate::model::StopReplicationTaskResponse {
                        replication_task: Some(model_task),
                    },
                )
            }
            Err(e) => dms_error_response(&e),
        }
    }

    async fn handle_describe_connections(
        &self,
        state: &Arc<tokio::sync::RwLock<DmsState>>,
    ) -> MockResponse {
        let s = state.read().await;
        let connections: Vec<ModelConnection> = s
            .describe_connections()
            .into_iter()
            .map(to_model_connection)
            .collect();
        wire::serialize_describe_connections_response(&crate::model::DescribeConnectionsResponse {
            connections: Some(connections),
            marker: None,
        })
    }

    async fn handle_test_connection(
        &self,
        state: &Arc<tokio::sync::RwLock<DmsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_test_connection_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.replication_instance_arn.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing ReplicationInstanceArn",
            );
        }
        if input.endpoint_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing EndpointArn");
        }
        let mut s = state.write().await;
        match s.test_connection(&input.replication_instance_arn, &input.endpoint_arn) {
            Ok(conn) => {
                let model_conn = to_model_connection(conn);
                wire::serialize_test_connection_response(&crate::model::TestConnectionResponse {
                    connection: Some(model_conn),
                })
            }
            Err(e) => dms_error_response(&e),
        }
    }

    async fn handle_add_tags_to_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<DmsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_add_tags_to_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing ResourceArn");
        }
        let tags = wire_tags_to_map(Some(input.tags.as_slice()));
        let mut s = state.write().await;
        s.add_tags_to_resource(&input.resource_arn, tags);
        wire::serialize_add_tags_to_resource_response(&crate::model::AddTagsToResourceResponse {})
    }

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<DmsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let resource_arn = match input.resource_arn.as_deref() {
            Some(a) if !a.is_empty() => a,
            _ => {
                return json_error_response(400, "ValidationException", "Missing ResourceArn");
            }
        };
        let s = state.read().await;
        let tag_list = s.list_tags_for_resource(resource_arn);
        wire::serialize_list_tags_for_resource_response(
            &crate::model::ListTagsForResourceResponse {
                tag_list: Some(tag_list),
            },
        )
    }

    async fn handle_remove_tags_from_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<DmsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_remove_tags_from_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing ResourceArn");
        }
        let mut s = state.write().await;
        s.remove_tags_from_resource(&input.resource_arn, &input.tag_keys);
        wire::serialize_remove_tags_from_resource_response(
            &crate::model::RemoveTagsFromResourceResponse {},
        )
    }

    async fn handle_describe_replication_subnet_groups(
        &self,
        state: &Arc<tokio::sync::RwLock<DmsState>>,
    ) -> MockResponse {
        let s = state.read().await;
        let groups: Vec<ModelSubnetGroup> = s
            .describe_replication_subnet_groups()
            .into_iter()
            .map(to_model_subnet_group)
            .collect();
        wire::serialize_describe_replication_subnet_groups_response(
            &crate::model::DescribeReplicationSubnetGroupsResponse {
                replication_subnet_groups: Some(groups),
                marker: None,
            },
        )
    }

    async fn handle_create_replication_subnet_group(
        &self,
        state: &Arc<tokio::sync::RwLock<DmsState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_replication_subnet_group_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.replication_subnet_group_identifier.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing ReplicationSubnetGroupIdentifier",
            );
        }
        let description = input.replication_subnet_group_description.as_str();
        let subnet_ids = input.subnet_ids;
        // VpcId not in this Smithy request - keep as None to preserve prior behaviour.
        let vpc_id: Option<&str> = None;
        let tags = wire_tags_to_map(input.tags.as_deref());

        let mut s = state.write().await;
        match s.create_replication_subnet_group(
            &input.replication_subnet_group_identifier,
            description,
            subnet_ids,
            vpc_id,
            account_id,
            region,
            tags,
        ) {
            Ok(group) => {
                let model_group = to_model_subnet_group(group);
                wire::serialize_create_replication_subnet_group_response(
                    &crate::model::CreateReplicationSubnetGroupResponse {
                        replication_subnet_group: Some(model_group),
                    },
                )
            }
            Err(e) => dms_error_response(&e),
        }
    }

    async fn handle_delete_replication_subnet_group(
        &self,
        state: &Arc<tokio::sync::RwLock<DmsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_replication_subnet_group_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.replication_subnet_group_identifier.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing ReplicationSubnetGroupIdentifier",
            );
        }
        let mut s = state.write().await;
        match s.delete_replication_subnet_group(&input.replication_subnet_group_identifier) {
            Ok(()) => wire::serialize_delete_replication_subnet_group_response(
                &crate::model::DeleteReplicationSubnetGroupResponse {},
            ),
            Err(e) => dms_error_response(&e),
        }
    }

    async fn handle_modify_replication_subnet_group(
        &self,
        state: &Arc<tokio::sync::RwLock<DmsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_modify_replication_subnet_group_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.replication_subnet_group_identifier.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing ReplicationSubnetGroupIdentifier",
            );
        }
        let description = input.replication_subnet_group_description.as_deref();
        // The wire model has SubnetIds as a required Vec<String> with default = empty,
        // so we treat an empty vec as "no change".
        let subnet_ids = if input.subnet_ids.is_empty() {
            None
        } else {
            Some(input.subnet_ids)
        };
        let mut s = state.write().await;
        match s.modify_replication_subnet_group(
            &input.replication_subnet_group_identifier,
            description,
            subnet_ids,
        ) {
            Ok(group) => {
                let model_group = to_model_subnet_group(group);
                wire::serialize_modify_replication_subnet_group_response(
                    &crate::model::ModifyReplicationSubnetGroupResponse {
                        replication_subnet_group: Some(model_group),
                    },
                )
            }
            Err(e) => dms_error_response(&e),
        }
    }

    async fn handle_import_certificate(
        &self,
        state: &Arc<tokio::sync::RwLock<DmsState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_import_certificate_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.certificate_identifier.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing CertificateIdentifier",
            );
        }
        let certificate_pem = input.certificate_pem.as_deref();
        let certificate_wallet = input.certificate_wallet.as_deref();
        let kms_key_id = input.kms_key_id.as_deref();
        let tags = wire_tags_to_map(input.tags.as_deref());
        let mut s = state.write().await;
        match s.import_certificate(
            &input.certificate_identifier,
            certificate_pem,
            certificate_wallet,
            kms_key_id,
            account_id,
            region,
            tags,
        ) {
            Ok(cert) => {
                let model_cert = to_model_certificate(cert);
                wire::serialize_import_certificate_response(
                    &crate::model::ImportCertificateResponse {
                        certificate: Some(model_cert),
                    },
                )
            }
            Err(e) => dms_error_response(&e),
        }
    }

    async fn handle_describe_certificates(
        &self,
        state: &Arc<tokio::sync::RwLock<DmsState>>,
    ) -> MockResponse {
        let s = state.read().await;
        let certs: Vec<ModelCertificate> = s
            .describe_certificates()
            .into_iter()
            .map(to_model_certificate)
            .collect();
        wire::serialize_describe_certificates_response(
            &crate::model::DescribeCertificatesResponse {
                certificates: Some(certs),
                marker: None,
            },
        )
    }

    async fn handle_delete_certificate(
        &self,
        state: &Arc<tokio::sync::RwLock<DmsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_certificate_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.certificate_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing CertificateArn");
        }
        let mut s = state.write().await;
        match s.delete_certificate(&input.certificate_arn) {
            Ok(cert) => {
                let model_cert = to_model_certificate(&cert);
                wire::serialize_delete_certificate_response(
                    &crate::model::DeleteCertificateResponse {
                        certificate: Some(model_cert),
                    },
                )
            }
            Err(e) => dms_error_response(&e),
        }
    }

    async fn handle_create_event_subscription(
        &self,
        state: &Arc<tokio::sync::RwLock<DmsState>>,
        body: &[u8],
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_event_subscription_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.subscription_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing SubscriptionName");
        }
        if input.sns_topic_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing SnsTopicArn");
        }
        let source_type = input.source_type.as_deref();
        let event_categories = input.event_categories.unwrap_or_default();
        let source_ids = input.source_ids.unwrap_or_default();
        let enabled = input.enabled.unwrap_or(true);
        let mut s = state.write().await;
        match s.create_event_subscription(
            &input.subscription_name,
            &input.sns_topic_arn,
            source_type,
            event_categories,
            source_ids,
            enabled,
            account_id,
        ) {
            Ok(sub) => {
                let model_sub = to_model_event_subscription(sub);
                wire::serialize_create_event_subscription_response(
                    &crate::model::CreateEventSubscriptionResponse {
                        event_subscription: Some(model_sub),
                    },
                )
            }
            Err(e) => dms_error_response(&e),
        }
    }

    async fn handle_describe_event_subscriptions(
        &self,
        state: &Arc<tokio::sync::RwLock<DmsState>>,
    ) -> MockResponse {
        let s = state.read().await;
        let subs: Vec<ModelEventSubscription> = s
            .describe_event_subscriptions()
            .into_iter()
            .map(to_model_event_subscription)
            .collect();
        wire::serialize_describe_event_subscriptions_response(
            &crate::model::DescribeEventSubscriptionsResponse {
                event_subscriptions_list: Some(subs),
                marker: None,
            },
        )
    }

    async fn handle_delete_event_subscription(
        &self,
        state: &Arc<tokio::sync::RwLock<DmsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_event_subscription_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.subscription_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing SubscriptionName");
        }
        let mut s = state.write().await;
        match s.delete_event_subscription(&input.subscription_name) {
            Ok(sub) => {
                let model_sub = to_model_event_subscription(&sub);
                wire::serialize_delete_event_subscription_response(
                    &crate::model::DeleteEventSubscriptionResponse {
                        event_subscription: Some(model_sub),
                    },
                )
            }
            Err(e) => dms_error_response(&e),
        }
    }

    async fn handle_modify_event_subscription(
        &self,
        state: &Arc<tokio::sync::RwLock<DmsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_modify_event_subscription_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.subscription_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing SubscriptionName");
        }
        let sns_topic_arn = input.sns_topic_arn.as_deref();
        let source_type = input.source_type.as_deref();
        let event_categories = input.event_categories;
        let enabled = input.enabled;
        let mut s = state.write().await;
        match s.modify_event_subscription(
            &input.subscription_name,
            sns_topic_arn,
            source_type,
            event_categories,
            enabled,
        ) {
            Ok(sub) => {
                let model_sub = to_model_event_subscription(sub);
                wire::serialize_modify_event_subscription_response(
                    &crate::model::ModifyEventSubscriptionResponse {
                        event_subscription: Some(model_sub),
                    },
                )
            }
            Err(e) => dms_error_response(&e),
        }
    }

    async fn handle_describe_account_attributes(
        &self,
        state: &Arc<tokio::sync::RwLock<DmsState>>,
    ) -> MockResponse {
        let s = state.read().await;
        let quotas = s.get_account_quotas().to_vec();
        wire::serialize_describe_account_attributes_response(
            &crate::model::DescribeAccountAttributesResponse {
                account_quotas: Some(quotas),
                unique_account_identifier: None,
            },
        )
    }

    async fn handle_describe_event_categories(
        &self,
        state: &Arc<tokio::sync::RwLock<DmsState>>,
    ) -> MockResponse {
        let s = state.read().await;
        let groups = s.get_event_category_groups().to_vec();
        wire::serialize_describe_event_categories_response(
            &crate::model::DescribeEventCategoriesResponse {
                event_category_group_list: Some(groups),
            },
        )
    }

    async fn handle_describe_orderable_replication_instances(
        &self,
        state: &Arc<tokio::sync::RwLock<DmsState>>,
    ) -> MockResponse {
        let s = state.read().await;
        let instances = s.get_orderable_replication_instances().to_vec();
        wire::serialize_describe_orderable_replication_instances_response(
            &crate::model::DescribeOrderableReplicationInstancesResponse {
                orderable_replication_instances: Some(instances),
                marker: None,
            },
        )
    }

    async fn handle_describe_engine_versions(
        &self,
        state: &Arc<tokio::sync::RwLock<DmsState>>,
    ) -> MockResponse {
        let s = state.read().await;
        let versions = s.get_engine_versions().to_vec();
        wire::serialize_describe_engine_versions_response(
            &crate::model::DescribeEngineVersionsResponse {
                engine_versions: Some(versions),
                marker: None,
            },
        )
    }

    async fn handle_describe_endpoint_types(
        &self,
        state: &Arc<tokio::sync::RwLock<DmsState>>,
    ) -> MockResponse {
        let s = state.read().await;
        let types = s.get_supported_endpoint_types().to_vec();
        wire::serialize_describe_endpoint_types_response(
            &crate::model::DescribeEndpointTypesResponse {
                supported_endpoint_types: Some(types),
                marker: None,
            },
        )
    }

    async fn handle_describe_endpoint_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<DmsState>>,
    ) -> MockResponse {
        let s = state.read().await;
        let settings = s.get_endpoint_settings().to_vec();
        wire::serialize_describe_endpoint_settings_response(
            &crate::model::DescribeEndpointSettingsResponse {
                endpoint_settings: Some(settings),
                marker: None,
            },
        )
    }

    async fn handle_describe_pending_maintenance_actions(
        &self,
        state: &Arc<tokio::sync::RwLock<DmsState>>,
    ) -> MockResponse {
        let s = state.read().await;
        let actions = s.get_pending_maintenance_actions().to_vec();
        wire::serialize_describe_pending_maintenance_actions_response(
            &crate::model::DescribePendingMaintenanceActionsResponse {
                pending_maintenance_actions: Some(actions),
                marker: None,
            },
        )
    }

    async fn handle_describe_applicable_individual_assessments(
        &self,
        state: &Arc<tokio::sync::RwLock<DmsState>>,
    ) -> MockResponse {
        let s = state.read().await;
        let names = s.get_individual_assessment_names().to_vec();
        wire::serialize_describe_applicable_individual_assessments_response(
            &crate::model::DescribeApplicableIndividualAssessmentsResponse {
                individual_assessment_names: Some(names),
                marker: None,
            },
        )
    }

    async fn handle_describe_events(
        &self,
        state: &Arc<tokio::sync::RwLock<DmsState>>,
    ) -> MockResponse {
        let s = state.read().await;
        let events = s.get_events().to_vec();
        wire::serialize_describe_events_response(&crate::model::DescribeEventsResponse {
            events: Some(events),
            marker: None,
        })
    }

    async fn handle_describe_replication_task_assessment_results(
        &self,
        state: &Arc<tokio::sync::RwLock<DmsState>>,
    ) -> MockResponse {
        let s = state.read().await;
        let results = s.get_replication_task_assessment_results().to_vec();
        wire::serialize_describe_replication_task_assessment_results_response(
            &crate::model::DescribeReplicationTaskAssessmentResultsResponse {
                replication_task_assessment_results: Some(results),
                bucket_name: None,
                marker: None,
            },
        )
    }

    async fn handle_describe_replication_task_assessment_runs(
        &self,
        state: &Arc<tokio::sync::RwLock<DmsState>>,
    ) -> MockResponse {
        let s = state.read().await;
        let runs = s.get_replication_task_assessment_runs().to_vec();
        wire::serialize_describe_replication_task_assessment_runs_response(
            &crate::model::DescribeReplicationTaskAssessmentRunsResponse {
                replication_task_assessment_runs: Some(runs),
                marker: None,
            },
        )
    }

    async fn handle_describe_replication_task_individual_assessments(
        &self,
        state: &Arc<tokio::sync::RwLock<DmsState>>,
    ) -> MockResponse {
        let s = state.read().await;
        let assessments = s.get_replication_task_individual_assessments().to_vec();
        wire::serialize_describe_replication_task_individual_assessments_response(
            &crate::model::DescribeReplicationTaskIndividualAssessmentsResponse {
                replication_task_individual_assessments: Some(assessments),
                marker: None,
            },
        )
    }
}

// ---- Conversion helpers ----

fn to_model_replication_instance(
    i: &crate::types::ReplicationInstance,
) -> ModelReplicationInstance {
    ModelReplicationInstance {
        replication_instance_identifier: Some(i.replication_instance_identifier.clone()),
        replication_instance_class: Some(i.replication_instance_class.clone()),
        allocated_storage: Some(i.allocated_storage),
        replication_instance_arn: Some(i.replication_instance_arn.clone()),
        availability_zone: i.availability_zone.clone(),
        publicly_accessible: Some(i.publicly_accessible),
        multi_a_z: Some(i.multi_az),
        engine_version: i.engine_version.clone(),
        instance_create_time: Some(i.instance_create_time),
        replication_instance_status: Some(i.status.clone()),
        ..Default::default()
    }
}

fn to_model_endpoint(e: &crate::types::Endpoint) -> ModelEndpoint {
    ModelEndpoint {
        endpoint_identifier: Some(e.endpoint_identifier.clone()),
        endpoint_type: Some(e.endpoint_type.clone()),
        engine_name: Some(e.engine_name.clone()),
        username: e.username.clone(),
        server_name: e.server_name.clone(),
        port: e.port,
        database_name: e.database_name.clone(),
        extra_connection_attributes: e.extra_connection_attributes.clone(),
        endpoint_arn: Some(e.endpoint_arn.clone()),
        status: Some(e.status.clone()),
        ..Default::default()
    }
}

fn to_model_replication_task(t: &crate::types::ReplicationTask) -> ModelReplicationTask {
    ModelReplicationTask {
        replication_task_identifier: Some(t.replication_task_identifier.clone()),
        source_endpoint_arn: Some(t.source_endpoint_arn.clone()),
        target_endpoint_arn: Some(t.target_endpoint_arn.clone()),
        replication_instance_arn: Some(t.replication_instance_arn.clone()),
        migration_type: Some(t.migration_type.clone()),
        table_mappings: Some(t.table_mappings.clone()),
        replication_task_settings: t.replication_task_settings.clone(),
        status: Some(t.status.clone()),
        replication_task_arn: Some(t.replication_task_arn.clone()),
        replication_task_creation_date: Some(t.replication_task_creation_date),
        replication_task_start_date: t.replication_task_start_date,
        ..Default::default()
    }
}

fn to_model_connection(c: &crate::types::Connection) -> ModelConnection {
    ModelConnection {
        replication_instance_arn: Some(c.replication_instance_arn.clone()),
        endpoint_arn: Some(c.endpoint_arn.clone()),
        replication_instance_identifier: Some(c.replication_instance_identifier.clone()),
        endpoint_identifier: Some(c.endpoint_identifier.clone()),
        status: Some(c.status.clone()),
        ..Default::default()
    }
}

fn to_model_subnet_group(g: &crate::types::ReplicationSubnetGroup) -> ModelSubnetGroup {
    let subnets: Vec<Subnet> = g
        .subnet_ids
        .iter()
        .map(|id| Subnet {
            subnet_identifier: Some(id.clone()),
            subnet_status: Some("Active".to_string()),
            ..Default::default()
        })
        .collect();
    ModelSubnetGroup {
        replication_subnet_group_identifier: Some(g.replication_subnet_group_identifier.clone()),
        replication_subnet_group_description: Some(g.replication_subnet_group_description.clone()),
        vpc_id: g.vpc_id.clone(),
        subnet_group_status: Some("Complete".to_string()),
        subnets: Some(subnets),
        ..Default::default()
    }
}

fn to_model_certificate(c: &crate::types::Certificate) -> ModelCertificate {
    ModelCertificate {
        certificate_identifier: Some(c.certificate_identifier.clone()),
        certificate_arn: Some(c.certificate_arn.clone()),
        certificate_pem: c.certificate_pem.clone(),
        certificate_wallet: c.certificate_wallet.clone(),
        kms_key_id: c.kms_key_id.clone(),
        certificate_creation_date: Some(c.certificate_creation_date),
        ..Default::default()
    }
}

fn to_model_event_subscription(s: &crate::types::EventSubscription) -> ModelEventSubscription {
    ModelEventSubscription {
        cust_subscription_id: Some(s.subscription_name.clone()),
        customer_aws_id: Some(s.customer_aws_id.clone()),
        sns_topic_arn: Some(s.sns_topic_arn.clone()),
        source_type: s.source_type.clone(),
        event_categories_list: if s.event_categories.is_empty() {
            None
        } else {
            Some(s.event_categories.clone())
        },
        source_ids_list: if s.source_ids.is_empty() {
            None
        } else {
            Some(s.source_ids.clone())
        },
        enabled: Some(s.enabled),
        status: Some(s.status.clone()),
        subscription_creation_time: Some(s.subscription_creation_time.clone()),
    }
}

// Ensure Tag is used in model (suppress unused import warning)
fn _tags_to_model(tags: &HashMap<String, String>, arn: &str) -> Vec<Tag> {
    tags.iter()
        .map(|(k, v)| Tag {
            key: Some(k.clone()),
            value: Some(v.clone()),
            resource_arn: Some(arn.to_string()),
        })
        .collect()
}
