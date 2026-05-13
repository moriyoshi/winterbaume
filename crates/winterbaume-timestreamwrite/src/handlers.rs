use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::{Value, json};
use uuid::Uuid;
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, StatefulService,
    default_account_id,
};

use crate::state::{TimestreamWriteError, TimestreamWriteState};
use crate::types::{self as statetypes, Dimension, Record};
use crate::views::TimestreamWriteStateView;
use crate::wire;

/// Timestream Write service handler that processes awsJson1.0 protocol requests.
pub struct TimestreamWriteService {
    pub(crate) state: Arc<BackendState<TimestreamWriteState>>,
    pub(crate) notifier: StateChangeNotifier<TimestreamWriteStateView>,
}

impl TimestreamWriteService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for TimestreamWriteService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for TimestreamWriteService {
    fn service_name(&self) -> &str {
        "timestream"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://ingest\.timestream\.(.+)\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl TimestreamWriteService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();

        // Extract action from X-Amz-Target header
        // Format: "Timestream_20181101.CreateDatabase"
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
            "CreateDatabase" => {
                self.handle_create_database(&state, body_bytes, account_id, &region)
                    .await
            }
            "DescribeDatabase" => self.handle_describe_database(&state, body_bytes).await,
            "DeleteDatabase" => self.handle_delete_database(&state, body_bytes).await,
            "ListDatabases" => self.handle_list_databases(&state).await,
            "UpdateDatabase" => self.handle_update_database(&state, body_bytes).await,
            "CreateTable" => {
                self.handle_create_table(&state, body_bytes, account_id, &region)
                    .await
            }
            "DescribeTable" => self.handle_describe_table(&state, body_bytes).await,
            "DeleteTable" => self.handle_delete_table(&state, body_bytes).await,
            "ListTables" => self.handle_list_tables(&state, body_bytes).await,
            "UpdateTable" => self.handle_update_table(&state, body_bytes).await,
            "WriteRecords" => self.handle_write_records(&state, body_bytes).await,
            "DescribeEndpoints" => self.handle_describe_endpoints(&region).await,
            "TagResource" => self.handle_tag_resource(&state, body_bytes).await,
            "UntagResource" => self.handle_untag_resource(&state, body_bytes).await,
            "ListTagsForResource" => self.handle_list_tags_for_resource(&state, body_bytes).await,
            "CreateBatchLoadTask" => self.handle_create_batch_load_task(&state, body_bytes).await,
            "DescribeBatchLoadTask" => {
                self.handle_describe_batch_load_task(&state, body_bytes)
                    .await
            }
            "ListBatchLoadTasks" => self.handle_list_batch_load_tasks(&state, body_bytes).await,
            "ResumeBatchLoadTask" => self.handle_resume_batch_load_task(&state, body_bytes).await,
            _ => json_error_response(
                400,
                "UnknownOperationException",
                &format!("Unknown operation: {action}"),
            ),
        };
        if response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    // --- Database handlers ---

    async fn handle_create_database(
        &self,
        state: &Arc<tokio::sync::RwLock<TimestreamWriteState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_database_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.database_name.is_empty() {
            return json_error_response(400, "ValidationException", "DatabaseName is required");
        }
        let kms_key_id = input.kms_key_id.as_deref();

        let mut state = state.write().await;
        match state.create_database(&input.database_name, account_id, region, kms_key_id) {
            Ok(db) => wire::serialize_create_database_response(&wire::CreateDatabaseResponse {
                database: Some(database_to_wire(db)),
            }),
            Err(e) => timestream_error_response(&e),
        }
    }

    async fn handle_describe_database(
        &self,
        state: &Arc<tokio::sync::RwLock<TimestreamWriteState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_database_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.database_name.is_empty() {
            return json_error_response(400, "ValidationException", "DatabaseName is required");
        }

        let state = state.read().await;
        match state.describe_database(&input.database_name) {
            Ok(db) => wire::serialize_describe_database_response(&wire::DescribeDatabaseResponse {
                database: Some(database_to_wire(db)),
            }),
            Err(e) => timestream_error_response(&e),
        }
    }

