use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, StatefulService,
    default_account_id,
};

use crate::state::{GlueError, GlueState};
use crate::views::GlueStateView;
use crate::wire;

/// Glue service handler that processes awsJson1.1 protocol requests.
pub struct GlueService {
    pub(crate) state: Arc<BackendState<GlueState>>,
    pub(crate) notifier: StateChangeNotifier<GlueStateView>,
}

impl GlueService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for GlueService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for GlueService {
    fn service_name(&self) -> &str {
        "glue"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://glue\.(.+)\.amazonaws\.com",
            r"https?://glue\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl GlueService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();

        let action = request
            .headers
            .get("x-amz-target")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.rsplit('.').next())
            .map(|s| s.to_string());

        let action = match action {
            Some(a) => a,
            None => {
                return json_error_response(400, "MissingAction", "Missing X-Amz-Target header");
            }
        };

        let body_bytes: &[u8] = &request.body;

        let state = self.state.get(account_id, &region);

        let response = match action.as_str() {
            // Database operations
            "CreateDatabase" => {
                self.handle_create_database(&state, body_bytes, account_id)
                    .await
            }
            "GetDatabase" => self.handle_get_database(&state, body_bytes).await,
            "DeleteDatabase" => self.handle_delete_database(&state, body_bytes).await,
            "GetDatabases" => self.handle_get_databases(&state).await,
            "UpdateDatabase" => self.handle_update_database(&state, body_bytes).await,

            // Table operations
            "CreateTable" => {
                self.handle_create_table(&state, body_bytes, account_id)
                    .await
            }
            "GetTable" => self.handle_get_table(&state, body_bytes).await,
            "GetTables" => self.handle_get_tables(&state, body_bytes).await,
            "DeleteTable" => self.handle_delete_table(&state, body_bytes).await,
            "BatchDeleteTable" => self.handle_batch_delete_table(&state, body_bytes).await,
            "UpdateTable" => self.handle_update_table(&state, body_bytes).await,
            "GetTableVersion" => self.handle_get_table_version(&state, body_bytes).await,
            "GetTableVersions" => self.handle_get_table_versions(&state, body_bytes).await,
            "DeleteTableVersion" => self.handle_delete_table_version(&state, body_bytes).await,

            // Partition operations
            "CreatePartition" => {
                self.handle_create_partition(&state, body_bytes, account_id)
                    .await
            }
            "BatchCreatePartition" => {
                self.handle_batch_create_partition(&state, body_bytes, account_id)
                    .await
            }
            "GetPartition" => self.handle_get_partition(&state, body_bytes).await,
            "BatchGetPartition" => self.handle_batch_get_partition(&state, body_bytes).await,
            "GetPartitions" => self.handle_get_partitions(&state, body_bytes).await,
            "DeletePartition" => self.handle_delete_partition(&state, body_bytes).await,
            "BatchDeletePartition" => self.handle_batch_delete_partition(&state, body_bytes).await,
            "UpdatePartition" => self.handle_update_partition(&state, body_bytes).await,
            "BatchUpdatePartition" => self.handle_batch_update_partition(&state, body_bytes).await,

            // Connection operations
            "CreateConnection" => self.handle_create_connection(&state, body_bytes).await,
            "GetConnection" => self.handle_get_connection(&state, body_bytes).await,
            "GetConnections" => self.handle_get_connections(&state).await,
            "DeleteConnection" => self.handle_delete_connection(&state, body_bytes).await,
            "UpdateConnection" => self.handle_update_connection(&state, body_bytes).await,
            "BatchDeleteConnection" => {
                self.handle_batch_delete_connection(&state, body_bytes)
                    .await
            }

            // Crawler operations
            "CreateCrawler" => self.handle_create_crawler(&state, body_bytes).await,
            "GetCrawler" => self.handle_get_crawler(&state, body_bytes).await,
            "GetCrawlers" => self.handle_get_crawlers(&state).await,
            "BatchGetCrawlers" => self.handle_batch_get_crawlers(&state, body_bytes).await,
            "ListCrawlers" => self.handle_list_crawlers(&state).await,
            "DeleteCrawler" => self.handle_delete_crawler(&state, body_bytes).await,
            "StartCrawler" => self.handle_start_crawler(&state, body_bytes).await,
            "StopCrawler" => self.handle_stop_crawler(&state, body_bytes).await,
            "ListCrawls" => self.handle_list_crawls(&state, body_bytes).await,
            "UpdateCrawler" => self.handle_update_crawler(&state, body_bytes).await,
            "UpdateCrawlerSchedule" => {
                self.handle_update_crawler_schedule(&state, body_bytes)
                    .await
            }
            "StartCrawlerSchedule" => self.handle_start_crawler_schedule(&state, body_bytes).await,
            "StopCrawlerSchedule" => self.handle_stop_crawler_schedule(&state, body_bytes).await,
            "GetCrawlerMetrics" => self.handle_get_crawler_metrics(&state).await,

            // Job operations
            "CreateJob" => self.handle_create_job(&state, body_bytes).await,
            "GetJob" => self.handle_get_job(&state, body_bytes).await,
            "GetJobs" => self.handle_get_jobs(&state).await,
            "BatchGetJobs" => self.handle_batch_get_jobs(&state, body_bytes).await,
            "ListJobs" => self.handle_list_jobs(&state).await,
            "DeleteJob" => self.handle_delete_job(&state, body_bytes).await,
            "StartJobRun" => self.handle_start_job_run(&state, body_bytes).await,
            "GetJobRun" => self.handle_get_job_run(&state, body_bytes).await,
            "GetJobRuns" => self.handle_get_job_runs(&state, body_bytes).await,
            "UpdateJob" => self.handle_update_job(&state, body_bytes).await,
            "BatchStopJobRun" => self.handle_batch_stop_job_run(&state, body_bytes).await,
            "GetJobBookmark" => self.handle_get_job_bookmark(&state, body_bytes).await,
            "ResetJobBookmark" => self.handle_reset_job_bookmark(&state, body_bytes).await,
            "UpdateJobFromSourceControl" => {
                self.handle_update_job_from_source_control(&state, body_bytes)
                    .await
            }
            "UpdateSourceControlFromJob" => {
                self.handle_update_source_control_from_job(&state, body_bytes)
                    .await
            }

            // Trigger operations
            "CreateTrigger" => self.handle_create_trigger(&state, body_bytes).await,
            "GetTrigger" => self.handle_get_trigger(&state, body_bytes).await,
            "GetTriggers" => self.handle_get_triggers(&state).await,
            "BatchGetTriggers" => self.handle_batch_get_triggers(&state, body_bytes).await,
            "ListTriggers" => self.handle_list_triggers(&state).await,
            "DeleteTrigger" => self.handle_delete_trigger(&state, body_bytes).await,
            "StartTrigger" => self.handle_start_trigger(&state, body_bytes).await,
            "StopTrigger" => self.handle_stop_trigger(&state, body_bytes).await,
            "UpdateTrigger" => self.handle_update_trigger(&state, body_bytes).await,

            // Workflow operations
            "CreateWorkflow" => self.handle_create_workflow(&state, body_bytes).await,
            "GetWorkflow" => self.handle_get_workflow(&state, body_bytes).await,
            "DeleteWorkflow" => self.handle_delete_workflow(&state, body_bytes).await,
            "ListWorkflows" => self.handle_list_workflows(&state).await,
            "UpdateWorkflow" => self.handle_update_workflow(&state, body_bytes).await,
            "StartWorkflowRun" => self.handle_start_workflow_run(&state, body_bytes).await,
            "StopWorkflowRun" => self.handle_stop_workflow_run(&state, body_bytes).await,
            "GetWorkflowRun" => self.handle_get_workflow_run(&state, body_bytes).await,
            "GetWorkflowRuns" => self.handle_get_workflow_runs(&state, body_bytes).await,
            "GetWorkflowRunProperties" => {
                self.handle_get_workflow_run_properties(&state, body_bytes)
                    .await
            }
            "PutWorkflowRunProperties" => {
                self.handle_put_workflow_run_properties(&state, body_bytes)
                    .await
            }
            "BatchGetWorkflows" => self.handle_batch_get_workflows(&state, body_bytes).await,
            "ResumeWorkflowRun" => self.handle_resume_workflow_run(&state, body_bytes).await,

            // DevEndpoint operations
            "CreateDevEndpoint" => self.handle_create_dev_endpoint(&state, body_bytes).await,
            "GetDevEndpoint" => self.handle_get_dev_endpoint(&state, body_bytes).await,
            "GetDevEndpoints" => self.handle_get_dev_endpoints(&state).await,
            "DeleteDevEndpoint" => self.handle_delete_dev_endpoint(&state, body_bytes).await,
            "ListDevEndpoints" => self.handle_list_dev_endpoints(&state).await,
            "BatchGetDevEndpoints" => {
                self.handle_batch_get_dev_endpoints(&state, body_bytes)
                    .await
            }
            "UpdateDevEndpoint" => self.handle_update_dev_endpoint(&state, body_bytes).await,

            // SecurityConfiguration operations
            "CreateSecurityConfiguration" => {
                self.handle_create_security_configuration(&state, body_bytes)
                    .await
            }
            "GetSecurityConfiguration" => {
                self.handle_get_security_configuration(&state, body_bytes)
                    .await
            }
            "GetSecurityConfigurations" => self.handle_get_security_configurations(&state).await,
            "DeleteSecurityConfiguration" => {
                self.handle_delete_security_configuration(&state, body_bytes)
                    .await
            }

            // Session operations
            "CreateSession" => self.handle_create_session(&state, body_bytes).await,
            "GetSession" => self.handle_get_session(&state, body_bytes).await,
            "ListSessions" => self.handle_list_sessions(&state).await,
            "DeleteSession" => self.handle_delete_session(&state, body_bytes).await,
            "StopSession" => self.handle_stop_session(&state, body_bytes).await,

            // Registry operations
            "CreateRegistry" => {
                self.handle_create_registry(&state, body_bytes, account_id, &region)
                    .await
            }
            "GetRegistry" => self.handle_get_registry(&state, body_bytes).await,
            "ListRegistries" => self.handle_list_registries(&state).await,
            "DeleteRegistry" => self.handle_delete_registry(&state, body_bytes).await,
            "UpdateRegistry" => self.handle_update_registry(&state, body_bytes).await,

            // Schema operations
            "CreateSchema" => {
                self.handle_create_schema(&state, body_bytes, account_id, &region)
                    .await
            }
            "GetSchema" => self.handle_get_schema(&state, body_bytes).await,
            "DeleteSchema" => self.handle_delete_schema(&state, body_bytes).await,
            "UpdateSchema" => self.handle_update_schema(&state, body_bytes).await,
            "RegisterSchemaVersion" => {
                self.handle_register_schema_version(&state, body_bytes)
                    .await
            }
            "GetSchemaVersion" => self.handle_get_schema_version(&state, body_bytes).await,
            "GetSchemaByDefinition" => {
                self.handle_get_schema_by_definition(&state, body_bytes)
                    .await
            }
            "PutSchemaVersionMetadata" => {
                self.handle_put_schema_version_metadata(&state, body_bytes)
                    .await
            }
            "ListSchemas" => self.handle_list_schemas(&state, body_bytes).await,
            "ListSchemaVersions" => self.handle_list_schema_versions(&state, body_bytes).await,
            "DeleteSchemaVersions" => self.handle_delete_schema_versions(&state, body_bytes).await,
            "CheckSchemaVersionValidity" => {
                self.handle_check_schema_version_validity(body_bytes).await
            }
            "QuerySchemaVersionMetadata" => {
                self.handle_query_schema_version_metadata(&state, body_bytes)
                    .await
            }
            "RemoveSchemaVersionMetadata" => {
                self.handle_remove_schema_version_metadata(&state, body_bytes)
                    .await
            }
            "GetSchemaVersionsDiff" => {
                self.handle_get_schema_versions_diff(&state, body_bytes)
                    .await
            }

            // ML Transform operations
            "CreateMLTransform" => self.handle_create_ml_transform(&state, body_bytes).await,
            "GetMLTransform" => self.handle_get_ml_transform(&state, body_bytes).await,
            "GetMLTransforms" => self.handle_get_ml_transforms(&state).await,
            "ListMLTransforms" => self.handle_list_ml_transforms(&state).await,
            "DeleteMLTransform" => self.handle_delete_ml_transform(&state, body_bytes).await,
            "UpdateMLTransform" => self.handle_update_ml_transform(&state, body_bytes).await,

            // Resource policy operations
            "PutResourcePolicy" => self.handle_put_resource_policy(&state, body_bytes).await,
            "GetResourcePolicy" => self.handle_get_resource_policy(&state).await,
            "DeleteResourcePolicy" => self.handle_delete_resource_policy(&state).await,

            // Search and catalog operations
            "SearchTables" => self.handle_search_tables(&state, body_bytes).await,
            "ImportCatalogToGlue" => self.handle_import_catalog_to_glue().await,

            // Data catalog encryption settings
            "PutDataCatalogEncryptionSettings" => {
                self.handle_put_data_catalog_encryption_settings(&state, body_bytes)
                    .await
            }
            "GetDataCatalogEncryptionSettings" => {
                self.handle_get_data_catalog_encryption_settings(&state)
                    .await
            }

            // Tag operations
            "TagResource" => self.handle_tag_resource(&state, body_bytes).await,
            "UntagResource" => self.handle_untag_resource(&state, body_bytes).await,
            "GetTags" => self.handle_get_tags(&state, body_bytes).await,

            _ => json_error_response(
                400,
                "InvalidAction",
                &format!("Could not find operation {action} for Glue"),
            ),
        };

