use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::Value;
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, default_account_id,
    json_error_response,
};

macro_rules! deser_or_400 {
    ($expr:expr) => {
        match $expr {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        }
    };
}

use crate::state::{ApplicationDiscoveryError, ApplicationDiscoveryState};
use crate::types::{Application, BatchDeleteTask, ContinuousExport, ExportTask, ImportTask};
use crate::views::ApplicationDiscoveryStateView;
use crate::wire;

pub struct ApplicationDiscoveryService {
    pub(crate) state: Arc<BackendState<ApplicationDiscoveryState>>,
    pub(crate) notifier: StateChangeNotifier<ApplicationDiscoveryStateView>,
}

impl ApplicationDiscoveryService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for ApplicationDiscoveryService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for ApplicationDiscoveryService {
    fn service_name(&self) -> &str {
        "discovery"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://discovery\..*\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

type SharedState = Arc<tokio::sync::RwLock<ApplicationDiscoveryState>>;

const MUTATING_ACTIONS: &[&str] = &[
    "AssociateConfigurationItemsToApplication",
    "BatchDeleteAgents",
    "BatchDeleteImportData",
    "CreateApplication",
    "CreateTags",
    "DeleteApplications",
    "DeleteTags",
    "DisassociateConfigurationItemsFromApplication",
    "StartBatchDeleteConfigurationTask",
    "StartContinuousExport",
    "StartDataCollectionByAgentIds",
    "StartExportTask",
    "StartImportTask",
    "StopContinuousExport",
    "StopDataCollectionByAgentIds",
    "UpdateApplication",
    "ExportConfigurations",
];

impl ApplicationDiscoveryService {
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
            None => return json_error_response(400, "MissingAction", "Missing X-Amz-Target"),
        };

        let body_bytes: &[u8] = if request.body.is_empty() {
            b"{}"
        } else {
            if serde_json::from_slice::<Value>(&request.body).is_err() {
                return json_error_response(400, "SerializationException", "Invalid JSON body");
            }
            &request.body
        };

        let state = self.state.get(account_id, &region);

        let response = match action.as_str() {
            "AssociateConfigurationItemsToApplication" => {
                self.handle_associate(&state, body_bytes).await
            }
            "DisassociateConfigurationItemsFromApplication" => {
                self.handle_disassociate(&state, body_bytes).await
            }
            "BatchDeleteAgents" => self.handle_batch_delete_agents().await,
            "BatchDeleteImportData" => {
                self.handle_batch_delete_import_data(&state, body_bytes)
                    .await
            }
            "CreateApplication" => self.handle_create_application(&state, body_bytes).await,
            "CreateTags" => self.handle_create_tags(&state, body_bytes).await,
            "DeleteApplications" => self.handle_delete_applications(&state, body_bytes).await,
            "DeleteTags" => self.handle_delete_tags(&state, body_bytes).await,
            "DescribeAgents" => self.handle_describe_agents().await,
            "DescribeBatchDeleteConfigurationTask" => {
                self.handle_describe_batch_delete_task(&state, body_bytes)
                    .await
            }
            "DescribeConfigurations" => {
                self.handle_describe_configurations(&state, body_bytes)
                    .await
            }
            "DescribeContinuousExports" => self.handle_describe_continuous_exports(&state).await,
            "DescribeExportConfigurations" => {
                self.handle_describe_export_configurations(&state).await
            }
            "DescribeExportTasks" => self.handle_describe_export_tasks(&state).await,
            "DescribeImportTasks" => self.handle_describe_import_tasks(&state).await,
            "DescribeTags" => self.handle_describe_tags(&state, body_bytes).await,
            "ExportConfigurations" => self.handle_export_configurations(&state).await,
            "GetDiscoverySummary" => self.handle_get_discovery_summary(&state).await,
            "ListConfigurations" => self.handle_list_configurations(&state, body_bytes).await,
            "ListServerNeighbors" => self.handle_list_server_neighbors(body_bytes).await,
            "StartBatchDeleteConfigurationTask" => {
                self.handle_start_batch_delete_task(&state, body_bytes)
                    .await
            }
            "StartContinuousExport" => self.handle_start_continuous_export(&state).await,
            "StartDataCollectionByAgentIds" => self.handle_start_data_collection(body_bytes).await,
            "StartExportTask" => self.handle_start_export_task(&state, body_bytes).await,
            "StartImportTask" => self.handle_start_import_task(&state, body_bytes).await,
            "StopContinuousExport" => self.handle_stop_continuous_export(&state, body_bytes).await,
            "StopDataCollectionByAgentIds" => self.handle_stop_data_collection(body_bytes).await,
            "UpdateApplication" => self.handle_update_application(&state, body_bytes).await,
            other => json_error_response(
                400,
                "UnknownOperationException",
                &format!("Unknown action: {other}"),
            ),
        };

        use winterbaume_core::StatefulService;
        if response.status / 100 == 2 && MUTATING_ACTIONS.contains(&action.as_str()) {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_create_application(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = deser_or_400!(wire::deserialize_create_application_request(body));
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "name is required");
        }
        let id = format!("d-application-{}", uuid::Uuid::new_v4().simple());
        let now = chrono::Utc::now().timestamp();
        let app = Application {
            configuration_id: id.clone(),
            name: input.name,
            description: input.description,
            wave: input.wave,
            time_of_creation: now,
            last_modified_time_stamp: now,
        };
        let mut state = state.write().await;
        state.create_application(app);
        wire::serialize_create_application_response(&wire::CreateApplicationResponse {
            configuration_id: Some(id),
        })
    }

