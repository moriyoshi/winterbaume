use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::json;
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, default_account_id,
    extract_path, extract_query_string, parse_query_string, percent_decode, rest_json_error,
};

use crate::state::{ArcZonalShiftError, ArcZonalShiftState};
use crate::types::{ControlCondition, ManagedResource, PracticeRunConfiguration, ZonalShift};
use crate::views::ArcZonalShiftStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct ArcZonalShiftService {
    pub(crate) state: Arc<BackendState<ArcZonalShiftState>>,
    pub(crate) notifier: StateChangeNotifier<ArcZonalShiftStateView>,
}

impl ArcZonalShiftService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for ArcZonalShiftService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for ArcZonalShiftService {
    fn service_name(&self) -> &str {
        "arc-zonal-shift"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://arc-zonal-shift\..*\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl ArcZonalShiftService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let raw_query = extract_query_string(&request.uri);
        let query_map: HashMap<String, String> = parse_query_string(raw_query);
        let method = request.method.as_str().to_uppercase();

        let segments: Vec<&str> = path
            .trim_start_matches('/')
            .split('/')
            .filter(|s| !s.is_empty())
            .collect();

        let response = match (method.as_str(), segments.as_slice()) {
            ("POST", ["zonalshifts"]) => {
                self.handle_start_zonal_shift(&state, &request, &[], &query_map)
                    .await
            }
            ("GET", ["zonalshifts"]) => {
                self.handle_list_zonal_shifts(&state, &request, &[], &query_map)
                    .await
            }
            ("PATCH", ["zonalshifts", id]) => {
                let id_decoded = percent_decode(id);
                let labels: &[(&str, &str)] = &[("zonalShiftId", id_decoded.as_str())];
                self.handle_update_zonal_shift(&state, &request, labels, &query_map)
                    .await
            }
            ("DELETE", ["zonalshifts", id]) => {
                let id_decoded = percent_decode(id);
                let labels: &[(&str, &str)] = &[("zonalShiftId", id_decoded.as_str())];
                self.handle_cancel_zonal_shift(&state, &request, labels, &query_map)
                    .await
            }
            ("POST", ["configuration"]) => {
                self.handle_create_practice_run_configuration(&state, &request, &[], &query_map)
                    .await
            }
            ("PATCH", ["configuration", ident]) => {
                let id_decoded = percent_decode(ident);
                let labels: &[(&str, &str)] = &[("resourceIdentifier", id_decoded.as_str())];
                self.handle_update_practice_run_configuration(&state, &request, labels, &query_map)
                    .await
            }
            ("DELETE", ["configuration", ident]) => {
                let id_decoded = percent_decode(ident);
                let labels: &[(&str, &str)] = &[("resourceIdentifier", id_decoded.as_str())];
                self.handle_delete_practice_run_configuration(&state, &request, labels, &query_map)
                    .await
            }
            ("GET", ["managedresources"]) => {
                self.handle_list_managed_resources(&state, &request, &[], &query_map)
                    .await
            }
            ("GET", ["managedresources", ident]) => {
                let id_decoded = percent_decode(ident);
                let labels: &[(&str, &str)] = &[("resourceIdentifier", id_decoded.as_str())];
                self.handle_get_managed_resource(&state, &request, labels, &query_map)
                    .await
            }
            ("PUT", ["managedresources", ident]) => {
                let id_decoded = percent_decode(ident);
                let labels: &[(&str, &str)] = &[("resourceIdentifier", id_decoded.as_str())];
                self.handle_update_zonal_autoshift_configuration(
                    &state, &request, labels, &query_map,
                )
                .await
            }
            ("GET", ["autoshifts"]) => self.handle_list_autoshifts(&request, &[], &query_map).await,
            ("POST", ["practiceruns"]) => {
                self.handle_start_practice_run(&state, &request, &[], &query_map)
                    .await
            }
            ("DELETE", ["practiceruns", id]) => {
                let id_decoded = percent_decode(id);
                let labels: &[(&str, &str)] = &[("zonalShiftId", id_decoded.as_str())];
                self.handle_cancel_practice_run(&state, &request, labels, &query_map)
                    .await
            }
            ("GET", ["autoshift-observer-notification"]) => {
                self.handle_get_autoshift_observer_notification_status(
                    &state,
                    &request,
                    &[],
                    &query_map,
                )
                .await
            }
            ("PUT", ["autoshift-observer-notification"]) => {
                self.handle_update_autoshift_observer_notification_status(
                    &state,
                    &request,
                    &[],
                    &query_map,
                )
                .await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        };

        use winterbaume_core::StatefulService;
        if response.status / 100 == 2
            && matches!(method.as_str(), "POST" | "PATCH" | "DELETE" | "PUT")
        {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_start_zonal_shift(
        &self,
        state: &Arc<tokio::sync::RwLock<ArcZonalShiftState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_start_zonal_shift_request(request, labels, query) {
            Ok(v) => v,
            Err(_) => return rest_json_error(400, "BadRequestException", "Invalid JSON body"),
        };
        if input.resource_identifier.is_empty() {
            return rest_json_error(400, "ValidationException", "resourceIdentifier is required");
        }
        if input.away_from.is_empty() {
            return rest_json_error(400, "ValidationException", "awayFrom is required");
        }
        if input.expires_in.is_empty() {
            return rest_json_error(400, "ValidationException", "expiresIn is required");
        }
        if input.comment.is_empty() {
            return rest_json_error(400, "ValidationException", "comment is required");
        }

        let mut state = state.write().await;
        match state.start_zonal_shift(
            input.resource_identifier,
            input.away_from,
            input.expires_in,
            input.comment,
            "ZONAL_SHIFT",
        ) {
            Ok(s) => wire::serialize_start_zonal_shift_response(&zonal_shift_to_wire(&s)),
            Err(e) => arc_error_response(&e),
        }
    }

    async fn handle_cancel_zonal_shift(
        &self,
        state: &Arc<tokio::sync::RwLock<ArcZonalShiftState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_cancel_zonal_shift_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.cancel_zonal_shift(&input.zonal_shift_id) {
            Ok(s) => wire::serialize_cancel_zonal_shift_response(&zonal_shift_to_wire(&s)),
            Err(e) => arc_error_response(&e),
        }
    }

    async fn handle_update_zonal_shift(
        &self,
        state: &Arc<tokio::sync::RwLock<ArcZonalShiftState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_zonal_shift_request(request, labels, query) {
            Ok(v) => v,
            Err(_) => return rest_json_error(400, "BadRequestException", "Invalid JSON body"),
        };

        let mut state = state.write().await;
        match state.update_zonal_shift(&input.zonal_shift_id, input.comment, input.expires_in) {
            Ok(s) => wire::serialize_update_zonal_shift_response(&zonal_shift_to_wire(&s)),
            Err(e) => arc_error_response(&e),
        }
    }

    async fn handle_list_zonal_shifts(
        &self,
        state: &Arc<tokio::sync::RwLock<ArcZonalShiftState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_zonal_shifts_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };

        let state = state.read().await;
        let shifts = state.list_zonal_shifts(input.status.as_deref());
        let resource_id = input.resource_identifier.as_deref();
        let items: Vec<wire::ZonalShiftSummary> = shifts
            .into_iter()
            .filter(|s| resource_id.is_none_or(|r| s.resource_identifier == r))
            .map(zonal_shift_to_summary)
            .collect();

        wire::serialize_list_zonal_shifts_response(&wire::ListZonalShiftsResponse {
            items: if items.is_empty() { None } else { Some(items) },
            next_token: None,
        })
    }

    async fn handle_create_practice_run_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<ArcZonalShiftState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_practice_run_configuration_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(_) => return rest_json_error(400, "BadRequestException", "Invalid JSON body"),
        };
        if input.resource_identifier.is_empty() {
            return rest_json_error(400, "ValidationException", "resourceIdentifier is required");
        }
        if input.outcome_alarms.is_empty() {
            return rest_json_error(
                400,
                "ValidationException",
                "outcomeAlarms is required and must be non-empty",
            );
        }

        let resource_identifier = input.resource_identifier.clone();
        let config = PracticeRunConfiguration {
            resource_identifier: resource_identifier.clone(),
            blocking_alarms: control_conditions_from_wire(
                input.blocking_alarms.unwrap_or_default(),
            ),
            outcome_alarms: control_conditions_from_wire(input.outcome_alarms),
            blocked_windows: input.blocked_windows.unwrap_or_default(),
            allowed_windows: input.allowed_windows.unwrap_or_default(),
            blocked_dates: input.blocked_dates.unwrap_or_default(),
        };

        let mut state = state.write().await;
        match state.create_practice_run_configuration(config) {
            Ok(resource) => {
                let arn = resource.arn.clone();
                let name = resource.name.clone();
                let zonal_autoshift_status = resource.zonal_autoshift_status.clone();
                let practice_config = state
                    .practice_run_configurations
                    .get(&resource_identifier)
                    .cloned();
                let response = wire::CreatePracticeRunConfigurationResponse {
                    arn: Some(arn),
                    name: Some(name),
                    zonal_autoshift_status: Some(zonal_autoshift_status),
                    practice_run_configuration: practice_config.as_ref().map(practice_run_to_wire),
                };
                wire::serialize_create_practice_run_configuration_response(&response)
            }
            Err(e) => arc_error_response(&e),
        }
    }

