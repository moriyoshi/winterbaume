//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-budgets

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for awsJson protocol.
pub fn serialize_create_budget_response(result: &CreateBudgetResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_budget_action_response(
    result: &CreateBudgetActionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_notification_response(result: &CreateNotificationResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_create_subscriber_response(result: &CreateSubscriberResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_budget_response(result: &DeleteBudgetResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_budget_action_response(
    result: &DeleteBudgetActionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_notification_response(result: &DeleteNotificationResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_subscriber_response(result: &DeleteSubscriberResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_budget_response(result: &DescribeBudgetResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_budget_action_response(
    result: &DescribeBudgetActionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_budget_action_histories_response(
    result: &DescribeBudgetActionHistoriesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_budget_actions_for_account_response(
    result: &DescribeBudgetActionsForAccountResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_budget_actions_for_budget_response(
    result: &DescribeBudgetActionsForBudgetResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_budget_notifications_for_account_response(
    result: &DescribeBudgetNotificationsForAccountResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_budget_performance_history_response(
    result: &DescribeBudgetPerformanceHistoryResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_budgets_response(result: &DescribeBudgetsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_notifications_for_budget_response(
    result: &DescribeNotificationsForBudgetResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_subscribers_for_notification_response(
    result: &DescribeSubscribersForNotificationResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_execute_budget_action_response(
    result: &ExecuteBudgetActionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_tags_for_resource_response(
    result: &ListTagsForResourceResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_tag_resource_response(result: &TagResourceResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_untag_resource_response(result: &UntagResourceResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_budget_response(result: &UpdateBudgetResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_budget_action_response(
    result: &UpdateBudgetActionResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_notification_response(result: &UpdateNotificationResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_subscriber_response(result: &UpdateSubscriberResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_budget_request(body: &[u8]) -> Result<CreateBudgetRequest, String> {
    if body.is_empty() {
        return Ok(CreateBudgetRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateBudget request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_budget_action_request(
    body: &[u8],
) -> Result<CreateBudgetActionRequest, String> {
    if body.is_empty() {
        return Ok(CreateBudgetActionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateBudgetAction request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_notification_request(
    body: &[u8],
) -> Result<CreateNotificationRequest, String> {
    if body.is_empty() {
        return Ok(CreateNotificationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateNotification request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_subscriber_request(
    body: &[u8],
) -> Result<CreateSubscriberRequest, String> {
    if body.is_empty() {
        return Ok(CreateSubscriberRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateSubscriber request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_budget_request(body: &[u8]) -> Result<DeleteBudgetRequest, String> {
    if body.is_empty() {
        return Ok(DeleteBudgetRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteBudget request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_budget_action_request(
    body: &[u8],
) -> Result<DeleteBudgetActionRequest, String> {
    if body.is_empty() {
        return Ok(DeleteBudgetActionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteBudgetAction request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_notification_request(
    body: &[u8],
) -> Result<DeleteNotificationRequest, String> {
    if body.is_empty() {
        return Ok(DeleteNotificationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteNotification request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_subscriber_request(
    body: &[u8],
) -> Result<DeleteSubscriberRequest, String> {
    if body.is_empty() {
        return Ok(DeleteSubscriberRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteSubscriber request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_budget_request(body: &[u8]) -> Result<DescribeBudgetRequest, String> {
    if body.is_empty() {
        return Ok(DescribeBudgetRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeBudget request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_budget_action_request(
    body: &[u8],
) -> Result<DescribeBudgetActionRequest, String> {
    if body.is_empty() {
        return Ok(DescribeBudgetActionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeBudgetAction request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_budget_action_histories_request(
    body: &[u8],
) -> Result<DescribeBudgetActionHistoriesRequest, String> {
    if body.is_empty() {
        return Ok(DescribeBudgetActionHistoriesRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeBudgetActionHistories request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_budget_actions_for_account_request(
    body: &[u8],
) -> Result<DescribeBudgetActionsForAccountRequest, String> {
    if body.is_empty() {
        return Ok(DescribeBudgetActionsForAccountRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeBudgetActionsForAccount request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_budget_actions_for_budget_request(
    body: &[u8],
) -> Result<DescribeBudgetActionsForBudgetRequest, String> {
    if body.is_empty() {
        return Ok(DescribeBudgetActionsForBudgetRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeBudgetActionsForBudget request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_budget_notifications_for_account_request(
    body: &[u8],
) -> Result<DescribeBudgetNotificationsForAccountRequest, String> {
    if body.is_empty() {
        return Ok(DescribeBudgetNotificationsForAccountRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DescribeBudgetNotificationsForAccount request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_budget_performance_history_request(
    body: &[u8],
) -> Result<DescribeBudgetPerformanceHistoryRequest, String> {
    if body.is_empty() {
        return Ok(DescribeBudgetPerformanceHistoryRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeBudgetPerformanceHistory request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_budgets_request(body: &[u8]) -> Result<DescribeBudgetsRequest, String> {
    if body.is_empty() {
        return Ok(DescribeBudgetsRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeBudgets request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_notifications_for_budget_request(
    body: &[u8],
) -> Result<DescribeNotificationsForBudgetRequest, String> {
    if body.is_empty() {
        return Ok(DescribeNotificationsForBudgetRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeNotificationsForBudget request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_subscribers_for_notification_request(
    body: &[u8],
) -> Result<DescribeSubscribersForNotificationRequest, String> {
    if body.is_empty() {
        return Ok(DescribeSubscribersForNotificationRequest::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize DescribeSubscribersForNotification request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_execute_budget_action_request(
    body: &[u8],
) -> Result<ExecuteBudgetActionRequest, String> {
    if body.is_empty() {
        return Ok(ExecuteBudgetActionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ExecuteBudgetAction request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_tags_for_resource_request(
    body: &[u8],
) -> Result<ListTagsForResourceRequest, String> {
    if body.is_empty() {
        return Ok(ListTagsForResourceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListTagsForResource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_tag_resource_request(body: &[u8]) -> Result<TagResourceRequest, String> {
    if body.is_empty() {
        return Ok(TagResourceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize TagResource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_untag_resource_request(body: &[u8]) -> Result<UntagResourceRequest, String> {
    if body.is_empty() {
        return Ok(UntagResourceRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UntagResource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_budget_request(body: &[u8]) -> Result<UpdateBudgetRequest, String> {
    if body.is_empty() {
        return Ok(UpdateBudgetRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateBudget request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_budget_action_request(
    body: &[u8],
) -> Result<UpdateBudgetActionRequest, String> {
    if body.is_empty() {
        return Ok(UpdateBudgetActionRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateBudgetAction request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_notification_request(
    body: &[u8],
) -> Result<UpdateNotificationRequest, String> {
    if body.is_empty() {
        return Ok(UpdateNotificationRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateNotification request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_subscriber_request(
    body: &[u8],
) -> Result<UpdateSubscriberRequest, String> {
    if body.is_empty() {
        return Ok(UpdateSubscriberRequest::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateSubscriber request: {e}"))
}
