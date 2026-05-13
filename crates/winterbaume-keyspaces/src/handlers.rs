use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::Value;
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, StatefulService,
    default_account_id, json_error_response,
};

use crate::state::{KeyspacesError, KeyspacesState};
use crate::types;
use crate::views::KeyspacesStateView;
use crate::wire;

pub struct KeyspacesService {
    pub(crate) state: Arc<BackendState<KeyspacesState>>,
    pub(crate) notifier: StateChangeNotifier<KeyspacesStateView>,
}

impl KeyspacesService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for KeyspacesService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for KeyspacesService {
    fn service_name(&self) -> &str {
        "cassandra"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://cassandra\..*\.amazonaws\.com",
            r"https?://cassandra\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl KeyspacesService {
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
            None => {
                return json_error_response(400, "MissingAction", "Missing X-Amz-Target header");
            }
        };

        // Validate the body is well-formed JSON up-front; the typed deserialisers in
        // `wire` re-parse the bytes per operation.
        if !request.body.is_empty() && serde_json::from_slice::<Value>(&request.body).is_err() {
            return json_error_response(400, "SerializationException", "Invalid JSON body");
        }
        let body_bytes: &[u8] = &request.body;

        let state = self.state.get(account_id, &region);

        let response = match action.as_str() {
            "CreateKeyspace" => {
                self.handle_create_keyspace(&state, body_bytes, account_id, &region)
                    .await
            }
            "GetKeyspace" => self.handle_get_keyspace(&state, body_bytes).await,
            "DeleteKeyspace" => self.handle_delete_keyspace(&state, body_bytes).await,
            "UpdateKeyspace" => self.handle_update_keyspace(&state, body_bytes).await,
            "ListKeyspaces" => self.handle_list_keyspaces(&state).await,
            "CreateTable" => {
                self.handle_create_table(&state, body_bytes, account_id, &region)
                    .await
            }
            "GetTable" => self.handle_get_table(&state, body_bytes).await,
            "DeleteTable" => self.handle_delete_table(&state, body_bytes).await,
            "UpdateTable" => self.handle_update_table(&state, body_bytes).await,
            "ListTables" => self.handle_list_tables(&state, body_bytes).await,
            "RestoreTable" => {
                self.handle_restore_table(&state, body_bytes, account_id, &region)
                    .await
            }
            "GetTableAutoScalingSettings" => {
                self.handle_get_table_auto_scaling_settings(&state, body_bytes)
                    .await
            }
            "CreateType" => self.handle_create_type(&state, body_bytes).await,
            "GetType" => {
                self.handle_get_type(&state, body_bytes, account_id, &region)
                    .await
            }
            "DeleteType" => {
                self.handle_delete_type(&state, body_bytes, account_id, &region)
                    .await
            }
            "ListTypes" => self.handle_list_types(&state, body_bytes).await,
            "TagResource" => self.handle_tag_resource(&state, body_bytes).await,
            "UntagResource" => self.handle_untag_resource(&state, body_bytes).await,
            "ListTagsForResource" => self.handle_list_tags_for_resource(&state, body_bytes).await,
            _ => json_error_response(400, "InvalidAction", &format!("Unknown operation {action}")),
        };

        if response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    // ---- Keyspace handlers ----

    async fn handle_create_keyspace(
        &self,
        state: &Arc<tokio::sync::RwLock<KeyspacesState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_keyspace_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.keyspace_name.is_empty() {
            return json_error_response(400, "ValidationException", "keyspaceName is required");
        }

        let (replication_strategy, replication_regions) =
            replication_spec_to_pair(input.replication_specification.as_ref());
        let tags = tag_list_to_map(input.tags.as_deref());

        let mut guard = state.write().await;
        match guard.create_keyspace(
            &input.keyspace_name,
            &replication_strategy,
            replication_regions,
            tags,
            account_id,
            region,
        ) {
            Ok(arn) => wire::serialize_create_keyspace_response(&wire::CreateKeyspaceResponse {
                resource_arn: Some(arn),
            }),
            Err(e) => keyspaces_error_response(&e),
        }
    }