    async fn handle_update_practice_run_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<ArcZonalShiftState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_practice_run_configuration_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(_) => return rest_json_error(400, "BadRequestException", "Invalid JSON body"),
        };

        let resource_identifier = input.resource_identifier.clone();
        let blocking_alarms = input.blocking_alarms.map(control_conditions_from_wire);
        let outcome_alarms = input.outcome_alarms.map(control_conditions_from_wire);
        let blocked_windows = input.blocked_windows;
        let allowed_windows = input.allowed_windows;
        let blocked_dates = input.blocked_dates;

        let mut state = state.write().await;
        let result = state.update_practice_run_configuration(&resource_identifier, |c| {
            if let Some(v) = blocking_alarms {
                c.blocking_alarms = v;
            }
            if let Some(v) = outcome_alarms {
                c.outcome_alarms = v;
            }
            if let Some(v) = blocked_windows {
                c.blocked_windows = v;
            }
            if let Some(v) = allowed_windows {
                c.allowed_windows = v;
            }
            if let Some(v) = blocked_dates {
                c.blocked_dates = v;
            }
        });
        match result {
            Ok(resource) => {
                let arn = resource.arn.clone();
                let name = resource.name.clone();
                let zonal_autoshift_status = resource.zonal_autoshift_status.clone();
                let practice_config = state
                    .practice_run_configurations
                    .get(&resource_identifier)
                    .cloned();
                let response = wire::UpdatePracticeRunConfigurationResponse {
                    arn: Some(arn),
                    name: Some(name),
                    zonal_autoshift_status: Some(zonal_autoshift_status),
                    practice_run_configuration: practice_config.as_ref().map(practice_run_to_wire),
                };
                wire::serialize_update_practice_run_configuration_response(&response)
            }
            Err(e) => arc_error_response(&e),
        }
    }

    async fn handle_delete_practice_run_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<ArcZonalShiftState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_practice_run_configuration_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.delete_practice_run_configuration(&input.resource_identifier) {
            Ok(resource) => {
                let response = wire::DeletePracticeRunConfigurationResponse {
                    arn: Some(resource.arn.clone()),
                    name: Some(resource.name.clone()),
                    zonal_autoshift_status: Some(resource.zonal_autoshift_status.clone()),
                };
                wire::serialize_delete_practice_run_configuration_response(&response)
            }
            Err(e) => arc_error_response(&e),
        }
    }

    async fn handle_get_managed_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<ArcZonalShiftState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_managed_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let resource_identifier = input.resource_identifier;
        let state = state.read().await;
        match state.get_managed_resource(&resource_identifier) {
            Ok(resource) => {
                let practice_config = state
                    .practice_run_configurations
                    .get(&resource_identifier)
                    .cloned();
                let zonal_shifts: Vec<wire::ZonalShiftInResource> = state
                    .zonal_shifts
                    .values()
                    .filter(|s| {
                        s.resource_identifier == resource_identifier && s.status == "ACTIVE"
                    })
                    .map(zonal_shift_to_in_resource)
                    .collect();
                let response = wire::GetManagedResourceResponse {
                    arn: Some(resource.arn.clone()),
                    name: Some(resource.name.clone()),
                    applied_weights: Some(resource.applied_weights.clone()),
                    autoshifts: Some(vec![]),
                    practice_run_configuration: practice_config.as_ref().map(practice_run_to_wire),
                    zonal_autoshift_status: Some(resource.zonal_autoshift_status.clone()),
                    zonal_shifts: Some(zonal_shifts),
                };
                wire::serialize_get_managed_resource_response(&response)
            }
            Err(e) => arc_error_response(&e),
        }
    }

    async fn handle_list_managed_resources(
        &self,
        state: &Arc<tokio::sync::RwLock<ArcZonalShiftState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_list_managed_resources_request(request, labels, query) {
            return rest_json_error(400, "ValidationException", &e);
        }
        let state = state.read().await;
        let items: Vec<wire::ManagedResourceSummary> = state
            .list_managed_resources()
            .into_iter()
            .map(|r| {
                let zonal_shifts: Vec<wire::ZonalShiftInResource> = state
                    .zonal_shifts
                    .values()
                    .filter(|s| {
                        s.resource_identifier == r.resource_identifier && s.status == "ACTIVE"
                    })
                    .map(zonal_shift_to_in_resource)
                    .collect();
                let practice_run_status = if state
                    .practice_run_configurations
                    .contains_key(&r.resource_identifier)
                {
                    "ENABLED"
                } else {
                    "DISABLED"
                };
                wire::ManagedResourceSummary {
                    arn: Some(r.arn.clone()),
                    name: Some(r.name.clone()),
                    availability_zones: Some(r.availability_zones.clone()),
                    applied_weights: Some(r.applied_weights.clone()),
                    autoshifts: Some(vec![]),
                    practice_run_status: Some(practice_run_status.to_string()),
                    zonal_autoshift_status: Some(r.zonal_autoshift_status.clone()),
                    zonal_shifts: Some(zonal_shifts),
                }
            })
            .collect();

        wire::serialize_list_managed_resources_response(&wire::ListManagedResourcesResponse {
            items: if items.is_empty() { None } else { Some(items) },
            next_token: None,
        })
    }

    async fn handle_update_zonal_autoshift_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<ArcZonalShiftState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_zonal_autoshift_configuration_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(_) => return rest_json_error(400, "BadRequestException", "Invalid JSON body"),
        };
        if input.zonal_autoshift_status.is_empty() {
            return rest_json_error(
                400,
                "ValidationException",
                "zonalAutoshiftStatus is required",
            );
        }
        if input.zonal_autoshift_status != "ENABLED" && input.zonal_autoshift_status != "DISABLED" {
            return rest_json_error(
                400,
                "ValidationException",
                "zonalAutoshiftStatus must be ENABLED or DISABLED",
            );
        }
        let mut state = state.write().await;
        match state.update_zonal_autoshift_configuration(
            &input.resource_identifier,
            &input.zonal_autoshift_status,
        ) {
            Ok(resource) => wire::serialize_update_zonal_autoshift_configuration_response(
                &wire::UpdateZonalAutoshiftConfigurationResponse {
                    resource_identifier: Some(resource.resource_identifier.clone()),
                    zonal_autoshift_status: Some(resource.zonal_autoshift_status.clone()),
                },
            ),
            Err(e) => arc_error_response(&e),
        }
    }

    // STUB[no-telemetry]: Autoshifts are AWS-initiated events driven by real infrastructure
    //   health telemetry; the mock cannot simulate them and always returns an empty list.
    async fn handle_list_autoshifts(
        &self,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_list_autoshifts_request(request, labels, query) {
            return rest_json_error(400, "ValidationException", &e);
        }
        wire::serialize_list_autoshifts_response(&wire::ListAutoshiftsResponse {
            items: Some(vec![]),
            next_token: None,
        })
    }

    async fn handle_start_practice_run(
        &self,
        state: &Arc<tokio::sync::RwLock<ArcZonalShiftState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_start_practice_run_request(request, labels, query) {
            Ok(v) => v,
            Err(_) => return rest_json_error(400, "BadRequestException", "Invalid JSON body"),
        };
        if input.resource_identifier.is_empty() {
            return rest_json_error(400, "ValidationException", "resourceIdentifier is required");
        }
        if input.away_from.is_empty() {
            return rest_json_error(400, "ValidationException", "awayFrom is required");
        }
        if input.comment.is_empty() {
            return rest_json_error(400, "ValidationException", "comment is required");
        }

        let mut state = state.write().await;
        match state.start_practice_run(input.resource_identifier, input.away_from, input.comment) {
            Ok(s) => {
                let response = wire::StartPracticeRunResponse {
                    away_from: Some(s.away_from.clone()),
                    comment: Some(s.comment.clone()),
                    expiry_time: Some(s.expiry_time as f64),
                    resource_identifier: Some(s.resource_identifier.clone()),
                    start_time: Some(s.start_time as f64),
                    status: Some(s.status.clone()),
                    zonal_shift_id: Some(s.zonal_shift_id.clone()),
                };
                wire::serialize_start_practice_run_response(&response)
            }
            Err(e) => arc_error_response(&e),
        }
    }

    async fn handle_cancel_practice_run(
        &self,
        state: &Arc<tokio::sync::RwLock<ArcZonalShiftState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_cancel_practice_run_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.cancel_practice_run(&input.zonal_shift_id) {
            Ok(s) => {
                let response = wire::CancelPracticeRunResponse {
                    away_from: Some(s.away_from.clone()),
                    comment: Some(s.comment.clone()),
                    expiry_time: Some(s.expiry_time as f64),
                    resource_identifier: Some(s.resource_identifier.clone()),
                    start_time: Some(s.start_time as f64),
                    status: Some(s.status.clone()),
                    zonal_shift_id: Some(s.zonal_shift_id.clone()),
                };
                wire::serialize_cancel_practice_run_response(&response)
            }
            Err(e) => arc_error_response(&e),
        }
    }

    async fn handle_get_autoshift_observer_notification_status(
        &self,
        state: &Arc<tokio::sync::RwLock<ArcZonalShiftState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_get_autoshift_observer_notification_status_request(
            request, labels, query,
        ) {
            return rest_json_error(400, "ValidationException", &e);
        }
        let state = state.read().await;
        wire::serialize_get_autoshift_observer_notification_status_response(
            &wire::GetAutoshiftObserverNotificationStatusResponse {
                status: Some(state.get_observer_notification_status().to_string()),
            },
        )
    }

    async fn handle_update_autoshift_observer_notification_status(
        &self,
        state: &Arc<tokio::sync::RwLock<ArcZonalShiftState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_autoshift_observer_notification_status_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(_) => return rest_json_error(400, "BadRequestException", "Invalid JSON body"),
        };
        if input.status.is_empty() {
            return rest_json_error(400, "ValidationException", "status is required");
        }
        if input.status != "ENABLED" && input.status != "DISABLED" {
            return rest_json_error(
                400,
                "ValidationException",
                "status must be ENABLED or DISABLED",
            );
        }
        let mut state = state.write().await;
        state.set_observer_notification_status(&input.status);
        wire::serialize_update_autoshift_observer_notification_status_response(
            &wire::UpdateAutoshiftObserverNotificationStatusResponse {
                status: Some(input.status),
            },
        )
    }
}

