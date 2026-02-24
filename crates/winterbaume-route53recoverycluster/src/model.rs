//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-route53recoverycluster

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRoutingControlStateRequest {
    #[serde(rename = "RoutingControlArn")]
    #[serde(default)]
    pub routing_control_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRoutingControlStateResponse {
    #[serde(rename = "RoutingControlArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_control_arn: Option<String>,
    #[serde(rename = "RoutingControlName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_control_name: Option<String>,
    #[serde(rename = "RoutingControlState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_control_state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRoutingControlsRequest {
    #[serde(rename = "ControlPanelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_panel_arn: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRoutingControlsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RoutingControls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_controls: Option<Vec<RoutingControl>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RoutingControl {
    #[serde(rename = "ControlPanelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_panel_arn: Option<String>,
    #[serde(rename = "ControlPanelName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_panel_name: Option<String>,
    #[serde(rename = "Owner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(rename = "RoutingControlArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_control_arn: Option<String>,
    #[serde(rename = "RoutingControlName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_control_name: Option<String>,
    #[serde(rename = "RoutingControlState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_control_state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRoutingControlStateRequest {
    #[serde(rename = "RoutingControlArn")]
    #[serde(default)]
    pub routing_control_arn: String,
    #[serde(rename = "RoutingControlState")]
    #[serde(default)]
    pub routing_control_state: String,
    #[serde(rename = "SafetyRulesToOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_rules_to_override: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRoutingControlStateResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRoutingControlStatesRequest {
    #[serde(rename = "SafetyRulesToOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_rules_to_override: Option<Vec<String>>,
    #[serde(rename = "UpdateRoutingControlStateEntries")]
    #[serde(default)]
    pub update_routing_control_state_entries: Vec<UpdateRoutingControlStateEntry>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRoutingControlStateEntry {
    #[serde(rename = "RoutingControlArn")]
    #[serde(default)]
    pub routing_control_arn: String,
    #[serde(rename = "RoutingControlState")]
    #[serde(default)]
    pub routing_control_state: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRoutingControlStatesResponse {}
