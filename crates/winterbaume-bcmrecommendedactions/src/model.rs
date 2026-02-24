//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-bcm-recommended-actions

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRecommendedActionsRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<RequestFilter>,
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
pub struct RequestFilter {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<ActionFilter>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActionFilter {
    #[serde(default)]
    pub key: String,
    #[serde(rename = "matchOption")]
    #[serde(default)]
    pub match_option: String,
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRecommendedActionsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "recommendedActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended_actions: Option<Vec<RecommendedAction>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecommendedAction {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "lastUpdatedTimeStamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time_stamp: Option<String>,
    #[serde(rename = "nextSteps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_steps: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}