fn control_conditions_from_wire(items: Vec<wire::ControlCondition>) -> Vec<ControlCondition> {
    items
        .into_iter()
        .map(|c| ControlCondition {
            alarm_identifier: c.alarm_identifier,
            r#type: c.r#type,
        })
        .collect()
}

fn zonal_shift_to_wire(s: &ZonalShift) -> wire::ZonalShift {
    wire::ZonalShift {
        away_from: Some(s.away_from.clone()),
        comment: Some(s.comment.clone()),
        expiry_time: Some(s.expiry_time as f64),
        resource_identifier: Some(s.resource_identifier.clone()),
        start_time: Some(s.start_time as f64),
        status: Some(s.status.clone()),
        zonal_shift_id: Some(s.zonal_shift_id.clone()),
    }
}

fn zonal_shift_to_summary(s: &ZonalShift) -> wire::ZonalShiftSummary {
    wire::ZonalShiftSummary {
        away_from: Some(s.away_from.clone()),
        comment: Some(s.comment.clone()),
        expiry_time: Some(s.expiry_time as f64),
        resource_identifier: Some(s.resource_identifier.clone()),
        start_time: Some(s.start_time as f64),
        status: Some(s.status.clone()),
        shift_type: Some(s.shift_type.clone()),
        practice_run_outcome: s.practice_run_outcome.clone(),
        zonal_shift_id: Some(s.zonal_shift_id.clone()),
    }
}