    async fn handle_get_keyspace(
        &self,
        state: &Arc<tokio::sync::RwLock<KeyspacesState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_keyspace_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.keyspace_name.is_empty() {
            return json_error_response(400, "ValidationException", "keyspaceName is required");
        }

        let guard = state.read().await;
        match guard.get_keyspace(&input.keyspace_name) {
            Ok(ks) => wire::serialize_get_keyspace_response(&wire::GetKeyspaceResponse {
                keyspace_name: Some(ks.name.clone()),
                resource_arn: Some(ks.arn.clone()),
                replication_strategy: Some(ks.replication_strategy.clone()),
                replication_regions: if ks.replication_regions.is_empty() {
                    None
                } else {
                    Some(ks.replication_regions.clone())
                },
                ..Default::default()
            }),
            Err(e) => keyspaces_error_response(&e),
        }
    }

    async fn handle_delete_keyspace(
        &self,
        state: &Arc<tokio::sync::RwLock<KeyspacesState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_keyspace_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.keyspace_name.is_empty() {
            return json_error_response(400, "ValidationException", "keyspaceName is required");
        }

        let mut guard = state.write().await;
        match guard.delete_keyspace(&input.keyspace_name) {
            Ok(()) => wire::serialize_delete_keyspace_response(&wire::DeleteKeyspaceResponse {}),
            Err(e) => keyspaces_error_response(&e),
        }
    }

    async fn handle_update_keyspace(
        &self,
        state: &Arc<tokio::sync::RwLock<KeyspacesState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_keyspace_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.keyspace_name.is_empty() {
            return json_error_response(400, "ValidationException", "keyspaceName is required");
        }

        let (replication_strategy, replication_regions) =
            replication_spec_to_pair(Some(&input.replication_specification));

        let mut guard = state.write().await;
        match guard.update_keyspace(
            &input.keyspace_name,
            &replication_strategy,
            replication_regions,
        ) {
            Ok(arn) => wire::serialize_update_keyspace_response(&wire::UpdateKeyspaceResponse {
                resource_arn: Some(arn),
            }),
            Err(e) => keyspaces_error_response(&e),
        }
    }

    async fn handle_list_keyspaces(
        &self,
        state: &Arc<tokio::sync::RwLock<KeyspacesState>>,
    ) -> MockResponse {
        let guard = state.read().await;
        let keyspaces = guard.list_keyspaces();

        wire::serialize_list_keyspaces_response(&wire::ListKeyspacesResponse {
            keyspaces: Some(
                keyspaces
                    .into_iter()
                    .map(|ks| wire::KeyspaceSummary {
                        keyspace_name: Some(ks.name.clone()),
                        resource_arn: Some(ks.arn.clone()),
                        replication_strategy: Some(ks.replication_strategy.clone()),
                        replication_regions: if ks.replication_regions.is_empty() {
                            None
                        } else {
                            Some(ks.replication_regions.clone())
                        },
                    })
                    .collect(),
            ),
            ..Default::default()
        })
    }

    // ---- Table handlers ----

    async fn handle_create_table(
        &self,
        state: &Arc<tokio::sync::RwLock<KeyspacesState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_table_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.keyspace_name.is_empty() {
            return json_error_response(400, "ValidationException", "keyspaceName is required");
        }
        if input.table_name.is_empty() {
            return json_error_response(400, "ValidationException", "tableName is required");
        }

        // The Smithy schemaDefinition member is required, but defaults to empty when absent.
        // Treat an empty allColumns/partitionKeys as missing to preserve existing behaviour.
        if input.schema_definition.all_columns.is_empty()
            && input.schema_definition.partition_keys.is_empty()
        {
            return json_error_response(400, "ValidationException", "schemaDefinition is required");
        }
        let schema = wire_schema_to_types(&input.schema_definition);

        let (capacity_mode, rcu, wcu) =
            capacity_spec_to_tuple(input.capacity_specification.as_ref(), "PAY_PER_REQUEST");
        let (encryption_type, kms_key) =
            encryption_spec_to_tuple(input.encryption_specification.as_ref());
        let pitr = input
            .point_in_time_recovery
            .as_ref()
            .map(|p| p.status == "ENABLED")
            .unwrap_or(false);
        let ttl_status = input
            .ttl
            .as_ref()
            .map(|t| t.status.clone())
            .unwrap_or_else(|| "ENABLED".to_string());
        let default_ttl = input.default_time_to_live;
        let comment = input
            .comment
            .as_ref()
            .map(|c| c.message.clone())
            .unwrap_or_default();
        let cst = input
            .client_side_timestamps
            .as_ref()
            .map(|c| c.status == "ENABLED")
            .unwrap_or(false);
        let tags = tag_list_to_map(input.tags.as_deref());

        let mut guard = state.write().await;
        match guard.create_table(
            &input.keyspace_name,
            &input.table_name,
            schema,
            &capacity_mode,
            rcu,
            wcu,
            &encryption_type,
            kms_key,
            pitr,
            &ttl_status,
            default_ttl,
            &comment,
            cst,
            tags,
            account_id,
            region,
        ) {
            Ok(arn) => wire::serialize_create_table_response(&wire::CreateTableResponse {
                resource_arn: Some(arn),
            }),
            Err(e) => keyspaces_error_response(&e),
        }
    }

    async fn handle_get_table(
        &self,
        state: &Arc<tokio::sync::RwLock<KeyspacesState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_table_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.keyspace_name.is_empty() {
            return json_error_response(400, "ValidationException", "keyspaceName is required");
        }
        if input.table_name.is_empty() {
            return json_error_response(400, "ValidationException", "tableName is required");
        }

        let guard = state.read().await;
        match guard.get_table(&input.keyspace_name, &input.table_name) {
            Ok(table) => wire::serialize_get_table_response(&wire::GetTableResponse {
                keyspace_name: Some(table.keyspace_name.clone()),
                table_name: Some(table.table_name.clone()),
                resource_arn: Some(table.arn.clone()),
                creation_timestamp: Some(table.creation_timestamp.timestamp() as f64),
                status: Some(table.status.clone()),
                schema_definition: Some(table_schema_to_wire(&table.schema_definition)),
                capacity_specification: Some(wire::CapacitySpecificationSummary {
                    throughput_mode: Some(table.capacity_mode.clone()),
                    read_capacity_units: table.read_capacity_units,
                    write_capacity_units: table.write_capacity_units,
                    ..Default::default()
                }),
                encryption_specification: Some(wire::EncryptionSpecification {
                    r#type: table.encryption_type.clone(),
                    kms_key_identifier: table.kms_key_identifier.clone(),
                }),
                point_in_time_recovery: Some(wire::PointInTimeRecoverySummary {
                    status: Some(if table.point_in_time_recovery_enabled {
                        "ENABLED".to_string()
                    } else {
                        "DISABLED".to_string()
                    }),
                    ..Default::default()
                }),
                ttl: Some(wire::TimeToLive {
                    status: table.ttl_status.clone(),
                }),
                default_time_to_live: table.default_time_to_live,
                comment: if table.comment.is_empty() {
                    None
                } else {
                    Some(wire::Comment {
                        message: table.comment.clone(),
                    })
                },
                client_side_timestamps: Some(wire::ClientSideTimestamps {
                    status: if table.client_side_timestamps_enabled {
                        "ENABLED".to_string()
                    } else {
                        "DISABLED".to_string()
                    },
                }),
                ..Default::default()
            }),
            Err(e) => keyspaces_error_response(&e),
        }
    }

    async fn handle_delete_table(
        &self,
        state: &Arc<tokio::sync::RwLock<KeyspacesState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_table_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.keyspace_name.is_empty() {
            return json_error_response(400, "ValidationException", "keyspaceName is required");
        }
        if input.table_name.is_empty() {
            return json_error_response(400, "ValidationException", "tableName is required");
        }

        let mut guard = state.write().await;
        match guard.delete_table(&input.keyspace_name, &input.table_name) {
            Ok(()) => wire::serialize_delete_table_response(&wire::DeleteTableResponse {}),
            Err(e) => keyspaces_error_response(&e),
        }
    }

    async fn handle_update_table(
        &self,
        state: &Arc<tokio::sync::RwLock<KeyspacesState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_table_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.keyspace_name.is_empty() {
            return json_error_response(400, "ValidationException", "keyspaceName is required");
        }
        if input.table_name.is_empty() {
            return json_error_response(400, "ValidationException", "tableName is required");
        }

        let capacity_mode = input
            .capacity_specification
            .as_ref()
            .map(|c| c.throughput_mode.clone());
        let rcu = input
            .capacity_specification
            .as_ref()
            .and_then(|c| c.read_capacity_units);
        let wcu = input
            .capacity_specification
            .as_ref()
            .and_then(|c| c.write_capacity_units);
        let encryption_type = input
            .encryption_specification
            .as_ref()
            .map(|e| e.r#type.clone());
        let kms_key = input
            .encryption_specification
            .as_ref()
            .and_then(|e| e.kms_key_identifier.clone());
        let pitr = input
            .point_in_time_recovery
            .as_ref()
            .map(|p| p.status == "ENABLED");
        let ttl_status = input.ttl.as_ref().map(|t| t.status.clone());
        let default_ttl = input.default_time_to_live;
        let cst = input
            .client_side_timestamps
            .as_ref()
            .map(|c| c.status == "ENABLED");

        let mut guard = state.write().await;

        // Handle addColumns first.
        if let Some(add_cols) = input.add_columns.as_deref() {
            let key = (input.keyspace_name.clone(), input.table_name.clone());
            if let Some(table) = guard.tables.get_mut(&key) {
                for col in add_cols {
                    table
                        .schema_definition
                        .all_columns
                        .push(types::ColumnDefinition {
                            name: col.name.clone(),
                            column_type: col.r#type.clone(),
                        });
                }
            }
        }

        match guard.update_table(
            &input.keyspace_name,
            &input.table_name,
            capacity_mode.as_deref(),
            rcu,
            wcu,
            encryption_type.as_deref(),
            kms_key,
            pitr,
            ttl_status.as_deref(),
            default_ttl,
            cst,
        ) {
            Ok(arn) => wire::serialize_update_table_response(&wire::UpdateTableResponse {
                resource_arn: Some(arn),
            }),
            Err(e) => keyspaces_error_response(&e),
        }
    }

    async fn handle_list_tables(
        &self,
        state: &Arc<tokio::sync::RwLock<KeyspacesState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_tables_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.keyspace_name.is_empty() {
            return json_error_response(400, "ValidationException", "keyspaceName is required");
        }

        let guard = state.read().await;
        match guard.list_tables(&input.keyspace_name) {
            Ok(tables) => wire::serialize_list_tables_response(&wire::ListTablesResponse {
                tables: Some(
                    tables
                        .into_iter()
                        .map(|t| wire::TableSummary {
                            keyspace_name: Some(t.keyspace_name.clone()),
                            table_name: Some(t.table_name.clone()),
                            resource_arn: Some(t.arn.clone()),
                        })
                        .collect(),
                ),
                ..Default::default()
            }),
            Err(e) => keyspaces_error_response(&e),
        }
    }

    async fn handle_restore_table(
        &self,
        state: &Arc<tokio::sync::RwLock<KeyspacesState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_restore_table_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.source_keyspace_name.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "sourceKeyspaceName is required",
            );
        }
        if input.source_table_name.is_empty() {
            return json_error_response(400, "ValidationException", "sourceTableName is required");
        }
        if input.target_keyspace_name.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "targetKeyspaceName is required",
            );
        }
        if input.target_table_name.is_empty() {
            return json_error_response(400, "ValidationException", "targetTableName is required");
        }

        let mut guard = state.write().await;
        match guard.restore_table(
            &input.source_keyspace_name,
            &input.source_table_name,
            &input.target_keyspace_name,
            &input.target_table_name,
            account_id,
            region,
        ) {
            Ok(arn) => wire::serialize_restore_table_response(&wire::RestoreTableResponse {
                restored_table_a_r_n: Some(arn),
            }),
            Err(e) => keyspaces_error_response(&e),
        }
    }

    async fn handle_get_table_auto_scaling_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<KeyspacesState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_table_auto_scaling_settings_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.keyspace_name.is_empty() {
            return json_error_response(400, "ValidationException", "keyspaceName is required");
        }
        if input.table_name.is_empty() {
            return json_error_response(400, "ValidationException", "tableName is required");
        }

        let guard = state.read().await;
        match guard.get_table(&input.keyspace_name, &input.table_name) {
            Ok(table) => wire::serialize_get_table_auto_scaling_settings_response(
                &wire::GetTableAutoScalingSettingsResponse {
                    keyspace_name: Some(table.keyspace_name.clone()),
                    table_name: Some(table.table_name.clone()),
                    resource_arn: Some(table.arn.clone()),
                    ..Default::default()
                },
            ),
            Err(e) => keyspaces_error_response(&e),
        }
    }

    // ---- Type handlers ----

    async fn handle_create_type(
        &self,
        state: &Arc<tokio::sync::RwLock<KeyspacesState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_create_type_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.keyspace_name.is_empty() {
            return json_error_response(400, "ValidationException", "keyspaceName is required");
        }
        if input.type_name.is_empty() {
            return json_error_response(400, "ValidationException", "typeName is required");
        }

        let field_defs: Vec<types::FieldDefinition> = input
            .field_definitions
            .into_iter()
            .map(|f| types::FieldDefinition {
                name: f.name,
                field_type: f.r#type,
            })
            .collect();

        let mut guard = state.write().await;
        // Get keyspace ARN for the response.
        let keyspace_arn = guard
            .get_keyspace(&input.keyspace_name)
            .ok()
            .map(|ks| ks.arn.clone());

        match guard.create_type(&input.keyspace_name, &input.type_name, field_defs) {
            Ok(_) => wire::serialize_create_type_response(&wire::CreateTypeResponse {
                keyspace_arn,
                type_name: Some(input.type_name.clone()),
            }),
            Err(e) => keyspaces_error_response(&e),
        }
    }

    async fn handle_get_type(
        &self,
        state: &Arc<tokio::sync::RwLock<KeyspacesState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_get_type_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.keyspace_name.is_empty() {
            return json_error_response(400, "ValidationException", "keyspaceName is required");
        }
        if input.type_name.is_empty() {
            return json_error_response(400, "ValidationException", "typeName is required");
        }

        let guard = state.read().await;
        match guard.get_type(&input.keyspace_name, &input.type_name) {
            Ok(udt) => {
                let keyspace_arn = format!(
                    "arn:aws:cassandra:{region}:{account_id}:/keyspace/{}/",
                    input.keyspace_name
                );
                wire::serialize_get_type_response(&wire::GetTypeResponse {
                    keyspace_name: Some(udt.keyspace_name.clone()),
                    type_name: Some(udt.type_name.clone()),
                    keyspace_arn: Some(keyspace_arn),
                    field_definitions: Some(
                        udt.field_definitions
                            .iter()
                            .map(|f| wire::FieldDefinition {
                                name: f.name.clone(),
                                r#type: f.field_type.clone(),
                            })
                            .collect(),
                    ),
                    last_modified_timestamp: Some(udt.creation_timestamp.timestamp() as f64),
                    status: Some(udt.status.clone()),
                    ..Default::default()
                })
            }
            Err(e) => keyspaces_error_response(&e),
        }
    }

    async fn handle_delete_type(
        &self,
        state: &Arc<tokio::sync::RwLock<KeyspacesState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_type_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.keyspace_name.is_empty() {
            return json_error_response(400, "ValidationException", "keyspaceName is required");
        }
        if input.type_name.is_empty() {
            return json_error_response(400, "ValidationException", "typeName is required");
        }

        let keyspace_arn = format!(
            "arn:aws:cassandra:{region}:{account_id}:/keyspace/{}/",
            input.keyspace_name
        );

        let mut guard = state.write().await;
        match guard.delete_type(&input.keyspace_name, &input.type_name) {
            Ok(()) => wire::serialize_delete_type_response(&wire::DeleteTypeResponse {
                keyspace_arn: Some(keyspace_arn),
                type_name: Some(input.type_name.clone()),
            }),
            Err(e) => keyspaces_error_response(&e),
        }
    }

    async fn handle_list_types(
        &self,
        state: &Arc<tokio::sync::RwLock<KeyspacesState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_types_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.keyspace_name.is_empty() {
            return json_error_response(400, "ValidationException", "keyspaceName is required");
        }

        let guard = state.read().await;
        match guard.list_types(&input.keyspace_name) {
            Ok(types) => wire::serialize_list_types_response(&wire::ListTypesResponse {
                types: Some(types.into_iter().map(|t| t.type_name.clone()).collect()),
                ..Default::default()
            }),
            Err(e) => keyspaces_error_response(&e),
        }
    }

    // ---- Tag handlers ----

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<KeyspacesState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(400, "ValidationException", "resourceArn is required");
        }

        let tags = tag_list_to_map(Some(input.tags.as_slice()));

        let mut guard = state.write().await;
        match guard.tag_resource(&input.resource_arn, tags) {
            Ok(()) => wire::serialize_tag_resource_response(&wire::TagResourceResponse {}),
            Err(e) => keyspaces_error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<KeyspacesState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(400, "ValidationException", "resourceArn is required");
        }

        let tag_keys: Vec<String> = input.tags.into_iter().map(|t| t.key).collect();

        let mut guard = state.write().await;
        match guard.untag_resource(&input.resource_arn, &tag_keys) {
            Ok(()) => wire::serialize_untag_resource_response(&wire::UntagResourceResponse {}),
            Err(e) => keyspaces_error_response(&e),
        }
    }

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<KeyspacesState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(400, "ValidationException", "resourceArn is required");
        }

        let guard = state.read().await;
        match guard.get_tags_for_resource(&input.resource_arn) {
            Ok(tags) => wire::serialize_list_tags_for_resource_response(
                &wire::ListTagsForResourceResponse {
                    tags: Some(
                        tags.into_iter()
                            .map(|(k, v)| wire::Tag { key: k, value: v })
                            .collect(),
                    ),
                    ..Default::default()
                },
            ),
            Err(e) => keyspaces_error_response(&e),
        }
    }
}