        if response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    // ─── Database handlers ───

    async fn handle_create_database(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_database_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let db_input = &input.database_input;
        if db_input.name.is_empty() {
            return json_error_response(
                400,
                "InvalidInputException",
                "Missing 'Name' in DatabaseInput",
            );
        }
        let name = db_input.name.as_str();
        let description = db_input.description.as_deref().unwrap_or("");
        let location_uri = db_input.location_uri.as_deref().unwrap_or("");
        let parameters = db_input.parameters.clone().unwrap_or_default();
        let catalog_id = input.catalog_id.as_deref().unwrap_or(account_id);

        let mut state = state.write().await;
        match state.create_database(name, description, location_uri, parameters, catalog_id) {
            Ok(_) => {
                wire::serialize_create_database_response(&crate::model::CreateDatabaseResponse {})
            }
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_get_database(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_database_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "InvalidInputException", "Missing 'Name'");
        }
        let name = input.name.as_str();

        let state = state.read().await;
        match state.get_database(name) {
            Ok(db) => wire::serialize_get_database_response(&crate::model::GetDatabaseResponse {
                database: Some(database_to_wire(db)),
            }),
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_delete_database(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_database_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "InvalidInputException", "Missing 'Name'");
        }
        let name = input.name.as_str();

        let mut state = state.write().await;
        match state.delete_database(name) {
            Ok(()) => {
                wire::serialize_delete_database_response(&crate::model::DeleteDatabaseResponse {})
            }
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_get_databases(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let databases = state.get_databases();
        let entries: Vec<crate::model::Database> =
            databases.iter().map(|db| database_to_wire(db)).collect();

        wire::serialize_get_databases_response(&crate::model::GetDatabasesResponse {
            database_list: Some(entries),
            next_token: None,
        })
    }

    async fn handle_update_database(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_database_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "InvalidInputException", "Missing 'Name'");
        }
        let name = input.name.as_str();
        let db_input = &input.database_input;
        let description = db_input.description.as_deref();
        let location_uri = db_input.location_uri.as_deref();
        let parameters = db_input.parameters.clone();

        let mut state = state.write().await;
        match state.update_database(name, description, location_uri, parameters) {
            Ok(()) => {
                wire::serialize_update_database_response(&crate::model::UpdateDatabaseResponse {})
            }
            Err(e) => glue_error_response(&e),
        }
    }

    // ─── Table handlers ───

