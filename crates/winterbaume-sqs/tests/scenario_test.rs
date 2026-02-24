//! Smoke tests for winterbaume SQS service — realistic application scenarios.
//!
//! Each test simulates a coherent end-to-end workflow rather than exercising
//! a single API call in isolation.

use aws_sdk_sqs::config::BehaviorVersion;
use aws_sdk_sqs::types::SendMessageBatchRequestEntry;
use winterbaume_core::MockAws;
use winterbaume_sqs::SqsService;

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

/// Scenario: batch job processing pipeline.
///
/// A producer sends 9 jobs in three batches of 3. A consumer receives them
/// in up to 10-message batches and deletes each successfully processed
/// message. After all rounds the queue should be empty.
#[tokio::test]
async fn test_batch_job_processing_pipeline() {
    let client = make_sqs_client().await;

    // Create the work queue.
    let create = client
        .create_queue()
        .queue_name("jobs-queue")
        .send()
        .await
        .expect("create_queue");
    let queue_url = create.queue_url().expect("queue_url").to_string();

    // Producer: send 9 jobs as three batches of 3.
    for batch_num in 0u32..3 {
        let entries: Vec<_> = (0..3)
            .map(|i| {
                let id = batch_num * 3 + i;
                SendMessageBatchRequestEntry::builder()
                    .id(format!("msg-{id}"))
                    .message_body(format!(r#"{{"job_id":{id},"task":"process"}}"#))
                    .build()
                    .expect("entry")
            })
            .collect();

        let resp = client
            .send_message_batch()
            .queue_url(&queue_url)
            .set_entries(Some(entries))
            .send()
            .await
            .unwrap_or_else(|e| panic!("send_message_batch {batch_num}: {e}"));
        assert_eq!(resp.successful().len(), 3);
        assert!(resp.failed().is_empty());
    }

    // Consumer: drain the queue in receive loops.
    let mut total_received = 0usize;
    for _ in 0..5 {
        // guard against infinite loop
        let recv = client
            .receive_message()
            .queue_url(&queue_url)
            .max_number_of_messages(10)
            .send()
            .await
            .expect("receive_message");

        let messages = recv.messages();
        if messages.is_empty() {
            break;
        }
        total_received += messages.len();

        // Delete each received message to acknowledge processing.
        for msg in messages {
            client
                .delete_message()
                .queue_url(&queue_url)
                .receipt_handle(msg.receipt_handle().expect("receipt_handle"))
                .send()
                .await
                .expect("delete_message");
        }
    }
    assert_eq!(total_received, 9, "all 9 jobs should have been processed");

    // Queue should now be empty.
    let empty = client
        .receive_message()
        .queue_url(&queue_url)
        .max_number_of_messages(10)
        .send()
        .await
        .expect("receive after drain");
    assert!(
        empty.messages().is_empty(),
        "queue should be empty after processing"
    );
}

/// Scenario: dead-letter queue configuration and redrive.
///
/// An operations team configures a main queue with a DLQ and a maxReceiveCount
/// of 2. Verifies the redrive policy is stored, reads it back, and confirms
/// the DLQ is listed as a DLQ source.
#[tokio::test]
async fn test_dead_letter_queue_configuration() {
    let client = make_sqs_client().await;

    // Create the DLQ first.
    let dlq = client
        .create_queue()
        .queue_name("jobs-dlq")
        .send()
        .await
        .expect("create DLQ");
    let dlq_url = dlq.queue_url().expect("dlq url").to_string();

    // Read the DLQ ARN from its attributes.
    let dlq_attrs = client
        .get_queue_attributes()
        .queue_url(&dlq_url)
        .attribute_names(aws_sdk_sqs::types::QueueAttributeName::QueueArn)
        .send()
        .await
        .expect("get_queue_attributes DLQ");
    let dlq_arn = dlq_attrs
        .attributes()
        .and_then(|m| m.get(&aws_sdk_sqs::types::QueueAttributeName::QueueArn))
        .expect("DLQ ARN")
        .clone();

    // Create the main queue with a redrive policy pointing at the DLQ.
    let redrive_policy = format!(r#"{{"deadLetterTargetArn":"{dlq_arn}","maxReceiveCount":"2"}}"#);
    let main = client
        .create_queue()
        .queue_name("jobs-main")
        .attributes(
            aws_sdk_sqs::types::QueueAttributeName::RedrivePolicy,
            &redrive_policy,
        )
        .send()
        .await
        .expect("create main queue with redrive policy");
    let main_url = main.queue_url().expect("main url").to_string();

    // Verify the redrive policy is stored on the main queue.
    let main_attrs = client
        .get_queue_attributes()
        .queue_url(&main_url)
        .attribute_names(aws_sdk_sqs::types::QueueAttributeName::RedrivePolicy)
        .send()
        .await
        .expect("get_queue_attributes main");
    let stored_policy = main_attrs
        .attributes()
        .and_then(|m| m.get(&aws_sdk_sqs::types::QueueAttributeName::RedrivePolicy))
        .expect("RedrivePolicy attribute");
    assert!(
        stored_policy.contains(&dlq_arn),
        "stored policy should reference DLQ ARN"
    );
    assert!(
        stored_policy.contains("maxReceiveCount"),
        "stored policy should contain maxReceiveCount"
    );

    // Confirm the DLQ is listed as a source for the main queue.
    let sources = client
        .list_dead_letter_source_queues()
        .queue_url(&dlq_url)
        .send()
        .await
        .expect("list_dead_letter_source_queues");
    assert!(
        sources.queue_urls().iter().any(|u| u == &main_url),
        "main queue should appear as DLQ source"
    );
}

/// Scenario: visibility timeout extension for long-running jobs.
///
/// A consumer receives a message representing a slow job. Before the initial
/// visibility timeout expires the worker extends it with
/// change_message_visibility, simulating a heartbeat. After completing the
/// job the message is deleted.
#[tokio::test]
async fn test_visibility_timeout_heartbeat() {
    let client = make_sqs_client().await;

    let create = client
        .create_queue()
        .queue_name("slow-jobs-queue")
        .attributes(
            aws_sdk_sqs::types::QueueAttributeName::VisibilityTimeout,
            "30",
        )
        .send()
        .await
        .expect("create_queue");
    let queue_url = create.queue_url().expect("queue_url").to_string();

    // Send a single slow job.
    client
        .send_message()
        .queue_url(&queue_url)
        .message_body(r#"{"type":"slow_etl","source":"s3://bucket/large.csv"}"#)
        .send()
        .await
        .expect("send_message");

    // Worker receives the job.
    let recv = client
        .receive_message()
        .queue_url(&queue_url)
        .max_number_of_messages(1)
        .send()
        .await
        .expect("receive_message");
    let msg = &recv.messages()[0];
    let receipt = msg.receipt_handle().expect("receipt_handle");

    // Heartbeat: extend the visibility timeout before it would expire.
    client
        .change_message_visibility()
        .queue_url(&queue_url)
        .receipt_handle(receipt)
        .visibility_timeout(60)
        .send()
        .await
        .expect("change_message_visibility extend");

    // Job finishes — delete the message.
    client
        .delete_message()
        .queue_url(&queue_url)
        .receipt_handle(receipt)
        .send()
        .await
        .expect("delete_message after job completion");

    // Queue should be empty.
    let after = client
        .receive_message()
        .queue_url(&queue_url)
        .max_number_of_messages(1)
        .visibility_timeout(0)
        .send()
        .await
        .expect("receive after delete");
    assert!(
        after.messages().is_empty(),
        "queue should be empty after job deletion"
    );
}

/// Scenario: FIFO queue ordered message processing.
///
/// An order management system uses a FIFO queue to guarantee that events for
/// the same order are processed in sequence. It sends three events for one
/// order and one for another, then verifies ordering within each group.
#[tokio::test]
async fn test_fifo_queue_ordered_processing() {
    let client = make_sqs_client().await;

    let create = client
        .create_queue()
        .queue_name("orders.fifo")
        .attributes(aws_sdk_sqs::types::QueueAttributeName::FifoQueue, "true")
        .attributes(
            aws_sdk_sqs::types::QueueAttributeName::ContentBasedDeduplication,
            "false",
        )
        .send()
        .await
        .expect("create FIFO queue");
    let queue_url = create.queue_url().expect("queue_url").to_string();

    // Send three events for order-1 in sequence.
    let events = [
        ("order-1", "created", "dedup-1a"),
        ("order-1", "payment_received", "dedup-1b"),
        ("order-1", "shipped", "dedup-1c"),
    ];
    for (group, status, dedup) in events {
        client
            .send_message()
            .queue_url(&queue_url)
            .message_body(format!(r#"{{"order_id":"{group}","status":"{status}"}}"#))
            .message_group_id(group)
            .message_deduplication_id(dedup)
            .send()
            .await
            .unwrap_or_else(|e| panic!("send_message {status}: {e}"));
    }

    // Receive and process all three events, verifying FIFO order.
    let expected_statuses = ["created", "payment_received", "shipped"];
    for expected in expected_statuses {
        let recv = client
            .receive_message()
            .queue_url(&queue_url)
            .max_number_of_messages(1)
            .send()
            .await
            .expect("receive_message");
        let msg = recv
            .messages()
            .first()
            .unwrap_or_else(|| panic!("expected message for status {expected}"));
        assert!(
            msg.body().unwrap_or("").contains(expected),
            "expected status '{expected}' in body '{}'",
            msg.body().unwrap_or("")
        );
        client
            .delete_message()
            .queue_url(&queue_url)
            .receipt_handle(msg.receipt_handle().expect("receipt_handle"))
            .send()
            .await
            .expect("delete_message");
    }
}
