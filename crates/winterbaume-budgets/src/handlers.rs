use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, default_account_id,
    json_error_response,
};

use crate::state::{BudgetsError, BudgetsState};
use crate::views::BudgetsStateView;
use crate::wire;

pub struct BudgetsService {
    pub(crate) state: Arc<BackendState<BudgetsState>>,
    pub(crate) notifier: StateChangeNotifier<BudgetsStateView>,
}

impl BudgetsService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for BudgetsService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for BudgetsService {
    fn service_name(&self) -> &str {
        "budgets"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://budgets\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl BudgetsService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();

        let action = request
            .headers
            .get("x-amz-target")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.split('.').next_back())
            .map(|s| s.to_string());

        let action = match action {
            Some(a) => a,
            None => {
                return json_error_response(400, "MissingAction", "Missing X-Amz-Target header");
            }
        };

        if serde_json::from_slice::<Value>(&request.body).is_err() {
            return json_error_response(400, "SerializationException", "Invalid JSON body");
        }
        let body_bytes: &[u8] = &request.body;

        let state = self.state.get(account_id, &region);

        match action.as_str() {
            "CreateBudget" => self.handle_create_budget(&state, body_bytes).await,
            "DescribeBudget" => self.handle_describe_budget(&state, body_bytes).await,
            "DescribeBudgets" => self.handle_describe_budgets(&state).await,
            "DeleteBudget" => self.handle_delete_budget(&state, body_bytes).await,
            // --- Unimplemented operations (auto-generated stubs) ---
            "CreateBudgetAction" => json_error_response(
                501,
                "NotImplementedError",
                "CreateBudgetAction is not yet implemented in winterbaume-budgets",
            ),
            "CreateNotification" => self.handle_create_notification(&state, body_bytes).await,
            "CreateSubscriber" => json_error_response(
                501,
                "NotImplementedError",
                "CreateSubscriber is not yet implemented in winterbaume-budgets",
            ),
            "DeleteBudgetAction" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteBudgetAction is not yet implemented in winterbaume-budgets",
            ),
            "DeleteNotification" => self.handle_delete_notification(&state, body_bytes).await,
            "DeleteSubscriber" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteSubscriber is not yet implemented in winterbaume-budgets",
            ),
            "DescribeBudgetAction" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeBudgetAction is not yet implemented in winterbaume-budgets",
            ),
            "DescribeBudgetActionHistories" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeBudgetActionHistories is not yet implemented in winterbaume-budgets",
            ),
            "DescribeBudgetActionsForAccount" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeBudgetActionsForAccount is not yet implemented in winterbaume-budgets",
            ),
            "DescribeBudgetActionsForBudget" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeBudgetActionsForBudget is not yet implemented in winterbaume-budgets",
            ),
            "DescribeBudgetNotificationsForAccount" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeBudgetNotificationsForAccount is not yet implemented in winterbaume-budgets",
            ),
            "DescribeBudgetPerformanceHistory" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeBudgetPerformanceHistory is not yet implemented in winterbaume-budgets",
            ),
            "DescribeNotificationsForBudget" => {
                self.handle_describe_notifications_for_budget(&state, body_bytes)
                    .await
            }
            "DescribeSubscribersForNotification" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeSubscribersForNotification is not yet implemented in winterbaume-budgets",
            ),
            "ExecuteBudgetAction" => json_error_response(
                501,
                "NotImplementedError",
                "ExecuteBudgetAction is not yet implemented in winterbaume-budgets",
            ),
            "ListTagsForResource" => json_error_response(
                501,
                "NotImplementedError",
                "ListTagsForResource is not yet implemented in winterbaume-budgets",
            ),
            "TagResource" => json_error_response(
                501,
                "NotImplementedError",
                "TagResource is not yet implemented in winterbaume-budgets",
            ),
            "UntagResource" => json_error_response(
                501,
                "NotImplementedError",
                "UntagResource is not yet implemented in winterbaume-budgets",
            ),
            "UpdateBudget" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateBudget is not yet implemented in winterbaume-budgets",
            ),
            "UpdateBudgetAction" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateBudgetAction is not yet implemented in winterbaume-budgets",
            ),
            "UpdateNotification" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateNotification is not yet implemented in winterbaume-budgets",
            ),
            "UpdateSubscriber" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateSubscriber is not yet implemented in winterbaume-budgets",
            ),
            _ => json_error_response(
                400,
                "InvalidAction",
                &format!("Could not find operation {action} for Budgets"),
            ),
        }
    }

    async fn handle_create_budget(
        &self,
        state: &Arc<tokio::sync::RwLock<BudgetsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_create_budget_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let budget = &input.budget;
        if budget.budget_name.is_empty() {
            return json_error_response(400, "InvalidParameterException", "BudgetName is required");
        }
        let name = budget.budget_name.as_str();
        let budget_type = if budget.budget_type.is_empty() {
            "COST"
        } else {
            budget.budget_type.as_str()
        };
        let time_unit = if budget.time_unit.is_empty() {
            "MONTHLY"
        } else {
            budget.time_unit.as_str()
        };
        let (limit_amount, limit_unit) = match &budget.budget_limit {
            Some(spend) => {
                let amount = if spend.amount.is_empty() {
                    "0"
                } else {
                    spend.amount.as_str()
                };
                let unit = if spend.unit.is_empty() {
                    "USD"
                } else {
                    spend.unit.as_str()
                };
                (amount, unit)
            }
            None => ("0", "USD"),
        };

        let mut state = state.write().await;
        match state.create_budget(name, budget_type, limit_amount, limit_unit, time_unit) {
            Ok(()) => {
                // Process inline NotificationsWithSubscribers if present.
                if let Some(notifs) = input.notifications_with_subscribers.as_ref() {
                    if let Some(budget) = state.budgets.get_mut(name) {
                        for entry in notifs {
                            let notification = &entry.notification;
                            let notification_type = if notification.notification_type.is_empty() {
                                "ACTUAL".to_string()
                            } else {
                                notification.notification_type.clone()
                            };
                            let comparison_operator = if notification.comparison_operator.is_empty()
                            {
                                "GREATER_THAN".to_string()
                            } else {
                                notification.comparison_operator.clone()
                            };
                            let threshold = if notification.threshold == 0.0 {
                                80.0
                            } else {
                                notification.threshold
                            };
                            let threshold_type = notification
                                .threshold_type
                                .clone()
                                .unwrap_or_else(|| "PERCENTAGE".to_string());
                            budget.notifications.push(crate::types::Notification {
                                notification_type,
                                comparison_operator,
                                threshold,
                                threshold_type,
                            });
                        }
                    }
                }
                wire::serialize_create_budget_response(&wire::CreateBudgetResponse {})
            }
            Err(e) => budgets_error_response(&e),
        }
    }

    async fn handle_describe_budget(
        &self,
        state: &Arc<tokio::sync::RwLock<BudgetsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_budget_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.budget_name.is_empty() {
            return json_error_response(400, "InvalidParameterException", "BudgetName is required");
        }
        let name = input.budget_name.as_str();

        let state = state.read().await;
        match state.describe_budget(name) {
            Ok(budget) => wire::serialize_describe_budget_response(&wire::DescribeBudgetResponse {
                budget: Some(budget_to_wire(budget)),
            }),
            Err(e) => budgets_error_response(&e),
        }
    }

    async fn handle_describe_budgets(
        &self,
        state: &Arc<tokio::sync::RwLock<BudgetsState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let budgets = state.describe_budgets();
        let entries: Vec<wire::Budget> = budgets.iter().map(|b| budget_to_wire(b)).collect();

        wire::serialize_describe_budgets_response(&wire::DescribeBudgetsResponse {
            budgets: Some(entries),
            next_token: None,
        })
    }

    async fn handle_delete_budget(
        &self,
        state: &Arc<tokio::sync::RwLock<BudgetsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_budget_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.budget_name.is_empty() {
            return json_error_response(400, "InvalidParameterException", "BudgetName is required");
        }
        let name = input.budget_name.as_str();

        let mut state = state.write().await;
        match state.delete_budget(name) {
            Ok(()) => wire::serialize_delete_budget_response(&wire::DeleteBudgetResponse {}),
            Err(e) => budgets_error_response(&e),
        }
    }

    async fn handle_create_notification(
        &self,
        state: &Arc<tokio::sync::RwLock<BudgetsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_create_notification_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.budget_name.is_empty() {
            return json_error_response(400, "InvalidParameterException", "BudgetName is required");
        }
        let budget_name = input.budget_name.as_str();
        let notification = &input.notification;

        let notification_type = if notification.notification_type.is_empty() {
            "ACTUAL".to_string()
        } else {
            notification.notification_type.clone()
        };
        let comparison_operator = if notification.comparison_operator.is_empty() {
            "GREATER_THAN".to_string()
        } else {
            notification.comparison_operator.clone()
        };
        let threshold = if notification.threshold == 0.0 {
            80.0
        } else {
            notification.threshold
        };
        let threshold_type = notification
            .threshold_type
            .clone()
            .unwrap_or_else(|| "PERCENTAGE".to_string());

        let mut state = state.write().await;
        match state.budgets.get_mut(budget_name) {
            Some(budget) => {
                let duplicate = budget.notifications.iter().any(|n| {
                    n.notification_type == notification_type
                        && n.comparison_operator == comparison_operator
                        && (n.threshold - threshold).abs() < f64::EPSILON
                });
                if duplicate {
                    return budgets_error_response(&BudgetsError::DuplicateNotification {
                        budget_name: budget_name.to_string(),
                    });
                }
                budget.notifications.push(crate::types::Notification {
                    notification_type,
                    comparison_operator,
                    threshold,
                    threshold_type,
                });
                wire::serialize_create_notification_response(&wire::CreateNotificationResponse {})
            }
            None => budgets_error_response(&BudgetsError::BudgetNotFound {
                name: budget_name.to_string(),
            }),
        }
    }

    async fn handle_delete_notification(
        &self,
        state: &Arc<tokio::sync::RwLock<BudgetsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_notification_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.budget_name.is_empty() {
            return json_error_response(400, "InvalidParameterException", "BudgetName is required");
        }
        let budget_name = input.budget_name.as_str();
        let notification = &input.notification;

        let notification_type = notification.notification_type.as_str();
        let comparison_operator = notification.comparison_operator.as_str();
        let threshold = notification.threshold;

        let mut state = state.write().await;
        match state.budgets.get_mut(budget_name) {
            Some(budget) => {
                let before = budget.notifications.len();
                budget.notifications.retain(|n| {
                    !(n.notification_type == notification_type
                        && n.comparison_operator == comparison_operator
                        && (n.threshold - threshold).abs() < f64::EPSILON)
                });
                if budget.notifications.len() == before {
                    return budgets_error_response(&BudgetsError::NotificationNotFound);
                }
                wire::serialize_delete_notification_response(&wire::DeleteNotificationResponse {})
            }
            None => budgets_error_response(&BudgetsError::BudgetNotFound {
                name: budget_name.to_string(),
            }),
        }
    }

    async fn handle_describe_notifications_for_budget(
        &self,
        state: &Arc<tokio::sync::RwLock<BudgetsState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_notifications_for_budget_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.budget_name.is_empty() {
            return json_error_response(400, "InvalidParameterException", "BudgetName is required");
        }
        let budget_name = input.budget_name.as_str();

        let state = state.read().await;
        match state.budgets.get(budget_name) {
            Some(budget) => {
                let entries: Vec<wire::Notification> = budget
                    .notifications
                    .iter()
                    .map(notification_to_wire)
                    .collect();
                wire::serialize_describe_notifications_for_budget_response(
                    &wire::DescribeNotificationsForBudgetResponse {
                        notifications: Some(entries),
                        next_token: None,
                    },
                )
            }
            None => budgets_error_response(&BudgetsError::BudgetNotFound {
                name: budget_name.to_string(),
            }),
        }
    }
}

