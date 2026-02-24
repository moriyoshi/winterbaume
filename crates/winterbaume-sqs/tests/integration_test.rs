//! Integration tests for winterbaume SQS service.
//!
//! These tests verify that aws-sdk-sqs operations work end-to-end
//! through the winterbaume mock infrastructure.

use aws_sdk_sqs::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_sqs::SqsService;

/// Helper to create a configured SQS client backed by winterbaume.
async fn make_sqs_client() -> aws_sdk_sqs::Client {
    let mock = MockAws::builder().with_service(SqsService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_sqs::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_sqs::Client::new(&config)
}

#[tokio::test]
async fn test_create_queue() {
    let client = make_sqs_client().await;

    let resp = client
        .create_queue()
        .queue_name("test-queue")
        .send()
        .await
        .expect("create_queue should succeed");

    let url = resp.queue_url().expect("should have queue URL");
    assert!(url.contains("test-queue"), "URL should contain queue name");
}

#[tokio::test]
async fn test_get_queue_url() {
    let client = make_sqs_client().await;

    client
        .create_queue()
        .queue_name("url-queue")
        .send()
        .await
        .unwrap();

    let resp = client
        .get_queue_url()
        .queue_name("url-queue")
        .send()
        .await
        .expect("get_queue_url should succeed");

    let url = resp.queue_url().expect("should have queue URL");
    assert!(url.contains("url-queue"));
}

#[tokio::test]
async fn test_list_queues() {
    let client = make_sqs_client().await;

    client
        .create_queue()
        .queue_name("list-queue-a")
        .send()
        .await
        .unwrap();

    client
        .create_queue()
        .queue_name("list-queue-b")
        .send()
        .await
        .unwrap();

    client
        .create_queue()
        .queue_name("other-queue")
        .send()
        .await
        .unwrap();

    // List all queues
    let resp = client
        .list_queues()
        .send()
        .await
        .expect("list_queues should succeed");

    let urls = resp.queue_urls();
    assert_eq!(urls.len(), 3, "should have 3 queues");

    // List with prefix
    let resp = client
        .list_queues()
        .queue_name_prefix("list-queue")
        .send()
        .await
        .unwrap();

    let urls = resp.queue_urls();
    assert_eq!(urls.len(), 2, "should have 2 queues matching prefix");
}

#[tokio::test]
async fn test_delete_queue() {
    let client = make_sqs_client().await;

    let resp = client
        .create_queue()
        .queue_name("del-queue")
        .send()
        .await
        .unwrap();

    let queue_url = resp.queue_url().unwrap();

    client
        .delete_queue()
        .queue_url(queue_url)
        .send()
        .await
        .expect("delete_queue should succeed");

    // get_queue_url should fail
    let err = client.get_queue_url().queue_name("del-queue").send().await;
    assert!(err.is_err(), "get_queue_url should fail after delete");
}

#[tokio::test]
async fn test_send_and_receive_message() {
    let client = make_sqs_client().await;

    let resp = client
        .create_queue()
        .queue_name("msg-queue")
        .send()
        .await
        .unwrap();
    let queue_url = resp.queue_url().unwrap().to_string();

    // Send a message
    let send_resp = client
        .send_message()
        .queue_url(&queue_url)
        .message_body("Hello, SQS!")
        .send()
        .await
        .expect("send_message should succeed");

    assert!(send_resp.message_id().is_some(), "should have message ID");
    assert!(send_resp.md5_of_message_body().is_some(), "should have MD5");

    // Receive the message
    let recv_resp = client
        .receive_message()
        .queue_url(&queue_url)
        .max_number_of_messages(10)
        .send()
        .await
        .expect("receive_message should succeed");

    let messages = recv_resp.messages();
    assert_eq!(messages.len(), 1, "should receive 1 message");
    assert_eq!(messages[0].body(), Some("Hello, SQS!"));
    assert!(
        messages[0].receipt_handle().is_some(),
        "should have receipt handle"
    );
}

#[tokio::test]
async fn test_delete_message() {
    let client = make_sqs_client().await;

    let resp = client
        .create_queue()
        .queue_name("delmsg-queue")
        .send()
        .await
        .unwrap();
    let queue_url = resp.queue_url().unwrap().to_string();

    // Send
    client
        .send_message()
        .queue_url(&queue_url)
        .message_body("to delete")
        .send()
        .await
        .unwrap();

    // Receive
    let recv_resp = client
        .receive_message()
        .queue_url(&queue_url)
        .send()
        .await
        .unwrap();

    let receipt_handle = recv_resp.messages()[0]
        .receipt_handle()
        .unwrap()
        .to_string();

    // Delete
    client
        .delete_message()
        .queue_url(&queue_url)
        .receipt_handle(&receipt_handle)
        .send()
        .await
        .expect("delete_message should succeed");

    // Receive again - should be empty (we use visibility_timeout=0 to ensure the queue is truly empty)
    // Since the message was deleted, even after visibility timeout expires, it won't come back.
    // We'll check by trying to receive with visibility_timeout=0
    let recv_resp = client
        .receive_message()
        .queue_url(&queue_url)
        .visibility_timeout(0)
        .send()
        .await
        .unwrap();

    assert_eq!(
        recv_resp.messages().len(),
        0,
        "should have no messages after delete"
    );
}

#[tokio::test]
async fn test_get_queue_attributes() {
    let client = make_sqs_client().await;

    let resp = client
        .create_queue()
        .queue_name("attr-queue")
        .send()
        .await
        .unwrap();
    let queue_url = resp.queue_url().unwrap().to_string();

    let attrs_resp = client
        .get_queue_attributes()
        .queue_url(&queue_url)
        .attribute_names(aws_sdk_sqs::types::QueueAttributeName::All)
        .send()
        .await
        .expect("get_queue_attributes should succeed");

    let attrs = attrs_resp.attributes();
    assert!(attrs.is_some(), "should have attributes");
    let attrs = attrs.unwrap();
    assert!(
        attrs.contains_key(&aws_sdk_sqs::types::QueueAttributeName::QueueArn),
        "should have QueueArn"
    );
    assert!(
        attrs.contains_key(&aws_sdk_sqs::types::QueueAttributeName::VisibilityTimeout),
        "should have VisibilityTimeout"
    );
}

#[tokio::test]
async fn test_multiple_messages() {
    let client = make_sqs_client().await;

    let resp = client
        .create_queue()
        .queue_name("multi-queue")
        .send()
        .await
        .unwrap();
    let queue_url = resp.queue_url().unwrap().to_string();

    // Send 3 messages
    for i in 0..3 {
        client
            .send_message()
            .queue_url(&queue_url)
            .message_body(format!("msg-{i}"))
            .send()
            .await
            .unwrap();
    }

    // Receive up to 10
    let recv_resp = client
        .receive_message()
        .queue_url(&queue_url)
        .max_number_of_messages(10)
        .send()
        .await
        .unwrap();

    assert_eq!(
        recv_resp.messages().len(),
        3,
        "should receive all 3 messages"
    );
}

#[tokio::test]
async fn test_create_queue_idempotent() {
    let client = make_sqs_client().await;

    let resp1 = client
        .create_queue()
        .queue_name("idem-queue")
        .send()
        .await
        .unwrap();

    let resp2 = client
        .create_queue()
        .queue_name("idem-queue")
        .send()
        .await
        .unwrap();

    assert_eq!(
        resp1.queue_url(),
        resp2.queue_url(),
        "idempotent create should return same URL"
    );
}

#[tokio::test]
async fn test_get_nonexistent_queue_url_fails() {
    let client = make_sqs_client().await;

    let result = client
        .get_queue_url()
        .queue_name("nonexistent-queue")
        .send()
        .await;

    assert!(
        result.is_err(),
        "get_queue_url for nonexistent queue should fail"
    );
}

#[tokio::test]
async fn test_delete_nonexistent_queue() {
    let client = make_sqs_client().await;

    let resp = client
        .create_queue()
        .queue_name("delete-twice-queue")
        .send()
        .await
        .unwrap();
    let queue_url = resp.queue_url().unwrap().to_string();

    // First delete should succeed
    client
        .delete_queue()
        .queue_url(&queue_url)
        .send()
        .await
        .expect("first delete_queue should succeed");

    // Second delete should fail because the queue no longer exists
    let result = client.delete_queue().queue_url(&queue_url).send().await;

    assert!(
        result.is_err(),
        "deleting an already-deleted queue should fail"
    );
}

#[tokio::test]
async fn test_send_message_to_nonexistent_queue() {
    let client = make_sqs_client().await;

    let fake_url = "https://sqs.us-east-1.amazonaws.com/123456789012/no-such-queue";

    let result = client
        .send_message()
        .queue_url(fake_url)
        .message_body("hello")
        .send()
        .await;

    assert!(
        result.is_err(),
        "send_message to nonexistent queue should fail"
    );
}

#[tokio::test]
async fn test_receive_from_empty_queue() {
    let client = make_sqs_client().await;

    let resp = client
        .create_queue()
        .queue_name("empty-recv-queue")
        .send()
        .await
        .unwrap();
    let queue_url = resp.queue_url().unwrap().to_string();

    let recv_resp = client
        .receive_message()
        .queue_url(&queue_url)
        .max_number_of_messages(10)
        .send()
        .await
        .expect("receive_message on empty queue should succeed");

    assert_eq!(
        recv_resp.messages().len(),
        0,
        "empty queue should return 0 messages"
    );
}

#[tokio::test]
async fn test_receive_max_messages_limit() {
    let client = make_sqs_client().await;

    let resp = client
        .create_queue()
        .queue_name("maxmsg-queue")
        .send()
        .await
        .unwrap();
    let queue_url = resp.queue_url().unwrap().to_string();

    // Send 5 messages
    for i in 0..5 {
        client
            .send_message()
            .queue_url(&queue_url)
            .message_body(format!("message-{i}"))
            .send()
            .await
            .unwrap();
    }

    // Receive with max 3
    let recv_resp = client
        .receive_message()
        .queue_url(&queue_url)
        .max_number_of_messages(3)
        .send()
        .await
        .expect("receive_message should succeed");

    assert_eq!(
        recv_resp.messages().len(),
        3,
        "should receive exactly 3 messages"
    );
}

#[tokio::test]
async fn test_message_attributes() {
    let client = make_sqs_client().await;

    let resp = client
        .create_queue()
        .queue_name("attr-msg-queue")
        .send()
        .await
        .unwrap();
    let queue_url = resp.queue_url().unwrap().to_string();

    // Send a message with a custom attribute
    client
        .send_message()
        .queue_url(&queue_url)
        .message_body("body with attrs")
        .message_attributes(
            "myattr",
            aws_sdk_sqs::types::MessageAttributeValue::builder()
                .data_type("String")
                .string_value("hello")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("send_message with attributes should succeed");

    // Receive the message
    let recv_resp = client
        .receive_message()
        .queue_url(&queue_url)
        .message_attribute_names("All")
        .send()
        .await
        .expect("receive_message should succeed");

    let messages = recv_resp.messages();
    assert_eq!(messages.len(), 1, "should receive 1 message");

    let attrs = messages[0].message_attributes();
    assert!(
        attrs.is_some(),
        "received message should have message attributes"
    );
    let attrs = attrs.unwrap();
    assert!(
        attrs.contains_key("myattr"),
        "should contain 'myattr' attribute"
    );
    let attr_val = attrs.get("myattr").unwrap();
    assert_eq!(attr_val.string_value(), Some("hello"));
}

#[tokio::test]
async fn test_send_message_with_delay() {
    let client = make_sqs_client().await;

    let resp = client
        .create_queue()
        .queue_name("delay-msg-queue")
        .send()
        .await
        .unwrap();
    let queue_url = resp.queue_url().unwrap().to_string();

    // Send a message with delay 0 (immediately visible)
    client
        .send_message()
        .queue_url(&queue_url)
        .message_body("immediate message")
        .delay_seconds(0)
        .send()
        .await
        .expect("send_message with delay_seconds(0) should succeed");

    // Receive immediately
    let recv_resp = client
        .receive_message()
        .queue_url(&queue_url)
        .max_number_of_messages(10)
        .send()
        .await
        .expect("receive_message should succeed");

    assert_eq!(
        recv_resp.messages().len(),
        1,
        "should receive 1 message with delay 0"
    );
}

#[tokio::test]
async fn test_queue_attributes_message_count() {
    let client = make_sqs_client().await;

    let resp = client
        .create_queue()
        .queue_name("count-attr-queue")
        .send()
        .await
        .unwrap();
    let queue_url = resp.queue_url().unwrap().to_string();

    // Send 2 messages
    for i in 0..2 {
        client
            .send_message()
            .queue_url(&queue_url)
            .message_body(format!("count-msg-{i}"))
            .send()
            .await
            .unwrap();
    }

    // Get queue attributes
    let attrs_resp = client
        .get_queue_attributes()
        .queue_url(&queue_url)
        .attribute_names(aws_sdk_sqs::types::QueueAttributeName::All)
        .send()
        .await
        .expect("get_queue_attributes should succeed");

    let attrs = attrs_resp.attributes().expect("should have attributes");
    let msg_count = attrs
        .get(&aws_sdk_sqs::types::QueueAttributeName::ApproximateNumberOfMessages)
        .expect("should have ApproximateNumberOfMessages");
    assert_eq!(msg_count, "2", "should report 2 messages in queue");
}

// ===================================================================
// Moto-parity tests: translated from vendor/moto/tests/test_sqs/
// ===================================================================

/// Parity with moto test_message_send_without_attributes.
/// Verifies exact MD5 of message body.
#[tokio::test]
async fn test_moto_send_message_md5() {
    let client = make_sqs_client().await;
    let resp = client
        .create_queue()
        .queue_name("moto-md5-queue")
        .send()
        .await
        .unwrap();
    let queue_url = resp.queue_url().unwrap().to_string();

    let send_resp = client
        .send_message()
        .queue_url(&queue_url)
        .message_body("derp")
        .send()
        .await
        .unwrap();

    assert_eq!(
        send_resp.md5_of_message_body(),
        Some("58fd9edd83341c29f1aebba81c31e257"),
        "MD5 of 'derp' should match"
    );
    assert!(
        send_resp.md5_of_message_attributes().is_none(),
        "should not have MD5OfMessageAttributes when no attrs"
    );
    // MessageId should not contain whitespace
    let msg_id = send_resp.message_id().unwrap();
    assert!(
        !msg_id.contains(' ') && !msg_id.contains('\n'),
        "MessageId should not contain whitespace"
    );

    let recv = client
        .receive_message()
        .queue_url(&queue_url)
        .send()
        .await
        .unwrap();
    assert_eq!(recv.messages().len(), 1);
}

/// Parity with moto test_send_receive_message_without_attributes.
/// Verifies two messages sent and received in order with exact body.
#[tokio::test]
async fn test_moto_send_receive_two_messages_in_order() {
    let client = make_sqs_client().await;
    let resp = client
        .create_queue()
        .queue_name("moto-order-queue")
        .send()
        .await
        .unwrap();
    let queue_url = resp.queue_url().unwrap().to_string();

    let body_one = "this is a test message";
    let body_two = "this is another test message";

    client
        .send_message()
        .queue_url(&queue_url)
        .message_body(body_one)
        .send()
        .await
        .unwrap();
    client
        .send_message()
        .queue_url(&queue_url)
        .message_body(body_two)
        .send()
        .await
        .unwrap();

    let recv = client
        .receive_message()
        .queue_url(&queue_url)
        .max_number_of_messages(2)
        .send()
        .await
        .unwrap();

    let messages = recv.messages();
    assert_eq!(messages.len(), 2);
    assert_eq!(messages[0].body(), Some(body_one));
    assert_eq!(messages[1].body(), Some(body_two));
}

/// Parity with moto test_send_message_with_unicode_characters.
#[tokio::test]
async fn test_moto_send_receive_unicode() {
    let client = make_sqs_client().await;
    let resp = client
        .create_queue()
        .queue_name("moto-unicode-queue")
        .send()
        .await
        .unwrap();
    let queue_url = resp.queue_url().unwrap().to_string();

    let body = "H\u{00e9}llo!\u{1f600}";

    client
        .send_message()
        .queue_url(&queue_url)
        .message_body(body)
        .send()
        .await
        .unwrap();

    let recv = client
        .receive_message()
        .queue_url(&queue_url)
        .send()
        .await
        .unwrap();

    assert_eq!(recv.messages()[0].body(), Some(body));
}

/// Parity with moto test_delete_queue.
/// Verifies queue is removed from list after deletion.
#[tokio::test]
async fn test_moto_delete_queue_list() {
    let client = make_sqs_client().await;
    let resp = client
        .create_queue()
        .queue_name("moto-del-queue")
        .send()
        .await
        .unwrap();
    let queue_url = resp.queue_url().unwrap().to_string();

    // Verify queue is in list
    let list = client.list_queues().send().await.unwrap();
    let urls: Vec<&str> = list.queue_urls().iter().map(|s| s.as_str()).collect();
    assert!(
        urls.iter().any(|u| u.contains("moto-del-queue")),
        "queue should be in list"
    );

    // Delete
    client
        .delete_queue()
        .queue_url(&queue_url)
        .send()
        .await
        .unwrap();

    // Verify not in list
    let list = client.list_queues().send().await.unwrap();
    let urls: Vec<&str> = list.queue_urls().iter().map(|s| s.as_str()).collect();
    assert!(
        !urls.iter().any(|u| u.contains("moto-del-queue")),
        "queue should not be in list after delete"
    );
}

/// Parity with moto test_get_queue_url_error_not_exists.
/// Verifies error code for non-existent queue.
#[tokio::test]
async fn test_moto_get_queue_url_nonexistent_error_code() {
    let client = make_sqs_client().await;

    let result = client.get_queue_url().queue_name("not-exists").send().await;

    let err = result.err().unwrap();
    let service_err = err.as_service_error().expect("should be a service error");
    assert!(
        service_err.is_queue_does_not_exist(),
        "should be QueueDoesNotExist"
    );
}

/// Parity with moto test_get_queue_attributes (exact attribute values).
#[tokio::test]
async fn test_moto_get_queue_attributes_exact_values() {
    let client = make_sqs_client().await;
    let resp = client
        .create_queue()
        .queue_name("moto-attrs-queue")
        .send()
        .await
        .unwrap();
    let queue_url = resp.queue_url().unwrap().to_string();

    let attrs_resp = client
        .get_queue_attributes()
        .queue_url(&queue_url)
        .attribute_names(aws_sdk_sqs::types::QueueAttributeName::All)
        .send()
        .await
        .unwrap();

    let attrs = attrs_resp.attributes().unwrap();

    // Verify exact default values (matching moto)
    assert_eq!(
        attrs
            .get(&aws_sdk_sqs::types::QueueAttributeName::ApproximateNumberOfMessages)
            .unwrap(),
        "0"
    );
    assert_eq!(
        attrs
            .get(&aws_sdk_sqs::types::QueueAttributeName::ApproximateNumberOfMessagesNotVisible)
            .unwrap(),
        "0"
    );
    assert_eq!(
        attrs
            .get(&aws_sdk_sqs::types::QueueAttributeName::DelaySeconds)
            .unwrap(),
        "0"
    );
    assert_eq!(
        attrs
            .get(&aws_sdk_sqs::types::QueueAttributeName::VisibilityTimeout)
            .unwrap(),
        "30"
    );
    assert_eq!(
        attrs
            .get(&aws_sdk_sqs::types::QueueAttributeName::MessageRetentionPeriod)
            .unwrap(),
        "345600"
    );
    assert_eq!(
        attrs
            .get(&aws_sdk_sqs::types::QueueAttributeName::ReceiveMessageWaitTimeSeconds)
            .unwrap(),
        "0"
    );

    // QueueArn should have correct format
    let arn = attrs
        .get(&aws_sdk_sqs::types::QueueAttributeName::QueueArn)
        .unwrap();
    assert!(
        arn.contains("arn:aws:sqs:us-east-1:"),
        "ARN should contain region"
    );
    assert!(
        arn.ends_with(":moto-attrs-queue"),
        "ARN should end with queue name"
    );

    // CreatedTimestamp should be a parseable number
    let created = attrs
        .get(&aws_sdk_sqs::types::QueueAttributeName::CreatedTimestamp)
        .unwrap();
    assert!(
        created.parse::<i64>().is_ok(),
        "CreatedTimestamp should be a number"
    );
}

/// Parity with moto test_set_queue_attribute.
/// Verifies changing VisibilityTimeout.
#[tokio::test]
async fn test_moto_set_queue_attributes() {
    let client = make_sqs_client().await;
    let resp = client
        .create_queue()
        .queue_name("moto-setattr-queue")
        .send()
        .await
        .unwrap();
    let queue_url = resp.queue_url().unwrap().to_string();

    // Default VisibilityTimeout is 30
    let attrs = client
        .get_queue_attributes()
        .queue_url(&queue_url)
        .attribute_names(aws_sdk_sqs::types::QueueAttributeName::All)
        .send()
        .await
        .unwrap();
    assert_eq!(
        attrs
            .attributes()
            .unwrap()
            .get(&aws_sdk_sqs::types::QueueAttributeName::VisibilityTimeout)
            .unwrap(),
        "30"
    );

    // Change it to 45
    client
        .set_queue_attributes()
        .queue_url(&queue_url)
        .attributes(
            aws_sdk_sqs::types::QueueAttributeName::VisibilityTimeout,
            "45",
        )
        .send()
        .await
        .unwrap();

    // Verify new value
    let attrs = client
        .get_queue_attributes()
        .queue_url(&queue_url)
        .attribute_names(aws_sdk_sqs::types::QueueAttributeName::All)
        .send()
        .await
        .unwrap();
    assert_eq!(
        attrs
            .attributes()
            .unwrap()
            .get(&aws_sdk_sqs::types::QueueAttributeName::VisibilityTimeout)
            .unwrap(),
        "45"
    );
}

/// Parity with moto: purge queue removes all messages.
#[tokio::test]
async fn test_moto_purge_queue() {
    let client = make_sqs_client().await;
    let resp = client
        .create_queue()
        .queue_name("moto-purge-queue")
        .send()
        .await
        .unwrap();
    let queue_url = resp.queue_url().unwrap().to_string();

    // Send 3 messages
    for i in 0..3 {
        client
            .send_message()
            .queue_url(&queue_url)
            .message_body(format!("msg-{i}"))
            .send()
            .await
            .unwrap();
    }

    // Verify messages exist
    let attrs = client
        .get_queue_attributes()
        .queue_url(&queue_url)
        .attribute_names(aws_sdk_sqs::types::QueueAttributeName::All)
        .send()
        .await
        .unwrap();
    assert_eq!(
        attrs
            .attributes()
            .unwrap()
            .get(&aws_sdk_sqs::types::QueueAttributeName::ApproximateNumberOfMessages)
            .unwrap(),
        "3"
    );

    // Purge
    client
        .purge_queue()
        .queue_url(&queue_url)
        .send()
        .await
        .expect("purge_queue should succeed");

    // Verify empty
    let recv = client
        .receive_message()
        .queue_url(&queue_url)
        .max_number_of_messages(10)
        .visibility_timeout(0)
        .send()
        .await
        .unwrap();
    assert_eq!(
        recv.messages().len(),
        0,
        "queue should be empty after purge"
    );
}

/// Parity with moto test_create_queue (queue URL format).
#[tokio::test]
async fn test_moto_queue_url_and_arn_format() {
    let client = make_sqs_client().await;
    let resp = client
        .create_queue()
        .queue_name("moto-url-queue")
        .send()
        .await
        .unwrap();
    let queue_url = resp.queue_url().unwrap();

    assert!(
        queue_url.contains("moto-url-queue"),
        "URL should contain queue name"
    );

    let attrs = client
        .get_queue_attributes()
        .queue_url(queue_url)
        .attribute_names(aws_sdk_sqs::types::QueueAttributeName::All)
        .send()
        .await
        .unwrap();
    let arn = attrs
        .attributes()
        .unwrap()
        .get(&aws_sdk_sqs::types::QueueAttributeName::QueueArn)
        .unwrap()
        .clone();

    // ARN format: arn:aws:sqs:REGION:ACCOUNT:QUEUE_NAME
    let parts: Vec<&str> = arn.split(':').collect();
    assert_eq!(parts.last().unwrap(), &"moto-url-queue");
    assert_eq!(parts[3], "us-east-1");
}

/// Parity with moto test_create_queue_with_same_attributes (idempotent).
#[tokio::test]
async fn test_moto_create_queue_idempotent_same_attrs() {
    let client = make_sqs_client().await;

    let resp1 = client
        .create_queue()
        .queue_name("moto-idem-queue")
        .send()
        .await
        .unwrap();

    let resp2 = client
        .create_queue()
        .queue_name("moto-idem-queue")
        .send()
        .await
        .unwrap();

    assert_eq!(
        resp1.queue_url(),
        resp2.queue_url(),
        "idempotent create should return same URL"
    );
}

/// Parity with moto test_get_queue_with_prefix.
/// Verifies listing queues by prefix.
#[tokio::test]
async fn test_moto_list_queues_prefix() {
    let client = make_sqs_client().await;
    client
        .create_queue()
        .queue_name("alpha-one")
        .send()
        .await
        .unwrap();
    client
        .create_queue()
        .queue_name("alpha-two")
        .send()
        .await
        .unwrap();
    client
        .create_queue()
        .queue_name("beta-one")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_queues()
        .queue_name_prefix("alpha")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.queue_urls().len(), 2, "should match 2 alpha queues");

    let resp = client
        .list_queues()
        .queue_name_prefix("beta")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.queue_urls().len(), 1, "should match 1 beta queue");

    let resp = client
        .list_queues()
        .queue_name_prefix("gamma")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.queue_urls().len(), 0, "should match 0 gamma queues");
}

