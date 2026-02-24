//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-budgets

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateBudgetActionRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "ActionThreshold")]
    #[serde(default)]
    pub action_threshold: ActionThreshold,
    #[serde(rename = "ActionType")]
    #[serde(default)]
    pub action_type: String,
    #[serde(rename = "ApprovalModel")]
    #[serde(default)]
    pub approval_model: String,
    #[serde(rename = "BudgetName")]
    #[serde(default)]
    pub budget_name: String,
    #[serde(rename = "Definition")]
    #[serde(default)]
    pub definition: Definition,
    #[serde(rename = "ExecutionRoleArn")]
    #[serde(default)]
    pub execution_role_arn: String,
    #[serde(rename = "NotificationType")]
    #[serde(default)]
    pub notification_type: String,
    #[serde(rename = "ResourceTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_tags: Option<Vec<ResourceTag>>,
    #[serde(rename = "Subscribers")]
    #[serde(default)]
    pub subscribers: Vec<Subscriber>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActionThreshold {
    #[serde(rename = "ActionThresholdType")]
    #[serde(default)]
    pub action_threshold_type: String,
    #[serde(rename = "ActionThresholdValue")]
    #[serde(default)]
    pub action_threshold_value: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Definition {
    #[serde(rename = "IamActionDefinition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_action_definition: Option<IamActionDefinition>,
    #[serde(rename = "ScpActionDefinition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scp_action_definition: Option<ScpActionDefinition>,
    #[serde(rename = "SsmActionDefinition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssm_action_definition: Option<SsmActionDefinition>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IamActionDefinition {
    #[serde(rename = "Groups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
    #[serde(rename = "PolicyArn")]
    #[serde(default)]
    pub policy_arn: String,
    #[serde(rename = "Roles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,
    #[serde(rename = "Users")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScpActionDefinition {
    #[serde(rename = "PolicyId")]
    #[serde(default)]
    pub policy_id: String,
    #[serde(rename = "TargetIds")]
    #[serde(default)]
    pub target_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SsmActionDefinition {
    #[serde(rename = "ActionSubType")]
    #[serde(default)]
    pub action_sub_type: String,
    #[serde(rename = "InstanceIds")]
    #[serde(default)]
    pub instance_ids: Vec<String>,
    #[serde(rename = "Region")]
    #[serde(default)]
    pub region: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceTag {
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Subscriber {
    #[serde(rename = "Address")]
    #[serde(default)]
    pub address: String,
    #[serde(rename = "SubscriptionType")]
    #[serde(default)]
    pub subscription_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateBudgetActionResponse {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "ActionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_id: Option<String>,
    #[serde(rename = "BudgetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateBudgetRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "Budget")]
    #[serde(default)]
    pub budget: Budget,
    #[serde(rename = "NotificationsWithSubscribers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notifications_with_subscribers: Option<Vec<NotificationWithSubscribers>>,
    #[serde(rename = "ResourceTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_tags: Option<Vec<ResourceTag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Budget {
    #[serde(rename = "AutoAdjustData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_adjust_data: Option<AutoAdjustData>,
    #[serde(rename = "BillingViewArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_view_arn: Option<String>,
    #[serde(rename = "BudgetLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_limit: Option<Spend>,
    #[serde(rename = "BudgetName")]
    #[serde(default)]
    pub budget_name: String,
    #[serde(rename = "BudgetType")]
    #[serde(default)]
    pub budget_type: String,
    #[serde(rename = "CalculatedSpend")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calculated_spend: Option<CalculatedSpend>,
    #[serde(rename = "CostFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_filters: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "CostTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_types: Option<CostTypes>,
    #[serde(rename = "FilterExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_expression: Option<Expression>,
    #[serde(rename = "HealthStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_status: Option<HealthStatus>,
    #[serde(rename = "LastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(rename = "Metrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Vec<String>>,
    #[serde(rename = "PlannedBudgetLimits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub planned_budget_limits: Option<std::collections::HashMap<String, Spend>>,
    #[serde(rename = "TimePeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_period: Option<TimePeriod>,
    #[serde(rename = "TimeUnit")]
    #[serde(default)]
    pub time_unit: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutoAdjustData {
    #[serde(rename = "AutoAdjustType")]
    #[serde(default)]
    pub auto_adjust_type: String,
    #[serde(rename = "HistoricalOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub historical_options: Option<HistoricalOptions>,
    #[serde(rename = "LastAutoAdjustTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_auto_adjust_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HistoricalOptions {
    #[serde(rename = "BudgetAdjustmentPeriod")]
    #[serde(default)]
    pub budget_adjustment_period: i32,
    #[serde(rename = "LookBackAvailablePeriods")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub look_back_available_periods: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Spend {
    #[serde(rename = "Amount")]
    #[serde(default)]
    pub amount: String,
    #[serde(rename = "Unit")]
    #[serde(default)]
    pub unit: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CalculatedSpend {
    #[serde(rename = "ActualSpend")]
    #[serde(default)]
    pub actual_spend: Spend,
    #[serde(rename = "ForecastedSpend")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecasted_spend: Option<Spend>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CostTypes {
    #[serde(rename = "IncludeCredit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_credit: Option<bool>,
    #[serde(rename = "IncludeDiscount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_discount: Option<bool>,
    #[serde(rename = "IncludeOtherSubscription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_other_subscription: Option<bool>,
    #[serde(rename = "IncludeRecurring")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_recurring: Option<bool>,
    #[serde(rename = "IncludeRefund")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_refund: Option<bool>,
    #[serde(rename = "IncludeSubscription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_subscription: Option<bool>,
    #[serde(rename = "IncludeSupport")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_support: Option<bool>,
    #[serde(rename = "IncludeTax")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_tax: Option<bool>,
    #[serde(rename = "IncludeUpfront")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_upfront: Option<bool>,
    #[serde(rename = "UseAmortized")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_amortized: Option<bool>,
    #[serde(rename = "UseBlended")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_blended: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Expression {
    #[serde(rename = "And")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and: Option<Vec<Expression>>,
    #[serde(rename = "CostCategories")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_categories: Option<CostCategoryValues>,
    #[serde(rename = "Dimensions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<ExpressionDimensionValues>,
    #[serde(rename = "Not")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not: Option<Box<Expression>>,
    #[serde(rename = "Or")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub or: Option<Vec<Expression>>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagValues>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CostCategoryValues {
    #[serde(rename = "Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "MatchOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_options: Option<Vec<String>>,
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExpressionDimensionValues {
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "MatchOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_options: Option<Vec<String>>,
    #[serde(rename = "Values")]
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagValues {
    #[serde(rename = "Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "MatchOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_options: Option<Vec<String>>,
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HealthStatus {
    #[serde(rename = "LastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TimePeriod {
    #[serde(rename = "End")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<f64>,
    #[serde(rename = "Start")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NotificationWithSubscribers {
    #[serde(rename = "Notification")]
    #[serde(default)]
    pub notification: Notification,
    #[serde(rename = "Subscribers")]
    #[serde(default)]
    pub subscribers: Vec<Subscriber>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Notification {
    #[serde(rename = "ComparisonOperator")]
    #[serde(default)]
    pub comparison_operator: String,
    #[serde(rename = "NotificationState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_state: Option<String>,
    #[serde(rename = "NotificationType")]
    #[serde(default)]
    pub notification_type: String,
    #[serde(rename = "Threshold")]
    #[serde(default)]
    pub threshold: f64,
    #[serde(rename = "ThresholdType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateBudgetResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateNotificationRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "BudgetName")]
    #[serde(default)]
    pub budget_name: String,
    #[serde(rename = "Notification")]
    #[serde(default)]
    pub notification: Notification,
    #[serde(rename = "Subscribers")]
    #[serde(default)]
    pub subscribers: Vec<Subscriber>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateNotificationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSubscriberRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "BudgetName")]
    #[serde(default)]
    pub budget_name: String,
    #[serde(rename = "Notification")]
    #[serde(default)]
    pub notification: Notification,
    #[serde(rename = "Subscriber")]
    #[serde(default)]
    pub subscriber: Subscriber,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSubscriberResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteBudgetActionRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "ActionId")]
    #[serde(default)]
    pub action_id: String,
    #[serde(rename = "BudgetName")]
    #[serde(default)]
    pub budget_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteBudgetActionResponse {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "Action")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<Action>,
    #[serde(rename = "BudgetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Action {
    #[serde(rename = "ActionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_id: Option<String>,
    #[serde(rename = "ActionThreshold")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_threshold: Option<ActionThreshold>,
    #[serde(rename = "ActionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_type: Option<String>,
    #[serde(rename = "ApprovalModel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_model: Option<String>,
    #[serde(rename = "BudgetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_name: Option<String>,
    #[serde(rename = "Definition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<Definition>,
    #[serde(rename = "ExecutionRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role_arn: Option<String>,
    #[serde(rename = "NotificationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_type: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Subscribers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribers: Option<Vec<Subscriber>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteBudgetRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "BudgetName")]
    #[serde(default)]
    pub budget_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteBudgetResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteNotificationRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "BudgetName")]
    #[serde(default)]
    pub budget_name: String,
    #[serde(rename = "Notification")]
    #[serde(default)]
    pub notification: Notification,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteNotificationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSubscriberRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "BudgetName")]
    #[serde(default)]
    pub budget_name: String,
    #[serde(rename = "Notification")]
    #[serde(default)]
    pub notification: Notification,
    #[serde(rename = "Subscriber")]
    #[serde(default)]
    pub subscriber: Subscriber,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSubscriberResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeBudgetActionHistoriesRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "ActionId")]
    #[serde(default)]
    pub action_id: String,
    #[serde(rename = "BudgetName")]
    #[serde(default)]
    pub budget_name: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TimePeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_period: Option<TimePeriod>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeBudgetActionHistoriesResponse {
    #[serde(rename = "ActionHistories")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_histories: Option<Vec<ActionHistory>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActionHistory {
    #[serde(rename = "ActionHistoryDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_history_details: Option<ActionHistoryDetails>,
    #[serde(rename = "EventType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Timestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActionHistoryDetails {
    #[serde(rename = "Action")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<Action>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeBudgetActionRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "ActionId")]
    #[serde(default)]
    pub action_id: String,
    #[serde(rename = "BudgetName")]
    #[serde(default)]
    pub budget_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeBudgetActionResponse {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "Action")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<Action>,
    #[serde(rename = "BudgetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeBudgetActionsForAccountRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
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
pub struct DescribeBudgetActionsForAccountResponse {
    #[serde(rename = "Actions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<Action>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeBudgetActionsForBudgetRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "BudgetName")]
    #[serde(default)]
    pub budget_name: String,
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
pub struct DescribeBudgetActionsForBudgetResponse {
    #[serde(rename = "Actions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<Action>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeBudgetNotificationsForAccountRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
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
pub struct DescribeBudgetNotificationsForAccountResponse {
    #[serde(rename = "BudgetNotificationsForAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_notifications_for_account: Option<Vec<BudgetNotificationsForAccount>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BudgetNotificationsForAccount {
    #[serde(rename = "BudgetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_name: Option<String>,
    #[serde(rename = "Notifications")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notifications: Option<Vec<Notification>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeBudgetPerformanceHistoryRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "BudgetName")]
    #[serde(default)]
    pub budget_name: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TimePeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_period: Option<TimePeriod>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeBudgetPerformanceHistoryResponse {
    #[serde(rename = "BudgetPerformanceHistory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_performance_history: Option<BudgetPerformanceHistory>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BudgetPerformanceHistory {
    #[serde(rename = "BillingViewArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_view_arn: Option<String>,
    #[serde(rename = "BudgetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_name: Option<String>,
    #[serde(rename = "BudgetType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_type: Option<String>,
    #[serde(rename = "BudgetedAndActualAmountsList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budgeted_and_actual_amounts_list: Option<Vec<BudgetedAndActualAmounts>>,
    #[serde(rename = "CostFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_filters: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "CostTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_types: Option<CostTypes>,
    #[serde(rename = "FilterExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_expression: Option<Expression>,
    #[serde(rename = "Metrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Vec<String>>,
    #[serde(rename = "TimeUnit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_unit: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BudgetedAndActualAmounts {
    #[serde(rename = "ActualAmount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_amount: Option<Spend>,
    #[serde(rename = "BudgetedAmount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budgeted_amount: Option<Spend>,
    #[serde(rename = "TimePeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_period: Option<TimePeriod>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeBudgetRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "BudgetName")]
    #[serde(default)]
    pub budget_name: String,
    #[serde(rename = "ShowFilterExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_filter_expression: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeBudgetResponse {
    #[serde(rename = "Budget")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget: Option<Budget>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeBudgetsRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ShowFilterExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_filter_expression: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeBudgetsResponse {
    #[serde(rename = "Budgets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budgets: Option<Vec<Budget>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeNotificationsForBudgetRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "BudgetName")]
    #[serde(default)]
    pub budget_name: String,
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
pub struct DescribeNotificationsForBudgetResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Notifications")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notifications: Option<Vec<Notification>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSubscribersForNotificationRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "BudgetName")]
    #[serde(default)]
    pub budget_name: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Notification")]
    #[serde(default)]
    pub notification: Notification,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSubscribersForNotificationResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Subscribers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribers: Option<Vec<Subscriber>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExecuteBudgetActionRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "ActionId")]
    #[serde(default)]
    pub action_id: String,
    #[serde(rename = "BudgetName")]
    #[serde(default)]
    pub budget_name: String,
    #[serde(rename = "ExecutionType")]
    #[serde(default)]
    pub execution_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExecuteBudgetActionResponse {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "ActionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_id: Option<String>,
    #[serde(rename = "BudgetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_name: Option<String>,
    #[serde(rename = "ExecutionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(rename = "ResourceTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_tags: Option<Vec<ResourceTag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
    #[serde(rename = "ResourceTags")]
    #[serde(default)]
    pub resource_tags: Vec<ResourceTag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
    #[serde(rename = "ResourceTagKeys")]
    #[serde(default)]
    pub resource_tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateBudgetActionRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "ActionId")]
    #[serde(default)]
    pub action_id: String,
    #[serde(rename = "ActionThreshold")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_threshold: Option<ActionThreshold>,
    #[serde(rename = "ApprovalModel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_model: Option<String>,
    #[serde(rename = "BudgetName")]
    #[serde(default)]
    pub budget_name: String,
    #[serde(rename = "Definition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<Definition>,
    #[serde(rename = "ExecutionRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role_arn: Option<String>,
    #[serde(rename = "NotificationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_type: Option<String>,
    #[serde(rename = "Subscribers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribers: Option<Vec<Subscriber>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateBudgetActionResponse {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "BudgetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_name: Option<String>,
    #[serde(rename = "NewAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_action: Option<Action>,
    #[serde(rename = "OldAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_action: Option<Action>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateBudgetRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "NewBudget")]
    #[serde(default)]
    pub new_budget: Budget,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateBudgetResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateNotificationRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "BudgetName")]
    #[serde(default)]
    pub budget_name: String,
    #[serde(rename = "NewNotification")]
    #[serde(default)]
    pub new_notification: Notification,
    #[serde(rename = "OldNotification")]
    #[serde(default)]
    pub old_notification: Notification,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateNotificationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSubscriberRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "BudgetName")]
    #[serde(default)]
    pub budget_name: String,
    #[serde(rename = "NewSubscriber")]
    #[serde(default)]
    pub new_subscriber: Subscriber,
    #[serde(rename = "Notification")]
    #[serde(default)]
    pub notification: Notification,
    #[serde(rename = "OldSubscriber")]
    #[serde(default)]
    pub old_subscriber: Subscriber,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSubscriberResponse {}
