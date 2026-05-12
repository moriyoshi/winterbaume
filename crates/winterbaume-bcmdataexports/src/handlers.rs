use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::Value;
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, default_account_id,
    json_error_response,
};

use crate::state::{BcmDataExportsError, BcmDataExportsState};
use crate::types::Export;
use crate::views::BcmDataExportsStateView;
use crate::wire;

pub struct BcmDataExportsService {
    pub(crate) state: Arc<BackendState<BcmDataExportsState>>,
    pub(crate) notifier: StateChangeNotifier<BcmDataExportsStateView>,
}

impl BcmDataExportsService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for BcmDataExportsService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for BcmDataExportsService {
    fn service_name(&self) -> &str {
        "bcm-data-exports"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://bcm-data-exports\..*\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

type SharedState = Arc<tokio::sync::RwLock<BcmDataExportsState>>;

impl BcmDataExportsService {
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

        // Validate JSON, then pass raw bytes to typed deserialisers.
        if !request.body.is_empty() && serde_json::from_slice::<Value>(&request.body).is_err() {
            return json_error_response(400, "SerializationException", "Invalid JSON body");
        }
        // For empty bodies, normalise to "{}" so typed deserialisers
        // see a parseable payload.
        let body_owned: Vec<u8> = if request.body.is_empty() {
            b"{}".to_vec()
        } else {
            request.body.to_vec()
        };
        let body_bytes: &[u8] = &body_owned;

        let state = self.state.get(account_id, &region);

        let response = match action.as_str() {
            "CreateExport" => {
                self.handle_create_export(&state, account_id, &region, body_bytes)
                    .await
            }
            "DeleteExport" => self.handle_delete_export(&state, body_bytes).await,
            "GetExecution" => self.handle_get_execution(&state, body_bytes).await,
            "GetExport" => self.handle_get_export(&state, body_bytes).await,
            "GetTable" => self.handle_get_table(&state, body_bytes).await,
            "ListExecutions" => self.handle_list_executions(&state, body_bytes).await,
            "ListExports" => self.handle_list_exports(&state, body_bytes).await,
            "ListTables" => self.handle_list_tables(&state, body_bytes).await,
            "ListTagsForResource" => self.handle_list_tags(&state, body_bytes).await,
            "TagResource" => self.handle_tag_resource(&state, body_bytes).await,
            "UntagResource" => self.handle_untag_resource(&state, body_bytes).await,
            "UpdateExport" => self.handle_update_export(&state, body_bytes).await,
            other => json_error_response(
                400,
                "UnknownOperationException",
                &format!("Unknown action: {other}"),
            ),
        };

        use winterbaume_core::StatefulService;
        if response.status / 100 == 2
            && matches!(
                action.as_str(),
                "CreateExport" | "DeleteExport" | "TagResource" | "UntagResource" | "UpdateExport"
            )
        {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_create_export(
        &self,
        state: &SharedState,
        account_id: &str,
        region: &str,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_create_export_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.export.name.is_empty() {
            return json_error_response(400, "ValidationException", "Export.Name is required");
        }
        let name = input.export.name.clone();

        let arn = format!(
            "arn:aws:bcm-data-exports:{}:{}:export/{}",
            region,
            account_id,
            uuid::Uuid::new_v4()
        );
        let now = chrono::Utc::now().to_rfc3339();
        let resource_tags = input
            .resource_tags
            .as_deref()
            .map(|tags| {
                tags.iter()
                    .map(|t| (t.key.clone(), t.value.clone()))
                    .collect::<HashMap<_, _>>()
            })
            .unwrap_or_default();
        let export = Export {
            arn: arn.clone(),
            name,
            description: input.export.description.clone(),
            data_query: serde_json::to_value(&input.export.data_query).unwrap_or_default(),
            destination_configurations: serde_json::to_value(
                &input.export.destination_configurations,
            )
            .unwrap_or_default(),
            refresh_cadence: serde_json::to_value(&input.export.refresh_cadence)
                .unwrap_or_default(),
            created_at: now.clone(),
            last_updated_at: now,
            last_refreshed_at: None,
            status_code: "HEALTHY".to_string(),
            status_reason: None,
            tags: resource_tags,
        };

        let mut state = state.write().await;
        match state.create_export(export) {
            Ok(_) => wire::serialize_create_export_response(&wire::CreateExportResponse {
                export_arn: Some(arn),
            }),
            Err(e) => bcm_error_response(&e),
        }
    }

    async fn handle_delete_export(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_delete_export_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.export_arn.is_empty() {
            return json_error_response(400, "ValidationException", "ExportArn is required");
        }
        let arn = input.export_arn.clone();
        let mut state = state.write().await;
        match state.delete_export(&arn) {
            Ok(()) => wire::serialize_delete_export_response(&wire::DeleteExportResponse {
                export_arn: Some(arn),
            }),
            Err(e) => bcm_error_response(&e),
        }
    }

    async fn handle_get_export(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_get_export_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.export_arn.is_empty() {
            return json_error_response(400, "ValidationException", "ExportArn is required");
        }
        let arn = input.export_arn.clone();
        let state = state.read().await;
        match state.get_export(&arn) {
            Ok(e) => wire::serialize_get_export_response(&wire::GetExportResponse {
                export: Some(export_to_wire(e)),
                export_status: Some(wire::ExportStatus {
                    created_at: Some(e.created_at.clone()),
                    last_refreshed_at: e.last_refreshed_at.clone(),
                    last_updated_at: Some(e.last_updated_at.clone()),
                    status_code: Some(e.status_code.clone()),
                    status_reason: e.status_reason.clone(),
                }),
            }),
            Err(e) => bcm_error_response(&e),
        }
    }

    async fn handle_update_export(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        // Parse the raw body as a generic Value so we can detect which top-level
        // fields were actually present in `Export` (the typed deserialiser fills
        // missing fields with `Default::default()` and we'd otherwise overwrite
        // existing state with empty defaults).
        let raw: Value = match serde_json::from_slice(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e.to_string()),
        };
        let input = match wire::deserialize_update_export_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.export_arn.is_empty() {
            return json_error_response(400, "ValidationException", "ExportArn is required");
        }
        let arn = input.export_arn.clone();
        let raw_export = raw.get("Export").cloned();

        let mut state = state.write().await;
        match state.update_export(&arn, |e| {
            if let Some(eo) = raw_export.as_ref().and_then(|v| v.as_object()) {
                if eo.contains_key("Name") && !input.export.name.is_empty() {
                    e.name = input.export.name.clone();
                }
                if eo.contains_key("Description") {
                    e.description = input.export.description.clone();
                }
                if eo.contains_key("DataQuery") {
                    e.data_query =
                        serde_json::to_value(&input.export.data_query).unwrap_or_default();
                }
                if eo.contains_key("DestinationConfigurations") {
                    e.destination_configurations =
                        serde_json::to_value(&input.export.destination_configurations)
                            .unwrap_or_default();
                }
                if eo.contains_key("RefreshCadence") {
                    e.refresh_cadence =
                        serde_json::to_value(&input.export.refresh_cadence).unwrap_or_default();
                }
            }
        }) {
            Ok(_) => wire::serialize_update_export_response(&wire::UpdateExportResponse {
                export_arn: Some(arn),
            }),
            Err(e) => bcm_error_response(&e),
        }
    }

    async fn handle_list_exports(&self, state: &SharedState, _body: &[u8]) -> MockResponse {
        let state = state.read().await;
        let exports: Vec<wire::ExportReference> = state
            .list_exports()
            .into_iter()
            .map(|e| wire::ExportReference {
                export_arn: Some(e.arn.clone()),
                export_name: Some(e.name.clone()),
                export_status: Some(wire::ExportStatus {
                    created_at: Some(e.created_at.clone()),
                    last_refreshed_at: e.last_refreshed_at.clone(),
                    last_updated_at: Some(e.last_updated_at.clone()),
                    status_code: Some(e.status_code.clone()),
                    status_reason: e.status_reason.clone(),
                }),
            })
            .collect();
        wire::serialize_list_exports_response(&wire::ListExportsResponse {
            exports: Some(exports),
            next_token: None,
        })
    }

    async fn handle_list_executions(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_list_executions_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.export_arn.is_empty() {
            return json_error_response(400, "ValidationException", "ExportArn is required");
        }
        let arn = input.export_arn.clone();
        let state = state.read().await;
        if state.get_export(&arn).is_err() {
            return bcm_error_response(&BcmDataExportsError::ExportNotFound { arn });
        }
        let executions: Vec<wire::ExecutionReference> = state
            .list_executions(&arn)
            .into_iter()
            .map(|e| wire::ExecutionReference {
                execution_id: Some(e.id.clone()),
                execution_status: Some(wire::ExecutionStatus {
                    completed_at: e.completed_at.clone(),
                    created_at: Some(e.created_at.clone()),
                    last_updated_at: Some(e.last_updated_at.clone()),
                    status_code: Some(e.status_code.clone()),
                    status_reason: e.status_reason.clone(),
                }),
            })
            .collect();
        wire::serialize_list_executions_response(&wire::ListExecutionsResponse {
            executions: Some(executions),
            next_token: None,
        })
    }

    async fn handle_get_execution(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_get_execution_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.execution_id.is_empty() {
            return json_error_response(400, "ValidationException", "ExecutionId is required");
        }
        let id = input.execution_id.clone();
        let state = state.read().await;
        match state.get_execution(&id) {
            Ok(exec) => {
                let parent_export = state.get_export(&exec.export_arn).ok();
                wire::serialize_get_execution_response(&wire::GetExecutionResponse {
                    execution_id: Some(exec.id.clone()),
                    execution_status: Some(wire::ExecutionStatus {
                        completed_at: exec.completed_at.clone(),
                        created_at: Some(exec.created_at.clone()),
                        last_updated_at: Some(exec.last_updated_at.clone()),
                        status_code: Some(exec.status_code.clone()),
                        status_reason: exec.status_reason.clone(),
                    }),
                    export: parent_export.map(export_to_wire),
                })
            }
            Err(e) => bcm_error_response(&e),
        }
    }

    async fn handle_list_tables(&self, state: &SharedState, _body: &[u8]) -> MockResponse {
        let state = state.read().await;
        let tables: Vec<wire::Table> = state
            .list_tables()
            .into_iter()
            .map(|t| wire::Table {
                description: t.description.clone(),
                table_name: Some(t.name.clone()),
                table_properties: if t.properties.is_empty() {
                    None
                } else {
                    Some(
                        t.properties
                            .iter()
                            .map(|(k, v)| wire::TablePropertyDescription {
                                default_value: Some(v.clone()),
                                description: None,
                                name: Some(k.clone()),
                                valid_values: None,
                            })
                            .collect(),
                    )
                },
            })
            .collect();
        wire::serialize_list_tables_response(&wire::ListTablesResponse {
            next_token: None,
            tables: Some(tables),
        })
    }

    async fn handle_get_table(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_get_table_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.table_name.is_empty() {
            return json_error_response(400, "ValidationException", "TableName is required");
        }
        let name = input.table_name.clone();
        let state = state.read().await;
        match state.get_table(&name) {
            Ok(t) => {
                let schema: Vec<wire::Column> = t
                    .schema
                    .iter()
                    .map(|c| wire::Column {
                        description: c.description.clone(),
                        name: Some(c.name.clone()),
                        r#type: Some(c.r#type.clone()),
                    })
                    .collect();
                wire::serialize_get_table_response(&wire::GetTableResponse {
                    description: t.description.clone(),
                    schema: if schema.is_empty() {
                        None
                    } else {
                        Some(schema)
                    },
                    table_name: Some(t.name.clone()),
                    table_properties: if t.properties.is_empty() {
                        None
                    } else {
                        Some(t.properties.clone())
                    },
                })
            }
            Err(e) => bcm_error_response(&e),
        }
    }

    async fn handle_tag_resource(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(400, "ValidationException", "ResourceArn is required");
        }
        let arn = input.resource_arn.clone();
        if input.resource_tags.is_empty() {
            return json_error_response(400, "ValidationException", "ResourceTags is required");
        }
        let tags: HashMap<String, String> = input
            .resource_tags
            .iter()
            .map(|t| (t.key.clone(), t.value.clone()))
            .collect();
        let mut state = state.write().await;
        match state.tag_resource(&arn, tags) {
            Ok(()) => wire::serialize_tag_resource_response(&wire::TagResourceResponse {}),
            Err(e) => bcm_error_response(&e),
        }
    }

    async fn handle_untag_resource(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(400, "ValidationException", "ResourceArn is required");
        }
        let arn = input.resource_arn.clone();
        let keys = input.resource_tag_keys.clone();
        if keys.is_empty() {
            return json_error_response(400, "ValidationException", "ResourceTagKeys is required");
        }
        let mut state = state.write().await;
        match state.untag_resource(&arn, &keys) {
            Ok(()) => wire::serialize_untag_resource_response(&wire::UntagResourceResponse {}),
            Err(e) => bcm_error_response(&e),
        }
    }

