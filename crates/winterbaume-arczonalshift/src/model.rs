//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-arczonalshift

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelPracticeRunRequest {
    #[serde(rename = "zonalShiftId")]
    #[serde(default)]
    pub zonal_shift_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelPracticeRunResponse {
    #[serde(rename = "awayFrom")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub away_from: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "expiryTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_time: Option<f64>,
    #[serde(rename = "resourceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_identifier: Option<String>,
    #[serde(rename = "startTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "zonalShiftId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zonal_shift_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelZonalShiftRequest {
    #[serde(rename = "zonalShiftId")]
    #[serde(default)]
    pub zonal_shift_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePracticeRunConfigurationRequest {
    #[serde(rename = "allowedWindows")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_windows: Option<Vec<String>>,
    #[serde(rename = "blockedDates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked_dates: Option<Vec<String>>,
    #[serde(rename = "blockedWindows")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked_windows: Option<Vec<String>>,
    #[serde(rename = "blockingAlarms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocking_alarms: Option<Vec<ControlCondition>>,
    #[serde(rename = "outcomeAlarms")]
    #[serde(default)]
    pub outcome_alarms: Vec<ControlCondition>,
    #[serde(rename = "resourceIdentifier")]
    #[serde(default)]
    pub resource_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ControlCondition {
    #[serde(rename = "alarmIdentifier")]
    #[serde(default)]
    pub alarm_identifier: String,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePracticeRunConfigurationResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "practiceRunConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub practice_run_configuration: Option<PracticeRunConfiguration>,
    #[serde(rename = "zonalAutoshiftStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zonal_autoshift_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PracticeRunConfiguration {
    #[serde(rename = "allowedWindows")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_windows: Option<Vec<String>>,
    #[serde(rename = "blockedDates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked_dates: Option<Vec<String>>,
    #[serde(rename = "blockedWindows")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked_windows: Option<Vec<String>>,
    #[serde(rename = "blockingAlarms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocking_alarms: Option<Vec<ControlCondition>>,
    #[serde(rename = "outcomeAlarms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outcome_alarms: Option<Vec<ControlCondition>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePracticeRunConfigurationRequest {
    #[serde(rename = "resourceIdentifier")]
    #[serde(default)]
    pub resource_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePracticeRunConfigurationResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "zonalAutoshiftStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zonal_autoshift_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAutoshiftObserverNotificationStatusRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAutoshiftObserverNotificationStatusResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetManagedResourceRequest {
    #[serde(rename = "resourceIdentifier")]
    #[serde(default)]
    pub resource_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetManagedResourceResponse {
    #[serde(rename = "appliedWeights")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applied_weights: Option<std::collections::HashMap<String, f32>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autoshifts: Option<Vec<AutoshiftInResource>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "practiceRunConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub practice_run_configuration: Option<PracticeRunConfiguration>,
    #[serde(rename = "zonalAutoshiftStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zonal_autoshift_status: Option<String>,
    #[serde(rename = "zonalShifts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zonal_shifts: Option<Vec<ZonalShiftInResource>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutoshiftInResource {
    #[serde(rename = "appliedStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applied_status: Option<String>,
    #[serde(rename = "awayFrom")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub away_from: Option<String>,
    #[serde(rename = "startTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ZonalShiftInResource {
    #[serde(rename = "appliedStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applied_status: Option<String>,
    #[serde(rename = "awayFrom")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub away_from: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "expiryTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_time: Option<f64>,
    #[serde(rename = "practiceRunOutcome")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub practice_run_outcome: Option<String>,
    #[serde(rename = "resourceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_identifier: Option<String>,
    #[serde(rename = "shiftType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shift_type: Option<String>,
    #[serde(rename = "startTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(rename = "zonalShiftId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zonal_shift_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAutoshiftsRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAutoshiftsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<AutoshiftSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutoshiftSummary {
    #[serde(rename = "awayFrom")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub away_from: Option<String>,
    #[serde(rename = "endTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "startTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListManagedResourcesRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListManagedResourcesResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<ManagedResourceSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ManagedResourceSummary {
    #[serde(rename = "appliedWeights")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applied_weights: Option<std::collections::HashMap<String, f32>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autoshifts: Option<Vec<AutoshiftInResource>>,
    #[serde(rename = "availabilityZones")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "practiceRunStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub practice_run_status: Option<String>,
    #[serde(rename = "zonalAutoshiftStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zonal_autoshift_status: Option<String>,
    #[serde(rename = "zonalShifts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zonal_shifts: Option<Vec<ZonalShiftInResource>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListZonalShiftsRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "resourceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_identifier: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListZonalShiftsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<ZonalShiftSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ZonalShiftSummary {
    #[serde(rename = "awayFrom")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub away_from: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "expiryTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_time: Option<f64>,
    #[serde(rename = "practiceRunOutcome")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub practice_run_outcome: Option<String>,
    #[serde(rename = "resourceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_identifier: Option<String>,
    #[serde(rename = "shiftType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shift_type: Option<String>,
    #[serde(rename = "startTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "zonalShiftId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zonal_shift_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartPracticeRunRequest {
    #[serde(rename = "awayFrom")]
    #[serde(default)]
    pub away_from: String,
    #[serde(default)]
    pub comment: String,
    #[serde(rename = "resourceIdentifier")]
    #[serde(default)]
    pub resource_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartPracticeRunResponse {
    #[serde(rename = "awayFrom")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub away_from: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "expiryTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_time: Option<f64>,
    #[serde(rename = "resourceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_identifier: Option<String>,
    #[serde(rename = "startTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "zonalShiftId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zonal_shift_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartZonalShiftRequest {
    #[serde(rename = "awayFrom")]
    #[serde(default)]
    pub away_from: String,
    #[serde(default)]
    pub comment: String,
    #[serde(rename = "expiresIn")]
    #[serde(default)]
    pub expires_in: String,
    #[serde(rename = "resourceIdentifier")]
    #[serde(default)]
    pub resource_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAutoshiftObserverNotificationStatusRequest {
    #[serde(default)]
    pub status: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAutoshiftObserverNotificationStatusResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePracticeRunConfigurationRequest {
    #[serde(rename = "allowedWindows")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_windows: Option<Vec<String>>,
    #[serde(rename = "blockedDates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked_dates: Option<Vec<String>>,
    #[serde(rename = "blockedWindows")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked_windows: Option<Vec<String>>,
    #[serde(rename = "blockingAlarms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocking_alarms: Option<Vec<ControlCondition>>,
    #[serde(rename = "outcomeAlarms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outcome_alarms: Option<Vec<ControlCondition>>,
    #[serde(rename = "resourceIdentifier")]
    #[serde(default)]
    pub resource_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePracticeRunConfigurationResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "practiceRunConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub practice_run_configuration: Option<PracticeRunConfiguration>,
    #[serde(rename = "zonalAutoshiftStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zonal_autoshift_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateZonalAutoshiftConfigurationRequest {
    #[serde(rename = "resourceIdentifier")]
    #[serde(default)]
    pub resource_identifier: String,
    #[serde(rename = "zonalAutoshiftStatus")]
    #[serde(default)]
    pub zonal_autoshift_status: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateZonalAutoshiftConfigurationResponse {
    #[serde(rename = "resourceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_identifier: Option<String>,
    #[serde(rename = "zonalAutoshiftStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zonal_autoshift_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateZonalShiftRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "expiresIn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_in: Option<String>,
    #[serde(rename = "zonalShiftId")]
    #[serde(default)]
    pub zonal_shift_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ZonalShift {
    #[serde(rename = "awayFrom")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub away_from: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "expiryTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_time: Option<f64>,
    #[serde(rename = "resourceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_identifier: Option<String>,
    #[serde(rename = "startTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "zonalShiftId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zonal_shift_id: Option<String>,
}