    async fn handle_create_table(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_table_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        if input.database_name.is_empty() {
            return json_error_response(400, "InvalidInputException", "Missing 'DatabaseName'");
        }
        let database_name = input.database_name.as_str();
        let table_input = match &input.table_input {
            Some(v) => v,
            None => {
                return json_error_response(400, "InvalidInputException", "Missing 'TableInput'");
            }
        };
        if table_input.name.is_empty() {
            return json_error_response(400, "InvalidInputException", "Missing 'Name'");
        }
        let name = table_input.name.as_str();
        let catalog_id = input.catalog_id.as_deref().unwrap_or(account_id);
        let description = table_input.description.as_deref().unwrap_or("");
        let owner = table_input.owner.as_deref().unwrap_or("");
        let table_type = table_input.table_type.as_deref().unwrap_or("");
        let parameters = table_input.parameters.clone().unwrap_or_default();
        let sd = table_input
            .storage_descriptor
            .as_ref()
            .and_then(|v| serde_json::to_value(v).ok());
        let pk = table_input
            .partition_keys
            .as_ref()
            .and_then(|v| serde_json::to_value(v).ok());
        let retention = table_input.retention.unwrap_or(0);

        let mut state = state.write().await;
        match state.create_table(
            database_name,
            name,
            catalog_id,
            description,
            owner,
            table_type,
            parameters,
            sd,
            pk,
            retention,
        ) {
            Ok(()) => wire::serialize_create_table_response(&crate::model::CreateTableResponse {}),
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_get_table(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_table_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        if input.database_name.is_empty() || input.name.is_empty() {
            return json_error_response(400, "InvalidInputException", "Missing required fields");
        }
        let database_name = input.database_name.as_str();
        let name = input.name.as_str();
        let state = state.read().await;
        match state.get_table(database_name, name) {
            Ok(tbl) => wire::serialize_get_table_response(&crate::model::GetTableResponse {
                table: Some(table_to_wire(tbl)),
            }),
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_get_tables(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_tables_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        if input.database_name.is_empty() {
            return json_error_response(400, "InvalidInputException", "Missing 'DatabaseName'");
        }
        let database_name = input.database_name.as_str();
        let state = state.read().await;
        let tables = state.get_tables(database_name);
        let entries: Vec<crate::model::Table> = tables.iter().map(|t| table_to_wire(t)).collect();
        wire::serialize_get_tables_response(&crate::model::GetTablesResponse {
            table_list: Some(entries),
            next_token: None,
        })
    }

    async fn handle_delete_table(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_table_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        if input.database_name.is_empty() || input.name.is_empty() {
            return json_error_response(400, "InvalidInputException", "Missing required fields");
        }
        let mut state = state.write().await;
        match state.delete_table(&input.database_name, &input.name) {
            Ok(()) => wire::serialize_delete_table_response(&crate::model::DeleteTableResponse {}),
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_batch_delete_table(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_batch_delete_table_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let database_name = if input.database_name.is_empty() {
            "default"
        } else {
            input.database_name.as_str()
        };
        let table_names: Vec<String> = input.tables_to_delete;

        let mut state = state.write().await;
        let results = state.batch_delete_table(database_name, &table_names);
        let errors: Vec<crate::model::TableError> = results
            .into_iter()
            .filter_map(|(name, err)| {
                err.map(|e| crate::model::TableError {
                    table_name: Some(name),
                    error_detail: Some(crate::model::ErrorDetail {
                        error_code: Some(glue_error_type(&e).to_string()),
                        error_message: Some(e.to_string()),
                    }),
                })
            })
            .collect();

        wire::serialize_batch_delete_table_response(&crate::model::BatchDeleteTableResponse {
            errors: if errors.is_empty() {
                None
            } else {
                Some(errors)
            },
        })
    }

    async fn handle_update_table(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_table_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let database_name = if input.database_name.is_empty() {
            "default"
        } else {
            input.database_name.as_str()
        };
        let table_input = match &input.table_input {
            Some(v) => v,
            None => {
                return json_error_response(400, "InvalidInputException", "Missing 'TableInput'");
            }
        };
        if table_input.name.is_empty() {
            return json_error_response(400, "InvalidInputException", "Missing 'Name'");
        }
        let name = table_input.name.as_str();
        let description = table_input.description.as_deref();
        let parameters = table_input.parameters.clone();
        let sd = table_input
            .storage_descriptor
            .as_ref()
            .and_then(|v| serde_json::to_value(v).ok());
        let retention = table_input.retention;

        let mut state = state.write().await;
        match state.update_table(database_name, name, description, parameters, sd, retention) {
            Ok(()) => wire::serialize_update_table_response(&crate::model::UpdateTableResponse {}),
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_get_table_version(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_table_version_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let database_name = if input.database_name.is_empty() {
            "default"
        } else {
            input.database_name.as_str()
        };
        let table_name = input.table_name.as_str();
        let version_id = input.version_id.as_deref().unwrap_or("0");

        let state = state.read().await;
        match state.get_table_version(database_name, table_name, version_id) {
            Ok(_snapshot) => {
                wire::serialize_get_table_version_response(&crate::model::GetTableVersionResponse {
                    table_version: Some(crate::model::TableVersion {
                        version_id: Some(version_id.to_string()),
                        table: None,
                        ..Default::default()
                    }),
                    ..Default::default()
                })
            }
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_get_table_versions(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_table_versions_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let database_name = if input.database_name.is_empty() {
            "default"
        } else {
            input.database_name.as_str()
        };
        let table_name = input.table_name.as_str();

        let state = state.read().await;
        match state.get_table_versions(database_name, table_name) {
            Ok(versions) => {
                let entries: Vec<crate::model::TableVersion> = versions
                    .iter()
                    .map(|(vid, _)| crate::model::TableVersion {
                        version_id: Some(vid.clone()),
                        table: None,
                        ..Default::default()
                    })
                    .collect();
                wire::serialize_get_table_versions_response(
                    &crate::model::GetTableVersionsResponse {
                        table_versions: Some(entries),
                        next_token: None,
                    },
                )
            }
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_delete_table_version(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_table_version_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let database_name = if input.database_name.is_empty() {
            "default"
        } else {
            input.database_name.as_str()
        };
        let table_name = input.table_name.as_str();
        let version_id = input.version_id.as_str();

        let mut state = state.write().await;
        match state.delete_table_version(database_name, table_name, version_id) {
            Ok(()) => wire::serialize_delete_table_version_response(
                &crate::model::DeleteTableVersionResponse {},
            ),
            Err(e) => glue_error_response(&e),
        }
    }

    // ─── Partition handlers ───

    async fn handle_create_partition(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_partition_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let database_name = if input.database_name.is_empty() {
            "default"
        } else {
            input.database_name.as_str()
        };
        let table_name = input.table_name.as_str();
        let catalog_id = input.catalog_id.as_deref().unwrap_or(account_id);
        let partition_input = &input.partition_input;
        let values = partition_input.values.clone().unwrap_or_default();
        let parameters = partition_input.parameters.clone().unwrap_or_default();
        let sd = partition_input
            .storage_descriptor
            .as_ref()
            .and_then(|v| serde_json::to_value(v).ok());

        let mut state = state.write().await;
        match state.create_partition(
            database_name,
            table_name,
            catalog_id,
            values,
            parameters,
            sd,
        ) {
            Ok(()) => {
                wire::serialize_create_partition_response(&crate::model::CreatePartitionResponse {})
            }
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_batch_create_partition(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_batch_create_partition_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let database_name = if input.database_name.is_empty() {
            "default"
        } else {
            input.database_name.as_str()
        };
        let table_name = input.table_name.as_str();
        let catalog_id = input.catalog_id.as_deref().unwrap_or(account_id);
        let partition_inputs: Vec<(Vec<String>, HashMap<String, String>, Option<Value>)> = input
            .partition_input_list
            .into_iter()
            .map(|pi| {
                let values = pi.values.clone().unwrap_or_default();
                let parameters = pi.parameters.clone().unwrap_or_default();
                let sd = pi
                    .storage_descriptor
                    .as_ref()
                    .and_then(|v| serde_json::to_value(v).ok());
                (values, parameters, sd)
            })
            .collect();

        let mut state = state.write().await;
        let errors =
            state.batch_create_partition(database_name, table_name, catalog_id, partition_inputs);
        let partition_errors: Vec<crate::model::PartitionError> = errors
            .into_iter()
            .filter_map(|e| {
                e.map(|err| crate::model::PartitionError {
                    error_detail: Some(crate::model::ErrorDetail {
                        error_code: Some(glue_error_type(&err).to_string()),
                        error_message: Some(err.to_string()),
                    }),
                    partition_values: None,
                })
            })
            .collect();

        wire::serialize_batch_create_partition_response(
            &crate::model::BatchCreatePartitionResponse {
                errors: if partition_errors.is_empty() {
                    None
                } else {
                    Some(partition_errors)
                },
            },
        )
    }

    async fn handle_get_partition(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_partition_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let database_name = if input.database_name.is_empty() {
            "default"
        } else {
            input.database_name.as_str()
        };
        let table_name = input.table_name.as_str();
        let values = input.partition_values.clone();

        let state = state.read().await;
        match state.get_partition(database_name, table_name, &values) {
            Ok(p) => wire::serialize_get_partition_response(&crate::model::GetPartitionResponse {
                partition: Some(partition_to_wire(p)),
            }),
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_batch_get_partition(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_batch_get_partition_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let database_name = if input.database_name.is_empty() {
            "default"
        } else {
            input.database_name.as_str()
        };
        let table_name = input.table_name.as_str();
        let partitions_to_get: Vec<Vec<String>> = input
            .partitions_to_get
            .into_iter()
            .map(|p| p.values)
            .collect();

        let state = state.read().await;
        let results = state.batch_get_partition(database_name, table_name, &partitions_to_get);
        let partitions: Vec<crate::model::Partition> = results
            .into_iter()
            .flatten()
            .map(partition_to_wire)
            .collect();

        wire::serialize_batch_get_partition_response(&crate::model::BatchGetPartitionResponse {
            partitions: Some(partitions),
            unprocessed_keys: None,
            ..Default::default()
        })
    }

    async fn handle_get_partitions(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_partitions_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let database_name = if input.database_name.is_empty() {
            "default"
        } else {
            input.database_name.as_str()
        };
        let table_name = input.table_name.as_str();

        let state = state.read().await;
        let parts = state.get_partitions(database_name, table_name);
        let entries: Vec<crate::model::Partition> =
            parts.iter().map(|p| partition_to_wire(p)).collect();

        wire::serialize_get_partitions_response(&crate::model::GetPartitionsResponse {
            partitions: Some(entries),
            next_token: None,
        })
    }

    async fn handle_delete_partition(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_partition_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let database_name = if input.database_name.is_empty() {
            "default"
        } else {
            input.database_name.as_str()
        };
        let table_name = input.table_name.as_str();
        let values = input.partition_values.clone();

        let mut state = state.write().await;
        match state.delete_partition(database_name, table_name, &values) {
            Ok(()) => {
                wire::serialize_delete_partition_response(&crate::model::DeletePartitionResponse {})
            }
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_batch_delete_partition(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_batch_delete_partition_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let database_name = if input.database_name.is_empty() {
            "default"
        } else {
            input.database_name.as_str()
        };
        let table_name = input.table_name.as_str();
        let partitions_to_delete: Vec<Vec<String>> = input
            .partitions_to_delete
            .into_iter()
            .map(|p| p.values)
            .collect();

        let mut state = state.write().await;
        let errors = state.batch_delete_partition(database_name, table_name, &partitions_to_delete);
        let partition_errors: Vec<crate::model::PartitionError> = errors
            .into_iter()
            .filter_map(|e| {
                e.map(|err| crate::model::PartitionError {
                    error_detail: Some(crate::model::ErrorDetail {
                        error_code: Some(glue_error_type(&err).to_string()),
                        error_message: Some(err.to_string()),
                    }),
                    partition_values: None,
                })
            })
            .collect();

        wire::serialize_batch_delete_partition_response(
            &crate::model::BatchDeletePartitionResponse {
                errors: if partition_errors.is_empty() {
                    None
                } else {
                    Some(partition_errors)
                },
            },
        )
    }

    async fn handle_update_partition(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_partition_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let database_name = if input.database_name.is_empty() {
            "default"
        } else {
            input.database_name.as_str()
        };
        let table_name = input.table_name.as_str();
        let values = input.partition_value_list.clone();
        let partition_input = &input.partition_input;
        let parameters = partition_input.parameters.clone();
        let sd = partition_input
            .storage_descriptor
            .as_ref()
            .and_then(|v| serde_json::to_value(v).ok());

        let mut state = state.write().await;
        match state.update_partition(database_name, table_name, &values, parameters, sd) {
            Ok(()) => {
                wire::serialize_update_partition_response(&crate::model::UpdatePartitionResponse {})
            }
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_batch_update_partition(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_batch_update_partition_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let database_name = if input.database_name.is_empty() {
            "default"
        } else {
            input.database_name.as_str()
        };
        let table_name = input.table_name.as_str();
        let entries: Vec<(Vec<String>, HashMap<String, String>, Option<Value>)> = input
            .entries
            .into_iter()
            .map(|e| {
                let values = e.partition_value_list.clone();
                let pi = &e.partition_input;
                let params = pi.parameters.clone().unwrap_or_default();
                let sd = pi
                    .storage_descriptor
                    .as_ref()
                    .and_then(|v| serde_json::to_value(v).ok());
                (values, params, sd)
            })
            .collect();

        let mut state = state.write().await;
        let errors = state.batch_update_partition(database_name, table_name, entries);
        let failure_entries: Vec<crate::model::BatchUpdatePartitionFailureEntry> = errors
            .into_iter()
            .filter_map(|e| {
                e.map(|err| crate::model::BatchUpdatePartitionFailureEntry {
                    error_detail: Some(crate::model::ErrorDetail {
                        error_code: Some(glue_error_type(&err).to_string()),
                        error_message: Some(err.to_string()),
                    }),
                    partition_value_list: None,
                })
            })
            .collect();

        wire::serialize_batch_update_partition_response(
            &crate::model::BatchUpdatePartitionResponse {
                errors: if failure_entries.is_empty() {
                    None
                } else {
                    Some(failure_entries)
                },
            },
        )
    }

    // ─── Connection handlers ───

    async fn handle_create_connection(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_create_connection_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let conn_input = &input.connection_input;
        let name = conn_input.name.as_str();
        let conn_type = if conn_input.connection_type.is_empty() {
            "JDBC"
        } else {
            conn_input.connection_type.as_str()
        };
        let conn_props = conn_input.connection_properties.clone();
        let description = conn_input.description.as_deref().unwrap_or("");
        let match_criteria = conn_input.match_criteria.clone().unwrap_or_default();
        let pcr = conn_input
            .physical_connection_requirements
            .as_ref()
            .and_then(|v| serde_json::to_value(v).ok());

        let mut state = state.write().await;
        match state.create_connection(
            name,
            conn_type,
            conn_props,
            description,
            match_criteria,
            pcr,
        ) {
            Ok(()) => wire::serialize_create_connection_response(
                &crate::model::CreateConnectionResponse {
                    ..Default::default()
                },
            ),
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_get_connection(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_connection_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let name = input.name.as_str();
        let state = state.read().await;
        match state.get_connection(name) {
            Ok(c) => {
                wire::serialize_get_connection_response(&crate::model::GetConnectionResponse {
                    connection: Some(connection_to_wire(c)),
                })
            }
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_get_connections(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let conns = state.get_connections();
        let entries: Vec<crate::model::Connection> =
            conns.iter().map(|c| connection_to_wire(c)).collect();
        wire::serialize_get_connections_response(&crate::model::GetConnectionsResponse {
            connection_list: Some(entries),
            next_token: None,
        })
    }

    async fn handle_delete_connection(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_connection_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let name = input.connection_name.as_str();
        let mut state = state.write().await;
        match state.delete_connection(name) {
            Ok(()) => wire::serialize_delete_connection_response(
                &crate::model::DeleteConnectionResponse {
                    ..Default::default()
                },
            ),
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_update_connection(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_connection_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let name = input.name.as_str();
        let conn_input = &input.connection_input;
        let conn_type = if conn_input.connection_type.is_empty() {
            None
        } else {
            Some(conn_input.connection_type.as_str())
        };
        // ConnectionProperties is non-Option in Smithy; treat empty map as "not provided".
        let conn_props = if conn_input.connection_properties.is_empty() {
            None
        } else {
            Some(conn_input.connection_properties.clone())
        };
        let description = conn_input.description.as_deref();
        let match_criteria = conn_input.match_criteria.clone();
        let pcr = conn_input
            .physical_connection_requirements
            .as_ref()
            .and_then(|v| serde_json::to_value(v).ok());

        let mut state = state.write().await;
        match state.update_connection(
            name,
            conn_type,
            conn_props,
            description,
            match_criteria,
            pcr,
        ) {
            Ok(()) => wire::serialize_update_connection_response(
                &crate::model::UpdateConnectionResponse {},
            ),
            Err(e) => glue_error_response(&e),
        }
    }

    // ─── Crawler handlers ───

    async fn handle_create_crawler(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_create_crawler_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let name = input.name.as_str();
        let role = input.role.as_str();
        let database_name = input.database_name.as_deref().unwrap_or("");
        let description = input.description.as_deref().unwrap_or("");
        let targets = serde_json::to_value(&input.targets).ok();
        let schedule = input.schedule.as_deref();
        let classifiers = input.classifiers.clone().unwrap_or_default();
        let table_prefix = input.table_prefix.as_deref().unwrap_or("");
        let configuration = input.configuration.as_deref().unwrap_or("");

        let tags = input.tags.clone().unwrap_or_default();

        let mut state = state.write().await;
        match state.create_crawler(
            name,
            role,
            database_name,
            description,
            targets,
            schedule,
            classifiers,
            table_prefix,
            configuration,
        ) {
            Ok(()) => {
                if !tags.is_empty() {
                    let arn = format!(
                        "arn:aws:glue:us-east-1:{}:crawler/{}",
                        default_account_id(),
                        name
                    );
                    state.tag_resource(&arn, tags);
                }
                wire::serialize_create_crawler_response(&crate::model::CreateCrawlerResponse {})
            }
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_get_crawler(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_crawler_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let name = input.name.as_str();
        let state = state.read().await;
        match state.get_crawler(name) {
            Ok(c) => wire::serialize_get_crawler_response(&crate::model::GetCrawlerResponse {
                crawler: Some(crawler_to_wire(c)),
            }),
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_get_crawlers(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let crawlers = state.get_crawlers();
        let entries: Vec<crate::model::Crawler> =
            crawlers.iter().map(|c| crawler_to_wire(c)).collect();
        wire::serialize_get_crawlers_response(&crate::model::GetCrawlersResponse {
            crawlers: Some(entries),
            next_token: None,
        })
    }

    async fn handle_batch_get_crawlers(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_batch_get_crawlers_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let names = input.crawler_names;
        let state = state.read().await;
        let (found, not_found_names) = state.batch_get_crawlers(&names);
        let entries: Vec<crate::model::Crawler> =
            found.iter().map(|c| crawler_to_wire(c)).collect();
        wire::serialize_batch_get_crawlers_response(&crate::model::BatchGetCrawlersResponse {
            crawlers: Some(entries),
            crawlers_not_found: if not_found_names.is_empty() {
                None
            } else {
                Some(not_found_names)
            },
        })
    }

    async fn handle_list_crawlers(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let names = state.list_crawlers();
        wire::serialize_list_crawlers_response(&crate::model::ListCrawlersResponse {
            crawler_names: Some(names),
            next_token: None,
        })
    }

    async fn handle_delete_crawler(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_crawler_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let name = input.name.as_str();
        let mut state = state.write().await;
        match state.delete_crawler(name) {
            Ok(()) => {
                wire::serialize_delete_crawler_response(&crate::model::DeleteCrawlerResponse {})
            }
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_start_crawler(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_start_crawler_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let name = input.name.as_str();
        let mut state = state.write().await;
        match state.start_crawler(name) {
            Ok(()) => {
                wire::serialize_start_crawler_response(&crate::model::StartCrawlerResponse {})
            }
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_stop_crawler(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_stop_crawler_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let name = input.name.as_str();
        let mut state = state.write().await;
        match state.stop_crawler(name) {
            Ok(()) => wire::serialize_stop_crawler_response(&crate::model::StopCrawlerResponse {}),
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_list_crawls(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_crawls_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let crawler_name = input.crawler_name.as_str();
        let state = state.read().await;
        match state.list_crawls(crawler_name) {
            Ok(records) => {
                let crawls: Vec<crate::model::CrawlerHistory> = records
                    .iter()
                    .map(|r| crate::model::CrawlerHistory {
                        crawl_id: Some(r.crawl_id.clone()),
                        state: Some(r.state.clone()),
                        start_time: Some(r.start_time.timestamp() as f64),
                        end_time: r.end_time.map(|t| t.timestamp() as f64),
                        ..Default::default()
                    })
                    .collect();
                wire::serialize_list_crawls_response(&crate::model::ListCrawlsResponse {
                    crawls: Some(crawls),
                    next_token: None,
                })
            }
            Err(e) => glue_error_response(&e),
        }
    }

    // ─── Job handlers ───

    async fn handle_create_job(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_create_job_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let name = input.name.as_str();
        let description = input.description.as_deref().unwrap_or("");
        let role = input.role.as_str();
        let command = serde_json::to_value(&input.command).ok();
        let default_arguments = input.default_arguments.clone().unwrap_or_default();
        let max_retries = input.max_retries.unwrap_or(0);
        let timeout = input.timeout.unwrap_or(2880);
        let max_capacity = input.max_capacity;
        let number_of_workers = input.number_of_workers;
        let worker_type = input.worker_type.clone();
        let glue_version = input.glue_version.clone();
        let tags = input.tags.clone().unwrap_or_default();

        let mut state = state.write().await;
        match state.create_job(
            name,
            description,
            role,
            command,
            default_arguments,
            max_retries,
            timeout,
            max_capacity,
            number_of_workers,
            worker_type,
            glue_version,
        ) {
            Ok(n) => {
                if !tags.is_empty() {
                    let arn = format!("arn:aws:glue:us-east-1:{}:job/{}", default_account_id(), n);
                    state.tag_resource(&arn, tags);
                }
                wire::serialize_create_job_response(&crate::model::CreateJobResponse {
                    name: Some(n),
                })
            }
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_get_job(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_job_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let name = input.job_name.as_str();
        let state = state.read().await;
        match state.get_job(name) {
            Ok(j) => wire::serialize_get_job_response(&crate::model::GetJobResponse {
                job: Some(job_to_wire(j)),
            }),
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_get_jobs(&self, state: &Arc<tokio::sync::RwLock<GlueState>>) -> MockResponse {
        let state = state.read().await;
        let jobs = state.get_jobs();
        let entries: Vec<crate::model::Job> = jobs.iter().map(|j| job_to_wire(j)).collect();
        wire::serialize_get_jobs_response(&crate::model::GetJobsResponse {
            jobs: Some(entries),
            next_token: None,
        })
    }

    async fn handle_batch_get_jobs(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_batch_get_jobs_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let names = input.job_names;
        let state = state.read().await;
        let (found, not_found_names) = state.batch_get_jobs(&names);
        let entries: Vec<crate::model::Job> = found.iter().map(|j| job_to_wire(j)).collect();
        wire::serialize_batch_get_jobs_response(&crate::model::BatchGetJobsResponse {
            jobs: Some(entries),
            jobs_not_found: if not_found_names.is_empty() {
                None
            } else {
                Some(not_found_names)
            },
        })
    }

    async fn handle_list_jobs(&self, state: &Arc<tokio::sync::RwLock<GlueState>>) -> MockResponse {
        let state = state.read().await;
        let names = state.list_jobs();
        wire::serialize_list_jobs_response(&crate::model::ListJobsResponse {
            job_names: Some(names),
            next_token: None,
        })
    }

    async fn handle_delete_job(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_job_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let name = input.job_name.as_str();
        let mut state = state.write().await;
        match state.delete_job(name) {
            Ok(n) => wire::serialize_delete_job_response(&crate::model::DeleteJobResponse {
                job_name: Some(n),
            }),
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_start_job_run(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_start_job_run_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let job_name = input.job_name.as_str();
        let arguments = input.arguments.clone().unwrap_or_default();
        let timeout = input.timeout;
        let max_capacity = input.max_capacity;
        let number_of_workers = input.number_of_workers;
        let worker_type = input.worker_type.clone();

        let mut state = state.write().await;
        match state.start_job_run(
            job_name,
            arguments,
            timeout,
            max_capacity,
            number_of_workers,
            worker_type,
        ) {
            Ok(run_id) => {
                wire::serialize_start_job_run_response(&crate::model::StartJobRunResponse {
                    job_run_id: Some(run_id),
                })
            }
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_get_job_run(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_job_run_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let job_name = input.job_name.as_str();
        let run_id = input.run_id.as_str();
        let state = state.read().await;
        match state.get_job_run(job_name, run_id) {
            Ok(jr) => wire::serialize_get_job_run_response(&crate::model::GetJobRunResponse {
                job_run: Some(job_run_to_wire(jr)),
            }),
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_get_job_runs(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_job_runs_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let job_name = input.job_name.as_str();
        let state = state.read().await;
        let runs = state.get_job_runs(job_name);
        let entries: Vec<crate::model::JobRun> =
            runs.iter().map(|jr| job_run_to_wire(jr)).collect();
        wire::serialize_get_job_runs_response(&crate::model::GetJobRunsResponse {
            job_runs: Some(entries),
            next_token: None,
        })
    }

    // ─── Trigger handlers ───

    async fn handle_create_trigger(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_create_trigger_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let name = input.name.as_str();
        let trigger_type = if input.r#type.is_empty() {
            "ON_DEMAND"
        } else {
            input.r#type.as_str()
        };
        let description = input.description.as_deref().unwrap_or("");
        let schedule = input.schedule.as_deref();
        let actions = if input.actions.is_empty() {
            None
        } else {
            serde_json::to_value(&input.actions).ok()
        };
        let predicate = input
            .predicate
            .as_ref()
            .and_then(|v| serde_json::to_value(v).ok());
        let workflow_name = input.workflow_name.as_deref();

        let mut state = state.write().await;
        match state.create_trigger(
            name,
            trigger_type,
            description,
            schedule,
            actions,
            predicate,
            workflow_name,
        ) {
            Ok(n) => {
                wire::serialize_create_trigger_response(&crate::model::CreateTriggerResponse {
                    name: Some(n),
                })
            }
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_get_trigger(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_trigger_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let name = input.name.as_str();
        let state = state.read().await;
        match state.get_trigger(name) {
            Ok(t) => wire::serialize_get_trigger_response(&crate::model::GetTriggerResponse {
                trigger: Some(trigger_to_wire(t)),
            }),
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_get_triggers(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let triggers = state.get_triggers();
        let entries: Vec<crate::model::Trigger> =
            triggers.iter().map(|t| trigger_to_wire(t)).collect();
        wire::serialize_get_triggers_response(&crate::model::GetTriggersResponse {
            triggers: Some(entries),
            next_token: None,
        })
    }

    async fn handle_batch_get_triggers(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_batch_get_triggers_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let names = input.trigger_names;
        let state = state.read().await;
        let (found, not_found_names) = state.batch_get_triggers(&names);
        let entries: Vec<crate::model::Trigger> =
            found.iter().map(|t| trigger_to_wire(t)).collect();
        wire::serialize_batch_get_triggers_response(&crate::model::BatchGetTriggersResponse {
            triggers: Some(entries),
            triggers_not_found: if not_found_names.is_empty() {
                None
            } else {
                Some(not_found_names)
            },
        })
    }

    async fn handle_list_triggers(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let names = state.list_triggers();
        wire::serialize_list_triggers_response(&crate::model::ListTriggersResponse {
            trigger_names: Some(names),
            next_token: None,
        })
    }

    async fn handle_delete_trigger(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_trigger_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let name = input.name.as_str();
        let mut state = state.write().await;
        match state.delete_trigger(name) {
            Ok(n) => {
                wire::serialize_delete_trigger_response(&crate::model::DeleteTriggerResponse {
                    name: Some(n),
                })
            }
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_start_trigger(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_start_trigger_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let name = input.name.as_str();
        let mut state = state.write().await;
        match state.start_trigger(name) {
            Ok(n) => wire::serialize_start_trigger_response(&crate::model::StartTriggerResponse {
                name: Some(n),
            }),
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_stop_trigger(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_stop_trigger_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let name = input.name.as_str();
        let mut state = state.write().await;
        match state.stop_trigger(name) {
            Ok(n) => wire::serialize_stop_trigger_response(&crate::model::StopTriggerResponse {
                name: Some(n),
            }),
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_update_trigger(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_trigger_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let name = input.name.as_str();
        let trigger_update = &input.trigger_update;
        let description = trigger_update.description.as_deref();
        // Smithy's Option<String> cannot distinguish "absent" from "null"; map
        // present values to Some(Some(s)) and absent to None.
        let schedule = trigger_update.schedule.as_deref().map(Some);
        let actions = trigger_update
            .actions
            .as_ref()
            .and_then(|v| serde_json::to_value(v).ok());
        let predicate = trigger_update
            .predicate
            .as_ref()
            .and_then(|v| serde_json::to_value(v).ok());

        let mut state = state.write().await;
        match state.update_trigger(name, description, schedule, actions, predicate) {
            Ok(t) => {
                wire::serialize_update_trigger_response(&crate::model::UpdateTriggerResponse {
                    trigger: Some(trigger_to_wire(t)),
                })
            }
            Err(e) => glue_error_response(&e),
        }
    }

    // ─── Workflow handlers ───

    async fn handle_create_workflow(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_create_workflow_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let name = input.name.as_str();
        let description = input.description.as_deref().unwrap_or("");
        let default_run_properties = input.default_run_properties.clone().unwrap_or_default();
        let max_concurrent_runs = input.max_concurrent_runs;

        let mut state = state.write().await;
        match state.create_workflow(
            name,
            description,
            default_run_properties,
            max_concurrent_runs,
        ) {
            Ok(n) => {
                wire::serialize_create_workflow_response(&crate::model::CreateWorkflowResponse {
                    name: Some(n),
                })
            }
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_get_workflow(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_workflow_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let name = input.name.as_str();
        let state = state.read().await;
        match state.get_workflow(name) {
            Ok(w) => wire::serialize_get_workflow_response(&crate::model::GetWorkflowResponse {
                workflow: Some(workflow_to_wire(w)),
            }),
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_delete_workflow(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_workflow_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let name = input.name.as_str();
        let mut state = state.write().await;
        match state.delete_workflow(name) {
            Ok(n) => {
                wire::serialize_delete_workflow_response(&crate::model::DeleteWorkflowResponse {
                    name: Some(n),
                })
            }
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_list_workflows(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let names = state.list_workflows();
        wire::serialize_list_workflows_response(&crate::model::ListWorkflowsResponse {
            workflows: Some(names),
            next_token: None,
        })
    }

    async fn handle_update_workflow(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_workflow_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let name = input.name.as_str();
        let description = input.description.as_deref();
        let default_run_properties = input.default_run_properties.clone();
        let max_concurrent_runs = input.max_concurrent_runs;

        let mut state = state.write().await;
        match state.update_workflow(
            name,
            description,
            default_run_properties,
            max_concurrent_runs,
        ) {
            Ok(n) => {
                wire::serialize_update_workflow_response(&crate::model::UpdateWorkflowResponse {
                    name: Some(n),
                })
            }
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_start_workflow_run(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_start_workflow_run_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let name = input.name.as_str();
        let mut state = state.write().await;
        match state.start_workflow_run(name) {
            Ok(run_id) => wire::serialize_start_workflow_run_response(
                &crate::model::StartWorkflowRunResponse {
                    run_id: Some(run_id),
                },
            ),
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_stop_workflow_run(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_stop_workflow_run_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let name = input.name.as_str();
        let run_id = input.run_id.as_str();
        let mut state = state.write().await;
        match state.stop_workflow_run(name, run_id) {
            Ok(()) => wire::serialize_stop_workflow_run_response(
                &crate::model::StopWorkflowRunResponse {},
            ),
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_get_workflow_run(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_workflow_run_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let name = input.name.as_str();
        let run_id = input.run_id.as_str();
        let state = state.read().await;
        match state.get_workflow_run(name, run_id) {
            Ok(r) => {
                wire::serialize_get_workflow_run_response(&crate::model::GetWorkflowRunResponse {
                    run: Some(workflow_run_to_wire(r)),
                })
            }
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_get_workflow_runs(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_workflow_runs_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let name = input.name.as_str();
        let state = state.read().await;
        match state.get_workflow_runs(name) {
            Ok(runs) => {
                let entries: Vec<crate::model::WorkflowRun> =
                    runs.iter().map(|r| workflow_run_to_wire(r)).collect();
                wire::serialize_get_workflow_runs_response(&crate::model::GetWorkflowRunsResponse {
                    runs: Some(entries),
                    next_token: None,
                })
            }
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_get_workflow_run_properties(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_workflow_run_properties_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let name = input.name.as_str();
        let run_id = input.run_id.as_str();
        let state = state.read().await;
        match state.get_workflow_run_properties(name, run_id) {
            Ok(props) => wire::serialize_get_workflow_run_properties_response(
                &crate::model::GetWorkflowRunPropertiesResponse {
                    run_properties: Some(props.clone()),
                },
            ),
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_put_workflow_run_properties(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_put_workflow_run_properties_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let name = input.name.as_str();
        let run_id = input.run_id.as_str();
        let run_properties = input.run_properties;
        let mut state = state.write().await;
        match state.put_workflow_run_properties(name, run_id, run_properties) {
            Ok(()) => wire::serialize_put_workflow_run_properties_response(
                &crate::model::PutWorkflowRunPropertiesResponse {},
            ),
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_batch_get_workflows(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_batch_get_workflows_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let names = input.names;
        let state = state.read().await;
        let (found, missing) = state.batch_get_workflows(&names);
        let entries: Vec<crate::model::Workflow> =
            found.iter().map(|w| workflow_to_wire(w)).collect();
        wire::serialize_batch_get_workflows_response(&crate::model::BatchGetWorkflowsResponse {
            workflows: Some(entries),
            missing_workflows: if missing.is_empty() {
                None
            } else {
                Some(missing)
            },
        })
    }

    async fn handle_resume_workflow_run(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_resume_workflow_run_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let name = input.name.as_str();
        let run_id = input.run_id.as_str();
        let mut state = state.write().await;
        match state.resume_workflow_run(name, run_id) {
            Ok((rid, node_ids)) => wire::serialize_resume_workflow_run_response(
                &crate::model::ResumeWorkflowRunResponse {
                    run_id: Some(rid),
                    node_ids: if node_ids.is_empty() {
                        None
                    } else {
                        Some(node_ids)
                    },
                },
            ),
            Err(e) => glue_error_response(&e),
        }
    }

    // ─── DevEndpoint handlers ───

    async fn handle_create_dev_endpoint(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_create_dev_endpoint_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let name = input.endpoint_name.as_str();
        let role_arn = input.role_arn.as_str();
        let sg_ids = input.security_group_ids.clone().unwrap_or_default();
        let subnet_id = input.subnet_id.as_deref().unwrap_or("");
        let num_nodes = input.number_of_nodes.unwrap_or(5);
        let num_workers = input.number_of_workers;
        let worker_type = input.worker_type.clone();
        let glue_version = input.glue_version.clone();
        let public_key = input.public_key.clone();
        let public_keys = input.public_keys.clone().unwrap_or_default();
        let extra_python = input.extra_python_libs_s3_path.clone();
        let extra_jars = input.extra_jars_s3_path.clone();
        let arguments = input.arguments.clone().unwrap_or_default();

        let mut state = state.write().await;
        match state.create_dev_endpoint(
            name,
            role_arn,
            sg_ids,
            subnet_id,
            num_nodes,
            num_workers,
            worker_type,
            glue_version,
            public_key,
            public_keys,
            extra_python,
            extra_jars,
            arguments,
        ) {
            Ok(de) => wire::serialize_create_dev_endpoint_response(
                &crate::model::CreateDevEndpointResponse {
                    endpoint_name: Some(de.endpoint_name.clone()),
                    role_arn: Some(de.role_arn.clone()),
                    status: Some(de.status.clone()),
                    created_timestamp: Some(de.created_timestamp.timestamp() as f64),
                    ..Default::default()
                },
            ),
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_get_dev_endpoint(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_dev_endpoint_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let name = input.endpoint_name.as_str();
        let state = state.read().await;
        match state.get_dev_endpoint(name) {
            Ok(de) => {
                wire::serialize_get_dev_endpoint_response(&crate::model::GetDevEndpointResponse {
                    dev_endpoint: Some(dev_endpoint_to_wire(de)),
                })
            }
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_get_dev_endpoints(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let endpoints = state.get_dev_endpoints();
        let entries: Vec<crate::model::DevEndpoint> = endpoints
            .iter()
            .map(|de| dev_endpoint_to_wire(de))
            .collect();
        wire::serialize_get_dev_endpoints_response(&crate::model::GetDevEndpointsResponse {
            dev_endpoints: Some(entries),
            next_token: None,
        })
    }

    async fn handle_delete_dev_endpoint(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_dev_endpoint_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let name = input.endpoint_name.as_str();
        let mut state = state.write().await;
        match state.delete_dev_endpoint(name) {
            Ok(()) => wire::serialize_delete_dev_endpoint_response(
                &crate::model::DeleteDevEndpointResponse {},
            ),
            Err(e) => glue_error_response(&e),
        }
    }

    // ─── SecurityConfiguration handlers ───

    async fn handle_create_security_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_create_security_configuration_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let name = input.name.as_str();
        let enc = serde_json::to_value(&input.encryption_configuration).ok();
        let mut state = state.write().await;
        match state.create_security_configuration(name, enc) {
            Ok(n) => wire::serialize_create_security_configuration_response(
                &crate::model::CreateSecurityConfigurationResponse {
                    name: Some(n),
                    created_timestamp: Some(chrono::Utc::now().timestamp() as f64),
                },
            ),
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_get_security_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_security_configuration_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let name = input.name.as_str();
        let state = state.read().await;
        match state.get_security_configuration(name) {
            Ok(sc) => wire::serialize_get_security_configuration_response(
                &crate::model::GetSecurityConfigurationResponse {
                    security_configuration: Some(security_config_to_wire(sc)),
                },
            ),
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_get_security_configurations(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let configs = state.get_security_configurations();
        let entries: Vec<crate::model::SecurityConfiguration> = configs
            .iter()
            .map(|sc| security_config_to_wire(sc))
            .collect();
        wire::serialize_get_security_configurations_response(
            &crate::model::GetSecurityConfigurationsResponse {
                security_configurations: Some(entries),
                next_token: None,
            },
        )
    }

    async fn handle_delete_security_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_security_configuration_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let name = input.name.as_str();
        let mut state = state.write().await;
        match state.delete_security_configuration(name) {
            Ok(()) => wire::serialize_delete_security_configuration_response(
                &crate::model::DeleteSecurityConfigurationResponse {},
            ),
            Err(e) => glue_error_response(&e),
        }
    }

    // ─── Session handlers ───

    async fn handle_create_session(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_create_session_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let id = input.id.as_str();
        let description = input.description.as_deref().unwrap_or("");
        let role = input.role.as_str();
        let command = serde_json::to_value(&input.command).ok();
        let glue_version = input.glue_version.clone();
        let max_capacity = input.max_capacity;
        let num_workers = input.number_of_workers;
        let worker_type = input.worker_type.clone();
        let idle_timeout = input.idle_timeout;
        let default_arguments = input.default_arguments.clone().unwrap_or_default();
        let security_configuration = input.security_configuration.clone();

        let mut state = state.write().await;
        match state.create_session(
            id,
            description,
            role,
            command,
            glue_version,
            max_capacity,
            num_workers,
            worker_type,
            idle_timeout,
            default_arguments,
            security_configuration,
        ) {
            Ok(s) => {
                wire::serialize_create_session_response(&crate::model::CreateSessionResponse {
                    session: Some(session_to_wire(s).await),
                })
            }
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_get_session(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_session_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let id = input.id.as_str();
        let state = state.read().await;
        match state.get_session(id) {
            Ok(s) => wire::serialize_get_session_response(&crate::model::GetSessionResponse {
                session: Some(session_to_wire(s).await),
            }),
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_list_sessions(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let sessions = state.list_sessions();
        let mut entries = Vec::new();
        for s in sessions.iter() {
            entries.push(session_to_wire(s).await);
        }
        wire::serialize_list_sessions_response(&crate::model::ListSessionsResponse {
            ids: None,
            next_token: None,
            sessions: Some(entries),
        })
    }

    async fn handle_delete_session(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_session_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let id = input.id.as_str();
        let mut state = state.write().await;
        match state.delete_session(id) {
            Ok(deleted_id) => {
                wire::serialize_delete_session_response(&crate::model::DeleteSessionResponse {
                    id: Some(deleted_id),
                })
            }
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_stop_session(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_stop_session_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let id = input.id.as_str();
        let mut state = state.write().await;
        match state.stop_session(id) {
            Ok(stopped_id) => {
                wire::serialize_stop_session_response(&crate::model::StopSessionResponse {
                    id: Some(stopped_id),
                })
            }
            Err(e) => glue_error_response(&e),
        }
    }

    // ─── Registry handlers ───

    async fn handle_create_registry(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_registry_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let name = input.registry_name.as_str();
        let description = input.description.as_deref().unwrap_or("");
        let tags = input.tags.clone().unwrap_or_default();

        let mut state = state.write().await;
        match state.create_registry(name, description, tags.clone(), account_id, region) {
            Ok(r) => {
                wire::serialize_create_registry_response(&crate::model::CreateRegistryResponse {
                    registry_name: Some(r.registry_name.clone()),
                    registry_arn: Some(r.registry_arn.clone()),
                    description: Some(r.description.clone()),
                    tags: if tags.is_empty() { None } else { Some(tags) },
                })
            }
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_get_registry(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_registry_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let name = input
            .registry_id
            .registry_name
            .as_deref()
            .or(input.registry_id.registry_arn.as_deref())
            .unwrap_or("");

        let state = state.read().await;
        match state.get_registry(name) {
            Ok(r) => wire::serialize_get_registry_response(&crate::model::GetRegistryResponse {
                registry_name: Some(r.registry_name.clone()),
                registry_arn: Some(r.registry_arn.clone()),
                description: Some(r.description.clone()),
                status: Some(r.status.clone()),
                created_time: Some(r.created_time.to_rfc3339()),
                updated_time: Some(r.updated_time.to_rfc3339()),
            }),
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_list_registries(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let registries = state.list_registries();
        let entries: Vec<crate::model::RegistryListItem> = registries
            .iter()
            .map(|r| crate::model::RegistryListItem {
                registry_name: Some(r.registry_name.clone()),
                registry_arn: Some(r.registry_arn.clone()),
                description: Some(r.description.clone()),
                status: Some(r.status.clone()),
                created_time: Some(r.created_time.to_rfc3339()),
                updated_time: Some(r.updated_time.to_rfc3339()),
            })
            .collect();
        wire::serialize_list_registries_response(&crate::model::ListRegistriesResponse {
            registries: Some(entries),
            next_token: None,
        })
    }

    async fn handle_delete_registry(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_registry_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let name = input
            .registry_id
            .registry_name
            .as_deref()
            .or(input.registry_id.registry_arn.as_deref())
            .unwrap_or("");

        let mut state = state.write().await;
        match state.delete_registry(name) {
            Ok(r) => {
                wire::serialize_delete_registry_response(&crate::model::DeleteRegistryResponse {
                    registry_name: Some(r.registry_name),
                    registry_arn: Some(r.registry_arn),
                    status: Some("DELETING".to_string()),
                })
            }
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_update_registry(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_registry_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let name = input
            .registry_id
            .registry_name
            .as_deref()
            .or(input.registry_id.registry_arn.as_deref())
            .unwrap_or("");
        let description = input.description.as_str();

        let mut state = state.write().await;
        match state.update_registry(name, description) {
            Ok(r) => {
                wire::serialize_update_registry_response(&crate::model::UpdateRegistryResponse {
                    registry_name: Some(r.registry_name.clone()),
                    registry_arn: Some(r.registry_arn.clone()),
                })
            }
            Err(e) => glue_error_response(&e),
        }
    }

    // ─── Schema handlers ───

    async fn handle_create_schema(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_schema_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let schema_name = input.schema_name.as_str();
        let registry_name = input
            .registry_id
            .as_ref()
            .and_then(|r| r.registry_name.as_deref().or(r.registry_arn.as_deref()))
            .unwrap_or("default-registry");
        let data_format = if input.data_format.is_empty() {
            "AVRO"
        } else {
            input.data_format.as_str()
        };
        let compatibility = input.compatibility.as_deref().unwrap_or("NONE");
        let description = input.description.as_deref().unwrap_or("");
        let schema_definition = input.schema_definition.as_deref();
        let tags = input.tags.clone().unwrap_or_default();

        let mut state = state.write().await;
        match state.create_schema(
            schema_name,
            registry_name,
            data_format,
            compatibility,
            description,
            schema_definition,
            tags,
            account_id,
            region,
        ) {
            Ok(s) => {
                let first_version_id = s.versions.first().map(|v| v.schema_version_id.clone());
                wire::serialize_create_schema_response(&crate::model::CreateSchemaResponse {
                    schema_name: Some(s.schema_name.clone()),
                    schema_arn: Some(s.schema_arn.clone()),
                    registry_name: Some(s.registry_name.clone()),
                    registry_arn: Some(s.registry_arn.clone()),
                    data_format: Some(s.data_format.clone()),
                    compatibility: Some(s.compatibility.clone()),
                    description: if s.description.is_empty() {
                        None
                    } else {
                        Some(s.description.clone())
                    },
                    schema_status: Some(s.schema_status.clone()),
                    latest_schema_version: Some(s.latest_schema_version),
                    next_schema_version: Some(s.next_schema_version),
                    schema_checkpoint: Some(s.schema_checkpoint),
                    schema_version_id: first_version_id,
                    schema_version_status: Some("AVAILABLE".to_string()),
                    tags: if s.tags.is_empty() {
                        None
                    } else {
                        Some(s.tags.clone())
                    },
                })
            }
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_get_schema(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_schema_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let name = input
            .schema_id
            .schema_name
            .as_deref()
            .or(input.schema_id.schema_arn.as_deref())
            .unwrap_or("");

        let state = state.read().await;
        match state.get_schema(name) {
            Ok(s) => wire::serialize_get_schema_response(&crate::model::GetSchemaResponse {
                schema_name: Some(s.schema_name.clone()),
                schema_arn: Some(s.schema_arn.clone()),
                registry_name: Some(s.registry_name.clone()),
                registry_arn: Some(s.registry_arn.clone()),
                data_format: Some(s.data_format.clone()),
                compatibility: Some(s.compatibility.clone()),
                description: if s.description.is_empty() {
                    None
                } else {
                    Some(s.description.clone())
                },
                schema_status: Some(s.schema_status.clone()),
                latest_schema_version: Some(s.latest_schema_version),
                next_schema_version: Some(s.next_schema_version),
                schema_checkpoint: Some(s.schema_checkpoint),
                created_time: Some(s.created_time.to_rfc3339()),
                updated_time: Some(s.updated_time.to_rfc3339()),
            }),
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_delete_schema(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_schema_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let name = input
            .schema_id
            .schema_name
            .as_deref()
            .or(input.schema_id.schema_arn.as_deref())
            .unwrap_or("");

        let mut state = state.write().await;
        match state.delete_schema(name) {
            Ok(s) => wire::serialize_delete_schema_response(&crate::model::DeleteSchemaResponse {
                schema_name: Some(s.schema_name),
                schema_arn: Some(s.schema_arn),
                status: Some("DELETING".to_string()),
            }),
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_update_schema(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_schema_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let name = input
            .schema_id
            .schema_name
            .as_deref()
            .or(input.schema_id.schema_arn.as_deref())
            .unwrap_or("");
        let compatibility = input.compatibility.as_deref();
        let description = input.description.as_deref();

        let mut state = state.write().await;
        match state.update_schema(name, compatibility, description) {
            Ok(s) => wire::serialize_update_schema_response(&crate::model::UpdateSchemaResponse {
                schema_name: Some(s.schema_name.clone()),
                schema_arn: Some(s.schema_arn.clone()),
                registry_name: Some(s.registry_name.clone()),
            }),
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_register_schema_version(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_register_schema_version_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let name = input
            .schema_id
            .schema_name
            .as_deref()
            .or(input.schema_id.schema_arn.as_deref())
            .unwrap_or("");
        let definition = input.schema_definition.as_str();

        let mut state = state.write().await;
        match state.register_schema_version(name, definition) {
            Ok(sv) => wire::serialize_register_schema_version_response(
                &crate::model::RegisterSchemaVersionResponse {
                    schema_version_id: Some(sv.schema_version_id.clone()),
                    version_number: Some(sv.version_number),
                    status: Some(sv.status.clone()),
                },
            ),
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_get_schema_version(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_schema_version_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let name = input
            .schema_id
            .as_ref()
            .and_then(|sid| sid.schema_name.as_deref().or(sid.schema_arn.as_deref()))
            .unwrap_or("");
        let version_number = input
            .schema_version_number
            .as_ref()
            .and_then(|svn| svn.version_number);
        let schema_version_id = input.schema_version_id.as_deref();

        let state = state.read().await;
        match state.get_schema_version(name, version_number, schema_version_id) {
            Ok(sv) => {
                // Find schema for this version to get arn
                let schema_arn = state
                    .schemas
                    .values()
                    .find(|s| {
                        s.versions
                            .iter()
                            .any(|v| v.schema_version_id == sv.schema_version_id)
                    })
                    .map(|s| s.schema_arn.clone());

                wire::serialize_get_schema_version_response(
                    &crate::model::GetSchemaVersionResponse {
                        schema_version_id: Some(sv.schema_version_id.clone()),
                        schema_definition: Some(sv.schema_definition.clone()),
                        version_number: Some(sv.version_number),
                        status: Some(sv.status.clone()),
                        schema_arn,
                        data_format: None,
                        created_time: Some(sv.created_time.to_rfc3339()),
                    },
                )
            }
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_get_schema_by_definition(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_schema_by_definition_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let name = input
            .schema_id
            .schema_name
            .as_deref()
            .or(input.schema_id.schema_arn.as_deref())
            .unwrap_or("");
        let definition = input.schema_definition.as_str();

        let state = state.read().await;
        match state.get_schema_by_definition(name, definition) {
            Ok(sv) => {
                let schema_arn = state
                    .schemas
                    .values()
                    .find(|s| {
                        s.versions
                            .iter()
                            .any(|v| v.schema_version_id == sv.schema_version_id)
                    })
                    .map(|s| s.schema_arn.clone());

                wire::serialize_get_schema_by_definition_response(
                    &crate::model::GetSchemaByDefinitionResponse {
                        schema_version_id: Some(sv.schema_version_id.clone()),
                        schema_arn,
                        status: Some(sv.status.clone()),
                        data_format: None,
                        created_time: Some(sv.created_time.to_rfc3339()),
                    },
                )
            }
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_put_schema_version_metadata(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_put_schema_version_metadata_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let schema_version_id = input.schema_version_id.as_deref().unwrap_or("");
        let key = input
            .metadata_key_value
            .metadata_key
            .as_deref()
            .unwrap_or("");
        let value = input
            .metadata_key_value
            .metadata_value
            .as_deref()
            .unwrap_or("");

        let mut state = state.write().await;
        match state.put_schema_version_metadata(schema_version_id, key, value) {
            Ok(()) => wire::serialize_put_schema_version_metadata_response(
                &crate::model::PutSchemaVersionMetadataResponse {
                    schema_version_id: Some(schema_version_id.to_string()),
                    metadata_key: Some(key.to_string()),
                    metadata_value: Some(value.to_string()),
                    ..Default::default()
                },
            ),
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_list_schemas(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_schemas_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let registry_name = input
            .registry_id
            .as_ref()
            .and_then(|rid| rid.registry_name.as_deref().or(rid.registry_arn.as_deref()));

        let state = state.read().await;
        let schemas = state.list_schemas(registry_name);
        let entries: Vec<crate::model::SchemaListItem> = schemas
            .iter()
            .map(|s| crate::model::SchemaListItem {
                schema_name: Some(s.schema_name.clone()),
                schema_arn: Some(s.schema_arn.clone()),
                registry_name: Some(s.registry_name.clone()),
                schema_status: Some(s.schema_status.clone()),
                description: if s.description.is_empty() {
                    None
                } else {
                    Some(s.description.clone())
                },
                created_time: Some(s.created_time.to_rfc3339()),
                updated_time: Some(s.updated_time.to_rfc3339()),
            })
            .collect();
        wire::serialize_list_schemas_response(&crate::model::ListSchemasResponse {
            schemas: Some(entries),
            next_token: None,
        })
    }

    async fn handle_list_schema_versions(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_schema_versions_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let schema_name = input
            .schema_id
            .schema_name
            .as_deref()
            .or(input.schema_id.schema_arn.as_deref())
            .unwrap_or("");

        let state = state.read().await;
        match state.get_schema(schema_name) {
            Ok(s) => {
                let entries: Vec<crate::model::SchemaVersionListItem> = s
                    .versions
                    .iter()
                    .map(|v| crate::model::SchemaVersionListItem {
                        schema_arn: Some(s.schema_arn.clone()),
                        schema_version_id: Some(v.schema_version_id.clone()),
                        version_number: Some(v.version_number),
                        status: Some(v.status.clone()),
                        created_time: Some(v.created_time.to_rfc3339()),
                    })
                    .collect();
                wire::serialize_list_schema_versions_response(
                    &crate::model::ListSchemaVersionsResponse {
                        schemas: Some(entries),
                        next_token: None,
                    },
                )
            }
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_delete_schema_versions(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_schema_versions_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let schema_name = input
            .schema_id
            .schema_name
            .as_deref()
            .or(input.schema_id.schema_arn.as_deref())
            .unwrap_or("");
        let versions_range = input.versions.as_str();

        let mut state = state.write().await;
        let results = state.delete_schema_versions(schema_name, versions_range);
        let errors: Vec<crate::model::SchemaVersionErrorItem> = results
            .into_iter()
            .filter_map(|(ver, err)| {
                err.map(|e| crate::model::SchemaVersionErrorItem {
                    version_number: Some(ver),
                    error_details: Some(crate::model::ErrorDetails {
                        error_code: Some(glue_error_type(&e).to_string()),
                        error_message: Some(e.to_string()),
                    }),
                })
            })
            .collect();
        wire::serialize_delete_schema_versions_response(
            &crate::model::DeleteSchemaVersionsResponse {
                schema_version_errors: if errors.is_empty() {
                    None
                } else {
                    Some(errors)
                },
            },
        )
    }

    async fn handle_check_schema_version_validity(&self, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_check_schema_version_validity_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let data_format = if input.data_format.is_empty() {
            "AVRO"
        } else {
            input.data_format.as_str()
        };
        let schema_definition = input.schema_definition.as_str();

        let validation_result = validate_schema(data_format, schema_definition);
        let (valid, error) = match validation_result {
            Ok(()) => (true, None),
            Err(msg) => (false, Some(msg)),
        };

        wire::serialize_check_schema_version_validity_response(
            &crate::model::CheckSchemaVersionValidityResponse {
                valid: Some(valid),
                error,
            },
        )
    }

    async fn handle_query_schema_version_metadata(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_query_schema_version_metadata_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let schema_version_id = input.schema_version_id.as_deref().unwrap_or("");

        let state = state.read().await;
        match state.query_schema_version_metadata(schema_version_id) {
            Ok(metadata) => {
                let info_map: std::collections::HashMap<String, crate::model::MetadataInfo> =
                    metadata
                        .iter()
                        .map(|(k, v)| {
                            (
                                k.clone(),
                                crate::model::MetadataInfo {
                                    metadata_value: Some(v.clone()),
                                    ..Default::default()
                                },
                            )
                        })
                        .collect();
                wire::serialize_query_schema_version_metadata_response(
                    &crate::model::QuerySchemaVersionMetadataResponse {
                        schema_version_id: Some(schema_version_id.to_string()),
                        metadata_info_map: if info_map.is_empty() {
                            None
                        } else {
                            Some(info_map)
                        },
                        next_token: None,
                    },
                )
            }
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_remove_schema_version_metadata(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_remove_schema_version_metadata_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let schema_version_id = input.schema_version_id.as_deref().unwrap_or("");
        let metadata_key = input
            .metadata_key_value
            .metadata_key
            .as_deref()
            .unwrap_or("");

        let mut state = state.write().await;
        match state.remove_schema_version_metadata(schema_version_id, metadata_key) {
            Ok(()) => wire::serialize_remove_schema_version_metadata_response(
                &crate::model::RemoveSchemaVersionMetadataResponse {
                    schema_version_id: Some(schema_version_id.to_string()),
                    metadata_key: Some(metadata_key.to_string()),
                    ..Default::default()
                },
            ),
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_get_schema_versions_diff(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_schema_versions_diff_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let schema_name = input
            .schema_id
            .schema_name
            .as_deref()
            .or(input.schema_id.schema_arn.as_deref())
            .unwrap_or("");
        let first_version = input
            .first_schema_version_number
            .version_number
            .unwrap_or(1);
        let second_version = input
            .second_schema_version_number
            .version_number
            .unwrap_or(2);
        let diff_type = if input.schema_diff_type.is_empty() {
            "SYNTAX_DIFF"
        } else {
            input.schema_diff_type.as_str()
        };

        let state = state.read().await;
        match state.get_schema_versions_diff(schema_name, first_version, second_version, diff_type)
        {
            Ok(diff) => wire::serialize_get_schema_versions_diff_response(
                &crate::model::GetSchemaVersionsDiffResponse { diff: Some(diff) },
            ),
            Err(e) => glue_error_response(&e),
        }
    }

    // ─── ML Transform handlers ───

    async fn handle_create_ml_transform(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_create_m_l_transform_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let name = input.name.as_str();
        let description = input.description.as_deref();
        let role = input.role.as_str();
        let glue_version = input.glue_version.as_deref();
        let max_capacity = input.max_capacity;
        let max_retries = input.max_retries;
        let timeout = input.timeout;
        let number_of_workers = input.number_of_workers;
        let worker_type = input.worker_type.as_deref();
        let parameters = serde_json::to_value(&input.parameters).ok();
        let input_record_tables: Vec<serde_json::Value> = input
            .input_record_tables
            .iter()
            .filter_map(|t| serde_json::to_value(t).ok())
            .collect();

        let mut state = state.write().await;
        match state.create_ml_transform(
            name,
            description,
            role,
            glue_version,
            max_capacity,
            max_retries,
            timeout,
            number_of_workers,
            worker_type,
            parameters,
            input_record_tables,
        ) {
            Ok(transform_id) => wire::serialize_create_m_l_transform_response(
                &crate::model::CreateMLTransformResponse {
                    transform_id: Some(transform_id),
                },
            ),
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_get_ml_transform(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_m_l_transform_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let transform_id = input.transform_id.as_str();
        let state = state.read().await;
        match state.get_ml_transform(transform_id) {
            Ok(t) => {
                wire::serialize_get_m_l_transform_response(&crate::model::GetMLTransformResponse {
                    transform_id: Some(t.transform_id.clone()),
                    name: Some(t.name.clone()),
                    description: if t.description.is_empty() {
                        None
                    } else {
                        Some(t.description.clone())
                    },
                    role: Some(t.role.clone()),
                    glue_version: t.glue_version.clone(),
                    max_capacity: t.max_capacity,
                    max_retries: t.max_retries,
                    timeout: t.timeout,
                    number_of_workers: t.number_of_workers,
                    worker_type: t.worker_type.clone(),
                    status: Some(t.status.clone()),
                    created_on: Some(t.created_on.timestamp() as f64),
                    last_modified_on: Some(t.last_modified_on.timestamp() as f64),
                    ..Default::default()
                })
            }
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_get_ml_transforms(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let transforms = state.get_ml_transforms();
        let entries: Vec<crate::model::MLTransform> = transforms
            .iter()
            .map(|t| crate::model::MLTransform {
                transform_id: Some(t.transform_id.clone()),
                name: Some(t.name.clone()),
                description: if t.description.is_empty() {
                    None
                } else {
                    Some(t.description.clone())
                },
                role: Some(t.role.clone()),
                glue_version: t.glue_version.clone(),
                max_capacity: t.max_capacity,
                max_retries: t.max_retries,
                timeout: t.timeout,
                number_of_workers: t.number_of_workers,
                worker_type: t.worker_type.clone(),
                status: Some(t.status.clone()),
                created_on: Some(t.created_on.timestamp() as f64),
                last_modified_on: Some(t.last_modified_on.timestamp() as f64),
                ..Default::default()
            })
            .collect();
        wire::serialize_get_m_l_transforms_response(&crate::model::GetMLTransformsResponse {
            transforms: Some(entries),
            next_token: None,
        })
    }

    async fn handle_list_ml_transforms(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let ids = state.list_ml_transforms();
        wire::serialize_list_m_l_transforms_response(&crate::model::ListMLTransformsResponse {
            transform_ids: Some(ids),
            next_token: None,
        })
    }

    async fn handle_delete_ml_transform(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_m_l_transform_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let transform_id = input.transform_id.as_str();
        let mut state = state.write().await;
        match state.delete_ml_transform(transform_id) {
            Ok(tid) => wire::serialize_delete_m_l_transform_response(
                &crate::model::DeleteMLTransformResponse {
                    transform_id: Some(tid),
                },
            ),
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_update_ml_transform(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_m_l_transform_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let transform_id = input.transform_id.as_str();
        let description = input.description.as_deref();
        let role = input.role.as_deref();
        let glue_version = input.glue_version.as_deref();
        let max_capacity = input.max_capacity;
        let max_retries = input.max_retries;
        let timeout = input.timeout;
        let number_of_workers = input.number_of_workers;
        let worker_type = input.worker_type.as_deref();
        let parameters = input
            .parameters
            .as_ref()
            .and_then(|v| serde_json::to_value(v).ok());

        let mut state = state.write().await;
        match state.update_ml_transform(
            transform_id,
            description,
            role,
            glue_version,
            max_capacity,
            max_retries,
            timeout,
            number_of_workers,
            worker_type,
            parameters,
        ) {
            Ok(tid) => wire::serialize_update_m_l_transform_response(
                &crate::model::UpdateMLTransformResponse {
                    transform_id: Some(tid),
                },
            ),
            Err(e) => glue_error_response(&e),
        }
    }

    // ─── Resource policy handlers ───

    async fn handle_put_resource_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_put_resource_policy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let policy = if input.policy_in_json.is_empty() {
            "{}"
        } else {
            input.policy_in_json.as_str()
        };
        let mut state = state.write().await;
        let hash = state.put_resource_policy(policy);
        wire::serialize_put_resource_policy_response(&crate::model::PutResourcePolicyResponse {
            policy_hash: Some(hash),
        })
    }

    async fn handle_get_resource_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_resource_policy() {
            Ok(rp) => wire::serialize_get_resource_policy_response(
                &crate::model::GetResourcePolicyResponse {
                    policy_in_json: Some(rp.policy_in_json.clone()),
                    policy_hash: Some(rp.policy_hash.clone()),
                    create_time: Some(rp.create_time.timestamp() as f64),
                    update_time: Some(rp.update_time.timestamp() as f64),
                },
            ),
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_delete_resource_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_resource_policy() {
            Ok(()) => wire::serialize_delete_resource_policy_response(
                &crate::model::DeleteResourcePolicyResponse {},
            ),
            Err(e) => glue_error_response(&e),
        }
    }

    // ─── Data catalog encryption settings handlers ───

    async fn handle_put_data_catalog_encryption_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_put_data_catalog_encryption_settings_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let settings = &input.data_catalog_encryption_settings;
        let ear = settings
            .encryption_at_rest
            .as_ref()
            .and_then(|v| serde_json::to_value(v).ok());
        let cpe = settings
            .connection_password_encryption
            .as_ref()
            .and_then(|v| serde_json::to_value(v).ok());

        let mut state = state.write().await;
        state.put_data_catalog_encryption_settings(ear, cpe);
        wire::serialize_put_data_catalog_encryption_settings_response(
            &crate::model::PutDataCatalogEncryptionSettingsResponse {},
        )
    }

    async fn handle_get_data_catalog_encryption_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let _settings = state.get_data_catalog_encryption_settings();
        wire::serialize_get_data_catalog_encryption_settings_response(
            &crate::model::GetDataCatalogEncryptionSettingsResponse {
                data_catalog_encryption_settings: Some(
                    crate::model::DataCatalogEncryptionSettings {
                        ..Default::default()
                    },
                ),
            },
        )
    }

    // ─── Tag handlers ───

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let arn = input.resource_arn.as_str();
        let tags = input.tags_to_add;
        let mut state = state.write().await;
        state.tag_resource(arn, tags);
        wire::serialize_tag_resource_response(&crate::model::TagResourceResponse {})
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let arn = input.resource_arn.as_str();
        let keys = input.tags_to_remove;
        let mut state = state.write().await;
        state.untag_resource(arn, &keys);
        wire::serialize_untag_resource_response(&crate::model::UntagResourceResponse {})
    }

    async fn handle_get_tags(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_tags_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let arn = input.resource_arn.as_str();
        let state = state.read().await;
        let tags = state.get_tags(arn);
        wire::serialize_get_tags_response(&crate::model::GetTagsResponse { tags: Some(tags) })
    }

    // ─── UpdateCrawler handler ───

    async fn handle_update_crawler(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_crawler_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let name = input.name.as_str();
        let role = input.role.as_deref();
        let database_name = input.database_name.as_deref();
        let description = input.description.as_deref();
        let targets = input
            .targets
            .as_ref()
            .and_then(|v| serde_json::to_value(v).ok());
        // Smithy's Option<String> cannot distinguish "absent" from "null"; map
        // present values to Some(Some(s)) and absent to None.
        let schedule = input.schedule.as_deref().map(Some);
        let classifiers = input.classifiers.clone();
        let table_prefix = input.table_prefix.as_deref();
        let configuration = input.configuration.as_deref();

        let mut state = state.write().await;
        match state.update_crawler(
            name,
            role,
            database_name,
            description,
            targets,
            schedule,
            classifiers,
            table_prefix,
            configuration,
        ) {
            Ok(()) => {
                wire::serialize_update_crawler_response(&crate::model::UpdateCrawlerResponse {})
            }
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_update_crawler_schedule(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_crawler_schedule_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let name = input.crawler_name.as_str();
        let schedule_expression = input.schedule.as_deref();
        let mut state = state.write().await;
        match state.update_crawler_schedule(name, schedule_expression) {
            Ok(()) => wire::serialize_update_crawler_schedule_response(
                &crate::model::UpdateCrawlerScheduleResponse {},
            ),
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_start_crawler_schedule(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_start_crawler_schedule_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let name = input.crawler_name.as_str();
        let mut state = state.write().await;
        match state.start_crawler_schedule(name) {
            Ok(()) => wire::serialize_start_crawler_schedule_response(
                &crate::model::StartCrawlerScheduleResponse {},
            ),
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_stop_crawler_schedule(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_stop_crawler_schedule_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let name = input.crawler_name.as_str();
        let mut state = state.write().await;
        match state.stop_crawler_schedule(name) {
            Ok(()) => wire::serialize_stop_crawler_schedule_response(
                &crate::model::StopCrawlerScheduleResponse {},
            ),
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_get_crawler_metrics(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let metrics = state.get_crawler_metrics();
        let entries: Vec<crate::model::CrawlerMetrics> = metrics
            .iter()
            .map(|m| crate::model::CrawlerMetrics {
                crawler_name: Some(m.crawler_name.clone()),
                time_left_seconds: Some(m.time_left_seconds),
                still_estimating: Some(m.still_estimating),
                last_runtime_seconds: Some(m.last_runtime_seconds),
                median_runtime_seconds: Some(m.median_runtime_seconds),
                tables_created: Some(m.tables_created),
                tables_updated: Some(m.tables_updated),
                tables_deleted: Some(m.tables_deleted),
            })
            .collect();
        wire::serialize_get_crawler_metrics_response(&crate::model::GetCrawlerMetricsResponse {
            crawler_metrics_list: if entries.is_empty() {
                None
            } else {
                Some(entries)
            },
            next_token: None,
        })
    }

    // ─── UpdateJob handler ───

    async fn handle_update_job(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_job_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let name = input.job_name.as_str();
        let job_update = &input.job_update;
        let description = job_update.description.as_deref();
        let role = job_update.role.as_deref();
        let command = job_update
            .command
            .as_ref()
            .and_then(|v| serde_json::to_value(v).ok());
        let default_arguments = job_update.default_arguments.clone();
        let max_retries = job_update.max_retries;
        let timeout = job_update.timeout;
        let max_capacity = job_update.max_capacity;
        let number_of_workers = job_update.number_of_workers;
        let worker_type = job_update.worker_type.clone();
        let glue_version = job_update.glue_version.clone();

        let mut state = state.write().await;
        match state.update_job(
            name,
            description,
            role,
            command,
            default_arguments,
            max_retries,
            timeout,
            max_capacity,
            number_of_workers,
            worker_type,
            glue_version,
        ) {
            Ok(n) => wire::serialize_update_job_response(&crate::model::UpdateJobResponse {
                job_name: Some(n),
            }),
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_batch_stop_job_run(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_batch_stop_job_run_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let job_name = input.job_name.as_str();
        let job_run_ids = input.job_run_ids;
        let mut state = state.write().await;
        let (succeeded, errors) = state.batch_stop_job_run(job_name, &job_run_ids);
        let successful: Vec<crate::model::BatchStopJobRunSuccessfulSubmission> = succeeded
            .into_iter()
            .map(
                |(jn, run_id)| crate::model::BatchStopJobRunSuccessfulSubmission {
                    job_name: Some(jn),
                    job_run_id: Some(run_id),
                },
            )
            .collect();
        let error_list: Vec<crate::model::BatchStopJobRunError> = errors
            .into_iter()
            .map(|(jn, run_id, err)| crate::model::BatchStopJobRunError {
                job_name: Some(jn),
                job_run_id: Some(run_id),
                error_detail: Some(crate::model::ErrorDetail {
                    error_code: Some(glue_error_type(&err).to_string()),
                    error_message: Some(err.to_string()),
                }),
            })
            .collect();
        wire::serialize_batch_stop_job_run_response(&crate::model::BatchStopJobRunResponse {
            successful_submissions: if successful.is_empty() {
                None
            } else {
                Some(successful)
            },
            errors: if error_list.is_empty() {
                None
            } else {
                Some(error_list)
            },
        })
    }

    async fn handle_get_job_bookmark(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_job_bookmark_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let job_name = input.job_name.as_str();
        let state = state.read().await;
        match state.get_job_bookmark(job_name) {
            Ok(()) => {
                wire::serialize_get_job_bookmark_response(&crate::model::GetJobBookmarkResponse {
                    job_bookmark_entry: None,
                })
            }
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_reset_job_bookmark(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_reset_job_bookmark_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let job_name = input.job_name.as_str();
        let mut state = state.write().await;
        match state.reset_job_bookmark(job_name) {
            Ok(()) => wire::serialize_reset_job_bookmark_response(
                &crate::model::ResetJobBookmarkResponse {
                    job_bookmark_entry: None,
                },
            ),
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_update_job_from_source_control(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_job_from_source_control_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let job_name = input.job_name.as_deref().unwrap_or("");
        let state = state.read().await;
        match state.get_job(job_name) {
            Ok(_) => wire::serialize_update_job_from_source_control_response(
                &crate::model::UpdateJobFromSourceControlResponse {
                    job_name: Some(job_name.to_string()),
                },
            ),
            Err(e) => glue_error_response(&e),
        }
    }

    async fn handle_update_source_control_from_job(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_source_control_from_job_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let job_name = input.job_name.as_deref().unwrap_or("");
        let state = state.read().await;
        match state.get_job(job_name) {
            Ok(_) => wire::serialize_update_source_control_from_job_response(
                &crate::model::UpdateSourceControlFromJobResponse {
                    job_name: Some(job_name.to_string()),
                },
            ),
            Err(e) => glue_error_response(&e),
        }
    }

    // ─── BatchDeleteConnection handler ───

    async fn handle_batch_delete_connection(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_batch_delete_connection_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let names = input.connection_name_list;
        let mut state = state.write().await;
        let (succeeded, errors) = state.batch_delete_connection(&names);
        let error_map: HashMap<String, crate::model::ErrorDetail> = errors
            .into_iter()
            .map(|(name, err)| {
                (
                    name,
                    crate::model::ErrorDetail {
                        error_code: Some(glue_error_type(&err).to_string()),
                        error_message: Some(err.to_string()),
                    },
                )
            })
            .collect();
        wire::serialize_batch_delete_connection_response(
            &crate::model::BatchDeleteConnectionResponse {
                succeeded: if succeeded.is_empty() {
                    None
                } else {
                    Some(succeeded)
                },
                errors: if error_map.is_empty() {
                    None
                } else {
                    Some(error_map)
                },
            },
        )
    }

    // ─── DevEndpoint extended handlers ───

    async fn handle_list_dev_endpoints(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let names = state.list_dev_endpoints();
        wire::serialize_list_dev_endpoints_response(&crate::model::ListDevEndpointsResponse {
            dev_endpoint_names: if names.is_empty() { None } else { Some(names) },
            next_token: None,
        })
    }

    async fn handle_batch_get_dev_endpoints(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_batch_get_dev_endpoints_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let names = input.dev_endpoint_names;
        let state = state.read().await;
        let (found, not_found_names) = state.batch_get_dev_endpoints(&names);
        let entries: Vec<crate::model::DevEndpoint> =
            found.iter().map(|de| dev_endpoint_to_wire(de)).collect();
        wire::serialize_batch_get_dev_endpoints_response(
            &crate::model::BatchGetDevEndpointsResponse {
                dev_endpoints: if entries.is_empty() {
                    None
                } else {
                    Some(entries)
                },
                dev_endpoints_not_found: if not_found_names.is_empty() {
                    None
                } else {
                    Some(not_found_names)
                },
            },
        )
    }

    async fn handle_update_dev_endpoint(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_dev_endpoint_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let name = input.endpoint_name.as_str();
        let public_key = input.public_key.clone();
        let add_public_keys = input.add_public_keys.clone().unwrap_or_default();
        let delete_public_keys = input.delete_public_keys.clone().unwrap_or_default();
        let add_arguments = input.add_arguments.clone().unwrap_or_default();
        let delete_arguments = input.delete_arguments.clone().unwrap_or_default();
        // UpdateDevEndpointRequest has no GlueVersion field in Smithy; preserve
        // historical behaviour by passing None.
        let glue_version: Option<String> = None;

        let mut state = state.write().await;
        match state.update_dev_endpoint(
            name,
            public_key,
            add_public_keys,
            delete_public_keys,
            add_arguments,
            delete_arguments,
            glue_version,
        ) {
            Ok(()) => wire::serialize_update_dev_endpoint_response(
                &crate::model::UpdateDevEndpointResponse {},
            ),
            Err(e) => glue_error_response(&e),
        }
    }

    // ─── SearchTables handler ───

    async fn handle_search_tables(
        &self,
        state: &Arc<tokio::sync::RwLock<GlueState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_search_tables_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidInputException", &e),
        };
        let search_text = input.search_text.as_deref();
        let state = state.read().await;
        let tables = state.search_tables(search_text);
        let entries: Vec<crate::model::Table> = tables.iter().map(|t| table_to_wire(t)).collect();
        wire::serialize_search_tables_response(&crate::model::SearchTablesResponse {
            table_list: if entries.is_empty() {
                None
            } else {
                Some(entries)
            },
            next_token: None,
        })
    }

    // ─── ImportCatalogToGlue handler ───

    async fn handle_import_catalog_to_glue(&self) -> MockResponse {
        wire::serialize_import_catalog_to_glue_response(
            &crate::model::ImportCatalogToGlueResponse {},
        )
    }
}

// ─── Helper functions ───

fn database_to_wire(db: &crate::types::Database) -> crate::model::Database {
    crate::model::Database {
        name: Some(db.name.clone()),
        catalog_id: Some(db.catalog_id.clone()),
        create_time: Some(db.create_time.timestamp() as f64),
        description: if db.description.is_empty() {
            None
        } else {
            Some(db.description.clone())
        },
        location_uri: if db.location_uri.is_empty() {
            None
        } else {
            Some(db.location_uri.clone())
        },
        parameters: if db.parameters.is_empty() {
            None
        } else {
            Some(db.parameters.clone())
        },
        ..Default::default()
    }
}

fn table_to_wire(tbl: &crate::types::Table) -> crate::model::Table {
    crate::model::Table {
        name: Some(tbl.name.clone()),
        database_name: Some(tbl.database_name.clone()),
        catalog_id: Some(tbl.catalog_id.clone()),
        create_time: Some(tbl.create_time.timestamp() as f64),
        update_time: Some(tbl.update_time.timestamp() as f64),
        description: if tbl.description.is_empty() {
            None
        } else {
            Some(tbl.description.clone())
        },
        owner: if tbl.owner.is_empty() {
            None
        } else {
            Some(tbl.owner.clone())
        },
        table_type: if tbl.table_type.is_empty() {
            None
        } else {
            Some(tbl.table_type.clone())
        },
        parameters: if tbl.parameters.is_empty() {
            None
        } else {
            Some(tbl.parameters.clone())
        },
        retention: Some(tbl.retention),
        version_id: Some(tbl.version_id.clone()),
        ..Default::default()
    }
}

fn partition_to_wire(p: &crate::types::Partition) -> crate::model::Partition {
    crate::model::Partition {
        values: Some(p.values.clone()),
        database_name: Some(p.database_name.clone()),
        table_name: Some(p.table_name.clone()),
        catalog_id: Some(p.catalog_id.clone()),
        creation_time: Some(p.creation_time.timestamp() as f64),
        parameters: if p.parameters.is_empty() {
            None
        } else {
            Some(p.parameters.clone())
        },
        ..Default::default()
    }
}

fn connection_to_wire(c: &crate::types::Connection) -> crate::model::Connection {
    crate::model::Connection {
        name: Some(c.name.clone()),
        connection_type: Some(c.connection_type.clone()),
        connection_properties: if c.connection_properties.is_empty() {
            None
        } else {
            Some(c.connection_properties.clone())
        },
        description: if c.description.is_empty() {
            None
        } else {
            Some(c.description.clone())
        },
        creation_time: Some(c.creation_time.timestamp() as f64),
        last_updated_time: Some(c.last_updated_time.timestamp() as f64),
        match_criteria: if c.match_criteria.is_empty() {
            None
        } else {
            Some(c.match_criteria.clone())
        },
        ..Default::default()
    }
}

fn crawler_to_wire(c: &crate::types::Crawler) -> crate::model::Crawler {
    crate::model::Crawler {
        name: Some(c.name.clone()),
        role: Some(c.role.clone()),
        database_name: Some(c.database_name.clone()),
        description: if c.description.is_empty() {
            None
        } else {
            Some(c.description.clone())
        },
        state: Some(c.state.clone()),
        creation_time: Some(c.creation_time.timestamp() as f64),
        last_updated: Some(c.last_updated.timestamp() as f64),
        table_prefix: if c.table_prefix.is_empty() {
            None
        } else {
            Some(c.table_prefix.clone())
        },
        version: Some(c.version),
        ..Default::default()
    }
}

fn job_to_wire(j: &crate::types::Job) -> crate::model::Job {
    crate::model::Job {
        name: Some(j.name.clone()),
        description: if j.description.is_empty() {
            None
        } else {
            Some(j.description.clone())
        },
        role: Some(j.role.clone()),
        default_arguments: if j.default_arguments.is_empty() {
            None
        } else {
            Some(j.default_arguments.clone())
        },
        max_retries: Some(j.max_retries),
        timeout: Some(j.timeout),
        max_capacity: j.max_capacity,
        number_of_workers: j.number_of_workers,
        worker_type: j.worker_type.clone(),
        glue_version: j.glue_version.clone(),
        created_on: Some(j.created_on.timestamp() as f64),
        last_modified_on: Some(j.last_modified_on.timestamp() as f64),
        ..Default::default()
    }
}

fn job_run_to_wire(jr: &crate::types::JobRun) -> crate::model::JobRun {
    crate::model::JobRun {
        id: Some(jr.id.clone()),
        job_name: Some(jr.job_name.clone()),
        started_on: Some(jr.started_on.timestamp() as f64),
        completed_on: jr.completed_on.map(|t| t.timestamp() as f64),
        job_run_state: Some(jr.job_run_state.clone()),
        arguments: if jr.arguments.is_empty() {
            None
        } else {
            Some(jr.arguments.clone())
        },
        timeout: Some(jr.timeout),
        max_capacity: jr.max_capacity,
        number_of_workers: jr.number_of_workers,
        worker_type: jr.worker_type.clone(),
        ..Default::default()
    }
}

fn trigger_to_wire(t: &crate::types::Trigger) -> crate::model::Trigger {
    crate::model::Trigger {
        name: Some(t.name.clone()),
        r#type: Some(t.trigger_type.clone()),
        state: Some(t.state.clone()),
        description: if t.description.is_empty() {
            None
        } else {
            Some(t.description.clone())
        },
        schedule: t.schedule.clone(),
        workflow_name: t.workflow_name.clone(),
        ..Default::default()
    }
}

fn workflow_to_wire(w: &crate::types::Workflow) -> crate::model::Workflow {
    crate::model::Workflow {
        name: Some(w.name.clone()),
        description: if w.description.is_empty() {
            None
        } else {
            Some(w.description.clone())
        },
        default_run_properties: if w.default_run_properties.is_empty() {
            None
        } else {
            Some(w.default_run_properties.clone())
        },
        created_on: Some(w.created_on.timestamp() as f64),
        last_modified_on: Some(w.last_modified_on.timestamp() as f64),
        max_concurrent_runs: w.max_concurrent_runs,
        ..Default::default()
    }
}

fn workflow_run_to_wire(r: &crate::types::WorkflowRun) -> crate::model::WorkflowRun {
    crate::model::WorkflowRun {
        workflow_run_id: Some(r.workflow_run_id.clone()),
        name: Some(r.name.clone()),
        started_on: Some(r.started_on.timestamp() as f64),
        completed_on: r.completed_on.map(|t| t.timestamp() as f64),
        status: Some(r.status.clone()),
        workflow_run_properties: if r.run_properties.is_empty() {
            None
        } else {
            Some(r.run_properties.clone())
        },
        ..Default::default()
    }
}

fn dev_endpoint_to_wire(de: &crate::types::DevEndpoint) -> crate::model::DevEndpoint {
    crate::model::DevEndpoint {
        endpoint_name: Some(de.endpoint_name.clone()),
        role_arn: Some(de.role_arn.clone()),
        status: Some(de.status.clone()),
        created_timestamp: Some(de.created_timestamp.timestamp() as f64),
        last_modified_timestamp: Some(de.last_modified_timestamp.timestamp() as f64),
        number_of_nodes: Some(de.number_of_nodes),
        number_of_workers: de.number_of_workers,
        worker_type: de.worker_type.clone(),
        glue_version: de.glue_version.clone(),
        security_group_ids: if de.security_group_ids.is_empty() {
            None
        } else {
            Some(de.security_group_ids.clone())
        },
        subnet_id: if de.subnet_id.is_empty() {
            None
        } else {
            Some(de.subnet_id.clone())
        },
        ..Default::default()
    }
}

fn security_config_to_wire(
    sc: &crate::types::SecurityConfiguration,
) -> crate::model::SecurityConfiguration {
    crate::model::SecurityConfiguration {
        name: Some(sc.name.clone()),
        created_time_stamp: Some(sc.created_time_stamp.timestamp() as f64),
        ..Default::default()
    }
}

async fn session_to_wire(s: &crate::types::Session) -> crate::model::Session {
    crate::model::Session {
        id: Some(s.id.clone()),
        description: if s.description.is_empty() {
            None
        } else {
            Some(s.description.clone())
        },
        role: Some(s.role.clone()),
        status: Some(s.status.clone()),
        created_on: Some(s.created_on.timestamp() as f64),
        glue_version: s.glue_version.clone(),
        max_capacity: s.max_capacity,
        number_of_workers: s.number_of_workers,
        worker_type: s.worker_type.clone(),
        idle_timeout: s.idle_timeout,
        default_arguments: if s.default_arguments.is_empty() {
            None
        } else {
            Some(s.default_arguments.clone())
        },
        security_configuration: s.security_configuration.clone(),
        ..Default::default()
    }
}

fn glue_error_type(err: &GlueError) -> &'static str {
    match err {
        GlueError::EntityNotFoundException(_) => "EntityNotFoundException",
        GlueError::AlreadyExistsException(_) => "AlreadyExistsException",
        GlueError::InvalidInputException(_) => "InvalidInputException",
    }
}

fn glue_error_response(err: &GlueError) -> MockResponse {
    let (status, error_type) = match err {
        GlueError::EntityNotFoundException(_) => (400u16, "EntityNotFoundException"),
        GlueError::AlreadyExistsException(_) => (400u16, "AlreadyExistsException"),
        GlueError::InvalidInputException(_) => (400u16, "InvalidInputException"),
    };
    let body = json!({
        "__type": error_type,
        "Message": err.to_string(),
    });
    MockResponse::json(status, body.to_string())
}

fn json_error_response(status: u16, code: &str, message: &str) -> MockResponse {
    let body = json!({
        "__type": code,
        "Message": message,
    });
    MockResponse::json(status, body.to_string())
}

/// Validate a schema definition string for the given Glue `DataFormat`.
///
/// Returns `Ok(())` when the schema is valid, or `Err(message)` with a
/// human-readable reason when it is not.
fn validate_schema(data_format: &str, schema_definition: &str) -> Result<(), String> {
    if schema_definition.is_empty() {
        return Err(format!("Empty schema definition for format {data_format}"));
    }

    match data_format.to_uppercase().as_str() {
        "AVRO" => validate_avro_schema(schema_definition),
        "JSON" => validate_json_schema(schema_definition),
        other => Err(format!("Unknown data format: {other}")),
    }
}

/// AVRO schemas are JSON documents.  A valid top-level AVRO schema is one of:
///   • A JSON string naming a primitive type ("null", "boolean", "int", …).
///   • A JSON object with a "type" field.
///   • A JSON array (union type whose elements are themselves valid schemas,
///     but we only check the outer structure here).
const AVRO_PRIMITIVE_TYPES: &[&str] = &[
    "null", "boolean", "int", "long", "float", "double", "bytes", "string",
];

fn validate_avro_schema(schema_definition: &str) -> Result<(), String> {
    let v: serde_json::Value = serde_json::from_str(schema_definition)
        .map_err(|e| format!("AVRO schema is not valid JSON: {e}"))?;

    match &v {
        serde_json::Value::String(type_name) => {
            if AVRO_PRIMITIVE_TYPES.contains(&type_name.as_str()) {
                Ok(())
            } else {
                Err(format!("Unknown AVRO primitive type: \"{type_name}\""))
            }
        }
        serde_json::Value::Object(obj) => {
            if obj.contains_key("type") {
                Ok(())
            } else {
                Err("AVRO schema object must have a \"type\" field".to_string())
            }
        }
        serde_json::Value::Array(_) => {
            // Union type — valid at the top level.
            Ok(())
        }
        other => Err(format!(
            "AVRO schema must be a JSON object, string, or array; got {}",
            other.type_name_for_display()
        )),
    }
}

/// JSON Schema: any valid JSON document (object or boolean at the top level).
fn validate_json_schema(schema_definition: &str) -> Result<(), String> {
    let v: serde_json::Value = serde_json::from_str(schema_definition)
        .map_err(|e| format!("JSON Schema is not valid JSON: {e}"))?;

    match &v {
        serde_json::Value::Object(_) | serde_json::Value::Bool(_) => Ok(()),
        other => Err(format!(
            "JSON Schema must be a JSON object or boolean; got {}",
            other.type_name_for_display()
        )),
    }
}

trait JsonValueExt {
    fn type_name_for_display(&self) -> &'static str;
}

impl JsonValueExt for serde_json::Value {
    fn type_name_for_display(&self) -> &'static str {
        match self {
            serde_json::Value::Null => "null",
            serde_json::Value::Bool(_) => "boolean",
            serde_json::Value::Number(_) => "number",
            serde_json::Value::String(_) => "string",
            serde_json::Value::Array(_) => "array",
            serde_json::Value::Object(_) => "object",
        }
    }
}