    async fn handle_update_application(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = deser_or_400!(wire::deserialize_update_application_request(body));
        if input.configuration_id.is_empty() {
            return json_error_response(400, "ValidationException", "configurationId is required");
        }
        let new_name = input.name;
        let new_desc = input.description;
        let new_wave = input.wave;
        let mut state = state.write().await;
        match state.update_application(&input.configuration_id, |a| {
            if let Some(n) = new_name {
                a.name = n;
            }
            if let Some(d) = new_desc {
                a.description = Some(d);
            }
            if let Some(w) = new_wave {
                a.wave = Some(w);
            }
        }) {
            Ok(_) => {
                wire::serialize_update_application_response(&wire::UpdateApplicationResponse {})
            }
            Err(e) => ad_error_response(&e),
        }
    }

    async fn handle_delete_applications(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = deser_or_400!(wire::deserialize_delete_applications_request(body));
        let mut state = state.write().await;
        state.delete_applications(&input.configuration_ids);
        wire::serialize_delete_applications_response(&wire::DeleteApplicationsResponse {})
    }

    async fn handle_list_configurations(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = deser_or_400!(wire::deserialize_list_configurations_request(body));
        let configuration_type = if input.configuration_type.is_empty() {
            "APPLICATION"
        } else {
            input.configuration_type.as_str()
        };
        let state = state.read().await;
        let configurations: Vec<std::collections::HashMap<String, String>> =
            if configuration_type == "APPLICATION" {
                state
                    .list_applications()
                    .into_iter()
                    .map(|a| {
                        let mut m = std::collections::HashMap::new();
                        m.insert(
                            "application.configurationId".to_string(),
                            a.configuration_id.clone(),
                        );
                        m.insert("application.name".to_string(), a.name.clone());
                        if let Some(d) = &a.description {
                            m.insert("application.description".to_string(), d.clone());
                        }
                        if let Some(w) = &a.wave {
                            m.insert("application.wave".to_string(), w.clone());
                        }
                        m
                    })
                    .collect()
            } else {
                vec![]
            };
        wire::serialize_list_configurations_response(&wire::ListConfigurationsResponse {
            configurations: Some(configurations),
            next_token: None,
        })
    }

    async fn handle_describe_configurations(
        &self,
        state: &SharedState,
        body: &[u8],
    ) -> MockResponse {
        let input = deser_or_400!(wire::deserialize_describe_configurations_request(body));
        let state = state.read().await;
        let configurations: Vec<std::collections::HashMap<String, String>> = input
            .configuration_ids
            .iter()
            .filter_map(|id| state.applications.get(id))
            .map(|a| {
                let mut m = std::collections::HashMap::new();
                m.insert(
                    "application.configurationId".to_string(),
                    a.configuration_id.clone(),
                );
                m.insert("application.name".to_string(), a.name.clone());
                m
            })
            .collect();
        wire::serialize_describe_configurations_response(&wire::DescribeConfigurationsResponse {
            configurations: Some(configurations),
        })
    }

