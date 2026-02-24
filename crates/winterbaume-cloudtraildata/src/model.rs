//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-cloudtraildata

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutAuditEventsRequest {
    #[serde(rename = "auditEvents")]
    #[serde(default)]
    pub audit_events: Vec<AuditEvent>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuditEvent {
    #[serde(rename = "eventData")]
    #[serde(default)]
    pub event_data: String,
    #[serde(rename = "eventDataChecksum")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_data_checksum: Option<String>,
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutAuditEventsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed: Option<Vec<ResultErrorEntry>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful: Option<Vec<AuditEventResultEntry>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResultErrorEntry {
    #[serde(rename = "errorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "errorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuditEventResultEntry {
    #[serde(rename = "eventID")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_i_d: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
