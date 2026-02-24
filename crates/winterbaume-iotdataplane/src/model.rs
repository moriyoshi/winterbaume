//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-iot-data-plane

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteConnectionRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteThingShadowRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteThingShadowResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRetainedMessageRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRetainedMessageResponse {
    #[serde(rename = "lastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qos: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    #[serde(rename = "userProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_properties: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetThingShadowRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetThingShadowResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListNamedShadowsForThingRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListNamedShadowsForThingResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRetainedMessagesRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRetainedMessagesResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "retainedTopics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retained_topics: Option<Vec<RetainedMessageSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RetainedMessageSummary {
    #[serde(rename = "lastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<i64>,
    #[serde(rename = "payloadSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_size: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qos: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PublishRequest {
    #[serde(rename = "correlationData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correlation_data: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,
    #[serde(rename = "payloadFormatIndicator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_format_indicator: Option<String>,
    #[serde(rename = "userProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_properties: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateThingShadowRequest {
    #[serde(default)]
    pub payload: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateThingShadowResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,
}
