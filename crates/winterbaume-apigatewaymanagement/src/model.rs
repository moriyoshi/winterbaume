//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-apigatewaymanagementapi

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetConnectionRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteConnectionRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetConnectionResponse {
    #[serde(rename = "connectedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
    #[serde(rename = "lastActiveAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_active_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Identity {
    #[serde(rename = "sourceIp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ip: Option<String>,
    #[serde(rename = "userAgent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PostToConnectionRequest {
    #[serde(rename = "Data")]
    #[serde(default)]
    pub data: String,
}