// ---- Error shaping ----

fn keyspaces_error_response(err: &KeyspacesError) -> MockResponse {
    let (status, error_type) = match err {
        KeyspacesError::NotFound { .. } => (404, "ResourceNotFoundException"),
        KeyspacesError::AlreadyExists { .. } => (409, "ConflictException"),
        KeyspacesError::Validation { .. } => (400, "ValidationException"),
        KeyspacesError::Conflict { .. } => (409, "ConflictException"),
    };
    json_error_response(status, error_type, &err.to_string())
}

// ---- Wire to domain helpers ----

fn replication_spec_to_pair(
    spec: Option<&wire::ReplicationSpecification>,
) -> (String, Vec<String>) {
    match spec {
        Some(s) => {
            let strategy = if s.replication_strategy.is_empty() {
                "SINGLE_REGION".to_string()
            } else {
                s.replication_strategy.clone()
            };
            let regions = s.region_list.clone().unwrap_or_default();
            (strategy, regions)
        }
        None => ("SINGLE_REGION".to_string(), Vec::new()),
    }
}

fn tag_list_to_map(tags: Option<&[wire::Tag]>) -> HashMap<String, String> {
    let mut map = HashMap::new();
    if let Some(list) = tags {
        for t in list {
            map.insert(t.key.clone(), t.value.clone());
        }
    }
    map
}

