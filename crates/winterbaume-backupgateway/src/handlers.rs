use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::Value;
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
    json_error_response,
};

use crate::state::{BackupGatewayError, BackupGatewayState};
use crate::types::{
    BandwidthRateLimitInterval, Gateway, Hypervisor, MaintenanceStartTime, TagMapping,
};
use crate::views::BackupGatewayStateView;
use crate::wire;

pub struct BackupGatewayService {
    pub(crate) state: Arc<BackendState<BackupGatewayState>>,
    pub(crate) notifier: StateChangeNotifier<BackupGatewayStateView>,
}

impl BackupGatewayService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for BackupGatewayService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for BackupGatewayService {
    fn service_name(&self) -> &str {
        "backup-gateway"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://backup-gateway\..*\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

type SharedState = Arc<tokio::sync::RwLock<BackupGatewayState>>;

const MUTATING_ACTIONS: &[&str] = &[
    "AssociateGatewayToServer",
    "CreateGateway",
    "DeleteGateway",
    "DeleteHypervisor",
    "DisassociateGatewayFromServer",
    "ImportHypervisorConfiguration",
    "PutBandwidthRateLimitSchedule",
    "PutHypervisorPropertyMappings",
    "PutMaintenanceStartTime",
    "StartVirtualMachinesMetadataSync",
    "TagResource",
    "UntagResource",
    "UpdateGatewayInformation",
    "UpdateGatewaySoftwareNow",
    "UpdateHypervisor",
];

impl BackupGatewayService {
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
            "AssociateGatewayToServer" => self.handle_associate_gateway(&state, body_bytes).await,
            "CreateGateway" => {
                self.handle_create_gateway(&state, account_id, &region, body_bytes)
                    .await
            }
            "DeleteGateway" => self.handle_delete_gateway(&state, body_bytes).await,
            "DeleteHypervisor" => self.handle_delete_hypervisor(&state, body_bytes).await,
            "DisassociateGatewayFromServer" => {
                self.handle_disassociate_gateway(&state, body_bytes).await
            }
            "GetBandwidthRateLimitSchedule" => {
                self.handle_get_bandwidth_rate_limit_schedule(&state, body_bytes)
                    .await
            }
            "GetGateway" => self.handle_get_gateway(&state, body_bytes).await,
            "GetHypervisor" => self.handle_get_hypervisor(&state, body_bytes).await,
            "GetHypervisorPropertyMappings" => {
                self.handle_get_hypervisor_property_mappings(&state, body_bytes)
                    .await
            }
            "GetVirtualMachine" => self.handle_get_virtual_machine(&state, body_bytes).await,
            "ImportHypervisorConfiguration" => {
                self.handle_import_hypervisor(&state, account_id, &region, body_bytes)
                    .await
            }
            "ListGateways" => self.handle_list_gateways(&state).await,
            "ListHypervisors" => self.handle_list_hypervisors(&state).await,
            "ListTagsForResource" => self.handle_list_tags(&state, body_bytes).await,
            "ListVirtualMachines" => self.handle_list_virtual_machines(&state, body_bytes).await,
            "PutBandwidthRateLimitSchedule" => {
                self.handle_put_bandwidth_rate_limit_schedule(&state, body_bytes)
                    .await
            }
            "PutHypervisorPropertyMappings" => {
                self.handle_put_hypervisor_property_mappings(&state, body_bytes)
                    .await
            }
            "PutMaintenanceStartTime" => {
                self.handle_put_maintenance_start_time(&state, body_bytes)
                    .await
            }
            "StartVirtualMachinesMetadataSync" => {
                self.handle_start_metadata_sync(&state, body_bytes).await
            }
            "TagResource" => self.handle_tag_resource(&state, body_bytes).await,
            "TestHypervisorConfiguration" => self.handle_test_hypervisor(&state, body_bytes).await,
            "UntagResource" => self.handle_untag_resource(&state, body_bytes).await,
            "UpdateGatewayInformation" => self.handle_update_gateway(&state, body_bytes).await,
            "UpdateGatewaySoftwareNow" => {
                self.handle_update_gateway_software(&state, body_bytes)
                    .await
            }
            "UpdateHypervisor" => self.handle_update_hypervisor(&state, body_bytes).await,
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

    async fn handle_create_gateway(
        &self,
        state: &SharedState,
        account_id: &str,
        region: &str,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_create_gateway_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.gateway_display_name.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "GatewayDisplayName is required",
            );
        }
        if input.gateway_type.is_empty() {
            return json_error_response(400, "ValidationException", "GatewayType is required");
        }
        let display_name = input.gateway_display_name;
        let gateway_type = input.gateway_type;
        let id = uuid::Uuid::new_v4().to_string();
        let arn = format!(
            "arn:aws:backup-gateway:{}:{}:gateway/{}",
            region, account_id, id
        );
        let gateway = Gateway {
            arn: arn.clone(),
            display_name,
            gateway_type,
            hypervisor_arn: None,
            last_seen_time: chrono::Utc::now().timestamp(),
            software_version: "1.0.0".to_string(),
            vpc_endpoint: None,
            maintenance_start_time: None,
            bandwidth_rate_limit_intervals: vec![],
            tags: wire_tags_to_map(input.tags.as_deref()),
        };
        let mut state = state.write().await;
        state.create_gateway(gateway);
        wire::serialize_create_gateway_response(&wire::CreateGatewayOutput {
            gateway_arn: Some(arn),
        })
    }

