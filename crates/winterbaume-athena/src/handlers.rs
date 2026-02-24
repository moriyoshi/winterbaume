use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
    StatefulService, json_error_response,
};

use crate::backend::{AthenaQueryBackend, InMemoryAthenaQueryBackend};
use crate::state::{AthenaError, AthenaState};
use crate::views::AthenaStateView;
use crate::wire;

pub struct AthenaService {
    pub(crate) query_backend: Arc<dyn AthenaQueryBackend>,
    pub(crate) state: Arc<BackendState<AthenaState>>,
    pub(crate) notifier: StateChangeNotifier<AthenaStateView>,
}

impl AthenaService {
    pub fn new() -> Self {
        Self {
            query_backend: Arc::new(InMemoryAthenaQueryBackend),
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }

    /// Construct with a custom query execution backend.
    pub fn with_query_backend(query_backend: Arc<dyn AthenaQueryBackend>) -> Self {
        Self {
            query_backend,
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for AthenaService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for AthenaService {
    fn service_name(&self) -> &str {
        "athena"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://athena\..*\.amazonaws\.com",
            r"https?://athena\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl AthenaService {
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
            "CreateWorkGroup" => self.handle_create_work_group(&state, body_bytes).await,
            "GetWorkGroup" => self.handle_get_work_group(&state, body_bytes).await,
            "DeleteWorkGroup" => self.handle_delete_work_group(&state, body_bytes).await,
            "ListWorkGroups" => self.handle_list_work_groups(&state).await,
            "CreateCapacityReservation" => {
                self.handle_create_capacity_reservation(&state, body_bytes)
                    .await
            }
            "GetCapacityReservation" => {
                self.handle_get_capacity_reservation(&state, body_bytes)
                    .await
            }
            "ListCapacityReservations" => self.handle_list_capacity_reservations(&state).await,
            "UpdateCapacityReservation" => {
                self.handle_update_capacity_reservation(&state, body_bytes)
                    .await
            }
            "CreateDataCatalog" => {
                self.handle_create_data_catalog(&state, body_bytes, account_id, &region)
                    .await
            }
            "GetDataCatalog" => self.handle_get_data_catalog(&state, body_bytes).await,
            "ListDataCatalogs" => self.handle_list_data_catalogs(&state).await,
            "CreateNamedQuery" => self.handle_create_named_query(&state, body_bytes).await,
            "GetNamedQuery" => self.handle_get_named_query(&state, body_bytes).await,
            "ListNamedQueries" => self.handle_list_named_queries(&state).await,
            "CreatePreparedStatement" => {
                self.handle_create_prepared_statement(&state, body_bytes)
                    .await
            }
            "GetPreparedStatement" => self.handle_get_prepared_statement(&state, body_bytes).await,
            "StartQueryExecution" => self.handle_start_query_execution(&state, body_bytes).await,
            "GetQueryExecution" => self.handle_get_query_execution(&state, body_bytes).await,
            "GetQueryResults" => self.handle_get_query_results(&state, body_bytes).await,
            "GetQueryRuntimeStatistics" => {
                self.handle_get_query_runtime_statistics(&state, body_bytes)
                    .await
            }
            "StopQueryExecution" => self.handle_stop_query_execution(&state, body_bytes).await,
            "ListQueryExecutions" => self.handle_list_query_executions(&state).await,
            "ListTagsForResource" => self.handle_list_tags_for_resource(&state, body_bytes).await,
            "TagResource" => self.handle_tag_resource(&state, body_bytes).await,
            "UntagResource" => self.handle_untag_resource(&state, body_bytes).await,
            // --- Unimplemented operations (auto-generated stubs) ---
            "BatchGetNamedQuery" => json_error_response(
                501,
                "NotImplementedError",
                "BatchGetNamedQuery is not yet implemented in winterbaume-athena",
            ),
            "BatchGetPreparedStatement" => json_error_response(
                501,
                "NotImplementedError",
                "BatchGetPreparedStatement is not yet implemented in winterbaume-athena",
            ),
            "BatchGetQueryExecution" => json_error_response(
                501,
                "NotImplementedError",
                "BatchGetQueryExecution is not yet implemented in winterbaume-athena",
            ),
            "CancelCapacityReservation" => json_error_response(
                501,
                "NotImplementedError",
                "CancelCapacityReservation is not yet implemented in winterbaume-athena",
            ),
            "CreateNotebook" => json_error_response(
                501,
                "NotImplementedError",
                "CreateNotebook is not yet implemented in winterbaume-athena",
            ),
            "CreatePresignedNotebookUrl" => json_error_response(
                501,
                "NotImplementedError",
                "CreatePresignedNotebookUrl is not yet implemented in winterbaume-athena",
            ),
            "DeleteCapacityReservation" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteCapacityReservation is not yet implemented in winterbaume-athena",
            ),
            "DeleteDataCatalog" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteDataCatalog is not yet implemented in winterbaume-athena",
            ),
            "DeleteNamedQuery" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteNamedQuery is not yet implemented in winterbaume-athena",
            ),
            "DeleteNotebook" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteNotebook is not yet implemented in winterbaume-athena",
            ),
            "DeletePreparedStatement" => json_error_response(
                501,
                "NotImplementedError",
                "DeletePreparedStatement is not yet implemented in winterbaume-athena",
            ),
            "ExportNotebook" => json_error_response(
                501,
                "NotImplementedError",
                "ExportNotebook is not yet implemented in winterbaume-athena",
            ),
            "GetCalculationExecution" => json_error_response(
                501,
                "NotImplementedError",
                "GetCalculationExecution is not yet implemented in winterbaume-athena",
            ),
            "GetCalculationExecutionCode" => json_error_response(
                501,
                "NotImplementedError",
                "GetCalculationExecutionCode is not yet implemented in winterbaume-athena",
            ),
            "GetCalculationExecutionStatus" => json_error_response(
                501,
                "NotImplementedError",
                "GetCalculationExecutionStatus is not yet implemented in winterbaume-athena",
            ),
            "GetCapacityAssignmentConfiguration" => json_error_response(
                501,
                "NotImplementedError",
                "GetCapacityAssignmentConfiguration is not yet implemented in winterbaume-athena",
            ),
            "GetDatabase" => json_error_response(
                501,
                "NotImplementedError",
                "GetDatabase is not yet implemented in winterbaume-athena",
            ),
            "GetNotebookMetadata" => json_error_response(
                501,
                "NotImplementedError",
                "GetNotebookMetadata is not yet implemented in winterbaume-athena",
            ),
            "GetResourceDashboard" => json_error_response(
                501,
                "NotImplementedError",
                "GetResourceDashboard is not yet implemented in winterbaume-athena",
            ),
            "GetSession" => json_error_response(
                501,
                "NotImplementedError",
                "GetSession is not yet implemented in winterbaume-athena",
            ),
            "GetSessionEndpoint" => json_error_response(
                501,
                "NotImplementedError",
                "GetSessionEndpoint is not yet implemented in winterbaume-athena",
            ),
            "GetSessionStatus" => json_error_response(
                501,
                "NotImplementedError",
                "GetSessionStatus is not yet implemented in winterbaume-athena",
            ),
            "GetTableMetadata" => json_error_response(
                501,
                "NotImplementedError",
                "GetTableMetadata is not yet implemented in winterbaume-athena",
            ),
            "ImportNotebook" => json_error_response(
                501,
                "NotImplementedError",
                "ImportNotebook is not yet implemented in winterbaume-athena",
            ),
            "ListApplicationDPUSizes" => json_error_response(
                501,
                "NotImplementedError",
                "ListApplicationDPUSizes is not yet implemented in winterbaume-athena",
            ),
            "ListCalculationExecutions" => json_error_response(
                501,
                "NotImplementedError",
                "ListCalculationExecutions is not yet implemented in winterbaume-athena",
            ),
            "ListDatabases" => json_error_response(
                501,
                "NotImplementedError",
                "ListDatabases is not yet implemented in winterbaume-athena",
            ),
            "ListEngineVersions" => json_error_response(
                501,
                "NotImplementedError",
                "ListEngineVersions is not yet implemented in winterbaume-athena",
            ),
            "ListExecutors" => json_error_response(
                501,
                "NotImplementedError",
                "ListExecutors is not yet implemented in winterbaume-athena",
            ),
            "ListNotebookMetadata" => json_error_response(
                501,
                "NotImplementedError",
                "ListNotebookMetadata is not yet implemented in winterbaume-athena",
            ),
            "ListNotebookSessions" => json_error_response(
                501,
                "NotImplementedError",
                "ListNotebookSessions is not yet implemented in winterbaume-athena",
            ),
            "ListPreparedStatements" => json_error_response(
                501,
                "NotImplementedError",
                "ListPreparedStatements is not yet implemented in winterbaume-athena",
            ),
            "ListSessions" => json_error_response(
                501,
                "NotImplementedError",
                "ListSessions is not yet implemented in winterbaume-athena",
            ),
            "ListTableMetadata" => json_error_response(
                501,
                "NotImplementedError",
                "ListTableMetadata is not yet implemented in winterbaume-athena",
            ),
            "PutCapacityAssignmentConfiguration" => json_error_response(
                501,
                "NotImplementedError",
                "PutCapacityAssignmentConfiguration is not yet implemented in winterbaume-athena",
            ),
            "StartCalculationExecution" => json_error_response(
                501,
                "NotImplementedError",
                "StartCalculationExecution is not yet implemented in winterbaume-athena",
            ),
            "StartSession" => json_error_response(
                501,
                "NotImplementedError",
                "StartSession is not yet implemented in winterbaume-athena",
            ),
            "StopCalculationExecution" => json_error_response(
                501,
                "NotImplementedError",
                "StopCalculationExecution is not yet implemented in winterbaume-athena",
            ),
            "TerminateSession" => json_error_response(
                501,
                "NotImplementedError",
                "TerminateSession is not yet implemented in winterbaume-athena",
            ),
            "UpdateDataCatalog" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateDataCatalog is not yet implemented in winterbaume-athena",
            ),
            "UpdateNamedQuery" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateNamedQuery is not yet implemented in winterbaume-athena",
            ),
            "UpdateNotebook" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateNotebook is not yet implemented in winterbaume-athena",
            ),
            "UpdateNotebookMetadata" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateNotebookMetadata is not yet implemented in winterbaume-athena",
            ),
            "UpdatePreparedStatement" => json_error_response(
                501,
                "NotImplementedError",
                "UpdatePreparedStatement is not yet implemented in winterbaume-athena",
            ),
            "UpdateWorkGroup" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateWorkGroup is not yet implemented in winterbaume-athena",
            ),
            _ => json_error_response(400, "InvalidAction", &format!("Unknown operation {action}")),
        };

        if response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    // ---- WorkGroup handlers ----

    async fn handle_create_work_group(
        &self,
        state: &Arc<tokio::sync::RwLock<AthenaState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_create_work_group_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidRequestException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "InvalidRequestException", "Name is required");
        }
        let description = input.description.as_deref().unwrap_or("");
        let (output_location, enforce) = match input.configuration.as_ref() {
            Some(cfg) => {
                let output = cfg
                    .result_configuration
                    .as_ref()
                    .and_then(|rc| rc.output_location.as_deref())
                    .unwrap_or("");
                let enforce = cfg.enforce_work_group_configuration.unwrap_or(true);
                (output.to_string(), enforce)
            }
            None => (String::new(), true),
        };

        let mut state = state.write().await;
        match state.create_work_group(&input.name, description, &output_location, enforce) {
            Ok(()) => wire::serialize_create_work_group_response(&wire::CreateWorkGroupOutput {}),
            Err(e) => athena_error_response(&e),
        }
    }

    async fn handle_get_work_group(
        &self,
        state: &Arc<tokio::sync::RwLock<AthenaState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_work_group_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidRequestException", &e),
        };
        if input.work_group.is_empty() {
            return json_error_response(400, "InvalidRequestException", "WorkGroup is required");
        }

        let state = state.read().await;
        match state.get_work_group(&input.work_group) {
            Ok(wg) => wire::serialize_get_work_group_response(&wire::GetWorkGroupOutput {
                work_group: Some(work_group_to_wire(wg)),
            }),
            Err(e) => athena_error_response(&e),
        }
    }

    async fn handle_delete_work_group(
        &self,
        state: &Arc<tokio::sync::RwLock<AthenaState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_work_group_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidRequestException", &e),
        };
        if input.work_group.is_empty() {
            return json_error_response(400, "InvalidRequestException", "WorkGroup is required");
        }

        let mut state = state.write().await;
        match state.delete_work_group(&input.work_group) {
            Ok(()) => wire::serialize_delete_work_group_response(&wire::DeleteWorkGroupOutput {}),
            Err(e) => athena_error_response(&e),
        }
    }

    async fn handle_list_work_groups(
        &self,
        state: &Arc<tokio::sync::RwLock<AthenaState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let wgs = state.list_work_groups();

        wire::serialize_list_work_groups_response(&wire::ListWorkGroupsOutput {
            next_token: None,
            work_groups: Some(
                wgs.iter()
                    .map(|wg| wire::WorkGroupSummary {
                        name: Some(wg.name.clone()),
                        state: Some(wg.state.clone()),
                        description: Some(wg.description.clone()),
                        creation_time: Some(wg.creation_time.timestamp() as f64),
                        ..Default::default()
                    })
                    .collect(),
            ),
        })
    }

    // ---- CapacityReservation handlers ----

    async fn handle_create_capacity_reservation(
        &self,
        state: &Arc<tokio::sync::RwLock<AthenaState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_create_capacity_reservation_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidRequestException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "InvalidRequestException", "Name is required");
        }
        let tags = wire_tags_to_map(&input.tags);

        let mut state = state.write().await;
        match state.create_capacity_reservation(&input.name, input.target_dpus, tags) {
            Ok(()) => wire::serialize_create_capacity_reservation_response(
                &wire::CreateCapacityReservationOutput {},
            ),
            Err(e) => athena_error_response(&e),
        }
    }

    async fn handle_get_capacity_reservation(
        &self,
        state: &Arc<tokio::sync::RwLock<AthenaState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_capacity_reservation_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidRequestException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "InvalidRequestException", "Name is required");
        }

        let state = state.read().await;
        match state.get_capacity_reservation(&input.name) {
            Ok(cr) => wire::serialize_get_capacity_reservation_response(
                &wire::GetCapacityReservationOutput {
                    capacity_reservation: Some(wire::CapacityReservation {
                        name: Some(cr.name.clone()),
                        target_dpus: Some(cr.target_dpus),
                        allocated_dpus: Some(cr.allocated_dpus),
                        status: Some(cr.status.clone()),
                        creation_time: Some(cr.creation_time.timestamp() as f64),
                        ..Default::default()
                    }),
                },
            ),
            Err(e) => athena_error_response(&e),
        }
    }

    async fn handle_list_capacity_reservations(
        &self,
        state: &Arc<tokio::sync::RwLock<AthenaState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let crs = state.list_capacity_reservations();

        wire::serialize_list_capacity_reservations_response(&wire::ListCapacityReservationsOutput {
            next_token: None,
            capacity_reservations: Some(
                crs.iter()
                    .map(|cr| wire::CapacityReservation {
                        name: Some(cr.name.clone()),
                        target_dpus: Some(cr.target_dpus),
                        allocated_dpus: Some(cr.allocated_dpus),
                        status: Some(cr.status.clone()),
                        creation_time: Some(cr.creation_time.timestamp() as f64),
                        ..Default::default()
                    })
                    .collect(),
            ),
        })
    }

    async fn handle_update_capacity_reservation(
        &self,
        state: &Arc<tokio::sync::RwLock<AthenaState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_capacity_reservation_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidRequestException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "InvalidRequestException", "Name is required");
        }

        let mut state = state.write().await;
        match state.update_capacity_reservation(&input.name, input.target_dpus) {
            Ok(()) => wire::serialize_update_capacity_reservation_response(
                &wire::UpdateCapacityReservationOutput {},
            ),
            Err(e) => athena_error_response(&e),
        }
    }

    // ---- DataCatalog handlers ----

    async fn handle_create_data_catalog(
        &self,
        state: &Arc<tokio::sync::RwLock<AthenaState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_data_catalog_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidRequestException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "InvalidRequestException", "Name is required");
        }
        if input.r#type.is_empty() {
            return json_error_response(400, "InvalidRequestException", "Type is required");
        }
        let description = input.description.as_deref().unwrap_or("");
        let parameters = input.parameters.clone().unwrap_or_default();
        let tags = wire_tags_to_map(&input.tags);

        let mut state = state.write().await;
        match state.create_data_catalog(
            &input.name,
            &input.r#type,
            description,
            parameters,
            tags,
            account_id,
            region,
        ) {
            Ok(dc) => {
                wire::serialize_create_data_catalog_response(&wire::CreateDataCatalogOutput {
                    data_catalog: Some(wire::DataCatalog {
                        name: Some(dc.name.clone()),
                        r#type: Some(dc.catalog_type.clone()),
                        description: Some(dc.description.clone()),
                        parameters: if dc.parameters.is_empty() {
                            None
                        } else {
                            Some(dc.parameters.clone())
                        },
                        ..Default::default()
                    }),
                })
            }
            Err(e) => athena_error_response(&e),
        }
    }

    async fn handle_get_data_catalog(
        &self,
        state: &Arc<tokio::sync::RwLock<AthenaState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_data_catalog_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidRequestException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "InvalidRequestException", "Name is required");
        }

        let state = state.read().await;
        match state.get_data_catalog(&input.name) {
            Ok(dc) => wire::serialize_get_data_catalog_response(&wire::GetDataCatalogOutput {
                data_catalog: Some(wire::DataCatalog {
                    name: Some(dc.name.clone()),
                    r#type: Some(dc.catalog_type.clone()),
                    description: Some(dc.description.clone()),
                    parameters: if dc.parameters.is_empty() {
                        None
                    } else {
                        Some(dc.parameters.clone())
                    },
                    ..Default::default()
                }),
            }),
            Err(e) => athena_error_response(&e),
        }
    }

    async fn handle_list_data_catalogs(
        &self,
        state: &Arc<tokio::sync::RwLock<AthenaState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let dcs = state.list_data_catalogs();

        wire::serialize_list_data_catalogs_response(&wire::ListDataCatalogsOutput {
            next_token: None,
            data_catalogs_summary: Some(
                dcs.iter()
                    .map(|dc| wire::DataCatalogSummary {
                        catalog_name: Some(dc.name.clone()),
                        r#type: Some(dc.catalog_type.clone()),
                        ..Default::default()
                    })
                    .collect(),
            ),
        })
    }

    // ---- NamedQuery handlers ----

    async fn handle_create_named_query(
        &self,
        state: &Arc<tokio::sync::RwLock<AthenaState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_create_named_query_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidRequestException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "InvalidRequestException", "Name is required");
        }
        if input.database.is_empty() {
            return json_error_response(400, "InvalidRequestException", "Database is required");
        }
        if input.query_string.is_empty() {
            return json_error_response(400, "InvalidRequestException", "QueryString is required");
        }
        let description = input.description.as_deref().unwrap_or("");
        let work_group = input.work_group.as_deref().unwrap_or("primary");

        let mut state = state.write().await;
        let id = state.create_named_query(
            &input.name,
            description,
            &input.database,
            &input.query_string,
            work_group,
        );

        wire::serialize_create_named_query_response(&wire::CreateNamedQueryOutput {
            named_query_id: Some(id),
        })
    }

    async fn handle_get_named_query(
        &self,
        state: &Arc<tokio::sync::RwLock<AthenaState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_named_query_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidRequestException", &e),
        };
        if input.named_query_id.is_empty() {
            return json_error_response(400, "InvalidRequestException", "NamedQueryId is required");
        }

        let state = state.read().await;
        match state.get_named_query(&input.named_query_id) {
            Ok(nq) => wire::serialize_get_named_query_response(&wire::GetNamedQueryOutput {
                named_query: Some(wire::NamedQuery {
                    named_query_id: Some(nq.id.clone()),
                    name: Some(nq.name.clone()),
                    description: Some(nq.description.clone()),
                    database: Some(nq.database.clone()),
                    query_string: Some(nq.query_string.clone()),
                    work_group: Some(nq.work_group.clone()),
                }),
            }),
            Err(e) => athena_error_response(&e),
        }
    }

    async fn handle_list_named_queries(
        &self,
        state: &Arc<tokio::sync::RwLock<AthenaState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let ids = state.list_named_queries();

        wire::serialize_list_named_queries_response(&wire::ListNamedQueriesOutput {
            named_query_ids: Some(ids),
            next_token: None,
        })
    }

    // ---- PreparedStatement handlers ----

    async fn handle_create_prepared_statement(
        &self,
        state: &Arc<tokio::sync::RwLock<AthenaState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_create_prepared_statement_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidRequestException", &e),
        };
        if input.statement_name.is_empty() {
            return json_error_response(
                400,
                "InvalidRequestException",
                "StatementName is required",
            );
        }
        if input.work_group.is_empty() {
            return json_error_response(400, "InvalidRequestException", "WorkGroup is required");
        }
        if input.query_statement.is_empty() {
            return json_error_response(
                400,
                "InvalidRequestException",
                "QueryStatement is required",
            );
        }
        let description = input.description.as_deref().unwrap_or("");

        let mut state = state.write().await;
        match state.create_prepared_statement(
            &input.statement_name,
            &input.work_group,
            &input.query_statement,
            description,
        ) {
            Ok(()) => wire::serialize_create_prepared_statement_response(
                &wire::CreatePreparedStatementOutput {},
            ),
            Err(e) => athena_error_response(&e),
        }
    }

    async fn handle_get_prepared_statement(
        &self,
        state: &Arc<tokio::sync::RwLock<AthenaState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_prepared_statement_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidRequestException", &e),
        };
        if input.statement_name.is_empty() {
            return json_error_response(
                400,
                "InvalidRequestException",
                "StatementName is required",
            );
        }
        if input.work_group.is_empty() {
            return json_error_response(400, "InvalidRequestException", "WorkGroup is required");
        }

        let state = state.read().await;
        match state.get_prepared_statement(&input.statement_name, &input.work_group) {
            Ok(ps) => {
                wire::serialize_get_prepared_statement_response(&wire::GetPreparedStatementOutput {
                    prepared_statement: Some(wire::PreparedStatement {
                        statement_name: Some(ps.statement_name.clone()),
                        work_group_name: Some(ps.work_group_name.clone()),
                        query_statement: Some(ps.query_statement.clone()),
                        description: Some(ps.description.clone()),
                        last_modified_time: Some(ps.last_modified_time.timestamp() as f64),
                    }),
                })
            }
            Err(e) => athena_error_response(&e),
        }
    }

    // ---- QueryExecution handlers ----

    async fn handle_start_query_execution(
        &self,
        state: &Arc<tokio::sync::RwLock<AthenaState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_start_query_execution_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidRequestException", &e),
        };
        if input.query_string.is_empty() {
            return json_error_response(400, "InvalidRequestException", "QueryString is required");
        }
        let work_group = input.work_group.as_deref().unwrap_or("primary");
        let database = input
            .query_execution_context
            .as_ref()
            .and_then(|qec| qec.database.as_deref());
        let catalog = input
            .query_execution_context
            .as_ref()
            .and_then(|qec| qec.catalog.as_deref());
        let output_location = input
            .result_configuration
            .as_ref()
            .and_then(|rc| rc.output_location.as_deref());

        let result = self
            .query_backend
            .execute_query(input.query_string.clone())
            .await;

        let mut state = state.write().await;
        let id = state.start_query_execution(
            &input.query_string,
            work_group,
            database,
            catalog,
            output_location,
            result,
        );

        wire::serialize_start_query_execution_response(&wire::StartQueryExecutionOutput {
            query_execution_id: Some(id),
        })
    }

    async fn handle_get_query_execution(
        &self,
        state: &Arc<tokio::sync::RwLock<AthenaState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_query_execution_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidRequestException", &e),
        };
        if input.query_execution_id.is_empty() {
            return json_error_response(
                400,
                "InvalidRequestException",
                "QueryExecutionId is required",
            );
        }

        let state = state.read().await;
        match state.get_query_execution(&input.query_execution_id) {
            Ok(qe) => {
                wire::serialize_get_query_execution_response(&wire::GetQueryExecutionOutput {
                    query_execution: Some(query_execution_to_wire(qe)),
                })
            }
            Err(e) => athena_error_response(&e),
        }
    }

    async fn handle_get_query_results(
        &self,
        state: &Arc<tokio::sync::RwLock<AthenaState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_query_results_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidRequestException", &e),
        };
        if input.query_execution_id.is_empty() {
            return json_error_response(
                400,
                "InvalidRequestException",
                "QueryExecutionId is required",
            );
        }

        let state = state.read().await;
        match state.get_query_execution(&input.query_execution_id) {
            Ok(qe) => {
                let column_info: Vec<wire::ColumnInfo> = qe
                    .result_columns
                    .iter()
                    .map(|(name, type_str)| wire::ColumnInfo {
                        name: Some(name.clone()),
                        r#type: Some(type_str.clone()),
                        ..Default::default()
                    })
                    .collect();

                // Athena convention: first row contains column name headers.
                let header_row = wire::Row {
                    data: Some(
                        qe.result_columns
                            .iter()
                            .map(|(name, _)| wire::Datum {
                                var_char_value: Some(name.clone()),
                            })
                            .collect(),
                    ),
                };

                let data_rows: Vec<wire::Row> = qe
                    .result_rows
                    .iter()
                    .map(|row| wire::Row {
                        data: Some(
                            row.iter()
                                .map(|cell| wire::Datum {
                                    var_char_value: cell.clone(),
                                })
                                .collect(),
                        ),
                    })
                    .collect();

                let mut rows = vec![header_row];
                rows.extend(data_rows);

                wire::serialize_get_query_results_response(&wire::GetQueryResultsOutput {
                    result_set: Some(wire::ResultSet {
                        result_set_metadata: Some(wire::ResultSetMetadata {
                            column_info: Some(column_info),
                        }),
                        rows: Some(rows),
                    }),
                    next_token: None,
                    update_count: None,
                })
            }
            Err(e) => athena_error_response(&e),
        }
    }

    async fn handle_get_query_runtime_statistics(
        &self,
        state: &Arc<tokio::sync::RwLock<AthenaState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_query_runtime_statistics_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidRequestException", &e),
        };
        if input.query_execution_id.is_empty() {
            return json_error_response(
                400,
                "InvalidRequestException",
                "QueryExecutionId is required",
            );
        }

        let state = state.read().await;
        match state.get_query_execution(&input.query_execution_id) {
            Ok(_qe) => wire::serialize_get_query_runtime_statistics_response(
                &wire::GetQueryRuntimeStatisticsOutput {
                    query_runtime_statistics: Some(wire::QueryRuntimeStatistics {
                        timeline: Some(wire::QueryRuntimeStatisticsTimeline {
                            engine_execution_time_in_millis: Some(100),
                            total_execution_time_in_millis: Some(200),
                            ..Default::default()
                        }),
                        rows: Some(wire::QueryRuntimeStatisticsRows {
                            input_rows: Some(0),
                            output_rows: Some(0),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                },
            ),
            Err(e) => athena_error_response(&e),
        }
    }

    async fn handle_stop_query_execution(
        &self,
        state: &Arc<tokio::sync::RwLock<AthenaState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_stop_query_execution_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidRequestException", &e),
        };
        if input.query_execution_id.is_empty() {
            return json_error_response(
                400,
                "InvalidRequestException",
                "QueryExecutionId is required",
            );
        }

        let mut state = state.write().await;
        match state.stop_query_execution(&input.query_execution_id) {
            Ok(()) => {
                wire::serialize_stop_query_execution_response(&wire::StopQueryExecutionOutput {})
            }
            Err(e) => athena_error_response(&e),
        }
    }

    async fn handle_list_query_executions(
        &self,
        state: &Arc<tokio::sync::RwLock<AthenaState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let ids = state.list_query_executions();

        wire::serialize_list_query_executions_response(&wire::ListQueryExecutionsOutput {
            query_execution_ids: Some(ids),
            next_token: None,
        })
    }

    // ---- Tags handler ----

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<AthenaState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidRequestException", &e),
        };
        if input.resource_a_r_n.is_empty() {
            return json_error_response(400, "InvalidRequestException", "ResourceARN is required");
        }

        let state = state.read().await;

        // Validate the resource exists by checking if the ARN matches a known resource
        if !state.resource_exists(&input.resource_a_r_n) {
            return json_error_response(
                400,
                "InvalidRequestException",
                &format!("Athena Resource, {} Does Not Exist", input.resource_a_r_n),
            );
        }

        let tags_map = state.list_tags_for_resource(&input.resource_a_r_n);

        wire::serialize_list_tags_for_resource_response(&wire::ListTagsForResourceOutput {
            next_token: None,
            tags: Some(
                tags_map
                    .iter()
                    .map(|(k, v)| wire::Tag {
                        key: Some(k.clone()),
                        value: Some(v.clone()),
                    })
                    .collect(),
            ),
        })
    }
    // ---- Tag/Untag handlers ----

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<AthenaState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidRequestException", &e),
        };
        if input.resource_a_r_n.is_empty() {
            return json_error_response(400, "InvalidRequestException", "ResourceARN is required");
        }
        let tags = wire_tags_to_map(&Some(input.tags));

        let mut state = state.write().await;
        match state.tag_resource(&input.resource_a_r_n, tags) {
            Ok(()) => wire::serialize_tag_resource_response(&wire::TagResourceOutput {}),
            Err(e) => athena_error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<AthenaState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidRequestException", &e),
        };
        if input.resource_a_r_n.is_empty() {
            return json_error_response(400, "InvalidRequestException", "ResourceARN is required");
        }

        let mut state = state.write().await;
        match state.untag_resource(&input.resource_a_r_n, &input.tag_keys) {
            Ok(()) => wire::serialize_untag_resource_response(&wire::UntagResourceOutput {}),
            Err(e) => athena_error_response(&e),
        }
    }
}

