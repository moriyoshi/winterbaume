//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-personalizeruntime

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetActionRecommendationsRequest {
    #[serde(rename = "campaignArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_arn: Option<String>,
    #[serde(rename = "filterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_arn: Option<String>,
    #[serde(rename = "filterValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_values: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "numResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_results: Option<i32>,
    #[serde(rename = "userId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetActionRecommendationsResponse {
    #[serde(rename = "actionList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_list: Option<Vec<PredictedAction>>,
    #[serde(rename = "recommendationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PredictedAction {
    #[serde(rename = "actionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPersonalizedRankingRequest {
    #[serde(rename = "campaignArn")]
    #[serde(default)]
    pub campaign_arn: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "filterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_arn: Option<String>,
    #[serde(rename = "filterValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_values: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "inputList")]
    #[serde(default)]
    pub input_list: Vec<String>,
    #[serde(rename = "metadataColumns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_columns: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "userId")]
    #[serde(default)]
    pub user_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPersonalizedRankingResponse {
    #[serde(rename = "personalizedRanking")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personalized_ranking: Option<Vec<PredictedItem>>,
    #[serde(rename = "recommendationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PredictedItem {
    #[serde(rename = "itemId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "promotionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRecommendationsRequest {
    #[serde(rename = "campaignArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "filterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_arn: Option<String>,
    #[serde(rename = "filterValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_values: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "itemId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    #[serde(rename = "metadataColumns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_columns: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "numResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_results: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotions: Option<Vec<Promotion>>,
    #[serde(rename = "recommenderArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommender_arn: Option<String>,
    #[serde(rename = "userId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Promotion {
    #[serde(rename = "filterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_arn: Option<String>,
    #[serde(rename = "filterValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_values: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "percentPromotedItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_promoted_items: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRecommendationsResponse {
    #[serde(rename = "itemList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_list: Option<Vec<PredictedItem>>,
    #[serde(rename = "recommendationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_id: Option<String>,
}