    async fn handle_create_tags(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = deser_or_400!(wire::deserialize_create_tags_request(body));
        let tags: Vec<(String, String)> =
            input.tags.into_iter().map(|t| (t.key, t.value)).collect();
        if input.configuration_ids.is_empty() || tags.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "configurationIds and tags are required",
            );
        }
        let mut state = state.write().await;
        state.create_tags(&input.configuration_ids, tags);
        wire::serialize_create_tags_response(&wire::CreateTagsResponse {})
    }

    async fn handle_delete_tags(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = deser_or_400!(wire::deserialize_delete_tags_request(body));
        let keys: Vec<String> = input
            .tags
            .unwrap_or_default()
            .into_iter()
            .map(|t| t.key)
            .collect();
        let mut state = state.write().await;
        state.delete_tags(&input.configuration_ids, &keys);
        wire::serialize_delete_tags_response(&wire::DeleteTagsResponse {})
    }

    async fn handle_describe_tags(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = deser_or_400!(wire::deserialize_describe_tags_request(body));
        // Filter by configurationId in filters[0].values[0] if present.
        let cid = input
            .filters
            .as_ref()
            .and_then(|filters| filters.first())
            .and_then(|f| f.values.first())
            .map(|s| s.as_str());
        let state = state.read().await;
        let tags: Vec<wire::ConfigurationTag> = state
            .list_tags(cid)
            .into_iter()
            .map(|t| wire::ConfigurationTag {
                configuration_id: Some(t.configuration_id.clone()),
                configuration_type: Some(t.configuration_type.clone()),
                key: Some(t.key.clone()),
                time_of_creation: Some(t.time_of_creation as f64),
                value: Some(t.value.clone()),
            })
            .collect();
        wire::serialize_describe_tags_response(&wire::DescribeTagsResponse {
            next_token: None,
            tags: Some(tags),
        })
    }

    async fn handle_associate(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = deser_or_400!(
            wire::deserialize_associate_configuration_items_to_application_request(body)
        );
        if input.application_configuration_id.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "applicationConfigurationId is required",
            );
        }
        let mut state = state.write().await;
        state.add_associations(
            &input.application_configuration_id,
            &input.configuration_ids,
        );
        wire::serialize_associate_configuration_items_to_application_response(
            &wire::AssociateConfigurationItemsToApplicationResponse {},
        )
    }

    async fn handle_disassociate(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = deser_or_400!(
            wire::deserialize_disassociate_configuration_items_from_application_request(body)
        );
        if input.application_configuration_id.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "applicationConfigurationId is required",
            );
        }
        let mut state = state.write().await;
        state.remove_associations(
            &input.application_configuration_id,
            &input.configuration_ids,
        );
        wire::serialize_disassociate_configuration_items_from_application_response(
            &wire::DisassociateConfigurationItemsFromApplicationResponse {},
        )
    }

    async fn handle_batch_delete_agents(&self) -> MockResponse {
        wire::serialize_batch_delete_agents_response(&wire::BatchDeleteAgentsResponse {
            errors: Some(vec![]),
        })
    }

    async fn handle_batch_delete_import_data(
        &self,
        state: &SharedState,
        body: &[u8],
    ) -> MockResponse {
        let input = deser_or_400!(wire::deserialize_batch_delete_import_data_request(body));
        let delete_history = input.delete_history.unwrap_or(false);
        let mut state = state.write().await;
        let errors_ids = state.batch_delete_import_data(&input.import_task_ids, delete_history);
        let errors: Vec<wire::BatchDeleteImportDataError> = errors_ids
            .into_iter()
            .map(|id| wire::BatchDeleteImportDataError {
                error_code: Some("NOT_FOUND".to_string()),
                error_description: Some(format!("Import task {id} not found")),
                import_task_id: Some(id),
            })
            .collect();
        wire::serialize_batch_delete_import_data_response(&wire::BatchDeleteImportDataResponse {
            errors: Some(errors),
        })
    }

    async fn handle_describe_agents(&self) -> MockResponse {
        wire::serialize_describe_agents_response(&wire::DescribeAgentsResponse {
            agents_info: Some(vec![]),
            next_token: None,
        })
    }

    async fn handle_describe_continuous_exports(&self, state: &SharedState) -> MockResponse {
        let state = state.read().await;
        let descriptions: Vec<wire::ContinuousExportDescription> = state
            .list_continuous_exports()
            .into_iter()
            .map(|e| wire::ContinuousExportDescription {
                data_source: Some(e.data_source.clone()),
                export_id: Some(e.export_id.clone()),
                s3_bucket: e.s3_bucket.clone(),
                schema_storage_config: Some(e.schema_storage_config.clone()),
                start_time: e.start_time.map(|t| t as f64),
                status: Some(e.status.clone()),
                status_detail: e.status_detail.clone(),
                stop_time: e.stop_time.map(|t| t as f64),
            })
            .collect();
        wire::serialize_describe_continuous_exports_response(
            &wire::DescribeContinuousExportsResponse {
                descriptions: Some(descriptions),
                next_token: None,
            },
        )
    }

    async fn handle_describe_export_configurations(&self, state: &SharedState) -> MockResponse {
        let state = state.read().await;
        let exports_info: Vec<wire::ExportInfo> = state
            .list_export_tasks()
            .into_iter()
            .map(export_to_wire)
            .collect();
        wire::serialize_describe_export_configurations_response(
            &wire::DescribeExportConfigurationsResponse {
                exports_info: Some(exports_info),
                next_token: None,
            },
        )
    }

    async fn handle_describe_export_tasks(&self, state: &SharedState) -> MockResponse {
        let state = state.read().await;
        let exports_info: Vec<wire::ExportInfo> = state
            .list_export_tasks()
            .into_iter()
            .map(export_to_wire)
            .collect();
        wire::serialize_describe_export_tasks_response(&wire::DescribeExportTasksResponse {
            exports_info: Some(exports_info),
            next_token: None,
        })
    }

    async fn handle_describe_import_tasks(&self, state: &SharedState) -> MockResponse {
        let state = state.read().await;
        let tasks: Vec<wire::ImportTask> = state
            .list_import_tasks()
            .into_iter()
            .map(import_to_wire)
            .collect();
        wire::serialize_describe_import_tasks_response(&wire::DescribeImportTasksResponse {
            next_token: None,
            tasks: Some(tasks),
        })
    }

    async fn handle_describe_batch_delete_task(
        &self,
        state: &SharedState,
        body: &[u8],
    ) -> MockResponse {
        let input =
            deser_or_400!(wire::deserialize_describe_batch_delete_configuration_task_request(body));
        if input.task_id.is_empty() {
            return json_error_response(400, "ValidationException", "taskId is required");
        }
        let id = input.task_id;
        let state = state.read().await;
        match state.get_batch_delete_task(&id) {
            Ok(t) => wire::serialize_describe_batch_delete_configuration_task_response(
                &wire::DescribeBatchDeleteConfigurationTaskResponse {
                    task: Some(wire::BatchDeleteConfigurationTask {
                        configuration_type: Some(t.configuration_type.clone()),
                        deleted_configurations: Some(vec![]),
                        deletion_warnings: None,
                        end_time: t.end_time.map(|x| x as f64),
                        failed_configurations: None,
                        requested_configurations: None,
                        start_time: Some(t.start_time as f64),
                        status: Some(t.status.clone()),
                        task_id: Some(t.id.clone()),
                    }),
                },
            ),
            Err(e) => ad_error_response(&e),
        }
    }

    async fn handle_export_configurations(&self, state: &SharedState) -> MockResponse {
        let id = format!("export-{}", uuid::Uuid::new_v4().simple());
        let now = chrono::Utc::now().timestamp();
        let task = ExportTask {
            id: id.clone(),
            status: "SUCCEEDED".to_string(),
            status_message: None,
            configurations_download_url: None,
            export_request_time: now,
            is_truncated: false,
            requested_start_time: None,
            requested_end_time: None,
            data_source: "AGENT".to_string(),
            filters: vec![],
            preferences: None,
        };
        let mut state = state.write().await;
        state.add_export_task(task);
        wire::serialize_export_configurations_response(&wire::ExportConfigurationsResponse {
            export_id: Some(id),
        })
    }

    async fn handle_get_discovery_summary(&self, state: &SharedState) -> MockResponse {
        let state = state.read().await;
        wire::serialize_get_discovery_summary_response(&wire::GetDiscoverySummaryResponse {
            agent_summary: None,
            agentless_collector_summary: None,
            applications: Some(state.applications.len() as i64),
            connector_summary: None,
            me_collector_summary: None,
            servers: Some(0),
            servers_mapped_to_applications: Some(0),
            servers_mappedto_tags: Some(0),
        })
    }

    async fn handle_list_server_neighbors(&self, body: &[u8]) -> MockResponse {
        let _input = deser_or_400!(wire::deserialize_list_server_neighbors_request(body));
        wire::serialize_list_server_neighbors_response(&wire::ListServerNeighborsResponse {
            known_dependency_count: Some(0),
            neighbors: Some(vec![]),
            next_token: None,
        })
    }

    async fn handle_start_batch_delete_task(
        &self,
        state: &SharedState,
        body: &[u8],
    ) -> MockResponse {
        let input =
            deser_or_400!(wire::deserialize_start_batch_delete_configuration_task_request(body));
        let id = format!("task-{}", uuid::Uuid::new_v4().simple());
        let now = chrono::Utc::now().timestamp();
        let configuration_type = if input.configuration_type.is_empty() {
            "SERVER".to_string()
        } else {
            input.configuration_type
        };
        let task = BatchDeleteTask {
            id: id.clone(),
            configuration_type,
            status: "INITIALIZING".to_string(),
            start_time: now,
            end_time: None,
        };
        let mut state = state.write().await;
        state.add_batch_delete_task(task);
        wire::serialize_start_batch_delete_configuration_task_response(
            &wire::StartBatchDeleteConfigurationTaskResponse { task_id: Some(id) },
        )
    }

    async fn handle_start_continuous_export(&self, state: &SharedState) -> MockResponse {
        let id = format!("export-{}", uuid::Uuid::new_v4().simple());
        let now = chrono::Utc::now().timestamp();
        let export = ContinuousExport {
            export_id: id.clone(),
            status: "ACTIVE".to_string(),
            status_detail: None,
            s3_bucket: Some(format!(
                "aws-application-discovery-service-export-{}",
                uuid::Uuid::new_v4().simple()
            )),
            start_time: Some(now),
            stop_time: None,
            data_source: "AGENT".to_string(),
            schema_storage_config: Default::default(),
        };
        let mut state = state.write().await;
        let e = state.add_continuous_export(export);
        wire::serialize_start_continuous_export_response(&wire::StartContinuousExportResponse {
            data_source: Some(e.data_source.clone()),
            export_id: Some(e.export_id.clone()),
            s3_bucket: e.s3_bucket.clone(),
            schema_storage_config: Some(e.schema_storage_config.clone()),
            start_time: e.start_time.map(|t| t as f64),
        })
    }

    async fn handle_stop_continuous_export(
        &self,
        state: &SharedState,
        body: &[u8],
    ) -> MockResponse {
        let input = deser_or_400!(wire::deserialize_stop_continuous_export_request(body));
        if input.export_id.is_empty() {
            return json_error_response(400, "ValidationException", "exportId is required");
        }
        let id = input.export_id;
        let mut state = state.write().await;
        match state.stop_continuous_export(&id) {
            Ok(e) => wire::serialize_stop_continuous_export_response(
                &wire::StopContinuousExportResponse {
                    start_time: e.start_time.map(|t| t as f64),
                    stop_time: e.stop_time.map(|t| t as f64),
                },
            ),
            Err(e) => ad_error_response(&e),
        }
    }

    async fn handle_start_data_collection(&self, body: &[u8]) -> MockResponse {
        let input =
            deser_or_400!(wire::deserialize_start_data_collection_by_agent_ids_request(body));
        let configs: Vec<wire::AgentConfigurationStatus> = input
            .agent_ids
            .into_iter()
            .map(|a| wire::AgentConfigurationStatus {
                agent_id: Some(a),
                description: Some("Mock agent: data collection started".to_string()),
                operation_succeeded: Some(true),
            })
            .collect();
        wire::serialize_start_data_collection_by_agent_ids_response(
            &wire::StartDataCollectionByAgentIdsResponse {
                agents_configuration_status: Some(configs),
            },
        )
    }

    async fn handle_stop_data_collection(&self, body: &[u8]) -> MockResponse {
        let input = deser_or_400!(wire::deserialize_stop_data_collection_by_agent_ids_request(
            body
        ));
        let configs: Vec<wire::AgentConfigurationStatus> = input
            .agent_ids
            .into_iter()
            .map(|a| wire::AgentConfigurationStatus {
                agent_id: Some(a),
                description: Some("Mock agent: data collection stopped".to_string()),
                operation_succeeded: Some(true),
            })
            .collect();
        wire::serialize_stop_data_collection_by_agent_ids_response(
            &wire::StopDataCollectionByAgentIdsResponse {
                agents_configuration_status: Some(configs),
            },
        )
    }

    async fn handle_start_export_task(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = deser_or_400!(wire::deserialize_start_export_task_request(body));
        let id = format!("export-{}", uuid::Uuid::new_v4().simple());
        let now = chrono::Utc::now().timestamp();
        let data_source = input
            .export_data_format
            .as_ref()
            .and_then(|a| a.first())
            .map(|s| s.as_str())
            .unwrap_or("AGENT")
            .to_string();
        let filters: Vec<Value> = input
            .filters
            .unwrap_or_default()
            .iter()
            .filter_map(|f| serde_json::to_value(f).ok())
            .collect();
        let preferences = input
            .preferences
            .as_ref()
            .and_then(|p| serde_json::to_value(p).ok());
        let task = ExportTask {
            id: id.clone(),
            status: "SUCCEEDED".to_string(),
            status_message: None,
            configurations_download_url: None,
            export_request_time: now,
            is_truncated: false,
            requested_start_time: input.start_time.map(|t| t as i64),
            requested_end_time: input.end_time.map(|t| t as i64),
            data_source,
            filters,
            preferences,
        };
        let mut state = state.write().await;
        state.add_export_task(task);
        wire::serialize_start_export_task_response(&wire::StartExportTaskResponse {
            export_id: Some(id),
        })
    }

    async fn handle_start_import_task(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = deser_or_400!(wire::deserialize_start_import_task_request(body));
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "name is required");
        }
        if input.import_url.is_empty() {
            return json_error_response(400, "ValidationException", "importUrl is required");
        }
        let now = chrono::Utc::now().timestamp();
        let id = format!("import-task-{}", uuid::Uuid::new_v4().simple());
        let task = ImportTask {
            id: id.clone(),
            name: input.name,
            status: "IMPORT_COMPLETE".to_string(),
            import_url: input.import_url,
            import_request_time: now,
            import_completion_time: Some(now),
            server_import_success: 0,
            server_import_failure: 0,
            application_import_success: 0,
            application_import_failure: 0,
            error_url: None,
            import_deleted_time: None,
        };
        let mut state = state.write().await;
        let t = state.add_import_task(task);
        wire::serialize_start_import_task_response(&wire::StartImportTaskResponse {
            task: Some(import_to_wire(t)),
        })
    }
}

