//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-backupgateway

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for awsJson protocol.
pub fn serialize_associate_gateway_to_server_response(
    result: &AssociateGatewayToServerOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_gateway_response(result: &CreateGatewayOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_gateway_response(result: &DeleteGatewayOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_hypervisor_response(result: &DeleteHypervisorOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_disassociate_gateway_from_server_response(
    result: &DisassociateGatewayFromServerOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_bandwidth_rate_limit_schedule_response(
    result: &GetBandwidthRateLimitScheduleOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_gateway_response(result: &GetGatewayOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_hypervisor_response(result: &GetHypervisorOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_hypervisor_property_mappings_response(
    result: &GetHypervisorPropertyMappingsOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_virtual_machine_response(result: &GetVirtualMachineOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_import_hypervisor_configuration_response(
    result: &ImportHypervisorConfigurationOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_gateways_response(result: &ListGatewaysOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_hypervisors_response(result: &ListHypervisorsOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_tags_for_resource_response(
    result: &ListTagsForResourceOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_virtual_machines_response(
    result: &ListVirtualMachinesOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_bandwidth_rate_limit_schedule_response(
    result: &PutBandwidthRateLimitScheduleOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_hypervisor_property_mappings_response(
    result: &PutHypervisorPropertyMappingsOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_maintenance_start_time_response(
    result: &PutMaintenanceStartTimeOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_virtual_machines_metadata_sync_response(
    result: &StartVirtualMachinesMetadataSyncOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_tag_resource_response(result: &TagResourceOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_test_hypervisor_configuration_response(
    result: &TestHypervisorConfigurationOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_untag_resource_response(result: &UntagResourceOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_gateway_information_response(
    result: &UpdateGatewayInformationOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_gateway_software_now_response(
    result: &UpdateGatewaySoftwareNowOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_hypervisor_response(result: &UpdateHypervisorOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_associate_gateway_to_server_request(
    body: &[u8],
) -> Result<AssociateGatewayToServerInput, String> {
    if body.is_empty() {
        return Ok(AssociateGatewayToServerInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize AssociateGatewayToServer request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_gateway_request(body: &[u8]) -> Result<CreateGatewayInput, String> {
    if body.is_empty() {
        return Ok(CreateGatewayInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateGateway request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_gateway_request(body: &[u8]) -> Result<DeleteGatewayInput, String> {
    if body.is_empty() {
        return Ok(DeleteGatewayInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteGateway request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_hypervisor_request(body: &[u8]) -> Result<DeleteHypervisorInput, String> {
    if body.is_empty() {
        return Ok(DeleteHypervisorInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteHypervisor request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_disassociate_gateway_from_server_request(
    body: &[u8],
) -> Result<DisassociateGatewayFromServerInput, String> {
    if body.is_empty() {
        return Ok(DisassociateGatewayFromServerInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DisassociateGatewayFromServer request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_bandwidth_rate_limit_schedule_request(
    body: &[u8],
) -> Result<GetBandwidthRateLimitScheduleInput, String> {
    if body.is_empty() {
        return Ok(GetBandwidthRateLimitScheduleInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetBandwidthRateLimitSchedule request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_gateway_request(body: &[u8]) -> Result<GetGatewayInput, String> {
    if body.is_empty() {
        return Ok(GetGatewayInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetGateway request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_hypervisor_request(body: &[u8]) -> Result<GetHypervisorInput, String> {
    if body.is_empty() {
        return Ok(GetHypervisorInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetHypervisor request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_hypervisor_property_mappings_request(
    body: &[u8],
) -> Result<GetHypervisorPropertyMappingsInput, String> {
    if body.is_empty() {
        return Ok(GetHypervisorPropertyMappingsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetHypervisorPropertyMappings request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_virtual_machine_request(
    body: &[u8],
) -> Result<GetVirtualMachineInput, String> {
    if body.is_empty() {
        return Ok(GetVirtualMachineInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetVirtualMachine request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_import_hypervisor_configuration_request(
    body: &[u8],
) -> Result<ImportHypervisorConfigurationInput, String> {
    if body.is_empty() {
        return Ok(ImportHypervisorConfigurationInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ImportHypervisorConfiguration request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_gateways_request(body: &[u8]) -> Result<ListGatewaysInput, String> {
    if body.is_empty() {
        return Ok(ListGatewaysInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListGateways request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_hypervisors_request(body: &[u8]) -> Result<ListHypervisorsInput, String> {
    if body.is_empty() {
        return Ok(ListHypervisorsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListHypervisors request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_tags_for_resource_request(
    body: &[u8],
) -> Result<ListTagsForResourceInput, String> {
    if body.is_empty() {
        return Ok(ListTagsForResourceInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListTagsForResource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_virtual_machines_request(
    body: &[u8],
) -> Result<ListVirtualMachinesInput, String> {
    if body.is_empty() {
        return Ok(ListVirtualMachinesInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListVirtualMachines request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_bandwidth_rate_limit_schedule_request(
    body: &[u8],
) -> Result<PutBandwidthRateLimitScheduleInput, String> {
    if body.is_empty() {
        return Ok(PutBandwidthRateLimitScheduleInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutBandwidthRateLimitSchedule request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_hypervisor_property_mappings_request(
    body: &[u8],
) -> Result<PutHypervisorPropertyMappingsInput, String> {
    if body.is_empty() {
        return Ok(PutHypervisorPropertyMappingsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutHypervisorPropertyMappings request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_put_maintenance_start_time_request(
    body: &[u8],
) -> Result<PutMaintenanceStartTimeInput, String> {
    if body.is_empty() {
        return Ok(PutMaintenanceStartTimeInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize PutMaintenanceStartTime request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_start_virtual_machines_metadata_sync_request(
    body: &[u8],
) -> Result<StartVirtualMachinesMetadataSyncInput, String> {
    if body.is_empty() {
        return Ok(StartVirtualMachinesMetadataSyncInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize StartVirtualMachinesMetadataSync request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_tag_resource_request(body: &[u8]) -> Result<TagResourceInput, String> {
    if body.is_empty() {
        return Ok(TagResourceInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize TagResource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_test_hypervisor_configuration_request(
    body: &[u8],
) -> Result<TestHypervisorConfigurationInput, String> {
    if body.is_empty() {
        return Ok(TestHypervisorConfigurationInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize TestHypervisorConfiguration request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_untag_resource_request(body: &[u8]) -> Result<UntagResourceInput, String> {
    if body.is_empty() {
        return Ok(UntagResourceInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UntagResource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_gateway_information_request(
    body: &[u8],
) -> Result<UpdateGatewayInformationInput, String> {
    if body.is_empty() {
        return Ok(UpdateGatewayInformationInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateGatewayInformation request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_gateway_software_now_request(
    body: &[u8],
) -> Result<UpdateGatewaySoftwareNowInput, String> {
    if body.is_empty() {
        return Ok(UpdateGatewaySoftwareNowInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateGatewaySoftwareNow request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_hypervisor_request(body: &[u8]) -> Result<UpdateHypervisorInput, String> {
    if body.is_empty() {
        return Ok(UpdateHypervisorInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateHypervisor request: {e}"))
}