    async fn handle_delete_gateway(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_delete_gateway_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.gateway_arn.is_empty() {
            return json_error_response(400, "ValidationException", "GatewayArn is required");
        }
        let arn = input.gateway_arn;
        let mut state = state.write().await;
        match state.delete_gateway(&arn) {
            Ok(()) => wire::serialize_delete_gateway_response(&wire::DeleteGatewayOutput {
                gateway_arn: Some(arn),
            }),
            Err(e) => bg_error_response(&e),
        }
    }

    async fn handle_get_gateway(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_get_gateway_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.gateway_arn.is_empty() {
            return json_error_response(400, "ValidationException", "GatewayArn is required");
        }
        let arn = input.gateway_arn;
        let state = state.read().await;
        match state.get_gateway(&arn) {
            Ok(g) => wire::serialize_get_gateway_response(&wire::GetGatewayOutput {
                gateway: Some(gateway_to_details(g)),
            }),
            Err(e) => bg_error_response(&e),
        }
    }

    async fn handle_list_gateways(&self, state: &SharedState) -> MockResponse {
        let state = state.read().await;
        let items: Vec<wire::Gateway> = state
            .list_gateways()
            .into_iter()
            .map(|g| wire::Gateway {
                gateway_arn: Some(g.arn.clone()),
                gateway_display_name: Some(g.display_name.clone()),
                gateway_type: Some(g.gateway_type.clone()),
                hypervisor_id: g.hypervisor_arn.clone(),
                last_seen_time: Some(g.last_seen_time as f64),
            })
            .collect();
        wire::serialize_list_gateways_response(&wire::ListGatewaysOutput {
            gateways: Some(items),
            next_token: None,
        })
    }

    async fn handle_update_gateway(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_update_gateway_information_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.gateway_arn.is_empty() {
            return json_error_response(400, "ValidationException", "GatewayArn is required");
        }
        let arn = input.gateway_arn;
        let new_name = input.gateway_display_name;
        let mut state = state.write().await;
        match state.update_gateway(&arn, |g| {
            if let Some(n) = new_name {
                g.display_name = n;
            }
        }) {
            Ok(_) => wire::serialize_update_gateway_information_response(
                &wire::UpdateGatewayInformationOutput {
                    gateway_arn: Some(arn),
                },
            ),
            Err(e) => bg_error_response(&e),
        }
    }

