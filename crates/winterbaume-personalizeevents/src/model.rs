//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-personalizeevents

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutActionInteractionsRequest {
    #[serde(rename = "actionInteractions")]
    #[serde(default)]
    pub action_interactions: Vec<ActionInteraction>,
    #[serde(rename = "trackingId")]
    #[serde(default)]
    pub tracking_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActionInteraction {
    #[serde(rename = "actionId")]
    #[serde(default)]
    pub action_id: String,
    #[serde(rename = "eventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    #[serde(rename = "eventType")]
    #[serde(default)]
    pub event_type: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impression: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<String>,
    #[serde(rename = "recommendationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_id: Option<String>,
    #[serde(rename = "sessionId")]
    #[serde(default)]
    pub session_id: String,
    #[serde(default)]
    pub timestamp: f64,
    #[serde(rename = "userId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutActionsRequest {
    #[serde(default)]
    pub actions: Vec<Action>,
    #[serde(rename = "datasetArn")]
    #[serde(default)]
    pub dataset_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Action {
    #[serde(rename = "actionId")]
    #[serde(default)]
    pub action_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutEventsRequest {
    #[serde(rename = "eventList")]
    #[serde(default)]
    pub event_list: Vec<Event>,
    #[serde(rename = "sessionId")]
    #[serde(default)]
    pub session_id: String,
    #[serde(rename = "trackingId")]
    #[serde(default)]
    pub tracking_id: String,
    #[serde(rename = "userId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Event {
    #[serde(rename = "eventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    #[serde(rename = "eventType")]
    #[serde(default)]
    pub event_type: String,
    #[serde(rename = "eventValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_value: Option<f32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impression: Option<Vec<String>>,
    #[serde(rename = "itemId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    #[serde(rename = "metricAttribution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_attribution: Option<MetricAttribution>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<String>,
    #[serde(rename = "recommendationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_id: Option<String>,
    #[serde(rename = "sentAt")]
    #[serde(default)]
    pub sent_at: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MetricAttribution {
    #[serde(rename = "eventAttributionSource")]
    #[serde(default)]
    pub event_attribution_source: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutItemsRequest {
    #[serde(rename = "datasetArn")]
    #[serde(default)]
    pub dataset_arn: String,
    #[serde(default)]
    pub items: Vec<Item>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Item {
    #[serde(rename = "itemId")]
    #[serde(default)]
    pub item_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutUsersRequest {
    #[serde(rename = "datasetArn")]
    #[serde(default)]
    pub dataset_arn: String,
    #[serde(default)]
    pub users: Vec<User>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct User {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<String>,
    #[serde(rename = "userId")]
    #[serde(default)]
    pub user_id: String,
}