    async fn handle_delete_database(
        &self,
        state: &Arc<tokio::sync::RwLock<TimestreamWriteState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_database_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.database_name.is_empty() {
            return json_error_response(400, "ValidationException", "DatabaseName is required");
        }

        let mut state = state.write().await;
        match state.delete_database(&input.database_name) {
            Ok(()) => wire::serialize_delete_database_response(),
            Err(e) => timestream_error_response(&e),
        }
    }

    async fn handle_list_databases(
        &self,
        state: &Arc<tokio::sync::RwLock<TimestreamWriteState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let databases = state.list_databases();

        let db_list: Vec<wire::Database> =
            databases.iter().map(|db| database_to_wire(db)).collect();

        wire::serialize_list_databases_response(&wire::ListDatabasesResponse {
            databases: Some(db_list),
            next_token: None,
        })
    }

    async fn handle_update_database(
        &self,
        state: &Arc<tokio::sync::RwLock<TimestreamWriteState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_database_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.database_name.is_empty() {
            return json_error_response(400, "ValidationException", "DatabaseName is required");
        }
        if input.kms_key_id.is_empty() {
            return json_error_response(400, "ValidationException", "KmsKeyId is required");
        }

        let mut state = state.write().await;
        match state.update_database(&input.database_name, &input.kms_key_id) {
            Ok(db) => wire::serialize_update_database_response(&wire::UpdateDatabaseResponse {
                database: Some(database_to_wire(db)),
            }),
            Err(e) => timestream_error_response(&e),
        }
    }

    // --- Table handlers ---

    async fn handle_create_table(
        &self,
        state: &Arc<tokio::sync::RwLock<TimestreamWriteState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_table_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.database_name.is_empty() {
            return json_error_response(400, "ValidationException", "DatabaseName is required");
        }
        if input.table_name.is_empty() {
            return json_error_response(400, "ValidationException", "TableName is required");
        }

        let retention_properties =
            input
                .retention_properties
                .map(|rp| statetypes::RetentionProperties {
                    memory_store_retention_period_in_hours: rp
                        .memory_store_retention_period_in_hours,
                    magnetic_store_retention_period_in_days: rp
                        .magnetic_store_retention_period_in_days,
                });

        let magnetic_store_write_properties = input.magnetic_store_write_properties.map(|mswp| {
            statetypes::MagneticStoreWriteProperties {
                enable_magnetic_store_writes: mswp.enable_magnetic_store_writes,
            }
        });

        let mut state = state.write().await;
        match state.create_table(
            &input.database_name,
            &input.table_name,
            account_id,
            region,
            retention_properties,
            magnetic_store_write_properties,
        ) {
            Ok(table) => wire::serialize_create_table_response(&wire::CreateTableResponse {
                table: Some(table_to_wire(table)),
            }),
            Err(e) => timestream_error_response(&e),
        }
    }

    async fn handle_describe_table(
        &self,
        state: &Arc<tokio::sync::RwLock<TimestreamWriteState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_table_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.database_name.is_empty() {
            return json_error_response(400, "ValidationException", "DatabaseName is required");
        }
        if input.table_name.is_empty() {
            return json_error_response(400, "ValidationException", "TableName is required");
        }

        let state = state.read().await;
        match state.describe_table(&input.database_name, &input.table_name) {
            Ok(table) => wire::serialize_describe_table_response(&wire::DescribeTableResponse {
                table: Some(table_to_wire(table)),
            }),
            Err(e) => timestream_error_response(&e),
        }
    }

    async fn handle_delete_table(
        &self,
        state: &Arc<tokio::sync::RwLock<TimestreamWriteState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_table_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.database_name.is_empty() {
            return json_error_response(400, "ValidationException", "DatabaseName is required");
        }
        if input.table_name.is_empty() {
            return json_error_response(400, "ValidationException", "TableName is required");
        }

        let mut state = state.write().await;
        match state.delete_table(&input.database_name, &input.table_name) {
            Ok(()) => wire::serialize_delete_table_response(),
            Err(e) => timestream_error_response(&e),
        }
    }