/// Parity with moto: send_message_batch.
#[tokio::test]
async fn test_moto_send_message_batch() {
    let client = make_sqs_client().await;
    let resp = client
        .create_queue()
        .queue_name("moto-batch-queue")
        .send()
        .await
        .unwrap();
    let queue_url = resp.queue_url().unwrap().to_string();

    use aws_sdk_sqs::types::SendMessageBatchRequestEntry;

    let entries = vec![
        SendMessageBatchRequestEntry::builder()
            .id("msg1")
            .message_body("body one")
            .build()
            .unwrap(),
        SendMessageBatchRequestEntry::builder()
            .id("msg2")
            .message_body("body two")
            .build()
            .unwrap(),
        SendMessageBatchRequestEntry::builder()
            .id("msg3")
            .message_body("body three")
            .build()
            .unwrap(),
    ];

    let batch_resp = client
        .send_message_batch()
        .queue_url(&queue_url)
        .set_entries(Some(entries))
        .send()
        .await
        .expect("send_message_batch should succeed");

    assert_eq!(
        batch_resp.successful().len(),
        3,
        "all 3 messages should succeed"
    );

    // Verify all 3 can be received
    let recv = client
        .receive_message()
        .queue_url(&queue_url)
        .max_number_of_messages(10)
        .send()
        .await
        .unwrap();
    assert_eq!(recv.messages().len(), 3);

    let bodies: Vec<&str> = recv.messages().iter().map(|m| m.body().unwrap()).collect();
    assert!(bodies.contains(&"body one"));
    assert!(bodies.contains(&"body two"));
    assert!(bodies.contains(&"body three"));
}

