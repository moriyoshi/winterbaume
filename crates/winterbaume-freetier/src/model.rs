//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-freetier

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAccountActivityRequest {
    #[serde(rename = "activityId")]
    #[serde(default)]
    pub activity_id: String,
    #[serde(rename = "languageCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAccountActivityResponse {
    #[serde(rename = "activityId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<String>,
    #[serde(rename = "completedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "estimatedTimeToCompleteInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_time_to_complete_in_minutes: Option<i32>,
    #[serde(rename = "expiresAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    #[serde(rename = "instructionsUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions_url: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reward: Option<ActivityReward>,
    #[serde(rename = "startedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActivityReward {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit: Option<MonetaryAmount>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MonetaryAmount {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAccountPlanStateRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAccountPlanStateResponse {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "accountPlanExpirationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_plan_expiration_date: Option<String>,
    #[serde(rename = "accountPlanRemainingCredits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_plan_remaining_credits: Option<MonetaryAmount>,
    #[serde(rename = "accountPlanStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_plan_status: Option<String>,
    #[serde(rename = "accountPlanType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_plan_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFreeTierUsageRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Expression>,
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
pub struct Expression {
    #[serde(rename = "And")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and: Option<Vec<Expression>>,
    #[serde(rename = "Dimensions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<DimensionValues>,
    #[serde(rename = "Not")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not: Option<Box<Expression>>,
    #[serde(rename = "Or")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub or: Option<Vec<Expression>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DimensionValues {
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "MatchOptions")]
    #[serde(default)]
    pub match_options: Vec<String>,
    #[serde(rename = "Values")]
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFreeTierUsageResponse {
    #[serde(rename = "freeTierUsages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free_tier_usages: Option<Vec<FreeTierUsage>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FreeTierUsage {
    #[serde(rename = "actualUsageAmount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_usage_amount: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "forecastedUsageAmount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecasted_usage_amount: Option<f64>,
    #[serde(rename = "freeTierType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free_tier_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(rename = "usageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAccountActivitiesRequest {
    #[serde(rename = "filterActivityStatuses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_activity_statuses: Option<Vec<String>>,
    #[serde(rename = "languageCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
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
pub struct ListAccountActivitiesResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activities: Option<Vec<ActivitySummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActivitySummary {
    #[serde(rename = "activityId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reward: Option<ActivityReward>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpgradeAccountPlanRequest {
    #[serde(rename = "accountPlanType")]
    #[serde(default)]
    pub account_plan_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpgradeAccountPlanResponse {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "accountPlanStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_plan_status: Option<String>,
    #[serde(rename = "accountPlanType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_plan_type: Option<String>,
}