    async fn handle_list_tables(
        &self,
        state: &Arc<tokio::sync::RwLock<TimestreamWriteState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_tables_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let database_name = match input.database_name.as_deref() {
            Some(n) if !n.is_empty() => n,
            _ => {
                return json_error_response(400, "ValidationException", "DatabaseName is required");
            }
        };

        let state = state.read().await;
        match state.list_tables(database_name) {
            Ok(tables) => {
                let table_list: Vec<wire::Table> =
                    tables.iter().map(|t| table_to_wire(t)).collect();
                wire::serialize_list_tables_response(&wire::ListTablesResponse {
                    tables: Some(table_list),
                    next_token: None,
                })
            }
            Err(e) => timestream_error_response(&e),
        }
    }

    async fn handle_update_table(
        &self,
        state: &Arc<tokio::sync::RwLock<TimestreamWriteState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_table_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.database_name.is_empty() {
            return json_error_response(400, "ValidationException", "DatabaseName is required");
        }
        if input.table_name.is_empty() {
            return json_error_response(400, "ValidationException", "TableName is required");
        }

        let retention_properties =
            input
                .retention_properties
                .map(|rp| statetypes::RetentionProperties {
                    memory_store_retention_period_in_hours: rp
                        .memory_store_retention_period_in_hours,
                    magnetic_store_retention_period_in_days: rp
                        .magnetic_store_retention_period_in_days,
                });

        let magnetic_store_write_properties = input.magnetic_store_write_properties.map(|mswp| {
            statetypes::MagneticStoreWriteProperties {
                enable_magnetic_store_writes: mswp.enable_magnetic_store_writes,
            }
        });

        let mut state = state.write().await;
        match state.update_table(
            &input.database_name,
            &input.table_name,
            retention_properties,
            magnetic_store_write_properties,
        ) {
            Ok(table) => wire::serialize_update_table_response(&wire::UpdateTableResponse {
                table: Some(table_to_wire(table)),
            }),
            Err(e) => timestream_error_response(&e),
        }
    }

    // --- WriteRecords ---

    async fn handle_write_records(
        &self,
        state: &Arc<tokio::sync::RwLock<TimestreamWriteState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_write_records_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.database_name.is_empty() {
            return json_error_response(400, "ValidationException", "DatabaseName is required");
        }
        if input.table_name.is_empty() {
            return json_error_response(400, "ValidationException", "TableName is required");
        }

        // Common attributes that apply to all records
        let common = input.common_attributes.unwrap_or_default();
        let common_dimensions: Vec<Dimension> = common
            .dimensions
            .unwrap_or_default()
            .into_iter()
            .map(wire_dimension_to_state)
            .collect();
        let common_measure_name = common.measure_name;
        let common_measure_value = common.measure_value;
        let common_measure_value_type = common.measure_value_type;
        let common_time = common.time;
        let common_time_unit = common.time_unit;

        let mut records = Vec::new();
        for r in input.records {
            let mut dims = common_dimensions.clone();
            if let Some(extra) = r.dimensions {
                dims.extend(extra.into_iter().map(wire_dimension_to_state));
            }

            let measure_name = r.measure_name.or_else(|| common_measure_name.clone());
            let measure_value = r.measure_value.or_else(|| common_measure_value.clone());
            let measure_value_type = r
                .measure_value_type
                .or_else(|| common_measure_value_type.clone());
            let time = r.time.or_else(|| common_time.clone());
            let time_unit = r.time_unit.or_else(|| common_time_unit.clone());

            records.push(Record {
                dimensions: dims,
                measure_name,
                measure_value,
                measure_value_type,
                time,
                time_unit,
            });
        }

        let mut state = state.write().await;
        match state.write_records(&input.database_name, &input.table_name, records) {
            Ok(count) => wire::serialize_write_records_response(&wire::WriteRecordsResponse {
                records_ingested: Some(wire::RecordsIngested {
                    total: Some(count),
                    memory_store: Some(count),
                    magnetic_store: Some(0),
                }),
            }),
            Err(e) => timestream_error_response(&e),
        }
    }