fn import_to_wire(t: &ImportTask) -> wire::ImportTask {
    wire::ImportTask {
        application_import_failure: Some(t.application_import_failure),
        application_import_success: Some(t.application_import_success),
        client_request_token: None,
        errors_and_failed_entries_zip: t.error_url.clone(),
        file_classification: None,
        import_completion_time: t.import_completion_time.map(|x| x as f64),
        import_deleted_time: t.import_deleted_time.map(|x| x as f64),
        import_request_time: Some(t.import_request_time as f64),
        import_task_id: Some(t.id.clone()),
        import_url: Some(t.import_url.clone()),
        name: Some(t.name.clone()),
        server_import_failure: Some(t.server_import_failure),
        server_import_success: Some(t.server_import_success),
        status: Some(t.status.clone()),
    }
}

fn export_to_wire(t: &ExportTask) -> wire::ExportInfo {
    wire::ExportInfo {
        configurations_download_url: t.configurations_download_url.clone(),
        export_id: Some(t.id.clone()),
        export_request_time: Some(t.export_request_time as f64),
        export_status: Some(t.status.clone()),
        is_truncated: Some(t.is_truncated),
        requested_end_time: t.requested_end_time.map(|x| x as f64),
        requested_start_time: t.requested_start_time.map(|x| x as f64),
        status_message: t.status_message.clone(),
    }
}

fn ad_error_response(err: &ApplicationDiscoveryError) -> MockResponse {
    let (status, error_type) = match err {
        ApplicationDiscoveryError::NotFound { .. } => (404, "ResourceNotFoundException"),
        ApplicationDiscoveryError::Validation { .. } => (400, "ValidationException"),
    };
    json_error_response(status, error_type, &err.to_string())
}
