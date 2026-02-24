use aws_sdk_budgets::config::BehaviorVersion;
use aws_sdk_budgets::types::{
    Budget, BudgetType, ComparisonOperator, Notification, NotificationType, Spend, ThresholdType,
    TimeUnit,
};
use winterbaume_budgets::BudgetsService;
use winterbaume_core::MockAws;

async fn make_client() -> aws_sdk_budgets::Client {
    let mock = MockAws::builder()
        .with_service(BudgetsService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_budgets::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_budgets::Client::new(&config)
}

fn test_budget(name: &str, amount: &str) -> Budget {
    Budget::builder()
        .budget_name(name)
        .budget_type(BudgetType::Cost)
        .budget_limit(Spend::builder().amount(amount).unit("USD").build().unwrap())
        .time_unit(TimeUnit::Monthly)
        .build()
        .unwrap()
}

#[tokio::test]
async fn test_create_and_describe_budget() {
    let client = make_client().await;

    client
        .create_budget()
        .account_id("123456789012")
        .budget(test_budget("my-budget", "100"))
        .send()
        .await
        .expect("create_budget should succeed");

    let resp = client
        .describe_budget()
        .account_id("123456789012")
        .budget_name("my-budget")
        .send()
        .await
        .expect("describe_budget should succeed");

    let budget = resp.budget().expect("should have budget");
    assert_eq!(budget.budget_name(), "my-budget");
}

#[tokio::test]
async fn test_describe_budgets() {
    let client = make_client().await;

    for name in ["budget-a", "budget-b"] {
        client
            .create_budget()
            .account_id("123456789012")
            .budget(test_budget(name, "50"))
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .describe_budgets()
        .account_id("123456789012")
        .send()
        .await
        .expect("describe_budgets should succeed");

    assert_eq!(resp.budgets().len(), 2);
}

#[tokio::test]
async fn test_delete_budget() {
    let client = make_client().await;

    client
        .create_budget()
        .account_id("123456789012")
        .budget(test_budget("delete-me", "200"))
        .send()
        .await
        .unwrap();

    client
        .delete_budget()
        .account_id("123456789012")
        .budget_name("delete-me")
        .send()
        .await
        .expect("delete_budget should succeed");

    let result = client
        .describe_budget()
        .account_id("123456789012")
        .budget_name("delete-me")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_describe_nonexistent_budget() {
    let client = make_client().await;

    let result = client
        .describe_budget()
        .account_id("123456789012")
        .budget_name("nonexistent-budget")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_describe_budgets_empty() {
    let client = make_client().await;

    let resp = client
        .describe_budgets()
        .account_id("123456789012")
        .send()
        .await
        .expect("describe_budgets should succeed on empty state");

    assert_eq!(resp.budgets().len(), 0);
}

#[tokio::test]
async fn test_delete_nonexistent_budget() {
    let client = make_client().await;

    let result = client
        .delete_budget()
        .account_id("123456789012")
        .budget_name("nonexistent-budget")
        .send()
        .await;
    assert!(result.is_err());
}

fn test_notification() -> Notification {
    Notification::builder()
        .notification_type(NotificationType::Actual)
        .comparison_operator(ComparisonOperator::GreaterThan)
        .threshold(80.0)
        .threshold_type(ThresholdType::Percentage)
        .build()
        .unwrap()
}

#[tokio::test]
async fn test_create_existing_budget() {
    let client = make_client().await;

    client
        .create_budget()
        .account_id("123456789012")
        .budget(test_budget("duplicate-budget", "100"))
        .send()
        .await
        .expect("first create should succeed");

    // Creating the same budget again should fail with DuplicateRecordException
    let result = client
        .create_budget()
        .account_id("123456789012")
        .budget(test_budget("duplicate-budget", "200"))
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_create_notification() {
    let client = make_client().await;

    client
        .create_budget()
        .account_id("123456789012")
        .budget(test_budget("notify-budget", "100"))
        .send()
        .await
        .unwrap();

    client
        .create_notification()
        .account_id("123456789012")
        .budget_name("notify-budget")
        .notification(test_notification())
        .subscribers(
            aws_sdk_budgets::types::Subscriber::builder()
                .subscription_type(aws_sdk_budgets::types::SubscriptionType::Email)
                .address("test@example.com")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create_notification should succeed");

    let resp = client
        .describe_notifications_for_budget()
        .account_id("123456789012")
        .budget_name("notify-budget")
        .send()
        .await
        .expect("describe_notifications should succeed");

    assert_eq!(resp.notifications().len(), 1);
}

#[tokio::test]
async fn test_create_notification_unknown_budget() {
    let client = make_client().await;

    let result = client
        .create_notification()
        .account_id("123456789012")
        .budget_name("nonexistent-budget")
        .notification(test_notification())
        .subscribers(
            aws_sdk_budgets::types::Subscriber::builder()
                .subscription_type(aws_sdk_budgets::types::SubscriptionType::Email)
                .address("test@example.com")
                .build()
                .unwrap(),
        )
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_delete_notification() {
    let client = make_client().await;

    client
        .create_budget()
        .account_id("123456789012")
        .budget(test_budget("notify-del-budget", "100"))
        .send()
        .await
        .unwrap();

    let notif = test_notification();

    client
        .create_notification()
        .account_id("123456789012")
        .budget_name("notify-del-budget")
        .notification(notif.clone())
        .subscribers(
            aws_sdk_budgets::types::Subscriber::builder()
                .subscription_type(aws_sdk_budgets::types::SubscriptionType::Email)
                .address("test@example.com")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    client
        .delete_notification()
        .account_id("123456789012")
        .budget_name("notify-del-budget")
        .notification(notif)
        .send()
        .await
        .expect("delete_notification should succeed");

    let resp = client
        .describe_notifications_for_budget()
        .account_id("123456789012")
        .budget_name("notify-del-budget")
        .send()
        .await
        .expect("describe_notifications should succeed");

    assert_eq!(resp.notifications().len(), 0);
}

#[tokio::test]
async fn test_delete_notification_unknown_budget() {
    let client = make_client().await;

    let result = client
        .delete_notification()
        .account_id("123456789012")
        .budget_name("nonexistent-budget")
        .notification(test_notification())
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_describe_notifications_for_budget_empty() {
    let client = make_client().await;

    client
        .create_budget()
        .account_id("123456789012")
        .budget(test_budget("empty-notify-budget", "100"))
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_notifications_for_budget()
        .account_id("123456789012")
        .budget_name("empty-notify-budget")
        .send()
        .await
        .expect("describe_notifications should succeed on empty list");

    assert_eq!(resp.notifications().len(), 0);
}

// ============================================================================
// Tests derived from AWS documentation: AWS Budgets
// (write-tests skill, 2026-03-28)
// ============================================================================

#[tokio::test]
async fn test_describe_notifications_for_nonexistent_budget() {
    let client = make_client().await;

    let result = client
        .describe_notifications_for_budget()
        .account_id("123456789012")
        .budget_name("nonexistent-budget")
        .send()
        .await;

    assert!(
        result.is_err(),
        "describe_notifications for nonexistent budget should fail"
    );
}

#[tokio::test]
async fn test_multiple_notifications_for_budget() {
    let client = make_client().await;

    client
        .create_budget()
        .account_id("123456789012")
        .budget(test_budget("multi-notify-budget", "100"))
        .send()
        .await
        .unwrap();

    let notif1 = Notification::builder()
        .notification_type(NotificationType::Actual)
        .comparison_operator(ComparisonOperator::GreaterThan)
        .threshold(80.0)
        .threshold_type(ThresholdType::Percentage)
        .build()
        .unwrap();

    let notif2 = Notification::builder()
        .notification_type(NotificationType::Forecasted)
        .comparison_operator(ComparisonOperator::GreaterThan)
        .threshold(100.0)
        .threshold_type(ThresholdType::Percentage)
        .build()
        .unwrap();

    client
        .create_notification()
        .account_id("123456789012")
        .budget_name("multi-notify-budget")
        .notification(notif1)
        .subscribers(
            aws_sdk_budgets::types::Subscriber::builder()
                .subscription_type(aws_sdk_budgets::types::SubscriptionType::Email)
                .address("test1@example.com")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    client
        .create_notification()
        .account_id("123456789012")
        .budget_name("multi-notify-budget")
        .notification(notif2)
        .subscribers(
            aws_sdk_budgets::types::Subscriber::builder()
                .subscription_type(aws_sdk_budgets::types::SubscriptionType::Email)
                .address("test2@example.com")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_notifications_for_budget()
        .account_id("123456789012")
        .budget_name("multi-notify-budget")
        .send()
        .await
        .expect("describe_notifications should succeed");

    assert_eq!(resp.notifications().len(), 2);
}

#[tokio::test]
async fn test_delete_one_of_multiple_notifications() {
    let client = make_client().await;

    client
        .create_budget()
        .account_id("123456789012")
        .budget(test_budget("del-one-budget", "100"))
        .send()
        .await
        .unwrap();

    let notif1 = Notification::builder()
        .notification_type(NotificationType::Actual)
        .comparison_operator(ComparisonOperator::GreaterThan)
        .threshold(80.0)
        .threshold_type(ThresholdType::Percentage)
        .build()
        .unwrap();

    let notif2 = Notification::builder()
        .notification_type(NotificationType::Forecasted)
        .comparison_operator(ComparisonOperator::GreaterThan)
        .threshold(100.0)
        .threshold_type(ThresholdType::Percentage)
        .build()
        .unwrap();

    let sub = aws_sdk_budgets::types::Subscriber::builder()
        .subscription_type(aws_sdk_budgets::types::SubscriptionType::Email)
        .address("sub@example.com")
        .build()
        .unwrap();

    client
        .create_notification()
        .account_id("123456789012")
        .budget_name("del-one-budget")
        .notification(notif1.clone())
        .subscribers(sub.clone())
        .send()
        .await
        .unwrap();

    client
        .create_notification()
        .account_id("123456789012")
        .budget_name("del-one-budget")
        .notification(notif2)
        .subscribers(sub)
        .send()
        .await
        .unwrap();

    client
        .delete_notification()
        .account_id("123456789012")
        .budget_name("del-one-budget")
        .notification(notif1)
        .send()
        .await
        .expect("delete_notification should succeed");

    let resp = client
        .describe_notifications_for_budget()
        .account_id("123456789012")
        .budget_name("del-one-budget")
        .send()
        .await
        .unwrap();

    assert_eq!(resp.notifications().len(), 1);
}

// ============================================================================
// Additional tests derived from AWS documentation: AWS Budgets
// Source: https://docs.aws.amazon.com/aws-cost-management/latest/APIReference/API_Operations_AWS_Budgets.html
// ============================================================================

// Scenario 1: Verify all key response fields are correctly populated in DescribeBudget
#[tokio::test]
async fn test_describe_budget_fields() {
    let client = make_client().await;

    client
        .create_budget()
        .account_id("123456789012")
        .budget(test_budget("fields-budget", "500"))
        .send()
        .await
        .expect("create_budget should succeed");

    let resp = client
        .describe_budget()
        .account_id("123456789012")
        .budget_name("fields-budget")
        .send()
        .await
        .expect("describe_budget should succeed");

    let budget = resp.budget().expect("should have budget");
    assert_eq!(budget.budget_name(), "fields-budget");
    assert_eq!(budget.budget_type(), &BudgetType::Cost);
    assert_eq!(budget.time_unit(), &TimeUnit::Monthly);

    let limit = budget.budget_limit().expect("should have budget_limit");
    assert_eq!(limit.amount(), "500");
    assert_eq!(limit.unit(), "USD");
}

// Scenario 2: Verify fields in DescribeBudgets list match the created budget
#[tokio::test]
async fn test_describe_budgets_fields() {
    let client = make_client().await;

    client
        .create_budget()
        .account_id("123456789012")
        .budget(test_budget("list-fields-budget", "250"))
        .send()
        .await
        .expect("create_budget should succeed");

    let resp = client
        .describe_budgets()
        .account_id("123456789012")
        .send()
        .await
        .expect("describe_budgets should succeed");

    let budgets = resp.budgets();
    assert_eq!(budgets.len(), 1);
    let budget = &budgets[0];
    assert_eq!(budget.budget_name(), "list-fields-budget");
    assert_eq!(budget.budget_type(), &BudgetType::Cost);
    assert_eq!(budget.time_unit(), &TimeUnit::Monthly);

    let limit = budget.budget_limit().expect("should have budget_limit");
    assert_eq!(limit.amount(), "250");
    assert_eq!(limit.unit(), "USD");
}

// Scenario 3: Create USAGE type budget and verify budget_type is reflected
#[tokio::test]
async fn test_create_budget_usage_type() {
    let client = make_client().await;

    let usage_budget = Budget::builder()
        .budget_name("usage-type-budget")
        .budget_type(BudgetType::Usage)
        .budget_limit(Spend::builder().amount("1000").unit("GB").build().unwrap())
        .time_unit(TimeUnit::Monthly)
        .build()
        .unwrap();

    client
        .create_budget()
        .account_id("123456789012")
        .budget(usage_budget)
        .send()
        .await
        .expect("create_budget with USAGE type should succeed");

    let resp = client
        .describe_budget()
        .account_id("123456789012")
        .budget_name("usage-type-budget")
        .send()
        .await
        .expect("describe_budget should succeed");

    let budget = resp.budget().expect("should have budget");
    assert_eq!(budget.budget_name(), "usage-type-budget");
    assert_eq!(budget.budget_type(), &BudgetType::Usage);
}

// Scenario 4: Create budget with inline NotificationsWithSubscribers and verify both the budget
// and the notification are persisted and returned by DescribeNotificationsForBudget.
#[tokio::test]
async fn test_create_budget_with_inline_notifications() {
    let client = make_client().await;

    let sub = aws_sdk_budgets::types::Subscriber::builder()
        .subscription_type(aws_sdk_budgets::types::SubscriptionType::Email)
        .address("inline@example.com")
        .build()
        .unwrap();

    let notif_with_sub = aws_sdk_budgets::types::NotificationWithSubscribers::builder()
        .notification(test_notification())
        .subscribers(sub)
        .build()
        .unwrap();

    client
        .create_budget()
        .account_id("123456789012")
        .budget(test_budget("inline-notif-budget", "100"))
        .notifications_with_subscribers(notif_with_sub)
        .send()
        .await
        .expect("create_budget with inline notifications should succeed");

    let resp = client
        .describe_budget()
        .account_id("123456789012")
        .budget_name("inline-notif-budget")
        .send()
        .await
        .expect("describe_budget should succeed");
    assert_eq!(
        resp.budget().expect("should have budget").budget_name(),
        "inline-notif-budget"
    );

    // Verify that the inline notification was persisted
    let notif_resp = client
        .describe_notifications_for_budget()
        .account_id("123456789012")
        .budget_name("inline-notif-budget")
        .send()
        .await
        .expect("describe_notifications_for_budget should succeed");
    assert_eq!(
        notif_resp.notifications().len(),
        1,
        "inline notification should be persisted and returned by DescribeNotificationsForBudget"
    );
    let notif = &notif_resp.notifications()[0];
    assert_eq!(
        notif.notification_type(),
        &aws_sdk_budgets::types::NotificationType::Actual
    );
    assert_eq!(
        notif.comparison_operator(),
        &aws_sdk_budgets::types::ComparisonOperator::GreaterThan
    );
    assert_eq!(notif.threshold(), 80.0);
}

// Scenario 5: Delete a notification that doesn't exist (budget exists) — should return NotFoundException
#[tokio::test]
async fn test_delete_notification_not_found() {
    let client = make_client().await;

    client
        .create_budget()
        .account_id("123456789012")
        .budget(test_budget("del-notif-notfound-budget", "100"))
        .send()
        .await
        .unwrap();

    // No notification was ever created; delete should fail with NotFoundException
    let result = client
        .delete_notification()
        .account_id("123456789012")
        .budget_name("del-notif-notfound-budget")
        .notification(test_notification())
        .send()
        .await;

    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("NotFoundException") || err_str.contains("not found"),
        "Expected NotFoundException, got: {err_str}"
    );
}

// Scenario 6: Create the same notification twice — AWS returns DuplicateRecordException.
#[tokio::test]
async fn test_duplicate_notification_current_behavior() {
    let client = make_client().await;

    client
        .create_budget()
        .account_id("123456789012")
        .budget(test_budget("dup-notif-budget", "100"))
        .send()
        .await
        .unwrap();

    let sub = aws_sdk_budgets::types::Subscriber::builder()
        .subscription_type(aws_sdk_budgets::types::SubscriptionType::Email)
        .address("dup@example.com")
        .build()
        .unwrap();

    client
        .create_notification()
        .account_id("123456789012")
        .budget_name("dup-notif-budget")
        .notification(test_notification())
        .subscribers(sub.clone())
        .send()
        .await
        .expect("first create_notification should succeed");

    // Second identical notification must return DuplicateRecordException (matches AWS behavior).
    let second_result = client
        .create_notification()
        .account_id("123456789012")
        .budget_name("dup-notif-budget")
        .notification(test_notification())
        .subscribers(sub)
        .send()
        .await;

    let err = second_result
        .expect_err("second create_notification should fail with DuplicateRecordException");
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("DuplicateRecordException"),
        "Expected DuplicateRecordException, got: {err_str}"
    );
}

// Scenario 7: Full lifecycle — create -> describe -> list -> delete -> verify-gone
#[tokio::test]
async fn test_budget_full_lifecycle() {
    let client = make_client().await;

    // Create
    client
        .create_budget()
        .account_id("123456789012")
        .budget(test_budget("lifecycle-budget", "999"))
        .send()
        .await
        .expect("create_budget should succeed");

    // Describe — verify fields
    let resp = client
        .describe_budget()
        .account_id("123456789012")
        .budget_name("lifecycle-budget")
        .send()
        .await
        .expect("describe_budget should succeed");
    let budget = resp.budget().expect("should have budget");
    assert_eq!(budget.budget_name(), "lifecycle-budget");
    assert_eq!(
        budget
            .budget_limit()
            .expect("should have budget_limit")
            .amount(),
        "999"
    );

    // List — verify budget appears
    let list_resp = client
        .describe_budgets()
        .account_id("123456789012")
        .send()
        .await
        .expect("describe_budgets should succeed");
    let found = list_resp
        .budgets()
        .iter()
        .any(|b| b.budget_name() == "lifecycle-budget");
    assert!(found, "lifecycle-budget should appear in DescribeBudgets");

    // Delete
    client
        .delete_budget()
        .account_id("123456789012")
        .budget_name("lifecycle-budget")
        .send()
        .await
        .expect("delete_budget should succeed");

    // Verify gone — DescribeBudget should return NotFoundException
    let after_delete = client
        .describe_budget()
        .account_id("123456789012")
        .budget_name("lifecycle-budget")
        .send()
        .await;
    let err_str = format!("{:?}", after_delete.unwrap_err());
    assert!(
        err_str.contains("NotFoundException") || err_str.contains("not found"),
        "Expected NotFoundException after delete, got: {err_str}"
    );
}