    // --- DescribeEndpoints ---

    async fn handle_describe_endpoints(&self, region: &str) -> MockResponse {
        wire::serialize_describe_endpoints_response(&wire::DescribeEndpointsResponse {
            endpoints: Some(vec![wire::Endpoint {
                address: Some(format!("ingest.timestream.{region}.amazonaws.com")),
                cache_period_in_minutes: Some(1440),
            }]),
        })
    }

    // --- Tag operations ---

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<TimestreamWriteState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_a_r_n.is_empty() {
            return json_error_response(400, "ValidationException", "ResourceARN is required");
        }
        let tag_map: HashMap<String, String> =
            input.tags.into_iter().map(|t| (t.key, t.value)).collect();

        let mut state = state.write().await;
        match state.tag_resource(&input.resource_a_r_n, tag_map) {
            Ok(()) => wire::serialize_tag_resource_response(&wire::TagResourceResponse {}),
            Err(e) => timestream_error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<TimestreamWriteState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_a_r_n.is_empty() {
            return json_error_response(400, "ValidationException", "ResourceARN is required");
        }

        let mut state = state.write().await;
        match state.untag_resource(&input.resource_a_r_n, &input.tag_keys) {
            Ok(()) => wire::serialize_untag_resource_response(&wire::UntagResourceResponse {}),
            Err(e) => timestream_error_response(&e),
        }
    }

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<TimestreamWriteState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_a_r_n.is_empty() {
            return json_error_response(400, "ValidationException", "ResourceARN is required");
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
                        tags: Some(tag_list),
                    },
                )
            }
            Err(e) => timestream_error_response(&e),
        }
    }

    // --- BatchLoadTask handlers ---

    async fn handle_create_batch_load_task(
        &self,
        state: &Arc<tokio::sync::RwLock<TimestreamWriteState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_create_batch_load_task_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.target_database_name.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "TargetDatabaseName is required",
            );
        }
        if input.target_table_name.is_empty() {
            return json_error_response(400, "ValidationException", "TargetTableName is required");
        }
        // Bridge typed wire shapes back into Value for state storage.
        let data_source_configuration =
            serde_json::to_value(&input.data_source_configuration).unwrap_or(Value::Null);
        let report_configuration =
            serde_json::to_value(&input.report_configuration).unwrap_or(Value::Null);
        let data_model_configuration = input
            .data_model_configuration
            .as_ref()
            .map(|v| serde_json::to_value(v).unwrap_or(Value::Null));

        let task_id = Uuid::new_v4().to_string();

        let mut state = state.write().await;
        let task = state.create_batch_load_task(
            task_id,
            &input.target_database_name,
            &input.target_table_name,
            data_source_configuration,
            report_configuration,
            data_model_configuration,
        );
        wire::serialize_create_batch_load_task_response(&wire::CreateBatchLoadTaskResponse {
            task_id: Some(task.task_id.clone()),
        })
    }

    async fn handle_describe_batch_load_task(
        &self,
        state: &Arc<tokio::sync::RwLock<TimestreamWriteState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_batch_load_task_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.task_id.is_empty() {
            return json_error_response(400, "ValidationException", "TaskId is required");
        }

        let state = state.read().await;
        match state.describe_batch_load_task(&input.task_id) {
            Ok(task) => {
                let desc = batch_load_task_to_description(task);
                wire::serialize_describe_batch_load_task_response(
                    &wire::DescribeBatchLoadTaskResponse {
                        batch_load_task_description: Some(desc),
                    },
                )
            }
            Err(e) => timestream_error_response(&e),
        }
    }

    async fn handle_list_batch_load_tasks(
        &self,
        state: &Arc<tokio::sync::RwLock<TimestreamWriteState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_batch_load_tasks_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let task_status_filter = input.task_status;

        let state = state.read().await;
        let tasks = state.list_batch_load_tasks(task_status_filter.as_deref());

        let task_list: Vec<wire::BatchLoadTask> = tasks
            .iter()
            .map(|t| wire::BatchLoadTask {
                task_id: Some(t.task_id.clone()),
                task_status: Some(t.task_status.clone()),
                database_name: Some(t.database_name.clone()),
                table_name: Some(t.table_name.clone()),
                creation_time: Some(t.creation_time.timestamp() as f64),
                last_updated_time: Some(t.last_updated_time.timestamp() as f64),
                resumable_until: None,
            })
            .collect();

        wire::serialize_list_batch_load_tasks_response(&wire::ListBatchLoadTasksResponse {
            batch_load_tasks: Some(task_list),
            next_token: None,
        })
    }

    async fn handle_resume_batch_load_task(
        &self,
        state: &Arc<tokio::sync::RwLock<TimestreamWriteState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_resume_batch_load_task_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.task_id.is_empty() {
            return json_error_response(400, "ValidationException", "TaskId is required");
        }

        let mut state = state.write().await;
        match state.resume_batch_load_task(&input.task_id) {
            Ok(()) => wire::serialize_resume_batch_load_task_response(
                &wire::ResumeBatchLoadTaskResponse {},
            ),
            Err(e) => timestream_error_response(&e),
        }
    }
}