fn budget_to_wire(budget: &crate::types::Budget) -> wire::Budget {
    wire::Budget {
        budget_name: budget.budget_name.clone(),
        budget_type: budget.budget_type.clone(),
        budget_limit: Some(wire::Spend {
            amount: budget.budget_limit_amount.clone(),
            unit: budget.budget_limit_unit.clone(),
        }),
        time_unit: budget.time_unit.clone(),
        ..Default::default()
    }
}

fn notification_to_wire(n: &crate::types::Notification) -> wire::Notification {
    wire::Notification {
        notification_type: n.notification_type.clone(),
        comparison_operator: n.comparison_operator.clone(),
        threshold: n.threshold,
        threshold_type: Some(n.threshold_type.clone()),
        ..Default::default()
    }
}

fn budgets_error_response(err: &BudgetsError) -> MockResponse {
    let (status, error_type) = match err {
        BudgetsError::DuplicateBudget { .. } => (400, "DuplicateRecordException"),
        BudgetsError::BudgetNotFound { .. } => (400, "NotFoundException"),
        BudgetsError::DuplicateNotification { .. } => (400, "DuplicateRecordException"),
        BudgetsError::NotificationNotFound => (400, "NotFoundException"),
    };
    let body = json!({
        "__type": error_type,
        "message": err.to_string(),
    });
    MockResponse::json(status, body.to_string())
}