fn capacity_spec_to_tuple(
    spec: Option<&wire::CapacitySpecification>,
    default_mode: &str,
) -> (String, Option<i64>, Option<i64>) {
    match spec {
        Some(s) => {
            let mode = if s.throughput_mode.is_empty() {
                default_mode.to_string()
            } else {
                s.throughput_mode.clone()
            };
            (mode, s.read_capacity_units, s.write_capacity_units)
        }
        None => (default_mode.to_string(), None, None),
    }
}

fn encryption_spec_to_tuple(
    spec: Option<&wire::EncryptionSpecification>,
) -> (String, Option<String>) {
    match spec {
        Some(s) => {
            let enc_type = if s.r#type.is_empty() {
                "AWS_OWNED_KMS_KEY".to_string()
            } else {
                s.r#type.clone()
            };
            (enc_type, s.kms_key_identifier.clone())
        }
        None => ("AWS_OWNED_KMS_KEY".to_string(), None),
    }
}

fn wire_schema_to_types(schema: &wire::SchemaDefinition) -> types::SchemaDefinition {
    types::SchemaDefinition {
        all_columns: schema
            .all_columns
            .iter()
            .map(|c| types::ColumnDefinition {
                name: c.name.clone(),
                column_type: c.r#type.clone(),
            })
            .collect(),
        partition_keys: schema
            .partition_keys
            .iter()
            .map(|p| p.name.clone())
            .collect(),
        clustering_keys: schema
            .clustering_keys
            .as_deref()
            .map(|cs| {
                cs.iter()
                    .map(|c| types::ClusteringKey {
                        name: c.name.clone(),
                        order_by: if c.order_by.is_empty() {
                            "ASC".to_string()
                        } else {
                            c.order_by.clone()
                        },
                    })
                    .collect()
            })
            .unwrap_or_default(),
        static_columns: schema
            .static_columns
            .as_deref()
            .map(|cols| cols.iter().map(|c| c.name.clone()).collect())
            .unwrap_or_default(),
    }
}

fn table_schema_to_wire(schema: &types::SchemaDefinition) -> wire::SchemaDefinition {
    wire::SchemaDefinition {
        all_columns: schema
            .all_columns
            .iter()
            .map(|c| wire::ColumnDefinition {
                name: c.name.clone(),
                r#type: c.column_type.clone(),
            })
            .collect(),
        partition_keys: schema
            .partition_keys
            .iter()
            .map(|p| wire::PartitionKey { name: p.clone() })
            .collect(),
        clustering_keys: if schema.clustering_keys.is_empty() {
            None
        } else {
            Some(
                schema
                    .clustering_keys
                    .iter()
                    .map(|c| wire::ClusteringKey {
                        name: c.name.clone(),
                        order_by: c.order_by.clone(),
                    })
                    .collect(),
            )
        },
        static_columns: if schema.static_columns.is_empty() {
            None
        } else {
            Some(
                schema
                    .static_columns
                    .iter()
                    .map(|s| wire::StaticColumn { name: s.clone() })
                    .collect(),
            )
        },
    }
}