    async fn handle_update_gateway_software(
        &self,
        state: &SharedState,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_gateway_software_now_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.gateway_arn.is_empty() {
            return json_error_response(400, "ValidationException", "GatewayArn is required");
        }
        let arn = input.gateway_arn;
        let state = state.read().await;
        if state.get_gateway(&arn).is_err() {
            return bg_error_response(&BackupGatewayError::GatewayNotFound { arn });
        }
        wire::serialize_update_gateway_software_now_response(
            &wire::UpdateGatewaySoftwareNowOutput {
                gateway_arn: Some(arn),
            },
        )
    }

    async fn handle_associate_gateway(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_associate_gateway_to_server_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.gateway_arn.is_empty() {
            return json_error_response(400, "ValidationException", "GatewayArn is required");
        }
        if input.server_arn.is_empty() {
            return json_error_response(400, "ValidationException", "ServerArn is required");
        }
        let arn = input.gateway_arn;
        let server_arn = input.server_arn;
        let mut state = state.write().await;
        match state.update_gateway(&arn, |g| {
            g.hypervisor_arn = Some(server_arn);
        }) {
            Ok(_) => wire::serialize_associate_gateway_to_server_response(
                &wire::AssociateGatewayToServerOutput {
                    gateway_arn: Some(arn),
                },
            ),
            Err(e) => bg_error_response(&e),
        }
    }

    async fn handle_disassociate_gateway(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_disassociate_gateway_from_server_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.gateway_arn.is_empty() {
            return json_error_response(400, "ValidationException", "GatewayArn is required");
        }
        let arn = input.gateway_arn;
        let mut state = state.write().await;
        match state.update_gateway(&arn, |g| {
            g.hypervisor_arn = None;
        }) {
            Ok(_) => wire::serialize_disassociate_gateway_from_server_response(
                &wire::DisassociateGatewayFromServerOutput {
                    gateway_arn: Some(arn),
                },
            ),
            Err(e) => bg_error_response(&e),
        }
    }

    async fn handle_put_maintenance_start_time(
        &self,
        state: &SharedState,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_put_maintenance_start_time_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.gateway_arn.is_empty() {
            return json_error_response(400, "ValidationException", "GatewayArn is required");
        }
        let arn = input.gateway_arn;
        let hour = input.hour_of_day;
        let minute = input.minute_of_hour;
        let day_of_month = input.day_of_month;
        let day_of_week = input.day_of_week;
        let mut state = state.write().await;
        match state.update_gateway(&arn, |g| {
            g.maintenance_start_time = Some(MaintenanceStartTime {
                hour_of_day: hour,
                minute_of_hour: minute,
                day_of_month,
                day_of_week,
            });
        }) {
            Ok(_) => wire::serialize_put_maintenance_start_time_response(
                &wire::PutMaintenanceStartTimeOutput {
                    gateway_arn: Some(arn),
                },
            ),
            Err(e) => bg_error_response(&e),
        }
    }

    async fn handle_get_bandwidth_rate_limit_schedule(
        &self,
        state: &SharedState,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_bandwidth_rate_limit_schedule_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.gateway_arn.is_empty() {
            return json_error_response(400, "ValidationException", "GatewayArn is required");
        }
        let arn = input.gateway_arn;
        let state = state.read().await;
        match state.get_gateway(&arn) {
            Ok(g) => {
                let intervals: Vec<wire::BandwidthRateLimitInterval> = g
                    .bandwidth_rate_limit_intervals
                    .iter()
                    .map(|i| wire::BandwidthRateLimitInterval {
                        average_upload_rate_limit_in_bits_per_sec: i
                            .average_upload_rate_limit_in_bits_per_sec,
                        days_of_week: i.days_of_week.clone(),
                        end_hour_of_day: i.end_hour_of_day,
                        end_minute_of_hour: i.end_minute_of_hour,
                        start_hour_of_day: i.start_hour_of_day,
                        start_minute_of_hour: i.start_minute_of_hour,
                    })
                    .collect();
                wire::serialize_get_bandwidth_rate_limit_schedule_response(
                    &wire::GetBandwidthRateLimitScheduleOutput {
                        bandwidth_rate_limit_intervals: Some(intervals),
                        gateway_arn: Some(g.arn.clone()),
                    },
                )
            }
            Err(e) => bg_error_response(&e),
        }
    }