/// Parity with moto: delete_message_batch.
#[tokio::test]
async fn test_moto_delete_message_batch() {
    let client = make_sqs_client().await;
    let resp = client
        .create_queue()
        .queue_name("moto-delbatch-queue")
        .send()
        .await
        .unwrap();
    let queue_url = resp.queue_url().unwrap().to_string();

    // Send 3 messages
    for i in 0..3 {
        client
            .send_message()
            .queue_url(&queue_url)
            .message_body(format!("delbatch-msg-{i}"))
            .send()
            .await
            .unwrap();
    }

    // Receive all
    let recv = client
        .receive_message()
        .queue_url(&queue_url)
        .max_number_of_messages(10)
        .send()
        .await
        .unwrap();
    assert_eq!(recv.messages().len(), 3);

    // Delete batch
    use aws_sdk_sqs::types::DeleteMessageBatchRequestEntry;
    let entries: Vec<DeleteMessageBatchRequestEntry> = recv
        .messages()
        .iter()
        .enumerate()
        .map(|(i, m)| {
            DeleteMessageBatchRequestEntry::builder()
                .id(format!("del-{i}"))
                .receipt_handle(m.receipt_handle().unwrap())
                .build()
                .unwrap()
        })
        .collect();

    let del_resp = client
        .delete_message_batch()
        .queue_url(&queue_url)
        .set_entries(Some(entries))
        .send()
        .await
        .expect("delete_message_batch should succeed");

    assert_eq!(del_resp.successful().len(), 3);

    // Queue should be empty
    let recv = client
        .receive_message()
        .queue_url(&queue_url)
        .max_number_of_messages(10)
        .visibility_timeout(0)
        .send()
        .await
        .unwrap();
    assert_eq!(
        recv.messages().len(),
        0,
        "queue should be empty after batch delete"
    );
}

/// Parity with moto: FIFO queue creation.
#[tokio::test]
async fn test_moto_create_fifo_queue() {
    let client = make_sqs_client().await;
    let resp = client
        .create_queue()
        .queue_name("moto-fifo-queue.fifo")
        .attributes(aws_sdk_sqs::types::QueueAttributeName::FifoQueue, "true")
        .send()
        .await
        .unwrap();
    let queue_url = resp.queue_url().unwrap();
    assert!(queue_url.contains("moto-fifo-queue.fifo"));

    let attrs = client
        .get_queue_attributes()
        .queue_url(queue_url)
        .attribute_names(aws_sdk_sqs::types::QueueAttributeName::All)
        .send()
        .await
        .unwrap();
    let attrs = attrs.attributes().unwrap();

    // FifoQueue attribute should be "true"
    assert_eq!(
        attrs
            .get(&aws_sdk_sqs::types::QueueAttributeName::FifoQueue)
            .unwrap(),
        "true"
    );
    assert_eq!(
        attrs
            .get(&aws_sdk_sqs::types::QueueAttributeName::VisibilityTimeout)
            .unwrap(),
        "30"
    );
}

/// Parity with moto: change_message_visibility makes message visible again.
#[tokio::test]
async fn test_moto_change_message_visibility() {
    let client = make_sqs_client().await;
    let resp = client
        .create_queue()
        .queue_name("moto-vis-queue")
        .send()
        .await
        .unwrap();
    let queue_url = resp.queue_url().unwrap().to_string();

    // Send a message
    client
        .send_message()
        .queue_url(&queue_url)
        .message_body("visibility test")
        .send()
        .await
        .unwrap();

    // Receive with long visibility timeout
    let recv = client
        .receive_message()
        .queue_url(&queue_url)
        .visibility_timeout(300)
        .send()
        .await
        .unwrap();
    assert_eq!(recv.messages().len(), 1);
    let receipt_handle = recv.messages()[0].receipt_handle().unwrap().to_string();

    // Message should not be visible now
    let recv2 = client
        .receive_message()
        .queue_url(&queue_url)
        .send()
        .await
        .unwrap();
    assert_eq!(
        recv2.messages().len(),
        0,
        "message should not be visible during timeout"
    );

    // Change visibility to 0 to make it visible immediately
    client
        .change_message_visibility()
        .queue_url(&queue_url)
        .receipt_handle(&receipt_handle)
        .visibility_timeout(0)
        .send()
        .await
        .expect("change_message_visibility should succeed");

    // Should be visible again
    let recv3 = client
        .receive_message()
        .queue_url(&queue_url)
        .send()
        .await
        .unwrap();
    assert_eq!(
        recv3.messages().len(),
        1,
        "message should be visible after changing visibility to 0"
    );
    assert_eq!(recv3.messages()[0].body(), Some("visibility test"));
}

// ===================================================================
// Tests for newly added operations
// ===================================================================

#[tokio::test]
async fn test_tag_queue_and_list_queue_tags() {
    let client = make_sqs_client().await;

    let resp = client
        .create_queue()
        .queue_name("tag-queue")
        .send()
        .await
        .unwrap();
    let queue_url = resp.queue_url().unwrap().to_string();

    // Initially no tags
    let tags_resp = client
        .list_queue_tags()
        .queue_url(&queue_url)
        .send()
        .await
        .expect("list_queue_tags should succeed");
    assert!(
        tags_resp.tags().is_none() || tags_resp.tags().unwrap().is_empty(),
        "should have no tags initially"
    );

    // Tag the queue
    client
        .tag_queue()
        .queue_url(&queue_url)
        .tags("env", "production")
        .tags("team", "backend")
        .send()
        .await
        .expect("tag_queue should succeed");

    // List tags
    let tags_resp = client
        .list_queue_tags()
        .queue_url(&queue_url)
        .send()
        .await
        .expect("list_queue_tags should succeed");
    let tags = tags_resp.tags().expect("should have tags");
    assert_eq!(tags.get("env").unwrap(), "production");
    assert_eq!(tags.get("team").unwrap(), "backend");
}

#[tokio::test]
async fn test_untag_queue() {
    let client = make_sqs_client().await;

    let resp = client
        .create_queue()
        .queue_name("untag-queue")
        .send()
        .await
        .unwrap();
    let queue_url = resp.queue_url().unwrap().to_string();

    // Add tags
    client
        .tag_queue()
        .queue_url(&queue_url)
        .tags("env", "staging")
        .tags("team", "frontend")
        .tags("cost-center", "123")
        .send()
        .await
        .unwrap();

    // Remove one tag
    client
        .untag_queue()
        .queue_url(&queue_url)
        .tag_keys("team")
        .send()
        .await
        .expect("untag_queue should succeed");

    // Verify
    let tags_resp = client
        .list_queue_tags()
        .queue_url(&queue_url)
        .send()
        .await
        .unwrap();
    let tags = tags_resp.tags().expect("should have tags");
    assert_eq!(tags.len(), 2, "should have 2 tags after removing one");
    assert_eq!(tags.get("env").unwrap(), "staging");
    assert_eq!(tags.get("cost-center").unwrap(), "123");
    assert!(tags.get("team").is_none(), "removed tag should be gone");
}

#[tokio::test]
async fn test_tag_queue_overwrite() {
    let client = make_sqs_client().await;

    let resp = client
        .create_queue()
        .queue_name("tag-overwrite-queue")
        .send()
        .await
        .unwrap();
    let queue_url = resp.queue_url().unwrap().to_string();

    // Set a tag
    client
        .tag_queue()
        .queue_url(&queue_url)
        .tags("env", "dev")
        .send()
        .await
        .unwrap();

    // Overwrite the tag
    client
        .tag_queue()
        .queue_url(&queue_url)
        .tags("env", "prod")
        .send()
        .await
        .unwrap();

    let tags_resp = client
        .list_queue_tags()
        .queue_url(&queue_url)
        .send()
        .await
        .unwrap();
    let tags = tags_resp.tags().unwrap();
    assert_eq!(
        tags.get("env").unwrap(),
        "prod",
        "tag should be overwritten"
    );
}

#[tokio::test]
async fn test_add_permission_and_remove_permission() {
    let client = make_sqs_client().await;

    let resp = client
        .create_queue()
        .queue_name("perm-queue")
        .send()
        .await
        .unwrap();
    let queue_url = resp.queue_url().unwrap().to_string();

    // Add permission
    client
        .add_permission()
        .queue_url(&queue_url)
        .label("test-permission")
        .aws_account_ids("111122223333")
        .actions("SendMessage")
        .actions("ReceiveMessage")
        .send()
        .await
        .expect("add_permission should succeed");

    // Remove permission
    client
        .remove_permission()
        .queue_url(&queue_url)
        .label("test-permission")
        .send()
        .await
        .expect("remove_permission should succeed");
}