// --- Utility functions ---

fn wire_dimension_to_state(d: wire::Dimension) -> Dimension {
    Dimension {
        name: d.name,
        value: d.value,
        dimension_value_type: d.dimension_value_type,
    }
}

/// Convert a state `Database` to the wire `Database` model type.
fn database_to_wire(db: &statetypes::Database) -> wire::Database {
    wire::Database {
        database_name: Some(db.database_name.clone()),
        arn: Some(db.arn.clone()),
        table_count: Some(db.table_count),
        creation_time: Some(db.creation_time.timestamp() as f64),
        last_updated_time: Some(db.last_updated_time.timestamp() as f64),
        kms_key_id: db.kms_key_id.clone(),
    }
}

/// Convert a state `Table` to the wire `Table` model type.
fn table_to_wire(table: &statetypes::Table) -> wire::Table {
    wire::Table {
        table_name: Some(table.table_name.clone()),
        database_name: Some(table.database_name.clone()),
        arn: Some(table.arn.clone()),
        table_status: Some(table.table_status.clone()),
        creation_time: Some(table.creation_time.timestamp() as f64),
        last_updated_time: Some(table.last_updated_time.timestamp() as f64),
        retention_properties: Some(wire::RetentionProperties {
            memory_store_retention_period_in_hours: table
                .retention_properties
                .memory_store_retention_period_in_hours,
            magnetic_store_retention_period_in_days: table
                .retention_properties
                .magnetic_store_retention_period_in_days,
        }),
        magnetic_store_write_properties: Some(wire::MagneticStoreWriteProperties {
            enable_magnetic_store_writes: table
                .magnetic_store_write_properties
                .enable_magnetic_store_writes,
            ..Default::default()
        }),
        schema: None,
    }
}