fn zonal_shift_to_in_resource(s: &ZonalShift) -> wire::ZonalShiftInResource {
    wire::ZonalShiftInResource {
        applied_status: Some(if s.status == "ACTIVE" {
            "APPLIED".to_string()
        } else {
            "NOT_APPLIED".to_string()
        }),
        away_from: Some(s.away_from.clone()),
        comment: Some(s.comment.clone()),
        expiry_time: Some(s.expiry_time as f64),
        practice_run_outcome: s.practice_run_outcome.clone(),
        resource_identifier: Some(s.resource_identifier.clone()),
        shift_type: Some(s.shift_type.clone()),
        start_time: Some(s.start_time as f64),
        zonal_shift_id: Some(s.zonal_shift_id.clone()),
    }
}

fn practice_run_to_wire(c: &PracticeRunConfiguration) -> wire::PracticeRunConfiguration {
    wire::PracticeRunConfiguration {
        allowed_windows: if c.allowed_windows.is_empty() {
            None
        } else {
            Some(c.allowed_windows.clone())
        },
        blocked_dates: if c.blocked_dates.is_empty() {
            None
        } else {
            Some(c.blocked_dates.clone())
        },
        blocked_windows: if c.blocked_windows.is_empty() {
            None
        } else {
            Some(c.blocked_windows.clone())
        },
        blocking_alarms: if c.blocking_alarms.is_empty() {
            None
        } else {
            Some(
                c.blocking_alarms
                    .iter()
                    .map(|a| wire::ControlCondition {
                        alarm_identifier: a.alarm_identifier.clone(),
                        r#type: a.r#type.clone(),
                    })
                    .collect(),
            )
        },
        outcome_alarms: if c.outcome_alarms.is_empty() {
            None
        } else {
            Some(
                c.outcome_alarms
                    .iter()
                    .map(|a| wire::ControlCondition {
                        alarm_identifier: a.alarm_identifier.clone(),
                        r#type: a.r#type.clone(),
                    })
                    .collect(),
            )
        },
    }
}