    async fn handle_list_tags(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(400, "ValidationException", "ResourceArn is required");
        }
        let arn = input.resource_arn.clone();
        let state = state.read().await;
        match state.list_tags(&arn) {
            Ok(tags) => {
                let resource_tags: Vec<wire::ResourceTag> = tags
                    .into_iter()
                    .map(|(k, v)| wire::ResourceTag { key: k, value: v })
                    .collect();
                wire::serialize_list_tags_for_resource_response(
                    &wire::ListTagsForResourceResponse {
                        next_token: None,
                        resource_tags: if resource_tags.is_empty() {
                            None
                        } else {
                            Some(resource_tags)
                        },
                    },
                )
            }
            Err(e) => bcm_error_response(&e),
        }
    }
}

fn export_to_wire(e: &Export) -> wire::Export {
    wire::Export {
        data_query: serde_json::from_value(e.data_query.clone()).unwrap_or_default(),
        description: e.description.clone(),
        destination_configurations: serde_json::from_value(e.destination_configurations.clone())
            .unwrap_or_default(),
        export_arn: Some(e.arn.clone()),
        name: e.name.clone(),
        refresh_cadence: serde_json::from_value(e.refresh_cadence.clone()).unwrap_or_default(),
    }
}

fn bcm_error_response(err: &BcmDataExportsError) -> MockResponse {
    let (status, error_type) = match err {
        BcmDataExportsError::ExportNotFound { .. } => (404, "ResourceNotFoundException"),
        BcmDataExportsError::ExportAlreadyExists { .. } => (409, "ConflictException"),
        BcmDataExportsError::ExecutionNotFound { .. } => (404, "ResourceNotFoundException"),
        BcmDataExportsError::TableNotFound { .. } => (404, "ResourceNotFoundException"),
        BcmDataExportsError::Validation { .. } => (400, "ValidationException"),
    };
    json_error_response(status, error_type, &err.to_string())
}