#[tokio::test]
async fn test_list_dead_letter_source_queues() {
    let client = make_sqs_client().await;

    // Create the DLQ
    let dlq_resp = client
        .create_queue()
        .queue_name("my-dlq")
        .send()
        .await
        .unwrap();
    let dlq_url = dlq_resp.queue_url().unwrap().to_string();

    // Get the DLQ ARN
    let dlq_attrs = client
        .get_queue_attributes()
        .queue_url(&dlq_url)
        .attribute_names(aws_sdk_sqs::types::QueueAttributeName::All)
        .send()
        .await
        .unwrap();
    let dlq_arn = dlq_attrs
        .attributes()
        .unwrap()
        .get(&aws_sdk_sqs::types::QueueAttributeName::QueueArn)
        .unwrap()
        .clone();

    // Create source queues that point to the DLQ
    let redrive_policy = format!(
        r#"{{"deadLetterTargetArn":"{}","maxReceiveCount":"5"}}"#,
        dlq_arn
    );

    client
        .create_queue()
        .queue_name("source-queue-1")
        .attributes(
            aws_sdk_sqs::types::QueueAttributeName::RedrivePolicy,
            &redrive_policy,
        )
        .send()
        .await
        .unwrap();

    client
        .create_queue()
        .queue_name("source-queue-2")
        .attributes(
            aws_sdk_sqs::types::QueueAttributeName::RedrivePolicy,
            &redrive_policy,
        )
        .send()
        .await
        .unwrap();

    // Create a queue that does NOT point to the DLQ
    client
        .create_queue()
        .queue_name("unrelated-queue")
        .send()
        .await
        .unwrap();

    // List dead letter source queues
    let resp = client
        .list_dead_letter_source_queues()
        .queue_url(&dlq_url)
        .send()
        .await
        .expect("list_dead_letter_source_queues should succeed");

    let source_urls = resp.queue_urls();
    assert_eq!(source_urls.len(), 2, "should have 2 source queues");
    assert!(
        source_urls.iter().any(|u| u.contains("source-queue-1")),
        "should contain source-queue-1"
    );
    assert!(
        source_urls.iter().any(|u| u.contains("source-queue-2")),
        "should contain source-queue-2"
    );
}

#[tokio::test]
async fn test_list_dead_letter_source_queues_empty() {
    let client = make_sqs_client().await;

    let resp = client
        .create_queue()
        .queue_name("lonely-dlq")
        .send()
        .await
        .unwrap();
    let queue_url = resp.queue_url().unwrap().to_string();

    let resp = client
        .list_dead_letter_source_queues()
        .queue_url(&queue_url)
        .send()
        .await
        .expect("list_dead_letter_source_queues should succeed");

    assert_eq!(resp.queue_urls().len(), 0, "should have no source queues");
}

#[tokio::test]
async fn test_change_message_visibility_batch() {
    let client = make_sqs_client().await;

    let resp = client
        .create_queue()
        .queue_name("vis-batch-queue")
        .send()
        .await
        .unwrap();
    let queue_url = resp.queue_url().unwrap().to_string();

    // Send 3 messages
    for i in 0..3 {
        client
            .send_message()
            .queue_url(&queue_url)
            .message_body(format!("vis-msg-{i}"))
            .send()
            .await
            .unwrap();
    }

    // Receive all with long visibility timeout
    let recv = client
        .receive_message()
        .queue_url(&queue_url)
        .max_number_of_messages(10)
        .visibility_timeout(300)
        .send()
        .await
        .unwrap();
    assert_eq!(recv.messages().len(), 3);

    // They should not be visible now
    let recv2 = client
        .receive_message()
        .queue_url(&queue_url)
        .max_number_of_messages(10)
        .send()
        .await
        .unwrap();
    assert_eq!(recv2.messages().len(), 0, "messages should be invisible");

    // Change visibility to 0 for all via batch
    use aws_sdk_sqs::types::ChangeMessageVisibilityBatchRequestEntry;
    let entries: Vec<ChangeMessageVisibilityBatchRequestEntry> = recv
        .messages()
        .iter()
        .enumerate()
        .map(|(i, m)| {
            ChangeMessageVisibilityBatchRequestEntry::builder()
                .id(format!("entry-{i}"))
                .receipt_handle(m.receipt_handle().unwrap())
                .visibility_timeout(0)
                .build()
                .unwrap()
        })
        .collect();

    let batch_resp = client
        .change_message_visibility_batch()
        .queue_url(&queue_url)
        .set_entries(Some(entries))
        .send()
        .await
        .expect("change_message_visibility_batch should succeed");

    assert_eq!(
        batch_resp.successful().len(),
        3,
        "all 3 entries should succeed"
    );

    // Messages should be visible again
    let recv3 = client
        .receive_message()
        .queue_url(&queue_url)
        .max_number_of_messages(10)
        .send()
        .await
        .unwrap();
    assert_eq!(
        recv3.messages().len(),
        3,
        "all 3 messages should be visible after batch visibility change"
    );
}

// ============================================================================
// Ported from moto: test_sqs.py
// ============================================================================

// Ported from moto: test_sqs.py::test_create_fifo_queue_fail
#[tokio::test]
async fn test_moto_create_fifo_queue_fail() {
    let client = make_sqs_client().await;

    // FifoQueue=true but name doesn't end with .fifo
    let err = client
        .create_queue()
        .queue_name("some-queue")
        .attributes(aws_sdk_sqs::types::QueueAttributeName::FifoQueue, "true")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("InvalidParameterValue"),
        "Expected InvalidParameterValue error, got: {err_str}"
    );
}

// Ported from moto: test_sqs.py::test_create_queue_with_different_attributes_fail
#[tokio::test]
async fn test_moto_create_queue_with_different_attributes_fail() {
    let client = make_sqs_client().await;

    client
        .create_queue()
        .queue_name("diff-attrs-queue")
        .attributes(
            aws_sdk_sqs::types::QueueAttributeName::VisibilityTimeout,
            "10",
        )
        .send()
        .await
        .unwrap();

    // Creating same queue with different VisibilityTimeout should fail
    let err = client
        .create_queue()
        .queue_name("diff-attrs-queue")
        .attributes(
            aws_sdk_sqs::types::QueueAttributeName::VisibilityTimeout,
            "60",
        )
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("QueueAlreadyExists"),
        "Expected QueueAlreadyExists error, got: {err_str}"
    );
}

// Ported from moto: test_sqs.py::test_create_queue_with_same_attributes
#[tokio::test]
async fn test_moto_create_queue_with_same_attributes() {
    let client = make_sqs_client().await;

    let resp1 = client
        .create_queue()
        .queue_name("same-attrs-queue")
        .attributes(
            aws_sdk_sqs::types::QueueAttributeName::VisibilityTimeout,
            "45",
        )
        .attributes(aws_sdk_sqs::types::QueueAttributeName::DelaySeconds, "5")
        .send()
        .await
        .unwrap();

    // Same name, same attributes should succeed
    let resp2 = client
        .create_queue()
        .queue_name("same-attrs-queue")
        .attributes(
            aws_sdk_sqs::types::QueueAttributeName::VisibilityTimeout,
            "45",
        )
        .attributes(aws_sdk_sqs::types::QueueAttributeName::DelaySeconds, "5")
        .send()
        .await
        .unwrap();

    assert_eq!(resp1.queue_url(), resp2.queue_url());
}

// Ported from moto: test_sqs.py::test_create_queue_with_tags
#[tokio::test]
async fn test_moto_create_queue_with_tags() {
    let client = make_sqs_client().await;

    let resp = client
        .create_queue()
        .queue_name("tagged-queue")
        .tags("tag_key_1", "tag_value_1")
        .tags("tag_key_2", "")
        .send()
        .await
        .unwrap();
    let queue_url = resp.queue_url().unwrap().to_string();

    let tags_resp = client
        .list_queue_tags()
        .queue_url(&queue_url)
        .send()
        .await
        .unwrap();
    let tags = tags_resp.tags().unwrap();
    assert_eq!(tags.get("tag_key_1").unwrap(), "tag_value_1");
    assert_eq!(tags.get("tag_key_2").unwrap(), "");
}

// Ported from moto: test_sqs.py::test_tags (full tag/untag/list flow with non-existing tag removal)
#[tokio::test]
async fn test_moto_tags_full_flow() {
    let client = make_sqs_client().await;

    let resp = client
        .create_queue()
        .queue_name("tag-flow-queue.fifo")
        .attributes(aws_sdk_sqs::types::QueueAttributeName::FifoQueue, "true")
        .send()
        .await
        .unwrap();
    let queue_url = resp.queue_url().unwrap().to_string();

    client
        .tag_queue()
        .queue_url(&queue_url)
        .tags("test1", "value1")
        .tags("test2", "value2")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_queue_tags()
        .queue_url(&queue_url)
        .send()
        .await
        .unwrap();
    let tags = resp.tags().unwrap();
    assert!(tags.contains_key("test1"));
    assert!(tags.contains_key("test2"));

    client
        .untag_queue()
        .queue_url(&queue_url)
        .tag_keys("test2")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_queue_tags()
        .queue_url(&queue_url)
        .send()
        .await
        .unwrap();
    let tags = resp.tags().unwrap();
    assert!(tags.contains_key("test1"));
    assert!(!tags.contains_key("test2"));

    // Removing a non-existing tag should not raise an error
    client
        .untag_queue()
        .queue_url(&queue_url)
        .tag_keys("not-existing-tag")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_queue_tags()
        .queue_url(&queue_url)
        .send()
        .await
        .unwrap();
    let tags = resp.tags().unwrap();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags.get("test1").unwrap(), "value1");
}

// Ported from moto: test_sqs.py::test_permissions
#[tokio::test]
async fn test_moto_permissions_policy() {
    let client = make_sqs_client().await;

    let resp = client
        .create_queue()
        .queue_name("perm-policy-queue.fifo")
        .attributes(aws_sdk_sqs::types::QueueAttributeName::FifoQueue, "true")
        .send()
        .await
        .unwrap();
    let queue_url = resp.queue_url().unwrap().to_string();

    client
        .add_permission()
        .queue_url(&queue_url)
        .label("account1")
        .aws_account_ids("111111111111")
        .actions("*")
        .send()
        .await
        .unwrap();

    client
        .add_permission()
        .queue_url(&queue_url)
        .label("account2")
        .aws_account_ids("222211111111")
        .actions("SendMessage")
        .send()
        .await
        .unwrap();

    // Get the Policy attribute
    let attrs = client
        .get_queue_attributes()
        .queue_url(&queue_url)
        .attribute_names(aws_sdk_sqs::types::QueueAttributeName::Policy)
        .send()
        .await
        .unwrap();
    let policy_str = attrs
        .attributes()
        .unwrap()
        .get(&aws_sdk_sqs::types::QueueAttributeName::Policy)
        .expect("Policy attribute should be present");

    let policy: serde_json::Value = serde_json::from_str(policy_str).unwrap();
    assert_eq!(policy["Version"], "2012-10-17");

    let statements = policy["Statement"].as_array().unwrap();
    assert_eq!(statements.len(), 2);

    // Remove one permission
    client
        .remove_permission()
        .queue_url(&queue_url)
        .label("account2")
        .send()
        .await
        .unwrap();

    let attrs = client
        .get_queue_attributes()
        .queue_url(&queue_url)
        .attribute_names(aws_sdk_sqs::types::QueueAttributeName::Policy)
        .send()
        .await
        .unwrap();
    let policy_str = attrs
        .attributes()
        .unwrap()
        .get(&aws_sdk_sqs::types::QueueAttributeName::Policy)
        .unwrap();
    let policy: serde_json::Value = serde_json::from_str(policy_str).unwrap();
    assert_eq!(policy["Statement"].as_array().unwrap().len(), 1);
}

// Ported from moto: test_sqs.py::test_add_permission_errors
#[tokio::test]
async fn test_moto_add_permission_errors() {
    let client = make_sqs_client().await;

    let resp = client
        .create_queue()
        .queue_name("perm-error-queue")
        .send()
        .await
        .unwrap();
    let queue_url = resp.queue_url().unwrap().to_string();

    // First, add a permission
    client
        .add_permission()
        .queue_url(&queue_url)
        .label("test")
        .aws_account_ids("111111111111")
        .actions("ReceiveMessage")
        .send()
        .await
        .unwrap();

    // Duplicate label should fail
    let err = client
        .add_permission()
        .queue_url(&queue_url)
        .label("test")
        .aws_account_ids("111111111111")
        .actions("ReceiveMessage")
        .actions("SendMessage")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("Already exists"),
        "Expected 'Already exists' error, got: {err_str}"
    );

    // Disallowed action
    let err = client
        .add_permission()
        .queue_url(&queue_url)
        .label("test-2")
        .aws_account_ids("111111111111")
        .actions("RemovePermission")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("Only the queue owner"),
        "Expected 'Only the queue owner' error, got: {err_str}"
    );
}

