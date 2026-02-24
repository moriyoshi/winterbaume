//! Smoke tests for winterbaume SNS service — realistic application scenarios.
//!
//! Each test simulates a coherent end-to-end workflow rather than exercising
//! a single API call in isolation.

use aws_sdk_sns::config::BehaviorVersion;
use aws_sdk_sns::types::MessageAttributeValue;
use winterbaume_core::MockAws;
use winterbaume_sns::SnsService;

async fn make_sns_client() -> aws_sdk_sns::Client {
    let mock = MockAws::builder().with_service(SnsService::new()).build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_sns::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_sns::Client::new(&config)
}

/// Scenario: multi-subscriber notification hub.
///
/// An e-commerce platform publishes order events to a topic. Three downstream
/// services (email, inventory, analytics) subscribe to it. The test verifies
/// that the topic, subscriptions, and publish all succeed and that the
/// subscriber list reflects reality.
#[tokio::test]
async fn test_order_notification_hub() {
    let client = make_sns_client().await;

    // Create the orders topic.
    let topic = client
        .create_topic()
        .name("order-events")
        .send()
        .await
        .expect("create_topic");
    let topic_arn = topic.topic_arn().expect("topic_arn").to_string();

    // Subscribe three downstream consumers.
    let subscribers = [
        ("email", "arn:aws:sqs:us-east-1:123456789012:email-queue"),
        (
            "inventory",
            "arn:aws:sqs:us-east-1:123456789012:inventory-queue",
        ),
        (
            "analytics",
            "arn:aws:sqs:us-east-1:123456789012:analytics-queue",
        ),
    ];
    let mut subscription_arns = Vec::new();
    for (label, endpoint) in &subscribers {
        let sub = client
            .subscribe()
            .topic_arn(&topic_arn)
            .protocol("sqs")
            .endpoint(*endpoint)
            .send()
            .await
            .unwrap_or_else(|e| panic!("subscribe {label}: {e}"));
        subscription_arns.push(sub.subscription_arn().expect("sub_arn").to_string());
    }

    // List subscriptions on the topic and confirm all three are present.
    let subs = client
        .list_subscriptions_by_topic()
        .topic_arn(&topic_arn)
        .send()
        .await
        .expect("list_subscriptions_by_topic");
    assert_eq!(
        subs.subscriptions().len(),
        3,
        "all three subscribers should be listed"
    );

    // Publish an order-created event.
    let attr = MessageAttributeValue::builder()
        .data_type("String")
        .string_value("order_created")
        .build()
        .expect("attribute");
    let publish = client
        .publish()
        .topic_arn(&topic_arn)
        .message(r#"{"event":"order_created","order_id":"ORD-001","total":99.99}"#)
        .subject("Order Created")
        .message_attributes("event_type", attr)
        .send()
        .await
        .expect("publish");
    assert!(
        publish.message_id().is_some(),
        "publish should return a message ID"
    );

    // Unsubscribe the analytics consumer (it is being decomissioned).
    client
        .unsubscribe()
        .subscription_arn(&subscription_arns[2])
        .send()
        .await
        .expect("unsubscribe analytics");

    let subs_after = client
        .list_subscriptions_by_topic()
        .topic_arn(&topic_arn)
        .send()
        .await
        .expect("list after unsubscribe");
    assert_eq!(
        subs_after.subscriptions().len(),
        2,
        "only two subscribers should remain"
    );
}

/// Scenario: batch event publishing.
///
/// A monitoring service publishes a batch of alarm events in one call and
/// verifies that all are accepted without failures.
#[tokio::test]
async fn test_batch_alarm_publishing() {
    let client = make_sns_client().await;

    let topic = client
        .create_topic()
        .name("alarms")
        .send()
        .await
        .expect("create_topic");
    let topic_arn = topic.topic_arn().expect("topic_arn").to_string();

    // Subscribe a dummy SQS endpoint so the topic has subscribers.
    client
        .subscribe()
        .topic_arn(&topic_arn)
        .protocol("sqs")
        .endpoint("arn:aws:sqs:us-east-1:123456789012:alarms-queue")
        .send()
        .await
        .expect("subscribe");

    // Publish a batch of 5 alarm events.
    let entries: Vec<_> = (1u32..=5)
        .map(|i| {
            aws_sdk_sns::types::PublishBatchRequestEntry::builder()
                .id(format!("alarm-{i}"))
                .message(format!(
                    r#"{{"alarm_id":{i},"severity":"high","metric":"cpu_usage"}}"#
                ))
                .subject(format!("High CPU Alarm #{i}"))
                .build()
                .expect("entry")
        })
        .collect();

    let batch = client
        .publish_batch()
        .topic_arn(&topic_arn)
        .set_publish_batch_request_entries(Some(entries))
        .send()
        .await
        .expect("publish_batch");

    assert_eq!(batch.successful().len(), 5, "all 5 alarms should succeed");
    assert!(batch.failed().is_empty(), "no failures expected");

    // Each successful entry should carry a message ID.
    for entry in batch.successful() {
        assert!(
            entry.message_id().is_some(),
            "entry {} should have a message ID",
            entry.id().unwrap_or("?")
        );
    }
}

/// Scenario: topic attribute lifecycle.
///
/// A platform team creates a topic, sets a human-readable display name and a
/// custom attribute, reads them back, then deletes the topic and verifies it
/// is gone.
#[tokio::test]
async fn test_topic_attribute_lifecycle() {
    let client = make_sns_client().await;

    let topic = client
        .create_topic()
        .name("platform-events")
        .send()
        .await
        .expect("create_topic");
    let topic_arn = topic.topic_arn().expect("topic_arn").to_string();

    // Set the display name.
    client
        .set_topic_attributes()
        .topic_arn(&topic_arn)
        .attribute_name("DisplayName")
        .attribute_value("Platform Events")
        .send()
        .await
        .expect("set DisplayName");

    // Read back and verify.
    let attrs = client
        .get_topic_attributes()
        .topic_arn(&topic_arn)
        .send()
        .await
        .expect("get_topic_attributes");
    let display_name = attrs
        .attributes()
        .and_then(|m| m.get("DisplayName"))
        .map(String::as_str);
    assert_eq!(
        display_name,
        Some("Platform Events"),
        "DisplayName should match what was set"
    );

    // Publish one message to confirm the topic works with attributes set.
    let pub_resp = client
        .publish()
        .topic_arn(&topic_arn)
        .message("system_ready")
        .send()
        .await
        .expect("publish with display name set");
    assert!(pub_resp.message_id().is_some());

    // Delete the topic.
    client
        .delete_topic()
        .topic_arn(&topic_arn)
        .send()
        .await
        .expect("delete_topic");

    // Getting attributes on the deleted topic should now fail.
    let gone = client
        .get_topic_attributes()
        .topic_arn(&topic_arn)
        .send()
        .await;
    assert!(gone.is_err(), "deleted topic should not be accessible");
}

/// Scenario: subscription attribute update.
///
/// An alerting service subscribes to a topic and later sets a filter policy on
/// its subscription to narrow the events it receives. The test verifies the
/// attribute round-trips correctly.
#[tokio::test]
async fn test_subscription_filter_policy() {
    let client = make_sns_client().await;

    let topic = client
        .create_topic()
        .name("filtered-events")
        .send()
        .await
        .expect("create_topic");
    let topic_arn = topic.topic_arn().expect("topic_arn").to_string();

    let sub = client
        .subscribe()
        .topic_arn(&topic_arn)
        .protocol("sqs")
        .endpoint("arn:aws:sqs:us-east-1:123456789012:critical-queue")
        .send()
        .await
        .expect("subscribe");
    let sub_arn = sub.subscription_arn().expect("sub_arn").to_string();

    // Apply a filter policy so only "critical" severity events reach this
    // subscription.
    let filter_policy = r#"{"severity":["critical"]}"#;
    client
        .set_subscription_attributes()
        .subscription_arn(&sub_arn)
        .attribute_name("FilterPolicy")
        .attribute_value(filter_policy)
        .send()
        .await
        .expect("set FilterPolicy");

    // Read back and confirm the filter policy is stored.
    let sub_attrs = client
        .get_subscription_attributes()
        .subscription_arn(&sub_arn)
        .send()
        .await
        .expect("get_subscription_attributes");
    let stored = sub_attrs
        .attributes()
        .and_then(|m| m.get("FilterPolicy"))
        .map(String::as_str);
    assert_eq!(
        stored,
        Some(filter_policy),
        "FilterPolicy should match what was set"
    );
}