#[allow(dead_code)]
fn managed_resource_to_summary(r: &ManagedResource) -> wire::ManagedResourceSummary {
    wire::ManagedResourceSummary {
        applied_weights: Some(r.applied_weights.clone()),
        arn: Some(r.arn.clone()),
        autoshifts: Some(vec![]),
        availability_zones: Some(r.availability_zones.clone()),
        name: Some(r.name.clone()),
        practice_run_status: None,
        zonal_autoshift_status: Some(r.zonal_autoshift_status.clone()),
        zonal_shifts: Some(vec![]),
    }
}

fn arc_error_response(err: &ArcZonalShiftError) -> MockResponse {
    let (status, error_type) = match err {
        ArcZonalShiftError::ZonalShiftNotFound { .. } => (404, "ResourceNotFoundException"),
        ArcZonalShiftError::PracticeRunConfigurationNotFound { .. } => {
            (404, "ResourceNotFoundException")
        }
        ArcZonalShiftError::ManagedResourceNotFound { .. } => (404, "ResourceNotFoundException"),
        ArcZonalShiftError::PracticeRunConfigurationAlreadyExists { .. } => {
            (409, "ConflictException")
        }
        ArcZonalShiftError::ActiveZonalShiftExists { .. } => (409, "ConflictException"),
        ArcZonalShiftError::ZonalShiftNotActive { .. } => (409, "ConflictException"),
        ArcZonalShiftError::AutoshiftRequiresPracticeRun => (409, "ConflictException"),
        ArcZonalShiftError::Validation { .. } => (400, "ValidationException"),
    };
    let body = json!({"message": err.to_string()});
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}