// Ported from moto: test_sqs.py::test_remove_permission_errors
#[tokio::test]
async fn test_moto_remove_permission_errors() {
    let client = make_sqs_client().await;

    let resp = client
        .create_queue()
        .queue_name("rm-perm-error-queue")
        .send()
        .await
        .unwrap();
    let queue_url = resp.queue_url().unwrap().to_string();

    // Removing a non-existent label should fail
    let err = client
        .remove_permission()
        .queue_url(&queue_url)
        .label("test")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("can't find label"),
        "Expected 'can't find label' error, got: {err_str}"
    );
}

// Ported from moto: test_sqs.py::test_redrive_policy_available
#[tokio::test]
async fn test_moto_redrive_policy_available() {
    let client = make_sqs_client().await;

    // Create DLQ
    let dlq_resp = client
        .create_queue()
        .queue_name("redrive-dlq")
        .send()
        .await
        .unwrap();
    let dlq_url = dlq_resp.queue_url().unwrap().to_string();

    let dlq_attrs = client
        .get_queue_attributes()
        .queue_url(&dlq_url)
        .attribute_names(aws_sdk_sqs::types::QueueAttributeName::QueueArn)
        .send()
        .await
        .unwrap();
    let dlq_arn = dlq_attrs
        .attributes()
        .unwrap()
        .get(&aws_sdk_sqs::types::QueueAttributeName::QueueArn)
        .unwrap()
        .clone();

    let redrive_policy = format!(
        r#"{{"deadLetterTargetArn":"{}","maxReceiveCount":1}}"#,
        dlq_arn
    );

    let resp = client
        .create_queue()
        .queue_name("redrive-source")
        .attributes(
            aws_sdk_sqs::types::QueueAttributeName::RedrivePolicy,
            &redrive_policy,
        )
        .send()
        .await
        .unwrap();
    let queue_url = resp.queue_url().unwrap().to_string();

    // RedrivePolicy should be returned in attributes
    let attrs = client
        .get_queue_attributes()
        .queue_url(&queue_url)
        .attribute_names(aws_sdk_sqs::types::QueueAttributeName::RedrivePolicy)
        .send()
        .await
        .unwrap();
    let rp = attrs
        .attributes()
        .unwrap()
        .get(&aws_sdk_sqs::types::QueueAttributeName::RedrivePolicy)
        .expect("RedrivePolicy should be present");

    let parsed: serde_json::Value = serde_json::from_str(rp).unwrap();
    assert_eq!(parsed["deadLetterTargetArn"].as_str().unwrap(), dlq_arn);
    assert_eq!(parsed["maxReceiveCount"], 1);
}

// Ported from moto: test_sqs.py::test_redrive_policy_set_attributes
#[tokio::test]
async fn test_moto_redrive_policy_set_attributes() {
    let client = make_sqs_client().await;

    // Create source queue and DLQ
    let src_resp = client
        .create_queue()
        .queue_name("redrive-set-src")
        .send()
        .await
        .unwrap();
    let src_url = src_resp.queue_url().unwrap().to_string();

    let dlq_resp = client
        .create_queue()
        .queue_name("redrive-set-dlq")
        .send()
        .await
        .unwrap();
    let dlq_url = dlq_resp.queue_url().unwrap().to_string();

    let dlq_attrs = client
        .get_queue_attributes()
        .queue_url(&dlq_url)
        .attribute_names(aws_sdk_sqs::types::QueueAttributeName::QueueArn)
        .send()
        .await
        .unwrap();
    let dlq_arn = dlq_attrs
        .attributes()
        .unwrap()
        .get(&aws_sdk_sqs::types::QueueAttributeName::QueueArn)
        .unwrap()
        .clone();

    let redrive_policy = format!(
        r#"{{"deadLetterTargetArn":"{}","maxReceiveCount":1}}"#,
        dlq_arn
    );

    // Set via SetQueueAttributes
    client
        .set_queue_attributes()
        .queue_url(&src_url)
        .attributes(
            aws_sdk_sqs::types::QueueAttributeName::RedrivePolicy,
            &redrive_policy,
        )
        .send()
        .await
        .unwrap();

    // Verify
    let attrs = client
        .get_queue_attributes()
        .queue_url(&src_url)
        .attribute_names(aws_sdk_sqs::types::QueueAttributeName::RedrivePolicy)
        .send()
        .await
        .unwrap();
    let rp = attrs
        .attributes()
        .unwrap()
        .get(&aws_sdk_sqs::types::QueueAttributeName::RedrivePolicy)
        .expect("RedrivePolicy should be present after SetQueueAttributes");
    let parsed: serde_json::Value = serde_json::from_str(rp).unwrap();
    assert_eq!(parsed["deadLetterTargetArn"].as_str().unwrap(), dlq_arn);
}

// Ported from moto: test_sqs.py::test_get_queue_attributes (full attribute set)
#[tokio::test]
async fn test_moto_get_queue_attributes_full() {
    let client = make_sqs_client().await;

    // Create DLQ
    let dlq_resp = client
        .create_queue()
        .queue_name("full-attr-dlq")
        .send()
        .await
        .unwrap();
    let dlq_url = dlq_resp.queue_url().unwrap().to_string();
    let dlq_attrs = client
        .get_queue_attributes()
        .queue_url(&dlq_url)
        .attribute_names(aws_sdk_sqs::types::QueueAttributeName::QueueArn)
        .send()
        .await
        .unwrap();
    let dlq_arn = dlq_attrs
        .attributes()
        .unwrap()
        .get(&aws_sdk_sqs::types::QueueAttributeName::QueueArn)
        .unwrap()
        .clone();

    let redrive_policy = format!(
        r#"{{"deadLetterTargetArn":"{}","maxReceiveCount":2}}"#,
        dlq_arn
    );

    let resp = client
        .create_queue()
        .queue_name("full-attr-queue")
        .attributes(
            aws_sdk_sqs::types::QueueAttributeName::RedrivePolicy,
            &redrive_policy,
        )
        .send()
        .await
        .unwrap();
    let queue_url = resp.queue_url().unwrap().to_string();

    let attrs = client
        .get_queue_attributes()
        .queue_url(&queue_url)
        .attribute_names(aws_sdk_sqs::types::QueueAttributeName::All)
        .send()
        .await
        .unwrap();
    let a = attrs.attributes().unwrap();

    assert_eq!(
        a.get(&aws_sdk_sqs::types::QueueAttributeName::ApproximateNumberOfMessages)
            .unwrap(),
        "0"
    );
    assert_eq!(
        a.get(&aws_sdk_sqs::types::QueueAttributeName::ApproximateNumberOfMessagesDelayed)
            .unwrap(),
        "0"
    );
    assert_eq!(
        a.get(&aws_sdk_sqs::types::QueueAttributeName::ApproximateNumberOfMessagesNotVisible)
            .unwrap(),
        "0"
    );
    assert_eq!(
        a.get(&aws_sdk_sqs::types::QueueAttributeName::DelaySeconds)
            .unwrap(),
        "0"
    );
    assert_eq!(
        a.get(&aws_sdk_sqs::types::QueueAttributeName::MaximumMessageSize)
            .unwrap(),
        "262144"
    );
    assert_eq!(
        a.get(&aws_sdk_sqs::types::QueueAttributeName::MessageRetentionPeriod)
            .unwrap(),
        "345600"
    );
    assert_eq!(
        a.get(&aws_sdk_sqs::types::QueueAttributeName::ReceiveMessageWaitTimeSeconds)
            .unwrap(),
        "0"
    );
    assert_eq!(
        a.get(&aws_sdk_sqs::types::QueueAttributeName::VisibilityTimeout)
            .unwrap(),
        "30"
    );
    assert!(
        a.get(&aws_sdk_sqs::types::QueueAttributeName::CreatedTimestamp)
            .is_some()
    );
    assert!(
        a.get(&aws_sdk_sqs::types::QueueAttributeName::LastModifiedTimestamp)
            .is_some()
    );
    assert!(
        a.get(&aws_sdk_sqs::types::QueueAttributeName::RedrivePolicy)
            .is_some(),
        "RedrivePolicy should be in All attributes"
    );
}

// Ported from moto: test_sqs.py::test_delete_message_twice_using_same_receipt_handle
#[tokio::test]
async fn test_moto_delete_message_twice_same_receipt() {
    let client = make_sqs_client().await;

    let resp = client
        .create_queue()
        .queue_name("del-twice-queue")
        .send()
        .await
        .unwrap();
    let queue_url = resp.queue_url().unwrap().to_string();

    client
        .send_message()
        .queue_url(&queue_url)
        .message_body("body")
        .send()
        .await
        .unwrap();

    let recv = client
        .receive_message()
        .queue_url(&queue_url)
        .send()
        .await
        .unwrap();
    let receipt_handle = recv.messages()[0].receipt_handle().unwrap().to_string();

    // First delete
    client
        .delete_message()
        .queue_url(&queue_url)
        .receipt_handle(&receipt_handle)
        .send()
        .await
        .unwrap();

    // Second delete should also succeed (idempotent)
    client
        .delete_message()
        .queue_url(&queue_url)
        .receipt_handle(&receipt_handle)
        .send()
        .await
        .unwrap();
}

// Ported from moto: test_sqs.py::test_delete_message_using_old_receipt_handle
#[tokio::test]
async fn test_moto_delete_message_old_receipt_handle() {
    let client = make_sqs_client().await;

    let resp = client
        .create_queue()
        .queue_name("del-old-receipt-queue")
        .attributes(
            aws_sdk_sqs::types::QueueAttributeName::VisibilityTimeout,
            "0",
        )
        .send()
        .await
        .unwrap();
    let queue_url = resp.queue_url().unwrap().to_string();

    client
        .send_message()
        .queue_url(&queue_url)
        .message_body("body")
        .send()
        .await
        .unwrap();

    // Receive first time
    let recv1 = client
        .receive_message()
        .queue_url(&queue_url)
        .send()
        .await
        .unwrap();
    let receipt_1 = recv1.messages()[0].receipt_handle().unwrap().to_string();

    // With visibility_timeout=0, the message becomes immediately visible again.
    // Receive second time
    let recv2 = client
        .receive_message()
        .queue_url(&queue_url)
        .send()
        .await
        .unwrap();
    let receipt_2 = recv2.messages()[0].receipt_handle().unwrap().to_string();

    assert_ne!(receipt_1, receipt_2);

    // Delete using the OLD receipt handle should work
    client
        .delete_message()
        .queue_url(&queue_url)
        .receipt_handle(&receipt_1)
        .send()
        .await
        .unwrap();

    // Message should be gone
    let recv3 = client
        .receive_message()
        .queue_url(&queue_url)
        .send()
        .await
        .unwrap();
    assert_eq!(recv3.messages().len(), 0, "message should be deleted");

    // Deleting again with old receipt should succeed (idempotent)
    client
        .delete_message()
        .queue_url(&queue_url)
        .receipt_handle(&receipt_1)
        .send()
        .await
        .unwrap();

    // Deleting with second receipt handle should also succeed
    client
        .delete_message()
        .queue_url(&queue_url)
        .receipt_handle(&receipt_2)
        .send()
        .await
        .unwrap();
}

// Ported from moto: test_sqs.py::test_change_message_visibility_on_unknown_receipt_handle
#[tokio::test]
async fn test_moto_change_visibility_unknown_receipt() {
    let client = make_sqs_client().await;

    let resp = client
        .create_queue()
        .queue_name("vis-unknown-receipt-queue")
        .attributes(
            aws_sdk_sqs::types::QueueAttributeName::VisibilityTimeout,
            "2",
        )
        .send()
        .await
        .unwrap();
    let queue_url = resp.queue_url().unwrap().to_string();

    let err = client
        .change_message_visibility()
        .queue_url(&queue_url)
        .receipt_handle("unknown-stuff")
        .visibility_timeout(432)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ReceiptHandleIsInvalid"),
        "Expected ReceiptHandleIsInvalid, got: {err_str}"
    );
}