/// Convert a state WorkGroup to a wire model WorkGroup.
fn work_group_to_wire(wg: &crate::types::WorkGroup) -> wire::WorkGroup {
    wire::WorkGroup {
        name: Some(wg.name.clone()),
        state: Some(wg.state.clone()),
        description: Some(wg.description.clone()),
        creation_time: Some(wg.creation_time.timestamp() as f64),
        configuration: Some(wire::WorkGroupConfiguration {
            result_configuration: Some(wire::ResultConfiguration {
                output_location: Some(wg.output_location.clone()),
                ..Default::default()
            }),
            enforce_work_group_configuration: Some(wg.enforce_work_group_configuration),
            ..Default::default()
        }),
        ..Default::default()
    }
}

fn query_execution_to_wire(qe: &crate::types::QueryExecutionData) -> wire::QueryExecution {
    wire::QueryExecution {
        query_execution_id: Some(qe.query_execution_id.clone()),
        query: Some(qe.query.clone()),
        work_group: Some(qe.work_group.clone()),
        query_execution_context: Some(wire::QueryExecutionContext {
            database: qe.database.clone(),
            catalog: qe.catalog.clone(),
        }),
        result_configuration: qe
            .output_location
            .as_ref()
            .map(|loc| wire::ResultConfiguration {
                output_location: Some(loc.clone()),
                ..Default::default()
            }),
        status: Some(wire::QueryExecutionStatus {
            state: Some(qe.status.clone()),
            submission_date_time: Some(qe.submission_time.timestamp() as f64),
            completion_date_time: qe.completion_time.map(|t| t.timestamp() as f64),
            // Surface backend-side failure detail when the execution failed.
            // Real AWS Athena populates StateChangeReason with a one-line
            // summary and AthenaError with structured ErrorCategory/Type/
            // Message/Retryable; DuckDB rejections most naturally map to the
            // "user" error category ( 2 ) since they are query-shape mistakes
            // rather than service or infrastructure failures.
            state_change_reason: qe.error_message.clone(),
            athena_error: qe.error_message.as_ref().map(|msg| wire::AthenaError {
                error_category: Some(2),
                error_type: Some(0),
                error_message: Some(msg.clone()),
                retryable: Some(false),
            }),
            ..Default::default()
        }),
        ..Default::default()
    }
}