    async fn handle_put_bandwidth_rate_limit_schedule(
        &self,
        state: &SharedState,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_put_bandwidth_rate_limit_schedule_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.gateway_arn.is_empty() {
            return json_error_response(400, "ValidationException", "GatewayArn is required");
        }
        let arn = input.gateway_arn;
        let intervals: Vec<BandwidthRateLimitInterval> = input
            .bandwidth_rate_limit_intervals
            .into_iter()
            .map(|i| BandwidthRateLimitInterval {
                average_upload_rate_limit_in_bits_per_sec: i
                    .average_upload_rate_limit_in_bits_per_sec,
                days_of_week: i.days_of_week,
                end_hour_of_day: i.end_hour_of_day,
                end_minute_of_hour: i.end_minute_of_hour,
                start_hour_of_day: i.start_hour_of_day,
                start_minute_of_hour: i.start_minute_of_hour,
            })
            .collect();
        let mut state = state.write().await;
        match state.update_gateway(&arn, |g| {
            g.bandwidth_rate_limit_intervals = intervals;
        }) {
            Ok(_) => wire::serialize_put_bandwidth_rate_limit_schedule_response(
                &wire::PutBandwidthRateLimitScheduleOutput {
                    gateway_arn: Some(arn),
                },
            ),
            Err(e) => bg_error_response(&e),
        }
    }