/// Convert a state `BatchLoadTask` to the wire `BatchLoadTaskDescription` model type.
fn batch_load_task_to_description(
    task: &statetypes::BatchLoadTask,
) -> wire::BatchLoadTaskDescription {
    let dsc = &task.data_source_configuration;
    let data_source_configuration = Some(wire::DataSourceConfiguration {
        data_format: dsc
            .get("DataFormat")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string(),
        data_source_s3_configuration: {
            let s3cfg = dsc.get("DataSourceS3Configuration");
            wire::DataSourceS3Configuration {
                bucket_name: s3cfg
                    .and_then(|v| v.get("BucketName"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                object_key_prefix: s3cfg
                    .and_then(|v| v.get("ObjectKeyPrefix"))
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string()),
            }
        },
        csv_configuration: dsc
            .get("CsvConfiguration")
            .map(|csv| wire::CsvConfiguration {
                column_separator: csv
                    .get("ColumnSeparator")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string()),
                escape_char: csv
                    .get("EscapeChar")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string()),
                null_value: csv
                    .get("NullValue")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string()),
                quote_char: csv
                    .get("QuoteChar")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string()),
                trim_white_space: csv.get("TrimWhiteSpace").and_then(|v| v.as_bool()),
            }),
    });

    let rc = &task.report_configuration;
    let report_configuration = Some(wire::ReportConfiguration {
        report_s3_configuration: rc.get("ReportS3Configuration").map(|s3| {
            wire::ReportS3Configuration {
                bucket_name: s3
                    .get("BucketName")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                encryption_option: s3
                    .get("EncryptionOption")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string()),
                kms_key_id: s3
                    .get("KmsKeyId")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string()),
                object_key_prefix: s3
                    .get("ObjectKeyPrefix")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string()),
            }
        }),
    });

    let data_model_configuration =
        task.data_model_configuration
            .as_ref()
            .map(|dmc| wire::DataModelConfiguration {
                data_model: dmc.get("DataModel").map(|dm| wire::DataModel {
                    dimension_mappings: dm
                        .get("DimensionMappings")
                        .and_then(|v| v.as_array())
                        .map(|arr| {
                            arr.iter()
                                .map(|m| wire::DimensionMapping {
                                    source_column: m
                                        .get("SourceColumn")
                                        .and_then(|v| v.as_str())
                                        .map(|s| s.to_string()),
                                    destination_column: m
                                        .get("DestinationColumn")
                                        .and_then(|v| v.as_str())
                                        .map(|s| s.to_string()),
                                })
                                .collect()
                        })
                        .unwrap_or_default(),
                    measure_name_column: dm
                        .get("MeasureNameColumn")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string()),
                    time_column: dm
                        .get("TimeColumn")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string()),
                    time_unit: dm
                        .get("TimeUnit")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string()),
                    mixed_measure_mappings: None,
                    multi_measure_mappings: None,
                }),
                data_model_s3_configuration: dmc.get("DataModelS3Configuration").map(|s3| {
                    wire::DataModelS3Configuration {
                        bucket_name: s3
                            .get("BucketName")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string()),
                        object_key: s3
                            .get("ObjectKey")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string()),
                    }
                }),
            });

    wire::BatchLoadTaskDescription {
        task_id: Some(task.task_id.clone()),
        task_status: Some(task.task_status.clone()),
        target_database_name: Some(task.database_name.clone()),
        target_table_name: Some(task.table_name.clone()),
        data_source_configuration,
        report_configuration,
        data_model_configuration,
        error_message: task.error_message.clone(),
        creation_time: Some(task.creation_time.timestamp() as f64),
        last_updated_time: Some(task.last_updated_time.timestamp() as f64),
        resumable_until: None,
        progress_report: Some(wire::BatchLoadProgressReport::default()),
        record_version: None,
    }
}

fn timestream_error_response(err: &TimestreamWriteError) -> MockResponse {
    let (status, error_type) = match err {
        TimestreamWriteError::DatabaseAlreadyExists { .. }
        | TimestreamWriteError::TableAlreadyExists { .. } => (409u16, "ConflictException"),
        TimestreamWriteError::DatabaseNotEmpty { .. }
        | TimestreamWriteError::TaskCannotBeResumed { .. } => (400, "ValidationException"),
        TimestreamWriteError::DatabaseNotFound { .. }
        | TimestreamWriteError::TableNotFound { .. }
        | TimestreamWriteError::ResourceNotFound { .. }
        | TimestreamWriteError::TaskNotFound { .. } => (404, "ResourceNotFoundException"),
    };
    let body = json!({
        "__type": error_type,
        "message": err.to_string(),
    });
    MockResponse::json(status, body.to_string())
}

fn json_error_response(status: u16, error_type: &str, message: &str) -> MockResponse {
    let body = json!({
        "__type": error_type,
        "message": message,
    });
    MockResponse::json(status, body.to_string())
}