// Ported from moto: test_sqs.py::test_queue_length
#[tokio::test]
async fn test_moto_queue_length() {
    let client = make_sqs_client().await;

    let resp = client
        .create_queue()
        .queue_name("queue-length-queue")
        .send()
        .await
        .unwrap();
    let queue_url = resp.queue_url().unwrap().to_string();

    client
        .send_message()
        .queue_url(&queue_url)
        .message_body("this is a test message")
        .send()
        .await
        .unwrap();
    client
        .send_message()
        .queue_url(&queue_url)
        .message_body("this is another test message")
        .send()
        .await
        .unwrap();

    let attrs = client
        .get_queue_attributes()
        .queue_url(&queue_url)
        .attribute_names(aws_sdk_sqs::types::QueueAttributeName::ApproximateNumberOfMessages)
        .send()
        .await
        .unwrap();
    let count = attrs
        .attributes()
        .unwrap()
        .get(&aws_sdk_sqs::types::QueueAttributeName::ApproximateNumberOfMessages)
        .unwrap();
    assert_eq!(count, "2");
}

// Ported from moto: test_sqs.py::test_send_message_with_message_group_id
#[tokio::test]
async fn test_moto_send_message_with_message_group_id() {
    let client = make_sqs_client().await;

    let resp = client
        .create_queue()
        .queue_name("msg-group-queue.fifo")
        .attributes(aws_sdk_sqs::types::QueueAttributeName::FifoQueue, "true")
        .send()
        .await
        .unwrap();
    let queue_url = resp.queue_url().unwrap().to_string();

    client
        .send_message()
        .queue_url(&queue_url)
        .message_body("mydata")
        .message_deduplication_id("dedupe_id_1")
        .message_group_id("group_id_1")
        .send()
        .await
        .unwrap();

    let recv = client
        .receive_message()
        .queue_url(&queue_url)
        .max_number_of_messages(1)
        .send()
        .await
        .unwrap();

    assert_eq!(recv.messages().len(), 1);
    assert_eq!(recv.messages()[0].body().unwrap(), "mydata");
}

// Ported from moto: test_sqs.py::test_create_queue_with_policy
#[tokio::test]
async fn test_moto_create_queue_with_policy() {
    let client = make_sqs_client().await;

    let policy = r#"{"Version":"2012-10-17","Id":"test","Statement":[{"Effect":"Allow","Principal":"*","Action":"*"}]}"#;

    let resp = client
        .create_queue()
        .queue_name("policy-queue")
        .attributes(aws_sdk_sqs::types::QueueAttributeName::Policy, policy)
        .send()
        .await
        .unwrap();
    let queue_url = resp.queue_url().unwrap().to_string();

    let attrs = client
        .get_queue_attributes()
        .queue_url(&queue_url)
        .attribute_names(aws_sdk_sqs::types::QueueAttributeName::Policy)
        .send()
        .await
        .unwrap();
    let returned_policy = attrs
        .attributes()
        .unwrap()
        .get(&aws_sdk_sqs::types::QueueAttributeName::Policy)
        .expect("Policy should be present");

    let parsed: serde_json::Value = serde_json::from_str(returned_policy).unwrap();
    assert_eq!(parsed["Version"], "2012-10-17");
    assert_eq!(parsed["Id"], "test");
}

// Ported from moto: test_sqs.py::test_set_queue_attribute_empty_policy_removes_attr
#[tokio::test]
async fn test_moto_set_queue_attribute_empty_policy_removes() {
    let client = make_sqs_client().await;

    let policy =
        r#"{"Version":"2012-10-17","Statement":[{"Effect":"Allow","Principal":"*","Action":"*"}]}"#;
    let resp = client
        .create_queue()
        .queue_name("empty-policy-queue")
        .attributes(aws_sdk_sqs::types::QueueAttributeName::Policy, policy)
        .send()
        .await
        .unwrap();
    let queue_url = resp.queue_url().unwrap().to_string();

    // Remove policy by setting empty string
    client
        .set_queue_attributes()
        .queue_url(&queue_url)
        .attributes(aws_sdk_sqs::types::QueueAttributeName::Policy, "")
        .send()
        .await
        .unwrap();

    let attrs = client
        .get_queue_attributes()
        .queue_url(&queue_url)
        .attribute_names(aws_sdk_sqs::types::QueueAttributeName::All)
        .send()
        .await
        .unwrap();
    let a = attrs.attributes().unwrap();
    assert!(
        a.get(&aws_sdk_sqs::types::QueueAttributeName::Policy)
            .is_none(),
        "Policy should be removed after setting empty"
    );
}

// Ported from moto: test_sqs.py::test_list_dead_letter_source_queues (using set_queue_attributes)
#[tokio::test]
async fn test_moto_list_dlq_source_queues_via_set_attributes() {
    let client = make_sqs_client().await;

    // Create DLQ
    let dlq_resp = client
        .create_queue()
        .queue_name("dlq-via-set-attr")
        .send()
        .await
        .unwrap();
    let dlq_url = dlq_resp.queue_url().unwrap().to_string();
    let dlq_attrs = client
        .get_queue_attributes()
        .queue_url(&dlq_url)
        .attribute_names(aws_sdk_sqs::types::QueueAttributeName::QueueArn)
        .send()
        .await
        .unwrap();
    let dlq_arn = dlq_attrs
        .attributes()
        .unwrap()
        .get(&aws_sdk_sqs::types::QueueAttributeName::QueueArn)
        .unwrap()
        .clone();

    // Create source queue without redrive policy
    let src_resp = client
        .create_queue()
        .queue_name("src-via-set-attr")
        .send()
        .await
        .unwrap();
    let src_url = src_resp.queue_url().unwrap().to_string();

    // Set redrive policy via SetQueueAttributes
    let redrive_policy = format!(
        r#"{{"deadLetterTargetArn":"{}","maxReceiveCount":5}}"#,
        dlq_arn
    );
    client
        .set_queue_attributes()
        .queue_url(&src_url)
        .attributes(
            aws_sdk_sqs::types::QueueAttributeName::RedrivePolicy,
            &redrive_policy,
        )
        .send()
        .await
        .unwrap();

    // List dead letter source queues
    let resp = client
        .list_dead_letter_source_queues()
        .queue_url(&dlq_url)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.queue_urls().len(), 1);
    assert!(resp.queue_urls()[0].contains("src-via-set-attr"));
}

// Ported from moto: test_sqs_message_attributes.py::test_message_attributes_should_not_include_system_attributes
#[tokio::test]
async fn test_moto_message_attributes_no_system_attrs() {
    let client = make_sqs_client().await;

    let resp = client
        .create_queue()
        .queue_name("msg-attr-no-sys")
        .send()
        .await
        .unwrap();
    let queue_url = resp.queue_url().unwrap().to_string();

    // Send without message attributes
    client
        .send_message()
        .queue_url(&queue_url)
        .message_body("Hello from app startup!")
        .send()
        .await
        .unwrap();

    // Receive with All system attributes
    let recv = client
        .receive_message()
        .queue_url(&queue_url)
        .message_system_attribute_names(aws_sdk_sqs::types::MessageSystemAttributeName::All)
        .message_attribute_names("All")
        .max_number_of_messages(1)
        .send()
        .await
        .unwrap();

    let msg = &recv.messages()[0];

    // System attributes should be present
    assert!(msg.attributes().is_some());
    let sys_attrs = msg.attributes().unwrap();
    assert!(sys_attrs.contains_key(&aws_sdk_sqs::types::MessageSystemAttributeName::SenderId));
    assert!(sys_attrs.contains_key(&aws_sdk_sqs::types::MessageSystemAttributeName::SentTimestamp));

    // MessageAttributes should be None or empty (no custom attrs sent)
    let msg_attrs = msg.message_attributes();
    assert!(
        msg_attrs.is_none() || msg_attrs.unwrap().is_empty(),
        "MessageAttributes should be empty when none were sent"
    );
}

// Ported from moto: test_sqs_message_attributes.py::test_message_attributes_work_correctly_when_present
#[tokio::test]
async fn test_moto_message_attributes_present() {
    let client = make_sqs_client().await;

    let resp = client
        .create_queue()
        .queue_name("msg-attr-present")
        .send()
        .await
        .unwrap();
    let queue_url = resp.queue_url().unwrap().to_string();

    client
        .send_message()
        .queue_url(&queue_url)
        .message_body("Hello")
        .message_attributes(
            "Author",
            aws_sdk_sqs::types::MessageAttributeValue::builder()
                .data_type("String")
                .string_value("John Doe")
                .build()
                .unwrap(),
        )
        .message_attributes(
            "Priority",
            aws_sdk_sqs::types::MessageAttributeValue::builder()
                .data_type("Number")
                .string_value("1")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let recv = client
        .receive_message()
        .queue_url(&queue_url)
        .message_system_attribute_names(aws_sdk_sqs::types::MessageSystemAttributeName::All)
        .message_attribute_names("All")
        .max_number_of_messages(1)
        .send()
        .await
        .unwrap();

    let msg = &recv.messages()[0];

    // System attributes should be present
    let sys_attrs = msg.attributes().unwrap();
    assert!(sys_attrs.contains_key(&aws_sdk_sqs::types::MessageSystemAttributeName::SenderId));

    // Message attributes should contain our custom attrs
    let msg_attrs = msg.message_attributes().unwrap();
    assert_eq!(msg_attrs.len(), 2);
    assert_eq!(
        msg_attrs.get("Author").unwrap().string_value().unwrap(),
        "John Doe"
    );
    assert_eq!(
        msg_attrs.get("Priority").unwrap().string_value().unwrap(),
        "1"
    );

    // System attributes should NOT be in MessageAttributes
    assert!(!msg_attrs.contains_key("SenderId"));
    assert!(!msg_attrs.contains_key("SentTimestamp"));
}

// Ported from moto: test_sqs.py::test_create_fifo_queue_invalid_name
#[tokio::test]
async fn test_moto_create_queue_invalid_name() {
    let client = make_sqs_client().await;

    // Empty name
    let err = client
        .create_queue()
        .queue_name("")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("InvalidParameterValue"),
        "Expected InvalidParameterValue for empty name, got: {err_str}"
    );

    // Name with invalid characters
    let err = client
        .create_queue()
        .queue_name("/my/test")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("InvalidParameterValue"),
        "Expected InvalidParameterValue for invalid chars, got: {err_str}"
    );
}