    async fn handle_import_hypervisor(
        &self,
        state: &SharedState,
        account_id: &str,
        region: &str,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_import_hypervisor_configuration_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.host.is_empty() {
            return json_error_response(400, "ValidationException", "Host is required");
        }
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "Name is required");
        }
        let host = input.host;
        let name = input.name;
        let id = uuid::Uuid::new_v4().to_string();
        let arn = format!(
            "arn:aws:backup-gateway:{}:{}:hypervisor/{}",
            region, account_id, id
        );
        let hypervisor = Hypervisor {
            arn: arn.clone(),
            host,
            name,
            state: "ONLINE".to_string(),
            kms_key_arn: input.kms_key_arn,
            log_group_arn: None,
            iam_role_arn: None,
            property_mappings: vec![],
            last_metadata_sync_status: "SUCCESSFUL".to_string(),
            last_metadata_sync_time: None,
            tags: wire_tags_to_map(input.tags.as_deref()),
        };
        let mut state = state.write().await;
        state.import_hypervisor(hypervisor);
        wire::serialize_import_hypervisor_configuration_response(
            &wire::ImportHypervisorConfigurationOutput {
                hypervisor_arn: Some(arn),
            },
        )
    }

    async fn handle_delete_hypervisor(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_delete_hypervisor_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.hypervisor_arn.is_empty() {
            return json_error_response(400, "ValidationException", "HypervisorArn is required");
        }
        let arn = input.hypervisor_arn;
        let mut state = state.write().await;
        match state.delete_hypervisor(&arn) {
            Ok(()) => wire::serialize_delete_hypervisor_response(&wire::DeleteHypervisorOutput {
                hypervisor_arn: Some(arn),
            }),
            Err(e) => bg_error_response(&e),
        }
    }

    async fn handle_get_hypervisor(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_get_hypervisor_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.hypervisor_arn.is_empty() {
            return json_error_response(400, "ValidationException", "HypervisorArn is required");
        }
        let arn = input.hypervisor_arn;
        let state = state.read().await;
        match state.get_hypervisor(&arn) {
            Ok(h) => wire::serialize_get_hypervisor_response(&wire::GetHypervisorOutput {
                hypervisor: Some(hypervisor_to_details(h)),
            }),
            Err(e) => bg_error_response(&e),
        }
    }

    async fn handle_list_hypervisors(&self, state: &SharedState) -> MockResponse {
        let state = state.read().await;
        let items: Vec<wire::Hypervisor> = state
            .list_hypervisors()
            .into_iter()
            .map(|h| wire::Hypervisor {
                host: Some(h.host.clone()),
                hypervisor_arn: Some(h.arn.clone()),
                kms_key_arn: h.kms_key_arn.clone(),
                name: Some(h.name.clone()),
                state: Some(h.state.clone()),
            })
            .collect();
        wire::serialize_list_hypervisors_response(&wire::ListHypervisorsOutput {
            hypervisors: Some(items),
            next_token: None,
        })
    }

    async fn handle_update_hypervisor(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_update_hypervisor_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.hypervisor_arn.is_empty() {
            return json_error_response(400, "ValidationException", "HypervisorArn is required");
        }
        let arn = input.hypervisor_arn;
        let new_host = input.host;
        let new_name = input.name;
        let new_log_group = input.log_group_arn;
        let mut state = state.write().await;
        match state.update_hypervisor(&arn, |h| {
            if let Some(host) = new_host {
                h.host = host;
            }
            if let Some(name) = new_name {
                h.name = name;
            }
            if let Some(lg) = new_log_group {
                h.log_group_arn = Some(lg);
            }
        }) {
            Ok(_) => wire::serialize_update_hypervisor_response(&wire::UpdateHypervisorOutput {
                hypervisor_arn: Some(arn),
            }),
            Err(e) => bg_error_response(&e),
        }
    }

    async fn handle_get_hypervisor_property_mappings(
        &self,
        state: &SharedState,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_hypervisor_property_mappings_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.hypervisor_arn.is_empty() {
            return json_error_response(400, "ValidationException", "HypervisorArn is required");
        }
        let arn = input.hypervisor_arn;
        let state = state.read().await;
        match state.get_hypervisor(&arn) {
            Ok(h) => {
                let mappings: Vec<wire::VmwareToAwsTagMapping> = h
                    .property_mappings
                    .iter()
                    .map(|m| wire::VmwareToAwsTagMapping {
                        aws_tag_key: m.aws_tag_key.clone(),
                        aws_tag_value: m.aws_tag_value.clone(),
                        vmware_category: m.vmware_category.clone(),
                        vmware_tag_name: m.vmware_tag_name.clone(),
                    })
                    .collect();
                wire::serialize_get_hypervisor_property_mappings_response(
                    &wire::GetHypervisorPropertyMappingsOutput {
                        hypervisor_arn: Some(h.arn.clone()),
                        iam_role_arn: h.iam_role_arn.clone(),
                        vmware_to_aws_tag_mappings: Some(mappings),
                    },
                )
            }
            Err(e) => bg_error_response(&e),
        }
    }

    async fn handle_put_hypervisor_property_mappings(
        &self,
        state: &SharedState,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_put_hypervisor_property_mappings_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.hypervisor_arn.is_empty() {
            return json_error_response(400, "ValidationException", "HypervisorArn is required");
        }
        if input.iam_role_arn.is_empty() {
            return json_error_response(400, "ValidationException", "IamRoleArn is required");
        }
        let arn = input.hypervisor_arn;
        let iam_role_arn = input.iam_role_arn;
        let mappings: Vec<TagMapping> = input
            .vmware_to_aws_tag_mappings
            .into_iter()
            .map(|m| TagMapping {
                aws_tag_key: m.aws_tag_key,
                aws_tag_value: m.aws_tag_value,
                vmware_category: m.vmware_category,
                vmware_tag_name: m.vmware_tag_name,
            })
            .collect();
        let mut state = state.write().await;
        match state.update_hypervisor(&arn, |h| {
            h.iam_role_arn = Some(iam_role_arn);
            h.property_mappings = mappings;
        }) {
            Ok(_) => wire::serialize_put_hypervisor_property_mappings_response(
                &wire::PutHypervisorPropertyMappingsOutput {
                    hypervisor_arn: Some(arn),
                },
            ),
            Err(e) => bg_error_response(&e),
        }
    }

    async fn handle_test_hypervisor(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_test_hypervisor_configuration_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.gateway_arn.is_empty() {
            return json_error_response(400, "ValidationException", "GatewayArn is required");
        }
        let arn = input.gateway_arn;
        let state = state.read().await;
        if state.get_gateway(&arn).is_err() {
            return bg_error_response(&BackupGatewayError::GatewayNotFound { arn });
        }
        wire::serialize_test_hypervisor_configuration_response(
            &wire::TestHypervisorConfigurationOutput {},
        )
    }

    async fn handle_start_metadata_sync(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_start_virtual_machines_metadata_sync_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.hypervisor_arn.is_empty() {
            return json_error_response(400, "ValidationException", "HypervisorArn is required");
        }
        let arn = input.hypervisor_arn;
        let mut state = state.write().await;
        match state.update_hypervisor(&arn, |h| {
            h.last_metadata_sync_status = "RUNNING".to_string();
            h.last_metadata_sync_time = Some(chrono::Utc::now().timestamp());
        }) {
            Ok(_) => wire::serialize_start_virtual_machines_metadata_sync_response(
                &wire::StartVirtualMachinesMetadataSyncOutput {
                    hypervisor_arn: Some(arn),
                },
            ),
            Err(e) => bg_error_response(&e),
        }
    }

    async fn handle_get_virtual_machine(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_get_virtual_machine_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(400, "ValidationException", "ResourceArn is required");
        }
        let arn = input.resource_arn;
        let state = state.read().await;
        match state.get_virtual_machine(&arn) {
            Ok(vm) => {
                wire::serialize_get_virtual_machine_response(&wire::GetVirtualMachineOutput {
                    virtual_machine: Some(wire::VirtualMachineDetails {
                        host_name: Some(vm.host_name.clone()),
                        hypervisor_id: Some(vm.hypervisor_arn.clone()),
                        last_backup_date: vm.last_backup_date.map(|t| t as f64),
                        name: Some(vm.name.clone()),
                        path: Some(vm.path.clone()),
                        resource_arn: Some(vm.resource_arn.clone()),
                        vmware_tags: Some(vec![]),
                    }),
                })
            }
            Err(e) => bg_error_response(&e),
        }
    }

    async fn handle_list_virtual_machines(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_list_virtual_machines_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let hypervisor_arn = input.hypervisor_arn.as_deref().filter(|s| !s.is_empty());
        let state = state.read().await;
        let items: Vec<wire::VirtualMachine> = state
            .list_virtual_machines(hypervisor_arn)
            .into_iter()
            .map(|vm| wire::VirtualMachine {
                host_name: Some(vm.host_name.clone()),
                hypervisor_id: Some(vm.hypervisor_arn.clone()),
                last_backup_date: vm.last_backup_date.map(|t| t as f64),
                name: Some(vm.name.clone()),
                path: Some(vm.path.clone()),
                resource_arn: Some(vm.resource_arn.clone()),
            })
            .collect();
        wire::serialize_list_virtual_machines_response(&wire::ListVirtualMachinesOutput {
            next_token: None,
            virtual_machines: Some(items),
        })
    }

    async fn handle_tag_resource(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_a_r_n.is_empty() {
            return json_error_response(400, "ValidationException", "ResourceARN is required");
        }
        let arn = input.resource_a_r_n;
        let tags = wire_tags_to_map(Some(input.tags.as_slice()));
        if tags.is_empty() {
            return json_error_response(400, "ValidationException", "Tags is required");
        }
        let mut state = state.write().await;
        match state.tag_resource(&arn, tags) {
            Ok(()) => wire::serialize_tag_resource_response(&wire::TagResourceOutput {
                resource_a_r_n: Some(arn),
            }),
            Err(e) => bg_error_response(&e),
        }
    }

    async fn handle_untag_resource(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_a_r_n.is_empty() {
            return json_error_response(400, "ValidationException", "ResourceARN is required");
        }
        let arn = input.resource_a_r_n;
        let keys = input.tag_keys;
        if keys.is_empty() {
            return json_error_response(400, "ValidationException", "TagKeys is required");
        }
        let mut state = state.write().await;
        match state.untag_resource(&arn, &keys) {
            Ok(()) => wire::serialize_untag_resource_response(&wire::UntagResourceOutput {
                resource_a_r_n: Some(arn),
            }),
            Err(e) => bg_error_response(&e),
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
        let arn = input.resource_arn;
        let state = state.read().await;
        match state.list_tags(&arn) {
            Ok(tags) => {
                let tags_wire: Vec<wire::Tag> = tags
                    .into_iter()
                    .map(|(k, v)| wire::Tag { key: k, value: v })
                    .collect();
                wire::serialize_list_tags_for_resource_response(&wire::ListTagsForResourceOutput {
                    resource_arn: Some(arn),
                    tags: if tags_wire.is_empty() {
                        None
                    } else {
                        Some(tags_wire)
                    },
                })
            }
            Err(e) => bg_error_response(&e),
        }
    }
}