fn athena_error_response(err: &AthenaError) -> MockResponse {
    let (status, error_type) = match err {
        AthenaError::WorkGroupAlreadyExists(_)
        | AthenaError::WorkGroupNotFound(_)
        | AthenaError::PrimaryWorkGroupCannotBeDeleted
        | AthenaError::CapacityReservationAlreadyExists(_)
        | AthenaError::CapacityReservationNotFound(_)
        | AthenaError::DataCatalogAlreadyExists(_)
        | AthenaError::DataCatalogNotFound(_)
        | AthenaError::NamedQueryNotFound(_)
        | AthenaError::PreparedStatementAlreadyExists { .. }
        | AthenaError::QueryExecutionNotFound(_) => (400, "InvalidRequestException"),
        AthenaError::PreparedStatementNotFound { .. } => (400, "ResourceNotFoundException"),
        AthenaError::ResourceNotFound(_) => (400, "InvalidRequestException"),
    };
    MockResponse::json(
        status,
        json!({"__type": error_type, "message": err.to_string()}).to_string(),
    )
}

fn wire_tags_to_map(tags: &Option<Vec<wire::Tag>>) -> HashMap<String, String> {
    let mut out = HashMap::new();
    if let Some(list) = tags {
        for tag in list {
            if let (Some(k), Some(v)) = (&tag.key, &tag.value) {
                out.insert(k.clone(), v.clone());
            }
        }
    }
    out
}