// Ported from moto: test_sqs.py::test_send_receive_message_with_attributes
#[tokio::test]
async fn test_moto_send_receive_with_attributes_roundtrip() {
    let client = make_sqs_client().await;

    let resp = client
        .create_queue()
        .queue_name("attr-roundtrip-queue")
        .send()
        .await
        .unwrap();
    let queue_url = resp.queue_url().unwrap().to_string();

    client
        .send_message()
        .queue_url(&queue_url)
        .message_body("test body")
        .message_attributes(
            "name1",
            aws_sdk_sqs::types::MessageAttributeValue::builder()
                .data_type("String")
                .string_value("value1")
                .build()
                .unwrap(),
        )
        .message_attributes(
            "name2",
            aws_sdk_sqs::types::MessageAttributeValue::builder()
                .data_type("Number")
                .string_value("42")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let recv = client
        .receive_message()
        .queue_url(&queue_url)
        .message_attribute_names("All")
        .max_number_of_messages(1)
        .send()
        .await
        .unwrap();

    let msg = &recv.messages()[0];
    assert_eq!(msg.body().unwrap(), "test body");

    let attrs = msg.message_attributes().unwrap();
    assert_eq!(attrs.len(), 2);
    assert_eq!(
        attrs.get("name1").unwrap().string_value().unwrap(),
        "value1"
    );
    assert_eq!(attrs.get("name1").unwrap().data_type(), "String");
    assert_eq!(attrs.get("name2").unwrap().string_value().unwrap(), "42");
    assert_eq!(attrs.get("name2").unwrap().data_type(), "Number");
}

// ============================================================================
// Coverage for FIX(terraform-e2e) handler fixes
// ============================================================================

// Covers FIX(terraform-e2e): Parse and store ContentBasedDeduplication for FIFO queues.
// Without this fix, GetQueueAttributes always returned "false" regardless of CreateQueue input.
#[tokio::test]
async fn test_fifo_content_based_deduplication_on_create() {
    let client = make_sqs_client().await;

    let resp = client
        .create_queue()
        .queue_name("fix-dedup-on.fifo")
        .attributes(aws_sdk_sqs::types::QueueAttributeName::FifoQueue, "true")
        .attributes(
            aws_sdk_sqs::types::QueueAttributeName::ContentBasedDeduplication,
            "true",
        )
        .send()
        .await
        .unwrap();
    let queue_url = resp.queue_url().unwrap().to_string();

    let attrs_resp = client
        .get_queue_attributes()
        .queue_url(&queue_url)
        .attribute_names(aws_sdk_sqs::types::QueueAttributeName::All)
        .send()
        .await
        .unwrap();
    let attrs = attrs_resp.attributes().unwrap();

    assert_eq!(
        attrs
            .get(&aws_sdk_sqs::types::QueueAttributeName::ContentBasedDeduplication)
            .unwrap(),
        "true",
        "ContentBasedDeduplication should be true when set on create"
    );
    assert_eq!(
        attrs
            .get(&aws_sdk_sqs::types::QueueAttributeName::FifoQueue)
            .unwrap(),
        "true"
    );
}

// Covers FIX(terraform-e2e): ContentBasedDeduplication defaults to false for FIFO queues.
#[tokio::test]
async fn test_fifo_content_based_deduplication_default_false() {
    let client = make_sqs_client().await;

    let resp = client
        .create_queue()
        .queue_name("fix-dedup-default.fifo")
        .attributes(aws_sdk_sqs::types::QueueAttributeName::FifoQueue, "true")
        .send()
        .await
        .unwrap();
    let queue_url = resp.queue_url().unwrap().to_string();

    let attrs_resp = client
        .get_queue_attributes()
        .queue_url(&queue_url)
        .attribute_names(aws_sdk_sqs::types::QueueAttributeName::All)
        .send()
        .await
        .unwrap();
    let attrs = attrs_resp.attributes().unwrap();

    assert_eq!(
        attrs
            .get(&aws_sdk_sqs::types::QueueAttributeName::ContentBasedDeduplication)
            .unwrap(),
        "false",
        "ContentBasedDeduplication should default to false for FIFO queues"
    );
}

// Covers FIX(terraform-e2e): SqsManagedSseEnabled must be present in GetQueueAttributes.
// The terraform provider polls until it sees this attribute.
#[tokio::test]
async fn test_sqs_managed_sse_enabled_present() {
    let client = make_sqs_client().await;

    let resp = client
        .create_queue()
        .queue_name("fix-sse-standard")
        .send()
        .await
        .unwrap();
    let queue_url = resp.queue_url().unwrap().to_string();

    let attrs_resp = client
        .get_queue_attributes()
        .queue_url(&queue_url)
        .attribute_names(aws_sdk_sqs::types::QueueAttributeName::All)
        .send()
        .await
        .unwrap();
    let attrs = attrs_resp.attributes().unwrap();

    assert_eq!(
        attrs
            .get(&aws_sdk_sqs::types::QueueAttributeName::SqsManagedSseEnabled)
            .unwrap(),
        "true",
        "SqsManagedSseEnabled should be present and set to true for standard queues"
    );
}

// Covers FIX(terraform-e2e): SqsManagedSseEnabled must also be present for FIFO queues.
#[tokio::test]
async fn test_sqs_managed_sse_enabled_fifo() {
    let client = make_sqs_client().await;

    let resp = client
        .create_queue()
        .queue_name("fix-sse-fifo.fifo")
        .attributes(aws_sdk_sqs::types::QueueAttributeName::FifoQueue, "true")
        .send()
        .await
        .unwrap();
    let queue_url = resp.queue_url().unwrap().to_string();

    let attrs_resp = client
        .get_queue_attributes()
        .queue_url(&queue_url)
        .attribute_names(aws_sdk_sqs::types::QueueAttributeName::All)
        .send()
        .await
        .unwrap();
    let attrs = attrs_resp.attributes().unwrap();

    assert_eq!(
        attrs
            .get(&aws_sdk_sqs::types::QueueAttributeName::SqsManagedSseEnabled)
            .unwrap(),
        "true",
        "SqsManagedSseEnabled should be present and set to true for FIFO queues"
    );
}

// Covers FIX(terraform-e2e): Allow updating ContentBasedDeduplication via SetQueueAttributes.
#[tokio::test]
async fn test_set_content_based_deduplication() {
    let client = make_sqs_client().await;

    // Create FIFO queue without dedup
    let resp = client
        .create_queue()
        .queue_name("fix-set-dedup.fifo")
        .attributes(aws_sdk_sqs::types::QueueAttributeName::FifoQueue, "true")
        .send()
        .await
        .unwrap();
    let queue_url = resp.queue_url().unwrap().to_string();

    // Verify default is false
    let attrs_resp = client
        .get_queue_attributes()
        .queue_url(&queue_url)
        .attribute_names(aws_sdk_sqs::types::QueueAttributeName::All)
        .send()
        .await
        .unwrap();
    assert_eq!(
        attrs_resp
            .attributes()
            .unwrap()
            .get(&aws_sdk_sqs::types::QueueAttributeName::ContentBasedDeduplication)
            .unwrap(),
        "false"
    );

    // Enable ContentBasedDeduplication via SetQueueAttributes
    client
        .set_queue_attributes()
        .queue_url(&queue_url)
        .attributes(
            aws_sdk_sqs::types::QueueAttributeName::ContentBasedDeduplication,
            "true",
        )
        .send()
        .await
        .expect("SetQueueAttributes for ContentBasedDeduplication should succeed");

    // Verify it is now true
    let attrs_resp = client
        .get_queue_attributes()
        .queue_url(&queue_url)
        .attribute_names(aws_sdk_sqs::types::QueueAttributeName::All)
        .send()
        .await
        .unwrap();
    assert_eq!(
        attrs_resp
            .attributes()
            .unwrap()
            .get(&aws_sdk_sqs::types::QueueAttributeName::ContentBasedDeduplication)
            .unwrap(),
        "true",
        "ContentBasedDeduplication should be true after SetQueueAttributes"
    );

    // Disable it again
    client
        .set_queue_attributes()
        .queue_url(&queue_url)
        .attributes(
            aws_sdk_sqs::types::QueueAttributeName::ContentBasedDeduplication,
            "false",
        )
        .send()
        .await
        .unwrap();

    let attrs_resp = client
        .get_queue_attributes()
        .queue_url(&queue_url)
        .attribute_names(aws_sdk_sqs::types::QueueAttributeName::All)
        .send()
        .await
        .unwrap();
    assert_eq!(
        attrs_resp
            .attributes()
            .unwrap()
            .get(&aws_sdk_sqs::types::QueueAttributeName::ContentBasedDeduplication)
            .unwrap(),
        "false",
        "ContentBasedDeduplication should be false after disabling"
    );
}

// Covers FIX(terraform-e2e): Full FIFO lifecycle with ContentBasedDeduplication.
// Tests create -> get attributes -> set attributes -> delete.
#[tokio::test]
async fn test_fifo_queue_lifecycle_with_deduplication() {
    let client = make_sqs_client().await;

    // Create FIFO queue with ContentBasedDeduplication enabled
    let resp = client
        .create_queue()
        .queue_name("fix-lifecycle-fifo.fifo")
        .attributes(aws_sdk_sqs::types::QueueAttributeName::FifoQueue, "true")
        .attributes(
            aws_sdk_sqs::types::QueueAttributeName::ContentBasedDeduplication,
            "true",
        )
        .send()
        .await
        .unwrap();
    let queue_url = resp.queue_url().unwrap().to_string();

    // Verify all FIFO-specific attributes
    let attrs_resp = client
        .get_queue_attributes()
        .queue_url(&queue_url)
        .attribute_names(aws_sdk_sqs::types::QueueAttributeName::All)
        .send()
        .await
        .unwrap();
    let attrs = attrs_resp.attributes().unwrap();

    assert_eq!(
        attrs
            .get(&aws_sdk_sqs::types::QueueAttributeName::FifoQueue)
            .unwrap(),
        "true"
    );
    assert_eq!(
        attrs
            .get(&aws_sdk_sqs::types::QueueAttributeName::ContentBasedDeduplication)
            .unwrap(),
        "true"
    );
    assert_eq!(
        attrs
            .get(&aws_sdk_sqs::types::QueueAttributeName::SqsManagedSseEnabled)
            .unwrap(),
        "true"
    );
    // DeduplicationScope and FifoThroughputLimit should also be present
    assert_eq!(
        attrs
            .get(&aws_sdk_sqs::types::QueueAttributeName::DeduplicationScope)
            .unwrap(),
        "queue"
    );
    assert_eq!(
        attrs
            .get(&aws_sdk_sqs::types::QueueAttributeName::FifoThroughputLimit)
            .unwrap(),
        "perQueue"
    );

    // Update: disable ContentBasedDeduplication
    client
        .set_queue_attributes()
        .queue_url(&queue_url)
        .attributes(
            aws_sdk_sqs::types::QueueAttributeName::ContentBasedDeduplication,
            "false",
        )
        .send()
        .await
        .unwrap();

    // Verify update
    let attrs_resp = client
        .get_queue_attributes()
        .queue_url(&queue_url)
        .attribute_names(aws_sdk_sqs::types::QueueAttributeName::All)
        .send()
        .await
        .unwrap();
    assert_eq!(
        attrs_resp
            .attributes()
            .unwrap()
            .get(&aws_sdk_sqs::types::QueueAttributeName::ContentBasedDeduplication)
            .unwrap(),
        "false"
    );

    // Delete
    client
        .delete_queue()
        .queue_url(&queue_url)
        .send()
        .await
        .expect("delete_queue should succeed");

    // Verify gone
    let err = client
        .get_queue_url()
        .queue_name("fix-lifecycle-fifo.fifo")
        .send()
        .await;
    assert!(err.is_err(), "queue should not exist after delete");
}

#[tokio::test]
async fn test_start_message_move_task() {
    let client = make_sqs_client().await;

    // Create a DLQ
    let dlq_resp = client
        .create_queue()
        .queue_name("move-dlq")
        .send()
        .await
        .unwrap();
    let dlq_url = dlq_resp.queue_url().unwrap().to_string();

    // Get the DLQ ARN
    let dlq_attrs = client
        .get_queue_attributes()
        .queue_url(&dlq_url)
        .attribute_names(aws_sdk_sqs::types::QueueAttributeName::All)
        .send()
        .await
        .unwrap();
    let dlq_arn = dlq_attrs
        .attributes()
        .unwrap()
        .get(&aws_sdk_sqs::types::QueueAttributeName::QueueArn)
        .unwrap()
        .clone();

    // Start a message move task
    let resp = client
        .start_message_move_task()
        .source_arn(&dlq_arn)
        .max_number_of_messages_per_second(100)
        .send()
        .await
        .expect("start_message_move_task should succeed");

    let task_handle = resp.task_handle().expect("should have task handle");
    assert!(!task_handle.is_empty(), "task handle should not be empty");
}

#[tokio::test]
async fn test_cancel_message_move_task() {
    let client = make_sqs_client().await;

    // Create a DLQ
    let dlq_resp = client
        .create_queue()
        .queue_name("cancel-move-dlq")
        .send()
        .await
        .unwrap();
    let dlq_url = dlq_resp.queue_url().unwrap().to_string();

    // Get the DLQ ARN
    let dlq_attrs = client
        .get_queue_attributes()
        .queue_url(&dlq_url)
        .attribute_names(aws_sdk_sqs::types::QueueAttributeName::All)
        .send()
        .await
        .unwrap();
    let dlq_arn = dlq_attrs
        .attributes()
        .unwrap()
        .get(&aws_sdk_sqs::types::QueueAttributeName::QueueArn)
        .unwrap()
        .clone();

    // Start a message move task
    let start_resp = client
        .start_message_move_task()
        .source_arn(&dlq_arn)
        .send()
        .await
        .expect("start_message_move_task should succeed");
    let task_handle = start_resp.task_handle().unwrap().to_string();

    // Cancel the task
    let cancel_resp = client
        .cancel_message_move_task()
        .task_handle(&task_handle)
        .send()
        .await
        .expect("cancel_message_move_task should succeed");

    assert_eq!(
        cancel_resp.approximate_number_of_messages_moved(),
        0,
        "no messages should have been moved"
    );
}

#[tokio::test]
async fn test_list_message_move_tasks() {
    let client = make_sqs_client().await;

    // Create a DLQ
    let dlq_resp = client
        .create_queue()
        .queue_name("list-move-dlq")
        .send()
        .await
        .unwrap();
    let dlq_url = dlq_resp.queue_url().unwrap().to_string();

    // Get the DLQ ARN
    let dlq_attrs = client
        .get_queue_attributes()
        .queue_url(&dlq_url)
        .attribute_names(aws_sdk_sqs::types::QueueAttributeName::All)
        .send()
        .await
        .unwrap();
    let dlq_arn = dlq_attrs
        .attributes()
        .unwrap()
        .get(&aws_sdk_sqs::types::QueueAttributeName::QueueArn)
        .unwrap()
        .clone();

    // Start a message move task
    client
        .start_message_move_task()
        .source_arn(&dlq_arn)
        .max_number_of_messages_per_second(50)
        .send()
        .await
        .expect("start_message_move_task should succeed");

    // List message move tasks
    let list_resp = client
        .list_message_move_tasks()
        .source_arn(&dlq_arn)
        .send()
        .await
        .expect("list_message_move_tasks should succeed");

    let results = list_resp.results();
    assert_eq!(results.len(), 1, "should have 1 task");
    let task = &results[0];
    assert_eq!(task.source_arn().unwrap(), &dlq_arn);
    assert_eq!(task.status().unwrap(), "RUNNING");
    assert_eq!(task.max_number_of_messages_per_second(), Some(50));
}

#[tokio::test]
async fn test_list_message_move_tasks_empty() {
    let client = make_sqs_client().await;

    // Create a DLQ
    let dlq_resp = client
        .create_queue()
        .queue_name("empty-move-dlq")
        .send()
        .await
        .unwrap();
    let dlq_url = dlq_resp.queue_url().unwrap().to_string();

    // Get the DLQ ARN
    let dlq_attrs = client
        .get_queue_attributes()
        .queue_url(&dlq_url)
        .attribute_names(aws_sdk_sqs::types::QueueAttributeName::All)
        .send()
        .await
        .unwrap();
    let dlq_arn = dlq_attrs
        .attributes()
        .unwrap()
        .get(&aws_sdk_sqs::types::QueueAttributeName::QueueArn)
        .unwrap()
        .clone();

    // List message move tasks (none exist)
    let list_resp = client
        .list_message_move_tasks()
        .source_arn(&dlq_arn)
        .send()
        .await
        .expect("list_message_move_tasks should succeed");

    let results = list_resp.results();
    assert!(results.is_empty(), "should have no tasks");
}

// ---------------------------------------------------------------------------
// Regression tests for the five SQS bugs documented in TODO.md
// (sqs-redis-* — verifying behaviour against the in-memory backend)
// ---------------------------------------------------------------------------

/// Regression for sqs-redis-default-visibility-timeout-not-applied:
/// when the queue's `VisibilityTimeout` is 10 and the caller does not pass an
/// explicit `--visibility-timeout`, a second receive immediately afterwards
/// must NOT return the same message.
#[tokio::test]
async fn test_default_visibility_timeout_applied_on_receive() {
    let client = make_sqs_client().await;

    let resp = client
        .create_queue()
        .queue_name("default-vt-queue")
        .attributes(
            aws_sdk_sqs::types::QueueAttributeName::VisibilityTimeout,
            "10",
        )
        .send()
        .await
        .unwrap();
    let queue_url = resp.queue_url().unwrap().to_string();

    client
        .send_message()
        .queue_url(&queue_url)
        .message_body("hello")
        .send()
        .await
        .unwrap();

    // First receive: should return the message.
    let recv1 = client
        .receive_message()
        .queue_url(&queue_url)
        .send()
        .await
        .unwrap();
    assert_eq!(
        recv1.messages().len(),
        1,
        "first receive should return the message"
    );

    // Second receive immediately afterwards: queue default is 10 s, so the
    // message must be hidden.
    let recv2 = client
        .receive_message()
        .queue_url(&queue_url)
        .send()
        .await
        .unwrap();
    assert_eq!(
        recv2.messages().len(),
        0,
        "second receive should not return the message (queue VisibilityTimeout=10 must hide it)"
    );
}

/// Regression for sqs-redis-delay-seconds-ignored:
/// per-message `DelaySeconds` must hide the message until the delay elapses,
/// and `ApproximateNumberOfMessagesDelayed` must reflect that.
#[tokio::test]
async fn test_delay_seconds_per_message_hides_and_counts() {
    let client = make_sqs_client().await;

    let resp = client
        .create_queue()
        .queue_name("delay-msg-queue")
        .send()
        .await
        .unwrap();
    let queue_url = resp.queue_url().unwrap().to_string();

    client
        .send_message()
        .queue_url(&queue_url)
        .message_body("delayed")
        .delay_seconds(5)
        .send()
        .await
        .unwrap();

    // Receive immediately must return nothing.
    let recv = client
        .receive_message()
        .queue_url(&queue_url)
        .visibility_timeout(60)
        .send()
        .await
        .unwrap();
    assert_eq!(
        recv.messages().len(),
        0,
        "delayed message should not be visible immediately"
    );

    // Counters must classify the message as Delayed, not NotVisible.
    let attrs = client
        .get_queue_attributes()
        .queue_url(&queue_url)
        .attribute_names(aws_sdk_sqs::types::QueueAttributeName::All)
        .send()
        .await
        .unwrap();
    let attrs = attrs.attributes().unwrap();
    let delayed = attrs
        .get(&aws_sdk_sqs::types::QueueAttributeName::ApproximateNumberOfMessagesDelayed)
        .map(|s| s.as_str())
        .unwrap_or("0");
    let not_visible = attrs
        .get(&aws_sdk_sqs::types::QueueAttributeName::ApproximateNumberOfMessagesNotVisible)
        .map(|s| s.as_str())
        .unwrap_or("0");
    assert_eq!(delayed, "1", "delayed message should be counted as Delayed");
    assert_eq!(
        not_visible, "0",
        "delayed message must not be counted as NotVisible"
    );
}

/// Regression for sqs-redis-redrive-policy-not-enforced:
/// once a message has been received `maxReceiveCount` times without being
/// deleted, it must move to the configured DLQ.
#[tokio::test]
async fn test_redrive_policy_moves_to_dlq_after_max_receive_count() {
    let client = make_sqs_client().await;

    let dlq_resp = client
        .create_queue()
        .queue_name("redrive-target-dlq")
        .send()
        .await
        .unwrap();
    let dlq_url = dlq_resp.queue_url().unwrap().to_string();

    let dlq_attrs = client
        .get_queue_attributes()
        .queue_url(&dlq_url)
        .attribute_names(aws_sdk_sqs::types::QueueAttributeName::QueueArn)
        .send()
        .await
        .unwrap();
    let dlq_arn = dlq_attrs
        .attributes()
        .unwrap()
        .get(&aws_sdk_sqs::types::QueueAttributeName::QueueArn)
        .unwrap()
        .clone();

    let redrive_policy = format!(
        r#"{{"deadLetterTargetArn":"{}","maxReceiveCount":"2"}}"#,
        dlq_arn
    );
    let src_resp = client
        .create_queue()
        .queue_name("redrive-source-q")
        .attributes(
            aws_sdk_sqs::types::QueueAttributeName::RedrivePolicy,
            &redrive_policy,
        )
        .attributes(
            aws_sdk_sqs::types::QueueAttributeName::VisibilityTimeout,
            "0",
        )
        .send()
        .await
        .unwrap();
    let src_url = src_resp.queue_url().unwrap().to_string();

    client
        .send_message()
        .queue_url(&src_url)
        .message_body("redrive-payload")
        .send()
        .await
        .unwrap();

    // Receive thrice with VisibilityTimeout=0 — the third receive should
    // exceed maxReceiveCount=2 and trigger the redrive.
    for _ in 0..3 {
        let _ = client
            .receive_message()
            .queue_url(&src_url)
            .visibility_timeout(0)
            .send()
            .await
            .unwrap();
    }

    // Source queue should be empty.
    let src_attrs = client
        .get_queue_attributes()
        .queue_url(&src_url)
        .attribute_names(aws_sdk_sqs::types::QueueAttributeName::ApproximateNumberOfMessages)
        .attribute_names(
            aws_sdk_sqs::types::QueueAttributeName::ApproximateNumberOfMessagesNotVisible,
        )
        .send()
        .await
        .unwrap();
    let src_attrs = src_attrs.attributes().unwrap();
    let src_count = src_attrs
        .get(&aws_sdk_sqs::types::QueueAttributeName::ApproximateNumberOfMessages)
        .map(|s| s.as_str())
        .unwrap_or("0");
    let src_not_vis = src_attrs
        .get(&aws_sdk_sqs::types::QueueAttributeName::ApproximateNumberOfMessagesNotVisible)
        .map(|s| s.as_str())
        .unwrap_or("0");
    assert_eq!(src_count, "0", "source queue should be empty after redrive");
    assert_eq!(
        src_not_vis, "0",
        "source queue should have no in-flight messages after redrive"
    );

    // DLQ should have exactly one message.
    let dlq_recv = client
        .receive_message()
        .queue_url(&dlq_url)
        .visibility_timeout(60)
        .send()
        .await
        .unwrap();
    assert_eq!(
        dlq_recv.messages().len(),
        1,
        "DLQ should contain the redriven message"
    );
    assert_eq!(
        dlq_recv.messages()[0].body().unwrap(),
        "redrive-payload",
        "DLQ message body should match the original"
    );
}

/// Regression for sqs-redis-fifo-deduplication-broken:
/// `ContentBasedDeduplication=true` on a FIFO queue must suppress duplicate
/// sends within the 5-minute dedup window. Both calls should succeed and
/// return the same `MessageId`, but the queue must contain exactly one
/// message.
#[tokio::test]
async fn test_fifo_content_based_deduplication_suppresses_duplicate() {
    let client = make_sqs_client().await;

    let resp = client
        .create_queue()
        .queue_name("dedup-test.fifo")
        .attributes(aws_sdk_sqs::types::QueueAttributeName::FifoQueue, "true")
        .attributes(
            aws_sdk_sqs::types::QueueAttributeName::ContentBasedDeduplication,
            "true",
        )
        .send()
        .await
        .unwrap();
    let queue_url = resp.queue_url().unwrap().to_string();

    let r1 = client
        .send_message()
        .queue_url(&queue_url)
        .message_body("dedup-body")
        .message_group_id("g1")
        .send()
        .await
        .unwrap();
    let r2 = client
        .send_message()
        .queue_url(&queue_url)
        .message_body("dedup-body")
        .message_group_id("g1")
        .send()
        .await
        .unwrap();

    assert_eq!(
        r1.message_id(),
        r2.message_id(),
        "duplicate send must return the original MessageId"
    );

    let attrs = client
        .get_queue_attributes()
        .queue_url(&queue_url)
        .attribute_names(aws_sdk_sqs::types::QueueAttributeName::ApproximateNumberOfMessages)
        .send()
        .await
        .unwrap();
    let count = attrs
        .attributes()
        .unwrap()
        .get(&aws_sdk_sqs::types::QueueAttributeName::ApproximateNumberOfMessages)
        .map(|s| s.as_str())
        .unwrap_or("0");
    assert_eq!(
        count, "1",
        "FIFO queue must contain exactly one message after duplicate send"
    );
}

/// Regression for sqs-redis-long-poll-not-event-driven:
/// `ReceiveMessage` with `WaitTimeSeconds=N` must return as soon as a message
/// arrives, not only at start-of-call snapshot time.
#[tokio::test]
async fn test_long_poll_returns_when_message_arrives() {
    let client = make_sqs_client().await;

    let resp = client
        .create_queue()
        .queue_name("long-poll-queue")
        .send()
        .await
        .unwrap();
    let queue_url = resp.queue_url().unwrap().to_string();

    let producer_url = queue_url.clone();
    let producer_client = client.clone();
    let producer = tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        producer_client
            .send_message()
            .queue_url(&producer_url)
            .message_body("late-arriving")
            .send()
            .await
            .unwrap();
    });

    let started = tokio::time::Instant::now();
    let recv = client
        .receive_message()
        .queue_url(&queue_url)
        .wait_time_seconds(2)
        .send()
        .await
        .unwrap();
    let elapsed = started.elapsed();
    producer.await.unwrap();

    assert_eq!(
        recv.messages().len(),
        1,
        "long-poll receive should pick up the late-arriving message"
    );
    assert!(
        elapsed < std::time::Duration::from_millis(1500),
        "long-poll should return promptly after the message arrives, elapsed={elapsed:?}"
    );
}