fn gateway_to_details(g: &Gateway) -> wire::GatewayDetails {
    wire::GatewayDetails {
        deprecation_date: None,
        gateway_arn: Some(g.arn.clone()),
        gateway_display_name: Some(g.display_name.clone()),
        gateway_type: Some(g.gateway_type.clone()),
        hypervisor_id: g.hypervisor_arn.clone(),
        last_seen_time: Some(g.last_seen_time as f64),
        maintenance_start_time: g.maintenance_start_time.as_ref().map(|m| {
            wire::MaintenanceStartTime {
                day_of_month: m.day_of_month,
                day_of_week: m.day_of_week,
                hour_of_day: Some(m.hour_of_day),
                minute_of_hour: Some(m.minute_of_hour),
            }
        }),
        next_update_availability_time: None,
        software_version: Some(g.software_version.clone()),
        vpc_endpoint: g.vpc_endpoint.clone(),
    }
}

fn hypervisor_to_details(h: &Hypervisor) -> wire::HypervisorDetails {
    wire::HypervisorDetails {
        host: Some(h.host.clone()),
        hypervisor_arn: Some(h.arn.clone()),
        kms_key_arn: h.kms_key_arn.clone(),
        last_successful_metadata_sync_time: h.last_metadata_sync_time.map(|t| t as f64),
        latest_metadata_sync_status: Some(h.last_metadata_sync_status.clone()),
        latest_metadata_sync_status_message: None,
        log_group_arn: h.log_group_arn.clone(),
        name: Some(h.name.clone()),
        state: Some(h.state.clone()),
    }
}

fn wire_tags_to_map(tags: Option<&[wire::Tag]>) -> HashMap<String, String> {
    tags.map(|a| a.iter().map(|t| (t.key.clone(), t.value.clone())).collect())
        .unwrap_or_default()
}

fn bg_error_response(err: &BackupGatewayError) -> MockResponse {
    let (status, error_type) = match err {
        BackupGatewayError::GatewayNotFound { .. } => (404, "ResourceNotFoundException"),
        BackupGatewayError::HypervisorNotFound { .. } => (404, "ResourceNotFoundException"),
        BackupGatewayError::VirtualMachineNotFound { .. } => (404, "ResourceNotFoundException"),
        BackupGatewayError::ResourceNotFound { .. } => (404, "ResourceNotFoundException"),
        BackupGatewayError::Validation { .. } => (400, "ValidationException"),
    };
    json_error_response(status, error_type, &err.to_string())
}
